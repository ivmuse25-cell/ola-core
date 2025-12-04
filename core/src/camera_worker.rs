use tokio::sync::{mpsc, oneshot};
use std::thread;
use super::camera;

#[derive(Debug)]
pub enum CameraRequest {
    ListCameras(oneshot::Sender<Vec<camera::CameraInfo>>),
    CaptureThumbnail(usize, oneshot::Sender<anyhow::Result<String>>),
    VerifyOnce(u64, oneshot::Sender<anyhow::Result<camera::VerificationResult>>),
}

pub struct CameraWorker {
    receiver: mpsc::Receiver<CameraRequest>,
}

impl CameraWorker {
    pub fn new() -> (Self, mpsc::Sender<CameraRequest>) {
        // Buffer size 32 is plenty for now
        let (tx, rx) = mpsc::channel(32);
        (Self { receiver: rx }, tx)
    }

    pub fn run(mut self) -> thread::JoinHandle<()> {
        // Spawn a dedicated OS thread for blocking camera operations
        thread::spawn(move || {
            // blocking_recv() waits until a message is available or channel is closed
            while let Some(req) = self.receiver.blocking_recv() {
                match req {
                    CameraRequest::ListCameras(tx) => {
                        let res = camera::list_cameras();
                        let _ = tx.send(res);
                    }
                    CameraRequest::CaptureThumbnail(idx, tx) => {
                        let res = camera::capture_thumbnail(idx);
                        let _ = tx.send(res);
                    }
                    CameraRequest::VerifyOnce(timeout, tx) => {
                        let res = camera::verify_once(timeout);
                        let _ = tx.send(res);
                    }
                }
            }
            // Loop ends when Sender is dropped (main thread shutdown)
        })
    }
}
