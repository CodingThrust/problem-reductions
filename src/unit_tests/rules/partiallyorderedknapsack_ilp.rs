use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::models::misc::PartiallyOrderedKnapsack;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_reduction_creates_valid_ilp() {
    // 3 items, weights [2,3,1], values [3,4,2], capacity 4, precedence (0,1)
    // Expected: 3 vars, 2 constraints (1 capacity + 1 precedence), Maximize
    let problem = PartiallyOrderedKnapsack::new(vec![2, 3, 1], vec![3, 4, 2], vec![(0, 1)], 4);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 3, "one variable per item");
    assert_eq!(
        ilp.constraints.len(),
        2,
        "one capacity constraint + one precedence constraint"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);

    // Check capacity constraint: Σ w_i·x_i ≤ 4
    let cap = &ilp.constraints[0];
    assert_eq!(cap.cmp, Comparison::Le);
    assert_eq!(cap.rhs, 4.0);
    assert_eq!(cap.terms, vec![(0, 2.0), (1, 3.0), (2, 1.0)]);

    // Check precedence constraint for (0,1): x_1 - x_0 ≤ 0
    let prec = &ilp.constraints[1];
    assert_eq!(prec.cmp, Comparison::Le);
    assert_eq!(prec.rhs, 0.0);
    assert_eq!(prec.terms, vec![(1, 1.0), (0, -1.0)]);

    // Check objective: maximize Σ v_i·x_i
    assert_eq!(ilp.objective, vec![(0, 3.0), (1, 4.0), (2, 2.0)]);
}

#[test]
fn test_partiallyorderedknapsack_to_ilp_bf_vs_ilp() {
    // Instance with multiple precedences; compare BruteForce and ILP results
    let problem = PartiallyOrderedKnapsack::new(
        vec![2, 3, 4, 1, 2, 3],
        vec![3, 2, 5, 4, 3, 8],
        vec![(0, 2), (0, 3), (1, 4), (3, 5), (4, 5)],
        11,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "PartiallyOrderedKnapsack->ILP closed loop",
    );

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Max(Some(20)),
        "ILP solution should yield optimal value 20"
    );
}

#[test]
fn test_solution_extraction() {
    // Verify that extracted solution correctly respects precedence constraints
    let problem = PartiallyOrderedKnapsack::new(vec![2, 3, 1], vec![3, 4, 2], vec![(0, 1)], 4);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    // Select items 0 and 2: weight=3≤4, values=5, precedence satisfied (item 1 not selected)
    let target_solution = vec![1, 0, 1];
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted, vec![1, 0, 1]);
    assert_eq!(problem.evaluate(&extracted), Max(Some(5)));
}

#[test]
fn test_partiallyorderedknapsack_to_ilp_trivial() {
    // No items: should produce a trivial ILP with one capacity constraint only
    let problem = PartiallyOrderedKnapsack::new(vec![], vec![], vec![], 10);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 0);
    assert_eq!(ilp.constraints.len(), 1, "capacity constraint only");
    assert_eq!(ilp.constraints[0].cmp, Comparison::Le);
    assert_eq!(ilp.constraints[0].rhs, 10.0);
    assert!(ilp.constraints[0].terms.is_empty());
    assert!(ilp.objective.is_empty());

    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("trivial ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, Vec::<usize>::new());
}

#[cfg(feature = "example-db")]
#[test]
fn test_partiallyorderedknapsack_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "partiallyorderedknapsack_to_ilp")
        .expect("missing canonical PartiallyOrderedKnapsack -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "PartiallyOrderedKnapsack");
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(example.source.instance["capacity"], 4);
    assert_eq!(example.target.instance["num_vars"], 3);
    assert_eq!(
        example.target.instance["constraints"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        example.solutions,
        vec![crate::export::SolutionPair {
            source_config: vec![1, 0, 1],
            target_config: vec![1, 0, 1],
        }]
    );
}
