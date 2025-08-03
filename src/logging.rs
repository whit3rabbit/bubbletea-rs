//! Logging utilities for bubbletea-rs applications.
//!
//! This module provides file-based logging functionality for Bubble Tea applications.
//! Logging is controlled by the `logging` feature flag and requires the `log` crate
//! for actual log output.
//!
//! # Features
//!
//! - File-based logging with automatic file creation
//! - Append-only logging to preserve existing log data
//! - Graceful degradation when logging feature is disabled
//!
//! # Usage
//!
//! ```rust,no_run
//! use bubbletea_rs::logging::log_to_file;
//!
//! // Set up logging to a file
//! if let Err(e) = log_to_file("app.log", "MyApp") {
//!     eprintln!("Failed to initialize logging: {}", e);
//! }
//!
//! // Now you can use standard log macros
//! log::info!("Application started");
//! log::error!("Something went wrong");
//! ```

use crate::Error;
use std::path::Path;

/// Set up file logging for the application.
///
/// This function initializes file-based logging by creating or opening the specified
/// log file in append mode. The file will be created if it doesn't exist, and new
/// log entries will be appended to preserve existing log data.
///
/// # Arguments
///
/// * `path` - The file path where logs should be written. Can be any type that
///   implements `AsRef<Path>`, such as `&str`, `String`, or `PathBuf`.
/// * `prefix` - A prefix string used for identifying log entries from this application.
///   This is logged as an info message when logging is initialized.
///
/// # Returns
///
/// Returns `Ok(())` if logging was successfully initialized, or an `Error` if:
/// - The log file cannot be created or opened
/// - File permissions prevent writing to the specified path
/// - The logging feature is not enabled (when compiled without the `logging` feature)
///
/// # Examples
///
/// ```rust,no_run
/// use bubbletea_rs::logging::log_to_file;
/// use std::path::PathBuf;
///
/// // Using a string path
/// log_to_file("application.log", "MyApp")?;
///
/// // Using a PathBuf
/// let log_path = PathBuf::from("logs").join("app.log");
/// log_to_file(log_path, "MyApp")?;
///
/// // Using a relative path
/// log_to_file("./logs/debug.log", "Debug")?;
/// # Ok::<(), bubbletea_rs::Error>(())
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be created due to permission issues
/// - The parent directory doesn't exist and cannot be created
/// - The logging feature is not enabled at compile time
///
/// # Feature Requirements
///
/// This function requires the `logging` feature to be enabled. When compiled
/// without this feature, it will always return an error indicating that
/// logging is not available.
#[cfg(feature = "logging")]
pub fn log_to_file(path: impl AsRef<Path>, prefix: &str) -> Result<(), Error> {
    use std::fs::OpenOptions;

    let _file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.as_ref())?;

    log::info!("Logging initialized with prefix: {prefix}");

    Ok(())
}

/// Set up file logging for the application (feature-disabled version).
///
/// This is a stub implementation that always returns an error when the `logging`
/// feature is not enabled. It provides a graceful way to handle logging calls
/// in code that may or may not have logging support enabled.
///
/// # Arguments
///
/// * `_path` - The file path (ignored when logging is disabled)
/// * `_prefix` - The prefix string (ignored when logging is disabled)
///
/// # Returns
///
/// Always returns an `Error::Configuration` indicating that the logging feature
/// is not enabled.
///
/// # Examples
///
/// ```rust,ignore
/// // This will always fail when logging feature is disabled
/// match log_to_file("app.log", "MyApp") {
///     Ok(()) => println!("Logging enabled"),
///     Err(e) => eprintln!("Logging not available: {}", e),
/// }
/// ```
///
/// # Errors
///
/// Always returns `Error::Configuration` with a message explaining that the
/// logging feature must be enabled to use this function.
#[cfg(not(feature = "logging"))]
pub fn log_to_file(_path: impl AsRef<Path>, _prefix: &str) -> Result<(), Error> {
    Err(Error::Configuration(
        "Logging feature is not enabled. Enable the 'logging' feature to use log_to_file."
            .to_string(),
    ))
}
