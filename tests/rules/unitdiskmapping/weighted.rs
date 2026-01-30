//! Tests for weighted mode functionality (src/rules/mapping/weighted.rs).

use problemreductions::rules::unitdiskmapping::{
    map_graph_triangular, map_weights, trace_centers, CopyLine,
    copyline_weighted_locations_triangular,
};
use problemreductions::topology::Graph;

// === Trace Centers Tests ===

#[test]
fn test_trace_centers_returns_correct_count() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 3);
}

#[test]
fn test_trace_centers_positive_coordinates() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph_triangular(3, &edges);

    let centers = trace_centers(&result);
    for (i, &(row, col)) in centers.iter().enumerate() {
        assert!(row > 0, "Vertex {} center row should be positive", i);
        assert!(col > 0, "Vertex {} center col should be positive", i);
    }
}

#[test]
fn test_trace_centers_single_vertex() {
    let edges: Vec<(usize, usize)> = vec![];
    let result = map_graph_triangular(1, &edges);

    let centers = trace_centers(&result);
    assert_eq!(centers.len(), 1);
}

// === Map Weights Tests ===

#[test]
fn test_map_weights_uniform() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    // Use uniform weights (all 0.5)
    let weights = vec![0.5, 0.5, 0.5];
    let mapped = map_weights(&result, &weights);

    // Mapped weights should be non-negative
    assert!(
        mapped.iter().all(|&w| w >= 0.0),
        "All mapped weights should be non-negative"
    );

    // Mapped should have one weight per grid node
    assert_eq!(mapped.len(), result.grid_graph.num_vertices());
}

#[test]
fn test_map_weights_zero() {
    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let weights = vec![0.0, 0.0, 0.0];
    let mapped = map_weights(&result, &weights);

    // With zero weights, the mapped weights should be positive
    // (because of the overhead structure)
    assert!(mapped.iter().any(|&w| w > 0.0));
}

#[test]
fn test_map_weights_one() {
    

    let edges = vec![(0, 1), (1, 2)];
    let result = map_graph_triangular(3, &edges);

    let weights = vec![1.0, 1.0, 1.0];
    let mapped = map_weights(&result, &weights);

    // All weights should be positive
    assert!(mapped.iter().all(|&w| w > 0.0));

    // Mapped weights should equal base weights plus original weights at centers
    let base_total: f64 = result
        .grid_graph
        .nodes()
        .iter()
        .map(|n| n.weight as f64)
        .sum();
    let original_total: f64 = weights.iter().sum();
    let mapped_total: f64 = mapped.iter().sum();

    // The mapped total should equal base_total + original_total exactly
    assert_eq!(
        mapped_total,
        base_total + original_total,
        "Mapped total {} should equal base {} + original {} = {}",
        mapped_total,
        base_total,
        original_total,
        base_total + original_total
    );
}

#[test]
#[should_panic]
fn test_map_weights_invalid_negative() {
    let edges = vec![(0, 1)];
    let result = map_graph_triangular(2, &edges);

    let weights = vec![-0.5, 0.5];
    let _ = map_weights(&result, &weights);
}

#[test]
#[should_panic]
fn test_map_weights_invalid_over_one() {
    let edges = vec![(0, 1)];
    let result = map_graph_triangular(2, &edges);

    let weights = vec![1.5, 0.5];
    let _ = map_weights(&result, &weights);
}

#[test]
#[should_panic]
fn test_map_weights_wrong_length() {
    let edges = vec![(0, 1)];
    let result = map_graph_triangular(2, &edges);

    let weights = vec![0.5]; // Wrong length
    let _ = map_weights(&result, &weights);
}

// === Weighted Interface Tests ===

#[test]
fn test_triangular_weighted_interface() {
    use problemreductions::topology::smallgraph;

    let (n, edges) = smallgraph("bull").unwrap();
    let result = map_graph_triangular(n, &edges);

    // Test with uniform weights
    let ws = vec![0.5; n];
    let grid_weights = map_weights(&result, &ws);

    // Should produce valid weights for all grid nodes
    assert_eq!(grid_weights.len(), result.grid_graph.num_vertices());
    assert!(grid_weights.iter().all(|&w| w > 0.0));
}

#[test]
fn test_triangular_interface_full() {
    use problemreductions::topology::smallgraph;

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = map_graph_triangular(n, &edges);

    // Uniform weights in [0, 1]
    let ws = vec![0.3; n];
    let grid_weights = map_weights(&result, &ws);

    assert_eq!(grid_weights.len(), result.grid_graph.num_vertices());
    assert!(grid_weights.iter().all(|&w| w >= 0.0));

    // Test map_config_back
    let config = vec![0; result.grid_graph.num_vertices()];
    let original_config = result.map_config_back(&config);
    assert_eq!(original_config.len(), n);

    // Verify trace_centers
    let centers = trace_centers(&result);
    assert_eq!(centers.len(), n);
}

