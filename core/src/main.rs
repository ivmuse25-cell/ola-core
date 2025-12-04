// src/main.rs
pub mod camera_worker;
pub mod camera;
mod secure_store;

use camera_worker::{CameraWorker, CameraRequest};
use tokio::sync::{mpsc, oneshot};

use tokio::net::{UnixListener, UnixStream};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use log::{info, error, warn};
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio_util::codec::{Framed, LinesCodec};
use futures::stream::StreamExt;
use futures::sink::SinkExt;
use tokio::signal::unix::{signal, SignalKind};
use listenfd::ListenFd;
use users::get_user_by_name;
use anyhow::Context;

use nix::sys::socket::{getsockopt, sockopt::PeerCredentials, UnixCredentials};

const SOCKET_PATH: &str = "/run/ola/ola.sock"; // systemd /run path

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    id: Option<u64>,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    id: Option<u64>,
    result: Option<serde_json::Value>,
    error: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Starting ola-core...");

    // Start Camera Worker
    // This spawns a dedicated thread for blocking camera operations.
    // We clone the sender (worker_tx) for each client connection.
    let (worker, worker_tx) = CameraWorker::new();
    let worker_handle = worker.run();

    // Allow overriding socket path (useful for dev/testing without root)
    let socket_path_str = std::env::var("OLA_SOCKET_PATH").unwrap_or_else(|_| SOCKET_PATH.to_string());
    let socket_path = Path::new(&socket_path_str);

    // Ensure parent directory exists (safe no-op if systemd manages it)
    if let Some(parent) = socket_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).ok();
        }
    }

    // Limit concurrent connections
    let max_conns = Arc::new(Semaphore::new(16));

    // Socket Activation: prefer systemd-provided FD
    let mut listenfd = ListenFd::from_env();
    let listener: UnixListener = if let Some(std_listener) = listenfd.take_unix_listener(0)? {
        info!("Using systemd socket activation");
        UnixListener::from_std(std_listener)?
    } else {
        // Safety: Only bind manually if explicitly in dev mode
        let mode = std::env::var("OLA_RUNMODE").unwrap_or_else(|_| "prod".to_string());
        if mode != "dev" {
            // In production, we strictly require systemd socket activation OR the allowlist file to exist (as a sanity check for the environment)
            if !Path::new("/etc/ola/allowlist").exists() {
                 error!("/etc/ola/allowlist missing in prod. Exiting.");
                 anyhow::bail!("/etc/ola/allowlist missing in production mode");
            }
            anyhow::bail!("No systemd socket found and OLA_RUNMODE != 'dev'. Refusing to bind manually in production.");
        }

        info!("Binding to socket path manually (DEV MODE): {}", socket_path.display());

        // Safer stale socket removal
        remove_stale_socket(socket_path);

        let l = UnixListener::bind(socket_path).context("binding socket")?;

        // If we're root, try to chown socket to 'ola' user
        if nix::unistd::geteuid().is_root() {
            if let Some(ola_user) = get_user_by_name("ola") {
                let uid = ola_user.uid();
                let gid = ola_user.primary_group_id();
                if let Err(e) = nix::unistd::chown(socket_path, Some(nix::unistd::Uid::from_raw(uid)), Some(nix::unistd::Gid::from_raw(gid))) {
                    warn!("Failed to chown socket to ola: {}", e);
                }
            }
        }

        // Set permissions
        if let Ok(mut perms) = fs::metadata(socket_path).map(|m| m.permissions()) {
            perms.set_mode(0o770);
            fs::set_permissions(socket_path, perms).ok();
        }

        l
    };

    info!("Listening on {}", socket_path.display());

    // Shutdown handler
    let shutdown = async {
        let mut sigint = signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");
        let mut sigterm = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = sigint.recv() => info!("SIGINT received"),
            _ = sigterm.recv() => info!("SIGTERM received"),
        }
    };

    tokio::select! {
        _ = async {
            loop {
                let (stream, _addr) = match listener.accept().await {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Accept error: {}", e);
                        continue;
                    }
                };

                let permit = max_conns.clone().acquire_owned().await.unwrap();
                let socket_path_clone = socket_path_str.clone();
                let worker_tx = worker_tx.clone();

                tokio::spawn(async move {
                    let _permit = permit; // release when task finishes
                    if let Err(e) = timeout(Duration::from_secs(20), handle_client(stream, socket_path_clone, worker_tx)).await {
                        error!("Client handling timed out or errored: {:?}", e);
                    }
                });
            }
        } => (),
        _ = shutdown => {
            info!("Shutting down listener...");
        }
    }

    // --- Graceful Shutdown ---
    info!("Signal received, starting graceful shutdown...");

    // 1. Stop accepting new requests (implicit by dropping the loop, but here we break the loop)
    // In a real server, we might want to stop the listener first.
    // For now, we just drop the worker sender, which signals the worker to stop after processing its queue.
    drop(worker_tx);

    // 2. Wait for worker to drain with timeout
    let shutdown_timeout = std::env::var("OLA_SHUTDOWN_TIMEOUT")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u64>()
        .unwrap_or(5);

    info!("Waiting up to {}s for camera worker to drain...", shutdown_timeout);

    let join_handle = tokio::task::spawn_blocking(move || {
        worker_handle.join()
    });

    match tokio::time::timeout(std::time::Duration::from_secs(shutdown_timeout), join_handle).await {
        Ok(Ok(_)) => {
            info!("Camera worker shut down successfully.");
        }
        Ok(Err(e)) => {
            error!("Camera worker thread panicked: {:?}", e);
        }
        Err(_) => {
            error!("Shutdown timed out! Forcing exit.");
            // In a real scenario, we might want to abort the thread if possible, 
            // but std::thread cannot be cancelled easily. 
            // We just exit the process, which kills the thread.
        }
    }

    info!("Ola Core service stopped.");
    // NOTE: we intentionally skip removing socket if systemd owns it.
    Ok(())
}

