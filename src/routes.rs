use warp::http::header::{HeaderMap, HeaderValue};
use warp::{Filter, Rejection, Reply};
use crate::config::CONFIG;

pub fn headers_wrapper() -> warp::filters::reply::WithHeaders {
    let mut pastes_headers = HeaderMap::new();
    pastes_headers.insert("content-type", HeaderValue::from_static("text/plain"));
    pastes_headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );

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
