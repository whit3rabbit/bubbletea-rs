// Test if List widget can use dots pagination
use bubbletea_widgets::list::{DefaultItem, DefaultDelegate, Model as List};
use bubbletea_widgets::paginator::Type;

fn main() {
    let items = vec![
        DefaultItem::new("Item 1", "Description 1"),
        DefaultItem::new("Item 2", "Description 2"),
        DefaultItem::new("Item 3", "Description 3"),
        DefaultItem::new("Item 4", "Description 4"),
        DefaultItem::new("Item 5", "Description 5"),
        DefaultItem::new("Item 6", "Description 6"),
        DefaultItem::new("Item 7", "Description 7"),
        DefaultItem::new("Item 8", "Description 8"),
        DefaultItem::new("Item 9", "Description 9"),
        DefaultItem::new("Item 10", "Description 10"),
        DefaultItem::new("Item 11", "Description 11"),
        DefaultItem::new("Item 12", "Description 12"),
    ];
    
    let delegate = DefaultDelegate::new();
    let mut list = List::new(items, delegate, 50, 5); // Small height to force pagination
    
    // Check if we can access and configure the paginator
    // This will show us if the List exposes paginator configuration
    
    println!("List created with {} items", list.items_len());
    
    // Try to check if there's a paginator field or method
    // to configure dots mode
}