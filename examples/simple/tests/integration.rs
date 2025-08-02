use bubbletea_rs::{Model, Msg, KeyMsg, QuitMsg};
use crossterm::event::{KeyCode, KeyModifiers};

// Import the model from the main file
// Note: In a real project, you'd typically move the model to a lib.rs file
#[path = "../main.rs"]
mod simple_main;
use simple_main::{SimpleModel, TickMsg};

#[test]
fn test_simple_model_init() {
    let (model, cmd) = SimpleModel::init();
    
    // Should start with count of 5
    assert_eq!(model.count, 5);
    
    // Should return a command (the timer)
    assert!(cmd.is_some());
}

#[test]
fn test_simple_model_view() {
    let model = SimpleModel { count: 3 };
    let view = model.view();
    
    // Should contain the countdown message
    assert!(view.contains("3 seconds"));
    assert!(view.contains("Hi. This program will exit"));
}

#[test]
fn test_simple_model_view_finished() {
    let model = SimpleModel { count: 0 };
    let view = model.view();
    
    // Should show completion message
    assert!(view.contains("Time's up"));
}

#[test]
fn test_tick_message_decrements_count() {
    let mut model = SimpleModel { count: 3 };
    let tick_msg = Box::new(TickMsg) as Msg;
    
    let cmd = model.update(tick_msg);
    
    // Count should be decremented
    assert_eq!(model.count, 2);
    
    // Should not return a quit command yet
    assert!(cmd.is_none());
}

#[test]
fn test_tick_message_at_zero_quits() {
    let mut model = SimpleModel { count: 1 };
    let tick_msg = Box::new(TickMsg) as Msg;
    
    let cmd = model.update(tick_msg);
    
    // Count should be decremented to 0
    assert_eq!(model.count, 0);
    
    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_q_key_quits() {
    let mut model = SimpleModel { count: 3 };
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Count should remain unchanged
    assert_eq!(model.count, 3);
    
    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_uppercase_q_key_quits() {
    let mut model = SimpleModel { count: 3 };
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
    let mut model = SimpleModel { count: 3 };
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_other_keys_ignored() {
    let mut model = SimpleModel { count: 3 };
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Count should remain unchanged
    assert_eq!(model.count, 3);
    
    // Should not return any command
    assert!(cmd.is_none());
}

#[test]
fn test_quit_message_handling() {
    let mut model = SimpleModel { count: 3 };
    let quit_msg = Box::new(QuitMsg) as Msg;
    
    let cmd = model.update(quit_msg);
    
    // Count should remain unchanged
    assert_eq!(model.count, 3);
    
    // Should not return any command (quit is already being processed)
    assert!(cmd.is_none());
}

#[test]
fn test_countdown_sequence() {
    let mut model = SimpleModel { count: 3 };
    
    // Simulate multiple tick messages
    for expected_count in [2, 1, 0] {
        let tick_msg = Box::new(TickMsg) as Msg;
        let cmd = model.update(tick_msg);
        
        assert_eq!(model.count, expected_count);
        
        if expected_count == 0 {
            // Last tick should trigger quit
            assert!(cmd.is_some());
        } else {
            // Previous ticks should not trigger quit
            assert!(cmd.is_none());
        }
    }
}

#[test]
fn test_negative_count_behavior() {
    let mut model = SimpleModel { count: 0 };
    let tick_msg = Box::new(TickMsg) as Msg;
    
    let cmd = model.update(tick_msg);
    
    // Count goes negative
    assert_eq!(model.count, -1);
    
    // Should still trigger quit
    assert!(cmd.is_some());
}

// Test that our custom message type works correctly
#[test]
fn test_tick_msg_type() {
    let tick_msg = Box::new(TickMsg) as Msg;
    
    // Should be able to downcast to TickMsg
    assert!(tick_msg.downcast_ref::<TickMsg>().is_some());
    
    // Should not be able to downcast to other types
    assert!(tick_msg.downcast_ref::<QuitMsg>().is_none());
    assert!(tick_msg.downcast_ref::<KeyMsg>().is_none());
}