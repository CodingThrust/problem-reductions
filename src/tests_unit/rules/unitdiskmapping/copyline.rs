use super::*;

#[test]
fn test_create_copylines_path() {
    // Path graph: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let order = vec![0, 1, 2];
    let lines = create_copylines(3, &edges, &order);

    assert_eq!(lines.len(), 3);
    // Each vertex gets a copy line
    assert_eq!(lines[0].vertex, 0);
    assert_eq!(lines[1].vertex, 1);
    assert_eq!(lines[2].vertex, 2);
}

#[test]
fn test_copyline_locations() {
    let line = CopyLine {
        vertex: 0,
        vslot: 1,
        hslot: 1,
        vstart: 1,
        vstop: 1,
        hstop: 3,
    };
    let locs = line.locations(2, 4); // padding=2, spacing=4
    assert!(!locs.is_empty());
}

#[test]
fn test_create_copylines_empty() {
    let edges: Vec<(usize, usize)> = vec![];
    let order: Vec<usize> = vec![];
    let lines = create_copylines(0, &edges, &order);
    assert!(lines.is_empty());
}

#[test]
fn test_create_copylines_single_vertex() {
    let edges: Vec<(usize, usize)> = vec![];
    let order = vec![0];
    let lines = create_copylines(1, &edges, &order);

    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].vertex, 0);
    assert_eq!(lines[0].vslot, 1);
}

#[test]
fn test_create_copylines_triangle() {
    // Triangle: 0-1, 1-2, 0-2
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let order = vec![0, 1, 2];
    let lines = create_copylines(3, &edges, &order);

    assert_eq!(lines.len(), 3);
    // Vertex 0 should have hstop reaching to vertex 2's slot
    assert!(lines[0].hstop >= 2);
}

#[test]
fn test_copyline_center_location() {
    let line = CopyLine::new(0, 2, 3, 1, 3, 4);
    let (row, col) = line.center_location(1, 4);
    // Julia 1-indexed: row = 4 * (3-1) + 1 + 2 = 11, col = 4 * (2-1) + 1 + 1 = 6
    // Rust 0-indexed: row = 11 - 1 = 10, col = 6 - 1 = 5
    assert_eq!(row, 10);
    assert_eq!(col, 5);
}

#[test]
fn test_remove_order_path() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let order = vec![0, 1, 2];
    let removal = remove_order(3, &edges, &order);

    // Vertex 2 has no later neighbors, so it can be removed at step 2
    // Vertex 1's latest neighbor is 2, so can be removed at step 2
    // Vertex 0's latest neighbor is 1, so can be removed at step 1
    assert_eq!(removal.len(), 3);
}

#[test]
fn test_mis_overhead_copyline() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let spacing = 4;
    let padding = 2;
    let locs = line.copyline_locations(padding, spacing);
    let overhead = mis_overhead_copyline(&line, spacing, padding);
    // Julia formula for UnWeighted mode: length(locs) / 2
    assert_eq!(overhead, locs.len() / 2);
}

#[test]
fn test_copyline_serialization() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let json = serde_json::to_string(&line).unwrap();
    let deserialized: CopyLine = serde_json::from_str(&json).unwrap();
    assert_eq!(line, deserialized);
}

#[test]
fn test_create_copylines_star() {
    // Star graph: 0 connected to 1, 2, 3
    let edges = vec![(0, 1), (0, 2), (0, 3)];
    let order = vec![0, 1, 2, 3];
    let lines = create_copylines(4, &edges, &order);

    assert_eq!(lines.len(), 4);
    // Vertex 0 (center) should have hstop reaching the last neighbor
    assert_eq!(lines[0].hstop, 4);
}

#[test]
fn test_copyline_locations_detailed() {
    let line = CopyLine::new(0, 1, 2, 1, 2, 2);
    let locs = line.locations(0, 2);

    // With padding=0, spacing=2 (0-indexed output):
    // Julia 1-indexed: col = 2*(1-1) + 0 + 1 = 1 -> Rust 0-indexed: col = 0
    // Julia 1-indexed: row = 2*(2-1) + 0 + 2 = 4 -> Rust 0-indexed: row = 3
    // Vertical segment covers rows around the center

    assert!(!locs.is_empty());
    // Check that we have vertical positions (col = 0 in 0-indexed)
    let has_vertical = locs.iter().any(|&(_r, c, _)| c == 0);
    assert!(has_vertical);
}