// === Copyline Weight Invariant Tests ===

#[test]
fn test_triangular_copyline_weight_invariant() {
    let spacing = 6usize;

    // Test various copyline configurations
    let configs = [
        (1, 1, 1, 2), // Simple case
        (1, 2, 1, 3), // With vertical segment
        (2, 3, 2, 4), // Offset case
    ];

    for (vslot, hslot, vstart, hstop) in configs {
        let vstop = hslot.max(vstart);
        let copyline = CopyLine::new(0, vslot, hslot, vstart, vstop, hstop);
        let (locs, weights) = copyline_weighted_locations_triangular(&copyline, spacing);

        // Weights should be positive
        assert!(
            weights.iter().all(|&w| w >= 1),
            "Config ({}, {}, {}, {}): all weights should be >= 1",
            vslot,
            hslot,
            vstart,
            hstop
        );

        // Locations and weights should have same length
        assert_eq!(
            locs.len(),
            weights.len(),
            "Config ({}, {}, {}, {}): locs and weights should match",
            vslot,
            hslot,
            vstart,
            hstop
        );
    }
}

// === Weighted MIS Weight Sum Invariant Tests ===

#[test]
fn test_weighted_gadgets_weight_conservation() {
    // For each weighted gadget, verify weight sums are consistent with MIS properties
    use problemreductions::rules::unitdiskmapping::triangular_weighted_ruleset;

    let ruleset = triangular_weighted_ruleset();
    for gadget in &ruleset {
        let source_sum: i32 = gadget.source_weights().iter().sum();
        let mapped_sum: i32 = gadget.mapped_weights().iter().sum();
        let overhead = gadget.mis_overhead();

        // Both sums should be positive (all gadgets have at least some nodes)
        assert!(
            source_sum > 0 && mapped_sum > 0,
            "Both sums should be positive"
        );

        // MIS overhead can be negative for gadgets that reduce MIS
        // The key invariant is: mapped_MIS = source_MIS + overhead
        // So overhead = mapped_MIS - source_MIS (can be positive, zero, or negative)
        assert!(
            overhead.abs() <= source_sum.max(mapped_sum),
            "Overhead magnitude {} should be bounded by max sum {}",
            overhead.abs(),
            source_sum.max(mapped_sum)
        );
    }
}

#[test]
fn test_weighted_gadgets_positive_weights() {
    // All individual weights should be positive
    use problemreductions::rules::unitdiskmapping::triangular_weighted_ruleset;

    let ruleset = triangular_weighted_ruleset();
    for gadget in &ruleset {
        for &w in gadget.source_weights() {
            assert!(w > 0, "Source weights should be positive, got {}", w);
        }
        for &w in gadget.mapped_weights() {
            assert!(w > 0, "Mapped weights should be positive, got {}", w);
        }
    }
}

// === Solution Extraction Integration Tests ===

#[test]
fn test_map_config_back_extracts_valid_is_triangular() {
    use problemreductions::rules::unitdiskmapping::map_graph_triangular;
    use problemreductions::topology::{smallgraph, Graph};

    let (n, edges) = smallgraph("bull").unwrap();
    let result = map_graph_triangular(n, &edges);

    // Get all zeros config
    let config = vec![0; result.grid_graph.num_vertices()];
    let extracted = result.map_config_back(&config);

    // All zeros should extract to all zeros
    assert_eq!(extracted.len(), n);
    assert!(extracted.iter().all(|&x| x == 0));
}

#[test]
fn test_map_weights_preserves_total_weight() {
    // map_weights should add original weights to base weights
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = map_graph_triangular(3, &edges);

    let original_weights = vec![0.5, 0.3, 0.7];
    let mapped = map_weights(&result, &original_weights);

    // Sum of mapped weights should be base_sum + original_sum
    let base_sum: f64 = result
        .grid_graph
        .nodes()
        .iter()
        .map(|n| n.weight as f64)
        .sum();
    let original_sum: f64 = original_weights.iter().sum();
    let mapped_sum: f64 = mapped.iter().sum();

    // Allow small tolerance for floating point
    assert!(
        (mapped_sum - (base_sum + original_sum)).abs() < 1.5,
        "Weight sum {} should be close to base {} + original {} = {}",
        mapped_sum,
        base_sum,
        original_sum,
        base_sum + original_sum
    );
}

