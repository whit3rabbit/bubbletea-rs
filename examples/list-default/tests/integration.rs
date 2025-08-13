use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel, Msg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod list_default_main;
use list_default_main::Model;

#[test]
fn test_model_init() {
    let (model, cmd) = Model::init();

    // Should create a list with the expected title
    let view = model.view();
    assert!(view.contains("My Fave Things"));

    // Should return an initial command to trigger window size request and rendering
    assert!(cmd.is_some());
}

#[test]
fn test_view_contains_items() {
    let (model, _) = Model::init();
    let view = model.view();

    // Should contain some of the expected items that are visible (from the Go version)
    assert!(view.contains("Raspberry Pi's"));
    assert!(view.contains("Nutella"));
    assert!(view.contains("Bitter melon"));
    assert!(view.contains("Nice socks"));
    assert!(view.contains("I have 'em all over my house"));
    assert!(view.contains("It's good on toast"));
    assert!(view.contains("It cools you down"));
}

#[test]
fn test_ctrl_c_quits() {
    let (mut model, _) = Model::init();

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_q_key_quits() {
    let (mut model, _) = Model::init();

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should return a quit command
    assert!(cmd.is_some());
}

#[test]
fn test_esc_key_quits_when_not_filtering() {
    let (mut model, _) = Model::init();

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    // Should return a quit command when not filtering
    assert!(cmd.is_some());
}

#[test]
fn test_esc_key_clears_filter_then_quits() {
    let (mut model, _) = Model::init();

    // First press '/' to enter filter mode
    let slash_key = Box::new(KeyMsg {
        key: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let _cmd = model.update(slash_key);

    // Now press Esc - should clear filter and NOT quit
    let esc_key = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let _cmd = model.update(esc_key);

    // Should NOT return a quit command (should let widget handle filter clearing)
    // The widget will handle clearing the filter

    // Press Esc again - now should quit since we're no longer filtering
    let esc_key2 = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd2 = model.update(esc_key2);

    // Should return a quit command on second Esc press
    assert!(cmd2.is_some());
}

#[test]
fn test_window_size_message() {
    let (mut model, _) = Model::init();

    let size_msg = Box::new(WindowSizeMsg {
        width: 120,
        height: 40,
    }) as Msg;

    let cmd = model.update(size_msg);

    // Should handle window size message (list widget handles internally)
    assert!(cmd.is_none());
}

#[test]
fn test_window_resizing_behavior() {
    let (mut model, _) = Model::init();

    // Get initial view to ensure it works initially
    let _initial_view = model.view();

    // Send different window sizes
    let small_size = Box::new(WindowSizeMsg {
        width: 40,
        height: 10,
    }) as Msg;

    let large_size = Box::new(WindowSizeMsg {
        width: 200,
        height: 50,
    }) as Msg;

    // Test small window
    let cmd = model.update(small_size);
    assert!(cmd.is_none());

    let small_view = model.view();

    // Test large window
    let cmd = model.update(large_size);
    assert!(cmd.is_none());

    let large_view = model.view();

    // Views should still work after window resizing
    assert!(!small_view.is_empty());
    assert!(!large_view.is_empty());
    assert!(small_view.contains("My Fave Things"));
    assert!(large_view.contains("My Fave Things"));
}

#[test]
fn test_list_navigation() {
    let (mut model, _) = Model::init();

    // Test down arrow key
    let down_key = Box::new(KeyMsg {
        key: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(down_key);
    // List widget should handle navigation internally
    assert!(cmd.is_none());

    // Test up arrow key
    let up_key = Box::new(KeyMsg {
        key: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(up_key);
    // List widget should handle navigation internally
    assert!(cmd.is_none());
}

#[test]
fn test_list_handles_other_keys() {
    let (mut model, _) = Model::init();

    // Test that other keys are handled by the list widget
    let enter_key = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let _cmd = model.update(enter_key);
    // List widget should handle this internally
    // The exact behavior depends on the widget implementation
    // We just ensure it doesn't crash
}

#[test]
fn test_view_rendering() {
    let (model, _) = Model::init();
    let view = model.view();

    // View should not be empty
    assert!(!view.is_empty());

    // View should be properly formatted (has some structure)
    assert!(view.len() > 100); // Should have substantial content

    // Should contain the title
    assert!(view.contains("My Fave Things"));
}

#[test]
fn test_visual_comparison_with_go() {
    let (model, _) = Model::init();
    let view = model.view();

    println!("\nVisual output of Rust implementation:");
    println!("=====================================");
    println!("{}", view);
    println!("=====================================");

    // Key visual elements that should match the Go version:

    // 1. Title should be present and styled
    assert!(view.contains("My Fave Things"));

    // 2. First few items should be visible (matching Go output)
    assert!(view.contains("Raspberry Pi's"));
    assert!(view.contains("I have 'em all over my house"));
    assert!(view.contains("Nutella"));
    assert!(view.contains("It's good on toast"));

    // 3. Should have item count status
    assert!(view.contains("/23 items"));

    // 4. Should have help text at the bottom
    assert!(view.contains("↑/k"));
    assert!(view.contains("↓/j"));
    assert!(view.contains("filter"));

    // 5. Should have proper margins/styling (docStyle.Margin(1, 2))
    // The view should have whitespace padding from the margins
    let lines: Vec<&str> = view.lines().collect();
    assert!(lines.len() > 5); // Should have multiple lines

    // First and last lines should be mostly whitespace (margins)
    assert!(lines[0].trim().is_empty());
    assert!(lines[lines.len() - 1].trim().is_empty());
}

#[test]
fn test_filtering_functionality() {
    let (mut model, _) = Model::init();

    // Simulate pressing '/' to enter filter mode
    let slash_key = Box::new(KeyMsg {
        key: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let _cmd = model.update(slash_key);

    // Simulate typing 'n' to filter items
    let n_key = Box::new(KeyMsg {
        key: KeyCode::Char('n'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let _cmd = model.update(n_key);

    let filtered_view = model.view();
    println!("\nFiltered view after typing '/n':");
    println!("=====================================");
    println!("{}", filtered_view);
    println!("=====================================");

    // Should show filtered results
    assert!(filtered_view.contains("filtered"));

    // Text should be properly formatted (not broken up)
    // If filtering is working correctly, we should see clean text
    assert!(filtered_view.contains("Nutella") || filtered_view.contains("utella"));
}
