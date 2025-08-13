use bubbletea_rs::Model as BubbleTeaModel;
use bubbletea_widgets::viewport;

#[test]
fn test_viewport_basic_paging() {
    // Height 5 -> effective content area is smaller due to default frame
    let mut vp = viewport::new(10, 5);
    vp.set_content("L1\nL2\nL3\nL4\nL5");

    // Initial visible top line should be L1
    let mut vis = vp.visible_lines();
    assert!(!vis.is_empty());
    assert_eq!(vis[0], "L1");

    // Page down; we should end up at bottom
    vp.page_down();
    assert!(vp.at_bottom());
    vis = vp.visible_lines();
    assert!(!vis.is_empty());
    assert_eq!(vis.last().unwrap(), "L5");

    // Goto top returns to the first block
    vp.goto_top();
    vis = vp.visible_lines();
    assert!(!vis.is_empty());
    assert_eq!(vis[0], "L1");
}

#[test]
fn test_viewport_horizontal_clipping() {
    // Width < content length to force clipping
    let mut vp = viewport::new(8, 3);
    let content = "HelloWorld"; // 10 chars
    vp.set_content(content);

    let before = vp.visible_lines();
    assert!(!before.is_empty());
    let first_before = &before[0];
    assert!(!first_before.is_empty());
    assert!(content.contains(first_before));
    assert!(first_before.len() <= 8);

    // Scroll right up to max offset
    vp.scroll_right();
    vp.scroll_right();

    let after = vp.visible_lines();
    assert!(!after.is_empty());
    let first_after = &after[0];
    assert!(!first_after.is_empty());
    assert!(content.contains(first_after));
    assert!(first_after.len() <= 8);
    assert_ne!(first_before, first_after);
}
