use bubbletea_rs::{Model, Msg, KeyMsg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod window_size_main;
use window_size_main::WindowSizeModel;

#[test]
fn test_window_size_model_init() {
    let (model, cmd) = WindowSizeModel::init();
    
    // Should start with default dimensions
    assert_eq!(model.width, 0);
    assert_eq!(model.height, 0);
    assert!(!model.ready);
    
    // Should return a command to get window size
    assert!(cmd.is_some());
}

#[test]
fn test_window_size_model_initial_view() {
    let model = WindowSizeModel {
        width: 0,
        height: 0,
        ready: false,
    };
    
    let view = model.view();
    
    // Should show loading message
    assert!(view.contains("Getting terminal dimensions"));
    assert!(view.contains("Press any key to quit"));
}

#[test]
fn test_window_size_model_ready_view() {
    let model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    let view = model.view();
    
    // Should show terminal dimensions
    assert!(view.contains("Terminal Size Information"));
    assert!(view.contains("80 columns"));
    assert!(view.contains("24 rows"));
    assert!(view.contains("1920 cells")); // 80 * 24 = 1920
    assert!(view.contains("Try resizing"));
}

#[test]
fn test_window_size_message_updates_dimensions() {
    let mut model = WindowSizeModel {
        width: 0,
        height: 0,
        ready: false,
    };
    
    let size_msg = Box::new(WindowSizeMsg {
        width: 100,
        height: 50,
    }) as Msg;
    
    let cmd = model.update(size_msg);
    
    // Dimensions should be updated
    assert_eq!(model.width, 100);
    assert_eq!(model.height, 50);
    assert!(model.ready);
    
    // Should not return any command
    assert!(cmd.is_none());
}

#[test]
fn test_window_size_message_updates_existing_dimensions() {
    let mut model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    let size_msg = Box::new(WindowSizeMsg {
        width: 120,
        height: 30,
    }) as Msg;
    
    let cmd = model.update(size_msg);
    
    // Dimensions should be updated to new values
    assert_eq!(model.width, 120);
    assert_eq!(model.height, 30);
    assert!(model.ready);
    
    // Should not return any command
    assert!(cmd.is_none());
}

#[test]
fn test_q_key_quits() {
    let mut model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Dimensions should remain unchanged
    assert_eq!(model.width, 80);
    assert_eq!(model.height, 24);
    
    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_uppercase_q_key_quits() {
    let mut model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('Q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_esc_key_quits() {
    let mut model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_any_key_quits() {
    let mut model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return a quit command (any key quits in this example)
    assert!(cmd.is_some());
}

#[test]
fn test_special_keys_quit() {
    let mut model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_cell_count_calculation() {
    let model = WindowSizeModel {
        width: 100,
        height: 50,
        ready: true,
    };
    
    let view = model.view();
    
    // Should calculate total cells correctly (100 * 50 = 5000)
    assert!(view.contains("5000 cells"));
}

#[test]
fn test_small_terminal_dimensions() {
    let model = WindowSizeModel {
        width: 10,
        height: 5,
        ready: true,
    };
    
    let view = model.view();
    
    // Should handle small dimensions correctly
    assert!(view.contains("10 columns"));
    assert!(view.contains("5 rows"));
    assert!(view.contains("50 cells")); // 10 * 5 = 50
}

#[test]
fn test_large_terminal_dimensions() {
    let model = WindowSizeModel {
        width: 200,
        height: 100,
        ready: true,
    };
    
    let view = model.view();
    
    // Should handle large dimensions correctly
    assert!(view.contains("200 columns"));
    assert!(view.contains("100 rows"));
    assert!(view.contains("20000 cells")); // 200 * 100 = 20000
}

#[test]
fn test_resize_sequence() {
    let mut model = WindowSizeModel {
        width: 80,
        height: 24,
        ready: true,
    };
    
    // Simulate multiple resize events
    let sizes = [(100, 30), (120, 40), (90, 25)];
    
    for (width, height) in sizes {
        let size_msg = Box::new(WindowSizeMsg { width, height }) as Msg;
        let cmd = model.update(size_msg);
        
        assert_eq!(model.width, width);
        assert_eq!(model.height, height);
        assert!(model.ready);
        assert!(cmd.is_none()); // No commands returned for resize events
    }
}