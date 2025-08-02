use bubbletea_rs::{Model, Msg, KeyMsg, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};

#[path = "../main.rs"]
mod list_default_main;
use list_default_main::{ListDefaultModel, ListItem};

#[test]
fn test_list_default_model_init() {
    let (model, cmd) = ListDefaultModel::init();
    
    // Should start with 23 items from the "My Fave Things" list
    assert_eq!(model.items.len(), 23);
    assert_eq!(model.cursor, 0);
    assert!(model.selected.is_none());
    assert!(model.filter_text.is_empty());
    assert!(model.show_help);
    assert!(model.show_status);
    assert_eq!(model.filtered_indices.len(), 23); // All items initially visible
    assert_eq!(model.window_width, 80);
    assert_eq!(model.window_height, 24);
    assert_eq!(model.view_offset, 0);
    
    // Should not return any initial command
    assert!(cmd.is_none());
}

#[test]
fn test_list_item_creation() {
    let item = ListItem::new("Test Title", "Test Description");
    assert_eq!(item.title, "Test Title");
    assert_eq!(item.description, "Test Description");
    assert_eq!(item.filter_value(), "Test Title");
}

#[test]
fn test_list_default_model_new() {
    let items = vec![
        ListItem::new("Item 1", "Description 1"),
        ListItem::new("Item 2", "Description 2"),
        ListItem::new("Item 3", "Description 3"),
    ];
    
    let model = ListDefaultModel::new(items.clone());
    
    assert_eq!(model.items, items);
    assert_eq!(model.cursor, 0);
    assert!(model.selected.is_none());
    assert!(model.filter_text.is_empty());
    assert_eq!(model.filtered_indices, vec![0, 1, 2]);
}

#[test]
fn test_filtered_items() {
    let items = vec![
        ListItem::new("Apple", "A fruit"),
        ListItem::new("Banana", "Another fruit"),
        ListItem::new("Carrot", "A vegetable"),
    ];
    let model = ListDefaultModel::new(items);
    
    let filtered = model.filtered_items();
    assert_eq!(filtered.len(), 3);
    assert_eq!(filtered[0].title, "Apple");
    assert_eq!(filtered[1].title, "Banana");
    assert_eq!(filtered[2].title, "Carrot");
}

#[test]
fn test_current_item() {
    let items = vec![
        ListItem::new("First", "First item"),
        ListItem::new("Second", "Second item"),
        ListItem::new("Third", "Third item"),
    ];
    let mut model = ListDefaultModel::new(items);
    
    let current = model.current_item().unwrap();
    assert_eq!(current.title, "First");
    
    model.cursor = 1;
    let current = model.current_item().unwrap();
    assert_eq!(current.title, "Second");
    
    model.cursor = 2;
    let current = model.current_item().unwrap();
    assert_eq!(current.title, "Third");
}

#[test]
fn test_current_item_with_empty_list() {
    let model = ListDefaultModel::new(vec![]);
    assert!(model.current_item().is_none());
}

#[test]
fn test_move_cursor_down() {
    let items = vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
        ListItem::new("Item 3", "Desc 3"),
    ];
    let mut model = ListDefaultModel::new(items);
    
    assert_eq!(model.cursor, 0);
    
    model.move_cursor_down();
    assert_eq!(model.cursor, 1);
    
    model.move_cursor_down();
    assert_eq!(model.cursor, 2);
}

#[test]
fn test_move_cursor_down_at_end() {
    let items = vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
    ];
    let mut model = ListDefaultModel::new(items);
    model.cursor = 1; // At last item
    
    model.move_cursor_down();
    
    // Should not move beyond last item
    assert_eq!(model.cursor, 1);
}

#[test]
fn test_move_cursor_up() {
    let items = vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
        ListItem::new("Item 3", "Desc 3"),
    ];
    let mut model = ListDefaultModel::new(items);
    model.cursor = 2;
    
    model.move_cursor_up();
    assert_eq!(model.cursor, 1);
    
    model.move_cursor_up();
    assert_eq!(model.cursor, 0);
}

#[test]
fn test_move_cursor_up_at_beginning() {
    let items = vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
    ];
    let mut model = ListDefaultModel::new(items);
    model.cursor = 0; // At first item
    
    model.move_cursor_up();
    
    // Should not move beyond first item
    assert_eq!(model.cursor, 0);
}

#[test]
fn test_select_current() {
    let items = vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
        ListItem::new("Item 3", "Desc 3"),
    ];
    let mut model = ListDefaultModel::new(items);
    model.cursor = 1;
    
    model.select_current();
    
    assert_eq!(model.selected, Some(1));
}

#[test]
fn test_select_current_first_item() {
    let items = vec![
        ListItem::new("First", "First item"),
        ListItem::new("Second", "Second item"),
    ];
    let mut model = ListDefaultModel::new(items);
    model.cursor = 0;
    
    model.select_current();
    
    assert_eq!(model.selected, Some(0));
}