use std::os::unix::net::UnixStream as StdUnixStream;
use std::io::ErrorKind;

fn remove_stale_socket(path: &Path) {
    if !path.exists() { return; }
    match StdUnixStream::connect(path) {
        Ok(_) => {
            // Someone is listening — do not remove.
            info!("Socket {} is active, not removing.", path.display());
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::ConnectionRefused | ErrorKind::NotFound => {
                    // Likely stale: safe to remove
                    info!("Removing stale socket {} (connect error: {})", path.display(), e);
                    let _ = fs::remove_file(path);
                },
                _ => {
                    info!("Socket {} connection error {:?}; not removing", path.display(), e);
                }
            }
        }
    }
}

fn check_allowlist(creds: &UnixCredentials) -> bool {
    // 1. Allow root
    if creds.uid() == 0 { return true; }

    // 2. Allow the user running the service
    if creds.uid() == nix::unistd::getuid().as_raw() { return true; }

    // 3. Explicit Allowlist File (/etc/ola/allowlist)
    if let Ok(content) = fs::read_to_string("/etc/ola/allowlist") {
        for line in content.lines() {
            // Remove comments and whitespace
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() { continue; }
            
            if let Ok(allowed_uid) = line.parse::<u32>() {
                if creds.uid() == allowed_uid { return true; }
            }
        }
    }

    // 4. Group-based policy removed for security (Sprint 1 Hardening).
    // We rely on explicit allowlist or root/service-user access.
    
    false
}

