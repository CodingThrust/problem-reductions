use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_knapsack_to_ilp_closed_loop() {
    let knapsack = Knapsack::new(vec![1, 3, 4, 5], vec![1, 4, 5, 7], 7);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack).expect("reduction should succeed");

    assert_bf_vs_ilp(&knapsack, &reduction);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, vec![false, true, true, false]);
}

#[test]
fn test_knapsack_to_ilp_bf_vs_ilp() {
    let knapsack = Knapsack::new(vec![1, 3, 4, 5], vec![1, 4, 5, 7], 7);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack).expect("reduction should succeed");

    let bf_solutions = BruteForce::new().find_all_witnesses(&knapsack).unwrap();
    let bf_value = knapsack.evaluate(&bf_solutions[0]).unwrap();

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_value = knapsack.evaluate(&extracted).unwrap();

    assert_eq!(bf_value, ilp_value);
    assert!(ilp_value.is_valid());
}

#[test]
fn test_knapsack_to_ilp_structure() {
    let knapsack = Knapsack::new(vec![1, 3, 4, 5], vec![1, 4, 5, 7], 7);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 4);
    assert_eq!(ilp.num_constraints(), 1);
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize);
    assert_eq!(ilp.objective(), vec![(0, 1), (1, 4), (2, 5), (3, 7)]);

    let constraint = &ilp.constraints()[0];
    assert_eq!(constraint.comparison(), Comparison::Le);
    assert_eq!(constraint.rhs(), 7);
    assert_eq!(constraint.terms(), vec![(0, 1), (1, 3), (2, 4), (3, 5)]);
}

#[test]
fn test_knapsack_to_ilp_zero_capacity() {
    let knapsack = Knapsack::new(vec![2, 3], vec![5, 7], 0);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack).expect("reduction should succeed");

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("zero-capacity ILP should still be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, vec![false, false]);
}

#[test]
fn test_knapsack_to_ilp_empty_instance() {
    let knapsack = Knapsack::new(vec![], vec![], 0);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 0);
    assert_eq!(ilp.num_constraints(), 1);
    assert_eq!(ilp.constraints()[0].comparison(), Comparison::Le);
    assert_eq!(ilp.constraints()[0].rhs(), 0);
    assert!(ilp.constraints()[0].terms().is_empty());
    assert!(ilp.objective().is_empty());

    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("empty Knapsack ILP should still be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(extracted, Vec::<bool>::new());
}

#[test]
fn test_knapsack_to_ilp_preserves_large_exact_weight() {
    let weight = crate::types::MAX_EXACT_F64_INTEGER + 1;
    let knapsack = Knapsack::new(vec![weight], vec![1], weight);

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&knapsack).unwrap();
    let constraint = &reduction.target_problem().constraints()[0];
    assert_eq!(constraint.terms(), vec![(0, weight)]);
    assert_eq!(constraint.rhs(), weight);
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
    assert_eq!(
        example.target.instance["variables"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
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
            source_config: serde_json::json!(vec![false, true, true, false]),
            target_config: serde_json::json!(vec![0, 1, 1, 0]),
        }]
    );
}
