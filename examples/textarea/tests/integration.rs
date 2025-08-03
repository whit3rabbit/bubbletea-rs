use bubbletea_rs::{KeyMsg, Model, Msg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod textarea_main;
use textarea_main::{BlinkMsg, TextAreaModel};

#[test]
fn test_textarea_model_init() {
    let (model, cmd) = TextAreaModel::init();

    // Should start with single empty line
    assert_eq!(model.content.len(), 1);
    assert!(model.content[0].is_empty());
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 0);
    assert_eq!(model.placeholder, "Once upon a time...");
    assert!(model.focused);
    assert!(model.show_cursor);
    assert_eq!(model.scroll_offset, 0);
    assert_eq!(model.height, 5);
    assert_eq!(model.width, 50);

    // Should return a command for cursor blinking
    assert!(cmd.is_some());
}

#[test]
fn test_textarea_new() {
    let model = TextAreaModel::new();

    assert_eq!(model.content, vec![String::new()]);
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 0);
    assert!(model.focused);
    assert!(model.show_cursor);
}

#[test]
fn test_focus_blur() {
    let mut model = TextAreaModel::new();

    assert!(model.focused);

    model.blur();
    assert!(!model.focused);

    model.focus();
    assert!(model.focused);
}

#[test]
fn test_is_empty() {
    let mut model = TextAreaModel::new();

    assert!(model.is_empty());

    model.content[0] = "Hello".to_string();
    assert!(!model.is_empty());

    model.content = vec![String::new()];
    assert!(model.is_empty());
}

#[test]
fn test_insert_char() {
    let mut model = TextAreaModel::new();

    model.insert_char('H');
    model.insert_char('i');

    assert_eq!(model.content[0], "Hi");
    assert_eq!(model.cursor_col, 2);
}

#[test]
fn test_insert_char_middle() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hlo".to_string();
    model.cursor_col = 1;

    model.insert_char('e');

    assert_eq!(model.content[0], "Helo");
    assert_eq!(model.cursor_col, 2);
}

#[test]
fn test_insert_newline() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2; // Between 'e' and 'l'

    model.insert_newline();

    assert_eq!(model.content.len(), 2);
    assert_eq!(model.content[0], "He");
    assert_eq!(model.content[1], "llo");
    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 0);
}

#[test]
fn test_insert_newline_at_end() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 5;

    model.insert_newline();

    assert_eq!(model.content.len(), 2);
    assert_eq!(model.content[0], "Hello");
    assert_eq!(model.content[1], "");
    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 0);
}

#[test]
fn test_backspace_within_line() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 5;

    model.backspace();

    assert_eq!(model.content[0], "Hell");
    assert_eq!(model.cursor_col, 4);
}

#[test]
fn test_backspace_at_line_start() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string(), "World".to_string()];
    model.cursor_line = 1;
    model.cursor_col = 0;

    model.backspace();

    assert_eq!(model.content.len(), 1);
    assert_eq!(model.content[0], "HelloWorld");
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 5);
}

#[test]
fn test_backspace_first_line_start() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 0;

    model.backspace();

    // Should not change anything
    assert_eq!(model.content[0], "Hello");
    assert_eq!(model.cursor_col, 0);
}

#[test]
fn test_delete_char_within_line() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2; // At 'l'

    model.delete_char();

    assert_eq!(model.content[0], "Helo");
    assert_eq!(model.cursor_col, 2);
}

#[test]
fn test_delete_char_at_line_end() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string(), "World".to_string()];
    model.cursor_line = 0;
    model.cursor_col = 5;

    model.delete_char();

    assert_eq!(model.content.len(), 1);
    assert_eq!(model.content[0], "HelloWorld");
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 5);
}

#[test]
fn test_move_cursor_left() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 3;

    model.move_cursor_left();

    assert_eq!(model.cursor_col, 2);
}

#[test]
fn test_move_cursor_left_across_lines() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string(), "World".to_string()];
    model.cursor_line = 1;
    model.cursor_col = 0;

    model.move_cursor_left();

    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 5);
}

