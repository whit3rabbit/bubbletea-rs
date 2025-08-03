use std::any::Any;
use std::time::Duration;

use bubbletea_rs::{command, event::BatchMsgInternal, Msg};

#[derive(Debug)]
struct FastMsg;
#[derive(Debug)]
struct SlowMsg;

fn is_type<T: Any>(msg: &Msg) -> bool {
    msg.downcast_ref::<T>().is_some()
}

#[tokio::test]
async fn batch_streams_messages_as_ready() {
    // Fast (10ms) and slow (300ms) commands
    let fast = command::tick(Duration::from_millis(10), |_| Box::new(FastMsg) as Msg);
    let slow = command::tick(Duration::from_millis(300), |_| Box::new(SlowMsg) as Msg);

    // Act: run batch, which should wait for both commands to complete and return a BatchMsgInternal
    let batch_cmd = command::batch(vec![fast, slow]);

    // The batch should complete within a reasonable time (longer than slow command)
    let res = tokio::time::timeout(Duration::from_millis(500), batch_cmd).await;
    assert!(res.is_ok(), "batch() did not complete in time");

    let batch_result = res.unwrap();
    assert!(
        batch_result.is_some(),
        "batch() should return a BatchMsgInternal"
    );

    let batch_msg = batch_result.unwrap();
    let batch_internal = batch_msg
        .downcast_ref::<BatchMsgInternal>()
        .expect("should be BatchMsgInternal");

    // Should contain both messages
    assert_eq!(batch_internal.messages.len(), 2);

    // Should contain both FastMsg and SlowMsg (order may vary due to concurrent execution)
    let has_fast = batch_internal
        .messages
        .iter()
        .any(|msg| is_type::<FastMsg>(msg));
    let has_slow = batch_internal
        .messages
        .iter()
        .any(|msg| is_type::<SlowMsg>(msg));

    assert!(has_fast, "batch should contain FastMsg");
    assert!(has_slow, "batch should contain SlowMsg");
}
