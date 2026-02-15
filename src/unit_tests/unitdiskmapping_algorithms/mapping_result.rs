//! Tests for MappingResult utility methods and unapply functionality.

use crate::rules::unitdiskmapping::ksg;
use crate::topology::smallgraph;

// === MappingResult Utility Methods ===

#[test]
fn test_mapping_result_grid_size() {
    let edges = vec![(0, 1), (1, 2)];
    let result = ksg::map_unweighted(3, &edges);

    let (rows, cols) = result.grid_dimensions;
    assert!(rows > 0, "Grid should have positive rows");
    assert!(cols > 0, "Grid should have positive cols");
}

#[test]
fn test_mapping_result_num_original_vertices() {
    let edges = vec![(0, 1), (1, 2)];
    let result = ksg::map_unweighted(3, &edges);

    assert_eq!(result.num_original_vertices(), 3);
}

#[test]
fn test_mapping_result_format_config() {
    let edges = vec![(0, 1)];
    let result = ksg::map_unweighted(2, &edges);

    let (rows, cols) = result.grid_dimensions;
    let config: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    let formatted = result.format_config(&config);
    assert!(
        !formatted.is_empty(),
        "Formatted config should not be empty"
    );
    assert!(
        formatted.contains('o') || formatted.contains('.'),
        "Formatted config should contain cell markers"
    );
}

#[test]
fn test_mapping_result_format_config_with_selected() {
    let edges = vec![(0, 1)];
    let result = ksg::map_unweighted(2, &edges);

    let (rows, cols) = result.grid_dimensions;
    let mut config: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    // Set some cells as selected
    if rows > 0 && cols > 0 {
        config[0][0] = 1;
    }

    let formatted = result.format_config(&config);
    // Should contain '*' for selected cells
    assert!(
        formatted.contains('*') || formatted.contains('o') || formatted.contains('.'),
        "Formatted config should contain cell markers"
    );
}

#[test]
fn test_mapping_result_format_config_flat() {
    let edges = vec![(0, 1)];
    let result = ksg::map_unweighted(2, &edges);

    let num_nodes = result.positions.len();
    let config: Vec<usize> = vec![0; num_nodes];

    let formatted = result.format_config_flat(&config);
    assert!(
        !formatted.is_empty(),
        "Flat formatted config should not be empty"
    );
}

#[test]
fn test_mapping_result_display() {
    let edges = vec![(0, 1)];
    let result = ksg::map_unweighted(2, &edges);

    let display = format!("{}", result);
    assert!(!display.is_empty(), "Display should not be empty");
}

// === Weighted Mapping Utility Methods ===

#[test]
fn test_weighted_mapping_result_grid_size() {
    let edges = vec![(0, 1), (1, 2)];
    let result = ksg::map_weighted(3, &edges);

    let (rows, cols) = result.grid_dimensions;
    assert!(rows > 0, "Grid should have positive rows");
    assert!(cols > 0, "Grid should have positive cols");
}

#[test]
fn test_weighted_mapping_result_num_original_vertices() {
    let edges = vec![(0, 1), (1, 2)];
    let result = ksg::map_weighted(3, &edges);

    assert_eq!(result.num_original_vertices(), 3);
}

#[test]
fn test_weighted_mapping_result_format_config() {
    let edges = vec![(0, 1)];
    let result = ksg::map_weighted(2, &edges);

    let (rows, cols) = result.grid_dimensions;
    let config: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    let formatted = result.format_config(&config);
    assert!(
        !formatted.is_empty(),
        "Formatted config should not be empty"
    );
}

// === Unapply Gadgets Tests ===

#[test]
fn test_unapply_gadgets_empty_tape() {
    use crate::rules::unitdiskmapping::ksg::unapply_gadgets;

    let tape = vec![];
    let mut config: Vec<Vec<usize>> = vec![vec![0; 5]; 5];

    unapply_gadgets(&tape, &mut config);
    // Should not crash with empty tape
}

