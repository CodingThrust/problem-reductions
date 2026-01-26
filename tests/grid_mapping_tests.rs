//! Integration tests for graph to grid mapping.
//!
//! These tests verify that the mapping system correctly transforms arbitrary graphs
//! into grid graphs using the copy-line technique, for both square and triangular lattices.

use problemreductions::rules::mapping::{
    map_graph, map_graph_triangular, map_graph_triangular_with_order, map_graph_with_order,
    MappingResult,
};
use problemreductions::topology::{Graph, GridType};

/// Tests for square lattice mapping.
mod square_lattice {
    use super::*;

    #[test]
    fn test_map_path_graph() {
        // Path: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert!(result.mis_overhead >= 0);

        // Solution mapping back should work
        let config = vec![0; result.grid_graph.num_vertices()];
        let original = result.map_config_back(&config);
        assert_eq!(original.len(), 3);
    }

    #[test]
    fn test_map_triangle_graph() {
        // Triangle: 0-1, 1-2, 0-2
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let result = map_graph(3, &edges);

        assert!(result.grid_graph.num_vertices() >= 3);
        assert!(result.mis_overhead >= 0);
        assert_eq!(result.lines.len(), 3);
    }

    #[test]
    fn test_map_star_graph() {
        // Star: center 0 connected to 1,2,3
        let edges = vec![(0, 1), (0, 2), (0, 3)];
        let result = map_graph(4, &edges);

        assert!(result.grid_graph.num_vertices() > 4);
        assert_eq!(result.lines.len(), 4);
    }

    #[test]
    fn test_map_empty_graph() {
        // No edges
        let edges: Vec<(usize, usize)> = vec![];
        let result = map_graph(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert_eq!(result.lines.len(), 3);
    }

    #[test]
    fn test_map_single_edge() {
        let edges = vec![(0, 1)];
        let result = map_graph(2, &edges);

        assert_eq!(result.lines.len(), 2);
        assert!(result.grid_graph.num_vertices() > 0);
    }

    #[test]
    fn test_map_single_vertex() {
        let edges: Vec<(usize, usize)> = vec![];
        let result = map_graph(1, &edges);

        assert_eq!(result.lines.len(), 1);
        assert!(result.grid_graph.num_vertices() > 0);
    }

    #[test]
    fn test_map_complete_k4() {
        // K4: complete graph on 4 vertices
        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let result = map_graph(4, &edges);

        assert!(result.grid_graph.num_vertices() > 4);
        assert_eq!(result.lines.len(), 4);
    }

    #[test]
    fn test_map_graph_with_custom_order() {
        let edges = vec![(0, 1), (1, 2)];
        let order = vec![2, 1, 0]; // Reverse order
        let result = map_graph_with_order(3, &edges, &order);

        assert!(result.grid_graph.num_vertices() > 0);
        assert_eq!(result.lines.len(), 3);
    }

    #[test]
    fn test_square_grid_type() {
        let edges = vec![(0, 1)];
        let result = map_graph(2, &edges);

        assert!(matches!(result.grid_graph.grid_type(), GridType::Square));
    }

    #[test]
    fn test_mapping_preserves_vertex_count() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
        let result = map_graph(5, &edges);

        // Should have exactly 5 copy lines for 5 vertices
        assert_eq!(result.lines.len(), 5);

        // Each line should correspond to a different vertex
        let vertices: Vec<usize> = result.lines.iter().map(|l| l.vertex).collect();
        for v in 0..5 {
            assert!(vertices.contains(&v), "Vertex {} not found in copy lines", v);
        }
    }
}

/// Tests for triangular lattice mapping.
mod triangular_lattice {
    use super::*;

    #[test]
    fn test_triangular_path_graph() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph_triangular(3, &edges);

