//! Everything related to the BridsEye birdseye-server configuration

use serde::{Deserialize, Serialize};
use std::env::var;
use std::path::PathBuf;
use tracing::warn;

/// Configuration for the Birds Eye birdseye-server
///
/// # Configuration
/// | Field | Environment Variable | Type    | Default       | Description                                                               |
/// |-------|----------------------|---------|---------------|---------------------------------------------------------------------------|
/// | key   | BE_SERVER_KEY        | PathBuf | `key.pem`     | The location of the key to be used by the birdseye-server for TLS         |
/// | cert  | BE_SERVER_CERT       | PathBuf | `cert.pem`    | The location of the certificate to be used by the birdseye-server for TLS |
/// | host  | BE_SERVER_HOST       | String  | `"127.0.0.1"` | The host for the BirdsEye birdseye-server to bind to                      |
/// | port  | BE_SERVER_PORT       | u16     | `42069`       | The port for the BirdsEye birdseye-server to bind to                      |
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub key: PathBuf,
    pub cert: PathBuf,
    pub host: String,
    pub port: u16,
    pub static_path: PathBuf,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let mut slf = Self::default();

        // Get the path for the birdseye-server key
        if let Ok(key) = var("BE_SERVER_KEY") {
            match key.parse() {
                Ok(key) => slf.key = key,
                Err(err) => warn!("Invalid path for BE_SERVER_KEY {err}, using default `key.pem `"),
            }
        }

        // Get the path for the birdseye-server certificate
        if let Ok(cert) = var("BE_SERVER_CERT") {
            match cert.parse() {
                Ok(cert) => slf.cert = cert,
                Err(err) => {
                    warn!("Invalid path for BE_SERVER_CERT {err}, using default `cert.pem `")
                }
            }
        }

        // Get the host to bind too
        if let Ok(host) = var("BE_SERVER_HOST") {
            slf.host = host;
        }

        // Get the port to bind to
        if let Ok(port) = var("BE_SERVER_PORT") {
            match port.parse() {
                Ok(port) => slf.port = port,
                Err(err) => warn!("Invalid port for BE_SERVER_PORT {err}, using default 42069"),
            }
        }

        slf
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            key: "".into(),
            cert: "".into(),
            port: 42069,
            host: "127.0.0.1".into(),
            static_path: "static".into(),
        }
    }
}
