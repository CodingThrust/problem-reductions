use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::ILPSolver;

#[test]
fn test_knapsack_to_ilp_closed_loop() {
    let knapsack = Knapsack::new(vec![1, 3, 4, 5], vec![1, 4, 5, 7], 7);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack);

    assert_optimization_round_trip_from_optimization_target(
        &knapsack,
        &reduction,
        "Knapsack->ILP closed loop",
    );

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 1, 1, 0]);
}

#[test]
fn test_knapsack_to_ilp_structure() {
    let knapsack = Knapsack::new(vec![1, 3, 4, 5], vec![1, 4, 5, 7], 7);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 4);
    assert_eq!(ilp.num_constraints(), 1);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
    assert_eq!(ilp.objective, vec![(0, 1.0), (1, 4.0), (2, 5.0), (3, 7.0)]);

    let constraint = &ilp.constraints[0];
    assert_eq!(constraint.cmp, Comparison::Le);
    assert_eq!(constraint.rhs, 7.0);
    assert_eq!(
        constraint.terms,
        vec![(0, 1.0), (1, 3.0), (2, 4.0), (3, 5.0)]
    );
}

#[test]
fn test_knapsack_to_ilp_zero_capacity() {
    let knapsack = Knapsack::new(vec![2, 3], vec![5, 7], 0);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("zero-capacity ILP should still be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0]);
}

#[test]
fn test_knapsack_to_ilp_empty_instance() {
    let knapsack = Knapsack::new(vec![], vec![], 0);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 0);
    assert_eq!(ilp.num_constraints(), 1);
    assert_eq!(ilp.constraints[0].cmp, Comparison::Le);
    assert_eq!(ilp.constraints[0].rhs, 0.0);
    assert!(ilp.constraints[0].terms.is_empty());
    assert!(ilp.objective.is_empty());

    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("empty Knapsack ILP should still be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, Vec::<usize>::new());
}

#[cfg(feature = "example-db")]
#[test]
fn test_knapsack_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "knapsack_to_ilp")
        .expect("missing canonical Knapsack -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "Knapsack");
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(example.source.instance["capacity"], 7);
    assert_eq!(example.target.instance["num_vars"], 4);
    assert_eq!(
        example.target.instance["constraints"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        example.solutions,
        vec![crate::export::SolutionPair {
            source_config: vec![0, 1, 1, 0],
            target_config: vec![0, 1, 1, 0],
        }]
    );
}
