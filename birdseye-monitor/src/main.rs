mod config;
mod net;

use crate::config::load_config;
use std::{fs::File, io::BufReader, net::ToSocketAddrs, process::exit, sync::Arc};
use tokio::{
    io::{self, copy, split, stdin as tokio_stdin, stdout as tokio_stdout, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{
    rustls::{self, Certificate, OwnedTrustAnchor},
    TlsConnector,
};
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug")
        .init();

    // Load application configuration
    let app_config = load_config();

    // Get the config address
    let addr =
        match match (app_config.server.host.as_str(), app_config.server.port).to_socket_addrs() {
            Ok(ok) => ok,
            Err(err) => {
                error!("Invalid address: {err}");
                exit(1)
            }
        }
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        {
            Ok(val) => val,
            Err(err) => {
                error!("Invalid address: {err}");
                exit(1);
            }
        };
    let domain = app_config
        .server
        .domain
        .unwrap_or_else(|| app_config.server.host.clone());

    let content = format!("GET / HTTP/1.0\r\nHost: {}\r\n\r\n", domain);

    let mut root_cert_store = rustls::RootCertStore::empty();

    // Load the specified certificate
    if let Some(path) = app_config.ca_cert {
        info!("Loading certificate {}", path.display());

        match File::open(&path) {
            Ok(val) => match rustls_pemfile::certs(&mut BufReader::new(val)) {
                Ok(val) => {
                    for a in val.into_iter().map(Certificate) {
                        match root_cert_store.add(&a) {
                            Ok(_) => {}
                            Err(err) => {
                                error!("Could not add certificate: {err}");
                            }
                        };
                    }
                }
                Err(err) => {
                    error!("Error reading certificate: {err}");
                }
            },
            Err(err) => {
                error!("Error opening certificate: {err}");
            }
        };
    } else {
        root_cert_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(
            |ta| {
                OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            },
        ));
    }

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth(); // i guess this was previously the default?
    let connector = TlsConnector::from(Arc::new(config));

    let stream = match TcpStream::connect(&addr).await {
        Ok(val) => val,
        Err(err) => {
            error!("Could not create TCP stream: {err}");
            exit(1);
        }
    };

    let (mut stdin, mut stdout) = (tokio_stdin(), tokio_stdout());

    debug!("Using domain {domain}");

    let domain = match rustls::ServerName::try_from(domain.as_str())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))
    {
        Ok(val) => val,
        Err(err) => {
            error!("Error parsing server name {err}");
            exit(1);
        }
    };

    let mut stream = match connector.connect(domain, stream).await {
        Ok(val) => val,
        Err(err) => {
            error!("Could not create TCP stream: {err}");
            exit(1);
        }
    };

    match stream.write_all(content.as_bytes()).await {
        Ok(_) => {}
        Err(err) => {
            warn!("Could not write bytes {err}");
        }
    };

    let (mut reader, mut writer) = split(stream);

    Ok(())
}
