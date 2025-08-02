use bubbletea_rs::Error;
use std::io;
use tokio::sync::mpsc;

#[test]
fn test_error_display() {
    let panic_err = Error::ProgramPanic("test panic".to_string());
    assert_eq!(panic_err.to_string(), "Program panic: test panic");

    let killed_err = Error::ProgramKilled;
    assert_eq!(killed_err.to_string(), "Program was killed");

    let interrupted_err = Error::Interrupted;
    assert_eq!(interrupted_err.to_string(), "Program was interrupted");

    let io_err = Error::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    assert!(io_err.to_string().contains("I/O error"));

    let terminal_err = Error::Terminal("terminal error".to_string());
    assert_eq!(terminal_err.to_string(), "Terminal error: terminal error");

    let channel_send_err = Error::ChannelSend;
    assert_eq!(channel_send_err.to_string(), "Channel send error");

    let channel_recv_err = Error::ChannelReceive;
    assert_eq!(channel_recv_err.to_string(), "Channel receive error");

    let config_err = Error::Configuration("config error".to_string());
    assert_eq!(config_err.to_string(), "Configuration error: config error");

    let cmd_err = Error::CommandExecution("command failed".to_string());
    assert_eq!(
        cmd_err.to_string(),
        "Command execution error: command failed"
    );
}

#[test]
fn test_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let error: Error = io_err.into();

    match error {
        Error::Io(inner) => {
            assert_eq!(inner.kind(), io::ErrorKind::PermissionDenied);
        }
        _ => panic!("Expected Io error variant"),
    }
}

#[test]
fn test_terminal_error_creation() {
    let error = Error::Terminal("crossterm error".to_string());

    match error {
        Error::Terminal(msg) => {
            assert_eq!(msg, "crossterm error");
        }
        _ => panic!("Expected Terminal error variant"),
    }
}

#[tokio::test]
async fn test_from_channel_send_error() {
    let (tx, _rx) = mpsc::unbounded_channel::<i32>();
    drop(_rx);

    let send_result = tx.send(42);
    assert!(send_result.is_err());

    let error: Error = send_result.unwrap_err().into();
    match error {
        Error::ChannelSend => {}
        _ => panic!("Expected ChannelSend error variant"),
    }
}

#[tokio::test]
async fn test_from_channel_recv_error() {
    let (_tx, mut rx) = mpsc::unbounded_channel::<i32>();
    drop(_tx);

    let recv_result = rx.try_recv();
    assert!(recv_result.is_err());

    let error: Error = recv_result.unwrap_err().into();
    match error {
        Error::ChannelReceive => {}
        _ => panic!("Expected ChannelReceive error variant"),
    }
}

#[tokio::test]
async fn test_from_oneshot_recv_error() {
    let (tx, rx) = tokio::sync::oneshot::channel::<i32>();
    drop(tx);

    let recv_result = rx.await;
    assert!(recv_result.is_err());

    let error: Error = recv_result.unwrap_err().into();
    match error {
        Error::ChannelReceive => {}
        _ => panic!("Expected ChannelReceive error variant"),
    }
}

#[test]
fn test_from_string() {
    let error: Error = "test error".into();
    match error {
        Error::Configuration(msg) => {
            assert_eq!(msg, "test error");
        }
        _ => panic!("Expected Configuration error variant"),
    }

    let error: Error = String::from("another test").into();
    match error {
        Error::Configuration(msg) => {
            assert_eq!(msg, "another test");
        }
        _ => panic!("Expected Configuration error variant"),
    }
}

#[test]
fn test_error_debug() {
    let error = Error::ProgramPanic("debug test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ProgramPanic"));
    assert!(debug_str.contains("debug test"));
}

#[test]
fn test_error_variants_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Error>();
}
