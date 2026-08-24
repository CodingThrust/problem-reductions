use super::*;
use crate::models::graph::BicliqueCover;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;
use crate::variant::KN;

/// Helper: extract a vertex-major `BicliqueCover` cell.
fn cell(config: &[usize], vertex: usize, biclique: usize, k: usize) -> usize {
    config[vertex * k + biclique]
}

/// Build a closed-loop test on the smallest source that is non-trivial yet
/// keeps `num_vertices * rank <= 16` so the BicliqueCover brute force runs
/// fast. With `n = 1`, `q = 1` we have `4n * (n + q) = 8` binary variables.
#[test]
fn test_kcoloring_to_bicliquecover_closed_loop_trivial() {
    // Single isolated vertex with q = 1: trivially 1-colorable.
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(1, vec![]), 1);
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // Solve the target via brute force and verify the extracted coloring
    // is a proper q-coloring of the source.
    let witness = BruteForce::new()
        .find_witness(target)
        .unwrap()
        .expect("trivial target must be feasible");
    let coloring = reduction.extract_solution(&witness).unwrap();
    assert_eq!(coloring.len(), 1);
    assert!(source.is_valid_solution(&coloring));
    // The source brute force agrees.
    assert_eq!(source.evaluate(&coloring).unwrap(), Or(true));
}

/// Structural assertions against the exact target sizes derived in the
/// issue. Picks a small but non-trivial instance: P_3 (path on 3 vertices)
/// with q = 2.
#[test]
fn test_kcoloring_to_bicliquecover_structure_path() {
    // n = 3, m = 2 (path 0-1-2), q = 2.
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 2);
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let n = 3;
    let m = 2;
    let q = 2;

    assert_eq!(target.left_size(), 2 * n);
    assert_eq!(target.right_size(), 2 * n);
    assert_eq!(target.num_vertices(), 4 * n);
    assert_eq!(target.k(), n + q);
    // 2 n (n-1) - 4 m + 3 n = 12 - 8 + 9 = 13.
    assert_eq!(target.num_edges(), 2 * n * (n - 1) - 4 * m + 3 * n);
    assert_eq!(target.num_edges(), 13);
}

/// Structure check on K_4 with q = 3 (NO instance for the source).
#[test]
fn test_kcoloring_to_bicliquecover_structure_clique() {
    // K_4: n = 4, m = 6, q = 3.
    let source = KColoring::<KN, _>::with_k(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        3,
    );
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    let n = 4;
    let m = 6;
    let q = 3;
    assert_eq!(target.num_vertices(), 4 * n);
    assert_eq!(target.k(), n + q);
    // 2 n (n-1) - 4 m + 3 n = 24 - 24 + 12 = 12.
    assert_eq!(target.num_edges(), 2 * n * (n - 1) - 4 * m + 3 * n);
    assert_eq!(target.num_edges(), 12);

    // K_4 with q = 3 has no proper coloring.
    assert!(BruteForce::new().find_witness(&source).unwrap().is_none());
}

/// Build the explicit forward witness (guard bicliques + color bicliques)
/// from the proof, verify it covers all target edges as a valid biclique
/// cover, and confirm `extract_solution` recovers a proper coloring.
#[test]
fn test_kcoloring_to_bicliquecover_forward_witness_path_q2() {
    // P_3 with the obvious 2-coloring (0, 1, 0). Independent sets are
    // {0, 2} (color 0) and {1} (color 1).
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 2);
    let coloring = vec![0usize, 1, 0];
    assert!(source.is_valid_solution(&coloring));

    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    let witness = forward_witness(&source, &coloring);

    // Witness covers all edges with rank <= n + q.
    assert!(target.is_valid_cover(&witness));
    // Extraction recovers a proper coloring.
    let extracted = reduction.extract_solution(&witness).unwrap();
    assert!(source.is_valid_solution(&extracted));
}

/// Forward witness on a small triangle-free graph. C_4 with q = 2: the
/// 4-cycle is bipartite, with a canonical 2-coloring (0,1,0,1).
#[test]
fn test_kcoloring_to_bicliquecover_forward_witness_cycle_q2() {
    let source =
        KColoring::<KN, _>::with_k(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]), 2);
    let coloring = vec![0usize, 1, 0, 1];
    assert!(source.is_valid_solution(&coloring));

    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    let witness = forward_witness(&source, &coloring);

    assert!(target.is_valid_cover(&witness));
    let extracted = reduction.extract_solution(&witness).unwrap();
    assert!(source.is_valid_solution(&extracted));
}

