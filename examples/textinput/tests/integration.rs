use bubbletea_rs::{KeyMsg, Model, Msg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod textinput_main;
use textinput_main::TextInputModel;

#[test]
fn test_textinput_model_init_returns_cmd() {
    let (_model, cmd) = TextInputModel::init();
    assert!(cmd.is_some());
}

#[test]
fn test_initial_view_contains_placeholder_and_prompt() {
    let (model, _) = TextInputModel::init();
    let view = model.view();
    assert!(view.contains("What’s your favorite Pokémon?"));
    assert!(view.contains("Pikachu"));
    assert!(view.contains("(esc to quit)"));
}

#[test]
fn test_typing_updates_view() {
    let (mut model, _) = TextInputModel::init();
    // Type 'P'
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('P'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    let _ = model.update(key_msg);

    let view = model.view();
    assert!(view.contains("P"));
}

#[test]
fn test_enter_key_quits() {
    let (mut model, _) = TextInputModel::init();
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    let cmd = model.update(key_msg);
    assert!(cmd.is_some());
}

#[test]
fn test_esc_key_quits() {
    let (mut model, _) = TextInputModel::init();
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    let cmd = model.update(key_msg);
    assert!(cmd.is_some());
}

#[test]
fn test_ctrl_c_quits() {
    let (mut model, _) = TextInputModel::init();
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    let cmd = model.update(key_msg);
    assert!(cmd.is_some());
}
