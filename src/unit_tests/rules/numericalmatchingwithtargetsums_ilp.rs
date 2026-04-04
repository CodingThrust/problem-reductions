use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_numericalmatchingwithtargetsums_to_ilp_closed_loop() {
    let problem =
        NumericalMatchingWithTargetSums::new(vec![1, 4, 7], vec![2, 5, 3], vec![3, 7, 12]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "NMTS->ILP closed loop",
    );

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_numericalmatchingwithtargetsums_to_ilp_bf_vs_ilp() {
    let problem =
        NumericalMatchingWithTargetSums::new(vec![1, 4, 7], vec![2, 5, 3], vec![3, 7, 12]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_numericalmatchingwithtargetsums_to_ilp_structure() {
    let problem =
        NumericalMatchingWithTargetSums::new(vec![1, 4, 7], vec![2, 5, 3], vec![3, 7, 12]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // Only compatible triples are created as variables
    // Check that we have 3m = 9 constraints (3 for x, 3 for y, 3 for targets)
    assert_eq!(ilp.num_constraints(), 9);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    // Feasibility: empty objective
    assert!(ilp.objective.is_empty());

    // All constraints should be equality constraints
    for c in &ilp.constraints {
        assert_eq!(c.cmp, Comparison::Eq);
        assert!((c.rhs - 1.0).abs() < 1e-9);
    }
}

#[test]
fn test_numericalmatchingwithtargetsums_to_ilp_unsatisfiable() {
    // m=2, no valid matching: sums {1+3,2+4}={4,6} or {1+4,2+3}={5,5}, neither = {10,20}
    let problem = NumericalMatchingWithTargetSums::new(vec![1, 2], vec![3, 4], vec![10, 20]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let result = ILPSolver::new().solve(reduction.target_problem());
    assert!(
        result.is_none(),
        "Unsatisfiable instance should have no ILP solution"
    );
}

#[test]
fn test_numericalmatchingwithtargetsums_to_ilp_single_pair() {
    let problem = NumericalMatchingWithTargetSums::new(vec![5], vec![3], vec![8]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 1 compatible triple: (0,0,0) since 5+3=8
    assert_eq!(ilp.num_vars(), 1);
    assert_eq!(ilp.num_constraints(), 3); // 3*1

    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("single-pair ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0]);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_numericalmatchingwithtargetsums_to_ilp_compatible_triples_only() {
    // Verify that only compatible triples generate variables
    // m=2, sizes_x=[1,2], sizes_y=[3,4], targets=[4,6]
    // Compatible: (0,0,0): 1+3=4, (1,1,1): 2+4=6
    // Also (0,1,1): 1+4=5≠6, (1,0,0): 2+3=5≠4 — NOT compatible
    // Wait: (0,1,0): 1+4=5≠4, (1,0,1): 2+3=5≠6 — also not
    // So only 2 variables
    let problem = NumericalMatchingWithTargetSums::new(vec![1, 2], vec![3, 4], vec![4, 6]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 2);

    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1]);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[cfg(feature = "example-db")]
#[test]
fn test_numericalmatchingwithtargetsums_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "numericalmatchingwithtargetsums_to_ilp")
        .expect("missing canonical NMTS -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "NumericalMatchingWithTargetSums");
    assert_eq!(example.target.problem, "ILP");
    assert!(!example.solutions.is_empty());
}
