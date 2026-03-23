use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;


#[test]
fn test_reduction_creates_valid_ilp() {
    // 3 items, weights [2,3,1], values [3,4,2], capacity 4, precedence (0,1)
    let problem = PartiallyOrderedKnapsack::new(vec![2, 3, 1], vec![3, 4, 2], vec![(0, 1)], 4);
    let reduction: ReductionPOKToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 3);
    assert_eq!(ilp.constraints.len(), 2); // 1 capacity + 1 precedence
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
}

#[test]
fn test_partiallyorderedknapsack_to_ilp_bf_vs_ilp() {
    let problem = PartiallyOrderedKnapsack::new(vec![2, 3, 1], vec![3, 4, 2], vec![(0, 1)], 4);
    let reduction: ReductionPOKToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
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
    let problem = PartiallyOrderedKnapsack::new(vec![2, 3, 1], vec![3, 4, 2], vec![(0, 1)], 4);
    let reduction: ReductionPOKToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_partiallyorderedknapsack_to_ilp_trivial() {
    let problem = PartiallyOrderedKnapsack::new(vec![], vec![], vec![], 0);
    let reduction: ReductionPOKToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 1); // capacity only
}
