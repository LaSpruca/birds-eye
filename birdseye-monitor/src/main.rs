mod client;
mod config;
mod platform;

use crate::client::process::monitor_processes;
use crate::config::load_config;
use crate::platform::get_current_user;
use sysinfo::SystemExt;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug")
        .init();

    // Load application configuration
    let _config = load_config();

    let mut stream = monitor_processes();

    info!("Current user is: {:?}", get_current_user());

    for usr in sysinfo::System::default().users() {
        info!("{:?}", usr);
    }

    use scrap::{Capturer, Display};
    use std::io::ErrorKind::WouldBlock;
    use std::io::Write;
    use std::process::{Command, Stdio};

    let d = Display::primary().unwrap();
    let (w, h) = (d.width(), d.height());

    let child = Command::new("ffplay")
        .args(&[
            "-f",
            "rawvideo",
            "-pixel_format",
            "bgr0",
            "-video_size",
            &format!("{}x{}", w, h),
            "-framerate",
            "60",
            "-",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");

    let mut capturer = Capturer::new(d).unwrap();
    let mut out = child.stdin.unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                // Write the frame, removing end-of-row padding.
                let stride = frame.len() / h;
                let rowlen = 4 * w;
                for row in frame.chunks(stride) {
                    let row = &row[..rowlen];
                    out.write_all(row).unwrap();
                }
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        }
    }
}