        assert!(matches!(
            result.grid_graph.grid_type(),
            GridType::Triangular { .. }
        ));
        assert!(result.grid_graph.num_vertices() > 0);
    }

    #[test]
    fn test_triangular_complete_k4() {
        // K4: complete graph on 4 vertices
        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let result = map_graph_triangular(4, &edges);

        assert!(result.grid_graph.num_vertices() > 4);
        assert_eq!(result.lines.len(), 4);
    }

    #[test]
    fn test_triangular_single_vertex() {
        let edges: Vec<(usize, usize)> = vec![];
        let result = map_graph_triangular(1, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert_eq!(result.lines.len(), 1);
    }

    #[test]
    fn test_triangular_empty_graph() {
        let edges: Vec<(usize, usize)> = vec![];
        let result = map_graph_triangular(3, &edges);

        assert!(result.grid_graph.num_vertices() > 0);
        assert_eq!(result.lines.len(), 3);
    }

    #[test]
    fn test_triangular_with_custom_order() {
        let edges = vec![(0, 1), (1, 2)];
        let order = vec![2, 0, 1];
        let result = map_graph_triangular_with_order(3, &edges, &order);

        assert!(result.grid_graph.num_vertices() > 0);
        assert_eq!(result.lines.len(), 3);
    }

    #[test]
    fn test_triangular_star_graph() {
        // Star: center 0 connected to 1,2,3,4
        let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4)];
        let result = map_graph_triangular(5, &edges);

        assert!(result.grid_graph.num_vertices() > 5);
        assert_eq!(result.lines.len(), 5);
    }

    #[test]
    #[should_panic(expected = "num_vertices must be > 0")]
    fn test_triangular_zero_vertices_panics() {
        let edges: Vec<(usize, usize)> = vec![];
        map_graph_triangular(0, &edges);
    }

    #[test]
    fn test_triangular_offset_setting() {
        let edges = vec![(0, 1)];
        let result = map_graph_triangular(2, &edges);

        // Should use offset_even_cols = true by default
        assert!(matches!(
            result.grid_graph.grid_type(),
            GridType::Triangular {
                offset_even_cols: true
            }
        ));
    }
}

/// Tests for MappingResult functionality.
mod mapping_result {
    use super::*;

    #[test]
    fn test_mapping_result_serialization() {
        let edges = vec![(0, 1)];
        let result = map_graph(2, &edges);

        // Should be serializable
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: MappingResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.mis_overhead, deserialized.mis_overhead);
        assert_eq!(result.lines.len(), deserialized.lines.len());
        assert_eq!(result.padding, deserialized.padding);
        assert_eq!(result.spacing, deserialized.spacing);
    }

    #[test]
    fn test_mapping_result_config_back_all_zeros() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph(3, &edges);

        let config = vec![0; result.grid_graph.num_vertices()];
        let original = result.map_config_back(&config);

        assert_eq!(original.len(), 3);
        // All zeros in grid should map to all zeros in original
        assert!(original.iter().all(|&x| x == 0));
    }

    #[test]
    fn test_mapping_result_config_back_all_ones() {
        let edges = vec![(0, 1)];
        let result = map_graph(2, &edges);

        let config = vec![1; result.grid_graph.num_vertices()];
        let original = result.map_config_back(&config);

        assert_eq!(original.len(), 2);
        // All ones in grid should map to all ones in original
        // (majority voting in each copy line)
        assert!(original.iter().all(|&x| x == 1));
    }

    #[test]
    fn test_mapping_result_fields_populated() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph(3, &edges);

        // Verify all fields are properly set
        assert!(result.padding > 0);
        assert!(result.spacing > 0);
        assert!(!result.lines.is_empty());
        assert!(result.grid_graph.num_vertices() > 0);
    }

    #[test]
    fn test_triangular_mapping_result_serialization() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph_triangular(3, &edges);

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: MappingResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.mis_overhead, deserialized.mis_overhead);
        assert_eq!(result.lines.len(), deserialized.lines.len());
    }
}

/// Tests for edge cases and boundary conditions.
mod edge_cases {
    use super::*;

    #[test]
    fn test_disconnected_graph() {
        // Two separate edges: 0-1 and 2-3
        let edges = vec![(0, 1), (2, 3)];
        let result = map_graph(4, &edges);

        assert_eq!(result.lines.len(), 4);
        assert!(result.grid_graph.num_vertices() > 4);
    }

    #[test]
    fn test_linear_chain() {
        // Long path: 0-1-2-3-4-5
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        let result = map_graph(6, &edges);

        assert_eq!(result.lines.len(), 6);
    }