/// Sub-biclique semantics: a configuration that tries to group two
/// adjacent source vertices `u, v` into a single color biclique is
/// rejected because the compatibility edge `(a_u, b_v)` is absent.
#[test]
fn test_kcoloring_to_bicliquecover_rejects_adjacent_grouping() {
    // P_2 with q = 2; edge (0, 1) means vertices 0 and 1 are adjacent.
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1)]), 2);
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // The "bad" witness re-uses the canonical forward witness but pretends
    // both source vertices share color 0 (an invalid coloring). The
    // resulting color biclique contains a_0, a_1, b_0, b_1, so it would
    // need to cover (a_0, b_1) and (a_1, b_0) which are absent.
    let n = 2usize;
    let q = 2usize;
    let k = n + q;
    let left_size = 2 * n;
    let num_vertices = 4 * n;
    let mut bad = vec![0usize; num_vertices * k];

    // Helper: set vertex `v` (unified index) as a member of biclique `r`.
    let set = |bad: &mut Vec<usize>, vertex: usize, biclique: usize| {
        bad[vertex * k + biclique] = 1;
    };
    // Guard bicliques (biclique indices 0 and 1) cover the guard-anchor
    // edges correctly.
    // Biclique 0: G_0 = ({a_0, g_0}, {h_0}); no nonadjacent w of 0.
    set(&mut bad, 0, 0); // a_0
    set(&mut bad, 1, 0); // g_0
    set(&mut bad, left_size + n, 0); // h_0
                                     // Biclique 1: G_1 = ({a_1, g_1}, {h_1}); no nonadjacent w of 1.
    set(&mut bad, 2, 1); // a_1
    set(&mut bad, 3, 1); // g_1
    set(&mut bad, left_size + n + 1, 1); // h_1

    // Bad color biclique (biclique 2): try to merge a_0, a_1 with b_0, b_1.
    set(&mut bad, 0, 2); // a_0
    set(&mut bad, 2, 2); // a_1
    set(&mut bad, left_size, 2); // b_0
    set(&mut bad, left_size + 1, 2); // b_1

    // This violates the sub-biclique requirement: (a_0, b_1) is not an
    // edge of H because {0,1} is in E(G), so the cover must reject.
    assert!(!target.is_valid_cover(&bad));
}

/// `extract_solution` returns a proper coloring on the canonical forward
/// witness, even though the source instance is small.
#[test]
fn test_kcoloring_to_bicliquecover_extract_solution_on_forward_witness() {
    // Triangle K_3 with q = 3: each vertex must have its own color.
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]), 3);
    let coloring = vec![0usize, 1, 2];
    assert!(source.is_valid_solution(&coloring));

    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();
    let witness = forward_witness(&source, &coloring);
    assert!(target.is_valid_cover(&witness));

    let extracted = reduction.extract_solution(&witness).unwrap();
    assert!(source.is_valid_solution(&extracted));
    // K_3 forces 3 distinct colors.
    let mut seen = std::collections::BTreeSet::new();
    for &c in &extracted {
        seen.insert(c);
    }
    assert_eq!(seen.len(), 3);
}

/// Edge-list sanity check: explicitly enumerate the expected edges for a
/// 2-vertex source with one edge, and confirm every constructed target
/// edge matches.
#[test]
fn test_kcoloring_to_bicliquecover_explicit_edges_p2() {
    // P_2: n = 2, m = 1, edge (0,1), q = 2.
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(2, vec![(0, 1)]), 2);
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // n = 2, m = 1, q = 2 => num_edges = 2*2*1 - 4*1 + 6 = 6.
    // Layout: a_v -> left v; g_v -> left (n+v); b_v -> right v; h_v -> right (n+v).
    // Expected (bipartite-local):
    //   diagonal:        (0,0), (1,1)            [a_0,b_0; a_1,b_1]
    //   compatibility:   none                    (only u != v pair is (0,1), and {0,1} in E)
    //   guard-anchor:    (0,2), (2,2), (1,3), (3,3)
    //   guard-compat:    none                    (no nonadjacent w)
    let expected: std::collections::HashSet<(usize, usize)> =
        [(0usize, 0usize), (1, 1), (0, 2), (2, 2), (1, 3), (3, 3)]
            .into_iter()
            .collect();
    assert_eq!(target.num_edges(), expected.len());
    let actual: std::collections::HashSet<(usize, usize)> =
        target.graph().left_edges().iter().copied().collect();
    assert_eq!(actual, expected);
}

/// Smoke test for `extract_solution` on the trivial closed loop: the only
/// biclique cover witness yields a single-vertex coloring of color 0.
#[test]
fn test_kcoloring_to_bicliquecover_extract_trivial_layout() {
    let source = KColoring::<KN, _>::with_k(SimpleGraph::new(1, vec![]), 1);
    let reduction =
        ReduceTo::<BicliqueCover>::reduce_to(&source).expect("reduction should succeed");
    let target = reduction.target_problem();

    // n = 1, q = 1, k = 2, num_vertices = 4.
    // Use the canonical forward witness for the only valid coloring.
    let coloring = vec![0usize];
    let witness = forward_witness(&source, &coloring);
    assert!(target.is_valid_cover(&witness));

    // The diagonal edge (a_0, b_0) should be in the color biclique r = 1
    // (since the guard biclique r = 0 holds (a_0, h_0) and (g_0, h_0)).
    let k = target.k();
    // a_0 is unified vertex 0, b_0 is unified vertex left_size = 2.
    assert_eq!(cell(&witness, 0, 1, k), 1);
    assert_eq!(cell(&witness, 2, 1, k), 1);

    let extracted = reduction.extract_solution(&witness).unwrap();
    assert_eq!(extracted, vec![0]);
}
