use crate::config::ServerConfig;
use rustls_pemfile::{certs, rsa_private_keys};
use std::fs::File;
use std::io::{self, BufReader};
use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{copy, sink, split, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::TlsAcceptor;
use tracing::{error, info};

fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    info!("Loading certs from {}", path.display());
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    info!("Loading certs from {}", path.display());

    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

pub async fn run(config: &ServerConfig) -> io::Result<()> {
    let addr = match (config.host.as_str(), config.port).to_socket_addrs() {
        Ok(val) => val,
        Err(err) => {
            error!("Error getting socket address: {err}");
            return Err(err);
        }
    }
    .next()
    .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;

    let certs = load_certs(&config.cert)?;
    let mut keys = load_keys(&config.key)?;

    info!("Certs {certs:?}");
    info!("Keys {keys:?}");

    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        let fut = async move {
            let mut stream = acceptor.accept(stream).await?;

            let (mut reader, mut writer) = split(stream);
            let n = copy(&mut reader, &mut writer).await?;
            writer.flush().await?;
            println!("Echo: {} - {}", peer_addr, n);

            Ok(()) as io::Result<()>
        };

        tokio::spawn(async move {
            if let Err(err) = fut.await {
                eprintln!("{:?}", err);
            }
        });
    }
}
