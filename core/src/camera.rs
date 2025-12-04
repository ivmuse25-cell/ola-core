use glob::glob;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraInfo {
    pub path: String,
    pub name: String,
    pub index: usize,
}

pub fn list_cameras() -> Vec<CameraInfo> {
    let mut cameras = Vec::new();
    
    // Simple glob-based probing that doesn't require native libraries (v4l/opencv)
    // This ensures the core compiles and runs even if system deps are missing.
    let pattern = "/dev/video*";
    
    if let Ok(paths) = glob(pattern) {
        for entry in paths {
            if let Ok(path) = entry {
                let path_str = path.to_string_lossy().to_string();
                
                // Extract index
                let index_str = path_str.trim_start_matches("/dev/video");
                if let Ok(index) = index_str.parse::<usize>() {
                    // In this stub mode, we can't query the device name, so we use a generic one.
                    // Later, when v4l/opencv is enabled, we will get the real name.
                    let name = format!("Camera Device {}", index);
                    
                    cameras.push(CameraInfo {
                        path: path_str,
                        name,
                        index,
                    });
                }
            }
        }
    }
    
    cameras
}

pub fn capture_thumbnail(_index: usize) -> anyhow::Result<String> {
    // STUB: Return a placeholder base64 image (a small red dot)
    // This proves we can transmit "large" strings over IPC without crashing.
    // Real implementation will use OpenCV to capture and encode.
    let placeholder = "iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==";
    Ok(placeholder.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub ok: bool,
    pub reason: Option<String>,
}

pub fn verify_once(_timeout_ms: u64) -> anyhow::Result<VerificationResult> {
    // STUB: Simulate a verification attempt.
    // For now, we'll just sleep a bit and return true to simulate success.
    // In the future, this will capture frames and run the ONNX model.
    
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Hardcoded success for testing flow
    Ok(VerificationResult {
        ok: true,
        reason: None,
    })
}
