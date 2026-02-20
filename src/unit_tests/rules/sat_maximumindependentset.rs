use super::*;
use crate::models::satisfiability::CNFClause;
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::traits::Problem;
include!("../jl_helpers.rs");

#[test]
fn test_boolvar_creation() {
    let var = BoolVar::new(0, false);
    assert_eq!(var.name, 0);
    assert!(!var.neg);

    let neg_var = BoolVar::new(1, true);
    assert_eq!(neg_var.name, 1);
    assert!(neg_var.neg);
}

#[test]
fn test_boolvar_from_literal() {
    // Positive literal: variable 1 (1-indexed) -> variable 0 (0-indexed), not negated
    let var = BoolVar::from_literal(1);
    assert_eq!(var.name, 0);
    assert!(!var.neg);

    // Negative literal: variable 2 (1-indexed) -> variable 1 (0-indexed), negated
    let neg_var = BoolVar::from_literal(-2);
    assert_eq!(neg_var.name, 1);
    assert!(neg_var.neg);
}

#[test]
fn test_boolvar_complement() {
    let x = BoolVar::new(0, false);
    let not_x = BoolVar::new(0, true);
    let y = BoolVar::new(1, false);

    assert!(x.is_complement(&not_x));
    assert!(not_x.is_complement(&x));
    assert!(!x.is_complement(&y));
    assert!(!x.is_complement(&x));
}

#[test]
fn test_sat_to_maximumindependentset_closed_loop() {
    // Simple SAT: (x1) - one clause with one literal
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is_problem = reduction.target_problem();

    // Should have 1 vertex (one literal)
    assert_eq!(is_problem.graph().num_vertices(), 1);
    // No edges (single vertex can't form a clique)
    assert_eq!(is_problem.graph().num_edges(), 0);
}

#[test]
fn test_two_clause_sat_to_is() {
    // SAT: (x1) AND (NOT x1)
    // This is unsatisfiable
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is_problem = reduction.target_problem();

    // Should have 2 vertices
    assert_eq!(is_problem.graph().num_vertices(), 2);
    // Should have 1 edge (between x1 and NOT x1)
    assert_eq!(is_problem.graph().num_edges(), 1);

    // Maximum IS should have size 1 (can't select both)
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(is_problem);
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
}

#[test]
fn test_extract_solution_basic() {
    // Simple case: (x1 OR x2)
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, 2])]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Select vertex 0 (literal x1)
    let is_sol = vec![1, 0];
    let sat_sol = reduction.extract_solution(&is_sol);
    assert_eq!(sat_sol, vec![1, 0]); // x1=true, x2=false

    // Select vertex 1 (literal x2)
    let is_sol = vec![0, 1];
    let sat_sol = reduction.extract_solution(&is_sol);
    assert_eq!(sat_sol, vec![0, 1]); // x1=false, x2=true
}

#[test]
fn test_extract_solution_with_negation() {
    // (NOT x1) - selecting NOT x1 means x1 should be false
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![-1])]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);

    let is_sol = vec![1];
    let sat_sol = reduction.extract_solution(&is_sol);
    assert_eq!(sat_sol, vec![0]); // x1=false (so NOT x1 is true)
}

#[test]
fn test_clique_edges_in_clause() {
    // A clause with 3 literals should form a clique (3 edges)
    let sat = Satisfiability::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is_problem = reduction.target_problem();

    // 3 vertices, 3 edges (complete graph K3)
    assert_eq!(is_problem.graph().num_vertices(), 3);
    assert_eq!(is_problem.graph().num_edges(), 3);
}

#[test]
fn test_complement_edges_across_clauses() {
    // (x1) AND (NOT x1) AND (x2) - three clauses
    // Vertices: 0 (x1), 1 (NOT x1), 2 (x2)
    // Edges: (0,1) for complement x1 and NOT x1
    let sat = Satisfiability::new(
        2,
        vec![
            CNFClause::new(vec![1]),
            CNFClause::new(vec![-1]),
            CNFClause::new(vec![2]),
        ],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is_problem = reduction.target_problem();

    assert_eq!(is_problem.graph().num_vertices(), 3);
    assert_eq!(is_problem.graph().num_edges(), 1); // Only the complement edge
}

#[test]
fn test_is_structure() {
    let sat = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is_problem = reduction.target_problem();

    // IS should have vertices for literals in clauses
    assert_eq!(is_problem.graph().num_vertices(), 4); // 2 + 2 literals
}

#[test]
fn test_empty_sat() {
    // Empty SAT (trivially satisfiable)
    let sat = Satisfiability::new(0, vec![]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is_problem = reduction.target_problem();

    assert_eq!(is_problem.graph().num_vertices(), 0);
    assert_eq!(is_problem.graph().num_edges(), 0);
    assert_eq!(reduction.num_clauses(), 0);
}

#[test]
fn test_literals_accessor() {
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);

    let literals = reduction.literals();
    assert_eq!(literals.len(), 2);
    assert_eq!(literals[0], BoolVar::new(0, false)); // x1
    assert_eq!(literals[1], BoolVar::new(1, true)); // NOT x2
}

#[test]
fn test_jl_parity_sat_to_independentset() {
    let sat_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/satisfiability.json")).unwrap();
    let fixtures: &[(&str, &str)] = &[
        (
            include_str!("../../../tests/data/jl/satisfiability_to_independentset.json"),
            "simple_clause",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat01_to_independentset.json"),
            "rule_sat01",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat02_to_independentset.json"),
            "rule_sat02",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat03_to_independentset.json"),
            "rule_sat03",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat04_unsat_to_independentset.json"),
            "rule_sat04_unsat",
        ),
        (
            include_str!("../../../tests/data/jl/rule_sat07_to_independentset.json"),
            "rule_sat07",
        ),
    ];
    for (fixture_str, label) in fixtures {
        let data: serde_json::Value = serde_json::from_str(fixture_str).unwrap();
        let inst = &jl_find_instance_by_label(&sat_data, label)["instance"];
        let (num_vars, clauses) = jl_parse_sat_clauses(inst);
        let source = Satisfiability::new(num_vars, clauses);
        let result = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
        let solver = BruteForce::new();
        let best_target = solver.find_all_best(result.target_problem());
        let extracted: HashSet<Vec<usize>> = best_target
            .iter()
            .map(|t| result.extract_solution(t))
            .collect();
        let sat_solutions: HashSet<Vec<usize>> =
            solver.find_all_satisfying(&source).into_iter().collect();
        for case in data["cases"].as_array().unwrap() {
            if sat_solutions.is_empty() {
                for sol in &extracted {
                    assert!(
                        !source.evaluate(sol),
                        "SAT->IS [{label}]: unsatisfiable but extracted satisfies"
                    );
                }
            } else {
                assert!(
                    extracted.is_subset(&sat_solutions),
                    "SAT->IS [{label}]: extracted not subset"
                );
                assert_eq!(
                    sat_solutions,
                    jl_parse_configs_set(&case["best_source"]),
                    "SAT->IS [{label}]: best source mismatch"
                );
            }
        }
    }
}
