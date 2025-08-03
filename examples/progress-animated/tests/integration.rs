use bubbletea_rs::{KeyMsg, Model, Msg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod progress_animated_main;
use progress_animated_main::{
    AnimatedProgressBar, ProgressAnimatedModel, ProgressFrameMsg, ProgressTickMsg,
};

#[test]
fn test_animated_progress_bar_new() {
    let progress = AnimatedProgressBar::new();

    assert_eq!(progress.width, 40);
    assert_eq!(progress.current_percent, 0.0);
    assert_eq!(progress.target_percent, 0.0);
    assert_eq!(progress.filled_char, '█');
    assert_eq!(progress.empty_char, '░');
    assert_eq!(progress.animation_speed, 0.1);
}

#[test]
fn test_animated_progress_bar_set_percent() {
    let mut progress = AnimatedProgressBar::new();

    // Set target to 50%
    let cmd = progress.set_percent(0.5);
    assert_eq!(progress.target_percent, 0.5);
    assert_eq!(progress.current_percent, 0.0); // Hasn't animated yet
    assert!(cmd.is_some()); // Should start animation

    // Set to same value - no animation needed
    let cmd = progress.set_percent(0.0);
    assert_eq!(progress.target_percent, 0.0);
    assert!(cmd.is_none()); // No animation needed
}

#[test]
fn test_animated_progress_bar_incr_percent() {
    let mut progress = AnimatedProgressBar::new();

    // Increment by 25%
    let cmd = progress.incr_percent(0.25);
    assert_eq!(progress.target_percent, 0.25);
    assert!(cmd.is_some()); // Should start animation

    // Increment again
    progress.incr_percent(0.25);
    assert_eq!(progress.target_percent, 0.5);
}

#[test]
fn test_animated_progress_bar_update_animation() {
    let mut progress = AnimatedProgressBar::new();
    progress.target_percent = 1.0;
    progress.current_percent = 0.0;

    // Should animate toward target
    let cmd = progress.update_animation();
    assert!(progress.current_percent > 0.0);
    assert!(progress.current_percent < 1.0);
    assert!(cmd.is_some()); // Should continue animation

    // When at target, animation should stop
    progress.current_percent = 1.0;
    let cmd = progress.update_animation();
    assert!(cmd.is_none()); // Animation complete
}

#[test]
fn test_animated_progress_bar_percent() {
    let mut progress = AnimatedProgressBar::new();
    progress.current_percent = 0.75;

    assert_eq!(progress.percent(), 0.75);
}

#[test]
fn test_animated_progress_bar_view() {
    let mut progress = AnimatedProgressBar::new();
    progress.current_percent = 0.5;

    let view = progress.view();
    assert!(view.contains("█")); // Filled portion
    assert!(view.contains("░")); // Empty portion
    assert!(view.contains("50.0%"));

    // Test 0%
    progress.current_percent = 0.0;
    let view = progress.view();
    assert!(view.contains("0.0%"));
    let empty_count = view.chars().filter(|&c| c == '░').count();
    assert_eq!(empty_count, 40);

    // Test 100%
    progress.current_percent = 1.0;
    let view = progress.view();
    assert!(view.contains("100.0%"));
    let filled_count = view.chars().filter(|&c| c == '█').count();
    assert_eq!(filled_count, 40);
}

#[test]
fn test_animated_progress_bar_clamp() {
    let mut progress = AnimatedProgressBar::new();

    // Test negative value gets clamped
    progress.set_percent(-0.5);
    assert_eq!(progress.target_percent, 0.0);

    // Test value > 1.0 gets clamped
    progress.set_percent(1.5);
    assert_eq!(progress.target_percent, 1.0);
}

#[test]
fn test_progress_animated_model_new() {
    let model = ProgressAnimatedModel::new();

    assert_eq!(model.progress.current_percent, 0.0);
    assert_eq!(model.progress.target_percent, 0.0);
    assert_eq!(model.progress.width, 40);
}

#[test]
fn test_progress_animated_model_init() {
    let (model, cmd) = ProgressAnimatedModel::init();

    // Should start with default values
    assert_eq!(model.progress.percent(), 0.0);

    // Should start progress updates
    assert!(cmd.is_some());
}

#[test]
fn test_update_window_size() {
    let mut model = ProgressAnimatedModel::new();

    model.update_window_size(100, 30);
    assert_eq!(model.progress.width, 80); // Max width cap

    // Test smaller window
    model.update_window_size(50, 20);
    assert_eq!(model.progress.width, 42); // 50 - 8 (padding)

    // Test very small window
    model.update_window_size(10, 5);
    assert_eq!(model.progress.width, 2); // 10 - 8 (padding)
}

#[test]
fn test_progress_tick_message() {
    let mut model = ProgressAnimatedModel::new();
    let initial_target = model.progress.target_percent;

    let tick_msg = Box::new(ProgressTickMsg) as Msg;
    let cmd = model.update(tick_msg);

    // Target progress should advance by 25%
    assert_eq!(model.progress.target_percent, initial_target + 0.25);

    // Should return batched commands (tick + animation)
    assert!(cmd.is_some());
}

#[test]
fn test_progress_tick_completion() {
    let mut model = ProgressAnimatedModel::new();
    model.progress.current_percent = 1.0;
    model.progress.target_percent = 1.0;

    let tick_msg = Box::new(ProgressTickMsg) as Msg;
    let cmd = model.update(tick_msg);

    // Should quit when at 100%
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_progress_frame_message() {
    let mut model = ProgressAnimatedModel::new();
    model.progress.target_percent = 0.5;
    model.progress.current_percent = 0.0;

    let frame_msg = Box::new(ProgressFrameMsg) as Msg;
    let cmd = model.update(frame_msg);

    // Should animate toward target
    assert!(model.progress.current_percent > 0.0);
    assert!(model.progress.current_percent < 0.5);
    assert!(cmd.is_some()); // Should continue animation
}

#[test]
fn test_window_size_message() {
    let mut model = ProgressAnimatedModel::new();

    let size_msg = Box::new(WindowSizeMsg {
        width: 120,
        height: 40,
    }) as Msg;

    let cmd = model.update(size_msg);

    assert_eq!(model.progress.width, 80); // Max width
    assert!(cmd.is_none());
}

#[test]
fn test_key_message_quits() {
    let mut model = ProgressAnimatedModel::new();

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_any_key_quits() {
    let keys = [
        KeyCode::Char('a'),
        KeyCode::Char(' '),
        KeyCode::Enter,
        KeyCode::Esc,
        KeyCode::Tab,
    ];

    for key in keys {
        let mut model = ProgressAnimatedModel::new();
        let key_msg = Box::new(KeyMsg {
            key,
            modifiers: KeyModifiers::NONE,
        }) as Msg;

        let cmd = model.update(key_msg);
        assert!(cmd.is_some(), "Key {:?} should quit", key);
    }
}

#[test]
fn test_ctrl_c_quits() {
    let mut model = ProgressAnimatedModel::new();

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_view_initial_state() {
    let model = ProgressAnimatedModel::new();
    let view = model.view();

    assert!(view.contains("0.0%"));
    assert!(view.contains("Press any key to quit"));
    assert!(view.contains("░")); // Should show empty progress bar

    // Should have proper padding
    assert!(view.starts_with('\n'));
    let lines: Vec<&str> = view.lines().collect();
    assert!(lines.len() >= 3);
    assert!(lines[1].starts_with("  ")); // Progress line has padding
    assert!(lines[3].starts_with("  ")); // Help line has padding
}

#[test]
fn test_view_in_progress() {
    let mut model = ProgressAnimatedModel::new();
    model.progress.current_percent = 0.5;

    let view = model.view();

    assert!(view.contains("50.0%"));
    assert!(view.contains("█")); // Should show filled progress
    assert!(view.contains("░")); // Should show empty progress
    assert!(view.contains("Press any key to quit"));
}

#[test]
fn test_view_completed() {
    let mut model = ProgressAnimatedModel::new();
    model.progress.current_percent = 1.0;

    let view = model.view();

    assert!(view.contains("100.0%"));
    assert!(view.contains("Press any key to quit"));

    // Should be all filled at 100%
    let filled_count = view.chars().filter(|&c| c == '█').count();
    assert_eq!(filled_count, 40);
}

#[test]
fn test_animation_progression() {
    let mut progress = AnimatedProgressBar::new();

    // Set target to 100%
    progress.set_percent(1.0);
    assert_eq!(progress.target_percent, 1.0);
    assert_eq!(progress.current_percent, 0.0);

    // Animate several frames
    let mut animation_steps = 0;
    while (progress.current_percent - progress.target_percent).abs() > 0.001
        && animation_steps < 100
    {
        let prev_percent = progress.current_percent;
        progress.update_animation();

        // Should make progress each frame
        assert!(progress.current_percent > prev_percent);
        assert!(progress.current_percent <= 1.0);

        animation_steps += 1;
    }

    // Should eventually reach target
    assert!((progress.current_percent - 1.0).abs() < 0.001);
    assert!(animation_steps > 0); // Should have taken some steps
}

#[test]
fn test_different_animation_speeds() {
    let mut progress = AnimatedProgressBar::new();
    progress.animation_speed = 0.2; // Faster animation
    progress.set_percent(1.0);

    let initial_percent = progress.current_percent;
    progress.update_animation();
    let fast_step = progress.current_percent - initial_percent;

    // Reset and try slower animation
    let mut progress = AnimatedProgressBar::new();
    progress.animation_speed = 0.05; // Slower animation
    progress.set_percent(1.0);

    let initial_percent = progress.current_percent;
    progress.update_animation();
    let slow_step = progress.current_percent - initial_percent;

    // Faster should take bigger steps
    assert!(fast_step > slow_step);
}

#[test]
fn test_animation_smoothness() {
    let mut progress = AnimatedProgressBar::new();
    progress.set_percent(0.5);

    let mut prev_percent = progress.current_percent;
    let mut step_sizes = Vec::new();

    // Collect several animation steps
    for _ in 0..10 {
        progress.update_animation();
        let step = progress.current_percent - prev_percent;
        if step > 0.0 {
            step_sizes.push(step);
        }
        prev_percent = progress.current_percent;

        if (progress.current_percent - progress.target_percent).abs() < 0.001 {
            break;
        }
    }

    // Steps should gradually decrease (exponential decay animation)
    if step_sizes.len() > 1 {
        // First step should be larger than last step (exponential decay)
        assert!(step_sizes[0] > step_sizes[step_sizes.len() - 1]);

        // All steps should be positive
        for step in &step_sizes {
            assert!(*step > 0.0);
        }
    }
}

#[test]
fn test_progress_animated_model_debug() {
    let model = ProgressAnimatedModel::new();
    let debug_str = format!("{:?}", model);

    assert!(debug_str.contains("ProgressAnimatedModel"));
}

#[test]
fn test_animated_progress_bar_debug() {
    let progress = AnimatedProgressBar::new();
    let debug_str = format!("{:?}", progress);

    assert!(debug_str.contains("AnimatedProgressBar"));
}

#[test]
fn test_progress_tick_msg_debug() {
    let msg = ProgressTickMsg;
    let debug_str = format!("{:?}", msg);

    assert!(debug_str.contains("ProgressTickMsg"));
}

#[test]
fn test_progress_frame_msg_debug() {
    let msg = ProgressFrameMsg;
    let debug_str = format!("{:?}", msg);

    assert!(debug_str.contains("ProgressFrameMsg"));
}

#[test]
fn test_edge_case_progress_values() {
    let mut progress = AnimatedProgressBar::new();

    // Test very small increment
    progress.set_percent(0.001);
    let view = progress.view();
    assert!(view.contains("0.0%")); // Should round to 0.0%

    // Test very close to 1.0
    progress.set_percent(0.999);
    assert!((progress.target_percent - 0.999).abs() < 0.001);
}

#[test]
fn test_multiple_increments() {
    let mut model = ProgressAnimatedModel::new();

    // Simulate the actual sequence: 0% -> 25% -> 50% -> 75% -> 100%
    let increments = [0.25, 0.25, 0.25, 0.25];

    for (i, _increment) in increments.iter().enumerate() {
        let tick_msg = Box::new(ProgressTickMsg) as Msg;
        let cmd = model.update(tick_msg);

        let expected_target = (i + 1) as f64 * 0.25;
        assert_eq!(model.progress.target_percent, expected_target);

        if expected_target < 1.0 {
            assert!(cmd.is_some()); // Should continue
        }
    }

    // Final state should be 100% target
    assert_eq!(model.progress.target_percent, 1.0);
}
