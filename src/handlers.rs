use futures_util::TryStreamExt;
use mime::Mime;
use mpart_async::server::MultipartStream;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_stream::{Stream, StreamExt};
use warp::hyper::Uri;
use warp::{Buf, Rejection};

use crate::config::CONFIG;

pub async fn index_handler() -> Result<impl warp::Reply, Rejection> {
    let version_string = &CONFIG.pkg_version;

    let content = format!(
        r#"paste v{}

A tiny paste utility for self-hosting.
https://github.com/s-thom/paste
"#,
        version_string
    );

    Ok(content)
}

pub async fn pastes_handler(filename: String) -> Result<impl warp::Reply, Rejection> {
    // Create pastes directory if not already present
    let base_dir = &CONFIG.paste_dir;
    let create_dir_result = fs::create_dir_all(base_dir).await;
    if let Err(err) = create_dir_result {
        log::error!("Error when creating pastes directory: {}", err);
        return Err(warp::reject());
    }

    let file_path = Path::new(base_dir).join(filename);
    let open_file_result = File::open(file_path).await;
    if let Err(err) = open_file_result {
        log::error!("Error when opening file: {}", err);
        return Err(warp::reject());
    }

    // Read file into buffer
    let mut file = open_file_result.unwrap();
    let mut contents = vec![];
    let read_result = file.read_to_end(&mut contents).await;
    if let Err(err) = read_result {
        log::error!("Error when reading file: {}", err);
        return Err(warp::reject());
    }

    Ok(contents)
}

pub async fn create_handler(
    mime: Mime,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin,
) -> Result<impl warp::Reply, Rejection> {
    let boundary = mime.get_param("boundary").map(|v| v.to_string()).unwrap();
    let mut stream = MultipartStream::new(
        boundary,
        body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining())),
    );

    if let Ok(Some(mut field)) = StreamExt::try_next(&mut stream).await {
        let field_name = field.name().unwrap();
        log::debug!("Field received: {}", field_name);

        // Create pastes directory if not already present
        let base_dir = &CONFIG.paste_dir;
        let create_dir_result = fs::create_dir_all(base_dir).await;
        if let Err(err) = create_dir_result {
            log::error!("Error when creating pastes directory: {}", err);
            return Err(warp::reject());
        }

        // Figure out path for the new file
        let new_id = ulid::Ulid::new().to_string();
        let new_file_path = Path::new(base_dir).join(&new_id);

        // Create the file
        let create_file_result = File::create(&new_file_path).await;
        if let Err(err) = create_file_result {
            log::error!("Error when creating file: {}", err);
            return Err(warp::reject());
        }

        let mut file = create_file_result.unwrap();
        // Write stream to file
        while let Ok(Some(bytes)) = tokio_stream::StreamExt::try_next(&mut field).await {
            file.write_all(&bytes)
                .await
                .expect("Failed to write to file");
        }

        let flush_result = file.flush().await;
        if let Err(err) = flush_result {
            log::error!("Failed to flush file, removing if possible: {}", err);
            let rm_result = fs::remove_file(new_file_path).await;
            if let Err(rm_err) = rm_result {
                log::warn!("Failed to remove file, ignoring this error: {}", rm_err);
            }
            return Err(warp::reject());
        }

        return Ok(warp::redirect(
            format!("/{}", new_id).parse::<Uri>().expect("valid path"),
        ));
    }

    log::warn!("Request is being ignored, as no fields were accepted",);
    Err(warp::reject())
}
