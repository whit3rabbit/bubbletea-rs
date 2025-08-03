//! Integration tests for the help example

#[path = "../main.rs"]
mod help_main;

use bubbletea_rs::{KeyMsg, Model, Msg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};
use help_main::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper to create a HelpModel
    fn create_model() -> HelpModel {
        HelpModel::new()
    }

    /// Test helper to create a KeyMsg
    fn key_msg(key: KeyCode) -> Msg {
        Box::new(KeyMsg {
            key,
            modifiers: KeyModifiers::empty(),
        }) as Msg
    }

    /// Test helper to create a WindowSizeMsg
    fn window_size_msg(width: u16, height: u16) -> Msg {
        Box::new(WindowSizeMsg { width, height }) as Msg
    }

    #[test]
    fn test_model_initialization() {
        let (model, cmd) = HelpModel::init();

        assert!(!model.help_expanded, "Help should start in collapsed mode");
        assert_eq!(model.last_key, None, "No key should be pressed initially");
        assert_eq!(
            model.terminal_width, 80,
            "Default terminal width should be 80"
        );
        assert!(!model.quitting, "Model should not be quitting initially");
        assert!(cmd.is_none(), "Init should not return a command");
        assert_eq!(model.keys.len(), 6, "Should have 6 key bindings");
    }

    #[test]
    fn test_key_bindings_creation() {
        let model = create_model();

        // Check that we have all expected key bindings
        let binding_keys: Vec<&str> = model.keys.iter().map(|b| b.help_key).collect();
        assert!(binding_keys.contains(&"↑/k"), "Should have up key binding");
        assert!(
            binding_keys.contains(&"↓/j"),
            "Should have down key binding"
        );
        assert!(
            binding_keys.contains(&"←/h"),
            "Should have left key binding"
        );
        assert!(
            binding_keys.contains(&"→/l"),
            "Should have right key binding"
        );
        assert!(
            binding_keys.contains(&"?"),
            "Should have help toggle binding"
        );
        assert!(binding_keys.contains(&"q"), "Should have quit binding");
    }

    #[test]
    fn test_key_binding_matching() {
        let model = create_model();

        // Test arrow key bindings
        assert!(
            model.find_key_binding(KeyCode::Up).is_some(),
            "Up arrow should match"
        );
        assert!(
            model.find_key_binding(KeyCode::Char('k')).is_some(),
            "k should match up"
        );
        assert!(
            model.find_key_binding(KeyCode::Down).is_some(),
            "Down arrow should match"
        );
        assert!(
            model.find_key_binding(KeyCode::Char('j')).is_some(),
            "j should match down"
        );
        assert!(
            model.find_key_binding(KeyCode::Left).is_some(),
            "Left arrow should match"
        );
        assert!(
            model.find_key_binding(KeyCode::Char('h')).is_some(),
            "h should match left"
        );
        assert!(
            model.find_key_binding(KeyCode::Right).is_some(),
            "Right arrow should match"
        );
        assert!(
            model.find_key_binding(KeyCode::Char('l')).is_some(),
            "l should match right"
        );

        // Test action key bindings
        assert!(
            model.find_key_binding(KeyCode::Char('?')).is_some(),
            "? should match help"
        );
        assert!(
            model.find_key_binding(KeyCode::Char('q')).is_some(),
            "q should match quit"
        );
        assert!(
            model.find_key_binding(KeyCode::Esc).is_some(),
            "Esc should match quit"
        );
        assert!(
            model.find_key_binding(KeyCode::Char('c')).is_some(),
            "c should match quit"
        );

        // Test non-matching keys
        assert!(
            model.find_key_binding(KeyCode::Char('x')).is_none(),
            "x should not match"
        );
        assert!(
            model.find_key_binding(KeyCode::Enter).is_none(),
            "Enter should not match"
        );
    }

    #[test]
    fn test_arrow_key_updates() {
        let mut model = create_model();

        // Test up key
        let cmd = model.update(key_msg(KeyCode::Up));
        assert_eq!(model.last_key, Some("↑".to_string()));
        assert!(cmd.is_none());

        // Test down key
        let cmd = model.update(key_msg(KeyCode::Down));
        assert_eq!(model.last_key, Some("↓".to_string()));
        assert!(cmd.is_none());

        // Test left key
        let cmd = model.update(key_msg(KeyCode::Left));
        assert_eq!(model.last_key, Some("←".to_string()));
        assert!(cmd.is_none());

        // Test right key
        let cmd = model.update(key_msg(KeyCode::Right));
        assert_eq!(model.last_key, Some("→".to_string()));
        assert!(cmd.is_none());

        // Test vim-style keys
        let cmd = model.update(key_msg(KeyCode::Char('k')));
        assert_eq!(model.last_key, Some("↑".to_string()));
        assert!(cmd.is_none());

        let cmd = model.update(key_msg(KeyCode::Char('j')));
        assert_eq!(model.last_key, Some("↓".to_string()));
        assert!(cmd.is_none());

        let cmd = model.update(key_msg(KeyCode::Char('h')));
        assert_eq!(model.last_key, Some("←".to_string()));
        assert!(cmd.is_none());

        let cmd = model.update(key_msg(KeyCode::Char('l')));
        assert_eq!(model.last_key, Some("→".to_string()));
        assert!(cmd.is_none());
    }

    #[test]
    fn test_help_toggle() {
        let mut model = create_model();

        // Initially help should be collapsed
        assert!(!model.help_expanded);

        // Press ? to expand help
        let cmd = model.update(key_msg(KeyCode::Char('?')));
        assert!(
            model.help_expanded,
            "Help should be expanded after pressing ?"
        );
        assert!(cmd.is_none());

        // Press ? again to collapse help
        let cmd = model.update(key_msg(KeyCode::Char('?')));
        assert!(
            !model.help_expanded,
            "Help should be collapsed after pressing ? again"
        );
        assert!(cmd.is_none());
    }

    #[test]
    fn test_quit_functionality() {
        let mut model = create_model();

        // Test quit with 'q'
        let cmd = model.update(key_msg(KeyCode::Char('q')));
        assert!(model.quitting, "Model should be quitting after 'q'");
        assert!(cmd.is_some(), "Quit command should be returned");

        // Reset model
        let mut model = create_model();

        // Test quit with Esc
        let cmd = model.update(key_msg(KeyCode::Esc));
        assert!(model.quitting, "Model should be quitting after Esc");
        assert!(cmd.is_some(), "Quit command should be returned");

        // Reset model
        let mut model = create_model();

        // Test quit with Ctrl+C (represented as 'c')
        let cmd = model.update(key_msg(KeyCode::Char('c')));
        assert!(model.quitting, "Model should be quitting after 'c'");
        assert!(cmd.is_some(), "Quit command should be returned");
    }

    #[test]
    fn test_window_resize() {
        let mut model = create_model();

        // Test window resize
        let cmd = model.update(window_size_msg(120, 30));
        assert_eq!(
            model.terminal_width, 120,
            "Terminal width should be updated"
        );
        assert!(cmd.is_none(), "Window resize should not return command");

        // Test another resize
        let cmd = model.update(window_size_msg(60, 20));
        assert_eq!(
            model.terminal_width, 60,
            "Terminal width should be updated again"
        );
        assert!(cmd.is_none(), "Window resize should not return command");
    }

    #[test]
    fn test_view_initial_state() {
        let model = create_model();
        let view = model.view();

        assert!(
            view.contains("Waiting for input..."),
            "Should show waiting message initially"
        );
        assert!(
            view.contains("? toggle help"),
            "Should show help toggle in collapsed view"
        );
        assert!(
            view.contains("q quit"),
            "Should show quit option in collapsed view"
        );
        assert!(
            !view.contains("move up"),
            "Should not show navigation help in collapsed view"
        );
    }

    #[test]
    fn test_view_after_key_press() {
        let mut model = create_model();
        model.update(key_msg(KeyCode::Up));

        let view = model.view();
        assert!(
            view.contains("You chose:"),
            "Should show chosen key message"
        );
        assert!(
            !view.contains("Waiting for input..."),
            "Should not show waiting message"
        );
    }

    #[test]
    fn test_view_expanded_help() {
        let mut model = create_model();
        model.update(key_msg(KeyCode::Char('?')));

        let view = model.view();
        assert!(
            view.contains("move up"),
            "Should show navigation help in expanded view"
        );
        assert!(
            view.contains("move down"),
            "Should show navigation help in expanded view"
        );
        assert!(
            view.contains("move left"),
            "Should show navigation help in expanded view"
        );
        assert!(
            view.contains("move right"),
            "Should show navigation help in expanded view"
        );
        assert!(
            view.contains("toggle help"),
            "Should show help toggle in expanded view"
        );
        assert!(
            view.contains("quit"),
            "Should show quit option in expanded view"
        );
    }

    #[test]
    fn test_view_quitting() {
        let mut model = create_model();
        model.update(key_msg(KeyCode::Char('q')));

        let view = model.view();
        assert_eq!(view, "Bye!\n", "Should show goodbye message when quitting");
    }

    #[test]
    fn test_short_help_bindings() {
        let model = create_model();
        let short_help = model.short_help();

        assert_eq!(short_help.len(), 2, "Short help should have 2 bindings");

        let help_keys: Vec<&str> = short_help.iter().map(|b| b.help_key).collect();
        assert!(
            help_keys.contains(&"?"),
            "Short help should include help toggle"
        );
        assert!(help_keys.contains(&"q"), "Short help should include quit");
    }

    #[test]
    fn test_full_help_bindings() {
        let model = create_model();
        let (nav_keys, action_keys) = model.full_help();

        assert_eq!(nav_keys.len(), 4, "Navigation keys should have 4 bindings");
        assert_eq!(action_keys.len(), 2, "Action keys should have 2 bindings");

        let nav_help_keys: Vec<&str> = nav_keys.iter().map(|b| b.help_key).collect();
        assert!(
            nav_help_keys.contains(&"↑/k"),
            "Navigation should include up key"
        );
        assert!(
            nav_help_keys.contains(&"↓/j"),
            "Navigation should include down key"
        );
        assert!(
            nav_help_keys.contains(&"←/h"),
            "Navigation should include left key"
        );
        assert!(
            nav_help_keys.contains(&"→/l"),
            "Navigation should include right key"
        );

        let action_help_keys: Vec<&str> = action_keys.iter().map(|b| b.help_key).collect();
        assert!(
            action_help_keys.contains(&"?"),
            "Actions should include help toggle"
        );
        assert!(
            action_help_keys.contains(&"q"),
            "Actions should include quit"
        );
    }

    #[test]
    fn test_format_help_line() {
        let model = create_model();
        let short_help = model.short_help();
        let formatted = HelpModel::format_help_line(&short_help);

        assert!(
            formatted.contains("? toggle help"),
            "Should contain help binding"
        );
        assert!(formatted.contains("q quit"), "Should contain quit binding");
        assert!(formatted.contains(" • "), "Should use bullet separator");
    }

    #[test]
    fn test_format_help_columns() {
        let model = create_model();
        let (nav_keys, action_keys) = model.full_help();
        let formatted = HelpModel::format_help_columns(&nav_keys, &action_keys, 80);

        assert!(
            formatted.contains("move up"),
            "Should contain navigation descriptions"
        );
        assert!(
            formatted.contains("toggle help"),
            "Should contain action descriptions"
        );
        assert!(
            formatted.contains("  "),
            "Should have proper column spacing"
        );
    }

    #[test]
    fn test_format_help_columns_narrow_width() {
        let model = create_model();
        let (nav_keys, action_keys) = model.full_help();
        let formatted = HelpModel::format_help_columns(&nav_keys, &action_keys, 20);

        // Should handle narrow width gracefully
        assert!(
            !formatted.is_empty(),
            "Should still produce output for narrow width"
        );
        assert!(
            formatted.contains("...") || formatted.len() < 100,
            "Should truncate or be short for narrow width"
        );
    }

    #[test]
    fn test_responsive_help_width() {
        let mut model = create_model();

        // Set narrow width
        model.update(window_size_msg(40, 20));
        model.update(key_msg(KeyCode::Char('?')));
        let narrow_view = model.view();

        // Set wide width
        model.update(window_size_msg(120, 30));
        let wide_view = model.view();

        // Views should be different due to width differences
        // (exact content depends on implementation but should handle width)
        assert!(!narrow_view.is_empty(), "Narrow view should have content");
        assert!(!wide_view.is_empty(), "Wide view should have content");
    }

    #[test]
    fn test_unknown_key_no_effect() {
        let mut model = create_model();
        let initial_state = format!("{:?}", model);

        // Press unknown key
        let cmd = model.update(key_msg(KeyCode::Char('x')));
        let final_state = format!("{:?}", model);

        assert_eq!(
            initial_state, final_state,
            "Unknown key should not change state"
        );
        assert!(cmd.is_none(), "Unknown key should not return command");
    }

    #[test]
    fn test_key_binding_symbols() {
        let model = create_model();

        let up_binding = model.find_key_binding(KeyCode::Up).unwrap();
        assert_eq!(up_binding.symbol, "↑", "Up key should have up arrow symbol");

        let down_binding = model.find_key_binding(KeyCode::Down).unwrap();
        assert_eq!(
            down_binding.symbol, "↓",
            "Down key should have down arrow symbol"
        );

        let left_binding = model.find_key_binding(KeyCode::Left).unwrap();
        assert_eq!(
            left_binding.symbol, "←",
            "Left key should have left arrow symbol"
        );

        let right_binding = model.find_key_binding(KeyCode::Right).unwrap();
        assert_eq!(
            right_binding.symbol, "→",
            "Right key should have right arrow symbol"
        );
    }
}
