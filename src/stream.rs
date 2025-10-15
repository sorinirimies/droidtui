use minifb::{Key, Window, WindowOptions};
use std::io::{BufReader, Read};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Configuration for screen streaming
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Window width
    pub width: usize,
    /// Window height
    pub height: usize,
    /// Video bitrate
    pub bitrate: String,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            width: 1080,
            height: 1920,
            bitrate: "8M".to_string(),
        }
    }
}

/// Stream control messages
pub enum StreamControl {
    Stop,
}

/// Stream state
#[derive(Debug)]
pub struct StreamState {
    adb_process: Option<Child>,
    ffmpeg_process: Option<Child>,
    control_tx: Option<Sender<StreamControl>>,
    stream_thread: Option<thread::JoinHandle<()>>,
}

impl Default for StreamState {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamState {
    pub fn new() -> Self {
        Self {
            adb_process: None,
            ffmpeg_process: None,
            control_tx: None,
            stream_thread: None,
        }
    }

    pub fn stop(&mut self) {
        // Send stop signal
        if let Some(tx) = self.control_tx.take() {
            let _ = tx.send(StreamControl::Stop);
        }

        // Wait for thread to finish
        if let Some(handle) = self.stream_thread.take() {
            let _ = handle.join();
        }

        // Kill processes
        if let Some(mut proc) = self.adb_process.take() {
            let _ = proc.kill();
        }
        if let Some(mut proc) = self.ffmpeg_process.take() {
            let _ = proc.kill();
        }
    }
}

impl Drop for StreamState {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Start screen streaming in a separate window
pub fn start_stream(config: StreamConfig) -> Result<StreamState, String> {
    let (control_tx, control_rx) = channel();

    // Spawn streaming thread
    let stream_thread = thread::spawn(move || {
        if let Err(e) = run_stream(config, control_rx) {
            eprintln!("Stream error: {}", e);
        }
    });

    Ok(StreamState {
        adb_process: None,
        ffmpeg_process: None,
        control_tx: Some(control_tx),
        stream_thread: Some(stream_thread),
    })
}

/// Main streaming loop
fn run_stream(config: StreamConfig, control_rx: Receiver<StreamControl>) -> Result<(), String> {
    // Get device resolution
    let resolution = get_device_resolution()?;

    // Use device resolution or config
    let (width, height) = resolution;

    // Create window
    let mut window = Window::new(
        "DroidTUI - Screen Stream",
        width,
        height,
        WindowOptions {
            resize: true,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .map_err(|e| format!("Failed to create window: {}", e))?;

    // Limit update rate
    window.set_target_fps(60);

    // Start ADB screen recording
    let mut adb_child = Command::new("adb")
        .args([
            "exec-out",
            "screenrecord",
            "--output-format=h264",
            "--size",
            &format!("{}x{}", width, height),
            "--bit-rate",
            &config.bitrate,
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start screenrecord: {}", e))?;

    let stdout = adb_child.stdout.take().ok_or("Failed to get stdout")?;

    // Start FFmpeg to decode H.264
    let mut ffmpeg_child = Command::new("ffmpeg")
        .args([
            "-f", "h264", "-i", "pipe:0", "-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start ffmpeg: {}", e))?;

    let mut ffmpeg_stdin = ffmpeg_child
        .stdin
        .take()
        .ok_or("Failed to get ffmpeg stdin")?;

    let ffmpeg_stdout = ffmpeg_child
        .stdout
        .take()
        .ok_or("Failed to get ffmpeg stdout")?;

    // Thread to pipe ADB output to FFmpeg
    let pipe_thread = thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut buffer = vec![0u8; 8192];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if std::io::Write::write_all(&mut ffmpeg_stdin, &buffer[..n]).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Main display loop
    let frame_size = width * height * 3; // RGB24
    let mut frame_buffer = vec![0u8; frame_size];
    let mut display_buffer: Vec<u32> = vec![0; width * height];
    let mut reader = BufReader::new(ffmpeg_stdout);

    loop {
        // Check for control messages
        if let Ok(StreamControl::Stop) = control_rx.try_recv() {
            break;
        }

        // Check if window is still open
        if !window.is_open() || window.is_key_down(Key::Escape) || window.is_key_down(Key::Q) {
            break;
        }

        // Read frame from FFmpeg
        match reader.read_exact(&mut frame_buffer) {
            Ok(_) => {
                // Convert RGB24 to ARGB32 for minifb
                for i in 0..(width * height) {
                    let r = frame_buffer[i * 3] as u32;
                    let g = frame_buffer[i * 3 + 1] as u32;
                    let b = frame_buffer[i * 3 + 2] as u32;
                    display_buffer[i] = (r << 16) | (g << 8) | b;
                }

                // Update window
                window
                    .update_with_buffer(&display_buffer, width, height)
                    .map_err(|e| format!("Failed to update window: {}", e))?;
            }
            Err(_) => {
                // No frame available, just update window
                window.update();
                thread::sleep(Duration::from_millis(10));
            }
        }
    }

    // Cleanup
    let _ = adb_child.kill();
    let _ = ffmpeg_child.kill();
    let _ = pipe_thread.join();

    Ok(())
}

/// Get device screen resolution
fn get_device_resolution() -> Result<(usize, usize), String> {
    let output = Command::new("adb")
        .args(["shell", "wm", "size"])
        .output()
        .map_err(|e| format!("Failed to get device resolution: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse output like "Physical size: 1080x1920"
    for line in stdout.lines() {
        if line.contains("Physical size:") || line.contains("Override size:") {
            if let Some(size_part) = line.split(':').nth(1) {
                let size_str = size_part.trim();
                if let Some((w, h)) = size_str.split_once('x') {
                    if let (Ok(width), Ok(height)) = (w.parse(), h.parse()) {
                        return Ok((width, height));
                    }
                }
            }
        }
    }

    // Default fallback
    Ok((1080, 1920))
}

/// Quick screenshot using existing method (for fallback)
pub async fn capture_screenshot() -> Result<Vec<u8>, String> {
    let output = tokio::process::Command::new("adb")
        .args(["exec-out", "screencap", "-p"])
        .output()
        .await
        .map_err(|e| format!("Failed to capture screenshot: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("screencap failed: {}", stderr));
    }

    if output.stdout.is_empty() {
        return Err("No screenshot data received".to_string());
    }

    Ok(output.stdout)
}
