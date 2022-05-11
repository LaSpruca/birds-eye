//! Everything related to the BridsEye birdseye-server configuration

use serde::{Deserialize, Serialize};
use std::env::var;
use std::path::PathBuf;
use tracing::warn;

/// Configuration for the Birds Eye birdseye-server
///
/// # Configuration
/// | Field  | Environment Variable | Type    | Default       | Description                                                                                                                                                                                            |
/// |--------|----------------------|---------|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | host   | SERVER_HOST          | String  | `"127.0.0.1"` | The host of the BirdsEye server                                                                                                                                                                        |
/// | domain | SERVER_DOMAIN        | String  | `None`        | The domain used for certificate validation, due to limitations in [`rustls`](https://docs.rs/rustls/latest/rustls) at the moment, this must be specified if using an ip address otherwise host is used |
/// | port   | SERVER_PORT          | u16     | `42069`       | The port of the BirdsEye server                                                                                                                                                                        |
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub host: String,
    pub domain: Option<String>,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let mut slf = Self::default();

        // Get the host to bind too
        if let Ok(host) = var("SERVER_HOST") {
            slf.host = host;
        }

        // Get the port to bind to
        if let Ok(port) = var("SERVER_PORT") {
            match port.parse() {
                Ok(port) => slf.port = port,
                Err(err) => warn!("Invalid port for BE_SERVER_PORT {err}, using default 42069"),
            }
        }

        // Get the port to bind to
        if let Ok(domain) = var("SERVER_DOMAIN") {
            slf.domain = Some(domain);
        }

        slf
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 42069,
            host: "127.0.0.1".into(),
            domain: None,
        }
    }
}
