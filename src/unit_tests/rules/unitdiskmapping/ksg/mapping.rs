use super::*;

#[test]
fn test_embed_graph_path() {
    // Path graph: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let result = embed_graph(3, &edges, &[0, 1, 2]);

    assert!(result.is_some());
    let grid = result.unwrap();
    assert!(!grid.occupied_coords().is_empty());
}

#[test]
fn test_map_unweighted_triangle() {
    // Triangle graph
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_unweighted(3, &edges);

    assert!(!result.positions.is_empty());
    // mis_overhead can be negative due to gadgets, so we just verify the function completes
}

#[test]
fn test_map_weighted_triangle() {
    // Triangle graph
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_weighted(3, &edges);

    assert!(!result.positions.is_empty());
}

#[test]
fn test_mapping_result_config_back_unweighted() {
    let edges = vec![(0, 1)];
    let result = map_unweighted(2, &edges);

    // Create a dummy config
    let config: Vec<usize> = vec![0; result.positions.len()];
    let original = result.map_config_back(&config);

    assert_eq!(original.len(), 2);
}

#[test]
fn test_mapping_result_config_back_weighted() {
    let edges = vec![(0, 1)];
    let result = map_weighted(2, &edges);

    // Create a dummy config
    let config: Vec<usize> = vec![0; result.positions.len()];
    let original = result.map_config_back(&config);

    assert_eq!(original.len(), 2);
}

#[test]
fn test_map_config_copyback_simple() {
    // Create a simple copyline
    let line = CopyLine::new(0, 1, 1, 1, 1, 3);
    let lines = vec![line];

    // Create config with some nodes selected
    let locs = lines[0].copyline_locations(PADDING, SPACING);
    let (rows, cols) = (20, 20);
    let mut config = vec![vec![0; cols]; rows];

    // Select all nodes in copyline
    for &(row, col, _) in &locs {
        if row < rows && col < cols {
            config[row][col] = 1;
        }
    }

    let doubled_cells = HashSet::new();
    let result = map_config_copyback(&lines, PADDING, SPACING, &config, &doubled_cells);

    // count = len(locs) (all selected with ci=1), overhead = len/2
    // result = count - overhead = n - n/2 = n/2
    let n = locs.len();
    let overhead = n / 2;
    let expected = n - overhead;
    assert_eq!(result[0], expected);
}

#[test]
fn test_map_unweighted_with_method() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_unweighted_with_method(3, &edges, PathDecompositionMethod::greedy());

    assert!(!result.positions.is_empty());
}

#[test]
fn test_map_weighted_with_method() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_weighted_with_method(3, &edges, PathDecompositionMethod::greedy());

    assert!(!result.positions.is_empty());
}