#[test]
fn test_apply_filter_title_match() {
    let items = vec![
        ListItem::new("Apple Pie", "A delicious dessert"),
        ListItem::new("Banana Bread", "A sweet bread"),
        ListItem::new("Cherry Tart", "A fruity dessert"),
    ];
    let mut model = ListDefaultModel::new(items);
    
    model.filter_text = "apple".to_string();
    model.apply_filter();
    
    assert_eq!(model.filtered_indices, vec![0]);
    assert_eq!(model.cursor, 0);
    assert_eq!(model.view_offset, 0);
}

#[test]
fn test_apply_filter_description_match() {
    let items = vec![
        ListItem::new("Apple Pie", "A delicious dessert"),
        ListItem::new("Banana Bread", "A sweet bread"),
        ListItem::new("Cherry Tart", "A fruity dessert"),
    ];
    let mut model = ListDefaultModel::new(items);
    
    model.filter_text = "dessert".to_string();
    model.apply_filter();
    
    assert_eq!(model.filtered_indices, vec![0, 2]); // Apple Pie and Cherry Tart
    assert_eq!(model.cursor, 0);
}

#[test]
fn test_apply_filter_case_insensitive() {
    let items = vec![
        ListItem::new("Apple Pie", "A delicious dessert"),
        ListItem::new("Banana Bread", "A sweet bread"),
    ];
    let mut model = ListDefaultModel::new(items);
    
    model.filter_text = "APPLE".to_string();
    model.apply_filter();
    
    assert_eq!(model.filtered_indices, vec![0]);
}

#[test]
fn test_apply_filter_no_matches() {
    let items = vec![
        ListItem::new("Apple Pie", "A delicious dessert"),
        ListItem::new("Banana Bread", "A sweet bread"),
    ];
    let mut model = ListDefaultModel::new(items);
    
    model.filter_text = "pizza".to_string();
    model.apply_filter();
    
    assert_eq!(model.filtered_indices, vec![]);
}

#[test]
fn test_apply_filter_empty_text() {
    let items = vec![
        ListItem::new("Apple Pie", "A delicious dessert"),
        ListItem::new("Banana Bread", "A sweet bread"),
    ];
    let mut model = ListDefaultModel::new(items);
    
    // First apply a filter
    model.filter_text = "apple".to_string();
    model.apply_filter();
    assert_eq!(model.filtered_indices, vec![0]);
    
    // Then clear the filter
    model.filter_text.clear();
    model.apply_filter();
    
    assert_eq!(model.filtered_indices, vec![0, 1]); // All items visible again
}

#[test]
fn test_update_window_size() {
    let items = vec![ListItem::new("Item", "Description")];
    let mut model = ListDefaultModel::new(items);
    
    model.update_window_size(100, 30);
    
    assert_eq!(model.window_width, 100);
    assert_eq!(model.window_height, 30);
    assert_eq!(model.items_per_page, 24); // 30 - 6 reserved lines
}

#[test]
fn test_update_window_size_small() {
    let items = vec![ListItem::new("Item", "Description")];
    let mut model = ListDefaultModel::new(items);
    
    model.update_window_size(50, 10);
    
    assert_eq!(model.window_width, 50);
    assert_eq!(model.window_height, 10);
    assert_eq!(model.items_per_page, 4); // 10 - 6 reserved lines
}

#[test]
fn test_render_item_normal() {
    let items = vec![ListItem::new("Test Item", "Test Description")];
    let mut model = ListDefaultModel::new(items);
    model.cursor = 1; // Set cursor to different item so item 0 is not selected
    
    let item = &model.items[0];
    let (title, desc) = model.render_item(0, item);
    
    assert_eq!(title, "  1. Test Item");
    assert_eq!(desc, "   Test Description");
}

#[test]
fn test_render_item_selected() {
    let items = vec![ListItem::new("Selected Item", "Selected Description")];
    let mut model = ListDefaultModel::new(items);
    model.cursor = 0; // Item 0 is selected
    
    let item = &model.items[0];
    let (title, desc) = model.render_item(0, item);
    
    assert_eq!(title, "▸ 1. Selected Item");
    assert_eq!(desc, "   Selected Description");
}

#[test]
fn test_down_arrow_key() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
        ListItem::new("Item 3", "Desc 3"),
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
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
        ListItem::new("Item 3", "Desc 3"),
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
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item 1", "Desc 1"),
        ListItem::new("Item 2", "Desc 2"),
    ]);
    model.cursor = 1;
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert_eq!(model.selected, Some(1));
    assert!(cmd.is_some()); // Should quit after selection
}

#[test]
fn test_q_key_quits() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item 1", "Desc 1"),
    ]);
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_ctrl_c_quits() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item 1", "Desc 1"),
    ]);
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit
}

