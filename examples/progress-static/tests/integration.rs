use bubbletea_rs::{Model, Msg, KeyMsg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod progress_static_main;
use progress_static_main::{ProgressStaticModel, ProgressConfig, ProgressStyle, ProgressTickMsg};

#[test]
fn test_progress_static_model_new() {
    let model = ProgressStaticModel::new();
    
    assert_eq!(model.percent, 0.0);
    assert_eq!(model.config.width, 40);
    assert!(model.config.show_percentage);
    assert_eq!(model.config.style, ProgressStyle::Basic);
    assert!(!model.completed);
    assert_eq!(model.window_width, 80);
    assert_eq!(model.window_height, 24);
}

#[test]
fn test_progress_static_model_init() {
    let (model, cmd) = ProgressStaticModel::init();
    
    // Should start with default values
    assert_eq!(model.percent, 0.0);
    assert!(!model.completed);
    
    // Should start progress updates
    assert!(cmd.is_some());
}

#[test]
fn test_progress_config_default() {
    let config = ProgressConfig::default();
    
    assert_eq!(config.width, 40);
    assert!(config.show_percentage);
    assert_eq!(config.empty_char, '░');
    assert_eq!(config.filled_char, '█');
    assert_eq!(config.style, ProgressStyle::Basic);
}

#[test]
fn test_progress_config_render_basic() {
    let config = ProgressConfig::default();
    
    // Test 0% progress
    let result = config.render_progress(0.0);
    assert!(result.contains("░")); // Should contain empty characters
    assert!(result.contains("0.0%"));
    // Check that it's all empty (40 empty chars for 0%)
    let empty_count = result.chars().filter(|&c| c == '░').count();
    assert_eq!(empty_count, 40);
    
    // Test 50% progress
    let result = config.render_progress(0.5);
    assert!(result.contains("█")); // Some filled
    assert!(result.contains("░")); // Some empty
    assert!(result.contains("50.0%"));
    
    // Test 100% progress
    let result = config.render_progress(1.0);
    assert!(result.contains("█")); // Should contain filled characters
    assert!(result.contains("100.0%"));
    // Check that it's all filled (40 filled chars for 100%)
    let filled_count = result.chars().filter(|&c| c == '█').count();
    assert_eq!(filled_count, 40);
}

#[test]
fn test_progress_config_render_without_percentage() {
    let mut config = ProgressConfig::default();
    config.show_percentage = false;
    
    let result = config.render_progress(0.5);
    assert!(!result.contains("%"));
    assert!(result.contains("█")); // Should still show progress bar
    assert!(result.contains("░"));
}

#[test]
fn test_progress_config_render_gradient() {
    let mut config = ProgressConfig::default();
    config.style = ProgressStyle::Gradient;
    
    let result = config.render_progress(0.5);
    // Should contain gradient characters
    assert!(result.contains("▌") || result.contains("▊") || result.contains("▉") || result.contains("█"));
    assert!(result.contains("50.0%"));
}

#[test]
fn test_progress_config_render_blocks() {
    let mut config = ProgressConfig::default();
    config.style = ProgressStyle::Blocks;
    
    let result = config.render_progress(0.3);
    assert!(result.contains("30.0%"));
    // Blocks style should contain block characters
    assert!(result.len() > 0);
}

#[test]
fn test_progress_config_render_dots() {
    let mut config = ProgressConfig::default();
    config.style = ProgressStyle::Dots;
    
    let result = config.render_progress(0.7);
    assert!(result.contains("70.0%"));
    // Should contain braille dot characters
    assert!(result.chars().any(|c| c >= '⠀' && c <= '⣿'));
}

#[test]
fn test_progress_clamp() {
    let config = ProgressConfig::default();
    
    // Test negative value gets clamped to 0
    let result = config.render_progress(-0.5);
    assert!(result.contains("0.0%"));
    
    // Test value > 1.0 gets clamped to 100%
    let result = config.render_progress(1.5);
    assert!(result.contains("100.0%"));
}

#[test]
fn test_increment_progress() {
    let mut model = ProgressStaticModel::new();
    
    assert_eq!(model.percent, 0.0);
    assert!(!model.completed);
    
    // First increment: 0% -> 25%
    model.increment_progress();
    assert_eq!(model.percent, 0.25);
    assert!(!model.completed);
    
    // Second increment: 25% -> 50%
    model.increment_progress();
    assert_eq!(model.percent, 0.5);
    assert!(!model.completed);
    
    // Third increment: 50% -> 75%
    model.increment_progress();
    assert_eq!(model.percent, 0.75);
    assert!(!model.completed);
    
    // Fourth increment: 75% -> 100%
    model.increment_progress();
    assert_eq!(model.percent, 1.0);
    assert!(model.completed);
    
    // Fifth increment: should stay at 100%
    model.increment_progress();
    assert_eq!(model.percent, 1.0);
    assert!(model.completed);
}

#[test]
fn test_update_window_size() {
    let mut model = ProgressStaticModel::new();
    
    model.update_window_size(100, 30);
    
    assert_eq!(model.window_width, 100);
    assert_eq!(model.window_height, 30);
    // Progress bar width should be updated (with padding and max width)
    assert_eq!(model.config.width, 80); // Max width cap
    
    // Test smaller window
    model.update_window_size(50, 20);
    assert_eq!(model.window_width, 50);
    assert_eq!(model.config.width, 42); // 50 - 8 (padding)
}

#[test]
fn test_change_style() {
    let mut model = ProgressStaticModel::new();
    
    // Should start with Basic
    assert_eq!(model.config.style, ProgressStyle::Basic);
    
    // Cycle through styles
    model.change_style();
    assert_eq!(model.config.style, ProgressStyle::Gradient);
    
    model.change_style();
    assert_eq!(model.config.style, ProgressStyle::Blocks);
    
    model.change_style();
    assert_eq!(model.config.style, ProgressStyle::Dots);
    
    model.change_style();
    assert_eq!(model.config.style, ProgressStyle::Basic); // Should cycle back
}

#[test]
fn test_toggle_percentage() {
    let mut model = ProgressStaticModel::new();
    
    // Should start with percentage shown
    assert!(model.config.show_percentage);
    
    model.toggle_percentage();
    assert!(!model.config.show_percentage);
    
    model.toggle_percentage();
    assert!(model.config.show_percentage);
}

#[test]
fn test_with_config() {
    let custom_config = ProgressConfig {
        width: 60,
        show_percentage: false,
        empty_char: '-',
        filled_char: '#',
        style: ProgressStyle::Gradient,
    };
    
    let model = ProgressStaticModel::new().with_config(custom_config.clone());
    
    assert_eq!(model.config.width, 60);
    assert!(!model.config.show_percentage);
    assert_eq!(model.config.empty_char, '-');
    assert_eq!(model.config.filled_char, '#');
    assert_eq!(model.config.style, ProgressStyle::Gradient);
}

#[test]
fn test_progress_tick_message() {
    let mut model = ProgressStaticModel::new();
    let initial_percent = model.percent;
    
    let tick_msg = Box::new(ProgressTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Progress should advance
    assert_eq!(model.percent, initial_percent + 0.25);
    
    // Should return a new tick command if not completed
    if !model.completed {
        assert!(cmd.is_some());
    }
}

#[test]
fn test_progress_tick_completion() {
    let mut model = ProgressStaticModel::new();
    model.percent = 0.75; // Start at 75%
    
    let tick_msg = Box::new(ProgressTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Should complete and quit
    assert_eq!(model.percent, 1.0);
    assert!(model.completed);
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_window_size_message() {
    let mut model = ProgressStaticModel::new();
    
    let size_msg = Box::new(WindowSizeMsg {
        width: 120,
        height: 40,
    }) as Msg;
    
    let cmd = model.update(size_msg);
    
    assert_eq!(model.window_width, 120);
    assert_eq!(model.window_height, 40);
    assert_eq!(model.config.width, 80); // Max width
    assert!(cmd.is_none());
}

#[test]
fn test_q_key_quits() {
    let mut model = ProgressStaticModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_esc_key_quits() {
    let mut model = ProgressStaticModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_ctrl_c_quits() {
    let mut model = ProgressStaticModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_space_changes_style() {
    let mut model = ProgressStaticModel::new();
    let initial_style = model.config.style.clone();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char(' '),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Style should change
    assert_ne!(model.config.style, initial_style);
    assert_eq!(model.config.style, ProgressStyle::Gradient);
    assert!(cmd.is_none());
}

#[test]
fn test_p_toggles_percentage() {
    let mut model = ProgressStaticModel::new();
    let initial_show = model.config.show_percentage;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('p'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Percentage display should toggle
    assert_ne!(model.config.show_percentage, initial_show);
    assert!(cmd.is_none());
}

#[test]
fn test_r_resets_progress() {
    let mut model = ProgressStaticModel::new();
    model.percent = 0.75;
    model.completed = true;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('r'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    // Progress should reset
    assert_eq!(model.percent, 0.0);
    assert!(!model.completed);
    assert!(cmd.is_some()); // Should restart tick
}

#[test]
fn test_any_other_key_quits() {
    let mut model = ProgressStaticModel::new();
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_view_initial_state() {
    let model = ProgressStaticModel::new();
    let view = model.view();
    
    assert!(view.contains("Progress:"));
    assert!(view.contains("0.0%"));
    assert!(view.contains("⏳ Processing... (0% complete)"));
    assert!(view.contains("Style: Basic"));
    assert!(view.contains("Width: 40 characters"));
    assert!(view.contains("Percentage: shown"));
    assert!(view.contains("r: reset"));
}

#[test]
fn test_view_in_progress() {
    let mut model = ProgressStaticModel::new();
    model.percent = 0.5;
    
    let view = model.view();
    
    assert!(view.contains("50.0%"));
    assert!(view.contains("⏳ Processing... (50% complete)"));
    assert!(view.contains("█")); // Should show filled progress
    assert!(view.contains("░")); // Should show empty progress
}

#[test]
fn test_view_completed() {
    let mut model = ProgressStaticModel::new();
    model.percent = 1.0;
    model.completed = true;
    
    let view = model.view();
    
    assert!(view.contains("100.0%"));
    assert!(view.contains("✅ Task completed!"));
    assert!(view.contains("Press any key to exit"));
    assert!(!view.contains("⏳ Processing"));
}

#[test]
fn test_view_different_styles() {
    let styles = [
        (ProgressStyle::Basic, "Basic"),
        (ProgressStyle::Gradient, "Gradient"),
        (ProgressStyle::Blocks, "Blocks"),
        (ProgressStyle::Dots, "Dots"),
    ];
    
    for (style, style_name) in styles {
        let mut model = ProgressStaticModel::new();
        model.config.style = style;
        model.percent = 0.5;
        
        let view = model.view();
        assert!(view.contains(&format!("Style: {}", style_name)));
    }
}

#[test]
fn test_view_without_percentage() {
    let mut model = ProgressStaticModel::new();
    model.config.show_percentage = false;
    model.percent = 0.3;
    
    let view = model.view();
    
    assert!(view.contains("Percentage: hidden"));
    // The progress bar itself shouldn't show percentage
    let progress_line = view.lines().find(|line| line.contains("Progress:")).unwrap();
    assert!(!progress_line.contains("%"));
}

#[test]
fn test_progress_style_debug() {
    let style = ProgressStyle::Basic;
    let debug_str = format!("{:?}", style);
    
    assert!(debug_str.contains("Basic"));
}

#[test]
fn test_progress_config_debug() {
    let config = ProgressConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("ProgressConfig"));
}

#[test]
fn test_progress_static_model_debug() {
    let model = ProgressStaticModel::new();
    let debug_str = format!("{:?}", model);
    
    assert!(debug_str.contains("ProgressStaticModel"));
}

#[test]
fn test_progress_tick_msg_debug() {
    let msg = ProgressTickMsg;
    let debug_str = format!("{:?}", msg);
    
    assert!(debug_str.contains("ProgressTickMsg"));
}

#[test]
fn test_progress_config_clone() {
    let config = ProgressConfig::default();
    let cloned = config.clone();
    
    assert_eq!(config.width, cloned.width);
    assert_eq!(config.show_percentage, cloned.show_percentage);
    assert_eq!(config.style, cloned.style);
}

#[test]
fn test_different_progress_widths() {
    let widths = [10, 20, 40, 80];
    
    for width in widths {
        let mut config = ProgressConfig::default();
        config.width = width;
        
        let result = config.render_progress(0.5);
        // Should contain appropriate number of characters (plus percentage if shown)
        assert!(result.len() >= width);
    }
}

#[test]
fn test_edge_case_progress_values() {
    let config = ProgressConfig::default();
    
    // Test very small positive value
    let result = config.render_progress(0.001);
    assert!(result.contains("0.1%"));
    
    // Test very close to 1.0
    let result = config.render_progress(0.999);
    assert!(result.contains("99.9%"));
}

#[test]
fn test_window_resize_edge_cases() {
    let mut model = ProgressStaticModel::new();
    
    // Test very small window
    model.update_window_size(10, 5);
    assert_eq!(model.window_width, 10);
    assert_eq!(model.config.width, 2); // 10 - 8 (padding)
    
    // Test very large window
    model.update_window_size(200, 50);
    assert_eq!(model.window_width, 200);
    assert_eq!(model.config.width, 80); // Should cap at max width
}