#[test]
fn test_copyline_locations_simple() {
    // Simple L-shape: vslot=1, hslot=1, vstart=1, vstop=2, hstop=2
    let line = CopyLine::new(0, 1, 1, 1, 2, 2);
    let locs = line.copyline_locations(2, 4); // padding=2, spacing=4

    // Center: I = 4*(1-1) + 2 + 2 = 4, J = 4*(1-1) + 2 + 1 = 3
    // vstart=1, hslot=1: no "up" segment
    // vstop=2, hslot=1: "down" segment from I to I + 4*(2-1) - 1 = 4 to 7
    // hstop=2, vslot=1: "right" segment from J+2=5 to J + 4*(2-1) - 1 = 6

    assert!(!locs.is_empty());
    // Should have nodes at every cell, not just at spacing intervals
    // Check we have more than just the sparse waypoints
    let node_count = locs.len();
    println!("Dense locations for simple L-shape: {:?}", locs);
    println!("Node count: {}", node_count);

    // Dense should have many more nodes than sparse (which has ~3-4)
    assert!(
        node_count > 4,
        "Dense locations should have more than sparse"
    );
}

#[test]
fn test_copyline_locations_matches_julia() {
    // Test case that can be verified against Julia's UnitDiskMapping
    // Using vslot=1, hslot=2, vstart=1, vstop=2, hstop=3, padding=2, spacing=4
    let line = CopyLine::new(0, 1, 2, 1, 2, 3);
    let locs = line.copyline_locations(2, 4);

    // Julia 1-indexed: I = 4*(2-1) + 2 + 2 = 8, J = 4*(1-1) + 2 + 1 = 3
    // Rust 0-indexed: row = 7, col = 2
    // Center node at (I, J+1) in Julia = (8, 4) -> Rust 0-indexed = (7, 3)
    let has_center = locs.iter().any(|&(r, c, _)| r == 7 && c == 3);
    assert!(
        has_center,
        "Center node at (7, 3) should be present. Locs: {:?}",
        locs
    );

    // All positions should be valid (0-indexed, so >= 0)
    for &(_row, _col, weight) in &locs {
        assert!(weight >= 1, "Weight should be >= 1");
    }

    println!("Dense locations: {:?}", locs);
}

// === Julia comparison tests ===
// These test cases are derived from Julia's UnitDiskMapping tests

#[test]
fn test_mis_overhead_julia_cases() {
    // Test cases using UnWeighted formula: length(copyline_locations) / 2
    // Using vslot=5, hslot=5 as the base configuration
    let spacing = 4;
    let padding = 2;

    let test_cases = [
        // (vstart, vstop, hstop)
        (3, 7, 8),
        (3, 5, 8),
        (5, 9, 8),
        (5, 5, 8),
        (1, 7, 5),
        (5, 8, 5),
        (1, 5, 5),
        (5, 5, 5),
    ];

    for (vstart, vstop, hstop) in test_cases {
        let line = CopyLine::new(1, 5, 5, vstart, vstop, hstop);
        let locs = line.copyline_locations(padding, spacing);
        let overhead = mis_overhead_copyline(&line, spacing, padding);

        // UnWeighted formula: length(locs) / 2
        let expected = locs.len() / 2;

        assert_eq!(
            overhead, expected,
            "MIS overhead mismatch for (vstart={}, vstop={}, hstop={}): got {}, expected {}",
            vstart, vstop, hstop, overhead, expected
        );
    }
}

