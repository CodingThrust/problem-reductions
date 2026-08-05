use crate::models::algebraic::MinimumMatrixDomination;
use crate::models::graph::MinimumMaximalMatching;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, Solver};
use crate::topology::{BipartiteGraph, Graph};
use crate::traits::Problem;
use crate::types::Min;

/// Build the canonical YES bipartite instance from the issue: L = {l0, l1},
/// R = {r0, r1, r2}, F = {(l0, r0), (l0, r1), (l0, r2), (l1, r1), (l1, r2)}.
///
/// In BipartiteGraph local coordinates this is:
///   left_size = 2, right_size = 3,
///   edges = (0,0), (0,1), (0,2), (1,1), (1,2).
fn yes_bipartite() -> BipartiteGraph {
    BipartiteGraph::new(2, 3, vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2)])
}

/// Build the canonical NO bipartite instance from the issue (a perfect
/// matching on 3+3 vertices: L = {l0, l1, l2}, R = {r0, r1, r2}, F =
/// {(l0, r0), (l1, r1), (l2, r2)}).
fn no_bipartite() -> BipartiteGraph {
    BipartiteGraph::new(3, 3, vec![(0, 0), (1, 1), (2, 2)])
}

#[test]
fn test_minimummaximalmatching_to_minimummatrixdomination_closed_loop() {
    let source = MinimumMaximalMatching::new(yes_bipartite());
    let reduction = ReduceTo::<MinimumMatrixDomination>::reduce_to(&source);
    let target = reduction.target_problem();

    // N = m + n = 2 + 3 = 5; |1-entries| = |F| = 5.
    assert_eq!(target.num_rows(), 5);
    assert_eq!(target.num_cols(), 5);
    assert_eq!(target.num_ones(), 5);

    let solver = BruteForce::new();

    // Source mmm(B) = 2 on this bipartite graph (two-edge maximal matching).
    assert_eq!(solver.solve(&source), Min(Some(2)));

    // Target minimum matrix domination = 2 by the Yannakakis-Gavril identity.
    assert_eq!(solver.solve(target), Min(Some(2)));

    // Closed-loop: every optimal target witness must extract to a valid
    // maximal matching of size mm(B) = 2.
    let target_witnesses = solver.find_all_witnesses(target);
    assert!(
        !target_witnesses.is_empty(),
        "matrix domination has at least one optimum"
    );
    for witness in &target_witnesses {
        let extracted = reduction.extract_solution(witness).unwrap();
        assert_eq!(
            source.evaluate(&extracted),
            Min(Some(2)),
            "extracted matching must be maximal of size 2"
        );
    }
}

#[test]
fn test_target_matrix_structure() {
    let source = MinimumMaximalMatching::new(yes_bipartite());
    let reduction = ReduceTo::<MinimumMatrixDomination>::reduce_to(&source);
    let target = reduction.target_problem();

    // Upper-right m x n block is B*. m = 2, n = 3, so 1-entries should be
    // exactly the cells (row, m + col) for each source edge (left, right):
    //   (l0, r0) -> (0, 2)
    //   (l0, r1) -> (0, 3)
    //   (l0, r2) -> (0, 4)
    //   (l1, r1) -> (1, 3)
    //   (l1, r2) -> (1, 4)
    let expected: Vec<(usize, usize)> = vec![(0, 2), (0, 3), (0, 4), (1, 3), (1, 4)];
    assert_eq!(target.ones().to_vec(), expected);

    // All other cells must be zero (upper triangular with zeros on/below the
    // diagonal of the row-block boundary).
    let matrix = target.matrix();
    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let on_block = i < 2 && j >= 2;
            if !on_block {
                assert!(!cell, "cell ({i}, {j}) must be zero");
            }
        }
    }
}

#[test]
fn test_extract_solution_returns_maximal_matching() {
    // Verify that for an arbitrary optimal target witness, extract_solution
    // returns some maximal matching whose value matches mm(B).
    let source = MinimumMaximalMatching::new(yes_bipartite());
    let reduction = ReduceTo::<MinimumMatrixDomination>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let target_witness = solver
        .find_witness(target)
        .expect("matrix domination has an optimum");
    let extracted = reduction.extract_solution(&target_witness).unwrap();

    // The result must be a valid maximal matching of the source graph and
    // realize mm(B) = 2.
    assert!(source.is_valid_maximal_matching(&extracted));
    let size: usize = extracted.iter().sum();
    assert_eq!(size, 2);
}

