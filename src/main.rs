use std::env;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let base_dir: String = match env::var("PASTE_DIR") {
        Ok(val) => val,
        Err(_e) => {
            log::trace!("PASTE_DIR variable was not set, defaulting to `pastes`");
            String::from("pastes")
        }
    };

    let mut pastes_headers = HeaderMap::new();
    pastes_headers.insert("content-type", HeaderValue::from_static("text/plain"));
    pastes_headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );

    // let path = Path::new(&base_dir);

    log::info!("Serving pastes from `{}`", base_dir);

    let get_index_route = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html("index route"));
    let get_pastes_route = warp::get()
        .and(warp::fs::dir(base_dir))
        .with(warp::reply::with::headers(pastes_headers));

    let routes = get_index_route.or(get_pastes_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
