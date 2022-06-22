mod config;

use crate::config::load_config;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug,rustls=info")
        .init();

    let config = load_config();
    info!("Loaded config");

    todo!("Insert server code here :)");

    Ok(())
}