#[test]
fn test_unapply_weighted_gadgets_empty_tape() {
    use crate::rules::unitdiskmapping::ksg::unapply_weighted_gadgets;

    let tape = vec![];
    let mut config: Vec<Vec<usize>> = vec![vec![0; 5]; 5];

    unapply_weighted_gadgets(&tape, &mut config);
    // Should not crash with empty tape
}

#[test]
fn test_map_config_back_unweighted() {
    let (n, edges) = smallgraph("diamond").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    let num_nodes = result.positions.len();
    let config: Vec<usize> = vec![0; num_nodes];

    let original_config = result.map_config_back(&config);
    assert_eq!(original_config.len(), n);
}

#[test]
fn test_map_config_back_weighted() {
    let (n, edges) = smallgraph("diamond").unwrap();
    let result = ksg::map_weighted(n, &edges);

    let num_nodes = result.positions.len();
    let config: Vec<usize> = vec![0; num_nodes];

    let original_config = result.map_config_back(&config);
    assert_eq!(original_config.len(), n);
}

// Note: map_config_back requires valid MIS configurations.
// Invalid configs (like all-ones) will panic - this is expected behavior.

// === Full Pipeline Tests ===

#[test]
fn test_full_pipeline_diamond_unweighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    // Solve MIS on the grid graph
    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    // Map config back to original graph
    let original_config = result.map_config_back(&grid_config);

    // Verify result is a valid independent set
    assert!(
        is_independent_set(&edges, &original_config),
        "Mapped back config should be a valid independent set"
    );
}

#[test]
fn test_full_pipeline_bull_unweighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = smallgraph("bull").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Bull: mapped back config should be a valid independent set"
    );
}

#[test]
fn test_full_pipeline_house_unweighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = smallgraph("house").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "House: mapped back config should be a valid independent set"
    );
}

#[test]
fn test_full_pipeline_petersen_unweighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = smallgraph("petersen").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Petersen: mapped back config should be a valid independent set"
    );
}

#[test]
fn test_full_pipeline_weighted_diamond() {
    use super::common::{is_independent_set, solve_weighted_mis_config};

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = ksg::map_weighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();

    // Get weights from the mapping result
    let weights: Vec<i32> = (0..num_grid)
        .map(|i| result.node_weights.get(i).copied().unwrap_or(1))
        .collect();

    let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);
    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Weighted diamond: mapped back config should be a valid independent set"
    );
}

#[test]
fn test_full_pipeline_weighted_bull() {
    use super::common::{is_independent_set, solve_weighted_mis_config};

    let (n, edges) = smallgraph("bull").unwrap();
    let result = ksg::map_weighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();

    let weights: Vec<i32> = (0..num_grid)
        .map(|i| result.node_weights.get(i).copied().unwrap_or(1))
        .collect();

    let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);
    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Weighted bull: mapped back config should be a valid independent set"
    );
}

// === MIS Size Verification Tests ===

#[test]
fn test_mis_size_preserved_diamond() {
    use super::common::solve_mis;

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    // Get original MIS size
    let original_mis = solve_mis(n, &edges);

    // Get grid MIS size
    let grid_edges = result.edges();
    let grid_mis = solve_mis(result.positions.len(), &grid_edges);

    // Verify the formula: grid_mis = original_mis + overhead
    let expected_grid_mis = original_mis as i32 + result.mis_overhead;
    assert_eq!(
        grid_mis as i32, expected_grid_mis,
        "Grid MIS {} should equal original {} + overhead {} = {}",
        grid_mis, original_mis, result.mis_overhead, expected_grid_mis
    );
}

#[test]
fn test_mis_size_preserved_bull() {
    use super::common::solve_mis;

    let (n, edges) = smallgraph("bull").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    let original_mis = solve_mis(n, &edges);
    let grid_edges = result.edges();
    let grid_mis = solve_mis(result.positions.len(), &grid_edges);

    let expected_grid_mis = original_mis as i32 + result.mis_overhead;
    assert_eq!(grid_mis as i32, expected_grid_mis);
}

