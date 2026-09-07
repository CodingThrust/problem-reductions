use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use num_bigint::BigUint;
include!("../../jl_helpers.rs");

#[test]
fn test_factoring_creation() {
    let problem = Factoring::with_factor_bits(15, 3, 3);
    assert_eq!(problem.m(), 3);
    assert_eq!(problem.n(), 3);
    assert_eq!(problem.target(), &BigUint::from(15u32));
    assert_eq!(problem.num_variables(), 6);
}

#[test]
fn test_factoring_derives_safe_default_widths() {
    let problem = Factoring::new(15);
    assert_eq!(problem.m(), 2);
    assert_eq!(problem.n(), 3);
    assert!(problem.is_valid_factorization(&(3u32.into(), 5u32.into())));
    assert!(!problem.is_valid_factorization(&(1u32.into(), 15u32.into())));
}

#[test]
fn test_explicit_widths_allow_trivial_factorization() {
    let problem = Factoring::with_factor_bits(15, 2, 4);
    assert!(problem.is_valid_factorization(&(1u32.into(), 15u32.into())));
}

#[test]
fn test_bits_to_biguint() {
    assert_eq!(bits_to_biguint(&[0, 0, 0]), BigUint::from(0u32));
    assert_eq!(bits_to_biguint(&[1, 0, 0]), BigUint::from(1u32));
    assert_eq!(bits_to_biguint(&[0, 1, 0]), BigUint::from(2u32));
    assert_eq!(bits_to_biguint(&[1, 1, 0]), BigUint::from(3u32));
    assert_eq!(bits_to_biguint(&[0, 0, 1]), BigUint::from(4u32));
    assert_eq!(bits_to_biguint(&[1, 1, 1]), BigUint::from(7u32));
}

#[test]
fn test_int_to_bits() {
    assert_eq!(int_to_bits(&BigUint::from(0u32), 3), vec![0, 0, 0]);
    assert_eq!(int_to_bits(&BigUint::from(1u32), 3), vec![1, 0, 0]);
    assert_eq!(int_to_bits(&BigUint::from(2u32), 3), vec![0, 1, 0]);
    assert_eq!(int_to_bits(&BigUint::from(3u32), 3), vec![1, 1, 0]);
    assert_eq!(int_to_bits(&BigUint::from(7u32), 3), vec![1, 1, 1]);
}

#[test]
fn test_read_factors() {
    let problem = Factoring::with_factor_bits(6, 2, 2);
    // bits: [a0, a1, b0, b1]
    // a=2 (binary 10), b=3 (binary 11) -> config = [0,1,1,1]
    let (a, b) = problem.decode_factors(&[0, 1, 1, 1]);
    assert_eq!(a, BigUint::from(2u32));
    assert_eq!(b, BigUint::from(3u32));
}

#[test]
fn test_is_factoring_function() {
    assert!(is_factoring(&6u32.into(), &2u32.into(), &3u32.into()));
    assert!(is_factoring(&6u32.into(), &3u32.into(), &2u32.into()));
    assert!(is_factoring(&15u32.into(), &3u32.into(), &5u32.into()));
    assert!(!is_factoring(&6u32.into(), &2u32.into(), &2u32.into()));
}

#[test]
fn test_is_valid_factorization() {
    let problem = Factoring::with_factor_bits(6, 2, 2);
    assert!(problem.is_valid_factorization(&(2u32.into(), 3u32.into())));
    assert!(!problem.is_valid_factorization(&(3u32.into(), 2u32.into())));
    assert!(!problem.is_valid_factorization(&(2u32.into(), 2u32.into())));
}

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../../../../tests/data/jl/factoring.json")).unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let m = instance["instance"]["m"].as_u64().unwrap() as usize;
        let n = instance["instance"]["n"].as_u64().unwrap() as usize;
        let input = instance["instance"]["input"].as_u64().unwrap();
        let problem = Factoring::with_factor_bits(input, m.min(n), m.max(n));
        for eval in instance["evaluations"].as_array().unwrap() {
            let raw = jl_parse_config(&eval["config"]);
            let left = bits_to_biguint(&raw[..m]);
            let right = bits_to_biguint(&raw[m..m + n]);
            let config = if left <= right {
                (left, right)
            } else {
                (right, left)
            };
            let result = problem.evaluate(&config).unwrap();
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(result.unwrap(), jl_valid);
        }
        let best = BruteForce::new().find_all_witnesses(&problem).unwrap();
        let jl_best: HashSet<(BigUint, BigUint)> =
            jl_parse_configs_set(&instance["best_solutions"])
                .iter()
                .map(|config| {
                    let left = bits_to_biguint(&config[..m]);
                    let right = bits_to_biguint(&config[m..m + n]);
                    if left <= right {
                        (left, right)
                    } else {
                        (right, left)
                    }
                })
                .collect();
        let rust_best: HashSet<(BigUint, BigUint)> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "Factoring best solutions mismatch");
    }
}

#[test]
fn test_is_valid_solution() {
    // Factor 15 = 3 × 5, 3 bits each
    let problem = Factoring::with_factor_bits(15, 3, 3);
    // Valid: 3 = [1,1,0], 5 = [1,0,1] → config = [1,1,0,1,0,1]
    assert!(problem.is_valid_solution(&(3u32.into(), 5u32.into())));
    // Invalid: 2 = [0,1,0], 3 = [1,1,0] → 2*3=6 ≠ 15
    assert!(!problem.is_valid_solution(&(2u32.into(), 3u32.into())));
}

#[test]
fn test_parameter_getters() {
    let problem = Factoring::with_factor_bits(15, 3, 3);
    assert_eq!(problem.num_bits_first(), 3);
    assert_eq!(problem.num_bits_second(), 3);
}

#[test]
fn test_factoring_paper_example() {
    // Paper: N=15, m=2 bits, n=3 bits, p=3, q=5
    let problem = Factoring::with_factor_bits(15, 2, 3);
    assert_eq!(problem.num_variables(), 5);

    // p=3 -> bits [1,1], q=5 -> bits [1,0,1]
    let config = (BigUint::from(3u32), BigUint::from(5u32));
    let (a, b) = config.clone();
    assert_eq!(a, BigUint::from(3u32));
    assert_eq!(b, BigUint::from(5u32));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_factoring_supports_values_beyond_u64() {
    let a = (BigUint::from(1u32) << 65) + BigUint::from(1u32);
    let b = BigUint::from(3u32);
    let target: BigUint = &a * &b;
    let problem = Factoring::with_factor_bits(target.clone(), 2, 66);

    let config = (b, a);
    assert!(problem.evaluate(&config).unwrap());
    assert_eq!(problem.target(), &target);

    let json = serde_json::to_string(&problem).unwrap();
    let restored: Factoring = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.target(), &target);
}

#[test]
fn test_deserialization_accepts_omitted_widths() {
    let problem: Factoring = serde_json::from_str(r#"{"target":"15"}"#).unwrap();
    assert_eq!((problem.m(), problem.n()), (2, 3));
}

#[test]
fn test_deserialization_requires_widths_together() {
    let error = serde_json::from_str::<Factoring>(r#"{"target":"15","m":2}"#).unwrap_err();
    assert!(error.to_string().contains("must be provided together"));
}

#[test]
fn test_default_widths_round_trip_for_one() {
    let problem = Factoring::new(1);
    let json = serde_json::to_string(&problem).unwrap();
    let restored: Factoring = serde_json::from_str(&json).unwrap();
    assert_eq!((restored.m(), restored.n()), (0, 1));
}
