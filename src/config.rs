use lazy_static::lazy_static;
use serde::Deserialize;

static PKG_VERSION: &str = std::env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref CONFIG: Config = initialize_config();
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(skip, default = "default_pkg_version")]
    pub pkg_version: String,
    #[serde(default = "default_paste_dir")]
    pub paste_dir: String,
    #[serde(default = "default_server_host")]
    pub server_host: String,
    #[serde(default = "default_server_port")]
    pub server_port: u16,
    #[serde()]
    pub paste_bearer_token: String,
}

fn default_pkg_version() -> String {
    PKG_VERSION.to_string()
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

fn initialize_config() -> Config {
    dotenv::dotenv().ok();
    // Initialising the env logger probably shouldn't be in the config, but this ensures that it's
    // immediately after the .env file is parsed.
    env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    config
}
