use serde::Deserialize;
use std::net::IpAddr;
use tokio::signal;
use warp::Filter;

use crate::config::CONFIG;

mod config;
mod routes;

#[derive(Deserialize)]
struct Config {
    #[serde(default = "default_paste_dir")]
    paste_dir: String,
    #[serde(default = "default_server_host")]
    server_host: String,
    #[serde(default = "default_server_port")]
    server_port: u16,
}

fn default_paste_dir() -> String {
    String::from("pastes")
}

fn default_server_host() -> String {
    String::from("127.0.0.1")
}

fn default_server_port() -> u16 {
    80
}

#[tokio::main]
async fn main() {
    log::info!("Version v{}", &CONFIG.pkg_version);
    log::info!("Serving pastes from `{}`", &CONFIG.paste_dir);

    let routes = routes::index_route()
        .or(routes::pastes_route())

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
