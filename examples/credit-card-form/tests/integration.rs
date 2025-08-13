use bubbletea_rs::{KeyMsg, Model};
use crossterm::event::{KeyCode, KeyModifiers};

// Re-export the main module for testing
#[path = "../main.rs"]
mod main;
use main::CreditCardForm;

#[tokio::test]
async fn test_credit_card_form_navigation() {
    let (mut model, _cmd) = CreditCardForm::init();

    // Test tab navigation
    let tab_key = KeyMsg {
        key: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
    };
    let _cmd = model.update(Box::new(tab_key));

    // Test shift+tab navigation
    let shift_tab_key = KeyMsg {
        key: KeyCode::BackTab,
        modifiers: KeyModifiers::SHIFT,
    };
    let _cmd = model.update(Box::new(shift_tab_key));

    // Test escape key
    let esc_key = KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    };
    let cmd = model.update(Box::new(esc_key));

    // Should return quit command
    assert!(cmd.is_some());
}

#[tokio::test]
async fn test_view_rendering() {
    let (model, _cmd) = CreditCardForm::init();
    let view = model.view();

    // Check that the view contains expected elements
    assert!(view.contains("Total: $21.50"));
    assert!(view.contains("Card Number"));
    assert!(view.contains("EXP"));
    assert!(view.contains("CVV"));
    assert!(view.contains("Continue ->"));
}

#[test]
fn test_ccn_formatting() {
    use main::format_ccn;

    // Test empty input
    assert_eq!(format_ccn(""), "");

    // Test single digit
    assert_eq!(format_ccn("4"), "4");

    // Test 4 digits (no space yet)
    assert_eq!(format_ccn("4505"), "4505");

    // Test 5 digits (first space)
    assert_eq!(format_ccn("45051"), "4505 1");

    // Test 8 digits (one space)
    assert_eq!(format_ccn("45051234"), "4505 1234");

    // Test 16 digits (full card number)
    assert_eq!(format_ccn("4505123456789012"), "4505 1234 5678 9012");

    // Test input with existing spaces (should reformat)
    assert_eq!(format_ccn("4505 1234 5678 9012"), "4505 1234 5678 9012");

    // Test input with letters (should filter out)
    assert_eq!(format_ccn("4505abc1234"), "4505 1234");

    // Test too many digits (should truncate)
    assert_eq!(format_ccn("45051234567890123456"), "4505 1234 5678 9012");
}

#[test]
fn test_exp_formatting() {
    use main::format_exp;

    // Test empty input
    assert_eq!(format_exp(""), "");

    // Test single digit
    assert_eq!(format_exp("1"), "1");

    // Test 2 digits (no slash yet)
    assert_eq!(format_exp("12"), "12");

    // Test 3 digits (slash appears)
    assert_eq!(format_exp("123"), "12/3");

    // Test 4 digits (complete MM/YY)
    assert_eq!(format_exp("1225"), "12/25");

    // Test input with letters (should filter out)
    assert_eq!(format_exp("12ab25"), "12/25");

    // Test too many digits (should truncate)
    assert_eq!(format_exp("122567"), "12/25");

    // Test input with existing slash (should reformat)
    assert_eq!(format_exp("12/25"), "12/25");
}
