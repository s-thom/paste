use std::net::IpAddr;
use tokio::signal;
use warp::Filter;

use crate::config::CONFIG;

mod config;
mod errors;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    log::info!("Version v{}", &CONFIG.pkg_version);
    log::info!("Serving pastes from `{}`", &CONFIG.paste_dir);

    let routes = routes::index_route()
        .or(routes::pastes_route())
        .or(routes::create_route())
        .recover(handlers::recover_handler)
        .with(routes::headers_wrapper());

    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(
        (
            CONFIG.server_host.parse::<IpAddr>().unwrap(),
            CONFIG.server_port,
        ),
        async {
            signal::ctrl_c()
                .await
                .expect("Failed to listen for `ctrl+c`");
            log::error!("Got shutdown signal");
        },
    );

    log::info!("Starting server on `{}`", addr);
    server.await;
}