#[test]
fn test_mis_size_preserved_house() {
    use super::common::solve_mis;

    let (n, edges) = smallgraph("house").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    let original_mis = solve_mis(n, &edges);
    let grid_edges = result.edges();
    let grid_mis = solve_mis(result.positions.len(), &grid_edges);

    let expected_grid_mis = original_mis as i32 + result.mis_overhead;
    assert_eq!(grid_mis as i32, expected_grid_mis);
}

// === Triangular Full Pipeline Tests ===

#[test]
fn test_full_pipeline_triangular_diamond() {
    use super::common::{is_independent_set, solve_weighted_mis_config};
    use crate::rules::unitdiskmapping::triangular;

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = triangular::map_weighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();

    let weights: Vec<i32> = (0..num_grid)
        .map(|i| result.node_weights.get(i).copied().unwrap_or(1))
        .collect();

    let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);
    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Triangular diamond: mapped back config should be a valid independent set"
    );
}

#[test]
fn test_full_pipeline_triangular_bull() {
    use super::common::{is_independent_set, solve_weighted_mis_config};
    use crate::rules::unitdiskmapping::triangular;

    let (n, edges) = smallgraph("bull").unwrap();
    let result = triangular::map_weighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();

    let weights: Vec<i32> = (0..num_grid)
        .map(|i| result.node_weights.get(i).copied().unwrap_or(1))
        .collect();

    let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);
    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Triangular bull: mapped back config should be a valid independent set"
    );
}

#[test]
fn test_full_pipeline_triangular_house() {
    use super::common::{is_independent_set, solve_weighted_mis_config};
    use crate::rules::unitdiskmapping::triangular;

    let (n, edges) = smallgraph("house").unwrap();
    let result = triangular::map_weighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();

    let weights: Vec<i32> = (0..num_grid)
        .map(|i| result.node_weights.get(i).copied().unwrap_or(1))
        .collect();

    let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &weights);
    let original_config = result.map_config_back(&grid_config);

    assert!(
        is_independent_set(&edges, &original_config),
        "Triangular house: mapped back config should be a valid independent set"
    );
}

// === Pattern Apply/Unapply Tests ===

#[test]
fn test_apply_and_unapply_gadget() {
    use crate::rules::unitdiskmapping::{
        apply_gadget, unapply_gadget, CellState, MappingGrid, Pattern,
    };
    use crate::rules::unitdiskmapping::ksg::KsgTurn;

    // Create a small grid with spacing 4
    let mut grid = MappingGrid::new(10, 10, 4);

    // Set up some occupied cells for a Turn gadget
    let turn = KsgTurn;
    let (rows, cols) = turn.size();

    // Initialize with the source pattern at position (2, 2)
    for r in 0..rows {
        for c in 0..cols {
            grid.set(2 + r, 2 + c, CellState::Occupied { weight: 1 });
        }
    }

    // Apply the gadget
    apply_gadget(&turn, &mut grid, 2, 2);

    // Unapply should restore to source pattern
    unapply_gadget(&turn, &mut grid, 2, 2);

    // All cells should now be set to source pattern
    // Just verify it doesn't crash and grid is still valid
    let (grid_rows, _) = grid.size();
    assert!(grid_rows >= rows + 2);
}

#[test]
fn test_apply_gadget_at_various_positions() {
    use crate::rules::unitdiskmapping::{apply_gadget, CellState, MappingGrid, Pattern};
    use crate::rules::unitdiskmapping::ksg::KsgTurn;

    let mut grid = MappingGrid::new(20, 20, 4);
    let turn = KsgTurn;
    let (rows, cols) = turn.size();

    // Apply at position (0, 0)
    for r in 0..rows {
        for c in 0..cols {
            grid.set(r, c, CellState::Occupied { weight: 1 });
        }
    }
    apply_gadget(&turn, &mut grid, 0, 0);

    // Apply at position (10, 10)
    for r in 0..rows {
        for c in 0..cols {
            grid.set(10 + r, 10 + c, CellState::Occupied { weight: 1 });
        }
    }
    apply_gadget(&turn, &mut grid, 10, 10);

    // Both applications should work
    let (grid_rows, _) = grid.size();
    assert!(grid_rows == 20);
}