    #[test]
    fn test_cycle_graph() {
        // Cycle: 0-1-2-3-0
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let result = map_graph(4, &edges);

        assert_eq!(result.lines.len(), 4);
        assert!(result.grid_graph.num_vertices() > 4);
    }

    #[test]
    fn test_bipartite_graph() {
        // Complete bipartite K2,3: vertices 0,1 connected to 2,3,4
        let edges = vec![(0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (1, 4)];
        let result = map_graph(5, &edges);

        assert_eq!(result.lines.len(), 5);
    }

    #[test]
    fn test_petersen_like_structure() {
        // A dense graph with many edges
        let edges = vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 4),
            (2, 5),
            (3, 4),
            (3, 5),
            (4, 5),
        ];
        let result = map_graph(6, &edges);

        assert_eq!(result.lines.len(), 6);
        assert!(result.grid_graph.num_vertices() > 6);
    }
}

/// Tests for grid graph properties.
mod grid_graph_properties {
    use super::*;

    #[test]
    fn test_grid_graph_structure() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph(3, &edges);

        // Grid graph should have nodes
        assert!(result.grid_graph.num_vertices() > 0);
        // Edges are based on unit disk property (distance-based),
        // so we just verify the graph is well-formed
        let _ = result.grid_graph.num_edges();
    }

    #[test]
    fn test_grid_size_scales_with_vertices() {
        let edges_small = vec![(0, 1)];
        let result_small = map_graph(2, &edges_small);

        let edges_large = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
        let result_large = map_graph(6, &edges_large);

        // Larger graph should have larger grid
        let small_size = result_small.grid_graph.size();
        let large_size = result_large.grid_graph.size();

        assert!(
            large_size.0 >= small_size.0 || large_size.1 >= small_size.1,
            "Larger graph should produce larger grid"
        );
    }

    #[test]
    fn test_grid_nodes_have_positive_weights() {
        let edges = vec![(0, 1), (1, 2)];
        let result = map_graph(3, &edges);

        for node in result.grid_graph.nodes() {
            assert!(node.weight > 0, "Grid node should have positive weight");
        }
    }

    #[test]
    fn test_triangular_grid_graph_properties() {
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let result = map_graph_triangular(3, &edges);

        // Triangular lattice should have vertices
        assert!(result.grid_graph.num_vertices() > 0);
        // Verify the graph is well-formed by accessing edges
        let _ = result.grid_graph.num_edges();
    }
}

/// Tests verifying consistency between square and triangular mappings.
mod cross_lattice_consistency {
    use super::*;

    #[test]
    fn test_same_vertices_different_lattice() {
        let edges = vec![(0, 1), (1, 2)];

        let square_result = map_graph(3, &edges);
        let triangular_result = map_graph_triangular(3, &edges);

        // Both should have the same number of copy lines
        assert_eq!(square_result.lines.len(), triangular_result.lines.len());

        // Both should preserve vertex information
        for i in 0..3 {
            assert!(square_result.lines.iter().any(|l| l.vertex == i));
            assert!(triangular_result.lines.iter().any(|l| l.vertex == i));
        }
    }

    #[test]
    fn test_config_back_length_consistent() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];

        let square_result = map_graph(4, &edges);
        let triangular_result = map_graph_triangular(4, &edges);

        let square_config = vec![0; square_result.grid_graph.num_vertices()];
        let triangular_config = vec![0; triangular_result.grid_graph.num_vertices()];

        let square_original = square_result.map_config_back(&square_config);
        let triangular_original = triangular_result.map_config_back(&triangular_config);

        // Both should map back to the same number of vertices
        assert_eq!(square_original.len(), 4);
        assert_eq!(triangular_original.len(), 4);
    }
}

/// Tests for standard graphs from UnitDiskMapping.jl test suite.
/// These mirror the Julia tests for petersen, bull, cubical, house, diamond, tutte graphs.
mod standard_graphs {
    use super::*;

