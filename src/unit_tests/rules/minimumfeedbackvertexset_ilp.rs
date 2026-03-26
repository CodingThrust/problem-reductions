use super::*;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Simple 3-cycle: 0 -> 1 -> 2 -> 0
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 2n = 6 variables (3 binary x_i + 3 integer o_i)
    assert_eq!(ilp.num_vars, 6, "Should have 2n variables");
    // m + 2n = 3 + 6 = 9 constraints
    assert_eq!(ilp.constraints.len(), 9, "Should have m + 2n constraints");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");
}

#[test]
fn test_minimumfeedbackvertexset_to_ilp_closed_loop() {
    // Simple 3-cycle: 0 -> 1 -> 2 -> 0
    // FVS = 1 (remove any single vertex)
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve with brute force on original problem
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_size = problem.evaluate(&bf_solutions[0]);

    // Solve via ILP reduction
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.evaluate(&extracted);

    // Both should find optimal size = 1
    assert_eq!(bf_size, Min(Some(1)));
    assert_eq!(ilp_size, Min(Some(1)));

    // Verify the ILP solution is valid for the original problem
    assert!(
        problem.evaluate(&extracted).is_valid(),
        "Extracted solution should be valid"
    );
}

#[test]
fn test_cycle_of_triangles() {
    // The example from issue #141: n=9, m=15, FVS=3
    let arcs = vec![
        (0, 1),
        (1, 2),
        (2, 0), // triangle 0-1-2
        (3, 4),
        (4, 5),
        (5, 3), // triangle 3-4-5
        (6, 7),
        (7, 8),
        (8, 6), // triangle 6-7-8
        (1, 3),
        (4, 6),
        (7, 0), // inter-triangle arcs
        (2, 5),
        (5, 8),
        (8, 2), // more inter-triangle arcs
    ];
    let graph = DirectedGraph::new(9, arcs);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 9]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Verify ILP structure
    assert_eq!(ilp.num_vars, 18, "Should have 2*9 = 18 variables");
    assert_eq!(
        ilp.constraints.len(),
        15 + 18,
        "Should have 15 arc + 18 bound constraints"
    );

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let size = problem.evaluate(&extracted);
    assert_eq!(size, Min(Some(3)), "FVS should be 3");
}

#[test]
fn test_dag_no_removal() {
    // DAG: 0 -> 1 -> 2 (no cycles, FVS = 0)
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let size = problem.evaluate(&extracted);
    assert_eq!(size, Min(Some(0)), "DAG needs no removal");
    assert_eq!(extracted, vec![0, 0, 0]);
}

#[test]
fn test_single_vertex() {
    // Single vertex, no arcs: FVS = 0
    let graph = DirectedGraph::new(1, vec![]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 2);
    // 0 arc constraints + 2 bound constraints
    assert_eq!(ilp.constraints.len(), 2);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![0]);
    assert_eq!(problem.evaluate(&extracted), Min(Some(0)));
}

#[test]
fn test_weighted() {
    // 3-cycle with different weights: prefer removing the cheapest vertex
    // Weights: v0=10, v1=1, v2=10
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![10, 1, 10]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Check that weights are correctly transferred to objective
    let mut coeffs: Vec<f64> = vec![0.0; ilp.num_vars];
    for &(var, coef) in &ilp.objective {
        coeffs[var] = coef;
    }
    assert!((coeffs[0] - 10.0).abs() < 1e-9);
    assert!((coeffs[1] - 1.0).abs() < 1e-9);
    assert!((coeffs[2] - 10.0).abs() < 1e-9);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Should remove vertex 1 (cheapest)
    assert_eq!(extracted[1], 1, "Should remove vertex 1 (cheapest)");
    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
}

#[test]
fn test_two_disjoint_cycles() {
    // Two disjoint 2-cycles: 0<->1 and 2<->3
    // Need to remove at least 1 from each cycle, FVS = 2
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 0), (2, 3), (3, 2)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 4]);

    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_size = problem.evaluate(&bf_solutions[0]);

    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_size = problem.evaluate(&extracted);

    assert_eq!(bf_size, Min(Some(2)));
    assert_eq!(ilp_size, Min(Some(2)));
}

#[test]
fn test_solution_extraction() {
    // Verify that extraction correctly takes first n values
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Simulate ILP solution: x_0=1, x_1=0, x_2=0, o_0=0, o_1=0, o_2=1
    let ilp_solution = vec![1, 0, 0, 0, 0, 1];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 0]);

    // Verify this is a valid FVS (removing vertex 0 breaks the 3-cycle)
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimumfeedbackvertexset_to_ilp_bf_vs_ilp() {
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
    let problem = MinimumFeedbackVertexSet::new(graph, vec![1i32; 3]);
    let reduction: ReductionMFVSToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let bf_value = BruteForce::new().solve(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), bf_value);
}
