#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::models::set::IntegerKnapsack;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;

#[test]
fn test_integerknapsack_to_ilp_closed_loop() {
    let source = IntegerKnapsack::new(vec![3, 4, 5], vec![4, 5, 7], 10);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);

    assert_bf_vs_ilp(&source, &reduction);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0, 2]);
}

#[test]
fn test_integerknapsack_to_ilp_structure() {
    let source = IntegerKnapsack::new(vec![3, 4, 5], vec![4, 5, 7], 10);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 3);
    assert_eq!(ilp.num_constraints(), 4);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
    assert_eq!(ilp.objective, vec![(0, 4.0), (1, 5.0), (2, 7.0)]);

    let capacity = &ilp.constraints[0];
    assert_eq!(capacity.cmp, Comparison::Le);
    assert_eq!(capacity.rhs, 10.0);
    assert_eq!(capacity.terms, vec![(0, 3.0), (1, 4.0), (2, 5.0)]);

    let bounds: Vec<_> = ilp.constraints[1..]
        .iter()
        .map(|constraint| (constraint.terms.clone(), constraint.cmp, constraint.rhs))
        .collect();
    assert_eq!(
        bounds,
        vec![
            (vec![(0, 1.0)], Comparison::Le, 3.0),
            (vec![(1, 1.0)], Comparison::Le, 2.0),
            (vec![(2, 1.0)], Comparison::Le, 2.0),
        ]
    );
}

#[test]
fn test_integerknapsack_to_ilp_zero_capacity() {
    let source = IntegerKnapsack::new(vec![1, 2], vec![10, 20], 0);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("zero-capacity ILP should still be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0, 0]);
}

#[test]
#[should_panic(
    expected = "IntegerKnapsack -> ILP requires multiplicity bounds to fit in ILP<i32> variable bounds"
)]
fn test_integerknapsack_to_ilp_rejects_too_large_multiplicity_bounds() {
    let source = IntegerKnapsack::new(vec![1], vec![1], i32::MAX as i64 + 1);
    let _: super::ReductionIntegerKnapsackToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
}

#[cfg(feature = "example-db")]
#[test]
fn test_integerknapsack_to_ilp_canonical_example_spec() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "integerknapsack_to_ilp")
        .expect("missing canonical IntegerKnapsack -> ILP example spec")
        .build)();

    assert_eq!(example.source.problem, "IntegerKnapsack");
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(example.source.instance["capacity"], 10);
    assert_eq!(example.target.instance["num_vars"], 3);
    assert_eq!(
        example.target.instance["constraints"]
            .as_array()
            .expect("constraints array")
            .len(),
        4
    );
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(example.solutions[0].source_config, vec![0, 0, 2]);
    assert_eq!(example.solutions[0].target_config, vec![0, 0, 2]);
}
