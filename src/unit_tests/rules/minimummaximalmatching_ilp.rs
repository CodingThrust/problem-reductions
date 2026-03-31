use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Path P4: 4 vertices, 3 edges
    let problem = MinimumMaximalMatching::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionMMMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_vars = num_edges = 3
    assert_eq!(ilp.num_vars, 3, "Should have one variable per edge");
    // num_constraints = num_vertices (with degree >= 1) + num_edges
    // Vertices 0,1,2,3 all have degree >= 1 → 4 matching constraints + 3 maximality constraints
    assert_eq!(ilp.constraints.len(), 7);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize, "Should minimize");
}

#[test]
fn test_minimummaximalmatching_to_ilp_closed_loop() {
    // Path P4: optimal minimum maximal matching = 1 edge (center edge (1,2)).
    let problem = MinimumMaximalMatching::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let reduction: ReductionMMMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solution = bf.find_witness(&problem).unwrap();
    let bf_value = problem.evaluate(&bf_solution);

    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, Min(Some(1)));
    assert_eq!(ilp_value, Min(Some(1)));
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimummaximalmatching_to_ilp_path_p6() {
    // Path P6: optimal = 2 edges.
    let problem = MinimumMaximalMatching::new(SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)],
    ));
    let reduction: ReductionMMMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&extracted), Min(Some(2)));
}

#[test]
fn test_minimummaximalmatching_to_ilp_triangle() {
    // Triangle: optimal = 1 (any single edge is maximal).
    let problem = MinimumMaximalMatching::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let reduction: ReductionMMMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&extracted), Min(Some(1)));
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimummaximalmatching_to_ilp_bf_vs_ilp() {
    let problem = MinimumMaximalMatching::new(SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)],
    ));
    let reduction: ReductionMMMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_empty_graph() {
    let problem = MinimumMaximalMatching::new(SimpleGraph::new(3, vec![]));
    let reduction: ReductionMMMToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);
    assert!(problem.evaluate(&[]).is_valid());
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}
