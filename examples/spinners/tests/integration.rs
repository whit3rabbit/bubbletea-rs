use bubbletea_rs::{Model, Msg, KeyMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod spinners_main;
use spinners_main::{SpinnersModel, SpinnerStyle, SpinnerTickMsg};

#[test]
fn test_spinners_model_new() {
    let model = SpinnersModel::new();
    
    assert_eq!(model.current_index, 0);
    assert_eq!(model.current_frame, 0);
    assert!(!model.quitting);
    assert_eq!(model.spinners.len(), 12); // Should have 12 different spinner styles
}

#[test]
fn test_spinners_model_init() {
    let (model, cmd) = SpinnersModel::init();
    
    // Should start with default values
    assert_eq!(model.current_index, 0);
    assert_eq!(model.current_frame, 0);
    assert!(!model.quitting);
    
    // Should start spinner animation
    assert!(cmd.is_some());
}

#[test]
fn test_spinner_style_all() {
    let all_styles = SpinnerStyle::all();
    
    assert_eq!(all_styles.len(), 12);
    assert_eq!(all_styles[0], SpinnerStyle::Line);
    assert_eq!(all_styles[1], SpinnerStyle::Dots);
    assert_eq!(all_styles[11], SpinnerStyle::Clock);
}

#[test]
fn test_spinner_style_frames() {
    assert_eq!(SpinnerStyle::Line.frames(), &["|", "/", "-", "\\"]);
    assert_eq!(SpinnerStyle::Dots.frames(), &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]);
    assert_eq!(SpinnerStyle::Points.frames(), &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"]);
    assert_eq!(SpinnerStyle::Globe.frames(), &["🌍", "🌎", "🌏"]);
}

#[test]
fn test_spinner_style_intervals() {
    use std::time::Duration;
    
    assert_eq!(SpinnerStyle::Line.interval(), Duration::from_millis(150));
    assert_eq!(SpinnerStyle::Dots.interval(), Duration::from_millis(100));
    assert_eq!(SpinnerStyle::Pulse.interval(), Duration::from_millis(80));
    assert_eq!(SpinnerStyle::Globe.interval(), Duration::from_millis(500));
}

#[test]
fn test_spinner_style_names() {
    assert_eq!(SpinnerStyle::Line.name(), "Line");
    assert_eq!(SpinnerStyle::Dots.name(), "Dots");
    assert_eq!(SpinnerStyle::MiniDots.name(), "Mini Dots");
    assert_eq!(SpinnerStyle::Globe.name(), "Globe");
}

#[test]
fn test_spinner_style_descriptions() {
    assert_eq!(SpinnerStyle::Line.description(), "Classic rotating line");
    assert_eq!(SpinnerStyle::Dots.description(), "Braille dot pattern");
    assert_eq!(SpinnerStyle::Globe.description(), "Rotating earth emoji");
    assert_eq!(SpinnerStyle::Clock.description(), "Clock face animation");
}

#[test]
fn test_current_spinner() {
    let model = SpinnersModel::new();
    
    // Should start with first spinner
    assert_eq!(model.current_spinner(), &SpinnerStyle::Line);
}

#[test]
fn test_current_frame_text() {
    let model = SpinnersModel::new();
    
    // Should start with first frame of Line spinner
    assert_eq!(model.current_frame_text(), "|");
}

#[test]
fn test_advance_frame() {
    let mut model = SpinnersModel::new();
    
    // Should start with first frame
    assert_eq!(model.current_frame_text(), "|");
    
    // Advance and check
    model.advance_frame();
    assert_eq!(model.current_frame_text(), "/");
    
    model.advance_frame();
    assert_eq!(model.current_frame_text(), "-");
    
    model.advance_frame();
    assert_eq!(model.current_frame_text(), "\\");
    
    // Should wrap around
    model.advance_frame();
    assert_eq!(model.current_frame_text(), "|");
}

#[test]
fn test_previous_spinner() {
    let mut model = SpinnersModel::new();
    
    // Should start at index 0 (Line)
    assert_eq!(model.current_index, 0);
    assert_eq!(model.current_spinner(), &SpinnerStyle::Line);
    
    // Go to previous (should wrap to last)
    model.previous_spinner();
    assert_eq!(model.current_index, 11); // Last index
    assert_eq!(model.current_spinner(), &SpinnerStyle::Clock);
    assert_eq!(model.current_frame, 0); // Should reset frame
    
    // Go to previous again
    model.previous_spinner();
    assert_eq!(model.current_index, 10);
    assert_eq!(model.current_spinner(), &SpinnerStyle::Bounce);
}

#[test]
fn test_next_spinner() {
    let mut model = SpinnersModel::new();
    
    // Should start at index 0 (Line)
    assert_eq!(model.current_index, 0);
    assert_eq!(model.current_spinner(), &SpinnerStyle::Line);
    
    // Go to next
    model.next_spinner();
    assert_eq!(model.current_index, 1);
    assert_eq!(model.current_spinner(), &SpinnerStyle::Dots);
    assert_eq!(model.current_frame, 0); // Should reset frame
    
    // Go to next again
    model.next_spinner();
    assert_eq!(model.current_index, 2);
    assert_eq!(model.current_spinner(), &SpinnerStyle::MiniDots);
}

#[test]
fn test_next_spinner_wraps_around() {
    let mut model = SpinnersModel::new();
    model.current_index = 11; // Last index
    
    // Go to next (should wrap to first)
    model.next_spinner();
    assert_eq!(model.current_index, 0);
    assert_eq!(model.current_spinner(), &SpinnerStyle::Line);
}

#[test]  
fn test_spinner_tick_message() {
    let mut model = SpinnersModel::new();
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
    let mut model = SpinnersModel::new();
    model.quitting = true;
    let initial_frame = model.current_frame;
    
    let tick_msg = Box::new(SpinnerTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Frame should not advance when quitting
    assert_eq!(model.current_frame, initial_frame);
    
    // Should not return tick command when quitting
    assert!(cmd.is_none());
}

#[test]
fn test_q_key_quits() {
    let mut model = SpinnersModel::new();
    
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
    let mut model = SpinnersModel::new();
    
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
    let mut model = SpinnersModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(model.quitting);
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_left_arrow_previous_spinner() {
    let mut model = SpinnersModel::new();
    let initial_index = model.current_index;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should go to previous spinner (wrapping around)
    assert_eq!(model.current_index, 11); // Wrapped to last
    assert_ne!(model.current_index, initial_index);
    assert_eq!(model.current_frame, 0); // Should reset frame
    
    // Should return new tick command with new interval
    assert!(cmd.is_some());
}

#[test]
fn test_right_arrow_next_spinner() {
    let mut model = SpinnersModel::new();
    let initial_index = model.current_index;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should go to next spinner
    assert_eq!(model.current_index, initial_index + 1);
    assert_eq!(model.current_frame, 0); // Should reset frame
    
    // Should return new tick command with new interval
    assert!(cmd.is_some());
}

#[test]
fn test_h_key_previous_spinner() {
    let mut model = SpinnersModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('h'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should go to previous spinner
    assert_eq!(model.current_index, 11); // Wrapped to last
    assert!(cmd.is_some());
}

#[test]
fn test_l_key_next_spinner() {
    let mut model = SpinnersModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('l'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should go to next spinner
    assert_eq!(model.current_index, 1);
    assert!(cmd.is_some());
}

#[test]
fn test_view_initial_state() {
    let model = SpinnersModel::new();
    let view = model.view();
    
    assert!(view.contains("|")); // Should show Line spinner frame
    assert!(view.contains("Spinning..."));
    assert!(view.contains("Style: Line (1/12)"));
    assert!(view.contains("Description: Classic rotating line"));
    assert!(view.contains("Interval: 150ms"));
    assert!(view.contains("Frames: 4"));
    assert!(view.contains("h/l, ←/→: change spinner"));
    assert!(view.contains("q: exit"));
}

#[test]
fn test_view_different_spinners() {
    let mut model = SpinnersModel::new();
    
    // Test Line spinner
    let view = model.view();
    assert!(view.contains("Style: Line (1/12)"));
    assert!(view.contains("Classic rotating line"));
    
    // Switch to Dots
    model.next_spinner();
    let view = model.view();
    assert!(view.contains("Style: Dots (2/12)"));
    assert!(view.contains("Braille dot pattern"));
    
    // Switch to Globe
    model.current_index = 6; // Globe is at index 6
    let view = model.view();
    assert!(view.contains("Style: Globe (7/12)"));
    assert!(view.contains("Rotating earth emoji"));
}

#[test]
fn test_view_when_quitting() {
    let mut model = SpinnersModel::new();
    model.quitting = true;
    
    let view = model.view();
    
    // Should end with extra newline when quitting
    assert!(view.ends_with("\n\n"));
}

#[test]
fn test_view_gap_handling() {
    let mut model = SpinnersModel::new();
    
    // Line spinner should have gap
    assert_eq!(model.current_spinner(), &SpinnerStyle::Line);
    let view = model.view();
    assert!(view.contains("| Spinning...")); // With space
    
    // Dots spinner should have no gap
    model.current_index = 1; // Dots
    let view = model.view();
    assert!(view.contains("⠋Spinning...")); // No space
}

#[test]
fn test_all_spinner_styles_have_properties() {
    let styles = SpinnerStyle::all();
    
    for style in styles {
        // All styles should have frames
        let frames = style.frames();
        assert!(!frames.is_empty(), "Style {:?} should have frames", style);
        
        // All styles should have positive interval
        let interval = style.interval();
        assert!(interval.as_millis() > 0, "Style {:?} should have positive interval", style);
        
        // All styles should have names and descriptions
        let name = style.name();
        assert!(!name.is_empty(), "Style {:?} should have a name", style);
        
        let description = style.description();
        assert!(!description.is_empty(), "Style {:?} should have a description", style);
    }
}

#[test]
fn test_spinner_style_debug() {
    let style = SpinnerStyle::Line;
    let debug_str = format!("{:?}", style);
    
    assert!(debug_str.contains("Line"));
}

#[test]
fn test_spinners_model_debug() {
    let model = SpinnersModel::new();
    let debug_str = format!("{:?}", model);
    
    assert!(debug_str.contains("SpinnersModel"));
}

#[test]
fn test_spinner_tick_msg_debug() {
    let msg = SpinnerTickMsg;
    let debug_str = format!("{:?}", msg);
    
    assert!(debug_str.contains("SpinnerTickMsg"));
}

#[test]
fn test_unknown_key_does_nothing() {
    let mut model = SpinnersModel::new();
    let initial_index = model.current_index;
    let initial_frame = model.current_frame;
    let initial_quitting = model.quitting;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Should not change state
    assert_eq!(model.current_index, initial_index);
    assert_eq!(model.current_frame, initial_frame);
    assert_eq!(model.quitting, initial_quitting);
    assert!(cmd.is_none());
}

#[test]
fn test_full_cycle_navigation() {
    let mut model = SpinnersModel::new();
    let total_spinners = model.spinners.len();
    
    // Navigate through all spinners forward
    for i in 0..total_spinners {
        assert_eq!(model.current_index, i);
        model.next_spinner();
    }
    
    // Should wrap back to first
    assert_eq!(model.current_index, 0);
    
    // Navigate through all spinners backward
    // Start from 0, so the sequence should be: 0 -> 11 -> 10 -> 9 -> ... -> 1 -> 0
    for expected_index in [11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0] {
        model.previous_spinner();
        assert_eq!(model.current_index, expected_index);
    }
    
    // After going through all, we should be back at 0, so one more previous should give us 11
    model.previous_spinner();
    assert_eq!(model.current_index, total_spinners - 1);
}

#[test]
fn test_frame_reset_on_spinner_change() {
    let mut model = SpinnersModel::new();
    
    // Advance frame on current spinner
    model.advance_frame();
    model.advance_frame();
    assert_eq!(model.current_frame, 2);
    
    // Change spinner - frame should reset
    model.next_spinner();
    assert_eq!(model.current_frame, 0);
    
    // Advance frame on new spinner
    model.advance_frame();
    assert_eq!(model.current_frame, 1);
    
    // Change spinner again - frame should reset
    model.previous_spinner();
    assert_eq!(model.current_frame, 0);
}

#[test]
fn test_specific_spinner_characteristics() {
    // Test specific characteristics we expect
    assert_eq!(SpinnerStyle::Pulse.frames().len(), 14);
    assert_eq!(SpinnerStyle::Moon.frames().len(), 8);
    assert_eq!(SpinnerStyle::Globe.frames().len(), 3);
    assert_eq!(SpinnerStyle::Line.frames().len(), 4);
    
    // Test intervals are different
    assert_ne!(SpinnerStyle::Pulse.interval(), SpinnerStyle::Globe.interval());
    assert_ne!(SpinnerStyle::Dots.interval(), SpinnerStyle::Points.interval());
}