async fn handle_client(stream: UnixStream, socket_path: String, worker_tx: mpsc::Sender<CameraRequest>) -> anyhow::Result<()> {
    // Security: Check Peer Credentials (SO_PEERCRED)
    // Note: getsockopt expects a type implementing AsFd. UnixStream implements AsFd.
    // Defensive: Handle getsockopt errors gracefully (e.g. abstract sockets, activation quirks)
    let creds = match getsockopt(&stream, PeerCredentials) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to get peer credentials: {}", e);
            // Return Ok(()) to drop connection gracefully without crashing the handler
            return Ok(());
        }
    };

    info!("Incoming connection: uid={}, gid={}, pid={}", creds.uid(), creds.gid(), creds.pid());

    if !check_allowlist(&creds) {
        error!("Rejecting connection from unauthorized UID {}", creds.uid());
        // Early return; connection drops when we drop stream
        return Ok(());
    }

    let mut framed = Framed::new(stream, LinesCodec::new());

    while let Some(line_result) = framed.next().await {
        match line_result {
            Ok(line) => {
                // Defensive: reject excessively large lines
                const MAX_LINE_BYTES: usize = 512 * 1024; // 512 KB
                if line.len() > MAX_LINE_BYTES {
                    error!("Rejecting overlong line ({} bytes)", line.len());
                    let resp = Response { id: None, result: None, error: Some("Payload too large".into()) };
                    let resp_str = serde_json::to_string(&resp)?;
                    if let Err(e) = framed.send(resp_str).await {
                        error!("Failed to send response: {}", e);
                        break;
                    }
                    continue;
                }

                info!("Processing request: {}", line);

                // 1. Parse Request first to determine method
                let req: Request = match serde_json::from_str(&line) {
                    Ok(r) => r,
                    Err(e) => {
                        let resp = Response {
                            id: None,
                            result: None,
                            error: Some(format!("Invalid JSON: {}", e)),
                        };
                        let resp_str = serde_json::to_string(&resp)?;
                        if let Err(e) = framed.send(resp_str).await {
                            error!("Failed to send parse error: {}", e);
                            break;
                        }
                        continue;
                    }
                };

                // 2. Determine Timeout based on method
                let timeout_duration = match req.method.as_str() {
                    "capture_thumbnail" => Duration::from_secs(15), // Camera ops need more time
                    "verify_once" => Duration::from_secs(10),       // Verification might take time
                    _ => Duration::from_secs(5),                    // Fast ops (ping, status)
                };

                // 3. Process with dynamic timeout
                let process = async {
                    match req.method.as_str() {
                        "ping" => Response {
                            id: req.id,
                            result: Some(serde_json::json!({"ok": true, "version": env!("CARGO_PKG_VERSION")})),
                            error: None,
                        },
                        "list_cameras" => {
                            let (tx, rx) = oneshot::channel();
                            if let Err(e) = worker_tx.send(CameraRequest::ListCameras(tx)).await {
                                return Response { id: req.id, result: None, error: Some(format!("Worker died: {}", e)) };
                            }
                            match rx.await {
                                Ok(cameras) => Response {
                                    id: req.id,
                                    result: Some(serde_json::json!(cameras)),
                                    error: None,
                                },
                                Err(_) => Response {
                                    id: req.id,
                                    result: None,
                                    error: Some("Worker dropped response".into()),
                                }
                            }
                        },
                        "capture_thumbnail" => {
                            let index = req.params
                                .and_then(|p| p.get("index").and_then(|i| i.as_u64()))
                                .map(|i| i as usize)
                                .unwrap_or(0);

                            let (tx, rx) = oneshot::channel();
                            if let Err(e) = worker_tx.send(CameraRequest::CaptureThumbnail(index, tx)).await {
                                return Response { id: req.id, result: None, error: Some(format!("Worker died: {}", e)) };
                            }

                            match rx.await {
                                Ok(Ok(base64_img)) => Response {
                                    id: req.id,
                                    result: Some(serde_json::json!({ "image": base64_img })),
                                    error: None,
                                },
                                Ok(Err(e)) => Response {
                                    id: req.id,
                                    result: None,
                                    error: Some(format!("Capture error: {}", e)),
                                },
                                Err(_) => Response {
                                    id: req.id,
                                    result: None,
                                    error: Some("Worker dropped response".into()),
                                }
                            }
                        },
                        "verify_once" => {
                            let (tx, rx) = oneshot::channel();
                            if let Err(e) = worker_tx.send(CameraRequest::VerifyOnce(2000, tx)).await {
                                return Response { id: req.id, result: None, error: Some(format!("Worker died: {}", e)) };
                            }

                            match rx.await {
                                Ok(Ok(res)) => Response {
                                    id: req.id,
                                    result: Some(serde_json::json!(res)),
                                    error: None,
                                },
                                Ok(Err(e)) => Response {
                                    id: req.id,
                                    result: None,
                                    error: Some(format!("Verification error: {}", e)),
                                },
                                Err(_) => Response {
                                    id: req.id,
                                    result: None,
                                    error: Some("Worker dropped response".into()),
                                }
                            }
                        },
                        "status" => Response {
                            id: req.id,
                            result: Some(serde_json::json!({
                                "status": "running",
                                "version": env!("CARGO_PKG_VERSION"),
                                "backend": "stubbed",
                                    "socket": socket_path
                            })),
                            error: None,
                        },
                        _ => Response {
                            id: req.id,
                            result: None,
                            error: Some(format!("Unknown method: {}", req.method)),
                        }
                    }
                };

                match timeout(timeout_duration, process).await {
                    Ok(response) => {
                        let resp_str = serde_json::to_string(&response)?;
                        if let Err(e) = framed.send(resp_str).await {
                            error!("Failed to send response to client: {}", e);
                            break;
                        }
                    }
                    Err(_) => {
                        error!("Request processing timed out");
                        let resp = Response { id: req.id, result: None, error: Some("Request timed out".into()) };
                        let resp_str = serde_json::to_string(&resp)?;
                        if let Err(e) = framed.send(resp_str).await {
                            error!("Failed to send timeout response: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                error!("Framing error: {}", e);
                return Err(anyhow::anyhow!("Framing error: {}", e));
            }
        }
    }

    Ok(())
}
