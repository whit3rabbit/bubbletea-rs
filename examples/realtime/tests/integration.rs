use bubbletea_rs::{Model, Msg, KeyMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod realtime_main;
use realtime_main::{RealtimeModel, SpinnerStyle, SpinnerTickMsg, ActivityMsg};

#[test]
fn test_realtime_model_new() {
    let (model, _tx) = RealtimeModel::new();
    
    assert_eq!(model.events_received, 0);
    assert_eq!(model.spinner_style, SpinnerStyle::Dots);
    assert_eq!(model.spinner_frame, 0);
    assert!(!model.quitting);
    assert_eq!(model.last_event_id, 0);
    assert!(model.activity_receiver.is_some());
    assert!(!model.activity_started);
}

#[test]
fn test_realtime_model_init() {
    let (model, cmd) = RealtimeModel::init();
    
    // Should start with default values
    assert_eq!(model.events_received, 0);
    assert_eq!(model.spinner_style, SpinnerStyle::Dots);
    assert_eq!(model.spinner_frame, 0);
    assert!(!model.quitting);
    assert_eq!(model.last_event_id, 0);
    assert!(!model.activity_started);
    
    // Should start spinner animation
    assert!(cmd.is_some());
}

#[test]
fn test_spinner_styles() {
    assert_eq!(SpinnerStyle::Dots.frames(), &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]);
    assert_eq!(SpinnerStyle::Line.frames(), &["|", "/", "-", "\\"]);
    assert_eq!(SpinnerStyle::Arc.frames(), &["◜", "◠", "◝", "◞", "◡", "◟"]);
}

#[test]
fn test_spinner_intervals() {
    use std::time::Duration;
    
    assert_eq!(SpinnerStyle::Dots.interval(), Duration::from_millis(100));
    assert_eq!(SpinnerStyle::Line.interval(), Duration::from_millis(100));
    assert_eq!(SpinnerStyle::Arc.interval(), Duration::from_millis(100));
}

#[test]
fn test_current_spinner_frame() {
    let (model, _tx) = RealtimeModel::new();
    
    // Should start with first frame
    assert_eq!(model.current_spinner_frame(), "⠋");
}

#[test]
fn test_advance_spinner() {
    let (mut model, _tx) = RealtimeModel::new();
    
    // Should start with first frame
    assert_eq!(model.current_spinner_frame(), "⠋");
    
    // Advance and check
    model.advance_spinner();
    assert_eq!(model.current_spinner_frame(), "⠙");
    
    model.advance_spinner();
    assert_eq!(model.current_spinner_frame(), "⠹");
}

#[test]
fn test_advance_spinner_wraps_around() {
    let (mut model, _tx) = RealtimeModel::new();
    model.spinner_style = SpinnerStyle::Line;
    model.spinner_frame = 0;
    
    let frames = SpinnerStyle::Line.frames();
    
    // Advance through all frames
    for i in 0..frames.len() {
        assert_eq!(model.current_spinner_frame(), frames[i]);
        model.advance_spinner();
    }
    
    // Should wrap back to first frame
    assert_eq!(model.current_spinner_frame(), frames[0]);
}

#[test]
fn test_record_activity() {
    let (mut model, _tx) = RealtimeModel::new();
    
    assert_eq!(model.events_received, 0);
    assert_eq!(model.last_event_id, 0);
    
    model.record_activity(42);
    
    assert_eq!(model.events_received, 1);
    assert_eq!(model.last_event_id, 42);
    
    model.record_activity(123);
    
    assert_eq!(model.events_received, 2);
    assert_eq!(model.last_event_id, 123);
}

