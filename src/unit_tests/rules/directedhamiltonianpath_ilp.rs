use super::*;
use crate::models::graph::DirectedHamiltonianPath;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Directed path: 0->1->2 (n=3)
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = DirectedHamiltonianPath::new(graph);
    let reduction: ReductionDirectedHamiltonianPathToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    // n=3, num_vars = 3^2 = 9
    assert_eq!(ilp.num_vars, 9);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_directedhamiltonianpath_to_ilp_closed_loop() {
    // Directed path: 0->1->2->3
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = DirectedHamiltonianPath::new(graph);

    // BruteForce to verify feasibility
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("brute-force should find a solution");
    assert_eq!(problem.evaluate(&bf_solution), Or(true));

    // Solve via ILP
    let reduction: ReductionDirectedHamiltonianPathToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "ILP solution should satisfy the DirectedHamiltonianPath constraint"
    );
}

#[test]
fn test_directedhamiltonianpath_to_ilp_issue_example() {
    // 6-vertex example from issue #813
    let graph = DirectedGraph::new(
        6,
        vec![
            (0, 1),
            (0, 3),
            (1, 3),
            (1, 4),
            (2, 0),
            (2, 4),
            (3, 2),
            (3, 5),
            (4, 5),
            (5, 1),
        ],
    );
    let problem = DirectedHamiltonianPath::new(graph);
    let reduction: ReductionDirectedHamiltonianPathToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should find a path");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "ILP solution should be a valid directed Hamiltonian path"
    );
}

#[test]
fn test_directedhamiltonianpath_to_ilp_no_path() {
    // No Hamiltonian path: 0->1, 0->2, but no outgoing arcs from 1 or 2
    let graph = DirectedGraph::new(3, vec![(0, 1), (0, 2)]);
    let problem = DirectedHamiltonianPath::new(graph);
    let reduction: ReductionDirectedHamiltonianPathToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(reduction.target_problem());
    assert!(
        result.is_none(),
        "Graph with no Hamiltonian path should be infeasible"
    );
}

#[test]
fn test_directedhamiltonianpath_to_ilp_bf_vs_ilp() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = DirectedHamiltonianPath::new(graph);
    let reduction: ReductionDirectedHamiltonianPathToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
