// Test to check if Paginator supports dots pagination
use bubbletea_widgets::paginator::{Model as Paginator, Type};

fn main() {
    let mut paginator = Paginator::new();
    paginator.set_per_page(5);
    paginator.set_total_items(25);
    
    // Test Arabic mode (default)
    paginator.paginator_type = Type::Arabic;
    println!("Arabic view: '{}'", paginator.view());
    
    // Test Dots mode
    paginator.paginator_type = Type::Dots;
    println!("Dots view: '{}'", paginator.view());
    
    // Move to page 3 and test dots
    paginator.page = 2;
    println!("Dots view (page 3): '{}'", paginator.view());
    
    // Test custom dots
    paginator.active_dot = "●".to_string();
    paginator.inactive_dot = "○".to_string();
    println!("Custom dots view: '{}'", paginator.view());
}