use super::*;
use crate::models::algebraic::MinimumMatrixCover;
use crate::models::graph::MaxCut;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::traits::ReduceTo;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Max, Min};

/// Brute-force verifies the algebraic identity `Σ a_ij f(i) f(j) = 2W − 4·cut(S)`
/// for every sign assignment on a small instance.
fn verify_identity(source: &MaxCut<SimpleGraph, i32>) {
    let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(source);
    let target = reduction.target_problem();
    let matrix = target.matrix();
    let n = source.num_vertices();
    let total_weight: i64 = source.edge_weights().iter().map(|&w| w as i64).sum();

    for bits in 0..(1u32 << n) {
        let config: Vec<usize> = (0..n).map(|i| ((bits >> i) & 1) as usize).collect();

        // qf(f) = Σ_{i,j} a_ij f(i) f(j)
        let signs: Vec<i64> = config.iter().map(|&x| 2 * x as i64 - 1).collect();
        let mut qf: i64 = 0;
        for i in 0..n {
            for j in 0..n {
                qf += matrix[i][j] * signs[i] * signs[j];
            }
        }

        // cut(S) from MaxCut.evaluate (Max value)
        let Max(Some(cut)) = source.evaluate(&config) else {
            panic!("MaxCut must yield a finite cut for every config");
        };
        let cut64 = cut;

        assert_eq!(
            qf,
            2 * total_weight - 4 * cut64,
            "identity failed for config {:?}: qf = {}, 2W - 4*cut = {}",
            config,
            qf,
            2 * total_weight - 4 * cut64
        );
    }
}

#[test]
fn test_maxcut_to_minimummatrixcover_closed_loop_c4() {
    // C_4 with unit weights: max cut = 4 (partition {0,2} vs {1,3} cuts all edges).
    let source = MaxCut::<SimpleGraph, i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
        vec![1, 1, 1, 1],
    );
    let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut C_4 unit weights -> MinimumMatrixCover",
    );

    // Verify target structure: 4x4 symmetric, zero diagonal, adjacency matrix.
    let target = reduction.target_problem();
    assert_eq!(target.num_rows(), 4);
    let expected: Vec<Vec<i64>> = vec![
        vec![0, 1, 0, 1],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 0, 1, 0],
    ];
    assert_eq!(target.matrix(), expected.as_slice());

    // Verify target's minimum value matches 2W - 4*MaxCut: 2*4 - 4*4 = -8.
    let solver = BruteForce::new();
    assert_eq!(solver.solve(target), Min(Some(-8)));
}

#[test]
fn test_maxcut_to_minimummatrixcover_closed_loop_p3_weighted() {
    // Path P_3 = 0-1-2 with weights (2, 3): max cut = 5 (split {1} vs {0, 2}).
    let source =
        MaxCut::<SimpleGraph, i32>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![2, 3]);
    let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut P_3 weights (2,3) -> MinimumMatrixCover",
    );

    let target = reduction.target_problem();
    // W = 5, max cut = 5, so min qf = 2*5 - 4*5 = -10.
    let solver = BruteForce::new();
    assert_eq!(solver.solve(target), Min(Some(-10)));

    // Verify the adjacency matrix is symmetric with zero diagonal.
    let expected: Vec<Vec<i64>> = vec![vec![0, 2, 0], vec![2, 0, 3], vec![0, 3, 0]];
    assert_eq!(target.matrix(), expected.as_slice());
}

#[test]
fn test_maxcut_to_minimummatrixcover_closed_loop_triangle() {
    // K_3 (triangle) with unit weights: max cut = 2.
    let source = MaxCut::<SimpleGraph, i32>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
    );
    let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaxCut K_3 unit -> MinimumMatrixCover",
    );

    let target = reduction.target_problem();
    // W = 3, max cut = 2, so min qf = 2*3 - 4*2 = -2.
    let solver = BruteForce::new();
    assert_eq!(solver.solve(target), Min(Some(-2)));
}

#[test]
fn test_target_structure_matches_adjacency_matrix() {
    // Verify the construction details on an asymmetric weighted graph.
    let source = MaxCut::<SimpleGraph, i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 3), (1, 2), (2, 3)]),
        vec![5, 7, 2, 3],
    );
    let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_rows(), 4);
    let expected: Vec<Vec<i64>> = vec![
        vec![0, 5, 0, 7],
        vec![5, 0, 2, 0],
        vec![0, 2, 0, 3],
        vec![7, 0, 3, 0],
    ];
    assert_eq!(target.matrix(), expected.as_slice());

    // Diagonal must be zero.
    for i in 0..4 {
        assert_eq!(target.matrix()[i][i], 0);
    }
    // Symmetry.
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(target.matrix()[i][j], target.matrix()[j][i]);
        }
    }
}

#[test]
fn test_algebraic_identity_c4_unit() {
    // The identity Σ a_ij f(i) f(j) = 2W − 4·cut(S) must hold for every f.
    let source = MaxCut::<SimpleGraph, i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
        vec![1, 1, 1, 1],
    );
    verify_identity(&source);
}

#[test]
fn test_algebraic_identity_p3_weighted() {
    let source =
        MaxCut::<SimpleGraph, i32>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![2, 3]);
    verify_identity(&source);
}

#[test]
fn test_algebraic_identity_triangle_weighted() {
    // Triangle with non-uniform weights.
    let source = MaxCut::<SimpleGraph, i32>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![4, 1, 2],
    );
    verify_identity(&source);
}

#[test]
fn test_extract_solution_is_identity() {
    let source =
        MaxCut::<SimpleGraph, i32>::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 1]);
    let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
    let target_sol = vec![1, 0, 1];
    assert_eq!(reduction.extract_solution(&target_sol).unwrap(), target_sol);
}

#[test]
fn test_empty_graph() {
    // n vertices, zero edges: matrix is all zeros, max cut = 0.
    let source = MaxCut::<SimpleGraph, i32>::new(SimpleGraph::new(3, vec![]), vec![]);
    let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_rows(), 3);
    let expected: Vec<Vec<i64>> = vec![vec![0; 3]; 3];
    assert_eq!(target.matrix(), expected.as_slice());

    let solver = BruteForce::new();
    assert_eq!(solver.solve(target), Min(Some(0)));
}

#[test]
fn test_overhead_num_rows_equals_num_vertices() {
    // Spot-check the exact size map: target.num_rows == source.num_vertices.
    for n in [1usize, 2, 5, 8] {
        let edges: Vec<(usize, usize)> = (0..n.saturating_sub(1)).map(|i| (i, i + 1)).collect();
        let weights: Vec<i32> = vec![1; edges.len()];
        let source = MaxCut::<SimpleGraph, i32>::new(SimpleGraph::new(n, edges), weights);
        let reduction = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
        assert_eq!(reduction.target_problem().num_rows(), n);
    }
}

#[test]
#[should_panic(expected = "nonnegative")]
fn test_negative_weight_panics() {
    // The reduction only handles nonnegative weights.
    let source = MaxCut::<SimpleGraph, i32>::new(SimpleGraph::new(2, vec![(0, 1)]), vec![-1]);
    let _ = ReduceTo::<MinimumMatrixCover>::reduce_to(&source);
}
