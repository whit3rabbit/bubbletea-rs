use bubbletea_rs::{
    event::{BatchCmdMsg, BatchMsgInternal}, Cmd, DisableReportFocusMsg, EnableMouseAllMotionMsg, EnableReportFocusMsg,
    InterruptMsg, KeyMsg, Msg, PrintMsg, PrintfMsg, QuitMsg, SuspendMsg,
};
use crossterm::event::{KeyCode, KeyModifiers};
use std::time::Duration;

fn create_test_cmd(msg: Msg) -> Cmd {
    Box::pin(async move { Some(msg) })
}

fn create_empty_cmd() -> Cmd {
    Box::pin(async move { None })
}

#[tokio::test]
async fn test_cmd_type_alias() {
    let quit_msg = QuitMsg;
    let cmd = create_test_cmd(Box::new(quit_msg) as Msg);

    let result = cmd.await;
    assert!(result.is_some());

    let msg = result.unwrap();
    assert!(msg.downcast_ref::<QuitMsg>().is_some());
}

#[tokio::test]
async fn test_cmd_with_different_message_types() {
    let key_msg = KeyMsg {
        key: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
    };
    let cmd = create_test_cmd(Box::new(key_msg) as Msg);
    let result = cmd.await;
    assert!(result.is_some());
    assert!(result.unwrap().downcast_ref::<KeyMsg>().is_some());

    let quit_msg = QuitMsg;
    let cmd = create_test_cmd(Box::new(quit_msg) as Msg);
    let result = cmd.await;
    assert!(result.is_some());
    assert!(result.unwrap().downcast_ref::<QuitMsg>().is_some());
}

#[tokio::test]
async fn test_cmd_returning_none() {
    let cmd = create_empty_cmd();
    let result = cmd.await;
    assert!(result.is_none());
}

#[tokio::test]
async fn test_cmd_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Cmd>();

    let cmd = create_test_cmd(Box::new(QuitMsg) as Msg);

    let handle = tokio::spawn(cmd);

    let result = handle.await.unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn test_complex_async_command() {
    let cmd: Cmd = Box::pin(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        Some(Box::new(KeyMsg {
            key: KeyCode::Enter,
            modifiers: KeyModifiers::CONTROL,
        }) as Msg)
    });

    let result = cmd.await;
    assert!(result.is_some());

    let msg = result.unwrap();
    let key_msg = msg.downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Enter);
    assert!(key_msg.modifiers.contains(KeyModifiers::CONTROL));
}

#[tokio::test]
async fn test_command_composition() {
    async fn create_composed_cmd() -> Option<Msg> {
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        Some(Box::new(QuitMsg) as Msg)
    }

    let cmd: Cmd = Box::pin(create_composed_cmd());
    let result = cmd.await;

    assert!(result.is_some());
    assert!(result.unwrap().downcast_ref::<QuitMsg>().is_some());
}

#[tokio::test]
async fn test_quit_command() {
    let cmd = bubbletea_rs::command::quit();
    let msg = cmd.await.unwrap();
    assert!(msg.downcast_ref::<QuitMsg>().is_some());
}

#[tokio::test]
async fn test_interrupt_command() {
    let cmd = bubbletea_rs::command::interrupt();
    let msg = cmd.await.unwrap();
    assert!(msg.downcast_ref::<InterruptMsg>().is_some());
}

#[tokio::test]
async fn test_suspend_command() {
    let cmd = bubbletea_rs::command::suspend();
    let msg = cmd.await.unwrap();
    assert!(msg.downcast_ref::<SuspendMsg>().is_some());
}

#[tokio::test]
async fn test_batch_command() {
    let cmd1 = create_test_cmd(Box::new(QuitMsg) as Msg);
    let cmd2 = create_test_cmd(Box::new(KeyMsg {
        key: KeyCode::Char('b'),
        modifiers: KeyModifiers::NONE,
    }) as Msg);
    let batch_cmd = bubbletea_rs::command::batch(vec![cmd1, cmd2]);

    let msg = batch_cmd.await.unwrap();
    let batch_cmd_msg = msg.downcast_ref::<BatchCmdMsg>().unwrap();
    assert_eq!(batch_cmd_msg.0.len(), 2);
}

#[tokio::test]
async fn test_sequence_command() {
    let cmd1 = create_test_cmd(Box::new(QuitMsg) as Msg);
    let cmd2 = create_test_cmd(Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::NONE,
    }) as Msg);
    let sequence_cmd = bubbletea_rs::command::sequence(vec![cmd1, cmd2]);

    let msg = sequence_cmd.await.unwrap();
    let batch_msg = msg.downcast_ref::<BatchMsgInternal>().unwrap();
    assert_eq!(batch_msg.messages.len(), 2);
    assert!(batch_msg.messages[0].downcast_ref::<QuitMsg>().is_some());
    assert!(batch_msg.messages[1].downcast_ref::<KeyMsg>().is_some());
}

#[tokio::test]
async fn test_tick_command() {
    let cmd = bubbletea_rs::tick(Duration::from_millis(50), |_d| {
        Box::new(KeyMsg {
            key: KeyCode::Char('t'),
            modifiers: KeyModifiers::NONE,
        }) as Msg
    });
    let msg = cmd.await.unwrap();
    // Just verify the command produces the expected message
    assert!(msg.downcast_ref::<KeyMsg>().is_some());
}

#[tokio::test]
async fn test_every_command() {
    // The every command now returns a special message that the Program handles
    let cmd = bubbletea_rs::every(Duration::from_millis(10), move |_d| {
        Box::new(KeyMsg {
            key: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
        }) as Msg
    });

    let msg = cmd.await.unwrap();
    assert!(msg.is::<bubbletea_rs::event::EveryMsgInternal>());
}

#[tokio::test]
async fn test_enable_mouse_all_motion_command() {
    let cmd = bubbletea_rs::command::enable_mouse_all_motion();
    let msg = cmd.await.unwrap();
    assert!(msg.downcast_ref::<EnableMouseAllMotionMsg>().is_some());
}

#[tokio::test]
async fn test_enable_report_focus_command() {
    let cmd = bubbletea_rs::command::enable_report_focus();
    let msg = cmd.await.unwrap();
    assert!(msg.downcast_ref::<EnableReportFocusMsg>().is_some());
}

#[tokio::test]
async fn test_disable_report_focus_command() {
    let cmd = bubbletea_rs::command::disable_report_focus();
    let msg = cmd.await.unwrap();
    assert!(msg.downcast_ref::<DisableReportFocusMsg>().is_some());
}

#[tokio::test]
async fn test_println_command() {
    let cmd = bubbletea_rs::command::println("Hello, world!".to_string());
    let msg = cmd.await.unwrap();
    let print_msg = msg.downcast_ref::<PrintMsg>().unwrap();
    assert_eq!(print_msg.0, "Hello, world!");
}

#[tokio::test]
async fn test_printf_command() {
    let cmd = bubbletea_rs::command::printf("Formatted: {}".to_string());
    let msg = cmd.await.unwrap();
    let printf_msg = msg.downcast_ref::<PrintfMsg>().unwrap();
    assert_eq!(printf_msg.0, "Formatted: {}");
}
