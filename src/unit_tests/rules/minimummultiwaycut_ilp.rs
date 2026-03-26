use super::*;
use crate::models::algebraic::ObjectiveSense;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Build the canonical 5-vertex, 3-terminal example from issue #185.
fn canonical_instance() -> MinimumMultiwayCut<SimpleGraph, i32> {
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5])
}

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = canonical_instance();
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let k = 3;
    let n = 5;
    let m = 6;
    // kn + m = 21 variables
    assert_eq!(ilp.num_vars, k * n + m);
    // n + 2km + k^2 = 5 + 36 + 9 = 50 constraints
    assert_eq!(ilp.constraints.len(), n + 2 * k * m + k * k);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimummultiwaycut_to_ilp_closed_loop() {
    let problem = canonical_instance();
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    // Solve original with brute force
    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_obj = problem.evaluate(&bf_solutions[0]);

    // Solve via ILP
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_obj = problem.evaluate(&extracted);

    // Optimal cut cost is 8
    assert_eq!(bf_obj, Min(Some(8)));
    assert_eq!(ilp_obj, Min(Some(8)));
}

#[test]
fn test_triangle_with_3_terminals() {
    // Triangle: 3 vertices, all terminals, edges: (0,1)=1, (1,2)=2, (0,2)=3
    // All 3 edges must be cut to separate every terminal pair (complete graph).
    // Optimal cost = 1 + 2 + 3 = 6
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 1, 2], vec![1, 2, 3]);

    let reduction: ReductionMMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let obj = problem.evaluate(&extracted);
    assert_eq!(obj, Min(Some(6)));
}

#[test]
fn test_two_terminals() {
    // Path: 0--1--2, terminals {0, 2}, weights [1, 2]
    // Optimal min s-t cut: cut edge (0,1) with cost 1
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1, 2]);

    let reduction: ReductionMMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let obj = problem.evaluate(&extracted);
    assert_eq!(obj, Min(Some(1)));
}

#[test]
fn test_solution_extraction() {
    let problem = canonical_instance();
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let k = 3;
    let n = 5;
    let m = 6;
    let num_vars = k * n + m;

    // Manually construct an ILP solution representing the optimal partition:
    // V_0 = {0}, V_1 = {1, 2, 3}, V_2 = {4}
    // Cut edges: (0,1)=idx 0, (3,4)=idx 3, (0,4)=idx 4
    let mut ilp_solution = vec![0usize; num_vars];

    // y_{0,v}: component 0 assignments (indices 0..5)
    ilp_solution[0] = 1; // y_{0,0} = 1 (vertex 0 in component 0)

    // y_{1,v}: component 1 assignments (indices 5..10)
    ilp_solution[5 + 1] = 1; // y_{1,1} = 1
    ilp_solution[5 + 2] = 1; // y_{1,2} = 1
    ilp_solution[5 + 3] = 1; // y_{1,3} = 1

    // y_{2,v}: component 2 assignments (indices 10..15)
    ilp_solution[10 + 4] = 1; // y_{2,4} = 1

    // x_e: cut indicators (indices 15..21)
    ilp_solution[15] = 1; // edge (0,1) cut
    ilp_solution[15 + 3] = 1; // edge (3,4) cut
    ilp_solution[15 + 4] = 1; // edge (0,4) cut

    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![1, 0, 0, 1, 1, 0]);

    let obj = problem.evaluate(&extracted);
    assert_eq!(obj, Min(Some(8)));
}

#[test]
fn test_solve_reduced() {
    let problem = canonical_instance();

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve_reduced(&problem)
        .expect("solve_reduced should work");

    assert!(problem.evaluate(&solution).is_valid());
    assert_eq!(problem.evaluate(&solution), Min(Some(8)));
}

#[test]
fn test_minimummultiwaycut_to_ilp_bf_vs_ilp() {
    let problem = canonical_instance();
    let reduction: ReductionMMCToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let bf_value = BruteForce::new().solve(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), bf_value);
}
