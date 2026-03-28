use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Path P3: 0-1-2
    let problem = HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction: ReductionHamiltonianPathToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // n=3, m=2, n_pos=2
    // num_x = 9, num_z = 2*2*2 = 8, total = 17
    assert_eq!(ilp.num_vars, 17);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_hamiltonianpath_to_ilp_closed_loop() {
    // Path graph: 0-1-2-3 (has Hamiltonian path)
    let problem = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    // BruteForce on source to verify feasibility
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert_eq!(problem.evaluate(&bf_solution), Or(true));

    // Solve via ILP
    let reduction: ReductionHamiltonianPathToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "ILP solution should satisfy the HamiltonianPath constraint"
    );
}

#[test]
fn test_hamiltonianpath_to_ilp_cycle_graph() {
    // C4: 0-1-2-3-0 (has multiple Hamiltonian paths)
    let problem = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]));
    // BruteForce on source
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert_eq!(problem.evaluate(&bf_solution), Or(true));

    // Solve via ILP
    let reduction: ReductionHamiltonianPathToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_hamiltonianpath_to_ilp_bf_vs_ilp() {
    let problem = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionHamiltonianPathToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_hamiltonianpath_to_ilp_no_path() {
    // Disconnected graph: no Hamiltonian path
    let problem = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]));
    let reduction: ReductionHamiltonianPathToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(reduction.target_problem());
    assert!(
        result.is_none(),
        "Disconnected graph should have no Hamiltonian path"
    );
}

#[test]
fn test_solution_extraction() {
    let problem = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionHamiltonianPathToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