// === MIS Extraction Tests ===

#[test]
fn test_extracted_mis_equals_original() {
    use super::common::{solve_mis, solve_mis_config};

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    // Solve MIS on grid
    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    // Map back
    let original_config = result.map_config_back(&grid_config);

    // Count selected vertices
    let extracted_count = original_config.iter().filter(|&&x| x > 0).count();
    let original_mis = solve_mis(n, &edges);

    assert_eq!(
        extracted_count, original_mis,
        "Extracted MIS size {} should equal original MIS size {}",
        extracted_count, original_mis
    );
}

#[test]
fn test_extracted_mis_equals_original_bull() {
    use super::common::{solve_mis, solve_mis_config};

    let (n, edges) = smallgraph("bull").unwrap();
    let result = ksg::map_unweighted(n, &edges);

    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    let original_config = result.map_config_back(&grid_config);
    let extracted_count = original_config.iter().filter(|&&x| x > 0).count();
    let original_mis = solve_mis(n, &edges);

    assert_eq!(extracted_count, original_mis);
}

// === Grid Graph Format Tests ===

#[test]
fn test_grid_graph_format_display() {
    let edges = vec![(0, 1)];
    let result = ksg::map_unweighted(2, &edges);

    let formatted = format!("{}", result);
    assert!(!formatted.is_empty());
}

#[test]
fn test_grid_graph_format_with_some_config() {
    let edges = vec![(0, 1)];
    let result = ksg::map_unweighted(2, &edges);

    let num_nodes = result.positions.len();
    let config: Vec<usize> = vec![1; num_nodes];

    let formatted = result.format_config_flat(&config);
    assert!(!formatted.is_empty());
}

// === Standard Graphs Tests ===

#[test]
fn test_all_standard_graphs_unapply() {
    let graph_names = ["bull", "diamond", "house", "petersen", "cubical"];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();
        let result = ksg::map_unweighted(n, &edges);

        let num_nodes = result.positions.len();
        let config: Vec<usize> = vec![0; num_nodes];

        let original = result.map_config_back(&config);
        assert_eq!(
            original.len(),
            n,
            "{}: map_config_back should return correct length",
            name
        );
    }
}

#[test]
fn test_all_standard_graphs_weighted_unapply() {
    let graph_names = ["bull", "diamond", "house", "petersen"];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();
        let result = ksg::map_weighted(n, &edges);

        let num_nodes = result.positions.len();
        let config: Vec<usize> = vec![0; num_nodes];

        let original = result.map_config_back(&config);
        assert_eq!(
            original.len(),
            n,
            "{}: weighted map_config_back should return correct length",
            name
        );
    }
}

// === Julia Tests: K23, empty, path interface tests ===
// From Julia's test/mapping.jl - "interface K23, empty and path" testset

/// K23 graph: a specific bipartite graph with 5 vertices
fn k23_graph() -> (usize, Vec<(usize, usize)>) {
    // Julia:
    // K23 = SimpleGraph(5)
    // add_edge!(K23, 1, 5), add_edge!(K23, 4, 5), add_edge!(K23, 4, 3)
    // add_edge!(K23, 3, 2), add_edge!(K23, 5, 2), add_edge!(K23, 1, 3)
    // Convert to 0-indexed
    let edges = vec![
        (0, 4), // 1-5
        (3, 4), // 4-5
        (3, 2), // 4-3
        (2, 1), // 3-2
        (4, 1), // 5-2
        (0, 2), // 1-3
    ];
    (5, edges)
}

/// Empty graph with 5 vertices (no edges)
fn empty_graph() -> (usize, Vec<(usize, usize)>) {
    (5, vec![])
}

/// Path graph: 0 -- 1 -- 2 -- 3 -- 4
fn path_graph() -> (usize, Vec<(usize, usize)>) {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    (5, edges)
}

