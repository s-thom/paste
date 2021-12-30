use warp::http::header::{HeaderMap, HeaderValue};
use warp::{Filter, Rejection, Reply};

pub fn index_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html("index route"))
}

pub fn pastes_route(
    paste_dir: String,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let mut pastes_headers = HeaderMap::new();
    pastes_headers.insert("content-type", HeaderValue::from_static("text/plain"));
    pastes_headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );

    warp::get()
        .and(warp::fs::dir(paste_dir))
        .with(warp::reply::with::headers(pastes_headers))
}
