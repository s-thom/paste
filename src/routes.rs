use futures_util::Stream;
use futures_util::TryStreamExt;
use mime::Mime;
use mpart_async::server::MultipartStream;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::{Buf, Filter, Rejection, Reply};

use crate::config::CONFIG;

pub fn headers_wrapper() -> warp::filters::reply::WithHeaders {
    let mut pastes_headers = HeaderMap::new();
    pastes_headers.insert("content-type", HeaderValue::from_static("text/plain"));
    pastes_headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    // TODO: Add strict CSP headers

    warp::reply::with::headers(pastes_headers)
}

pub fn index_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let version_string = &CONFIG.pkg_version;

    warp::path!().and(warp::get()).map(move || {
        warp::reply::html(format!(
            r#"paste v{}

A tiny paste utility for self-hosting.
https://github.com/s-thom/paste
"#,
            version_string
        ))
    })
}

pub fn pastes_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let paste_dir = &CONFIG.paste_dir;

    warp::path!().and(warp::get()).and(warp::fs::dir(paste_dir))
}

async fn upload_multipart(
    mime: Mime,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin,
) -> Result<impl warp::Reply, Rejection> {
    let boundary = mime.get_param("boundary").map(|v| v.to_string()).unwrap();

    let mut stream = MultipartStream::new(
        boundary,
        body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining())),
    );

    while let Ok(Some(mut field)) = stream.try_next().await {
        println!("Field received:{}", field.name().unwrap());
        if let Ok(filename) = field.filename() {
            println!("Field filename:{}", filename);
        }

        while let Ok(Some(bytes)) = field.try_next().await {
            println!("Bytes received:{}", bytes.len());
        }
    }

    Ok(format!("Thanks!\n"))
}

pub fn create_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!()
        .and(warp::post())
        .and(warp::header::<Mime>("content-type"))
        .and(warp::body::stream())
        .and_then(upload_multipart)
}
