use crate::models::algebraic::QuadraticCongruences;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn yes_problem() -> QuadraticCongruences {
    // a=4, b=15, c=10: x=2 → 4 mod 15 = 4 ✓; x=7 → 49 mod 15 = 4 ✓; x=8 → 64 mod 15 = 4 ✓
    QuadraticCongruences::new(4, 15, 10)
}

fn no_problem() -> QuadraticCongruences {
    // a=3, b=7, c=7: no x in {1..6} satisfies x² ≡ 3 (mod 7) (QRs mod 7 are {0,1,2,4})
    QuadraticCongruences::new(3, 7, 7)
}

#[test]
fn test_quadratic_congruences_creation_and_accessors() {
    let p = yes_problem();
    assert_eq!(p.a(), 4);
    assert_eq!(p.b(), 15);
    assert_eq!(p.c(), 10);
    // config[0] ∈ {0..8} → x ∈ {1..9}: dims = [9]
    assert_eq!(p.dims(), vec![9]);
    assert_eq!(p.num_variables(), 1);
    assert_eq!(
        <QuadraticCongruences as Problem>::NAME,
        "QuadraticCongruences"
    );
    assert_eq!(<QuadraticCongruences as Problem>::variant(), vec![]);
}

#[test]
fn test_quadratic_congruences_evaluate_yes() {
    let p = yes_problem();
    // x=2 (config[0]=1): 4 mod 15 = 4 ✓
    assert_eq!(p.evaluate(&[1]), Or(true));
    // x=7 (config[0]=6): 49 mod 15 = 4 ✓
    assert_eq!(p.evaluate(&[6]), Or(true));
    // x=8 (config[0]=7): 64 mod 15 = 4 ✓
    assert_eq!(p.evaluate(&[7]), Or(true));
    // x=1 (config[0]=0): 1 mod 15 = 1 ≠ 4
    assert_eq!(p.evaluate(&[0]), Or(false));
    // x=3 (config[0]=2): 9 mod 15 = 9 ≠ 4
    assert_eq!(p.evaluate(&[2]), Or(false));
}

#[test]
fn test_quadratic_congruences_evaluate_no() {
    let p = no_problem();
    // dims = [6]: x ∈ {1..6}
    assert_eq!(p.dims(), vec![6]);
    for cfg in 0..6 {
        // quadratic residues mod 7 are {0,1,2,4}; 3 is not one
        assert_eq!(p.evaluate(&[cfg]), Or(false));
    }
}

#[test]
fn test_quadratic_congruences_evaluate_invalid_config() {
    let p = yes_problem();
    // Wrong number of variables
    assert_eq!(p.evaluate(&[]), Or(false));
    assert_eq!(p.evaluate(&[0, 1]), Or(false));
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
fn test_quadratic_congruences_brute_force_finds_witness() {
    let solver = BruteForce::new();
    let witness = solver.find_witness(&yes_problem()).unwrap();
    assert_eq!(yes_problem().evaluate(&witness), Or(true));
}

#[test]
fn test_quadratic_congruences_brute_force_finds_all_witnesses() {
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&yes_problem());
    // x=2 (cfg=1), x=7 (cfg=6), x=8 (cfg=7)
    assert_eq!(all.len(), 3);
    assert!(all
        .iter()
        .all(|sol| yes_problem().evaluate(sol) == Or(true)));
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
    assert_eq!(json, serde_json::json!({"a": 4, "b": 15, "c": 10}));

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
    // Canonical example: a=4, b=15, c=10; optimal config [1] (x=2)
    let p = QuadraticCongruences::new(4, 15, 10);
    assert_eq!(p.evaluate(&[1]), Or(true));

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
