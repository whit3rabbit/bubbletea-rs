use bubbletea_rs::{Model, Msg, KeyMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod spinner_main;
use spinner_main::{SpinnerModel, SpinnerStyle, SpinnerTickMsg};

#[test]
fn test_spinner_model_init() {
    let (model, cmd) = SpinnerModel::init();
    
    // Should start with default values
    assert_eq!(model.style, SpinnerStyle::Dots);
    assert_eq!(model.current_frame, 0);
    assert_eq!(model.message, "Loading forever...press q to quit");
    assert!(!model.quitting);
    assert!(model.error.is_none());
    
    // Should start animation command
    assert!(cmd.is_some());
}

#[test]
fn test_spinner_model_new() {
    let model = SpinnerModel::new();
    
    assert_eq!(model.style, SpinnerStyle::Dots);
    assert_eq!(model.current_frame, 0);
    assert_eq!(model.message, "Loading forever...press q to quit");
    assert!(!model.quitting);
    assert!(model.error.is_none());
}

#[test]
fn test_spinner_model_with_style() {
    let model = SpinnerModel::new().with_style(SpinnerStyle::Line);
    
    assert_eq!(model.style, SpinnerStyle::Line);
    assert_eq!(model.current_frame, 0);
}

#[test]
fn test_spinner_model_with_message() {
    let custom_message = "Custom loading message".to_string();
    let model = SpinnerModel::new().with_message(custom_message.clone());
    
    assert_eq!(model.message, custom_message);
}

#[test]
fn test_set_error() {
    let mut model = SpinnerModel::new();
    let error_msg = "Something went wrong".to_string();
    
    model.set_error(error_msg.clone());
    
    assert_eq!(model.error, Some(error_msg));
}

#[test]
fn test_spinner_style_frames() {
    assert_eq!(SpinnerStyle::Dots.frames(), &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]);
    assert_eq!(SpinnerStyle::Line.frames(), &["|", "/", "-", "\\"]);
    assert_eq!(SpinnerStyle::Arc.frames(), &["◜", "◠", "◝", "◞", "◡", "◟"]);
    assert_eq!(SpinnerStyle::Bounce.frames(), &["⠁", "⠂", "⠄", "⠂"]);
    assert_eq!(SpinnerStyle::Clock.frames(), &["🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛"]);
}

#[test]
fn test_spinner_style_intervals() {
    use std::time::Duration;
    
    assert_eq!(SpinnerStyle::Dots.interval(), Duration::from_millis(100));
    assert_eq!(SpinnerStyle::Line.interval(), Duration::from_millis(150));
    assert_eq!(SpinnerStyle::Arc.interval(), Duration::from_millis(120));
    assert_eq!(SpinnerStyle::Bounce.interval(), Duration::from_millis(300));
    assert_eq!(SpinnerStyle::Clock.interval(), Duration::from_millis(500));
}

#[test]
fn test_current_spinner_frame() {
    let mut model = SpinnerModel::new();
    
    // Should start with first frame
    assert_eq!(model.current_spinner_frame(), "⠋");
    
    // Advance frame and check
    model.advance_frame();
    assert_eq!(model.current_spinner_frame(), "⠙");
    
    model.advance_frame();
    assert_eq!(model.current_spinner_frame(), "⠹");
}

#[test]
fn test_advance_frame_wraps_around() {
    let mut model = SpinnerModel::new().with_style(SpinnerStyle::Line);
    let frames = SpinnerStyle::Line.frames();
    
    // Advance through all frames and check wrap around
    for i in 0..frames.len() {
        assert_eq!(model.current_spinner_frame(), frames[i]);
        model.advance_frame();
    }
    
    // Should wrap back to first frame
    assert_eq!(model.current_spinner_frame(), frames[0]);
}

#[test]
fn test_change_style_cycles() {
    let mut model = SpinnerModel::new();
    
    // Start with Dots
    assert_eq!(model.style, SpinnerStyle::Dots);
    
    // Cycle through all styles
    model.change_style();
    assert_eq!(model.style, SpinnerStyle::Line);
    assert_eq!(model.current_frame, 0); // Should reset frame
    
    model.change_style();
    assert_eq!(model.style, SpinnerStyle::Arc);
    
    model.change_style();
    assert_eq!(model.style, SpinnerStyle::Bounce);
    
    model.change_style();
    assert_eq!(model.style, SpinnerStyle::Clock);
    
    model.change_style();
    assert_eq!(model.style, SpinnerStyle::Dots); // Should cycle back
}

