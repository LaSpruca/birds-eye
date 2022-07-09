use notify::Event;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::fs::read_to_string;
use std::path::{Component, PathBuf};
use std::process::{Child, Command};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, watch, RwLock};
use tracing::{error, info};

#[cfg(all(debug_assertions, unix))]
const SERVER_PATH: &str = "./target/debug/birdseye-server";
#[cfg(all(not(debug_assertions), unix))]
const SERVER_PATH: &str = "./target/release/birdseye-server";

#[cfg(all(debug_assertions, windows))]
const SERVER_PATH: &str = "./target/debug/birdseye-server.exe";
#[cfg(all(not(debug_assertions), windows))]
const SERVER_PATH: &str = "./target/release/birdseye-server.exe";

pub async fn asset_compiler(tx: Arc<RwLock<bool>>, mut rx: mpsc::Receiver<notify::Result<Event>>) {
    let mut crates = HashSet::new();
    let mut html = HashSet::new();
    let mut scss = HashSet::new();

    info!("Starting server");
    let mut server_process = spawn_server();

    loop {
        let mut restart_server = false;
        let while_loop = async {
            while let Some(Ok(val)) = rx.recv().await {
                if val.kind.is_modify() || val.kind.is_create() || val.kind.is_modify() {
                    for path in val
                        .paths
                        .iter()
                        .filter(|pth| {
                            pth.extension()
                                .map(|ext| {
                                    ext == OsStr::new("rs")
                                        || ext == OsStr::new("scss")
                                        || ext == OsStr::new("html")
                                        || ext == OsStr::new("toml")
                                })
                                .unwrap_or(false)
                        })
                        .filter(|path| {
                            path.components().any(|f| {
                                f.as_os_str() == OsStr::new("src")
                                    || f.as_os_str() == OsStr::new("resources")
                            } || path.file_name() == Some(OsStr::new("server.toml")))
                        })
                    {
                        let ext = path.extension().unwrap();
                        if path.file_name() == Some(OsStr::new("server.toml")) {
                            restart_server = true;
                        }
                        // Check to see if rust project file
                        else if ext == OsStr::new("rs") || ext == OsStr::new("toml") {
                            if let Some(pth) = path.components().find(|pth| {
                                pth.as_os_str()
                                    .to_str()
                                    .map(|str| str.contains("birdseye-"))
                                    .unwrap_or(false)
                            }) {
                                crates.insert(pth.as_os_str().to_str().unwrap().to_string());
                            }
                        } else if path.components().any(|pth| {
                            pth.as_os_str()
                                .to_str()
                                .map(|str| str.contains("resources"))
                                .unwrap_or(false)
                        }) {
                            if ext == OsStr::new("html") {
                                html.insert(path.to_owned());
                            } else {
                                scss.insert(path.to_owned());
                            }
                        }
                    }
                }
            }
        };

        tokio::select! {
        _ = while_loop => {},
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
            let mut send = false;
            compile_crates(&crates);

            // Check to see if the frontend should be refreshed
            if crates.contains("birdseye-frontend") {
                send = true;
            }

            // Check to see if the server should be restarted
            if crates.contains("birdseye-server") {
                restart_server = true;
            }

            // Check to see if the common crate was recompiled (both server and front end will need to be restarted)
            if crates.contains("birdseye-common") {
                restart_server = true;
                send = true;
            }


            crates = HashSet::new();

            if !html.is_empty() {
                compile_html(&html);
                send = true;
            }

            html = HashSet::new();

            if !scss.is_empty() {
                compile_scss(&scss);
                send = true;
            }

            scss = HashSet::new();

            // Signal dashboard to refresh
            if send {
                info!("Refreshing frontend");
                *tx.write().await = true;
            }

            // Restart the server if necessary
            if restart_server {
                info!("Stopping server");
                if let Some(mut child_process) = server_process {
                    match child_process.kill() {
                        Ok(_) => {},
                        Err(ex) => {
                            error!("Could not close server {ex}");
                        }
                    };
                }

                info!("Restarting server");
                server_process = spawn_server();
            }
        }};
    }
}