#[test]
fn test_spinner_tick_message() {
    let (mut model, _tx) = RealtimeModel::new();
    let initial_frame = model.spinner_frame;
    
    let tick_msg = Box::new(SpinnerTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Frame should advance
    assert_eq!(model.spinner_frame, initial_frame + 1);
    
    // Should return a new tick command
    assert!(cmd.is_some());
}

#[test]
fn test_spinner_tick_when_quitting() {
    let (mut model, _tx) = RealtimeModel::new();
    model.quitting = true;
    let initial_frame = model.spinner_frame;
    
    let tick_msg = Box::new(SpinnerTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Frame should not advance when quitting
    assert_eq!(model.spinner_frame, initial_frame);
    
    // Should not return tick command when quitting
    assert!(cmd.is_none());
}

#[test]
fn test_activity_message() {
    let (mut model, _tx) = RealtimeModel::new();
    
    let activity_msg = Box::new(ActivityMsg { event_id: 555 }) as Msg;
    let cmd = model.update(activity_msg);
    
    // Should record the activity
    assert_eq!(model.events_received, 1);
    assert_eq!(model.last_event_id, 555);
    
    // Should return a simulate_activity command to continue the simulation
    assert!(cmd.is_some());
}

#[test]
fn test_q_key_quits() {
    let (mut model, _tx) = RealtimeModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(model.quitting);
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_esc_key_quits() {
    let (mut model, _tx) = RealtimeModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(model.quitting);
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_ctrl_c_quits() {
    let (mut model, _tx) = RealtimeModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(model.quitting);
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_space_changes_spinner_style() {
    let (mut model, _tx) = RealtimeModel::new();
    
    // Should start with Dots
    assert_eq!(model.spinner_style, SpinnerStyle::Dots);
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char(' '),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg);
    
    // Should change to Line
    assert_eq!(model.spinner_style, SpinnerStyle::Line);
    assert_eq!(model.spinner_frame, 0); // Should reset frame
}

#[test]
fn test_space_cycles_spinner_styles() {
    let (mut model, _tx) = RealtimeModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char(' '),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    // Dots -> Line
    assert_eq!(model.spinner_style, SpinnerStyle::Dots);
    model.update(key_msg);
    assert_eq!(model.spinner_style, SpinnerStyle::Line);
    
    // Line -> Arc
    let key_msg2 = Box::new(KeyMsg {
        key: KeyCode::Char(' '),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(key_msg2);
    assert_eq!(model.spinner_style, SpinnerStyle::Arc);
    
    // Arc -> Dots
    let key_msg3 = Box::new(KeyMsg {
        key: KeyCode::Char(' '),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    model.update(key_msg3);
    assert_eq!(model.spinner_style, SpinnerStyle::Dots);
}

#[test]
fn test_r_key_resets_counter() {
    let (mut model, _tx) = RealtimeModel::new();
    
    // Set some values
    model.record_activity(123);
    model.record_activity(456);
    assert_eq!(model.events_received, 2);
    assert_eq!(model.last_event_id, 456);
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('r'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg);
    
    // Should reset counters
    assert_eq!(model.events_received, 0);
    assert_eq!(model.last_event_id, 0);
}

#[test]
fn test_view_initial_state() {
    let (model, _tx) = RealtimeModel::new();
    let view = model.view();
    
    assert!(view.contains("⠋")); // Spinner frame
    assert!(view.contains("Events received: 0"));
    assert!(view.contains("Spinner: Dots"));
    assert!(view.contains("space to change"));
    assert!(view.contains("Press 'r' to reset"));
    assert!(view.contains("Press any other key to exit"));
    assert!(!view.contains("Last event ID")); // Should not show when no events
}

#[test]
fn test_view_with_events() {
    let (mut model, _tx) = RealtimeModel::new();
    model.record_activity(42);
    model.record_activity(123);
    
    let view = model.view();
    
    assert!(view.contains("Events received: 2"));
    assert!(view.contains("Last event ID: 123"));
}

#[test]
fn test_view_different_spinner_styles() {
    let styles = [
        (SpinnerStyle::Dots, "Dots", "⠋"),
        (SpinnerStyle::Line, "Line", "|"),
        (SpinnerStyle::Arc, "Arc", "◜"),
    ];
    
    for (style, style_name, first_frame) in styles {
        let (mut model, _tx) = RealtimeModel::new();
        model.spinner_style = style;
        model.spinner_frame = 0;
        
        let view = model.view();
        
        assert!(view.contains(first_frame));
        assert!(view.contains(&format!("Spinner: {}", style_name)));
    }
}

#[test]
fn test_view_when_quitting() {
    let (mut model, _tx) = RealtimeModel::new();
    model.quitting = true;
    
    let view = model.view();
    
    // Should end with extra newline when quitting
    assert!(view.ends_with("\n\n"));
}

#[test]
fn test_activity_msg_debug() {
    let msg = ActivityMsg { event_id: 42 };
    let debug_str = format!("{:?}", msg);
    
    assert!(debug_str.contains("ActivityMsg"));
    assert!(debug_str.contains("42"));
}

#[test]
fn test_spinner_tick_msg_debug() {
    let msg = SpinnerTickMsg;
    let debug_str = format!("{:?}", msg);
    
    assert!(debug_str.contains("SpinnerTickMsg"));
}

#[test]
fn test_spinner_style_debug() {
    let style = SpinnerStyle::Dots;
    let debug_str = format!("{:?}", style);
    
    assert!(debug_str.contains("Dots"));
}

#[test]
fn test_realtime_model_debug() {
    let (model, _tx) = RealtimeModel::new();
    let debug_str = format!("{:?}", model);
    
    assert!(debug_str.contains("RealtimeModel"));
}

#[test]
fn test_unknown_key_does_nothing() {
    let (mut model, _tx) = RealtimeModel::new();
    let initial_events = model.events_received;
    let initial_style = model.spinner_style.clone();
    let initial_quitting = model.quitting;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not change state
    assert_eq!(model.events_received, initial_events);
    assert_eq!(model.spinner_style, initial_style);
    assert_eq!(model.quitting, initial_quitting);
    assert!(cmd.is_none());
}

#[test]
fn test_multiple_activities() {
    let (mut model, _tx) = RealtimeModel::new();
    
    // Simulate multiple activities
    for i in 1..=5 {
        let activity_msg = Box::new(ActivityMsg { event_id: i * 10 }) as Msg;
        model.update(activity_msg);
    }
    
    assert_eq!(model.events_received, 5);
    assert_eq!(model.last_event_id, 50);
}

#[test]
fn test_spinner_animation_sequence() {
    let (mut model, _tx) = RealtimeModel::new();
    model.spinner_style = SpinnerStyle::Line;
    model.spinner_frame = 0;
    
    let expected_frames = ["|", "/", "-", "\\", "|"]; // Should wrap around
    
    for expected_frame in expected_frames {
        assert_eq!(model.current_spinner_frame(), expected_frame);
        model.advance_spinner();
    }
}

#[test]
fn test_reset_after_style_change() {
    let (mut model, _tx) = RealtimeModel::new();
    
    // Advance spinner frame
    model.advance_spinner();
    model.advance_spinner();
    assert_eq!(model.spinner_frame, 2);
    
    // Change style with space
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char(' '),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg);
    
    // Frame should reset to 0
    assert_eq!(model.spinner_frame, 0);
    assert_eq!(model.spinner_style, SpinnerStyle::Line);
}