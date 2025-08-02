use bubbletea_rs::{Model, Msg, KeyMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod textinput_main;
use textinput_main::{TextInputModel, BlinkMsg};

#[test]
fn test_textinput_model_init() {
    let (model, cmd) = TextInputModel::init();
    
    // Should start with empty input
    assert!(model.input.is_empty());
    assert_eq!(model.cursor_pos, 0);
    assert_eq!(model.placeholder, "Pikachu");
    assert_eq!(model.char_limit, 156);
    assert_eq!(model.width, 20);
    assert!(model.focused);
    assert!(model.show_cursor);
    assert!(model.error.is_none());
    
    // Should return a command for cursor blinking
    assert!(cmd.is_some());
}

#[test]
fn test_initial_view() {
    let (model, _) = TextInputModel::init();
    let view = model.view();
    
    // Should show placeholder and prompt
    assert!(view.contains("What's your favorite Pokémon?"));
    assert!(view.contains("Pikachu"));
    assert!(view.contains("(esc to quit)"));
    assert!(view.contains("│")); // Cursor should be visible initially
}

#[test]
fn test_character_input() {
    let mut model = TextInputModel {
        input: String::new(),
        cursor_pos: 0,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('P'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Character should be inserted
    assert_eq!(model.input, "P");
    assert_eq!(model.cursor_pos, 1);
    assert!(model.show_cursor);
    assert!(cmd.is_none());
}

#[test]
fn test_multiple_character_input() {
    let mut model = TextInputModel {
        input: String::new(),
        cursor_pos: 0,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let chars = ['P', 'i', 'k', 'a', 'c', 'h', 'u'];
    
    for c in chars {
        let key_msg = Box::new(KeyMsg {
            key: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
        }) as Msg;
        
        model.update(key_msg);
    }
    
    // Should spell "Pikachu"
    assert_eq!(model.input, "Pikachu");
    assert_eq!(model.cursor_pos, 7);
}

#[test]
fn test_backspace() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should remove last character
    assert_eq!(model.input, "Pikach");
    assert_eq!(model.cursor_pos, 6);
    assert!(model.show_cursor);
    assert!(cmd.is_none());
}

#[test]
fn test_backspace_at_beginning() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 0,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not change anything
    assert_eq!(model.input, "Pikachu");
    assert_eq!(model.cursor_pos, 0);
    assert!(cmd.is_none());
}

#[test]
fn test_delete() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 3, // At 'a'
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Delete,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should remove character at cursor position
    assert_eq!(model.input, "Pikchu");
    assert_eq!(model.cursor_pos, 3);
    assert!(model.show_cursor);
    assert!(cmd.is_none());
}

#[test]
fn test_delete_at_end() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7, // At end
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Delete,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not change anything
    assert_eq!(model.input, "Pikachu");
    assert_eq!(model.cursor_pos, 7);
    assert!(cmd.is_none());
}

#[test]
fn test_cursor_left() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: false,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move cursor left
    assert_eq!(model.cursor_pos, 6);
    assert!(model.show_cursor); // Should show cursor when moving
    assert!(cmd.is_none());
}

#[test]
fn test_cursor_left_at_beginning() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 0,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not move cursor
    assert_eq!(model.cursor_pos, 0);
    assert!(cmd.is_none());
}

#[test]
fn test_cursor_right() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 3,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: false,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move cursor right
    assert_eq!(model.cursor_pos, 4);
    assert!(model.show_cursor); // Should show cursor when moving
    assert!(cmd.is_none());
}

#[test]
fn test_cursor_right_at_end() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not move cursor
    assert_eq!(model.cursor_pos, 7);
    assert!(cmd.is_none());
}

#[test]
fn test_home_key() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 5,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: false,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Home,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move cursor to beginning
    assert_eq!(model.cursor_pos, 0);
    assert!(model.show_cursor);
    assert!(cmd.is_none());
}

#[test]
fn test_end_key() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 2,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: false,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::End,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move cursor to end
    assert_eq!(model.cursor_pos, 7);
    assert!(model.show_cursor);
    assert!(cmd.is_none());
}

#[test]
fn test_character_limit() {
    let mut model = TextInputModel {
        input: "x".repeat(156), // At character limit
        cursor_pos: 156,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('z'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not add character if at limit
    assert_eq!(model.input.len(), 156);
    assert_eq!(model.cursor_pos, 156);
    assert!(cmd.is_none());
}

#[test]
fn test_insert_character_middle() {
    let mut model = TextInputModel {
        input: "Pichu".to_string(),
        cursor_pos: 2, // Between 'i' and 'c'
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should insert 'k' at cursor position
    assert_eq!(model.input, "Pikchu");
    assert_eq!(model.cursor_pos, 3);
    assert!(cmd.is_none());
}

#[test]
fn test_enter_key_quits() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return quit command
    assert!(cmd.is_some());
}

#[test]
fn test_esc_key_quits() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return quit command
    assert!(cmd.is_some());
}

#[test]
fn test_ctrl_c_quits() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return quit command
    assert!(cmd.is_some());
}

#[test]
fn test_blink_message() {
    let mut model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let blink_msg = Box::new(BlinkMsg) as Msg;
    let cmd = model.update(blink_msg);
    
    // Should toggle cursor visibility
    assert!(!model.show_cursor);
    assert!(cmd.is_some()); // Should return another blink command
}

#[test]
fn test_view_with_cursor() {
    let model = TextInputModel {
        input: "Pika".to_string(),
        cursor_pos: 4,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let view = model.view();
    
    // Should show input with cursor at end
    assert!(view.contains("What's your favorite Pokémon?"));
    assert!(view.contains("Pika│"));
    assert!(view.contains("(esc to quit)"));
}

#[test]
fn test_view_cursor_in_middle() {
    let model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 3, // After 'k'
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let view = model.view();
    
    // Should show cursor in middle of text
    assert!(view.contains("Pik│achu"));
}

#[test]
fn test_view_without_cursor() {
    let model = TextInputModel {
        input: "Pikachu".to_string(),
        cursor_pos: 7,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: false, // Cursor hidden
        error: None,
    };
    
    let view = model.view();
    
    // Should show input without cursor
    assert!(view.contains("Pikachu"));
    assert!(!view.contains("│"));
}

#[test]
fn test_view_empty_input_with_placeholder() {
    let model = TextInputModel {
        input: String::new(),
        cursor_pos: 0,
        placeholder: "Pikachu".to_string(),
        char_limit: 156,
        width: 20,
        focused: true,
        show_cursor: true,
        error: None,
    };
    
    let view = model.view();
    
    // Should show placeholder with cursor
    assert!(view.contains("│Pikachu"));
}