use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::models::formula::CNFClause;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

fn make_canonical_instance() -> Maximum2Satisfiability {
    Maximum2Satisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![1, -2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-1, -3]),
            CNFClause::new(vec![2, 4]),
            CNFClause::new(vec![-3, -4]),
            CNFClause::new(vec![3, 4]),
        ],
    )
}

#[test]
fn test_maximum2satisfiability_to_ilp_closed_loop() {
    let problem = make_canonical_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_optimization_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "Maximum2Satisfiability->ILP closed loop",
    );

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    // Optimal: 6 satisfied clauses
    let value = problem.evaluate(&extracted);
    assert_eq!(value, crate::types::Max(Some(6)));
}

#[test]
fn test_maximum2satisfiability_to_ilp_bf_vs_ilp() {
    let problem = make_canonical_instance();
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
fn test_maximum2satisfiability_to_ilp_structure() {
    let problem = make_canonical_instance();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 4 truth variables + 7 clause indicators = 11 ILP variables
    assert_eq!(ilp.num_vars(), 11);
    // One constraint per clause
    assert_eq!(ilp.num_constraints(), 7);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);

    // Objective: maximize sum of z_4..z_10
    let expected_objective: Vec<(usize, f64)> = (4..11).map(|j| (j, 1.0)).collect();
    assert_eq!(ilp.objective, expected_objective);

    // Check first constraint: clause (x1 OR x2) -> z_4 - y_0 - y_1 <= 0
    let c0 = &ilp.constraints[0];
    assert_eq!(c0.cmp, Comparison::Le);
    assert_eq!(c0.rhs, 0.0); // 0 negated literals
    assert_eq!(c0.terms, vec![(4, 1.0), (0, -1.0), (1, -1.0)]);

    // Check constraint for clause (~x1 OR x3) -> z_6 + y_0 - y_2 <= 1
    let c2 = &ilp.constraints[2];
    assert_eq!(c2.cmp, Comparison::Le);
    assert_eq!(c2.rhs, 1.0); // 1 negated literal
    assert_eq!(c2.terms, vec![(6, 1.0), (0, 1.0), (2, -1.0)]);

    // Check constraint for clause (~x1 OR ~x3) -> z_7 + y_0 + y_2 <= 2
    let c3 = &ilp.constraints[3];
    assert_eq!(c3.cmp, Comparison::Le);
    assert_eq!(c3.rhs, 2.0); // 2 negated literals
    assert_eq!(c3.terms, vec![(7, 1.0), (0, 1.0), (2, 1.0)]);
}

#[test]
fn test_maximum2satisfiability_to_ilp_all_satisfiable() {
    // Simple instance where all clauses can be satisfied: (x1 OR x2) AND (x1 OR ~x2)
    // x1 = true satisfies both.
    let problem = Maximum2Satisfiability::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![1, -2])],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let value = problem.evaluate(&extracted);
    // Both clauses should be satisfiable
    assert_eq!(value, crate::types::Max(Some(2)));
}

#[cfg(feature = "example-db")]
#[test]
fn test_maximum2satisfiability_to_ilp_canonical_example_spec() {
    let spec = canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "maximum2satisfiability_to_ilp")
        .expect("missing canonical Maximum2Satisfiability -> ILP example spec");
    let example = (spec.build)();

    assert_eq!(example.source.problem, "Maximum2Satisfiability");
    assert_eq!(example.target.problem, "ILP");
    assert_eq!(example.source.instance["num_vars"], 4);
    assert_eq!(example.target.instance["num_vars"], 11);
    assert_eq!(
        example.target.instance["constraints"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert_eq!(
        example.solutions,
        vec![crate::export::SolutionPair {
            source_config: vec![1, 1, 0, 1],
            target_config: vec![1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1],
        }]
    );
}