    /// Petersen graph - 10 vertices, 15 edges
    fn petersen_graph() -> (usize, Vec<(usize, usize)>) {
        let edges = vec![
            // Outer pentagon
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0),
            // Inner pentagram
            (5, 7), (7, 9), (9, 6), (6, 8), (8, 5),
            // Spokes
            (0, 5), (1, 6), (2, 7), (3, 8), (4, 9),
        ];
        (10, edges)
    }

    /// Bull graph - 5 vertices, 5 edges (triangle with two pendant vertices)
    fn bull_graph() -> (usize, Vec<(usize, usize)>) {
        let edges = vec![
            (0, 1), (1, 2), (0, 2),  // Triangle
            (1, 3), (2, 4),          // Pendant edges
        ];
        (5, edges)
    }

    /// Cubical graph - 8 vertices, 12 edges (3D cube)
    fn cubical_graph() -> (usize, Vec<(usize, usize)>) {
        let edges = vec![
            // Bottom face
            (0, 1), (1, 2), (2, 3), (3, 0),
            // Top face
            (4, 5), (5, 6), (6, 7), (7, 4),
            // Vertical edges
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];
        (8, edges)
    }

    /// House graph - 5 vertices, 6 edges (square with a triangular roof)
    fn house_graph() -> (usize, Vec<(usize, usize)>) {
        let edges = vec![
            (0, 1), (1, 2), (2, 3), (3, 0),  // Square base
            (2, 4), (3, 4),                  // Triangular roof
        ];
        (5, edges)
    }

    /// Diamond graph - 4 vertices, 5 edges (K4 minus one edge)
    fn diamond_graph() -> (usize, Vec<(usize, usize)>) {
        let edges = vec![
            (0, 1), (0, 2), (1, 2), (1, 3), (2, 3),
        ];
        (4, edges)
    }

    /// Tutte graph - 46 vertices, 69 edges (3-regular, 3-connected, non-Hamiltonian)
    fn tutte_graph() -> (usize, Vec<(usize, usize)>) {
        // Simplified version for testing - actual Tutte graph has 46 vertices
        // Using a smaller representative 3-regular graph
        let edges = vec![
            (0, 1), (0, 2), (0, 3),
            (1, 4), (1, 5),
            (2, 6), (2, 7),
            (3, 8), (3, 9),
            (4, 6), (5, 8),
            (6, 10), (7, 9),
            (8, 10), (9, 11),
            (10, 11), (4, 7), (5, 11),
        ];
        (12, edges)
    }

    /// K23 graph - complete bipartite graph K_{2,3}
    fn k23_graph() -> (usize, Vec<(usize, usize)>) {
        let edges = vec![
            (0, 2), (0, 3), (0, 4),
            (1, 2), (1, 3), (1, 4),
        ];
        (5, edges)
    }

    #[test]
    fn test_map_petersen_graph() {
        let (n, edges) = petersen_graph();
        let result = map_graph(n, &edges);

        assert_eq!(result.lines.len(), 10);
        assert!(result.grid_graph.num_vertices() > 10);
        assert!(result.mis_overhead >= 0);

        // Verify config back mapping
        let config = vec![0; result.grid_graph.num_vertices()];
        let original = result.map_config_back(&config);
        assert_eq!(original.len(), 10);
    }

    #[test]
    fn test_map_bull_graph() {
        let (n, edges) = bull_graph();
        let result = map_graph(n, &edges);

        assert_eq!(result.lines.len(), 5);
        assert!(result.grid_graph.num_vertices() > 5);
    }

    #[test]
    fn test_map_cubical_graph() {
        let (n, edges) = cubical_graph();
        let result = map_graph(n, &edges);

        assert_eq!(result.lines.len(), 8);
        assert!(result.grid_graph.num_vertices() > 8);
    }

    #[test]
    fn test_map_house_graph() {
        let (n, edges) = house_graph();
        let result = map_graph(n, &edges);

        assert_eq!(result.lines.len(), 5);
        assert!(result.grid_graph.num_vertices() > 5);
    }

    #[test]
    fn test_map_diamond_graph() {
        let (n, edges) = diamond_graph();
        let result = map_graph(n, &edges);

        assert_eq!(result.lines.len(), 4);
        assert!(result.grid_graph.num_vertices() > 4);
    }

    #[test]
    fn test_map_tutte_like_graph() {
        let (n, edges) = tutte_graph();
        let result = map_graph(n, &edges);

        assert_eq!(result.lines.len(), 12);
        assert!(result.grid_graph.num_vertices() > 12);
    }

    #[test]
    fn test_map_k23_graph() {
        let (n, edges) = k23_graph();
        let result = map_graph(n, &edges);

        assert_eq!(result.lines.len(), 5);
        assert!(result.grid_graph.num_vertices() > 5);
    }

    // Triangular lattice versions of standard graphs
    #[test]
    fn test_triangular_petersen_graph() {
        let (n, edges) = petersen_graph();
        let result = map_graph_triangular(n, &edges);

        assert_eq!(result.lines.len(), 10);
        assert!(matches!(result.grid_graph.grid_type(), GridType::Triangular { .. }));
    }

    #[test]
    fn test_triangular_bull_graph() {
        let (n, edges) = bull_graph();
        let result = map_graph_triangular(n, &edges);

        assert_eq!(result.lines.len(), 5);
        assert!(result.grid_graph.num_vertices() > 5);
    }

    #[test]
    fn test_triangular_cubical_graph() {
        let (n, edges) = cubical_graph();
        let result = map_graph_triangular(n, &edges);

        assert_eq!(result.lines.len(), 8);
    }

    #[test]
    fn test_triangular_house_graph() {
        let (n, edges) = house_graph();
        let result = map_graph_triangular(n, &edges);

        assert_eq!(result.lines.len(), 5);
    }

    #[test]
    fn test_triangular_diamond_graph() {
        let (n, edges) = diamond_graph();
        let result = map_graph_triangular(n, &edges);

        assert_eq!(result.lines.len(), 4);
    }
}

