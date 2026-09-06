use super::*;
use crate::models::formula::CNFClause;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::Graph;
include!("../jl_helpers.rs");

#[test]
fn test_sat_to_minimumdominatingset_closed_loop() {
    // Simple SAT: (x1) - one variable, one clause
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");
    let ds_problem = reduction.target_problem();

    // Should have 3 vertices (variable gadget) + 1 clause vertex = 4 vertices
    assert_eq!(ds_problem.graph().num_vertices(), 4);

    // Edges: 3 for triangle + 1 from positive literal to clause = 4
    // Triangle edges: (0,1), (0,2), (1,2)
    // Clause edge: (0, 3) since x1 positive connects to clause vertex
    assert_eq!(ds_problem.graph().num_edges(), 4);
}

#[test]
fn test_two_variable_sat_to_ds() {
    // SAT: (x1 OR x2)
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, 2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");
    let ds_problem = reduction.target_problem();

    // 2 variables * 3 = 6 gadget vertices + 1 clause vertex = 7
    assert_eq!(ds_problem.graph().num_vertices(), 7);

    // Edges:
    // - 3 edges for first triangle: (0,1), (0,2), (1,2)
    // - 3 edges for second triangle: (3,4), (3,5), (4,5)
    // - 2 edges from literals to clause: (0,6), (3,6)
    assert_eq!(ds_problem.graph().num_edges(), 8);
}

#[test]
fn test_extract_solution_positive_literal() {
    // (x1) - select positive literal
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");

    // Solution: select vertex 0 (positive literal x1)
    // This dominates vertices 1, 2 (gadget) and vertex 3 (clause)
    let ds_sol = vec![true, false, false, false];
    let sat_sol = reduction.extract_solution(&ds_sol).unwrap();
    assert_eq!(sat_sol, vec![true]); // x1 = true
}

#[test]
fn test_extract_solution_negative_literal() {
    // (NOT x1) - select negative literal
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![-1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");

    // Solution: select vertex 1 (negative literal NOT x1)
    // This dominates vertices 0, 2 (gadget) and vertex 3 (clause)
    let ds_sol = vec![false, true, false, false];
    let sat_sol = reduction.extract_solution(&ds_sol).unwrap();
    assert_eq!(sat_sol, vec![false]); // x1 = false
}

#[test]
fn test_extract_solution_unused_variable() {
    // The unit clause x1 leaves x2 unused.
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");

    // Only x1 occurs, so its triangle is the only gadget. The unused x2
    // remains false in the extracted source assignment.
    let ds_sol = vec![true, false, false, false];
    let sat_sol = reduction.extract_solution(&ds_sol).unwrap();
    assert_eq!(sat_sol, vec![true, false]); // x1 = true, x2 = false (from dummy)
}

#[test]
fn test_ds_structure() {
    let sat = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");
    let ds_problem = reduction.target_problem();

    // 3 vars * 3 = 9 gadget vertices + 2 clause vertices = 11
    assert_eq!(ds_problem.graph().num_vertices(), 11);
}

#[test]
fn test_empty_sat() {
    // Empty SAT (trivially satisfiable)
    let sat = Satisfiability::new(0, vec![]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");
    let ds_problem = reduction.target_problem();

    assert_eq!(ds_problem.graph().num_vertices(), 0);
    assert_eq!(ds_problem.graph().num_edges(), 0);
    assert_eq!(reduction.num_clauses(), 0);
    assert_eq!(reduction.num_literals(), 0);
}

#[test]
fn test_multiple_literals_same_variable() {
    // Clause with repeated variable: (x1 OR NOT x1) - tautology
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1, -1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");
    let ds_problem = reduction.target_problem();

    // 3 gadget vertices + 1 clause vertex = 4
    assert_eq!(ds_problem.graph().num_vertices(), 4);

    // Edges:
    // - 3 for triangle
    // - 2 from literals to clause (both positive and negative literals connect)
    assert_eq!(ds_problem.graph().num_edges(), 5);
}

#[test]
fn test_accessors() {
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");

    assert_eq!(reduction.num_literals(), 2);
    assert_eq!(reduction.num_clauses(), 1);
}

#[test]
fn test_extract_solution_too_many_selected() {
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");

    let ds_sol = vec![true, true, false, false];
    assert_eq!(
        reduction.extract_solution(&ds_sol).unwrap_err().to_string(),
        "target dominating set does not certify satisfiability"
    );
}

#[test]
fn test_extract_solution_rejects_unselected_variable_gadget() {
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&vec![false, false, false, false])
            .unwrap_err()
            .to_string(),
        "target dominating set does not certify satisfiability"
    );
}

#[test]
fn test_extract_solution_rejects_selected_clause_vertex() {
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&vec![true, false, false, true])
            .unwrap_err()
            .to_string(),
        "target dominating set does not certify satisfiability"
    );
}

#[test]
fn test_negated_variable_connection() {
    // (NOT x1 OR NOT x2) - both negated
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![-1, -2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&sat)
        .expect("reduction should succeed");
    let ds_problem = reduction.target_problem();

    // 2 * 3 = 6 gadget vertices + 1 clause = 7
    assert_eq!(ds_problem.graph().num_vertices(), 7);

    // Edges:
    // - 3 for first triangle: (0,1), (0,2), (1,2)
    // - 3 for second triangle: (3,4), (3,5), (4,5)
    // - 2 from negated literals to clause: (1,6), (4,6)
    assert_eq!(ds_problem.graph().num_edges(), 8);
}

