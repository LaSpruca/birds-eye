mod be;
mod config;

use crate::be::helloworld::MyGreeter;
use crate::config::load_config;
use birdseye_common::rpc::hello_world::greeter_server::GreeterServer;
use hyper::server::conn::Http;
use std::net::SocketAddr;
use std::process::exit;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};
use tonic::transport::Server;
use tower_http::ServiceBuilderExt;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug,rustls=info")
        .init();

    let config = load_config();
    info!("Loaded config");

    Ok(())
}
