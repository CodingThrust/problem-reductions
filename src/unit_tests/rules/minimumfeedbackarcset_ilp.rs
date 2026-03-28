use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Simple 3-cycle: 0 -> 1 -> 2 -> 0
    // m=3 arcs, n=3 vertices → 6 variables, m+m+n = 9 constraints
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionFASToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // m + n = 3 + 3 = 6 variables (3 binary y_a + 3 integer o_v)
    assert_eq!(ilp.num_vars, 6, "Should have m + n variables");
    // m (binary bounds) + n (order bounds) + m (arc constraints) = 3 + 3 + 3 = 9
    assert_eq!(ilp.constraints.len(), 9, "Should have 2*m + n constraints");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");
}

#[test]
fn test_minimumfeedbackarcset_to_ilp_bf_vs_ilp() {
    // Triangle cycle: 0 -> 1 -> 2 -> 0
    // FAS = 1 (remove any single arc to break the cycle)
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionFASToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    // Both should find optimal value = 1
    assert_eq!(bf_value, Min(Some(1)));
    assert_eq!(ilp_value, Min(Some(1)));
}

#[test]
fn test_solution_extraction() {
    // Verify that extraction correctly takes first m arc values
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionFASToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Simulate ILP solution: y_0=0, y_1=0, y_2=1, o_0=0, o_1=1, o_2=2
    let ilp_solution = vec![0, 0, 1, 0, 1, 2];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0, 1]);

    // Verify this is a valid FAS (removing arc 2->0 breaks the 3-cycle)
    assert!(
        problem.evaluate(&extracted).is_valid(),
        "Extracted solution should be a valid FAS"
    );
}

#[test]
fn test_minimumfeedbackarcset_to_ilp_trivial() {
    // DAG: 0 -> 1 -> 2 (no cycles, FAS = 0)
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumFeedbackArcSet::new(graph, vec![1i32; 2]);
    let reduction: ReductionFASToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // m=2, n=3 → 5 variables; 2 + 3 + 2 = 7 constraints
    assert_eq!(ilp.num_vars, 5);
    assert_eq!(ilp.constraints.len(), 7);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let value = problem.evaluate(&extracted);
    assert_eq!(value, Min(Some(0)), "DAG needs no arc removal");
    assert_eq!(extracted, vec![0, 0]);
}
