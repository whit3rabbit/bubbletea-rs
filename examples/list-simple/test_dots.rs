// Test to verify dots pagination works
use bubbletea_rs::Model;
use bubbletea_widgets::list::{DefaultDelegate, DefaultItem, Model as List};
use bubbletea_widgets::paginator::Type as PaginatorType;

fn main() {
    // Create items to force pagination
    let items: Vec<DefaultItem> = (1..=25)
        .map(|i| DefaultItem::new(&format!("Item {}", i), &format!("Description {}", i)))
        .collect();

    let delegate = DefaultDelegate::new();

    // Test with Arabic pagination (default)
    let arabic_list = List::new(items.clone(), delegate.clone(), 50, 8);
    println!("Arabic pagination (default):");
    println!("{}", arabic_list.view());
    println!();

    // Test with Dots pagination
    let mut dots_list =
        List::new(items.clone(), delegate.clone(), 50, 8).with_pagination_type(PaginatorType::Dots);

    // Ensure spinner is off to see if those are spinner dots
    dots_list.set_show_spinner(false);

    println!("Dots pagination (spinner off):");
    println!("{}", dots_list.view());
    println!();

    // Verify the pagination type
    assert_eq!(dots_list.pagination_type(), PaginatorType::Dots);
    println!("✓ Pagination type is correctly set to Dots");

    // Test standalone paginator to verify dots work
    println!("\nStandalone paginator test:");
    use bubbletea_widgets::paginator::Model as Paginator;
    let mut paginator = Paginator::new();
    paginator.set_per_page(5);
    paginator.set_total_items(25);
    paginator.paginator_type = PaginatorType::Dots;
    println!("Paginator with dots: '{}'", paginator.view());
}
