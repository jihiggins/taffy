#[test]
fn grid_percent_tracks_indefinite_only() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![percent(0.3f32), percent(0.6f32)],
                grid_template_columns: vec![percent(0.1f32), percent(0.2f32), percent(0.3f32)],
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node3.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node3.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node4.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node4.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node4.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node5.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node5.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node5.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node5.data(), 0f32, location.y);
}