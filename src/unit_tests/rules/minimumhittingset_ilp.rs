use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::set::MinimumHittingSet;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = MinimumHittingSet::new(3, vec![vec![0, 1], vec![1, 2]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 3, "one var per universe element");
    assert_eq!(ilp.constraints.len(), 2, "one constraint per set");
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_minimumhittingset_to_ilp_closed_loop() {
    let problem = MinimumHittingSet::new(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "MinimumHittingSet->ILP closed loop",
    );
}

#[test]
fn test_solution_extraction() {
    let problem = MinimumHittingSet::new(3, vec![vec![0, 1], vec![1, 2]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = vec![0, 1, 0]; // select element 1 (hits both sets)
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1, 0]);
    assert!(problem.evaluate(&extracted).is_valid());
}

#[test]
fn test_minimumhittingset_to_ilp_trivial() {
    let problem = MinimumHittingSet::new(0, vec![]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 0);
}