fn spawn_server() -> Option<Child> {
    match Command::new(SERVER_PATH)
        .env("CONFIG_FILE", "./server.toml")
        .spawn()
    {
        Ok(child) => Some(child),
        Err(err) => {
            error!("Could not start server: {err}");
            None
        }
    }
}

fn compile_scss(scss: &HashSet<PathBuf>) {
    for path in scss.iter() {
        info!("Compiling {}", path.display());
        let mut output: PathBuf = path
            .components()
            .map(|f| {
                if f == Component::Normal(OsStr::new("resources")) {
                    Component::Normal(OsStr::new("static"))
                } else {
                    f
                }
            })
            .collect();
        output.set_extension("css");

        match Command::new("dart-sass")
            .args([path.to_str().unwrap(), output.to_str().unwrap()])
            .spawn()
        {
            Ok(mut chld) => match chld.wait() {
                Ok(_) => {
                    info!("Compiled {}", path.display());
                }
                Err(err) => {
                    error!("dart-sass exited abnormally, {err}")
                }
            },
            Err(err) => {
                error!("Couldn't run dart-sass {err}, do you have dart-sass installed?");
            }
        };
    }
}

fn compile_html(html: &HashSet<PathBuf>) {
    for path in html.iter().filter(|pth| {
        pth.components()
            .any(|x| x == Component::Normal(OsStr::new("resources")))
    }) {
        info!("Compiling {}", path.display());
        let file = read_to_string(path).unwrap();
        #[cfg(debug_assertions)]
        let file = file.replace(
            "</head>",
            r#"<script type="module">import init from "https://be.laspruca.nz:42069/live-reload.js"; init("https://be.laspruca.nz:42069/live-reload_bg.wasm");</script></head>"#);

        let file = file
            .replace("\n", "")
            .replace("\t", "")
            .replace("    ", "")
            .replace("  ", "");

        let output: PathBuf = path
            .components()
            .map(|f| {
                if f.as_os_str() == OsStr::new("resources") {
                    Component::Normal(OsStr::new("static"))
                } else {
                    f
                }
            })
            .collect();

        match fs::write(&output, file) {
            Ok(_) => {
                info!("Compiled {}", path.display());
            }
            Err(err) => {
                error!("Could not write to {}: {err}", output.display());
            }
        };
    }
}

fn compile_crates(crates: &HashSet<String>) {
    for crate_ in crates.iter() {
        match crate_.as_str() {
            "birdseye-frontend" => {
                info!("Recompiling birdseye-frontend");
                match Command::new("wasm-pack")
                    .args([
                        "build",
                        "--target",
                        "no-modules",
                        "--out-name",
                        "wasm",
                        "--out-dir",
                        "./static",
                        "--no-typescript",
                        #[cfg(debug_assertions)]
                        "--debug",
                    ])
                    .current_dir("birdseye-frontend")
                    .spawn()
                {
                    Ok(mut chld) => match chld.wait() {
                        Ok(_) => {}
                        Err(err) => {
                            error!("wasm-pack exited abnormally, {err}")
                        }
                    },
                    Err(err) => {
                        error!("Couldn't run wasm_pack {err}");
                    }
                };
                info!("Recompiling birdseye-frontend");
            }
            "birdseye-common" => {
                info!("Recompiling birdseye-common dependent crates");
                let mut dependent = HashSet::new();

                dependent.insert("birdseye-frontend".to_string());
                dependent.insert("birdseye-server".into());

                compile_crates(&dependent);

                info!("Recompiled all crates dependent on birdseye-frontend")
            }
            _ => {
                info!("Recompiling {crate_}");
                match Command::new("cargo")
                    .args([
                        "build",
                        "--package",
                        crate_.as_str(),
                        #[cfg(not(debug_assertions))]
                        "--release",
                    ])
                    .spawn()
                {
                    Ok(mut chld) => match chld.wait() {
                        Ok(_) => {}
                        Err(err) => {
                            error!("Cargo exited abnormally, {err}")
                        }
                    },
                    Err(err) => {
                        error!("Couldn't run cargo {err}");
                    }
                };

                info!("Recompiled {crate_}");
            }
        }
    }
}
