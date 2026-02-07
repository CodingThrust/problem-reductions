use super::*;

#[test]
fn test_mapping_grid_create() {
    let grid = MappingGrid::new(10, 10, 4);
    assert_eq!(grid.size(), (10, 10));
    assert_eq!(grid.spacing(), 4);
}

#[test]
fn test_mapping_grid_with_padding() {
    let grid = MappingGrid::with_padding(8, 12, 3, 5);
    assert_eq!(grid.size(), (8, 12));
    assert_eq!(grid.spacing(), 3);
    assert_eq!(grid.padding(), 5);
}

#[test]
fn test_mapping_grid_add_node() {
    let mut grid = MappingGrid::new(10, 10, 4);
    grid.add_node(2, 3, 1);
    assert!(grid.is_occupied(2, 3));
    assert!(!grid.is_occupied(2, 4));
}

#[test]
fn test_mapping_grid_get_out_of_bounds() {
    let grid = MappingGrid::new(5, 5, 2);
    assert!(grid.get(0, 0).is_some());
    assert!(grid.get(4, 4).is_some());
    assert!(grid.get(5, 0).is_none());
    assert!(grid.get(0, 5).is_none());
    assert!(grid.get(10, 10).is_none());
}

#[test]
fn test_mapping_grid_add_node_doubled() {
    let mut grid = MappingGrid::new(10, 10, 4);
    grid.add_node(2, 3, 5);
    assert_eq!(grid.get(2, 3), Some(&CellState::Occupied { weight: 5 }));
    // Julia requires weights to match when doubling:
    // @assert m[i,j].weight == node.weight
    // Result keeps the same weight (not summed)
    grid.add_node(2, 3, 5);
    assert_eq!(grid.get(2, 3), Some(&CellState::Doubled { weight: 5 }));
}

#[test]
fn test_mapping_grid_connect() {
    let mut grid = MappingGrid::new(10, 10, 4);
    grid.add_node(3, 4, 7);
    assert_eq!(grid.get(3, 4), Some(&CellState::Occupied { weight: 7 }));
    grid.connect(3, 4);
    assert_eq!(grid.get(3, 4), Some(&CellState::Connected { weight: 7 }));
}

#[test]
fn test_mapping_grid_connect_empty_cell() {
    let mut grid = MappingGrid::new(10, 10, 4);
    grid.connect(3, 4);
    assert_eq!(grid.get(3, 4), Some(&CellState::Empty));
}

#[test]
fn test_mapping_grid_matches_pattern() {
    let mut grid = MappingGrid::new(10, 10, 4);
    grid.add_node(2, 2, 1);
    grid.add_node(2, 3, 1);
    grid.add_node(3, 2, 1);

    let pattern = vec![(0, 0), (0, 1), (1, 0)];
    assert!(grid.matches_pattern(&pattern, 2, 2));
    assert!(!grid.matches_pattern(&pattern, 0, 0));
}

#[test]
fn test_mapping_grid_matches_pattern_out_of_bounds() {
    let grid = MappingGrid::new(5, 5, 2);
    let pattern = vec![(0, 0), (1, 1)];
    assert!(!grid.matches_pattern(&pattern, 10, 10));
}

#[test]
fn test_mapping_grid_cross_at() {
    let grid = MappingGrid::new(20, 20, 4);
    // Julia's crossat uses larger position for col calculation (1-indexed)
    // Julia: row = (hslot - 1) * spacing + 2 + padding = 4 + 2 + 2 = 8
    // Julia: col = (larger_vslot - 1) * spacing + 1 + padding = 8 + 1 + 2 = 11
    // Rust 0-indexed: row = 8 - 1 = 7, col = 11 - 1 = 10
    let (row, col) = grid.cross_at(1, 3, 2);
    assert_eq!(row, 7); // 0-indexed
    assert_eq!(col, 10); // 0-indexed

    let (row2, col2) = grid.cross_at(3, 1, 2);
    assert_eq!((row, col), (row2, col2));
}

#[test]
fn test_cell_state_weight() {
    assert_eq!(CellState::Empty.weight(), 0);
    assert_eq!(CellState::Occupied { weight: 5 }.weight(), 5);
    assert_eq!(CellState::Doubled { weight: 10 }.weight(), 10);
    assert_eq!(CellState::Connected { weight: 3 }.weight(), 3);
}