#[test]
fn test_spinner_tick_message_advances_frame() {
    let mut model = SpinnerModel::new();
    let initial_frame = model.current_frame;
    
    let tick_msg = Box::new(SpinnerTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Frame should advance
    assert_eq!(model.current_frame, initial_frame + 1);
    
    // Should return a new tick command
    assert!(cmd.is_some());
}

#[test]
fn test_spinner_tick_when_quitting() {
    let mut model = SpinnerModel::new();
    model.quitting = true;
    
    let tick_msg = Box::new(SpinnerTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Should not return tick command when quitting
    assert!(cmd.is_none());
}

#[test]
fn test_q_key_quits() {
    let mut model = SpinnerModel::new();
    
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
    let mut model = SpinnerModel::new();
    
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
    let mut model = SpinnerModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(model.quitting);
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_space_changes_style() {
    let mut model = SpinnerModel::new();
    let initial_style = model.style.clone();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char(' '),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Style should change
    assert_ne!(model.style, initial_style);
    assert_eq!(model.style, SpinnerStyle::Line); // Should be next style
    assert_eq!(model.current_frame, 0); // Should reset frame
    
    // Should return new tick command with new interval
    assert!(cmd.is_some());
}

#[test]
fn test_view_normal() {
    let model = SpinnerModel::new();
    let view = model.view();
    
    assert!(view.contains("⠋")); // Should show spinner frame
    assert!(view.contains("Loading forever...press q to quit"));
    assert!(view.contains("Style: Dots"));
    assert!(view.contains("press space to change"));
}

#[test]
fn test_view_with_error() {
    let mut model = SpinnerModel::new();
    model.set_error("Test error".to_string());
    
    let view = model.view();
    
    assert!(view.contains("Error: Test error"));
    assert!(!view.contains("⠋")); // Should not show spinner when error
}

#[test]
fn test_view_when_quitting() {
    let mut model = SpinnerModel::new();
    model.quitting = true;
    
    let view = model.view();
    
    // Should end with extra newline when quitting
    assert!(view.ends_with("\n\n"));
}

#[test]
fn test_view_different_styles() {
    let styles = [
        (SpinnerStyle::Dots, "Dots", "⠋"),
        (SpinnerStyle::Line, "Line", "|"),
        (SpinnerStyle::Arc, "Arc", "◜"),
        (SpinnerStyle::Bounce, "Bounce", "⠁"),
        (SpinnerStyle::Clock, "Clock", "🕐"),
    ];
    
    for (style, style_name, first_frame) in styles {
        let model = SpinnerModel::new().with_style(style);
        let view = model.view();
        
        assert!(view.contains(first_frame));
        assert!(view.contains(&format!("Style: {}", style_name)));
    }
}

#[test]
fn test_custom_message() {
    let custom_message = "Processing data...".to_string();
    let model = SpinnerModel::new().with_message(custom_message.clone());
    let view = model.view();
    
    assert!(view.contains(&custom_message));
    assert!(!view.contains("Loading forever"));
}

#[test]
fn test_frame_animation_sequence() {
    let mut model = SpinnerModel::new().with_style(SpinnerStyle::Line);
    let expected_frames = ["|", "/", "-", "\\", "|"]; // Should wrap around
    
    for expected_frame in expected_frames {
        assert_eq!(model.current_spinner_frame(), expected_frame);
        model.advance_frame();
    }
}

#[test]
fn test_all_spinner_styles_have_frames() {
    let styles = [
        SpinnerStyle::Dots,
        SpinnerStyle::Line,
        SpinnerStyle::Arc,
        SpinnerStyle::Bounce,
        SpinnerStyle::Clock,
    ];
    
    for style in styles {
        let frames = style.frames();
        assert!(!frames.is_empty(), "Style {:?} should have frames", style);
        
        let interval = style.interval();
        assert!(interval.as_millis() > 0, "Style {:?} should have positive interval", style);
    }
}

#[test]
fn test_spinner_model_debug() {
    let model = SpinnerModel::new();
    let debug_str = format!("{:?}", model);
    
    // Should be able to debug print the model
    assert!(debug_str.contains("SpinnerModel"));
}

#[test]
fn test_spinner_style_debug() {
    let style = SpinnerStyle::Dots;
    let debug_str = format!("{:?}", style);
    
    // Should be able to debug print the style
    assert!(debug_str.contains("Dots"));
}

#[test]
fn test_unknown_key_does_nothing() {
    let mut model = SpinnerModel::new();
    let initial_state = format!("{:?}", model.style);
    let initial_quitting = model.quitting;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not change state
    assert_eq!(format!("{:?}", model.style), initial_state);
    assert_eq!(model.quitting, initial_quitting);
    assert!(cmd.is_none());
}