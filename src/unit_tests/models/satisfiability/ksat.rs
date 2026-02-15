use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::variant::{K2, K3, KN};
include!("../../jl_helpers.rs");

#[test]
fn test_3sat_creation() {
    let problem = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_clauses(), 2);
}

#[test]
#[should_panic(expected = "Clause 0 has 2 literals, expected 3")]
fn test_3sat_wrong_clause_size() {
    let _ = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2])]);
}

#[test]
fn test_2sat_creation() {
    let problem = KSatisfiability::<K2>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );
    assert_eq!(problem.num_vars(), 2);
    assert_eq!(problem.num_clauses(), 2);
}

#[test]
fn test_3sat_is_satisfying() {
    // (x1 OR x2 OR x3) AND (NOT x1 OR NOT x2 OR NOT x3)
    let problem = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );

    // x1=T, x2=F, x3=F satisfies both
    assert!(problem.is_satisfying(&[true, false, false]));
    // x1=T, x2=T, x3=T fails second clause
    assert!(!problem.is_satisfying(&[true, true, true]));
}

#[test]
fn test_3sat_brute_force() {
    let problem = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);

    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_ksat_allow_less() {
    // This should work - clause has 2 literals which is <= 3
    let problem = KSatisfiability::<K3>::new_allow_less(2, vec![CNFClause::new(vec![1, 2])]);
    assert_eq!(problem.num_clauses(), 1);
}

#[test]
#[should_panic(expected = "Clause 0 has 4 literals, expected at most 3")]
fn test_ksat_allow_less_too_many() {
    let _ = KSatisfiability::<K3>::new_allow_less(4, vec![CNFClause::new(vec![1, 2, 3, 4])]);
}

#[test]
fn test_ksat_get_clause() {
    let problem = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    assert_eq!(problem.get_clause(0), Some(&CNFClause::new(vec![1, 2, 3])));
    assert_eq!(problem.get_clause(1), None);
}

#[test]
fn test_ksat_count_satisfied() {
    let problem = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    // x1=T, x2=T, x3=T: first satisfied, second not
    assert_eq!(problem.count_satisfied(&[true, true, true]), 1);
    // x1=T, x2=F, x3=F: both satisfied
    assert_eq!(problem.count_satisfied(&[true, false, false]), 2);
}

#[test]
fn test_ksat_evaluate() {
    let problem = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    assert!(problem.evaluate(&[1, 0, 0])); // x1=T, x2=F, x3=F
    assert!(!problem.evaluate(&[1, 1, 1])); // x1=T, x2=T, x3=T
}

#[test]
fn test_ksat_problem_v2() {
    use crate::traits::Problem;

    let p = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );

    assert_eq!(p.dims(), vec![2, 2, 2]);
    assert!(p.evaluate(&[1, 0, 0]));
    assert!(!p.evaluate(&[1, 1, 1]));
    assert!(!p.evaluate(&[0, 0, 0]));
    assert!(p.evaluate(&[1, 0, 1]));
    assert_eq!(<KSatisfiability<K3> as Problem>::NAME, "KSatisfiability");
}

#[test]
fn test_ksat_problem_v2_2sat() {
    use crate::traits::Problem;

    let p = KSatisfiability::<K2>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );

    assert_eq!(p.dims(), vec![2, 2]);
    assert!(p.evaluate(&[1, 0]));
    assert!(p.evaluate(&[0, 1]));
    assert!(!p.evaluate(&[1, 1]));
    assert!(!p.evaluate(&[0, 0]));
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/data/jl/ksatisfiability.json"
    ))
    .unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let (num_vars, clauses) = jl_parse_sat_clauses(&instance["instance"]);
        let num_clauses = instance["instance"]["clauses"].as_array().unwrap().len();
        let problem = KSatisfiability::<K3>::new(num_vars, clauses);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let rust_result = problem.evaluate(&config);
            let jl_size = eval["size"].as_u64().unwrap() as usize;
            assert_eq!(
                rust_result,
                jl_size == num_clauses,
                "KSat eval mismatch for config {:?}",
                config
            );
        }
        let rust_best = BruteForce::new().find_all_satisfying(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best_set: HashSet<Vec<usize>> = rust_best.into_iter().collect();
        assert_eq!(rust_best_set, jl_best, "KSat best solutions mismatch");
    }
}

#[test]
fn test_kn_creation() {
    // KN accepts clauses of any length without validation
    let problem = KSatisfiability::<KN>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),    // 3 literals
            CNFClause::new(vec![-1, -2]),      // 2 literals
            CNFClause::new(vec![1]),           // 1 literal
        ],
    );
    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_clauses(), 3);
    assert!(problem.evaluate(&[1, 0, 0])); // x1=T, x2=F, x3=F
}

#[test]
fn test_kn_from_k3_clauses() {
    // KN can be constructed from clauses originally built for K3
    let k3 = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    let kn = KSatisfiability::<KN>::new(k3.num_vars(), k3.clauses().to_vec());
    // Both should agree on evaluations
    for config in &[[1, 0, 0], [0, 1, 0], [1, 1, 1]] {
        assert_eq!(k3.evaluate(config), kn.evaluate(config));
    }
}
