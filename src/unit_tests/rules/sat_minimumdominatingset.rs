use super::*;
use crate::models::satisfiability::CNFClause;
use crate::solvers::BruteForce;
use crate::traits::Problem;
include!("../jl_helpers.rs");

#[test]
fn test_simple_sat_to_ds() {
    // Simple SAT: (x1) - one variable, one clause
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // Should have 3 vertices (variable gadget) + 1 clause vertex = 4 vertices
    assert_eq!(ds_problem.num_vertices(), 4);

    // Edges: 3 for triangle + 1 from positive literal to clause = 4
    // Triangle edges: (0,1), (0,2), (1,2)
    // Clause edge: (0, 3) since x1 positive connects to clause vertex
    assert_eq!(ds_problem.num_edges(), 4);
}

#[test]
fn test_two_variable_sat_to_ds() {
    // SAT: (x1 OR x2)
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, 2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 2 variables * 3 = 6 gadget vertices + 1 clause vertex = 7
    assert_eq!(ds_problem.num_vertices(), 7);

    // Edges:
    // - 3 edges for first triangle: (0,1), (0,2), (1,2)
    // - 3 edges for second triangle: (3,4), (3,5), (4,5)
    // - 2 edges from literals to clause: (0,6), (3,6)
    assert_eq!(ds_problem.num_edges(), 8);
}

#[test]
fn test_extract_solution_positive_literal() {
    // (x1) - select positive literal
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Solution: select vertex 0 (positive literal x1)
    // This dominates vertices 1, 2 (gadget) and vertex 3 (clause)
    let ds_sol = vec![1, 0, 0, 0];
    let sat_sol = reduction.extract_solution(&ds_sol);
    assert_eq!(sat_sol, vec![1]); // x1 = true
}

#[test]
fn test_extract_solution_negative_literal() {
    // (NOT x1) - select negative literal
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![-1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Solution: select vertex 1 (negative literal NOT x1)
    // This dominates vertices 0, 2 (gadget) and vertex 3 (clause)
    let ds_sol = vec![0, 1, 0, 0];
    let sat_sol = reduction.extract_solution(&ds_sol);
    assert_eq!(sat_sol, vec![0]); // x1 = false
}

#[test]
fn test_extract_solution_dummy() {
    // (x1 OR x2) where only x1 matters
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Select: vertex 0 (x1 positive) and vertex 5 (x2 dummy)
    // Vertex 0 dominates: itself, 1, 2, and clause 6
    // Vertex 5 dominates: 3, 4, and itself
    let ds_sol = vec![1, 0, 0, 0, 0, 1, 0];
    let sat_sol = reduction.extract_solution(&ds_sol);
    assert_eq!(sat_sol, vec![1, 0]); // x1 = true, x2 = false (from dummy)
}

#[test]
fn test_ds_structure() {
    let sat = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 3 vars * 3 = 9 gadget vertices + 2 clause vertices = 11
    assert_eq!(ds_problem.num_vertices(), 11);
}

#[test]
fn test_empty_sat() {
    // Empty SAT (trivially satisfiable)
    let sat = Satisfiability::new(0, vec![]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    assert_eq!(ds_problem.num_vertices(), 0);
    assert_eq!(ds_problem.num_edges(), 0);
    assert_eq!(reduction.num_clauses(), 0);
    assert_eq!(reduction.num_literals(), 0);
}

#[test]
fn test_multiple_literals_same_variable() {
    // Clause with repeated variable: (x1 OR NOT x1) - tautology
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1, -1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 3 gadget vertices + 1 clause vertex = 4
    assert_eq!(ds_problem.num_vertices(), 4);

    // Edges:
    // - 3 for triangle
    // - 2 from literals to clause (both positive and negative literals connect)
    assert_eq!(ds_problem.num_edges(), 5);
}

#[test]
fn test_accessors() {
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    assert_eq!(reduction.num_literals(), 2);
    assert_eq!(reduction.num_clauses(), 1);
}

#[test]
fn test_extract_solution_too_many_selected() {
    // Test that extract_solution handles invalid (non-minimal) dominating sets
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Select all 4 vertices (more than num_literals=1)
    let ds_sol = vec![1, 1, 1, 1];
    let sat_sol = reduction.extract_solution(&ds_sol);
    // Should return default (all false)
    assert_eq!(sat_sol, vec![0]);
}

#[test]
fn test_negated_variable_connection() {
    // (NOT x1 OR NOT x2) - both negated
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![-1, -2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 2 * 3 = 6 gadget vertices + 1 clause = 7
    assert_eq!(ds_problem.num_vertices(), 7);

    // Edges:
    // - 3 for first triangle: (0,1), (0,2), (1,2)
    // - 3 for second triangle: (3,4), (3,5), (4,5)
    // - 2 from negated literals to clause: (1,6), (4,6)
    assert_eq!(ds_problem.num_edges(), 8);
}

#[test]
fn test_jl_parity_sat_to_dominatingset() {
    let sat_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/satisfiability.json")).unwrap();
    let fixtures: &[(&str, &str)] = &[
        (include_str!("../../../tests/data/jl/satisfiability_to_dominatingset.json"), "simple_clause"),
        (include_str!("../../../tests/data/jl/rule_sat01_to_dominatingset.json"), "rule_sat01"),
        (include_str!("../../../tests/data/jl/rule_sat02_to_dominatingset.json"), "rule_sat02"),
        (include_str!("../../../tests/data/jl/rule_sat03_to_dominatingset.json"), "rule_sat03"),
        (include_str!("../../../tests/data/jl/rule_sat04_unsat_to_dominatingset.json"), "rule_sat04_unsat"),
        (include_str!("../../../tests/data/jl/rule_sat07_to_dominatingset.json"), "rule_sat07"),
    ];
    for (fixture_str, label) in fixtures {
        let data: serde_json::Value = serde_json::from_str(fixture_str).unwrap();
        let inst = &jl_find_instance_by_label(&sat_data, label)["instance"];
        let (num_vars, clauses) = jl_parse_sat_clauses(inst);
        let source = Satisfiability::new(num_vars, clauses);
        let result = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&source);
        let solver = BruteForce::new();
        let best_target = solver.find_all_best(result.target_problem());
        let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
        let sat_solutions: HashSet<Vec<usize>> = solver.find_all_satisfying(&source).into_iter().collect();
        for case in data["cases"].as_array().unwrap() {
            if sat_solutions.is_empty() {
                for sol in &extracted {
                    assert!(!source.evaluate(sol), "SAT->DS [{label}]: unsatisfiable but extracted satisfies");
                }
            } else {
                assert!(extracted.is_subset(&sat_solutions), "SAT->DS [{label}]: extracted not subset");
                assert_eq!(sat_solutions, jl_parse_configs_set(&case["best_source"]),
                    "SAT->DS [{label}]: best source mismatch");
            }
        }
    }
}
