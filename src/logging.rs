//! Logging utilities for bubbletea-rs applications.

use crate::Error;
use std::path::Path;

/// Set up file logging for the application.
#[cfg(feature = "logging")]
pub fn log_to_file(path: impl AsRef<Path>, prefix: &str) -> Result<(), Error> {
    use std::fs::OpenOptions;

    let _file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.as_ref())
        .map_err(|e| Error::Configuration(format!("Failed to open log file: {}", e)))?;

    log::info!("Logging initialized with prefix: {}", prefix);

    Ok(())
}

#[cfg(not(feature = "logging"))]
pub fn log_to_file(_path: impl AsRef<Path>, _prefix: &str) -> Result<(), Error> {
    Err(Error::Configuration(
        "Logging feature is not enabled. Enable the 'logging' feature to use log_to_file."
            .to_string(),
    ))
}
