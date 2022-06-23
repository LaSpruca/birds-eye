mod server;

use crate::config::server::ServerConfig;
use serde::{Deserialize, Serialize};
use std::env::args;
use std::fs::read_to_string;
use std::{env::var, path::PathBuf};
use tracing::{warn};

/// Configuration for the monitor application
///
/// # Configuration
/// | Field       | Environment Variable | Type             | Default           | Description                                              |
/// |-------------|----------------------|------------------|-------------------|----------------------------------------------------------|
/// | server_addr | SERVER_ADDR          | String           | "localhost:42069" | The address to the birdseye server                       |
/// | ca_cert     | CA_CERT              | Option<PathBuff> | None              | Any additional CA Certificates to be used by application |
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub server: ServerConfig,
    pub ca_cert: Option<PathBuf>,
}

impl Config {
    pub fn from_env() -> Self {
        let mut slf = Self::default();

        slf.server = ServerConfig::from_env();

        if let Ok(ca_cert) = var("CA_CERT") {
            match ca_cert.parse() {
                Ok(ca_cert) => slf.ca_cert = Some(ca_cert),
                Err(err) => warn!("Could not pass value for CA_CERT: {err}, ignoring"),
            }
        }

        slf
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
                warn!("Could not read config file {err}, using defaults");
            }
        }
    }

    slf
}
