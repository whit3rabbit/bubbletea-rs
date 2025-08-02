use bubbletea_rs::{Model, Msg, KeyMsg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod progress_download_main;
use progress_download_main::{
    ProgressDownloadModel, DownloadSimulator, AnimatedProgressBar, 
    ProgressMsg, ProgressFrameMsg, ProgressErrMsg, FinalPauseMsg, DownloadTickMsg
};

#[test]
fn test_download_simulator_new() {
    let simulator = DownloadSimulator::new("test.zip".to_string(), 5);
    
    assert_eq!(simulator.file_name, "test.zip");
    assert_eq!(simulator.total_size, 5 * 1024 * 1024); // 5MB in bytes
    assert_eq!(simulator.progress(), 0.0);
    assert!(!simulator.is_complete);
    assert!(simulator.has_error.is_none());
}

#[test]
fn test_download_simulator_progress() {
    let mut simulator = DownloadSimulator::new("test.zip".to_string(), 1);
    
    // Simulate some progress
    simulator.downloaded = 512 * 1024; // Half of 1MB
    assert_eq!(simulator.progress(), 0.5);
    
    // Simulate completion
    simulator.downloaded = 1024 * 1024; // Full 1MB
    assert_eq!(simulator.progress(), 1.0);
}

#[test]
fn test_download_simulator_completion() {
    let mut simulator = DownloadSimulator::new("test.zip".to_string(), 1);
    
    assert!(!simulator.is_complete);
    
    simulator.is_complete = true;
    assert!(simulator.is_complete);
}

#[test]
fn test_download_simulator_error() {
    let mut simulator = DownloadSimulator::new("test.zip".to_string(), 1);
    
    assert!(simulator.has_error.is_none());
    
    simulator.has_error = Some("Network error".to_string());
    assert_eq!(simulator.has_error, Some("Network error".to_string()));
}

#[test]
fn test_download_simulator_tick() {
    let mut simulator = DownloadSimulator::new("test.zip".to_string(), 1);
    simulator.error_chance = 0; // No errors for this test
    
    let initial_progress = simulator.progress();
    let result = simulator.tick();
    
    assert!(result.is_ok());
    assert!(simulator.progress() > initial_progress);
    
    // Eventually should complete
    while !simulator.is_complete && simulator.has_error.is_none() {
        let _ = simulator.tick();
    }
    
    assert!(simulator.is_complete);
    assert_eq!(simulator.progress(), 1.0);
}

#[test]
fn test_animated_progress_bar_new() {
    let progress = AnimatedProgressBar::new();
    
    assert_eq!(progress.width, 40);
    assert_eq!(progress.current_percent, 0.0);
    assert_eq!(progress.target_percent, 0.0);
    assert_eq!(progress.filled_char, '█');
    assert_eq!(progress.empty_char, '░');
    assert_eq!(progress.animation_speed, 0.15);
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
    assert!(view.contains("50.0%"));
    
    // Test 0%
    progress.current_percent = 0.0;
    let view = progress.view();
    assert!(view.contains("0.0%"));
    
    // Test 100%
    progress.current_percent = 1.0;
    let view = progress.view();
    assert!(view.contains("100.0%"));
}

#[test]
fn test_progress_download_model_new() {
    let model = ProgressDownloadModel::new("test.zip".to_string(), 5);
    
    assert_eq!(model.downloader.file_name, "test.zip");
    assert_eq!(model.downloader.total_size, 5 * 1024 * 1024);
    assert_eq!(model.progress.current_percent, 0.0);
    assert!(model.error.is_none());
}

#[test]
fn test_progress_download_model_init() {
    let (model, cmd) = ProgressDownloadModel::init();
    
    // Should start with default values
    assert_eq!(model.progress.percent(), 0.0);
    assert!(model.error.is_none());
    
    // Should start download simulation with tick command
    assert!(cmd.is_some());
}

#[test]
fn test_update_window_size() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
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
fn test_download_tick_message() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    model.downloader.error_chance = 0; // No errors for this test 
    
    let initial_progress = model.progress.target_percent;
    
    let tick_msg = Box::new(DownloadTickMsg) as Msg;
    let cmd = model.update(tick_msg);
    
    // Progress should advance 
    assert!(model.progress.target_percent >= initial_progress);
    
    // Should return batched commands (animation + next tick or completion)
    assert!(cmd.is_some());
}

#[test]
fn test_progress_message() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    let initial_target = model.progress.target_percent;
    
    let progress_msg = Box::new(ProgressMsg(0.5)) as Msg;
    let cmd = model.update(progress_msg);
    
    // Target progress should be updated
    assert_eq!(model.progress.target_percent, 0.5);
    assert_ne!(model.progress.target_percent, initial_target);
    
    // Should return animation command
    assert!(cmd.is_some());
}

#[test]
fn test_progress_message_completion() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
    let progress_msg = Box::new(ProgressMsg(1.0)) as Msg;
    let cmd = model.update(progress_msg);
    
    // Should update progress to 100%
    assert_eq!(model.progress.target_percent, 1.0);
    
    // Should return batched commands (pause + quit + animation)
    assert!(cmd.is_some());
}

#[test]
fn test_progress_frame_message() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
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
fn test_progress_error_message() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
    let error_msg = Box::new(ProgressErrMsg {
        error: "Network timeout".to_string(),
    }) as Msg;
    
    let cmd = model.update(error_msg);
    
    // Should set error
    assert_eq!(model.error, Some("Network timeout".to_string()));
    
    // Should quit
    assert!(cmd.is_some());
}

