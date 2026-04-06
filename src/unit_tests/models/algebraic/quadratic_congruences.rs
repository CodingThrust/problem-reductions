use crate::models::algebraic::QuadraticCongruences;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;
use num_bigint::BigUint;

fn yes_problem() -> QuadraticCongruences {
    // a=4, b=15, c=10: x=2 → 4 mod 15 = 4 ✓; x=7 → 49 mod 15 = 4 ✓; x=8 → 64 mod 15 = 4 ✓
    QuadraticCongruences::new(4, 15, 10)
}

fn no_problem() -> QuadraticCongruences {
    // a=3, b=7, c=7: no x in {1..6} satisfies x² ≡ 3 (mod 7) (QRs mod 7 are {0,1,2,4})
    QuadraticCongruences::new(3, 7, 7)
}

fn bu(n: u32) -> BigUint {
    BigUint::from(n)
}

fn config_for_x(problem: &QuadraticCongruences, x: u32) -> Vec<usize> {
    problem.encode_witness(&bu(x)).unwrap()
}

#[test]
fn test_quadratic_congruences_creation_and_accessors() {
    let p = yes_problem();
    assert_eq!(p.a(), &bu(4));
    assert_eq!(p.b(), &bu(15));
    assert_eq!(p.c(), &bu(10));
    assert_eq!(p.bit_length_a(), 3);
    assert_eq!(p.bit_length_b(), 4);
    assert_eq!(p.bit_length_c(), 4);
    // x is encoded as 4 binary digits because c - 1 = 9 has 4 bits.
    assert_eq!(p.dims(), vec![2, 2, 2, 2]);
    assert_eq!(p.num_variables(), 4);
    assert_eq!(
        <QuadraticCongruences as Problem>::NAME,
        "QuadraticCongruences"
    );
    assert_eq!(<QuadraticCongruences as Problem>::variant(), vec![]);
}

#[test]
fn test_quadratic_congruences_evaluate_yes() {
    let p = yes_problem();
    assert_eq!(p.evaluate(&config_for_x(&p, 2)), Or(true));
    assert_eq!(p.evaluate(&config_for_x(&p, 7)), Or(true));
    assert_eq!(p.evaluate(&config_for_x(&p, 8)), Or(true));
    assert_eq!(p.evaluate(&config_for_x(&p, 1)), Or(false));
    assert_eq!(p.evaluate(&config_for_x(&p, 3)), Or(false));
}

#[test]
fn test_quadratic_congruences_evaluate_no() {
    let p = no_problem();
    // c - 1 = 6 has 3 bits.
    assert_eq!(p.dims(), vec![2, 2, 2]);
    for x in 1..7 {
        // quadratic residues mod 7 are {0,1,2,4}; 3 is not one
        assert_eq!(p.evaluate(&config_for_x(&p, x)), Or(false));
    }
}

#[test]
fn test_quadratic_congruences_evaluate_invalid_config() {
    let p = yes_problem();
    assert_eq!(p.evaluate(&[]), Or(false));
    assert_eq!(p.evaluate(&[0, 1]), Or(false));
    assert_eq!(p.evaluate(&[0, 1, 0, 2]), Or(false));
}

#[test]
fn test_quadratic_congruences_c_le_1() {
    // c=1: search space {1..0} is empty
    let p = QuadraticCongruences::new(0, 5, 1);
    assert_eq!(p.dims(), Vec::<usize>::new());
    assert_eq!(p.evaluate(&[0]), Or(false));
    assert_eq!(p.evaluate(&[]), Or(false));
}

#[test]
fn test_quadratic_congruences_bigint_witness_encoding_round_trip() {
    let c = BigUint::parse_bytes(b"2535301200456458802993406410753", 10).unwrap();
    let p = QuadraticCongruences::new(4u32, 15u32, c);
    let x = (BigUint::from(1u32) << 100usize) + BigUint::from(1u32);
    let config = p.encode_witness(&x).expect("x should be encodable");

    assert_eq!(config.len(), p.dims().len());
    assert_eq!(p.decode_witness(&config), Some(x));
}

#[test]
fn test_quadratic_congruences_brute_force_finds_witness() {
    let solver = BruteForce::new();
    let witness = solver.find_witness(&yes_problem()).unwrap();
    assert_eq!(yes_problem().evaluate(&witness), Or(true));
    let x = yes_problem().decode_witness(&witness).unwrap();
    assert!(matches!(x, v if v == bu(2) || v == bu(7) || v == bu(8)));
}

#[test]
fn test_quadratic_congruences_brute_force_finds_all_witnesses() {
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&yes_problem());
    assert_eq!(all.len(), 3);
    assert!(all
        .iter()
        .all(|sol| yes_problem().evaluate(sol) == Or(true)));
    let decoded = all
        .iter()
        .map(|sol| yes_problem().decode_witness(sol).unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        decoded,
        std::collections::BTreeSet::from([bu(2), bu(7), bu(8)])
    );
}

#[test]
fn test_quadratic_congruences_brute_force_no_witness() {
    let solver = BruteForce::new();
    assert!(solver.find_witness(&no_problem()).is_none());
}

#[test]
fn test_quadratic_congruences_serialization() {
    let p = yes_problem();
    let json = serde_json::to_value(&p).unwrap();
    assert_eq!(json, serde_json::json!({"a": "4", "b": "15", "c": "10"}));

    let restored: QuadraticCongruences = serde_json::from_value(json).unwrap();
    assert_eq!(restored.a(), p.a());
    assert_eq!(restored.b(), p.b());
    assert_eq!(restored.c(), p.c());
}

#[test]
fn test_quadratic_congruences_deserialization_rejects_invalid() {
    // b=0
    let r: Result<QuadraticCongruences, _> =
        serde_json::from_value(serde_json::json!({"a": 0, "b": 0, "c": 5}));
    assert!(r.is_err());
    // c=0
    let r: Result<QuadraticCongruences, _> =
        serde_json::from_value(serde_json::json!({"a": 0, "b": 5, "c": 0}));
    assert!(r.is_err());
    // a >= b
    let r: Result<QuadraticCongruences, _> =
        serde_json::from_value(serde_json::json!({"a": 7, "b": 5, "c": 10}));
    assert!(r.is_err());
}

#[test]
fn test_quadratic_congruences_paper_example() {
    // Canonical example: a=4, b=15, c=10; x=2 encodes to binary digits [0,1,0,0].
    let p = QuadraticCongruences::new(4, 15, 10);
    let config = config_for_x(&p, 2);
    assert_eq!(p.evaluate(&config), Or(true));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Or(true));
}

#[test]
#[should_panic(expected = "Modulus b must be positive")]
fn test_quadratic_congruences_panics_on_zero_b() {
    QuadraticCongruences::new(0, 0, 5);
}

#[test]
#[should_panic(expected = "Bound c must be positive")]
fn test_quadratic_congruences_panics_on_zero_c() {
    QuadraticCongruences::new(0, 5, 0);
}

#[test]
#[should_panic(expected = "Residue a")]
fn test_quadratic_congruences_panics_on_a_ge_b() {
    QuadraticCongruences::new(5, 5, 10);
}
