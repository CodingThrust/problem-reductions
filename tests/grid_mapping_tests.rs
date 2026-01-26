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