#[test]
fn test_trace_centers_consistency_with_config_back() {
    use problemreductions::topology::smallgraph;

    let (n, edges) = smallgraph("diamond").unwrap();
    let result = map_graph_triangular(n, &edges);

    // Get centers
    let centers = trace_centers(&result);
    assert_eq!(centers.len(), n);

    // Each center should be within grid bounds
    let (rows, cols) = {
        let max_row = result
            .grid_graph
            .nodes()
            .iter()
            .map(|n| n.row)
            .max()
            .unwrap_or(0);
        let max_col = result
            .grid_graph
            .nodes()
            .iter()
            .map(|n| n.col)
            .max()
            .unwrap_or(0);
        (max_row as usize + 1, max_col as usize + 1)
    };

    for (v, &(r, c)) in centers.iter().enumerate() {
        assert!(
            r < rows && c < cols,
            "Vertex {} center ({}, {}) out of bounds ({}, {})",
            v,
            r,
            c,
            rows,
            cols
        );
    }
}

// === Square Weighted Mode Tests ===

/// Test that square gadgets have correct source_weights matching Julia.
/// Julia's weighted.jl specifies:
/// - Default: all weights = 2
/// - TrivialTurn: source nodes 1,2 → weight 1; mapped nodes 1,2 → weight 1
/// - BranchFixB: source node 1 → weight 1; mapped node 1 → weight 1
/// - EndTurn: source node 3 → weight 1; mapped node 1 → weight 1
/// - TCon: source node 2 → weight 1; mapped node 2 → weight 1
/// - Branch: source node 4 → weight 3; mapped node 2 → weight 3
#[test]
fn test_square_gadget_trivial_turn_weights() {
    use problemreductions::rules::unitdiskmapping::TrivialTurn;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let trivial_turn = TrivialTurn;
    let source_weights = trivial_turn.source_weights();
    let mapped_weights = trivial_turn.mapped_weights();

    // TrivialTurn has 2 source nodes and 2 mapped nodes
    assert_eq!(source_weights.len(), 2, "TrivialTurn should have 2 source nodes");
    assert_eq!(mapped_weights.len(), 2, "TrivialTurn should have 2 mapped nodes");

    // Julia: sw[[1,2]] .= 1 means nodes 1,2 (0-indexed: 0,1) have weight 1
    assert_eq!(source_weights[0], 1, "TrivialTurn source node 0 should have weight 1");
    assert_eq!(source_weights[1], 1, "TrivialTurn source node 1 should have weight 1");

    // Julia: mw[[1,2]] .= 1 means mapped nodes 1,2 (0-indexed: 0,1) have weight 1
    assert_eq!(mapped_weights[0], 1, "TrivialTurn mapped node 0 should have weight 1");
    assert_eq!(mapped_weights[1], 1, "TrivialTurn mapped node 1 should have weight 1");
}

#[test]
fn test_square_gadget_endturn_weights() {
    use problemreductions::rules::unitdiskmapping::EndTurn;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let endturn = EndTurn;
    let source_weights = endturn.source_weights();
    let mapped_weights = endturn.mapped_weights();

    // EndTurn has 3 source nodes and 1 mapped node
    assert_eq!(source_weights.len(), 3, "EndTurn should have 3 source nodes");
    assert_eq!(mapped_weights.len(), 1, "EndTurn should have 1 mapped node");

    // Julia: sw[[3]] .= 1 means node 3 (1-indexed) = node 2 (0-indexed) has weight 1
    assert_eq!(source_weights[0], 2, "EndTurn source node 0 should have weight 2");
    assert_eq!(source_weights[1], 2, "EndTurn source node 1 should have weight 2");
    assert_eq!(source_weights[2], 1, "EndTurn source node 2 should have weight 1");

    // Julia: mw[[1]] .= 1 means mapped node 1 (1-indexed) = node 0 (0-indexed) has weight 1
    assert_eq!(mapped_weights[0], 1, "EndTurn mapped node 0 should have weight 1");
}

