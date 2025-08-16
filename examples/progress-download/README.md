# Progress Download

A real-world demonstration of HTTP file downloads with animated progress bars, showing how to build practical applications that track long-running operations with real-time progress feedback and error handling.

## Features

- **Real HTTP Downloads**: Downloads actual files from URLs with streaming
- **Animated Progress Bar**: Smooth gradient progress visualization
- **Command-line Interface**: URL specification via command-line arguments
- **Error Handling**: Graceful handling of network errors and failures
- **File Size Detection**: Progress based on actual content-length headers
- **Responsive Layout**: Progress bar adapts to terminal width
- **Auto-completion**: Program exits automatically when download completes

## Running the Example

From the repository root:

```bash
# Download a file with progress tracking
cargo run --example progress-download -- --url https://httpbin.org/bytes/1048576

# Download a real file
cargo run --example progress-download -- --url https://github.com/microsoft/vscode/archive/refs/heads/main.zip

# Test with smaller file
cargo run --example progress-download -- --url https://httpbin.org/bytes/102400
```

**Controls:**
- `q` / `Ctrl+C` - Cancel download and quit
- Progress updates automatically as download proceeds

## What this demonstrates

### Key Concepts for Beginners

**Real-world Progress Tracking**: This example shows how to:
1. Integrate progress bars with actual background operations
2. Stream large files without loading everything into memory
3. Handle network operations with proper error management
4. Provide user feedback during long-running operations
5. Build command-line tools with professional interfaces

**Async Integration**: Demonstrates coordinating UI updates with background async operations like HTTP requests.

### Public API Usage

**Core Framework:**
```rust
use bubbletea_rs::{batch, quit, sequence, tick, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
```

- Background async operations with progress reporting
- Message-based communication between UI and download task
- Responsive UI updates during long operations

**HTTP Client:**
```rust
use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
```

- Streaming HTTP downloads with reqwest
- Async file writing with tokio
- Progress tracking during streaming

**Command Line Parsing:**
```rust
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,
}
```

### Architecture Walkthrough

#### Download Task Structure

```rust
async fn download_file(url: String, tx: mpsc::Sender<ProgressMsg>) -> Result<String, String> {
    let response = reqwest::get(&url).await
        .map_err(|e| format!("Failed to start download: {}", e))?;
    
    let total_size = response.content_length()
        .ok_or_else(|| "Unable to get file size".to_string())?;
    
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();
    let mut file = File::create(&filename).await
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Download error: {}", e))?;
        
        file.write_all(&chunk).await
            .map_err(|e| format!("Write error: {}", e))?;
        
        downloaded += chunk.len() as u64;
        let progress = downloaded as f64 / total_size as f64;
        
        // Send progress update to UI
        let _ = tx.send(ProgressMsg(progress)).await;
    }
    
    Ok(filename)
}
```

#### Message Types

```rust
#[derive(Debug, Clone)]
pub struct ProgressMsg(pub f64);  // Progress from 0.0 to 1.0

#[derive(Debug)]
pub struct ProgressErrMsg {
    pub error: String,  // Error message for display
}

#[derive(Debug)]
pub struct FinalPauseMsg;  // Brief pause before auto-quit
```

#### Model State

```rust
pub struct ProgressDownloadModel {
    pub url: String,              // Download URL
    pub progress: f64,            // Current progress (0.0-1.0)
    pub downloaded_filename: String,  // Output filename
    pub error: Option<String>,    // Error message if any
    pub complete: bool,           // Download completion flag
    pub width: usize,             // Progress bar width
}
```

### Rust-Specific Patterns

**Streaming Downloads:**
```rust
let mut stream = response.bytes_stream();
while let Some(chunk) = stream.next().await {
    let chunk = chunk.map_err(|e| format!("Download error: {}", e))?;
    // Process chunk without loading entire file into memory
}
```

**Channel Communication:**
```rust
let (tx, mut rx) = mpsc::channel::<ProgressMsg>(100);

// Spawn download task
tokio::spawn(async move {
    match download_file(url, tx.clone()).await {
        Ok(filename) => {
            let _ = tx.send(ProgressMsg(1.0)).await;  // 100% complete
        }
        Err(e) => {
            let _ = tx.send(ProgressErrMsg { error: e }).await;
        }
    }
});

// Listen for progress updates in main loop
while let Ok(msg) = rx.try_recv() {
    // Forward to UI via command
}
```

**Error Propagation:**
```rust
let response = reqwest::get(&url).await
    .map_err(|e| format!("Failed to start download: {}", e))?;
```

Convert errors to user-friendly strings.

**File Path Handling:**
```rust
fn filename_from_url(url: &str) -> String {
    Path::new(url)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download")
        .to_string()
}
```

Extract filename from URL safely.

### Progress Visualization

```rust
pub fn view_progress(&self) -> String {
    let percent = (self.progress * 100.0).round() as u32;
    let filled_width = (self.width as f64 * self.progress).round() as usize;
    let empty_width = self.width.saturating_sub(filled_width);
    
    // Use gradient helper for consistent styling
    let filled = gradient_filled_segment(filled_width, '█');
    let empty = "░".repeat(empty_width);
    
    format!("{}{} {}%", filled, empty, percent)
}
```

