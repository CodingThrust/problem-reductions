use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_maximumlikelihoodranking_to_ilp_closed_loop() {
    let matrix = vec![vec![0, 3, 2], vec![2, 0, 4], vec![3, 1, 0]];
    let problem = MaximumLikelihoodRanking::new(matrix);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "MaximumLikelihoodRanking->ILP closed loop",
    );
}

#[test]
fn test_maximumlikelihoodranking_to_ilp_structure() {
    let matrix = vec![vec![0, 3, 2], vec![2, 0, 4], vec![3, 1, 0]];
    let problem = MaximumLikelihoodRanking::new(matrix);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3 items -> C(3,2) = 3 variables
    assert_eq!(ilp.num_vars(), 3);
    // C(3,3) = 1 triple -> 2 constraints
    assert_eq!(ilp.num_constraints(), 2);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
}

#[test]
fn test_maximumlikelihoodranking_to_ilp_bf_vs_ilp() {
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_solutions = BruteForce::new().find_all_witnesses(&problem);
    let bf_value = problem.evaluate(&bf_solutions[0]);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    assert_eq!(bf_value, ilp_value);
    assert!(ilp_value.is_valid());
}

#[test]
fn test_maximumlikelihoodranking_to_ilp_extraction() {
    // 3 items: simple instance
    let matrix = vec![vec![0, 3, 2], vec![2, 0, 4], vec![3, 1, 0]];
    let problem = MaximumLikelihoodRanking::new(matrix);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Verify the extracted config is a valid permutation
    let n = problem.num_items();
    assert_eq!(extracted.len(), n);
    let mut sorted = extracted.clone();
    sorted.sort();
    assert_eq!(sorted, (0..n).collect::<Vec<_>>());

    // Verify evaluation is valid
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid());
}

#[test]
fn test_maximumlikelihoodranking_to_ilp_two_items() {
    let matrix = vec![vec![0, 5], vec![3, 0]];
    let problem = MaximumLikelihoodRanking::new(matrix);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 2 items -> 1 variable, 0 transitivity constraints
    assert_eq!(ilp.num_vars(), 1);
    assert_eq!(ilp.num_constraints(), 0);

    let ilp_solution = ILPSolver::new().solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    assert!(value.is_valid());

    // Optimal: item 0 before item 1 costs matrix[1][0]=3
    //          item 1 before item 0 costs matrix[0][1]=5
    // So optimal is [0,1] with cost 3
    assert_eq!(value, Min(Some(3)));
}

#[test]
fn test_maximumlikelihoodranking_to_ilp_single_item() {
    let problem = MaximumLikelihoodRanking::new(vec![vec![0]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 0);
    assert_eq!(ilp.num_constraints(), 0);

    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("single-item ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted, vec![0]);
}

#[test]
fn test_maximumlikelihoodranking_to_ilp_larger_instance() {
    // 4-item instance from the issue
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    let problem = MaximumLikelihoodRanking::new(matrix);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 4 items -> C(4,2) = 6 variables
    assert_eq!(ilp.num_vars(), 6);
    // C(4,3) = 4 triples -> 8 constraints
    assert_eq!(ilp.num_constraints(), 8);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "4-item MaximumLikelihoodRanking->ILP",
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_maximumlikelihoodranking_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "maximum_likelihood_ranking_to_ilp")
        .expect("missing canonical MaximumLikelihoodRanking -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "MaximumLikelihoodRanking");
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(example.target.instance["num_vars"], 3);
    assert!(!example.solutions.is_empty());
}
