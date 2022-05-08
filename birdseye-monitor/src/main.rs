mod config;

use crate::config::load_config;
use rustls::ServerName;
use std::fs::read;
use std::process::exit;
use std::ptr::write;
use std::{
    io::{self, prelude::*},
    sync::Arc,
};
use tracing::{error, info};

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug")
        .init();

    let app_config = load_config();

    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let rc_config = Arc::new(config);

    let example_com = match (app_config.server.host.as_str(), app_config.server.port).try_into() {
        Ok(val) => val,
        Err(err) => {
            error!("Could not parse host: {err}");
            exit(1);
        }
    };

    let mut client = rustls::ClientConnection::new(rc_config, example_com).unwrap();

    let mut writer = client.writer();
    writer.write_all("Hello world!".as_bytes()).unwrap();
    writer.flush().unwrap();

    let mut msg = String::new();

    if client.wants_read() {
        let mut reader = client.reader();
        reader.read_to_string(&mut msg).unwrap();
    }

    info!("Got message {msg}");
}
