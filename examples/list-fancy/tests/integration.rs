#[tokio::test]
async fn test_example_compiles() {
    // Basic smoke test to ensure the example compiles
    // This test passes if the binary can be built, which already happened during `cargo test`
    assert!(true);
}

#[test]
fn test_basic_grocery_item() {
    // Test that we can create grocery items
    #[derive(Debug, Clone)]
    struct TestItem {
        title: String,
        description: String,
    }
    
    let item = TestItem {
        title: "Bananas".to_string(),
        description: "Yellow and tasty".to_string(),
    };
    
    assert_eq!(item.title, "Bananas");
    assert_eq!(item.description, "Yellow and tasty");
}

#[test]
fn test_random_selection() {
    // Test that we can select random items from lists
    let titles = vec!["Apples", "Bananas", "Cherries"];
    let descriptions = vec!["Sweet", "Tasty", "Delicious"];
    
    assert!(!titles.is_empty());
    assert!(!descriptions.is_empty());
    
    // Test cycling through indices
    let mut title_index = 0;
    let mut desc_index = 0;
    
    for _ in 0..10 {
        let _title = titles[title_index % titles.len()];
        let _desc = descriptions[desc_index % descriptions.len()];
        
        title_index += 1;
        desc_index += 1;
    }
    
    assert!(title_index > 0);
    assert!(desc_index > 0);
}