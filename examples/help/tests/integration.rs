//! Integration tests for the help example

#[path = "../main.rs"]
mod help_main;

use bubbletea_rs::{KeyMsg, Model as BubbleTeaModel, Msg, WindowSizeMsg};
use bubbletea_widgets::help::KeyMap as HelpKeyMap;
use crossterm::event::{KeyCode, KeyModifiers};
use help_main::Model;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper to create a Model
    fn create_model() -> Model {
        Model::new()
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
        let (model, cmd) = Model::init();

        assert!(!model.help.show_all, "Help should start in collapsed mode");
        assert!(
            model.last_key.is_empty(),
            "No key should be pressed initially"
        );
        assert!(!model.quitting, "Model should not be quitting initially");
        assert!(cmd.is_some(), "Init should return an InitRenderMsg command");
    }

    #[test]
    fn test_arrow_key_updates() {
        let mut model = create_model();

        // Test up key
        let cmd = model.update(key_msg(KeyCode::Up));
        assert_eq!(model.last_key, "↑");
        assert!(cmd.is_none());

        // Test down key
        let cmd = model.update(key_msg(KeyCode::Down));
        assert_eq!(model.last_key, "↓");
        assert!(cmd.is_none());

        // Test left key
        let cmd = model.update(key_msg(KeyCode::Left));
        assert_eq!(model.last_key, "←");
        assert!(cmd.is_none());

        // Test right key
        let cmd = model.update(key_msg(KeyCode::Right));
        assert_eq!(model.last_key, "→");
        assert!(cmd.is_none());

        // Test vim-style keys
        let cmd = model.update(key_msg(KeyCode::Char('k')));
        assert_eq!(model.last_key, "↑");
        assert!(cmd.is_none());

        let cmd = model.update(key_msg(KeyCode::Char('j')));
        assert_eq!(model.last_key, "↓");
        assert!(cmd.is_none());

        let cmd = model.update(key_msg(KeyCode::Char('h')));
        assert_eq!(model.last_key, "←");
        assert!(cmd.is_none());

        let cmd = model.update(key_msg(KeyCode::Char('l')));
        assert_eq!(model.last_key, "→");
        assert!(cmd.is_none());
    }

    #[test]
    fn test_help_toggle() {
        let mut model = create_model();

        // Initially help should be collapsed
        assert!(!model.help.show_all);

        // Press ? to expand help
        let cmd = model.update(key_msg(KeyCode::Char('?')));
        assert!(
            model.help.show_all,
            "Help should be expanded after pressing ?"
        );
        assert!(cmd.is_none());

        // Press ? again to collapse help
        let cmd = model.update(key_msg(KeyCode::Char('?')));
        assert!(
            !model.help.show_all,
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
    }

    #[test]
    fn test_window_resize() {
        let mut model = create_model();

        // Test window resize
        let cmd = model.update(window_size_msg(120, 30));
        assert_eq!(model.help.width, 120, "Help widget width should be updated");
        assert!(cmd.is_none(), "Window resize should not return command");

        // Test another resize
        let cmd = model.update(window_size_msg(60, 20));
        assert_eq!(
            model.help.width, 60,
            "Help widget width should be updated again"
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
            view.contains("?") && view.contains("toggle help"),
            "Should show help toggle in collapsed view"
        );
        assert!(
            view.contains("q") && view.contains("quit"),
            "Should show quit option in collapsed view"
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
    fn test_unknown_key_no_effect() {
        let mut model = create_model();
        let initial_last_key = model.last_key.clone();
        let initial_help_state = model.help.show_all;

        // Press unknown key
        let cmd = model.update(key_msg(KeyCode::Char('x')));

        assert_eq!(
            model.last_key, initial_last_key,
            "Unknown key should not change last_key"
        );
        assert_eq!(
            model.help.show_all, initial_help_state,
            "Unknown key should not change help state"
        );
        assert!(cmd.is_none(), "Unknown key should not return command");
    }

    #[test]
    fn test_keymap_interface() {
        let model = create_model();

        // Test short help
        let short_help = model.short_help();
        assert_eq!(short_help.len(), 2, "Short help should have 2 bindings");

        // Test full help
        let full_help = model.full_help();
        assert_eq!(full_help.len(), 2, "Full help should have 2 columns");
        assert_eq!(full_help[0].len(), 4, "First column should have 4 bindings");
        assert_eq!(
            full_help[1].len(),
            2,
            "Second column should have 2 bindings"
        );
    }
}
