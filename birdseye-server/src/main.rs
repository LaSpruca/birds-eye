use crate::be::run;
use crate::config::load_config;
use tracing::info;

mod be;
mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug,rustls=info")
        .init();

    let config = load_config();
    info!("Loaded config");

    info!("Starting BirdsEye server");

    run(&config.be_server).await.unwrap();
}
