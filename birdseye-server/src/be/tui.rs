use std::{
    io::Read,
    sync::{Arc, RwLock},
};

use tokio::io::{stdin, AsyncReadExt, BufReader};
use tracing::error;

pub async fn run(tokens: Arc<RwLock<Vec<String>>>) {
    let mut stdin = BufReader::new(stdin());
    let mut rng = rand::thread_rng();
    loop {
        let mut line = String::new();
        if stdin.read_to_string(&mut line).await.is_err() {
            error!("Could not read input");
        }
        let (command, rest) = line.split_once(" ").unwrap_or((line.as_str(), ""));
        match command {
            "new-client" => {}
        }
    }
}