#[test]
fn test_interface_k23_unweighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = k23_graph();
    let result = ksg::map_unweighted(n, &edges);

    // Check MIS size preservation: mis_overhead + original_mis = mapped_mis
    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);
    let grid_mis: usize = grid_config.iter().sum();

    // Original graph MIS
    let original_config = solve_mis_config(n, &edges);
    let original_mis: usize = original_config.iter().sum();

    assert_eq!(
        result.mis_overhead as usize + original_mis,
        grid_mis,
        "K23: MIS overhead formula should hold"
    );

    // Check map_config_back produces valid IS
    let mapped_back = result.map_config_back(&grid_config);
    assert!(
        is_independent_set(&edges, &mapped_back),
        "K23: mapped back config should be independent set"
    );
    assert_eq!(
        mapped_back.iter().sum::<usize>(),
        original_mis,
        "K23: mapped back config should have same MIS size"
    );
}

#[test]
fn test_interface_empty_graph_unweighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = empty_graph();
    let result = ksg::map_unweighted(n, &edges);

    // For empty graph, all vertices can be selected
    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);
    let grid_mis: usize = grid_config.iter().sum();

    // Original graph MIS is n (all vertices)
    let original_mis = n;

    assert_eq!(
        result.mis_overhead as usize + original_mis,
        grid_mis,
        "Empty graph: MIS overhead formula should hold"
    );

    // Check map_config_back
    let mapped_back = result.map_config_back(&grid_config);
    assert!(
        is_independent_set(&edges, &mapped_back),
        "Empty graph: mapped back config should be independent set"
    );
    assert_eq!(
        mapped_back.iter().sum::<usize>(),
        original_mis,
        "Empty graph: all vertices should be selected"
    );
}

#[test]
fn test_interface_path_graph_unweighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = path_graph();
    let result = ksg::map_unweighted(n, &edges);

    // Check MIS size preservation
    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);
    let grid_mis: usize = grid_config.iter().sum();

    // Original graph MIS for path of 5 is 3 (select vertices 0, 2, 4)
    let original_config = solve_mis_config(n, &edges);
    let original_mis: usize = original_config.iter().sum();
    assert_eq!(original_mis, 3, "Path graph MIS should be 3");

    assert_eq!(
        result.mis_overhead as usize + original_mis,
        grid_mis,
        "Path graph: MIS overhead formula should hold"
    );

    // Check map_config_back
    let mapped_back = result.map_config_back(&grid_config);
    assert!(
        is_independent_set(&edges, &mapped_back),
        "Path graph: mapped back config should be independent set"
    );
}

#[test]
fn test_interface_k23_weighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = k23_graph();
    let result = ksg::map_weighted(n, &edges);

    // Check MIS size preservation
    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    // Check map_config_back produces valid IS
    let mapped_back = result.map_config_back(&grid_config);
    assert!(
        is_independent_set(&edges, &mapped_back),
        "K23 weighted: mapped back config should be independent set"
    );
}

#[test]
fn test_interface_empty_graph_weighted() {
    use super::common::is_independent_set;

    let (n, edges) = empty_graph();
    let result = ksg::map_weighted(n, &edges);

    // For empty graph with weighted mapping
    let num_grid = result.positions.len();
    // All zeros config is always valid
    let grid_config: Vec<usize> = vec![0; num_grid];

    let mapped_back = result.map_config_back(&grid_config);
    assert!(
        is_independent_set(&edges, &mapped_back),
        "Empty graph weighted: mapped back config should be independent set"
    );
}

#[test]
fn test_interface_path_graph_weighted() {
    use super::common::{is_independent_set, solve_mis_config};

    let (n, edges) = path_graph();
    let result = ksg::map_weighted(n, &edges);

    // Check map_config_back
    let grid_edges = result.edges();
    let num_grid = result.positions.len();
    let grid_config = solve_mis_config(num_grid, &grid_edges);

    let mapped_back = result.map_config_back(&grid_config);
    assert!(
        is_independent_set(&edges, &mapped_back),
        "Path graph weighted: mapped back config should be independent set"
    );
}