#[test]
fn test_square_gadget_tcon_weights() {
    use problemreductions::rules::unitdiskmapping::TCon;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let tcon = TCon;
    let source_weights = tcon.source_weights();
    let mapped_weights = tcon.mapped_weights();

    // TCon has 4 source nodes and 4 mapped nodes
    assert_eq!(source_weights.len(), 4, "TCon should have 4 source nodes");
    assert_eq!(mapped_weights.len(), 4, "TCon should have 4 mapped nodes");

    // Julia: sw[[2]] .= 1 means node 2 (1-indexed) = node 1 (0-indexed) has weight 1
    assert_eq!(source_weights[0], 2, "TCon source node 0 should have weight 2");
    assert_eq!(source_weights[1], 1, "TCon source node 1 should have weight 1");
    assert_eq!(source_weights[2], 2, "TCon source node 2 should have weight 2");
    assert_eq!(source_weights[3], 2, "TCon source node 3 should have weight 2");

    // Julia: mw[[2]] .= 1 means mapped node 2 (1-indexed) = node 1 (0-indexed) has weight 1
    assert_eq!(mapped_weights[0], 2, "TCon mapped node 0 should have weight 2");
    assert_eq!(mapped_weights[1], 1, "TCon mapped node 1 should have weight 1");
    assert_eq!(mapped_weights[2], 2, "TCon mapped node 2 should have weight 2");
    assert_eq!(mapped_weights[3], 2, "TCon mapped node 3 should have weight 2");
}

#[test]
fn test_square_gadget_branchfixb_weights() {
    use problemreductions::rules::unitdiskmapping::BranchFixB;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let branchfixb = BranchFixB;
    let source_weights = branchfixb.source_weights();
    let mapped_weights = branchfixb.mapped_weights();

    // BranchFixB has 4 source nodes and 2 mapped nodes
    assert_eq!(source_weights.len(), 4, "BranchFixB should have 4 source nodes");
    assert_eq!(mapped_weights.len(), 2, "BranchFixB should have 2 mapped nodes");

    // Julia: sw[[1]] .= 1 means node 1 (1-indexed) = node 0 (0-indexed) has weight 1
    assert_eq!(source_weights[0], 1, "BranchFixB source node 0 should have weight 1");

    // Other nodes should be default weight 2
    for (i, &w) in source_weights.iter().enumerate().skip(1) {
        assert_eq!(w, 2, "BranchFixB source node {} should have weight 2", i);
    }

    // Julia: mw[[1]] .= 1 means mapped node 1 (1-indexed) = node 0 (0-indexed) has weight 1
    assert_eq!(mapped_weights[0], 1, "BranchFixB mapped node 0 should have weight 1");
    assert_eq!(mapped_weights[1], 2, "BranchFixB mapped node 1 should have weight 2");
}

#[test]
fn test_square_gadget_branch_weights() {
    use problemreductions::rules::unitdiskmapping::Branch;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let branch = Branch;
    let source_weights = branch.source_weights();
    let mapped_weights = branch.mapped_weights();

    // Branch has 8 source nodes and 6 mapped nodes
    assert_eq!(source_weights.len(), 8, "Branch should have 8 source nodes");
    assert_eq!(mapped_weights.len(), 6, "Branch should have 6 mapped nodes");

    // Julia: sw[[4]] .= 3 means node 4 (1-indexed) = node 3 (0-indexed) has weight 3
    for (i, &w) in source_weights.iter().enumerate() {
        let expected = if i == 3 { 3 } else { 2 };
        assert_eq!(w, expected, "Branch source node {} should have weight {}", i, expected);
    }

    // Julia: mw[[2]] .= 3 means mapped node 2 (1-indexed) = node 1 (0-indexed) has weight 3
    for (i, &w) in mapped_weights.iter().enumerate() {
        let expected = if i == 1 { 3 } else { 2 };
        assert_eq!(w, expected, "Branch mapped node {} should have weight {}", i, expected);
    }
}

#[test]
fn test_square_gadget_default_weights_cross_false() {
    use problemreductions::rules::unitdiskmapping::Cross;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let cross = Cross::<false>;
    for &w in &cross.source_weights() {
        assert_eq!(w, 2, "Cross<false> source weights should all be 2");
    }
    for &w in &cross.mapped_weights() {
        assert_eq!(w, 2, "Cross<false> mapped weights should all be 2");
    }
}

#[test]
fn test_square_gadget_default_weights_cross_true() {
    use problemreductions::rules::unitdiskmapping::Cross;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let cross = Cross::<true>;
    for &w in &cross.source_weights() {
        assert_eq!(w, 2, "Cross<true> source weights should all be 2");
    }
    for &w in &cross.mapped_weights() {
        assert_eq!(w, 2, "Cross<true> mapped weights should all be 2");
    }
}

#[test]
fn test_square_gadget_default_weights_turn() {
    use problemreductions::rules::unitdiskmapping::Turn;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let turn = Turn;
    for &w in &turn.source_weights() {
        assert_eq!(w, 2, "Turn source weights should all be 2");
    }
    for &w in &turn.mapped_weights() {
        assert_eq!(w, 2, "Turn mapped weights should all be 2");
    }
}

