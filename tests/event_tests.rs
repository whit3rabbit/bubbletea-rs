use bubbletea_rs::{
    BlurMsg, FocusMsg, InterruptMsg, KeyMsg, MouseMsg, Msg, QuitMsg, ResumeMsg, SuspendMsg,
    WindowSizeMsg,
};
use crossterm::event::{KeyCode, KeyModifiers, MouseEventKind};

#[test]
fn test_msg_type_alias() {
    let key_msg: Msg = Box::new(KeyMsg {
        key: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
    });

    let mouse_msg: Msg = Box::new(MouseMsg {
        x: 10,
        y: 20,
        button: MouseEventKind::Down(crossterm::event::MouseButton::Left),
        modifiers: KeyModifiers::CONTROL,
    });

    let quit_msg: Msg = Box::new(QuitMsg);

    assert!(key_msg.downcast_ref::<KeyMsg>().is_some());
    assert!(mouse_msg.downcast_ref::<MouseMsg>().is_some());
    assert!(quit_msg.downcast_ref::<QuitMsg>().is_some());

    assert!(key_msg.downcast_ref::<MouseMsg>().is_none());
}

#[test]
fn test_key_msg() {
    let key_msg = KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::SHIFT | KeyModifiers::CONTROL,
    };

    assert_eq!(key_msg.key, KeyCode::Enter);
    assert!(key_msg.modifiers.contains(KeyModifiers::SHIFT));
    assert!(key_msg.modifiers.contains(KeyModifiers::CONTROL));
    assert!(!key_msg.modifiers.contains(KeyModifiers::ALT));
}

#[test]
fn test_mouse_msg() {
    let mouse_msg = MouseMsg {
        x: 42,
        y: 24,
        button: MouseEventKind::Down(crossterm::event::MouseButton::Right),
        modifiers: KeyModifiers::ALT,
    };

    assert_eq!(mouse_msg.x, 42);
    assert_eq!(mouse_msg.y, 24);
    assert_eq!(
        mouse_msg.button,
        MouseEventKind::Down(crossterm::event::MouseButton::Right)
    );
    assert_eq!(mouse_msg.modifiers, KeyModifiers::ALT);
}

#[test]
fn test_window_size_msg() {
    let size_msg = WindowSizeMsg {
        width: 80,
        height: 24,
    };

    assert_eq!(size_msg.width, 80);
    assert_eq!(size_msg.height, 24);
}

#[test]
fn test_lifecycle_messages() {
    let quit_msg = QuitMsg;
    let interrupt_msg = InterruptMsg;
    let suspend_msg = SuspendMsg;
    let resume_msg = ResumeMsg;
    let focus_msg = FocusMsg;
    let blur_msg = BlurMsg;

    let _: QuitMsg = quit_msg;
    let _: InterruptMsg = interrupt_msg;
    let _: SuspendMsg = suspend_msg;
    let _: ResumeMsg = resume_msg;
    let _: FocusMsg = focus_msg;
    let _: BlurMsg = blur_msg;
}

#[test]
fn test_message_cloning() {
    let key_msg = KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    };
    let cloned = key_msg.clone();
    assert_eq!(key_msg.key, cloned.key);
    assert_eq!(key_msg.modifiers, cloned.modifiers);

    let mouse_msg = MouseMsg {
        x: 1,
        y: 2,
        button: MouseEventKind::Moved,
        modifiers: KeyModifiers::NONE,
    };
    let cloned = mouse_msg.clone();
    assert_eq!(mouse_msg.x, cloned.x);
    assert_eq!(mouse_msg.y, cloned.y);

    let size_msg = WindowSizeMsg {
        width: 100,
        height: 50,
    };
    let cloned = size_msg.clone();
    assert_eq!(size_msg.width, cloned.width);
    assert_eq!(size_msg.height, cloned.height);
}

#[test]
fn test_message_debug() {
    let key_msg = KeyMsg {
        key: KeyCode::Char('a'),
        modifiers: KeyModifiers::CONTROL,
    };
    let debug_str = format!("{:?}", key_msg);
    assert!(debug_str.contains("KeyMsg"));

    let quit_msg = QuitMsg;
    let debug_str = format!("{:?}", quit_msg);
    assert!(debug_str.contains("QuitMsg"));
}

#[test]
fn test_messages_are_send() {
    fn assert_send<T: Send>() {}

    assert_send::<KeyMsg>();
    assert_send::<MouseMsg>();
    assert_send::<WindowSizeMsg>();
    assert_send::<QuitMsg>();
    assert_send::<InterruptMsg>();
    assert_send::<SuspendMsg>();
    assert_send::<ResumeMsg>();
    assert_send::<FocusMsg>();
    assert_send::<BlurMsg>();
    assert_send::<Msg>();
}