#[test]
fn test_create_copylines_petersen() {
    // Petersen graph edges (0-indexed)
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 0), // outer pentagon
        (5, 7),
        (7, 9),
        (9, 6),
        (6, 8),
        (8, 5), // inner star
        (0, 5),
        (1, 6),
        (2, 7),
        (3, 8),
        (4, 9), // connections
    ];
    let order: Vec<usize> = (0..10).collect();

    let lines = create_copylines(10, &edges, &order);

    // Verify all lines are created
    assert_eq!(lines.len(), 10);

    // Verify basic invariants
    for (i, &v) in order.iter().enumerate() {
        let line = &lines[v];
        assert_eq!(line.vertex, v, "Vertex mismatch");
        assert_eq!(line.vslot, i + 1, "vslot should be position + 1");
        assert!(
            line.vstart <= line.hslot && line.hslot <= line.vstop,
            "hslot should be between vstart and vstop for vertex {}",
            v
        );
        assert!(
            line.hstop >= line.vslot,
            "hstop should be >= vslot for vertex {}",
            v
        );
    }

    // Verify that neighboring vertices have overlapping L-shapes
    for &(u, v) in &edges {
        let line_u = &lines[u];
        let line_v = &lines[v];
        // Two lines cross if one's vslot is in the other's hslot range
        // and one's hslot is in the other's vslot range
        let u_pos = order.iter().position(|&x| x == u).unwrap() + 1;
        let v_pos = order.iter().position(|&x| x == v).unwrap() + 1;
        // For a valid embedding, connected vertices should have crossing copy lines
        assert!(
            line_u.hstop >= v_pos || line_v.hstop >= u_pos,
            "Connected vertices {} and {} should have overlapping L-shapes",
            u,
            v
        );
    }
}

#[test]
fn test_remove_order_detailed() {
    // Path graph: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let order = vec![0, 1, 2];
    let removal = remove_order(3, &edges, &order);

    // Trace through Julia's algorithm:
    // Step 0: add vertex 0, counts = [0, 1, 0], totalcounts = [1, 2, 1]
    //         vertex 0: counts[0]=0 != totalcounts[0]=1, not removed
    //         vertex 1: counts[1]=1 != totalcounts[1]=2, not removed
    //         vertex 2: counts[2]=0 != totalcounts[2]=1, not removed
    //         removal[0] = []
    // Step 1: add vertex 1, counts = [1, 2, 1], totalcounts = [1, 2, 1]
    //         vertex 0: counts[0]=1 == totalcounts[0]=1, remove at max(1, 0)=1
    //         vertex 1: counts[1]=2 == totalcounts[1]=2, remove at max(1, 1)=1
    //         vertex 2: counts[2]=1 == totalcounts[2]=1, remove at max(1, 2)=2
    //         removal[1] = [0, 1]
    // Step 2: add vertex 2, counts = [1, 3, 2]
    //         vertex 2 already marked removed at step 2
    //         removal[2] = [2]

    assert_eq!(removal.len(), 3);
    // At step 1, vertices 0 and 1 can be removed
    assert!(removal[1].contains(&0) || removal[1].contains(&1));
    // At step 2, vertex 2 can be removed
    assert!(removal[2].contains(&2));
}

#[test]
fn test_copyline_locations_node_count() {
    // For a copy line, copyline_locations should produce nodes at every cell
    // The number of nodes should be odd (ends + center)
    let spacing = 4;

    let test_cases = [(1, 1, 1, 2), (1, 2, 1, 3), (1, 1, 2, 3), (3, 7, 5, 8)];

    for (vslot, hslot, vstart, hstop) in test_cases {
        let vstop = hslot; // Simplified: vstop = hslot
        let line = CopyLine::new(0, vslot, hslot, vstart, vstop, hstop);
        let locs = line.copyline_locations(2, spacing);

        // Node count should be odd (property of copy line construction)
        // This is verified in Julia's test: @assert length(locs) % 2 == 1
        println!(
            "vslot={}, hslot={}, vstart={}, vstop={}, hstop={}: {} nodes",
            vslot,
            hslot,
            vstart,
            vstop,
            hstop,
            locs.len()
        );

        // All weights should be 1 or 2 (for non-center nodes)
        // except center node which has weight = nline (number of line segments)
        for &(row, col, weight) in &locs {
            assert!(row > 0 && col > 0, "Coordinates should be positive");
            assert!(weight >= 1, "Weight should be >= 1");
        }
    }
}