#[test]
fn test_cell_state_is_empty() {
    assert!(CellState::Empty.is_empty());
    assert!(!CellState::Occupied { weight: 1 }.is_empty());
    assert!(!CellState::Doubled { weight: 2 }.is_empty());
    assert!(!CellState::Connected { weight: 1 }.is_empty());
}

#[test]
fn test_cell_state_is_occupied() {
    assert!(!CellState::Empty.is_occupied());
    assert!(CellState::Occupied { weight: 1 }.is_occupied());
    assert!(CellState::Doubled { weight: 2 }.is_occupied());
    assert!(CellState::Connected { weight: 1 }.is_occupied());
}

#[test]
fn test_mapping_grid_set() {
    let mut grid = MappingGrid::new(5, 5, 2);
    grid.set(2, 3, CellState::Occupied { weight: 7 });
    assert_eq!(grid.get(2, 3), Some(&CellState::Occupied { weight: 7 }));

    // Out of bounds set should be ignored
    grid.set(10, 10, CellState::Occupied { weight: 1 });
    assert!(grid.get(10, 10).is_none());
}

#[test]
fn test_mapping_grid_get_mut() {
    let mut grid = MappingGrid::new(5, 5, 2);
    grid.add_node(1, 1, 3);

    if let Some(cell) = grid.get_mut(1, 1) {
        *cell = CellState::Connected { weight: 5 };
    }
    assert_eq!(grid.get(1, 1), Some(&CellState::Connected { weight: 5 }));

    // Out of bounds get_mut should return None
    assert!(grid.get_mut(10, 10).is_none());
}

#[test]
fn test_mapping_grid_occupied_coords() {
    let mut grid = MappingGrid::new(5, 5, 2);
    grid.add_node(1, 2, 1);
    grid.add_node(3, 4, 2);
    grid.add_node(0, 0, 1);

    let coords = grid.occupied_coords();
    assert_eq!(coords.len(), 3);
    assert!(coords.contains(&(0, 0)));
    assert!(coords.contains(&(1, 2)));
    assert!(coords.contains(&(3, 4)));
}

#[test]
fn test_mapping_grid_add_node_out_of_bounds() {
    let mut grid = MappingGrid::new(5, 5, 2);
    // Should silently ignore out of bounds
    grid.add_node(10, 10, 1);
    assert!(grid.get(10, 10).is_none());
}

#[test]
fn test_mapping_grid_connect_out_of_bounds() {
    let mut grid = MappingGrid::new(5, 5, 2);
    // Should silently ignore out of bounds
    grid.connect(10, 10);
}

#[test]
fn test_cell_state_display() {
    assert_eq!(format!("{}", CellState::Empty), "⋅");
    assert_eq!(format!("{}", CellState::Occupied { weight: 1 }), "●");
    assert_eq!(format!("{}", CellState::Doubled { weight: 2 }), "◉");
    assert_eq!(format!("{}", CellState::Connected { weight: 1 }), "◇");
}

#[test]
fn test_mapping_grid_display() {
    let mut grid = MappingGrid::new(3, 3, 2);
    grid.add_node(0, 0, 1);
    grid.add_node(1, 1, 1);
    let display = format!("{}", grid);
    assert!(display.contains("●")); // Has occupied nodes
    assert!(display.contains("⋅")); // Has empty cells
}

#[test]
fn test_mapping_grid_format_with_config_none() {
    let mut grid = MappingGrid::new(3, 3, 2);
    grid.add_node(1, 1, 1);
    let output = grid.format_with_config(None);
    assert!(output.contains("●")); // Occupied nodes
}

#[test]
fn test_mapping_grid_format_with_config_some() {
    let mut grid = MappingGrid::new(3, 3, 2);
    grid.add_node(1, 1, 1);
    // Config with node at (1,1) selected
    let config = vec![0, 0, 0, 0, 1, 0, 0, 0, 0]; // 3x3 = 9 cells
    let output = grid.format_with_config(Some(&config));
    // Should have some output
    assert!(!output.is_empty());
}