#[test]
fn test_move_cursor_left_at_beginning() {
    let mut model = TextAreaModel::new();
    model.cursor_line = 0;
    model.cursor_col = 0;

    model.move_cursor_left();

    // Should not move
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 0);
}

#[test]
fn test_move_cursor_right() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2;

    model.move_cursor_right();

    assert_eq!(model.cursor_col, 3);
}

#[test]
fn test_move_cursor_right_across_lines() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string(), "World".to_string()];
    model.cursor_line = 0;
    model.cursor_col = 5;

    model.move_cursor_right();

    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 0);
}

#[test]
fn test_move_cursor_right_at_end() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string()];
    model.cursor_line = 0;
    model.cursor_col = 5;

    model.move_cursor_right();

    // Should not move beyond last line
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 5);
}

#[test]
fn test_move_cursor_up() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string(), "World".to_string()];
    model.cursor_line = 1;
    model.cursor_col = 3;

    model.move_cursor_up();

    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 3);
}

#[test]
fn test_move_cursor_up_with_column_adjustment() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hi".to_string(), "HelloWorld".to_string()];
    model.cursor_line = 1;
    model.cursor_col = 8;

    model.move_cursor_up();

    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 2); // Adjusted to line length
}

#[test]
fn test_move_cursor_up_at_first_line() {
    let mut model = TextAreaModel::new();
    model.cursor_line = 0;
    model.cursor_col = 3;

    model.move_cursor_up();

    // Should not move
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 3);
}

#[test]
fn test_move_cursor_down() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string(), "World".to_string()];
    model.cursor_line = 0;
    model.cursor_col = 3;

    model.move_cursor_down();

    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 3);
}

#[test]
fn test_move_cursor_down_with_column_adjustment() {
    let mut model = TextAreaModel::new();
    model.content = vec!["HelloWorld".to_string(), "Hi".to_string()];
    model.cursor_line = 0;
    model.cursor_col = 8;

    model.move_cursor_down();

    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 2); // Adjusted to line length
}

#[test]
fn test_move_cursor_down_at_last_line() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string()];
    model.cursor_line = 0;
    model.cursor_col = 3;

    model.move_cursor_down();

    // Should not move
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 3);
}

#[test]
fn test_move_cursor_home() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 3;

    model.move_cursor_home();

    assert_eq!(model.cursor_col, 0);
}

#[test]
fn test_move_cursor_end() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2;

    model.move_cursor_end();

    assert_eq!(model.cursor_col, 5);
}

#[test]
fn test_get_display_content_empty_not_focused() {
    let mut model = TextAreaModel::new();
    model.focused = false;

    let display = model.get_display_content();

    assert_eq!(display, vec!["Once upon a time..."]);
}

#[test]
fn test_get_display_content_with_cursor() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2;
    model.focused = true;
    model.show_cursor = true;

    let display = model.get_display_content();

    assert_eq!(display[0], "He│llo");
}

#[test]
fn test_get_display_content_cursor_at_end() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 5;
    model.focused = true;
    model.show_cursor = true;

    let display = model.get_display_content();

    assert_eq!(display[0], "Hello│");
}

#[test]
fn test_character_input_updates() {
    let mut model = TextAreaModel::new();

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('H'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert_eq!(model.content[0], "H");
    assert_eq!(model.cursor_col, 1);
    assert!(model.show_cursor);
    assert!(cmd.is_none());
}

#[test]
fn test_enter_key_creates_newline() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2;

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert_eq!(model.content.len(), 2);
    assert_eq!(model.content[0], "He");
    assert_eq!(model.content[1], "llo");
    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 0);
    assert!(cmd.is_none());
}

#[test]
fn test_backspace_key() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 5;

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert_eq!(model.content[0], "Hell");
    assert_eq!(model.cursor_col, 4);
    assert!(cmd.is_none());
}

#[test]
fn test_delete_key() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2;

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Delete,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert_eq!(model.content[0], "Helo");
    assert_eq!(model.cursor_col, 2);
    assert!(cmd.is_none());
}

