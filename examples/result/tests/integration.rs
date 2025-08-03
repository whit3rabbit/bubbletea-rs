use bubbletea_rs::{KeyMsg, Model, Msg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod result_main;
use result_main::{Choice, ResultModel};

#[test]
fn test_result_model_init() {
    let (model, cmd) = ResultModel::init();

    // Should start with 4 choices
    assert_eq!(model.choices.len(), 4);
    assert_eq!(model.cursor, 0);
    assert!(model.selected.is_none());

    // Should not return any initial command
    assert!(cmd.is_none());
}

#[test]
fn test_choices_content() {
    let (model, _) = ResultModel::init();

    // Verify the choices are correct
    assert_eq!(model.choices[0], Choice::Option1);
    assert_eq!(model.choices[1], Choice::Option2);
    assert_eq!(model.choices[2], Choice::Option3);
    assert_eq!(model.choices[3], Choice::Option4);

    // Test choice string representations
    assert_eq!(model.choices[0].as_str(), "Continue");
    assert_eq!(model.choices[1].as_str(), "Settings");
    assert_eq!(model.choices[2].as_str(), "Help");
    assert_eq!(model.choices[3].as_str(), "Exit");
}

#[test]
fn test_choice_descriptions() {
    let choice1 = Choice::Option1;
    let choice2 = Choice::Option2;
    let choice3 = Choice::Option3;
    let choice4 = Choice::Option4;

    assert!(choice1.description().contains("Proceed"));
    assert!(choice2.description().contains("Configure"));
    assert!(choice3.description().contains("help"));
    assert!(choice4.description().contains("Exit"));
}

#[test]
fn test_initial_view() {
    let (model, _) = ResultModel::init();
    let view = model.view();

    // Should show menu with cursor on first item
    assert!(view.contains("What would you like to do?"));
    assert!(view.contains("→ Continue")); // Cursor on first item
    assert!(view.contains(" Settings")); // No cursor on second item
    assert!(view.contains("Use ↑/↓ to navigate"));
}

#[test]
fn test_cursor_movement_down() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Cursor should move down
    assert_eq!(model.cursor, 1);
    assert!(cmd.is_none());
}

#[test]
fn test_cursor_movement_up() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 2,
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Cursor should move up
    assert_eq!(model.cursor, 1);
    assert!(cmd.is_none());
}

#[test]
fn test_cursor_at_top_boundary() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Cursor should stay at top
    assert_eq!(model.cursor, 0);
    assert!(cmd.is_none());
}

#[test]
fn test_cursor_at_bottom_boundary() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 3,
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Cursor should stay at bottom
    assert_eq!(model.cursor, 3);
    assert!(cmd.is_none());
}

#[test]
fn test_selection_first_option() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should select the first option
    assert_eq!(model.selected, Some(Choice::Option1));
    assert!(cmd.is_none()); // Should not quit for non-exit options
}

#[test]
fn test_selection_exit_option() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 3, // Exit option
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should select exit and return quit command
    assert_eq!(model.selected, Some(Choice::Option4));
    assert!(cmd.is_some()); // Should quit for exit option
}

#[test]
fn test_view_after_selection() {
    let model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 1,
        selected: Some(Choice::Option2),
    };

    let view = model.view();

    // Should show selection result
    assert!(view.contains("You selected: Settings"));
    assert!(view.contains("Configure application settings"));
    assert!(view.contains("Press any key to exit"));
}

#[test]
fn test_quit_key_before_selection() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should quit without making selection
    assert!(model.selected.is_none());
    assert!(cmd.is_some());
}

#[test]
fn test_quit_key_after_selection() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: Some(Choice::Option1),
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should quit after selection
    assert!(cmd.is_some());
}

#[test]
fn test_esc_key_quits() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: None,
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should quit
    assert!(cmd.is_some());
}

#[test]
fn test_any_key_quits_after_selection() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: Some(Choice::Option2),
    };

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should quit on any key after selection
    assert!(cmd.is_some());
}

#[test]
fn test_navigation_and_selection_sequence() {
    let mut model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 0,
        selected: None,
    };

    // Move down twice
    let down_msg = Box::new(KeyMsg {
        key: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    model.update(down_msg);
    assert_eq!(model.cursor, 1);

    let down_msg2 = Box::new(KeyMsg {
        key: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    model.update(down_msg2);
    assert_eq!(model.cursor, 2);

    // Select current option (Help)
    let enter_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(enter_msg);
    assert_eq!(model.selected, Some(Choice::Option3));
    assert!(cmd.is_none()); // Help option shouldn't quit immediately
}

#[test]
fn test_view_cursor_position() {
    let model = ResultModel {
        choices: vec![
            Choice::Option1,
            Choice::Option2,
            Choice::Option3,
            Choice::Option4,
        ],
        cursor: 2, // On Help option
        selected: None,
    };

    let view = model.view();

    // Cursor should be on Help option
    assert!(view.contains(" Continue")); // No cursor
    assert!(view.contains(" Settings")); // No cursor
    assert!(view.contains("→ Help")); // Cursor here
    assert!(view.contains(" Exit")); // No cursor
}
