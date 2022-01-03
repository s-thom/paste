use mime::Mime;
use warp::http::header::{
    HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_SECURITY_POLICY, CONTENT_TYPE,
    X_CONTENT_TYPE_OPTIONS,
};
use warp::{Filter, Rejection, Reply};

use crate::handlers;

pub fn headers_wrapper() -> warp::filters::reply::WithHeaders {
    let mut pastes_headers = HeaderMap::new();
    pastes_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    pastes_headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    pastes_headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'none'"),
    );

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
        .and(warp::header::<Mime>(CONTENT_TYPE.as_str()))
        .and(warp::body::stream())
        .and_then(handlers::create_handler)
}
