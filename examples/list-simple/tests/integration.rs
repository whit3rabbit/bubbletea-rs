use bubbletea_rs::{KeyMsg, Model, Msg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod list_simple_main;
use list_simple_main::{ListItem, ListSimpleModel};

#[test]
fn test_list_simple_model_init() {
    let (model, cmd) = ListSimpleModel::init();

    // Should start with 10 dinner items
    assert_eq!(model.items.len(), 10);
    assert_eq!(model.cursor, 0);
    assert!(model.selected.is_none());
    assert!(model.choice.is_none());
    assert!(!model.quitting);
    assert_eq!(model.height, 14);

    // Should not return any initial command
    assert!(cmd.is_none());
}

#[test]
fn test_list_item_creation() {
    let item = ListItem::new("Test Item");
    assert_eq!(item.title, "Test Item");
}

#[test]
fn test_list_simple_model_new() {
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];

    let model = ListSimpleModel::new(items.clone());

    assert_eq!(model.items, items);
    assert_eq!(model.cursor, 0);
    assert!(model.selected.is_none());
    assert!(model.choice.is_none());
    assert!(!model.quitting);
    assert_eq!(model.height, 14);
}

#[test]
fn test_move_cursor_down() {
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];
    let mut model = ListSimpleModel::new(items);

    assert_eq!(model.cursor, 0);

    model.move_cursor_down();
    assert_eq!(model.cursor, 1);

    model.move_cursor_down();
    assert_eq!(model.cursor, 2);
}

#[test]
fn test_move_cursor_down_at_end() {
    let items = vec![ListItem::new("Item 1"), ListItem::new("Item 2")];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 1; // At last item

    model.move_cursor_down();

    // Should not move beyond last item
    assert_eq!(model.cursor, 1);
}

#[test]
fn test_move_cursor_up() {
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 2;

    model.move_cursor_up();
    assert_eq!(model.cursor, 1);

    model.move_cursor_up();
    assert_eq!(model.cursor, 0);
}

#[test]
fn test_move_cursor_up_at_beginning() {
    let items = vec![ListItem::new("Item 1"), ListItem::new("Item 2")];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 0; // At first item

    model.move_cursor_up();

    // Should not move beyond first item
    assert_eq!(model.cursor, 0);
}

#[test]
fn test_select_current() {
    let items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 1;

    model.select_current();

    assert_eq!(model.selected, Some(1));
    assert_eq!(model.choice, Some("Item 2".to_string()));
}

#[test]
fn test_select_current_first_item() {
    let items = vec![ListItem::new("First"), ListItem::new("Second")];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 0;

    model.select_current();

    assert_eq!(model.selected, Some(0));
    assert_eq!(model.choice, Some("First".to_string()));
}

#[test]
fn test_select_current_last_item() {
    let items = vec![ListItem::new("First"), ListItem::new("Last")];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 1;

    model.select_current();

    assert_eq!(model.selected, Some(1));
    assert_eq!(model.choice, Some("Last".to_string()));
}

#[test]
fn test_render_item_normal() {
    let items = vec![ListItem::new("Test Item")];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 1; // Set cursor to different item so item 0 is not selected
    let item = &model.items[0];

    let rendered = model.render_item(0, item);

    assert_eq!(rendered, "    1. Test Item");
}

#[test]
fn test_render_item_selected() {
    let items = vec![ListItem::new("Selected Item")];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 0;
    let item = &model.items[0];

    let rendered = model.render_item(0, item);

    assert_eq!(rendered, "  > 1. Selected Item");
}

#[test]
fn test_render_item_numbering() {
    let items = vec![
        ListItem::new("First"),
        ListItem::new("Second"),
        ListItem::new("Third"),
    ];
    let mut model = ListSimpleModel::new(items);
    model.cursor = 999; // Set cursor beyond items so none are selected

    assert_eq!(model.render_item(0, &model.items[0]), "    1. First");
    assert_eq!(model.render_item(1, &model.items[1]), "    2. Second");
    assert_eq!(model.render_item(2, &model.items[2]), "    3. Third");
}

#[test]
fn test_down_arrow_key() {
    let mut model = ListSimpleModel::new(vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ]);

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert_eq!(model.cursor, 1);
    assert!(cmd.is_none());
}

#[test]
fn test_up_arrow_key() {
    let mut model = ListSimpleModel::new(vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ]);
    model.cursor = 2;

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert_eq!(model.cursor, 1);
    assert!(cmd.is_none());
}

