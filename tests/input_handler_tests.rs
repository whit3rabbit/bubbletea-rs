use bubbletea_rs::{InputHandler, KeyMsg, MouseMsg, Msg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use std::any::Any;
use tokio::sync::mpsc;

fn create_test_input_handler() -> (InputHandler, mpsc::UnboundedReceiver<Msg>) {
    let (tx, rx) = mpsc::unbounded_channel();
    let handler = InputHandler::new(tx);
    (handler, rx)
}

#[tokio::test]
async fn test_input_handler_creation() {
    let (handler, _rx) = create_test_input_handler();
    // Just verify we can create the handler without panicking
    drop(handler);
}

#[tokio::test]
async fn test_key_message_conversion() {
    // Test conversion from crossterm KeyEvent to KeyMsg
    let key_event = crossterm::event::KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);

    let key_msg = KeyMsg {
        key: key_event.code,
        modifiers: key_event.modifiers,
    };

    assert_eq!(key_msg.key, KeyCode::Char('a'));
    assert!(key_msg.modifiers.contains(KeyModifiers::CONTROL));
}

#[tokio::test]
async fn test_mouse_message_conversion() {
    // Test conversion from crossterm MouseEvent to MouseMsg
    let mouse_event = crossterm::event::MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 5,
        modifiers: KeyModifiers::SHIFT,
    };

    let mouse_msg = MouseMsg {
        x: mouse_event.column,
        y: mouse_event.row,
        button: mouse_event.kind,
        modifiers: mouse_event.modifiers,
    };

    assert_eq!(mouse_msg.x, 10);
    assert_eq!(mouse_msg.y, 5);
    assert!(matches!(
        mouse_msg.button,
        MouseEventKind::Down(MouseButton::Left)
    ));
    assert!(mouse_msg.modifiers.contains(KeyModifiers::SHIFT));
}

#[tokio::test]
async fn test_window_size_message_conversion() {
    // Test WindowSizeMsg creation
    let window_size_msg = WindowSizeMsg {
        width: 80,
        height: 24,
    };

    assert_eq!(window_size_msg.width, 80);
    assert_eq!(window_size_msg.height, 24);
}

#[tokio::test]
async fn test_message_type_checking() {
    // Test that messages can be properly downcast
    let key_msg = KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    };

    let msg: Msg = Box::new(key_msg);

    // Test successful downcast
    assert!(msg.downcast_ref::<KeyMsg>().is_some());
    assert!(msg.downcast_ref::<MouseMsg>().is_none());

    if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
        assert_eq!(key_msg.key, KeyCode::Enter);
    }
}

#[tokio::test]
async fn test_message_any_trait() {
    // Verify that our message types implement the Any trait correctly
    let key_msg = KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::ALT,
    };

    let any_ref: &dyn Any = &key_msg;
    assert!(any_ref.is::<KeyMsg>());
    assert!(!any_ref.is::<MouseMsg>());
}

#[tokio::test]
async fn test_special_key_codes() {
    // Test various special key codes
    let special_keys = vec![
        KeyCode::Enter,
        KeyCode::Tab,
        KeyCode::Backspace,
        KeyCode::Delete,
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
        KeyCode::Home,
        KeyCode::End,
        KeyCode::PageUp,
        KeyCode::PageDown,
        KeyCode::Esc,
    ];

    for key_code in special_keys {
        let key_msg = KeyMsg {
            key: key_code,
            modifiers: KeyModifiers::NONE,
        };

        assert_eq!(key_msg.key, key_code);

        // Verify message can be boxed and downcast
        let msg: Msg = Box::new(key_msg);
        assert!(msg.downcast_ref::<KeyMsg>().is_some());
    }
}

#[tokio::test]
async fn test_modifier_combinations() {
    // Test various modifier key combinations
    let modifiers = vec![
        KeyModifiers::NONE,
        KeyModifiers::SHIFT,
        KeyModifiers::CONTROL,
        KeyModifiers::ALT,
        KeyModifiers::SHIFT | KeyModifiers::CONTROL,
        KeyModifiers::SHIFT | KeyModifiers::ALT,
        KeyModifiers::CONTROL | KeyModifiers::ALT,
        KeyModifiers::SHIFT | KeyModifiers::CONTROL | KeyModifiers::ALT,
    ];

    for modifier in modifiers {
        let key_msg = KeyMsg {
            key: KeyCode::Char('x'),
            modifiers: modifier,
        };

        assert_eq!(key_msg.modifiers, modifier);
    }
}

#[tokio::test]
async fn test_mouse_button_types() {
    // Test different mouse button types
    let mouse_buttons = vec![
        MouseEventKind::Down(MouseButton::Left),
        MouseEventKind::Down(MouseButton::Right),
        MouseEventKind::Down(MouseButton::Middle),
        MouseEventKind::Up(MouseButton::Left),
        MouseEventKind::Up(MouseButton::Right),
        MouseEventKind::Up(MouseButton::Middle),
        MouseEventKind::Moved,
        MouseEventKind::Drag(MouseButton::Left),
    ];

    for button_kind in mouse_buttons {
        let mouse_msg = MouseMsg {
            x: 0,
            y: 0,
            button: button_kind,
            modifiers: KeyModifiers::NONE,
        };

        assert_eq!(mouse_msg.button, button_kind);

        // Verify message can be boxed and downcast
        let msg: Msg = Box::new(mouse_msg);
        assert!(msg.downcast_ref::<MouseMsg>().is_some());
    }
}

// Mock tests would require more complex setup to simulate actual crossterm events
// These tests focus on the message type conversions and type safety
