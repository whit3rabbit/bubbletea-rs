use bubbletea_rs::{Model, Msg, KeyMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod textinputs_main;
use textinputs_main::{TextInputsModel, TextInput, InputType, BlinkMsg};

#[test]
fn test_textinputs_model_init() {
    let (model, cmd) = TextInputsModel::init();
    
    // Should start with 3 inputs
    assert_eq!(model.inputs.len(), 3);
    assert_eq!(model.focus_index, 0);
    assert!(model.show_cursor);
    assert!(!model.submitted);
    assert!(!model.submit_focused);
    
    // First input should be focused
    assert!(model.inputs[0].focused);
    assert!(!model.inputs[1].focused);
    assert!(!model.inputs[2].focused);
    
    // Should return a command for cursor blinking
    assert!(cmd.is_some());
}

#[test]
fn test_textinput_creation() {
    let input1 = TextInput::new("Test", InputType::Text);
    let input2 = TextInput::new("Email", InputType::Email);
    let input3 = TextInput::new("Pass", InputType::Password);
    
    assert_eq!(input1.placeholder, "Test");
    assert_eq!(input1.char_limit, 32);
    assert_eq!(input1.input_type, InputType::Text);
    
    assert_eq!(input2.placeholder, "Email");
    assert_eq!(input2.char_limit, 64); // Email has higher limit
    assert_eq!(input2.input_type, InputType::Email);
    
    assert_eq!(input3.placeholder, "Pass");
    assert_eq!(input3.char_limit, 32);
    assert_eq!(input3.input_type, InputType::Password);
}

#[test]
fn test_textinput_focus_blur() {
    let mut input = TextInput::new("Test", InputType::Text);
    
    assert!(!input.focused);
    
    input.focus();
    assert!(input.focused);
    
    input.blur();
    assert!(!input.focused);
}

#[test]
fn test_textinput_character_insertion() {
    let mut input = TextInput::new("Test", InputType::Text);
    
    input.insert_char('H');
    input.insert_char('i');
    
    assert_eq!(input.value, "Hi");
    assert_eq!(input.cursor_pos, 2);
}

#[test]
fn test_textinput_backspace() {
    let mut input = TextInput::new("Test", InputType::Text);
    input.value = "Hello".to_string();
    input.cursor_pos = 5;
    
    input.backspace();
    
    assert_eq!(input.value, "Hell");
    assert_eq!(input.cursor_pos, 4);
}

#[test]
fn test_textinput_backspace_at_beginning() {
    let mut input = TextInput::new("Test", InputType::Text);
    input.value = "Hello".to_string();
    input.cursor_pos = 0;
    
    input.backspace();
    
    // Should not change anything
    assert_eq!(input.value, "Hello");
    assert_eq!(input.cursor_pos, 0);
}

#[test]
fn test_textinput_delete() {
    let mut input = TextInput::new("Test", InputType::Text);
    input.value = "Hello".to_string();
    input.cursor_pos = 2; // At 'l'
    
    input.delete_char();
    
    assert_eq!(input.value, "Helo");
    assert_eq!(input.cursor_pos, 2);
}

#[test]
fn test_textinput_cursor_movement() {
    let mut input = TextInput::new("Test", InputType::Text);
    input.value = "Hello".to_string();
    input.cursor_pos = 2;
    
    input.move_cursor_left();
    assert_eq!(input.cursor_pos, 1);
    
    input.move_cursor_right();
    assert_eq!(input.cursor_pos, 2);
    
    input.move_cursor_home();
    assert_eq!(input.cursor_pos, 0);
    
    input.move_cursor_end();
    assert_eq!(input.cursor_pos, 5);
}

#[test]
fn test_textinput_character_limit() {
    let mut input = TextInput::new("Test", InputType::Text);
    
    // Fill to limit
    for _ in 0..32 {
        input.insert_char('x');
    }
    
    assert_eq!(input.value.len(), 32);
    
    // Try to insert one more
    input.insert_char('y');
    
    // Should not exceed limit
    assert_eq!(input.value.len(), 32);
    assert!(!input.value.contains('y'));
}

#[test]
fn test_textinput_password_rendering() {
    let mut input = TextInput::new("Password", InputType::Password);
    input.value = "secret123".to_string();
    input.focused = true;
    
    let rendered = input.render(false);
    
    // Should show asterisks for password
    assert!(rendered.contains("*********"));
    assert!(!rendered.contains("secret123"));
}

#[test]
fn test_tab_navigation_forward() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[0].focus();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move to next field
    assert_eq!(model.focus_index, 1);
    assert!(!model.inputs[0].focused);
    assert!(model.inputs[1].focused);
    assert!(!model.submit_focused);
    assert!(cmd.is_none());
}

#[test]
fn test_tab_navigation_to_submit() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 2, // Last field
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[2].focus();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move to submit button
    assert!(!model.inputs[2].focused);
    assert!(model.submit_focused);
    assert!(cmd.is_none());
}

#[test]
fn test_shift_tab_navigation_backward() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 1,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[1].focus();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Tab,
        modifiers: KeyModifiers::SHIFT,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move to previous field
    assert_eq!(model.focus_index, 0);
    assert!(model.inputs[0].focused);
    assert!(!model.inputs[1].focused);
    assert!(!model.submit_focused);
    assert!(cmd.is_none());
}

#[test]
fn test_shift_tab_from_submit_to_last_field() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: true,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Tab,
        modifiers: KeyModifiers::SHIFT,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move to last field
    assert_eq!(model.focus_index, 2);
    assert!(model.inputs[2].focused);
    assert!(!model.submit_focused);
    assert!(cmd.is_none());
}

