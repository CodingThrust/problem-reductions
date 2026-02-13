use super::*;
use std::collections::HashSet;

#[test]
fn test_layout_empty() {
    let layout = Layout::empty(5);
    assert_eq!(layout.vertices.len(), 0);
    assert_eq!(layout.vsep(), 0);
    assert_eq!(layout.disconnected.len(), 5);
    assert_eq!(layout.neighbors.len(), 0);
}

#[test]
fn test_layout_new() {
    // Path graph: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let layout = Layout::new(3, &edges, vec![0, 1, 2]);
    assert_eq!(layout.vertices, vec![0, 1, 2]);
    assert_eq!(layout.vsep(), 1); // Path has pathwidth 1
}

#[test]
fn test_vsep_and_neighbors_path() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let adj = build_adj(3, &edges);
    let (vsep, _) = vsep_and_neighbors(3, &adj, &[0, 1, 2]);
    assert_eq!(vsep, 1);
}

#[test]
fn test_vsep_and_neighbors_star() {
    // Star: 0 connected to 1, 2, 3
    let edges = vec![(0, 1), (0, 2), (0, 3)];
    let adj = build_adj(4, &edges);
    // Order: 0, 1, 2, 3 - after adding 0, all others become neighbors
    let (vsep, _) = vsep_and_neighbors(4, &adj, &[0, 1, 2, 3]);
    assert_eq!(vsep, 3); // After adding 0, neighbors = {1, 2, 3}
}

#[test]
fn test_extend() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let adj = build_adj(3, &edges);
    let layout = Layout::empty(3);
    let layout = extend(&adj, &layout, 0);
    assert_eq!(layout.vertices, vec![0]);
    assert!(layout.neighbors.contains(&1));
    assert!(layout.disconnected.contains(&2));
}

#[test]
fn test_greedy_decompose_path() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let layout = greedy_decompose(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_greedy_decompose_triangle() {
    // Triangle: 0-1, 1-2, 0-2
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let layout = greedy_decompose(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 2); // Triangle has pathwidth 2
}

#[test]
fn test_greedy_decompose_k4() {
    // Complete graph K4
    let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
    let layout = greedy_decompose(4, &edges);
    assert_eq!(layout.vertices.len(), 4);
    assert_eq!(layout.vsep(), 3); // K4 has pathwidth 3
}

#[test]
fn test_branch_and_bound_path() {
    // Path: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let layout = branch_and_bound(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_branch_and_bound_triangle() {
    // Triangle
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let layout = branch_and_bound(3, &edges);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 2);
}

#[test]
fn test_pathwidth_greedy() {
    let edges = vec![(0, 1), (1, 2)];
    let layout = pathwidth(3, &edges, PathDecompositionMethod::greedy());
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_pathwidth_minhthi() {
    let edges = vec![(0, 1), (1, 2)];
    let layout = pathwidth(3, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 3);
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_vertex_order_from_layout() {
    let layout = Layout {
        vertices: vec![0, 1, 2],
        vsep: 1,
        neighbors: vec![],
        disconnected: vec![],
    };
    let order = vertex_order_from_layout(&layout);
    // Returns vertices in same order as layout (matching Julia's behavior)
    assert_eq!(order, vec![0, 1, 2]);
}

#[test]
fn test_petersen_graph_pathwidth() {
    // Petersen graph edges
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

    let layout = pathwidth(10, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 10);
    // Petersen graph has pathwidth 5
    assert_eq!(layout.vsep(), 5);
}

#[test]
fn test_cycle_graph_pathwidth() {
    // Cycle C5: 0-1-2-3-4-0
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
    let layout = pathwidth(5, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 5);
    // Cycle has pathwidth 2
    assert_eq!(layout.vsep(), 2);
}

#[test]
fn test_disconnected_graph() {
    // Two disconnected edges: 0-1, 2-3
    let edges = vec![(0, 1), (2, 3)];
    let layout = pathwidth(4, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 4);
    // Pathwidth is 1 (each component has pathwidth 1)
    assert_eq!(layout.vsep(), 1);
}

#[test]
fn test_empty_graph() {
    // No edges
    let edges: Vec<(usize, usize)> = vec![];
    let layout = pathwidth(5, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), 5);
    assert_eq!(layout.vsep(), 0); // No edges means pathwidth 0
}

#[test]
fn test_pathwidth_auto_small() {
    // Small graph (≤30 vertices) → Auto selects MinhThiTrick
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
    let layout = pathwidth(5, &edges, PathDecompositionMethod::Auto);
    assert_eq!(layout.vertices.len(), 5);
    assert_eq!(layout.vsep(), 2); // Cycle C5 has pathwidth 2
}

#[test]
fn test_pathwidth_auto_large() {
    // Large graph (>30 vertices) → Auto selects Greedy
    let n = 35;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let layout = pathwidth(n, &edges, PathDecompositionMethod::Auto);
    assert_eq!(layout.vertices.len(), n);
    assert_eq!(layout.vsep(), 1); // Path graph has pathwidth 1
}

// === Ground truth tests from JSON dataset ===

/// Compute vsep from scratch for a given vertex ordering on a graph.
/// This is an independent reimplementation for verification — it does NOT call
/// any function from the pathdecomposition module.
fn verify_vsep(num_vertices: usize, edges: &[(usize, usize)], order: &[usize]) -> usize {
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); num_vertices];
    for &(u, v) in edges {
        adj[u].insert(v);
        adj[v].insert(u);
    }

    let mut vsep = 0;
    let mut added: HashSet<usize> = HashSet::new();

    for &v in order {
        added.insert(v);
        // Count vertices not yet added but adjacent to some added vertex
        let frontier = (0..num_vertices)
            .filter(|w| !added.contains(w) && adj[*w].iter().any(|u| added.contains(u)))
            .count();
        vsep = vsep.max(frontier);
    }
    vsep
}

