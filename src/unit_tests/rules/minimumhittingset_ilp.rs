use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = MinimumHittingSet::new(3, vec![vec![0, 1], vec![1, 2]]);
    let reduction: ReductionHSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 3, "one var per universe element");
    assert_eq!(ilp.constraints.len(), 2, "one constraint per set");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimumhittingset_to_ilp_bf_vs_ilp() {
    let problem = MinimumHittingSet::new(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]);
    let reduction: ReductionHSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
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
    let problem = MinimumHittingSet::new(3, vec![vec![0, 1], vec![1, 2]]);
    let reduction: ReductionHSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = vec![0, 1, 0];
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1, 0]);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimumhittingset_to_ilp_trivial() {
    let problem = MinimumHittingSet::new(0, vec![]);
    let reduction: ReductionHSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);
}