#[test]
fn test_h_key_toggles_help() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item 1", "Desc 1"),
    ]);
    
    assert!(model.show_help); // Initially true
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('h'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg);
    assert!(!model.show_help); // Should toggle to false
    
    let key_msg2 = Box::new(KeyMsg {
        key: KeyCode::Char('h'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg2);
    assert!(model.show_help); // Should toggle back to true
}

#[test]
fn test_s_key_toggles_status() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item 1", "Desc 1"),
    ]);
    
    assert!(model.show_status); // Initially true
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('s'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg);
    assert!(!model.show_status); // Should toggle to false
    
    let key_msg2 = Box::new(KeyMsg {
        key: KeyCode::Char('s'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg2);
    assert!(model.show_status); // Should toggle back to true
}

#[test]
fn test_character_input_filters() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Apple", "Fruit"),
        ListItem::new("Banana", "Also fruit"),
        ListItem::new("Carrot", "Vegetable"),
    ]);
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert_eq!(model.filter_text, "a");
    assert_eq!(model.filtered_indices, vec![0, 1, 2]); // "a" matches Apple, Banana, Carrot
    assert!(cmd.is_none());
}

#[test]
fn test_backspace_clears_filter() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Apple", "Fruit"),
        ListItem::new("Banana", "Fruit"),
    ]);
    
    // First add some filter text
    model.filter_text = "app".to_string();
    model.apply_filter();
    assert_eq!(model.filtered_indices, vec![0]); // Only Apple matches
    
    // Then backspace once
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    model.update(key_msg);
    
    assert_eq!(model.filter_text, "ap");
    assert_eq!(model.filtered_indices, vec![0]); // Still only Apple
}

#[test]
fn test_backspace_with_empty_filter() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Apple", "Fruit"),
    ]);
    
    assert!(model.filter_text.is_empty());
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(model.filter_text.is_empty()); // Should remain empty
    assert!(cmd.is_none());
}

#[test]
fn test_esc_key_clears_filter() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Apple", "Fruit"),
        ListItem::new("Banana", "Fruit"),
    ]);
    
    // Add filter text
    model.filter_text = "apple".to_string();
    model.apply_filter();
    assert_eq!(model.filtered_indices, vec![0]);
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(model.filter_text.is_empty()); // Filter should be cleared
    assert_eq!(model.filtered_indices, vec![0, 1]); // All items visible again
    assert!(cmd.is_none()); // Should not quit when clearing filter
}

#[test]
fn test_esc_key_quits_when_no_filter() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Apple", "Fruit"),
    ]);
    
    assert!(model.filter_text.is_empty()); // No filter active
    
    let key_msg = Box::new(KeyMsg {
        key: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
    }) as Msg;
    
    let cmd = model.update(key_msg);
    
    assert!(cmd.is_some()); // Should quit when no filter to clear
}

#[test]
fn test_window_size_message() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item", "Description"),
    ]);
    
    let size_msg = Box::new(WindowSizeMsg {
        width: 120,
        height: 40,
    }) as Msg;
    
    let cmd = model.update(size_msg);
    
    assert_eq!(model.window_width, 120);
    assert_eq!(model.window_height, 40);
    assert_eq!(model.items_per_page, 34); // 40 - 6
    assert!(cmd.is_none());
}

#[test]
fn test_initial_view() {
    let (model, _) = ListDefaultModel::init();
    let view = model.view();
    
    assert!(view.contains("My Fave Things"));
    assert!(view.contains("▸ 1. Raspberry Pi's")); // First item selected
    assert!(view.contains("   I have 'em all over my house")); // Description
    assert!(view.contains("  2. Nutella")); // Second item not selected
    assert!(view.contains("1 of 23 items")); // Status
    assert!(view.contains("↑/↓: navigate")); // Help
}

#[test]
fn test_view_with_filter() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Apple Pie", "Delicious dessert"),
        ListItem::new("Banana Bread", "Sweet bread"),
        ListItem::new("Cherry Tart", "Fruity dessert"),
    ]);
    
    model.filter_text = "apple".to_string();
    model.apply_filter();
    
    let view = model.view();
    
    assert!(view.contains("Filter: apple"));
    assert!(view.contains("▸ 1. Apple Pie")); // Only filtered item
    assert!(!view.contains("Banana Bread")); // Filtered out
    assert!(view.contains("1 of 1 items (filtered from 3)")); // Status shows filtering
}

#[test]
fn test_view_without_help() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item", "Description"),
    ]);
    model.show_help = false;
    
    let view = model.view();
    
    assert!(!view.contains("↑/↓: navigate")); // Help should be hidden
    assert!(view.contains("My Fave Things")); // Title should still be there
}

#[test]
fn test_view_without_status() {
    let mut model = ListDefaultModel::new(vec![
        ListItem::new("Item", "Description"),
    ]);
    model.show_status = false;
    
    let view = model.view();
    
    assert!(!view.contains("1 of 1 items")); // Status should be hidden
    assert!(view.contains("My Fave Things")); // Title should still be there
}

#[test]
fn test_default_items_content() {
    let (model, _) = ListDefaultModel::init();
    
    // Verify some of the expected items are present
    assert_eq!(model.items[0].title, "Raspberry Pi's");
    assert_eq!(model.items[0].description, "I have 'em all over my house");
    
    assert_eq!(model.items[1].title, "Nutella");
    assert_eq!(model.items[1].description, "It's good on toast");
    
    assert_eq!(model.items[10].title, "Linux");
    assert_eq!(model.items[10].description, "Pretty much the best OS");
    
    // Verify total count
    assert_eq!(model.items.len(), 23);
}