### Network Error Handling

```rust
pub enum DownloadError {
    NetworkError(String),
    FileSystemError(String),
    InvalidUrl(String),
    NoContentLength,
}

impl From<reqwest::Error> for DownloadError {
    fn from(err: reqwest::Error) -> Self {
        DownloadError::NetworkError(err.to_string())
    }
}

impl From<tokio::io::Error> for DownloadError {
    fn from(err: tokio::io::Error) -> Self {
        DownloadError::FileSystemError(err.to_string())
    }
}
```

### Real-world Applications

**Software Updater:**
```rust
struct AppUpdater {
    current_version: String,
    download_progress: f64,
    update_url: String,
}

impl AppUpdater {
    async fn check_for_updates(&self) -> Result<Option<UpdateInfo>, Error> {
        // Check API for new version
        // Return update info if available
    }
    
    async fn download_update(&mut self, update: UpdateInfo) -> Result<PathBuf, Error> {
        // Download update with progress tracking
        // Show progress in UI
    }
}
```

**Backup Tool:**
```rust
struct BackupManager {
    files_to_backup: Vec<PathBuf>,
    current_file: usize,
    file_progress: f64,
    total_progress: f64,
}

impl BackupManager {
    async fn backup_files(&mut self) -> Result<(), Error> {
        for (i, file) in self.files_to_backup.iter().enumerate() {
            self.current_file = i;
            self.upload_file_with_progress(file).await?;
            self.total_progress = (i + 1) as f64 / self.files_to_backup.len() as f64;
        }
    }
}
```

**Package Manager:**
```rust
struct PackageDownloader {
    packages: Vec<Package>,
    download_queue: Vec<DownloadTask>,
    concurrent_downloads: usize,
}

impl PackageDownloader {
    async fn download_packages(&mut self) -> Result<(), Error> {
        // Download multiple packages with progress tracking
        // Show aggregate progress across all downloads
    }
}
```

### Performance Considerations

**Streaming Benefits:**
- Memory usage stays constant regardless of file size
- Progress updates are smooth and frequent
- Can handle multi-gigabyte files efficiently

**Channel Buffering:**
```rust
let (tx, rx) = mpsc::channel::<ProgressMsg>(100);  // Buffer 100 messages
```

Prevents UI blocking during rapid progress updates.

**Chunk Size Optimization:**
```rust
// reqwest uses optimal chunk sizes automatically
// For custom implementations, use 8KB-64KB chunks
const CHUNK_SIZE: usize = 32 * 1024;  // 32KB chunks
```

### Error Recovery

```rust
async fn download_with_retry(url: String, max_retries: u32) -> Result<String, String> {
    for attempt in 1..=max_retries {
        match download_file(url.clone(), tx.clone()).await {
            Ok(filename) => return Ok(filename),
            Err(e) if attempt < max_retries => {
                eprintln!("Attempt {} failed: {}. Retrying...", attempt, e);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
            Err(e) => return Err(format!("All {} attempts failed. Last error: {}", max_retries, e)),
        }
    }
    unreachable!()
}
```

### Testing Download Functionality

```bash
# Test with different file sizes
cargo run --example progress-download -- --url https://httpbin.org/bytes/1024      # 1KB
cargo run --example progress-download -- --url https://httpbin.org/bytes/1048576   # 1MB
cargo run --example progress-download -- --url https://httpbin.org/bytes/10485760  # 10MB

# Test error conditions
cargo run --example progress-download -- --url https://invalid-url.example.com/file.zip
cargo run --example progress-download -- --url https://httpbin.org/status/404

# Test network timeouts
cargo run --example progress-download -- --url https://httpbin.org/delay/30
```

### Command Line Integration

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Validate URL
    if !args.url.starts_with("http://") && !args.url.starts_with("https://") {
        eprintln!("Error: URL must start with http:// or https://");
        std::process::exit(1);
    }
    
    let program = Program::<ProgressDownloadModel>::builder()
        .build()?;
    
    let model = ProgressDownloadModel::new(args.url);
    program.run_with_model(model).await?;
    
    Ok(())
}
```

## Related Examples

- **[progress-animated](../progress-animated/)** - Progress bar animation techniques
- **[progress-static](../progress-static/)** - Simpler progress bar implementation  
- **[send-msg](../send-msg/)** - Background task communication patterns
- **[realtime](../realtime/)** - Real-time updates and async coordination

## Files

- `main.rs` — Complete HTTP download with progress tracking
- `Cargo.toml` — Dependencies including reqwest, clap, futures-util
- `README.md` — This documentation

## Usage Tips

- Test with different file sizes to see progress behavior
- Use `--url` parameter to specify any HTTP/HTTPS download URL
- Progress bar adapts to terminal width automatically
- Downloads are saved to current directory with filename from URL
- Cancel downloads with Ctrl+C - partial files will remain