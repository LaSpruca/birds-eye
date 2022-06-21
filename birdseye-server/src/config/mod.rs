//! All structs and code relating to application configuration

mod server;

pub use server::ServerConfig;

use serde::{Deserialize, Serialize};
use std::env::{args, var};
use std::fs::read_to_string;
use tracing::warn;

/// Configuration for the birdseye-server application
/// # Configuration
/// | Field     | Environment Variable | Type         | Default                                  | Description                                        |
/// |-----------|----------------------|--------------|------------------------------------------|----------------------------------------------------|
/// | be_server | BE_SERVER            | ServerConfig | See [ServerConfig](birdseye-server::ServerConfig) | The configuration for the birdseye-server |
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub be_server: ServerConfig,
}

impl Config {
    fn from_env() -> Self {
        Self {
            // Get the config for the BirdsEye Server
            be_server: ServerConfig::from_env(),
        }
    }
}

pub fn load_config() -> Config {
    let use_env = args().any(|arg| &arg == "--env" || &arg == "-e");
    let mut slf = Config::default();

    if use_env {
        Config::from_env();
    } else {
        let path = if let Ok(pth) = var("CONFIG_FILE") {
            pth
        } else {
            "config.toml".into()
        };

        match read_to_string(&path) {
            Ok(file) => match toml::from_str::<Config>(&file) {
                Ok(val) => {
                    slf = val;
                }
                Err(err) => {
                    warn!("Error parsing config file {err} using defaults");
                }
            },
            Err(err) => {
                warn!(
                    "Could not read config from path {}: {err}, using defaults",
                    path
                );
            }
        }
    }

    slf
}
