use super::*;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_sat_to_3sat_exact_size() {
    // Clause already has 3 literals - should remain unchanged
    let sat = Satisfiability::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    assert_eq!(ksat.num_vars(), 3);
    assert_eq!(ksat.num_clauses(), 1);
    assert_eq!(ksat.clauses()[0].literals, vec![1, 2, 3]);
}

#[test]
fn test_sat_to_3sat_padding() {
    // Clause has 2 literals - should be padded to 3
    // (a v b) becomes (a v b v x) AND (a v b v -x)
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, 2])]);

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Should have 2 clauses (positive and negative ancilla)
    assert_eq!(ksat.num_clauses(), 2);
    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }
}

#[test]
fn test_sat_to_3sat_splitting() {
    // Clause has 4 literals - should be split
    // (a v b v c v d) becomes (a v b v x) AND (-x v c v d)
    let sat = Satisfiability::new(4, vec![CNFClause::new(vec![1, 2, 3, 4])]);

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Should have 2 clauses after splitting
    assert_eq!(ksat.num_clauses(), 2);
    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }

    // Verify structure: first clause has positive ancilla, second has negative
    let c1 = &ksat.clauses()[0];
    let c2 = &ksat.clauses()[1];
    // First clause: [1, 2, 5] (ancilla is var 5)
    assert_eq!(c1.literals[0], 1);
    assert_eq!(c1.literals[1], 2);
    let ancilla = c1.literals[2];
    assert!(ancilla > 0);
    // Second clause: [-5, 3, 4]
    assert_eq!(c2.literals[0], -ancilla);
    assert_eq!(c2.literals[1], 3);
    assert_eq!(c2.literals[2], 4);
}

#[test]
fn test_sat_to_3sat_large_clause() {
    // Clause has 5 literals - requires multiple splits
    // (a v b v c v d v e) -> (a v b v x1) AND (-x1 v c v x2) AND (-x2 v d v e)
    let sat = Satisfiability::new(5, vec![CNFClause::new(vec![1, 2, 3, 4, 5])]);

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // Should have 3 clauses after splitting
    assert_eq!(ksat.num_clauses(), 3);
    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }
}

#[test]
fn test_sat_to_3sat_single_literal() {
    // Single literal clause - needs padding twice
    // (a) becomes (a v x v y) where we pad twice
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // With recursive padding: (a) -> (a v x) AND (a v -x)
    // Then each of those gets padded again
    // (a v x) -> (a v x v y) AND (a v x v -y)
    // (a v -x) -> (a v -x v z) AND (a v -x v -z)
    // Total: 4 clauses
    assert_eq!(ksat.num_clauses(), 4);
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }
}

#[test]
fn test_3sat_to_sat() {
    let ksat = KSatisfiability::<3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );

    let reduction = ReduceTo::<Satisfiability>::reduce_to(&ksat);
    let sat = reduction.target_problem();

    assert_eq!(sat.num_vars(), 3);
    assert_eq!(sat.num_clauses(), 2);

    // Verify clauses are preserved
    assert_eq!(sat.clauses()[0].literals, vec![1, 2, 3]);
    assert_eq!(sat.clauses()[1].literals, vec![-1, -2, 3]);
}

#[test]
fn test_3sat_to_sat_solution_extraction() {
    let ksat = KSatisfiability::<3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

    let reduction = ReduceTo::<Satisfiability>::reduce_to(&ksat);

    let sol = vec![1, 0, 1];
    let extracted = reduction.extract_solution(&sol);
    assert_eq!(extracted, vec![1, 0, 1]);
}

#[test]
fn test_sat_to_3sat_mixed_clause_types() {
    // Test padding, exact-size, and splitting all at once
    let sat = Satisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2]),           // Needs padding
            CNFClause::new(vec![1, 2, 3]),        // Exact
            CNFClause::new(vec![1, 2, 3, 4, -1]), // Needs splitting
        ],
    );

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // All clauses should have exactly 3 literals
    for clause in ksat.clauses() {
        assert_eq!(clause.len(), 3);
    }
}

#[test]
fn test_ksat_structure() {
    let sat = Satisfiability::new(3, vec![CNFClause::new(vec![1, 2, 3, 4])]);

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    // K-SAT should preserve original variables plus auxiliary vars
    // A 4-literal clause requires 1 auxiliary variable for Tseitin
    assert_eq!(ksat.num_vars(), 3 + 1); // Original vars + 1 auxiliary for Tseitin
}

#[test]
fn test_empty_sat_to_3sat() {
    let sat = Satisfiability::new(3, vec![]);

    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    assert_eq!(ksat.num_clauses(), 0);
    assert_eq!(ksat.num_vars(), 3);
}

#[test]
fn test_jl_parity_sat_to_ksat() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/satisfiability_to_ksatisfiability3.json")).unwrap();
    let sat_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/satisfiability.json")).unwrap();
    let inst = &sat_data["instances"][0]["instance"];
    let (num_vars, clauses) = jl_parse_sat_clauses(inst);
    let source = Satisfiability::new(num_vars, clauses);
    let result = ReduceTo::<KSatisfiability<3>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_satisfying(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_satisfying(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_ksat_to_sat() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/ksatisfiability_to_satisfiability.json")).unwrap();
    let ksat_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/ksatisfiability.json")).unwrap();
    let inst = &ksat_data["instances"][0]["instance"];
    let (num_vars, clauses) = jl_parse_sat_clauses(inst);
    let source = KSatisfiability::<3>::new(num_vars, clauses);
    let result = ReduceTo::<Satisfiability>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_satisfying(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_satisfying(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}

#[test]
fn test_jl_parity_rule_sat_to_ksat() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/rule_satisfiability_to_ksatisfiability3.json")).unwrap();
    let sat_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/satisfiability.json")).unwrap();
    let inst = &jl_find_instance_by_label(&sat_data, "rule_3sat_multi")["instance"];
    let (num_vars, clauses) = jl_parse_sat_clauses(inst);
    let source = Satisfiability::new(num_vars, clauses);
    let result = ReduceTo::<KSatisfiability<3>>::reduce_to(&source);
    let solver = BruteForce::new();
    let best_target = solver.find_all_satisfying(result.target_problem());
    let best_source: HashSet<Vec<usize>> = solver.find_all_satisfying(&source).into_iter().collect();
    let extracted: HashSet<Vec<usize>> = best_target.iter().map(|t| result.extract_solution(t)).collect();
    assert!(extracted.is_subset(&best_source));
    for case in data["cases"].as_array().unwrap() {
        assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]));
    }
}