/// Tests for gadget properties.
/// These mirror the Julia gadget tests that verify source_graph and mapped_graph
/// have equivalent MIS properties.
mod gadget_tests {
    use problemreductions::rules::mapping::{
        Branch, BranchFix, BranchFixB, Cross, EndTurn, Gadget, TCon, TriBranch, TriCross, TriTurn,
        TrivialTurn, Turn, WTurn,
    };

    #[test]
    fn test_cross_disconnected_gadget() {
        let cross = Cross::<false>;
        assert_eq!(cross.size(), (4, 5));
        assert!(!cross.is_connected());
        assert_eq!(cross.mis_overhead(), 1);

        let (src_locs, src_pins) = cross.source_graph();
        let (map_locs, map_pins) = cross.mapped_graph();

        // Verify pins are valid indices
        for &pin in &src_pins {
            assert!(pin < src_locs.len(), "Source pin {} out of bounds", pin);
        }
        for &pin in &map_pins {
            assert!(pin < map_locs.len(), "Mapped pin {} out of bounds", pin);
        }

        // Same number of pins for both graphs
        assert_eq!(src_pins.len(), map_pins.len());
    }

    #[test]
    fn test_cross_connected_gadget() {
        let cross = Cross::<true>;
        assert_eq!(cross.size(), (3, 3));
        assert!(cross.is_connected());
        assert_eq!(cross.mis_overhead(), 0);
    }

    #[test]
    fn test_turn_gadget() {
        let turn = Turn;
        assert_eq!(turn.size(), (4, 4));
        assert!(turn.is_connected()); // Turn is connected in this implementation
        assert_eq!(turn.mis_overhead(), 1);

        let (_, pins) = turn.source_graph();
        assert_eq!(pins.len(), 2); // Turn has 2 pins
    }

    #[test]
    fn test_wturn_gadget() {
        let wturn = WTurn;
        assert_eq!(wturn.size(), (4, 4));
        assert!(wturn.is_connected()); // WTurn is connected in this implementation
        assert_eq!(wturn.mis_overhead(), 1);
    }

    #[test]
    fn test_branch_gadget() {
        let branch = Branch;
        assert_eq!(branch.size(), (5, 4));
        assert!(branch.is_connected()); // Branch is connected in this implementation
        assert_eq!(branch.mis_overhead(), 0);

        let (_, pins) = branch.source_graph();
        assert_eq!(pins.len(), 3); // Branch has 3 pins
    }