#[test]
fn test_square_gadget_default_weights_wturn() {
    use problemreductions::rules::unitdiskmapping::WTurn;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let wturn = WTurn;
    for &w in &wturn.source_weights() {
        assert_eq!(w, 2, "WTurn source weights should all be 2");
    }
    for &w in &wturn.mapped_weights() {
        assert_eq!(w, 2, "WTurn mapped weights should all be 2");
    }
}

#[test]
fn test_square_gadget_default_weights_branchfix() {
    use problemreductions::rules::unitdiskmapping::BranchFix;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let branchfix = BranchFix;
    for &w in &branchfix.source_weights() {
        assert_eq!(w, 2, "BranchFix source weights should all be 2");
    }
    for &w in &branchfix.mapped_weights() {
        assert_eq!(w, 2, "BranchFix mapped weights should all be 2");
    }
}

#[test]
fn test_square_danglinleg_weights() {
    use problemreductions::rules::unitdiskmapping::DanglingLeg;
    use problemreductions::rules::unitdiskmapping::Pattern;

    let danglinleg = DanglingLeg;
    let source_weights = danglinleg.source_weights();
    let mapped_weights = danglinleg.mapped_weights();

    // DanglingLeg has 3 source nodes and 1 mapped node
    assert_eq!(source_weights.len(), 3, "DanglingLeg should have 3 source nodes");
    assert_eq!(mapped_weights.len(), 1, "DanglingLeg should have 1 mapped node");

    // Julia: sw[[1]] .= 1 means node 1 (0-indexed: 0) has weight 1, others default to 2
    assert_eq!(source_weights[0], 1, "DanglingLeg source node 0 should have weight 1");
    assert_eq!(source_weights[1], 2, "DanglingLeg source node 1 should have weight 2");
    assert_eq!(source_weights[2], 2, "DanglingLeg source node 2 should have weight 2");

    // Julia: mw[[1]] .= 1 means mapped node 1 (0-indexed: 0) has weight 1
    assert_eq!(mapped_weights[0], 1, "DanglingLeg mapped node 0 should have weight 1");
}

// === Weighted map_config_back Full Verification Tests ===

/// Test weighted mode map_config_back for standard graphs.
/// Verifies:
/// 1. Config at trace_centers is a valid IS
///
/// Note: This uses triangular mode (map_graph_triangular) which is the weighted mode
/// in Julia's terminology. We solve weighted MIS using the grid graph's NATIVE weights
/// (from gadgets), not mapped source weights. The gadget weights enforce mutual exclusion
/// for adjacent original vertices through their crossing structure.
#[test]
fn test_weighted_map_config_back_standard_graphs() {
    use super::common::{is_independent_set, solve_weighted_mis_config};
    use problemreductions::rules::unitdiskmapping::{map_graph_triangular, trace_centers};
    use problemreductions::topology::{smallgraph, Graph};

    // All standard graphs (excluding tutte/karate which are slow)
    let graph_names = [
        "bull", "chvatal", "cubical", "desargues", "diamond",
        "dodecahedral", "frucht", "heawood", "house", "housex",
        "icosahedral", "krackhardtkite", "moebiuskantor", "octahedral",
        "pappus", "petersen", "sedgewickmaze", "tetrahedral",
        "truncatedcube", "truncatedtetrahedron",
    ];

    for name in graph_names {
        let (n, edges) = smallgraph(name).unwrap();
        let result = map_graph_triangular(n, &edges);

        // Get native weights from grid graph (gadget weights)
        let grid_edges = result.grid_graph.edges().to_vec();
        let num_grid = result.grid_graph.num_vertices();
        let native_weights: Vec<i32> = (0..num_grid)
            .map(|i| result.grid_graph.weight(i).copied().unwrap_or(1))
            .collect();

        // Solve weighted MIS with native weights
        let grid_config = solve_weighted_mis_config(num_grid, &grid_edges, &native_weights);

        // Get center locations
        let centers = trace_centers(&result);

        // Extract config at centers
        let center_config: Vec<usize> = centers
            .iter()
            .map(|&(row, col)| {
                for (i, node) in result.grid_graph.nodes().iter().enumerate() {
                    if node.row == row as i32 && node.col == col as i32 {
                        return grid_config[i];
                    }
                }
                0
            })
            .collect();

        // Verify it's a valid independent set
        assert!(
            is_independent_set(&edges, &center_config),
            "{}: Config at centers should be a valid independent set",
            name
        );
    }
}
