#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use crate::models::algebraic::{Bounded, Comparison, ObjectiveSense, ILP};
use crate::models::set::IntegerKnapsack;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;

#[test]
fn test_integerknapsack_to_ilp_closed_loop() {
    let source = IntegerKnapsack::new(vec![3, 4, 5], vec![4, 5, 7], 10).unwrap();
    let reduction =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");

    assert_bf_vs_ilp(&source, &reduction);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, vec![0, 0, 2]);
}

#[test]
fn test_integerknapsack_to_ilp_structure() {
    let source = IntegerKnapsack::new(vec![3, 4, 5], vec![4, 5, 7], 10).unwrap();
    let reduction =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 3);
    assert_eq!(ilp.num_constraints(), 4);
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);
    assert_eq!(ilp.objective(), vec![(0, 4), (1, 5), (2, 7)]);

    let capacity = &ilp.constraints()[0];
    assert_eq!(capacity.comparison(), Comparison::Le);
    assert_eq!(capacity.rhs(), 10);
    assert_eq!(capacity.terms(), vec![(0, 3), (1, 4), (2, 5)]);

    let bounds: Vec<_> = ilp.constraints()[1..]
        .iter()
        .map(|constraint| {
            (
                constraint.terms().to_vec(),
                constraint.comparison(),
                constraint.rhs(),
            )
        })
        .collect();
    assert_eq!(
        bounds,
        vec![
            (vec![(0, 1)], Comparison::Le, 3),
            (vec![(1, 1)], Comparison::Le, 2),
            (vec![(2, 1)], Comparison::Le, 2),
        ]
    );
}

#[test]
fn test_integerknapsack_to_ilp_zero_capacity() {
    let source = IntegerKnapsack::new(vec![1, 2], vec![10, 20], 0).unwrap();
    let reduction =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("zero-capacity ILP should still be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, vec![0, 0]);
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
    assert_eq!(
        example.target.instance["variables"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        example.target.instance["constraints"]
            .as_array()
            .expect("constraints array")
            .len(),
        4
    );
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([0, 0, 2])
    );
    assert_eq!(
        example.solutions[0].target_config,
        serde_json::json!([0, 0, 2])
    );
}

#[test]
fn test_integer_knapsack_normalization_excludes_oversized_items_through_qubo() {
    use crate::models::algebraic::QUBO;
    use crate::traits::Problem;
    for capacity in [0, 2] {
        let source = IntegerKnapsack::new(vec![1, i64::MAX], vec![3, 4], capacity).unwrap();
        let ilp = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        assert!(ilp.target_problem().max_constraint_magnitude_bits() <= 2);
        let binary = ReduceTo::<ILP<bool>>::reduce_to(ilp.target_problem()).unwrap();
        let qubo = ReduceTo::<QUBO<i64>>::reduce_to(binary.target_problem()).unwrap();
        let solution = crate::solvers::BruteForce::new()
            .solve(qubo.target_problem())
            .unwrap()
            .unwrap();
        let recovered = ilp
            .extract_solution(
                &binary
                    .extract_solution(&qubo.extract_solution(&solution).unwrap())
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(recovered, vec![usize::try_from(capacity).unwrap(), 0]);
        assert_eq!(source.evaluate(&recovered).unwrap().0, Some(3 * capacity));
    }
}