#[test]
fn test_jl_parity_sat_to_dominatingset() {
    let sat_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/satisfiability.json")).unwrap();
    let fixtures: &[(&str, &str)] = &[
        (
            include_str!("../../../tests/data/jl/satisfiability_to_dominatingset.json"),
            "simple_clause",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat01_to_dominatingset.json"),
            "rule_sat01",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat02_to_dominatingset.json"),
            "rule_sat02",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat03_to_dominatingset.json"),
            "rule_sat03",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat04_unsat_to_dominatingset.json"),
            "rule_sat04_unsat",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat07_to_dominatingset.json"),
            "rule_sat07",
        ),
    ];
    for (fixture_str, label) in fixtures {
        let data: serde_json::Value = serde_json::from_str(fixture_str).unwrap();
        let inst = &jl_find_instance_by_label(&sat_data, label)["instance"];
        let (num_vars, clauses) = jl_parse_sat_clauses(inst);
        let source = Satisfiability::new(num_vars, clauses);
        let result = ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&source)
            .expect("reduction should succeed");
        let solver = BruteForce::new();
        let sat_solutions: HashSet<Vec<bool>> = solver
            .find_all_witnesses(&source)
            .unwrap()
            .into_iter()
            .collect();
        for case in data["cases"].as_array().unwrap() {
            if sat_solutions.is_empty() {
                let target_solution = BruteForce::new()
                    .solve(result.target_problem())
                    .unwrap()
                    .expect("SAT->DS: target should have an optimal solution");
                assert!(result.extract_solution(&target_solution).is_err());
            } else {
                assert_satisfaction_round_trip_from_optimization_target(
                    &source,
                    &result,
                    &format!("SAT->DS [{label}]"),
                );
                assert_eq!(
                    sat_solutions,
                    jl_parse_bool_configs_set(&case["best_source"]),
                    "SAT->DS [{label}]: best source mismatch"
                );
            }
        }
    }
}

#[test]
fn test_sat_to_dominatingset_native_certificates() {
    use crate::traits::Problem;
    for (n, clauses) in [
        (0, vec![]),
        (0, vec![vec![]]),
        (1, vec![vec![1]]),
        (1, vec![vec![1], vec![-1]]),
        (2, vec![vec![2, -2, 2]]),
        (2, vec![vec![1, 2]]),
        (3, vec![vec![1, 3], vec![-1, -3]]),
        (3, vec![vec![1], vec![]]),
    ] {
        let source = Satisfiability::new(n, clauses.into_iter().map(CNFClause::new).collect());
        let result =
            ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&source).unwrap();
        let target = result.target_problem();
        assert!(std::ptr::eq(
            target,
            crate::rules::AggregateReductionResult::target_problem(&result)
        ));
        let mut accepted = false;
        for mask in 0..(1usize << target.num_vertices()) {
            let config: Vec<_> = (0..target.num_vertices())
                .map(|i| mask & (1 << i) != 0)
                .collect();
            let value = target.evaluate(&config).unwrap();
            let certificate = value == Min(Some(result.target_size));
            assert_eq!(
                crate::rules::AggregateReductionResult::extract_value(&result, value),
                Or(certificate)
            );
            match result.extract_solution(&config) {
                Ok(x) => {
                    assert!(certificate);
                    assert_eq!(source.evaluate(&x).unwrap(), Or(true));
                    accepted = true;
                }
                Err(_) => assert!(!certificate),
            }
        }
        assert_eq!(
            accepted,
            BruteForce::new().solve(&source).unwrap().is_some()
        );
        assert!(result
            .extract_solution(&vec![false; target.num_vertices() + 1])
            .is_err());
    }
}

#[test]
fn test_sat_to_dominatingset_sparse_declared_variables() {
    for clauses in [vec![], vec![CNFClause::new(vec![i64::MAX])]] {
        let source = Satisfiability::new(i64::MAX as usize, clauses);
        let result =
            ReduceTo::<MinimumDominatingSet<SimpleGraph, i64>>::reduce_to(&source).unwrap();
        assert_eq!(result.num_literals(), i64::MAX as usize);
        assert!(result.target_problem().num_vertices() <= 4);
        // Construction is compact. Extracting an i64::MAX-length source vector
        // is intentionally not attempted in a unit test.
    }
}

#[test]
fn test_sat_to_dominatingset_dimension_boundaries() {
    assert_eq!(ReductionSATToDS::target_dimensions(0, 0).unwrap(), (0, 0));
    assert_eq!(ReductionSATToDS::target_dimensions(2, 3).unwrap(), (9, 2));
    assert!(ReductionSATToDS::target_dimensions(usize::MAX, 0).is_err());
    assert!(ReductionSATToDS::target_dimensions(1, usize::MAX).is_err());
    if usize::BITS == 64 {
        let maximum = i64::MAX as usize;
        assert_eq!(
            ReductionSATToDS::target_dimensions(0, maximum).unwrap(),
            (maximum, 0)
        );
        assert!(ReductionSATToDS::target_dimensions(0, maximum + 1).is_err());
        let variables = maximum / 3;
        assert_eq!(
            ReductionSATToDS::target_dimensions(variables, maximum % 3).unwrap(),
            (maximum, variables as i64)
        );
        assert!(ReductionSATToDS::target_dimensions(variables, maximum % 3 + 1).is_err());
    }
}