#[derive(serde::Deserialize)]
struct PathwidthEntry {
    graph: String,
    num_vertices: usize,
    num_edges: usize,
    pathwidth: usize,
    vertex_order: Vec<usize>,
}

fn load_pathwidth_ground_truth() -> Vec<PathwidthEntry> {
    let path = format!(
        "{}/tests/data/pathwidth_ground_truth.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let content = std::fs::read_to_string(&path).expect("Failed to read pathwidth ground truth");
    serde_json::from_str(&content).expect("Failed to parse pathwidth ground truth JSON")
}

#[test]
fn test_ground_truth_vertex_order_is_valid_permutation() {
    let entries = load_pathwidth_ground_truth();
    for entry in &entries {
        let (n, edges) = crate::topology::smallgraph(&entry.graph).unwrap();
        assert_eq!(n, entry.num_vertices, "{}: vertex count mismatch", entry.graph);
        assert_eq!(edges.len(), entry.num_edges, "{}: edge count mismatch", entry.graph);

        // vertex_order must be a permutation of 0..n
        let mut sorted = entry.vertex_order.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            (0..n).collect::<Vec<_>>(),
            "{}: vertex_order is not a valid permutation",
            entry.graph
        );
    }
}

#[test]
fn test_ground_truth_vsep_matches_claimed_pathwidth() {
    let entries = load_pathwidth_ground_truth();
    for entry in &entries {
        let (n, edges) = crate::topology::smallgraph(&entry.graph).unwrap();

        // Independently compute vsep for the given vertex order
        let computed_vsep = verify_vsep(n, &edges, &entry.vertex_order);
        assert_eq!(
            computed_vsep, entry.pathwidth,
            "{}: vsep of vertex_order ({}) != claimed pathwidth ({})",
            entry.graph, computed_vsep, entry.pathwidth
        );
    }
}

#[test]
fn test_branch_and_bound_matches_ground_truth() {
    let entries = load_pathwidth_ground_truth();
    for entry in &entries {
        // tutte (46 vertices) is too slow for routine B&B; tested separately with #[ignore]
        if entry.graph == "tutte" {
            continue;
        }
        let (n, edges) = crate::topology::smallgraph(&entry.graph).unwrap();
        let layout = pathwidth(n, &edges, PathDecompositionMethod::MinhThiTrick);

        // Must produce a complete layout
        assert_eq!(
            layout.vertices.len(), n,
            "{}: layout missing vertices", entry.graph
        );

        // Pathwidth must match ground truth (branch-and-bound is exact)
        assert_eq!(
            layout.vsep(), entry.pathwidth,
            "{}: branch_and_bound vsep ({}) != ground truth ({})",
            entry.graph, layout.vsep(), entry.pathwidth
        );

        // Independently verify the produced layout's vsep
        let verified = verify_vsep(n, &edges, &layout.vertices);
        assert_eq!(
            verified, layout.vsep(),
            "{}: Layout.vsep ({}) != independently verified vsep ({})",
            entry.graph, layout.vsep(), verified
        );
    }
}

#[test]
#[ignore] // tutte (46 vertices) takes ~10s in branch-and-bound
fn test_branch_and_bound_tutte() {
    let (n, edges) = crate::topology::smallgraph("tutte").unwrap();
    let layout = pathwidth(n, &edges, PathDecompositionMethod::MinhThiTrick);
    assert_eq!(layout.vertices.len(), n);
    assert_eq!(layout.vsep(), 6); // known pathwidth from ground truth
    let verified = verify_vsep(n, &edges, &layout.vertices);
    assert_eq!(verified, layout.vsep());
}

#[test]
fn test_greedy_respects_ground_truth_upper_bound() {
    let entries = load_pathwidth_ground_truth();
    for entry in &entries {
        let (n, edges) = crate::topology::smallgraph(&entry.graph).unwrap();
        let layout = pathwidth(n, &edges, PathDecompositionMethod::greedy_with_restarts(20));

        // Greedy may not be optimal but must not be worse than n-1
        assert!(
            layout.vsep() >= entry.pathwidth,
            "{}: greedy vsep ({}) < optimal ({}), which is impossible",
            entry.graph, layout.vsep(), entry.pathwidth
        );

        // Must be a complete layout
        assert_eq!(layout.vertices.len(), n, "{}: greedy layout incomplete", entry.graph);

        // Independently verify
        let verified = verify_vsep(n, &edges, &layout.vertices);
        assert_eq!(
            verified, layout.vsep(),
            "{}: greedy Layout.vsep ({}) != independently verified ({})",
            entry.graph, layout.vsep(), verified
        );
    }
}
