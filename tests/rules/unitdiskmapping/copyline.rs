//! Tests for copyline functionality (src/rules/mapping/copyline.rs).

use problemreductions::rules::unitdiskmapping::{map_graph, map_graph_triangular, CopyLine};

#[test]
fn test_copylines_have_valid_vertex_ids() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    for line in &result.lines {
        assert!(line.vertex < 3, "Vertex ID should be in range");
    }
}

#[test]
fn test_copylines_have_positive_slots() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    for line in &result.lines {
        assert!(line.vslot > 0, "vslot should be positive");
        assert!(line.hslot > 0, "hslot should be positive");
    }
}

#[test]
fn test_copylines_have_valid_ranges() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    for line in &result.lines {
        assert!(line.vstart <= line.vstop, "vstart should be <= vstop");
        assert!(line.vstart <= line.hslot, "vstart should be <= hslot");
        assert!(line.hslot <= line.vstop, "hslot should be <= vstop");
    }
}

#[test]
fn test_copyline_center_location() {
    let line = CopyLine::new(0, 2, 3, 1, 3, 4);
    let (row, col) = line.center_location(1, 4);
    // row = 4 * (3-1) + 1 + 2 = 8 + 3 = 11
    // col = 4 * (2-1) + 1 + 1 = 4 + 2 = 6
    assert_eq!(row, 11);
    assert_eq!(col, 6);
}

#[test]
fn test_copyline_center_location_offset() {
    // Test with different padding and spacing
    let line = CopyLine::new(0, 1, 1, 1, 1, 2);
    let (row, col) = line.center_location(2, 4);
    // row = 4 * (1-1) + 2 + 2 = 0 + 4 = 4
    // col = 4 * (1-1) + 2 + 1 = 0 + 3 = 3
    assert_eq!(row, 4);
    assert_eq!(col, 3);
}

#[test]
fn test_copyline_locations_basic() {
    let line = CopyLine::new(0, 1, 1, 1, 2, 2);
    let locs = line.locations(2, 4);

    // Should have nodes at vertical and horizontal segments
    assert!(!locs.is_empty());

    // All locations should have positive coordinates
    for &(row, col, weight) in &locs {
        assert!(row > 0);
        assert!(col > 0);
        assert!(weight >= 1);
    }
}

#[test]
fn test_copyline_dense_locations() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let locs = line.dense_locations(2, 4);

    assert!(!locs.is_empty());

    // Dense locations should have more nodes than sparse
    let sparse_locs = line.locations(2, 4);
    assert!(
        locs.len() >= sparse_locs.len(),
        "Dense should have at least as many nodes as sparse"
    );
}

#[test]
fn test_copyline_dense_locations_triangular() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let locs = line.dense_locations_triangular(2, 6);

    assert!(!locs.is_empty());

    // All weights should be valid
    for &(row, col, weight) in &locs {
        assert!(row > 0 || col > 0); // At least one coordinate non-zero
        assert!(weight >= 1);
    }
}

#[test]
fn test_mapping_result_has_copylines() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph(3, &edges);

    assert_eq!(result.lines.len(), 3);

    // Each vertex should have exactly one copy line
    let mut found = [false; 3];
    for line in &result.lines {
        found[line.vertex] = true;
    }
    assert!(found.iter().all(|&x| x));
}

#[test]
fn test_triangular_mapping_result_has_copylines() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_copyline_vslot_hslot_ordering() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph(3, &edges);

    // vslot is determined by vertex order, should be 1-indexed
    let mut vslots: Vec<usize> = result.lines.iter().map(|l| l.vslot).collect();
    vslots.sort();

    // vslots should be 1, 2, 3 for 3 vertices
    assert!(vslots.contains(&1));
    assert!(vslots.contains(&2));
    assert!(vslots.contains(&3));
}

#[test]
fn test_copyline_center_on_grid() {
    let edges = vec![(0, 1)];
    let result = map_graph(2, &edges);

    // Each copyline's center should correspond to a grid node
    for line in &result.lines {
        let (row, col) = line.center_location(result.padding, result.spacing);
        // Center should be at a valid grid position
        assert!(row >= result.padding);
        assert!(col >= result.padding);
    }
}

#[test]
fn test_copyline_serialization() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let json = serde_json::to_string(&line).unwrap();
    let deserialized: CopyLine = serde_json::from_str(&json).unwrap();
    assert_eq!(line, deserialized);
}

#[test]
fn test_copyline_hstop_determines_width() {
    // hstop determines horizontal extent
    let line1 = CopyLine::new(0, 1, 2, 1, 2, 3);
    let line2 = CopyLine::new(0, 1, 2, 1, 2, 5);

    let locs1 = line1.locations(2, 4);
    let locs2 = line2.locations(2, 4);

    // Line with larger hstop should have more nodes
    assert!(locs2.len() >= locs1.len());
}

#[test]
fn test_copyline_vstop_determines_height() {
    // vstop determines vertical extent
    let line1 = CopyLine::new(0, 1, 3, 1, 3, 3);
    let line2 = CopyLine::new(0, 1, 5, 1, 5, 5);

    let locs1 = line1.locations(2, 4);
    let locs2 = line2.locations(2, 4);

    // Line with larger vstop should have more nodes
    assert!(locs2.len() >= locs1.len());
}

#[test]
fn test_copyline_weights_positive() {
    let line = CopyLine::new(0, 2, 3, 1, 3, 5);
    let locs = line.locations(2, 4);

    // All weights should be positive
    for &(_row, _col, weight) in &locs {
        assert!(weight >= 1, "All weights should be positive");
    }
}

#[test]
fn test_copyline_dense_locations_structure() {
    let line = CopyLine::new(0, 2, 3, 1, 3, 5);
    let dense = line.dense_locations(2, 4);

    // Dense locations should have multiple nodes
    assert!(dense.len() > 1, "Dense should have multiple nodes");

    // Check weights follow pattern (ends are 1, middle can be 2)
    let weights: Vec<usize> = dense.iter().map(|&(_, _, w)| w).collect();
    assert!(weights.iter().all(|&w| w == 1 || w == 2));
}

#[test]
fn test_copyline_triangular_spacing() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    // Triangular uses spacing=6
    assert_eq!(result.spacing, 6);

    // Each copyline should produce valid triangular locations
    for line in &result.lines {
        let locs = line.dense_locations_triangular(result.padding, result.spacing);
        assert!(!locs.is_empty());
    }
}

#[test]
fn test_copyline_center_vs_locations() {
    let line = CopyLine::new(0, 2, 3, 1, 3, 4);
    let (center_row, center_col) = line.center_location(2, 4);
    let locs = line.locations(2, 4);

    // Center should be within the bounding box of locations
    let min_row = locs.iter().map(|&(r, _, _)| r).min().unwrap();
    let max_row = locs.iter().map(|&(r, _, _)| r).max().unwrap();
    let min_col = locs.iter().map(|&(_, c, _)| c).min().unwrap();
    let max_col = locs.iter().map(|&(_, c, _)| c).max().unwrap();

    assert!(
        center_row >= min_row && center_row <= max_row,
        "Center row should be within location bounds"
    );
    assert!(
        center_col >= min_col && center_col <= max_col,
        "Center col should be within location bounds"
    );
}
