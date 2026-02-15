//! Tests for copyline functionality (src/rules/mapping/copyline.rs).

use super::common::solve_weighted_mis;
use crate::rules::unitdiskmapping::{
    create_copylines, ksg, mis_overhead_copyline, triangular, CopyLine,
};

// === Edge Case Tests ===

#[test]
fn test_create_copylines_empty_graph() {
    // Test with no edges
    let edges: Vec<(usize, usize)> = vec![];
    let order = vec![0, 1, 2];
    let copylines = create_copylines(3, &edges, &order);

    assert_eq!(copylines.len(), 3);
}

#[test]
fn test_create_copylines_single_vertex() {
    let edges: Vec<(usize, usize)> = vec![];
    let order = vec![0];
    let copylines = create_copylines(1, &edges, &order);

    assert_eq!(copylines.len(), 1);
}

#[test]
fn test_mis_overhead_copyline_basic() {
    let line = CopyLine::new(0, 2, 3, 1, 3, 4);
    let _overhead = mis_overhead_copyline(&line, 4, 2);

    // Function should not panic for valid inputs
}

#[test]
fn test_mis_overhead_copyline_zero_hstop() {
    // Test edge case with minimal hstop
    let line = CopyLine::new(0, 1, 1, 1, 1, 1);
    let _overhead = mis_overhead_copyline(&line, 4, 2);

    // Function should not panic for edge case
}

#[test]
fn test_copylines_have_valid_vertex_ids() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = ksg::map_unweighted(3, &edges);

    for line in &result.lines {
        assert!(line.vertex < 3, "Vertex ID should be in range");
    }
}

#[test]
fn test_copylines_have_positive_slots() {
    let edges = vec![(0, 1), (1, 2)];
    let result = ksg::map_unweighted(3, &edges);

    for line in &result.lines {
        assert!(line.vslot > 0, "vslot should be positive");
        assert!(line.hslot > 0, "hslot should be positive");
    }
}

#[test]
fn test_copylines_have_valid_ranges() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = ksg::map_unweighted(3, &edges);

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
    // Rust 0-indexed: row = 4 * (3-1) + 1 + 2 - 1 = 10
    // Rust 0-indexed: col = 4 * (2-1) + 1 + 1 - 1 = 5
    assert_eq!(row, 10);
    assert_eq!(col, 5);
}

#[test]
fn test_copyline_center_location_offset() {
    // Test with different padding and spacing
    let line = CopyLine::new(0, 1, 1, 1, 1, 2);
    let (row, col) = line.center_location(2, 4);
    // Rust 0-indexed: row = 4 * (1-1) + 2 + 2 - 1 = 3
    // Rust 0-indexed: col = 4 * (1-1) + 2 + 1 - 1 = 2
    assert_eq!(row, 3);
    assert_eq!(col, 2);
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
fn test_copyline_copyline_locations() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let locs = line.copyline_locations(2, 4);

    assert!(!locs.is_empty());

    // Dense locations should have more nodes than sparse
    let sparse_locs = line.locations(2, 4);
    assert!(
        locs.len() >= sparse_locs.len(),
        "Dense should have at least as many nodes as sparse"
    );
}

#[test]
fn test_copyline_copyline_locations_triangular() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let locs = line.copyline_locations_triangular(2, 6);

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
    let result = ksg::map_unweighted(3, &edges);

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
    let result = triangular::map_weighted(3, &edges);

    assert_eq!(result.lines.len(), 3);
}