#[test]
fn test_no_instance_unreachable_threshold() {
    // For the 3+3 perfect-matching bipartite graph, mm(B) = 3 (the three
    // disjoint edges are the only maximal matching). The constructed matrix
    // has 3 pairwise non-attacking 1-entries (different rows and different
    // columns), so its minimum matrix domination is also 3.
    let source = MinimumMaximalMatching::new(no_bipartite());
    let reduction = ReduceTo::<MinimumMatrixDomination>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    assert_eq!(solver.solve(&source), Min(Some(3)));
    assert_eq!(solver.solve(target), Min(Some(3)));

    // The target value never drops below the source value: in particular, no
    // matrix-domination subset of size 2 exists.
    let target_value = solver.solve(target);
    if let Min(Some(value)) = target_value {
        assert!(value > 2, "matrix domination value must exceed 2");
    } else {
        panic!("target must have an optimum");
    }
}

#[test]
fn test_extract_solution_yg_transform_on_non_matching_eds() {
    // Verify that the Yannakakis-Gavril EDS->IEDS transformation correctly
    // handles a target witness whose corresponding source edges form a
    // *connected* subgraph (two edges sharing l0), not yet a matching.
    //
    // On the YES bipartite graph, the EDS {(l0, r1), (l0, r2)} is a size-2
    // minimum EDS that shares vertex l0. Its image in the matrix is the
    // 1-entries at indices 1 and 2 (positions (0, 3) and (0, 4)). This
    // dominates all other 1-entries via row 0 / column 3 / column 4.
    //
    // The naive "drop one edge" reduction fails: removing either of
    // {(l0, r1), (l0, r2)} from D leaves a vertex (l1 or r2/r1) whose
    // incident edge (l1, r2) is no longer dominated. The transformation
    // must therefore swap one of the adjacent edges for a different bipartite
    // edge whose new endpoint lies outside V(D \ {e}). The result is a valid
    // maximal matching of size <= 2, e.g. {(l0, r0), (l1, r1)} or
    // {(l0, r1), (l1, r2)}.
    let source = MinimumMaximalMatching::new(yes_bipartite());
    let reduction = ReduceTo::<MinimumMatrixDomination>::reduce_to(&source);

    // Construct the non-matching EDS witness explicitly. ones() ordering on
    // this instance is [(0,2),(0,3),(0,4),(1,3),(1,4)]; indices 1 and 2
    // pick (0,3) = source edge (l0, r1) and (0,4) = source edge (l0, r2).
    let target_witness = vec![0, 1, 1, 0, 0];

    // Sanity-check: this is actually a feasible MMD witness on the target.
    let target = reduction.target_problem();
    assert_eq!(target.evaluate(&target_witness), Min(Some(2)));

    let extracted = reduction.extract_solution(&target_witness).unwrap();

    // The extracted configuration must be a valid maximal matching of B of
    // size 2 (= mm(B)). Crucially it cannot be {(l0, r1), (l0, r2)} because
    // those two source edges share l0 and are not a matching.
    assert!(
        source.is_valid_maximal_matching(&extracted),
        "YG transform must produce a maximal matching, got {extracted:?}"
    );
    let size: usize = extracted.iter().sum();
    assert_eq!(
        size, 2,
        "extracted matching must have size mm(B) = 2, got size {size}"
    );
    assert!(
        !(extracted[1] == 1 && extracted[2] == 1),
        "transform must break the (l0,r1)-(l0,r2) adjacency"
    );
    assert_eq!(source.evaluate(&extracted), Min(Some(2)));
}

#[test]
fn test_identity_on_random_bipartite_instances() {
    // Verify the Yannakakis-Gavril identity mm(B) = min matrix domination of
    // the constructed instance on several small bipartite graphs.
    let solver = BruteForce::new();

    let instances = vec![
        // K_{1, 3} (star).
        BipartiteGraph::new(1, 3, vec![(0, 0), (0, 1), (0, 2)]),
        // K_{2, 2} (4-cycle as bipartite).
        BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        // 3+3 perfect matching.
        no_bipartite(),
        // Issue's YES instance.
        yes_bipartite(),
    ];

    for graph in instances {
        let m_left = graph.left_size();
        let n_right = graph.right_size();
        let num_edges = graph.num_edges();
        let source = MinimumMaximalMatching::new(graph);
        let reduction = ReduceTo::<MinimumMatrixDomination>::reduce_to(&source);
        let target = reduction.target_problem();

        // Structural checks: the matrix is square of side m + n and has |F|
        // 1-entries.
        assert_eq!(target.num_rows(), m_left + n_right);
        assert_eq!(target.num_cols(), m_left + n_right);
        assert_eq!(target.num_ones(), num_edges);

        let Min(Some(mm)) = solver.solve(&source) else {
            panic!("MinimumMaximalMatching always has a feasible optimum");
        };
        let Min(Some(md)) = solver.solve(target) else {
            panic!("MinimumMatrixDomination always has a feasible optimum");
        };

        assert_eq!(
            mm, md,
            "matrix-domination value must equal mm(B) by Yannakakis-Gavril"
        );
    }
}