#[test]
fn test_enter_moves_to_next_field() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[0].focus();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move to next field
    assert_eq!(model.focus_index, 1);
    assert!(!model.inputs[0].focused);
    assert!(model.inputs[1].focused);
    assert!(!model.submit_focused);
    assert!(cmd.is_none());
}

#[test]
fn test_enter_from_last_field_moves_to_submit() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 2,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[2].focus();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should move to submit button
    assert!(!model.inputs[2].focused);
    assert!(model.submit_focused);
    assert!(cmd.is_none());
}

#[test]
fn test_enter_on_submit_button() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 2,
        show_cursor: true,
        submitted: false,
        submit_focused: true,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should submit form and quit
    assert!(model.submitted);
    assert!(cmd.is_some()); // Should return quit command
}

#[test]
fn test_character_input_to_focused_field() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 1,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[1].focus();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('H'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should insert character into focused field only
    assert_eq!(model.inputs[1].value, "H");
    assert_eq!(model.inputs[0].value, "");
    assert_eq!(model.inputs[2].value, "");
    assert!(cmd.is_none());
}

#[test]
fn test_backspace_in_focused_field() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 1,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[1].focus();
    model.inputs[1].value = "Hello".to_string();
    model.inputs[1].cursor_pos = 5;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should remove character from focused field
    assert_eq!(model.inputs[1].value, "Hell");
    assert_eq!(model.inputs[1].cursor_pos, 4);
    assert!(cmd.is_none());
}

#[test]
fn test_arrow_keys_in_focused_field() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
            TextInput::new("Field3", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[0].focus();
    model.inputs[0].value = "Hello".to_string();
    model.inputs[0].cursor_pos = 2;
    
    let left_msg = Box::new(KeyMsg {
        key: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(left_msg);
    
    // Should move cursor left in focused field
    assert_eq!(model.inputs[0].cursor_pos, 1);
    assert!(cmd.is_none());
}

#[test]
fn test_esc_key_quits() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
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
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
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
    let mut model = TextInputsModel {
        inputs: vec![TextInput::new("Field1", InputType::Text)],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    
    let blink_msg = Box::new(BlinkMsg) as Msg;
    let cmd = model.update(blink_msg);
    
    // Should toggle cursor visibility
    assert!(!model.show_cursor);
    assert!(cmd.is_some()); // Should return another blink command
}

#[test]
fn test_initial_form_view() {
    let (model, _) = TextInputsModel::init();
    let view = model.view();
    
    // Should show form title and fields
    assert!(view.contains("Registration Form"));
    assert!(view.contains("Nickname:"));
    assert!(view.contains("Email:"));
    assert!(view.contains("Password:"));
    assert!(view.contains("Submit"));
    assert!(view.contains("Tab/Shift+Tab"));
}

#[test]
fn test_submitted_form_view() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Nickname", InputType::Text),
            TextInput::new("Email", InputType::Email),
            TextInput::new("Password", InputType::Password),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: true,
        submit_focused: false,
    };
    
    model.inputs[0].value = "john".to_string();
    model.inputs[1].value = "john@example.com".to_string();
    model.inputs[2].value = "secret123".to_string();
    
    let view = model.view();
    
    // Should show submitted form data
    assert!(view.contains("Form submitted successfully!"));
    assert!(view.contains("Nickname: john"));
    assert!(view.contains("Email: john@example.com"));
    assert!(view.contains("Password: *********")); // Masked password
    assert!(view.contains("Press any key to exit"));
}

#[test]
fn test_navigation_wrapping() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: true, // Start at submit button
    };
    
    // Tab should wrap to first field
    let tab_msg = Box::new(KeyMsg {
        key: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(tab_msg);
    
    assert_eq!(model.focus_index, 0);
    assert!(model.inputs[0].focused);
    assert!(!model.submit_focused);
}

#[test]
fn test_shift_tab_wrapping() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
            TextInput::new("Field2", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: false,
    };
    model.inputs[0].focus();
    
    // Shift+Tab from first field should wrap to submit
    let shift_tab_msg = Box::new(KeyMsg {
        key: KeyCode::Tab,
        modifiers: KeyModifiers::SHIFT,
    }) as Msg;
    
    model.update(shift_tab_msg);
    
    assert!(!model.inputs[0].focused);
    assert!(model.submit_focused);
}

#[test]
fn test_input_ignored_when_submit_focused() {
    let mut model = TextInputsModel {
        inputs: vec![
            TextInput::new("Field1", InputType::Text),
        ],
        focus_index: 0,
        show_cursor: true,
        submitted: false,
        submit_focused: true, // Submit button is focused
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg);
    
    // Should not modify any input field
    assert_eq!(model.inputs[0].value, "");
}

#[test]
fn test_textinput_width_truncation() {
    let mut input = TextInput::new("Test", InputType::Text);
    input.width = 5; // Set narrow width
    input.value = "ThisIsAVeryLongText".to_string();
    input.focused = true;
    
    let rendered = input.render(false);
    
    // Should be truncated to width (plus formatting)
    assert!(rendered.contains("ThisI")); // First 5 chars
    assert!(!rendered.contains("VeryLongText")); // Rest should be truncated
}

#[test]
fn test_textinput_width_truncation_with_cursor() {
    let mut input = TextInput::new("Test", InputType::Text);
    input.width = 6; // Set narrow width (accounting for cursor)
    input.value = "LongText".to_string();
    input.cursor_pos = 2;
    input.focused = true;
    
    let rendered = input.render(true);
    
    // Should be truncated to width including cursor
    let parts: Vec<&str> = rendered.split(']').collect();
    if parts.len() > 1 {
        let content = parts[0].split('[').last().unwrap_or("");
        // Should be truncated to 6 characters max (including cursor)
        assert!(content.chars().count() <= 6);
    }
}