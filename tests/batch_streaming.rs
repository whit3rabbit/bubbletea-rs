use std::time::Duration;

use bubbletea_rs::{command, event::BatchCmdMsg};

#[derive(Debug)]
struct FastMsg;
#[derive(Debug)]
struct SlowMsg;

#[tokio::test]
async fn batch_streams_messages_as_ready() {
    // Fast (10ms) and slow (300ms) commands
    let fast = command::tick(Duration::from_millis(10), |_| Box::new(FastMsg) as bubbletea_rs::Msg);
    let slow = command::tick(Duration::from_millis(300), |_| Box::new(SlowMsg) as bubbletea_rs::Msg);

    // Act: run batch, which should immediately return a BatchCmdMsg for non-blocking execution
    let batch_cmd = command::batch(vec![fast, slow]);

    // The batch should complete immediately (non-blocking behavior)
    let res = tokio::time::timeout(Duration::from_millis(10), batch_cmd).await;
    assert!(res.is_ok(), "batch() should return immediately");

    let batch_result = res.unwrap();
    assert!(
        batch_result.is_some(),
        "batch() should return a BatchCmdMsg"
    );

    let batch_msg = batch_result.unwrap();
    let batch_cmd_msg = batch_msg
        .downcast_ref::<BatchCmdMsg>()
        .expect("should be BatchCmdMsg");

    // Should contain both commands ready for spawning
    assert_eq!(batch_cmd_msg.0.len(), 2);
}
