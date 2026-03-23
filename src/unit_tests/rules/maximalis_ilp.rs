use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Path P3: 0-1-2
    let problem = MaximalIS::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 1, 1]);
    let reduction: ReductionMxISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 3);
    assert_eq!(ilp.constraints.len(), 5); // 2 edges + 3 maximality
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
}

#[test]
fn test_maximalis_to_ilp_bf_vs_ilp() {
    let problem = MaximalIS::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1, 1],
    );
    let reduction: ReductionMxISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    assert!(ilp_value.is_valid());
}

#[test]
fn test_solution_extraction() {
    let problem = MaximalIS::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 1, 1]);
    let reduction: ReductionMxISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_maximalis_to_ilp_trivial() {
    // Single vertex
    let problem = MaximalIS::new(SimpleGraph::new(1, vec![]), vec![1]);
    let reduction: ReductionMxISToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 1);
    assert_eq!(ilp.constraints.len(), 1); // 0 edges + 1 maximality
}
