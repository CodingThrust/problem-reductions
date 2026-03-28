use crate::models::algebraic::ConsecutiveOnesSubmatrix;
use crate::models::graph::HamiltonianPath;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::Problem;

/// Helper: build a path graph 0-1-2-3 (has HP).
fn path4() -> HamiltonianPath<SimpleGraph> {
    HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]))
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_closed_loop() {
    let source = path4();
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianPath -> ConsecutiveOnesSubmatrix",
    );
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_structure() {
    let source = path4();
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);
    let target = reduction.target_problem();

    // 4 vertices -> 4 rows
    assert_eq!(target.num_rows(), 4);
    // 3 edges -> 3 columns
    assert_eq!(target.num_cols(), 3);
    // K = n - 1 = 3
    assert_eq!(target.bound(), 3);

    // Check incidence matrix structure:
    // Edge 0: (0,1), Edge 1: (1,2), Edge 2: (2,3)
    let matrix = target.matrix();
    // Vertex 0 is endpoint of edge 0 only
    assert_eq!(matrix[0], vec![true, false, false]);
    // Vertex 1 is endpoint of edges 0 and 1
    assert_eq!(matrix[1], vec![true, true, false]);
    // Vertex 2 is endpoint of edges 1 and 2
    assert_eq!(matrix[2], vec![false, true, true]);
    // Vertex 3 is endpoint of edge 2 only
    assert_eq!(matrix[3], vec![false, false, true]);
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_extract_solution() {
    let source = path4();
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);

    // Select all 3 edges (columns) — they form the Hamiltonian path.
    let target_config = vec![1, 1, 1];
    let extracted = reduction.extract_solution(&target_config);

    assert_eq!(extracted.len(), 4);
    assert!(
        source.evaluate(&extracted).0,
        "extracted solution must be a valid Hamiltonian path"
    );
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_no_path_few_edges() {
    // Disconnected graph: 0-1, 2-3 (2 edges < n-1 = 3, no Hamiltonian path).
    // The reduction detects m < n-1 and produces a Tucker unsatisfiable instance.
    let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]));
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(
        witness.is_none(),
        "disconnected graph with too few edges should be unsatisfiable"
    );
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_no_path_disconnected() {
    // Two disjoint triangles: 6 vertices, 6 edges, no HP (disconnected).
    let source = HamiltonianPath::new(SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5)],
    ));
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_rows(), 6);
    assert_eq!(target.num_cols(), 6);
    assert_eq!(target.bound(), 5);

    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(
        witness.is_none(),
        "two disjoint triangles should have no Hamiltonian path"
    );
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_triangle() {
    // Triangle: 0-1, 1-2, 0-2 (has HP, e.g. 0-1-2)
    let source = HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]));
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianPath(triangle) -> ConsecutiveOnesSubmatrix",
    );
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_single_vertex() {
    // Single vertex, no edges — trivially has HP (path of length 0).
    let source = HamiltonianPath::new(SimpleGraph::new(1, vec![]));
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_rows(), 1);
    assert_eq!(target.num_cols(), 0);
    assert_eq!(target.bound(), 0);

    // K=0 is vacuously satisfiable; empty config selects 0 columns.
    let solver = BruteForce::new();
    let witness = solver.find_witness(target);
    assert!(witness.is_some(), "single vertex should be satisfiable");
}

#[test]
fn test_hamiltonianpath_to_consecutiveonessubmatrix_cycle5() {
    // 5-cycle: 0-1-2-3-4-0 (has HP, e.g. 0-1-2-3-4)
    let source = HamiltonianPath::new(SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4)],
    ));
    let reduction = ReduceTo::<ConsecutiveOnesSubmatrix>::reduce_to(&source);

    assert_eq!(reduction.target_problem().num_cols(), 5);
    assert_eq!(reduction.target_problem().bound(), 4);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "HamiltonianPath(C5) -> ConsecutiveOnesSubmatrix",
    );
}