#[test]
fn test_enter_key_selects() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1"), ListItem::new("Item 2")]);
    model.cursor = 1;

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert_eq!(model.selected, Some(1));
    assert_eq!(model.choice, Some("Item 2".to_string()));
    assert!(cmd.is_none());
}

#[test]
fn test_q_key_quits() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1")]);

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(model.quitting);
    assert!(cmd.is_none());
}

#[test]
fn test_uppercase_q_key_quits() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1")]);

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('Q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(model.quitting);
    assert!(cmd.is_none());
}

#[test]
fn test_esc_key_quits() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1")]);

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(model.quitting);
    assert!(cmd.is_none());
}

#[test]
fn test_ctrl_c_quits() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1")]);

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(model.quitting);
    assert!(cmd.is_none());
}

#[test]
fn test_any_key_quits_after_selection() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1")]);
    model.selected = Some(0);
    model.choice = Some("Item 1".to_string());

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_any_key_quits_when_quitting() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1")]);
    model.quitting = true;

    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('x'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;

    let cmd = model.update(key_msg);

    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_initial_view() {
    let (model, _) = ListSimpleModel::init();
    let view = model.view();

    assert!(view.contains("What do you want for dinner?"));
    assert!(view.contains("  > 1. Ramen")); // First item selected
    assert!(view.contains("    2. Tomato Soup")); // Second item not selected
    assert!(view.contains("Use ↑/↓ to navigate"));
}

#[test]
fn test_view_with_different_cursor() {
    let mut model = ListSimpleModel::new(vec![
        ListItem::new("First"),
        ListItem::new("Second"),
        ListItem::new("Third"),
    ]);
    model.cursor = 1;

    let view = model.view();

    assert!(view.contains("    1. First")); // Not selected
    assert!(view.contains("  > 2. Second")); // Selected
    assert!(view.contains("    3. Third")); // Not selected
}

#[test]
fn test_view_after_selection() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Pizza")]);
    model.selected = Some(0);
    model.choice = Some("Pizza".to_string());

    let view = model.view();

    assert!(view.contains("You chose Pizza."));
    assert!(view.contains("Press any key to quit."));
    assert!(!view.contains("What do you want for dinner?")); // Should not show list
}

#[test]
fn test_view_when_quitting() {
    let mut model = ListSimpleModel::new(vec![ListItem::new("Item 1")]);
    model.quitting = true;

    let view = model.view();

    assert!(view.contains("Goodbye!"));
    assert!(view.contains("Press any key to quit."));
    assert!(!view.contains("What do you want for dinner?")); // Should not show list
}

#[test]
fn test_navigation_sequence() {
    let mut model = ListSimpleModel::new(vec![
        ListItem::new("First"),
        ListItem::new("Second"),
        ListItem::new("Third"),
    ]);

    // Move down twice
    model.move_cursor_down();
    assert_eq!(model.cursor, 1);

    model.move_cursor_down();
    assert_eq!(model.cursor, 2);

    // Move up once
    model.move_cursor_up();
    assert_eq!(model.cursor, 1);

    // Select current
    model.select_current();
    assert_eq!(model.selected, Some(1));
    assert_eq!(model.choice, Some("Second".to_string()));
}

#[test]
fn test_empty_list_handling() {
    let model = ListSimpleModel::new(vec![]);

    // Should handle empty list gracefully
    assert_eq!(model.items.len(), 0);
    assert_eq!(model.cursor, 0);

    let view = model.view();
    assert!(view.contains("What do you want for dinner?"));
}

#[test]
fn test_cursor_bounds_with_empty_list() {
    let mut model = ListSimpleModel::new(vec![]);

    // Should not crash with empty list
    model.move_cursor_down();
    assert_eq!(model.cursor, 0);

    model.move_cursor_up();
    assert_eq!(model.cursor, 0);
}

#[test]
fn test_selection_with_empty_list() {
    let mut model = ListSimpleModel::new(vec![]);

    model.select_current();

    // Should not select anything with empty list
    assert!(model.selected.is_none());
    assert!(model.choice.is_none());
}

#[test]
fn test_default_dinner_items() {
    let (model, _) = ListSimpleModel::init();

    // Verify the standard dinner items are present
    let expected_items = vec![
        "Ramen",
        "Tomato Soup",
        "Grilled Cheese",
        "Burger",
        "Pizza",
        "Tacos",
        "Pasta",
        "Salad",
        "Sushi",
        "Curry",
    ];

    for (i, expected) in expected_items.iter().enumerate() {
        assert_eq!(model.items[i].title, *expected);
    }
}
