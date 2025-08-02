use bubbletea_rs::{InputHandler, InputSource, KeyMsg, Msg};
use crossterm::event::{KeyCode, KeyModifiers};
use std::io::Cursor;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_input_source_terminal() {
    let (event_tx, _event_rx) = mpsc::unbounded_channel::<Msg>();
    let input_handler = InputHandler::new(event_tx);

    // Verify the input handler uses terminal input by default
    match input_handler.input_source {
        InputSource::Terminal => {} // This is what we expect
        InputSource::Custom(_) => panic!("Expected Terminal input source"),
    }
}

#[tokio::test]
async fn test_input_source_custom() {
    let (event_tx, _event_rx) = mpsc::unbounded_channel::<Msg>();
    let test_input = Cursor::new("hello\n");
    let input_source = InputSource::Custom(Box::pin(test_input));
    let input_handler = InputHandler::with_source(event_tx, input_source);

    // Verify the input handler uses custom input
    match input_handler.input_source {
        InputSource::Custom(_) => {} // This is what we expect
        InputSource::Terminal => panic!("Expected Custom input source"),
    }
}

#[tokio::test]
async fn test_custom_input_processing() {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<Msg>();
    let test_input = Cursor::new("ab\n");
    let input_source = InputSource::Custom(Box::pin(test_input));
    let input_handler = InputHandler::with_source(event_tx, input_source);

    // Run the input handler in a background task
    let handle = tokio::spawn(async move {
        let _ = input_handler.run().await;
    });

    // Collect messages
    let mut messages = Vec::new();

    // We expect: 'a', 'b', Enter
    for _ in 0..3 {
        if let Some(msg) = event_rx.recv().await {
            messages.push(msg);
        }
    }

    // Verify we got the expected messages
    assert_eq!(messages.len(), 3);

    // Check first character 'a'
    let key_msg = messages[0].downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Char('a'));
    assert_eq!(key_msg.modifiers, KeyModifiers::NONE);

    // Check second character 'b'
    let key_msg = messages[1].downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Char('b'));
    assert_eq!(key_msg.modifiers, KeyModifiers::NONE);

    // Check Enter key
    let key_msg = messages[2].downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Enter);
    assert_eq!(key_msg.modifiers, KeyModifiers::NONE);

    // Clean up the background task
    handle.abort();
}

#[tokio::test]
async fn test_custom_input_empty_line() {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<Msg>();
    let test_input = Cursor::new("\n");
    let input_source = InputSource::Custom(Box::pin(test_input));
    let input_handler = InputHandler::with_source(event_tx, input_source);

    // Run the input handler in a background task
    let handle = tokio::spawn(async move {
        let _ = input_handler.run().await;
    });

    // We should only get an Enter key for an empty line
    let msg = event_rx.recv().await.unwrap();
    let key_msg = msg.downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Enter);
    assert_eq!(key_msg.modifiers, KeyModifiers::NONE);

    // Clean up the background task
    handle.abort();
}

#[tokio::test]
async fn test_custom_input_multiple_lines() {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<Msg>();
    let test_input = Cursor::new("hi\nbye\n");
    let input_source = InputSource::Custom(Box::pin(test_input));
    let input_handler = InputHandler::with_source(event_tx, input_source);

    // Run the input handler in a background task
    let handle = tokio::spawn(async move {
        let _ = input_handler.run().await;
    });

    // Collect messages
    let mut messages = Vec::new();

    // We expect: 'h', 'i', Enter, 'b', 'y', 'e', Enter
    for _ in 0..7 {
        if let Some(msg) = event_rx.recv().await {
            messages.push(msg);
        }
    }

    // Verify we got the expected messages
    assert_eq!(messages.len(), 7);

    // Check the sequence: h, i, Enter, b, y, e, Enter
    let expected_keys = [
        KeyCode::Char('h'),
        KeyCode::Char('i'),
        KeyCode::Enter,
        KeyCode::Char('b'),
        KeyCode::Char('y'),
        KeyCode::Char('e'),
        KeyCode::Enter,
    ];

    for (i, expected_key) in expected_keys.iter().enumerate() {
        let key_msg = messages[i].downcast_ref::<KeyMsg>().unwrap();
        assert_eq!(key_msg.key, *expected_key);
        assert_eq!(key_msg.modifiers, KeyModifiers::NONE);
    }

    // Clean up the background task
    handle.abort();
}

#[tokio::test]
async fn test_custom_input_eof() {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<Msg>();
    let test_input = Cursor::new("test"); // No newline, will hit EOF
    let input_source = InputSource::Custom(Box::pin(test_input));
    let input_handler = InputHandler::with_source(event_tx, input_source);

    // Run the input handler in a background task
    let handle = tokio::spawn(async move {
        let result = input_handler.run().await;
        assert!(result.is_ok()); // Should complete successfully on EOF
    });

    // Collect messages - should get 't', 'e', 's', 't' but no Enter
    let mut messages = Vec::new();

    for _ in 0..4 {
        if let Some(msg) = event_rx.recv().await {
            messages.push(msg);
        } else {
            break;
        }
    }

    // Verify we got the expected messages
    assert_eq!(messages.len(), 4);

    let expected_chars = ['t', 'e', 's', 't'];
    for (i, expected_char) in expected_chars.iter().enumerate() {
        let key_msg = messages[i].downcast_ref::<KeyMsg>().unwrap();
        assert_eq!(key_msg.key, KeyCode::Char(*expected_char));
        assert_eq!(key_msg.modifiers, KeyModifiers::NONE);
    }

    // Wait for the handler to complete
    let _ = handle.await;
}