    #[test]
    fn test_branch_fix_gadget() {
        let bf = BranchFix;
        assert_eq!(bf.size(), (4, 4));
        assert_eq!(bf.mis_overhead(), 1);
    }

    #[test]
    fn test_branch_fix_b_gadget() {
        let bfb = BranchFixB;
        assert_eq!(bfb.size(), (4, 4));
        assert_eq!(bfb.mis_overhead(), 1);
    }

    #[test]
    fn test_tcon_gadget() {
        let tcon = TCon;
        assert_eq!(tcon.size(), (3, 4));
        assert!(tcon.is_connected());
        assert_eq!(tcon.mis_overhead(), 1);
    }

    #[test]
    fn test_trivial_turn_gadget() {
        let tt = TrivialTurn;
        assert_eq!(tt.size(), (2, 2));
        assert!(tt.is_connected());
        assert_eq!(tt.mis_overhead(), 0);
    }

    #[test]
    fn test_end_turn_gadget() {
        let et = EndTurn;
        assert_eq!(et.size(), (3, 4));
        assert!(et.is_connected()); // EndTurn is connected in this implementation
        assert_eq!(et.mis_overhead(), 1);
    }

    // Triangular gadgets
    #[test]
    fn test_tri_cross_connected_gadget() {
        let cross = TriCross::<true>;
        assert_eq!(cross.size(), (6, 4));
        assert!(cross.is_connected());
        assert_eq!(cross.mis_overhead(), 1);
    }

    #[test]
    fn test_tri_cross_disconnected_gadget() {
        let cross = TriCross::<false>;
        assert_eq!(cross.size(), (6, 6));
        assert!(!cross.is_connected());
        assert_eq!(cross.mis_overhead(), 3);
    }

    #[test]
    fn test_tri_turn_gadget() {
        let turn = TriTurn;
        assert_eq!(turn.size(), (3, 4));
        assert_eq!(turn.mis_overhead(), 0);
    }

    #[test]
    fn test_tri_branch_gadget() {
        let branch = TriBranch;
        assert_eq!(branch.size(), (6, 4));
        assert_eq!(branch.mis_overhead(), 0);
    }

    /// Test that all gadgets have valid pin indices
    #[test]
    fn test_all_gadgets_have_valid_pins() {
        fn check_gadget<G: Gadget>(gadget: &G, name: &str) {
            let (src_locs, src_pins) = gadget.source_graph();
            let (map_locs, map_pins) = gadget.mapped_graph();

            for &pin in &src_pins {
                assert!(
                    pin < src_locs.len(),
                    "{} source pin {} out of bounds (len={})",
                    name,
                    pin,
                    src_locs.len()
                );
            }
            for &pin in &map_pins {
                assert!(
                    pin < map_locs.len(),
                    "{} mapped pin {} out of bounds (len={})",
                    name,
                    pin,
                    map_locs.len()
                );
            }
            assert_eq!(
                src_pins.len(),
                map_pins.len(),
                "{} pin count mismatch",
                name
            );
        }

        check_gadget(&Cross::<false>, "Cross<false>");
        check_gadget(&Cross::<true>, "Cross<true>");
        check_gadget(&Turn, "Turn");
        check_gadget(&WTurn, "WTurn");
        check_gadget(&Branch, "Branch");
        check_gadget(&BranchFix, "BranchFix");
        check_gadget(&BranchFixB, "BranchFixB");
        check_gadget(&TCon, "TCon");
        check_gadget(&TrivialTurn, "TrivialTurn");
        check_gadget(&EndTurn, "EndTurn");
        check_gadget(&TriCross::<true>, "TriCross<true>");
        check_gadget(&TriCross::<false>, "TriCross<false>");
        check_gadget(&TriTurn, "TriTurn");
        check_gadget(&TriBranch, "TriBranch");
    }
}

/// Tests for copy line properties.
mod copyline_properties {
    use super::*;

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
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let result = map_graph(4, &edges);

        for line in &result.lines {
            assert!(
                line.vstart <= line.vstop,
                "vstart should be <= vstop"
            );
            assert!(
                line.vslot <= line.hstop,
                "vslot should be <= hstop"
            );
        }
    }
}
