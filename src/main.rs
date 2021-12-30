use serde::Deserialize;
use std::net::IpAddr;
use tokio::signal;
use warp::Filter;

static PKG_VERSION: &str = std::env!("CARGO_PKG_VERSION");

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
    dotenv::dotenv().ok();
    env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    log::info!("Version v{}", PKG_VERSION);
    log::info!("Serving pastes from `{}`", config.paste_dir);

    let routes = routes::index_route(PKG_VERSION.to_string())
        .or(routes::pastes_route(config.paste_dir))
        .with(routes::headers_wrapper());

    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(
        (
            config.server_host.parse::<IpAddr>().unwrap(),
            config.server_port,
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
