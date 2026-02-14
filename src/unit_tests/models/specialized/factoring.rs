use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;
include!("../../jl_helpers.rs");

#[test]
fn test_factoring_creation() {
    let problem = Factoring::new(3, 3, 15);
    assert_eq!(problem.m(), 3);
    assert_eq!(problem.n(), 3);
    assert_eq!(problem.target(), 15);
    assert_eq!(problem.num_variables(), 6);
}

#[test]
fn test_bits_to_int() {
    assert_eq!(bits_to_int(&[0, 0, 0]), 0);
    assert_eq!(bits_to_int(&[1, 0, 0]), 1);
    assert_eq!(bits_to_int(&[0, 1, 0]), 2);
    assert_eq!(bits_to_int(&[1, 1, 0]), 3);
    assert_eq!(bits_to_int(&[0, 0, 1]), 4);
    assert_eq!(bits_to_int(&[1, 1, 1]), 7);
}

#[test]
fn test_int_to_bits() {
    assert_eq!(int_to_bits(0, 3), vec![0, 0, 0]);
    assert_eq!(int_to_bits(1, 3), vec![1, 0, 0]);
    assert_eq!(int_to_bits(2, 3), vec![0, 1, 0]);
    assert_eq!(int_to_bits(3, 3), vec![1, 1, 0]);
    assert_eq!(int_to_bits(7, 3), vec![1, 1, 1]);
}

#[test]
fn test_read_factors() {
    let problem = Factoring::new(2, 2, 6);
    // bits: [a0, a1, b0, b1]
    // a=2 (binary 10), b=3 (binary 11) -> config = [0,1,1,1]
    let (a, b) = problem.read_factors(&[0, 1, 1, 1]);
    assert_eq!(a, 2);
    assert_eq!(b, 3);
}

#[test]
fn test_is_factoring_function() {
    assert!(is_factoring(6, 2, 3));
    assert!(is_factoring(6, 3, 2));
    assert!(is_factoring(15, 3, 5));
    assert!(!is_factoring(6, 2, 2));
}

#[test]
fn test_direction() {
    let problem = Factoring::new(2, 2, 6);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_is_valid_factorization() {
    let problem = Factoring::new(2, 2, 6);
    assert!(problem.is_valid_factorization(&[0, 1, 1, 1])); // 2*3=6
    assert!(!problem.is_valid_factorization(&[0, 1, 0, 1])); // 2*2=4
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/factoring.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let m = instance["instance"]["m"].as_u64().unwrap() as usize;
        let n = instance["instance"]["n"].as_u64().unwrap() as usize;
        let input = instance["instance"]["input"].as_u64().unwrap();
        let problem = Factoring::new(m, n, input);
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            if jl_valid {
                assert_eq!(
                    result.unwrap(),
                    0,
                    "Factoring: valid config should have distance 0"
                );
            } else {
                assert_ne!(
                    result.unwrap(),
                    0,
                    "Factoring: invalid config should have nonzero distance"
                );
            }
        }
        let best = BruteForce::new().find_all_best(&problem);
        let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "Factoring best solutions mismatch");
    }
}