#[test]
fn test_copyline_vslot_hslot_ordering() {
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let result = ksg::map_unweighted(3, &edges);

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
    let result = ksg::map_unweighted(2, &edges);

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
fn test_copyline_copyline_locations_structure() {
    let line = CopyLine::new(0, 2, 3, 1, 3, 5);
    let dense = line.copyline_locations(2, 4);

    // Dense locations should have multiple nodes
    assert!(dense.len() > 1, "Dense should have multiple nodes");

    // Check weights follow pattern (ends are 1, middle can be 2)
    let weights: Vec<usize> = dense.iter().map(|&(_, _, w)| w).collect();
    assert!(weights.iter().all(|&w| w == 1 || w == 2));
}

#[test]
fn test_copyline_triangular_spacing() {
    let edges = vec![(0, 1), (1, 2)];
    let result = triangular::map_weighted(3, &edges);

    // Triangular uses spacing=6
    assert_eq!(result.spacing, 6);

    // Each copyline should produce valid triangular locations
    for line in &result.lines {
        let locs = line.copyline_locations_triangular(result.padding, result.spacing);
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

/// Test that weighted MIS of copyline graph equals mis_overhead_copyline.
/// This matches Julia's weighted.jl "copy lines" testset.
///
/// Julia's weighted formula for mis_overhead_copyline(Weighted(), line):
///   (hslot - vstart) * spacing +
///   (vstop - hslot) * spacing +
///   max((hstop - vslot) * spacing - 2, 0)
///
/// Note: The degenerate case (5, 5, 5) where vstart=hslot=vstop and hstop=vslot
/// is excluded because Julia's center weight is 0 while Rust's is min 1.
#[test]
fn test_copyline_weighted_mis_equals_overhead() {
    // Test cases: (vstart, vstop, hstop) as i32 for arithmetic
    // Note: Excluding (5, 5, 5) which is degenerate - only center node with
    // Julia weight=0 vs Rust weight=1 (Rust uses nline.max(1) for center)
    let test_cases: [(i32, i32, i32); 7] = [
        (3, 7, 8),
        (3, 5, 8),
        (5, 9, 8),
        (5, 5, 8),
        (1, 7, 5),
        (5, 8, 5),
        (1, 5, 5),
    ];

    let padding: usize = 2;
    let spacing: i32 = 4;

    for (vstart, vstop, hstop) in test_cases {
        // Create copyline with vslot=5, hslot=5 (matching Julia's test)
        let line = CopyLine::new(0, 5, 5, vstart as usize, vstop as usize, hstop as usize);

        // Get copyline locations with weights
        let locs = line.copyline_locations(padding, spacing as usize);
        let n = locs.len();

        // Build graph matching Julia's weighted.jl:
        // Julia loop: for i=1:length(locs)-1
        //   if i==1 || locs[i-1].weight == 1  # starting point
        //     add_edge!(g, length(locs), i)
        //   else
        //     add_edge!(g, i, i-1)
        //
        // Converting to 0-indexed Rust:
        // Julia i=1..n-1 becomes Rust i=0..n-2
        // Julia locs[i-1] at Julia i becomes locs[i-2] in 0-indexed when julia_i > 1
        // Julia add_edge!(g, length(locs), i) = edge(n-1, i-1) in Rust
        // Julia add_edge!(g, i, i-1) = edge(i-1, i-2) in Rust
        let mut edges = Vec::new();
        for julia_i in 1..n {
            // julia_i represents Julia's 1-indexed i value
            let is_start_point = if julia_i == 1 {
                true // First iteration always connects to last node
            } else {
                // Julia's locs[i-1] when julia_i>1 is locs[julia_i-2] in 0-indexed Rust
                locs[julia_i - 2].2 == 1
            };

            if is_start_point {
                // Julia's add_edge!(g, length(locs), i) connects last node to current
                // In 0-indexed: edge between (n-1) and (julia_i-1)
                edges.push((n - 1, julia_i - 1));
            } else {
                // Julia's add_edge!(g, i, i-1) connects current to previous
                // In 0-indexed: edge between (julia_i-1) and (julia_i-2)
                edges.push((julia_i - 1, julia_i - 2));
            }
        }

        let weights: Vec<i32> = locs.iter().map(|&(_, _, w)| w as i32).collect();

        // Solve weighted MIS
        let weighted_mis = solve_weighted_mis(n, &edges, &weights);

        // Calculate expected value using Julia's weighted formula:
        // mis_overhead_copyline(Weighted(), line) =
        //   (hslot - vstart) * s + (vstop - hslot) * s + max((hstop - vslot) * s - 2, 0)
        let hslot: i32 = 5;
        let vslot: i32 = 5;
        let expected = (hslot - vstart) * spacing
            + (vstop - hslot) * spacing
            + std::cmp::max((hstop - vslot) * spacing - 2, 0);

        assert_eq!(
            weighted_mis, expected,
            "Copyline vstart={}, vstop={}, hstop={}: weighted MIS {} should equal overhead {}",
            vstart, vstop, hstop, weighted_mis, expected
        );
    }
}