#[test]
fn test_final_pause_message() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
    let pause_msg = Box::new(FinalPauseMsg) as Msg;
    let cmd = model.update(pause_msg);
    
    // Should just pause, not quit directly
    assert!(cmd.is_none());
}

#[test]
fn test_window_size_message() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
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
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
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
        let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
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
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_view_initial_state() {
    let model = ProgressDownloadModel::new("example.zip".to_string(), 1);
    let view = model.view();
    
    assert!(view.contains("Downloading example.zip..."));
    assert!(view.contains("0.0%"));
    assert!(view.contains("Press any key to quit"));
    
    // Should have proper padding
    assert!(view.starts_with('\n'));
    let lines: Vec<&str> = view.lines().collect();
    assert!(lines.len() >= 4);
    assert!(lines[1].starts_with("  ")); // Download line has padding
    assert!(lines[2].starts_with("  ")); // Progress line has padding
    assert!(lines[4].starts_with("  ")); // Help line has padding
}

#[test]
fn test_view_in_progress() {
    let mut model = ProgressDownloadModel::new("example.zip".to_string(), 1);
    model.progress.current_percent = 0.75;
    
    let view = model.view();
    
    assert!(view.contains("Downloading example.zip..."));
    assert!(view.contains("75.0%"));
    assert!(view.contains("Press any key to quit"));
}

#[test]
fn test_view_with_error() {
    let mut model = ProgressDownloadModel::new("example.zip".to_string(), 1);
    model.error = Some("Connection failed".to_string());
    
    let view = model.view();
    
    assert!(view.contains("Error downloading: Connection failed"));
    assert!(!view.contains("Downloading example.zip..."));
    assert!(!view.contains("Press any key to quit"));
}

#[test]
fn test_view_completed() {
    let mut model = ProgressDownloadModel::new("example.zip".to_string(), 1);
    model.progress.current_percent = 1.0;
    
    let view = model.view();
    
    assert!(view.contains("100.0%"));
    assert!(view.contains("Downloading example.zip..."));
}

#[test]
fn test_progress_msg_clone() {
    let msg1 = ProgressMsg(0.5);
    let msg2 = msg1.clone();
    
    assert_eq!(msg1.0, msg2.0);
}

#[test]
fn test_multiple_progress_updates() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
    // Simulate progress updates: 0% -> 25% -> 50% -> 75% -> 100%
    let progress_values = [0.25, 0.5, 0.75, 1.0];
    
    for &progress in progress_values.iter() {
        let progress_msg = Box::new(ProgressMsg(progress)) as Msg;
        let cmd = model.update(progress_msg);
        
        assert_eq!(model.progress.target_percent, progress);
        
        if progress < 1.0 {
            assert!(cmd.is_some()); // Should return animation command
        } else {
            assert!(cmd.is_some()); // Should return completion commands
        }
    }
    
    // Final state should be 100%
    assert_eq!(model.progress.target_percent, 1.0);
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
    while (progress.current_percent - progress.target_percent).abs() > 0.001 && animation_steps < 100 {
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
fn test_download_simulator_zero_size() {
    let simulator = DownloadSimulator::new("empty.txt".to_string(), 0);
    
    assert_eq!(simulator.total_size, 0);
    assert_eq!(simulator.progress(), 0.0); // Should handle zero division
}

#[test]
fn test_progress_bar_clamp() {
    let mut progress = AnimatedProgressBar::new();
    
    // Test negative value gets clamped
    progress.set_percent(-0.5);
    assert_eq!(progress.target_percent, 0.0);
    
    // Test value > 1.0 gets clamped
    progress.set_percent(1.5);
    assert_eq!(progress.target_percent, 1.0);
}

#[test]
fn test_debug_implementations() {
    let simulator = DownloadSimulator::new("test.zip".to_string(), 1);
    let progress = AnimatedProgressBar::new();
    let model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    let progress_msg = ProgressMsg(0.5);
    let frame_msg = ProgressFrameMsg;
    let error_msg = ProgressErrMsg { error: "test".to_string() };
    let pause_msg = FinalPauseMsg;
    
    // All should implement Debug
    let _ = format!("{:?}", simulator);
    let _ = format!("{:?}", progress);
    let _ = format!("{:?}", model);
    let _ = format!("{:?}", progress_msg);
    let _ = format!("{:?}", frame_msg);
    let _ = format!("{:?}", error_msg);
    let _ = format!("{:?}", pause_msg);
}

#[test]
fn test_edge_case_small_progress_increments() {
    let mut model = ProgressDownloadModel::new("test.zip".to_string(), 1);
    
    // Test very small increment
    let progress_msg = Box::new(ProgressMsg(0.001)) as Msg;
    let _cmd = model.update(progress_msg);
    
    assert_eq!(model.progress.target_percent, 0.001);
    // Small increment might not trigger animation if current is 0.0
    // That's fine - it's an edge case
}

#[test]
fn test_large_file_size() {
    let mut simulator = DownloadSimulator::new("large-file.zip".to_string(), 1000); // 1GB
    
    assert_eq!(simulator.total_size, 1000 * 1024 * 1024);
    
    // Simulate 1% progress
    simulator.downloaded = 10 * 1024 * 1024; // 10MB of 1GB
    
    assert!((simulator.progress() - 0.01).abs() < 0.001);
}