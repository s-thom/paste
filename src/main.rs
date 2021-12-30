use serde::Deserialize;
use std::net::IpAddr;
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

    warp::serve(routes)
        .run((
            config.server_host.parse::<IpAddr>().unwrap(),
            config.server_port,
        ))
        .await;
}
