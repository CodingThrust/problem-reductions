use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Host: K4, Pattern: K3
    let host = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let pattern = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);
    let reduction: ReductionSubIsoToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // n_pat=3, n_host=4: num_vars=12
    assert_eq!(ilp.num_vars, 12);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_subgraphisomorphism_to_ilp_closed_loop() {
    // Host: K4, Pattern: K3 (always embeddable)
    let host = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let pattern = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // BruteForce on source to confirm feasibility
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert_eq!(problem.evaluate(&bf_solution), Or(true));

    // Solve via ILP
    let reduction: ReductionSubIsoToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "ILP solution should be a valid subgraph isomorphism"
    );
}

#[test]
fn test_subgraphisomorphism_to_ilp_path_in_cycle() {
    // Host: C4, Pattern: P3 (path on 3 vertices)
    let host = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let pattern = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);

    // BruteForce on source
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert_eq!(problem.evaluate(&bf_solution), Or(true));

    // Solve via ILP
    let reduction: ReductionSubIsoToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_subgraphisomorphism_to_ilp_infeasible() {
    // Host: path 0-1-2, Pattern: triangle K3 (not embeddable)
    let host = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let pattern = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);
    let reduction: ReductionSubIsoToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(reduction.target_problem());
    assert!(result.is_none(), "K3 in path should be infeasible");
}

#[test]
fn test_solution_extraction() {
    let host = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let pattern = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = SubgraphIsomorphism::new(host, pattern);
    let reduction: ReductionSubIsoToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
