use mime::Mime;
use warp::http::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use warp::{Filter, Rejection, Reply};

use crate::handlers;

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
    warp::path!()
        .and(warp::get())
        .and_then(handlers::index_handler)
}

pub fn pastes_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!(String)
        .and(warp::get())
        .and_then(handlers::pastes_handler)
}

pub fn create_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!()
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1024 * 64))
        .and(warp::header::<String>(AUTHORIZATION.as_str()))
        .and(warp::header::<Mime>("content-type"))
        .and(warp::body::stream())
        .and_then(handlers::create_handler)
}