#[test]
fn test_arrow_key_navigation() {
    let mut model = TextAreaModel::new();
    model.content = vec!["Hello".to_string(), "World".to_string()];
    model.cursor_line = 0;
    model.cursor_col = 2;

    // Test left arrow
    let left_msg = Box::new(KeyMsg {
        key: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(left_msg);
    assert_eq!(model.cursor_col, 1);

    // Test right arrow
    let right_msg = Box::new(KeyMsg {
        key: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(right_msg);
    assert_eq!(model.cursor_col, 2);

    // Test down arrow
    let down_msg = Box::new(KeyMsg {
        key: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(down_msg);
    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 2);

    // Test up arrow
    let up_msg = Box::new(KeyMsg {
        key: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(up_msg);
    assert_eq!(model.cursor_line, 0);
    assert_eq!(model.cursor_col, 2);
}

#[test]
fn test_home_end_keys() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello".to_string();
    model.cursor_col = 2;

    // Test Home key
    let home_msg = Box::new(KeyMsg {
        key: KeyCode::Home,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(home_msg);
    assert_eq!(model.cursor_col, 0);

    // Test End key
    let end_msg = Box::new(KeyMsg {
        key: KeyCode::End,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(end_msg);
    assert_eq!(model.cursor_col, 5);
}

#[test]
fn test_esc_key_blurs() {
    let mut model = TextAreaModel::new();
    assert!(model.focused);

    let esc_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(esc_msg);

    assert!(!model.focused);
    assert!(cmd.is_none());
}

#[test]
fn test_esc_key_quits_when_blurred() {
    let mut model = TextAreaModel::new();
    model.focused = false;

    let esc_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(esc_msg);

    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_ctrl_c_quits() {
    let mut model = TextAreaModel::new();

    let ctrl_c_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;

    let cmd = model.update(ctrl_c_msg);

    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_refocus_on_key_when_blurred() {
    let mut model = TextAreaModel::new();
    model.focused = false;

    let char_msg = Box::new(KeyMsg {
        key: KeyCode::Char('H'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    model.update(char_msg);

    assert!(model.focused); // Should refocus
}

#[test]
fn test_blink_message() {
    let mut model = TextAreaModel::new();
    model.show_cursor = true;

    let blink_msg = Box::new(BlinkMsg) as Msg;
    let cmd = model.update(blink_msg);

    assert!(!model.show_cursor); // Should toggle
    assert!(cmd.is_some()); // Should return another blink command
}

#[test]
fn test_view_rendering() {
    let (model, _) = TextAreaModel::init();
    let view = model.view();

    assert!(view.contains("Tell me a story."));
    assert!(view.contains("┌")); // Top border
    assert!(view.contains("└")); // Bottom border
    assert!(view.contains("│")); // Side borders
    assert!(view.contains("(ctrl+c to quit"));
}

#[test]
fn test_view_with_content() {
    let mut model = TextAreaModel::new();
    model.content[0] = "Hello World".to_string();
    model.focused = true;
    model.show_cursor = true;
    model.cursor_col = 5;

    let view = model.view();

    assert!(view.contains("Hello│ World")); // Should show cursor
}

#[test]
fn test_multiline_editing_sequence() {
    let mut model = TextAreaModel::new();

    // Type "Hello"
    for c in "Hello".chars() {
        model.insert_char(c);
    }
    assert_eq!(model.content[0], "Hello");
    assert_eq!(model.cursor_col, 5);

    // Press Enter
    model.insert_newline();
    assert_eq!(model.content.len(), 2);
    assert_eq!(model.cursor_line, 1);
    assert_eq!(model.cursor_col, 0);

    // Type "World"
    for c in "World".chars() {
        model.insert_char(c);
    }
    assert_eq!(model.content[1], "World");
    assert_eq!(model.cursor_col, 5);

    // Final state check
    assert_eq!(
        model.content,
        vec!["Hello".to_string(), "World".to_string()]
    );
}
