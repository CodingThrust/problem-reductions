use crate::models::algebraic::AlgebraicEquationsOverGF2;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

/// n=3, equations:
///   eq0: x0*x1 + x2 = 0
///   eq1: x1*x2 + x0 + 1 = 0
///   eq2: x0 + x1 + x2 + 1 = 0
/// Solution: (1,0,0)
fn satisfiable_problem() -> AlgebraicEquationsOverGF2 {
    AlgebraicEquationsOverGF2::new(
        3,
        vec![
            vec![vec![0, 1], vec![2]],
            vec![vec![1, 2], vec![0], vec![]],
            vec![vec![0], vec![1], vec![2], vec![]],
        ],
    )
    .unwrap()
}

/// n=2, equations:
///   eq0: x0 + x1 = 0  (x0 XOR x1 = 0, so x0 = x1)
///   eq1: x0 + x1 + 1 = 0  (x0 XOR x1 XOR 1 = 0, so x0 != x1)
/// No solution — contradictory.
fn unsatisfiable_problem() -> AlgebraicEquationsOverGF2 {
    AlgebraicEquationsOverGF2::new(
        2,
        vec![vec![vec![0], vec![1]], vec![vec![0], vec![1], vec![]]],
    )
    .unwrap()
}

#[test]
fn test_algebraic_equations_over_gf2_creation_and_accessors() {
    let p = satisfiable_problem();
    assert_eq!(p.num_variables(), 3);
    assert_eq!(p.num_equations(), 3);
    assert_eq!(p.equations().len(), 3);
    assert_eq!(p.dims(), vec![2, 2, 2]);
    assert_eq!(p.num_variables(), 3);
    assert_eq!(
        <AlgebraicEquationsOverGF2 as Problem>::NAME,
        "AlgebraicEquationsOverGF2"
    );
    assert_eq!(<AlgebraicEquationsOverGF2 as Problem>::variant(), vec![]);
}

#[test]
fn test_algebraic_equations_over_gf2_evaluate_satisfiable() {
    let p = satisfiable_problem();
    // config [1,0,0]:
    //   eq0: 1*0 + 0 = 0 ✓
    //   eq1: 0*0 + 1 + 1 = 0 ✓
    //   eq2: 1 + 0 + 0 + 1 = 0 ✓
    assert_eq!(p.evaluate(&[1, 0, 0]), Or(true));

    // config [0,0,0]:
    //   eq0: 0*0 + 0 = 0 ✓
    //   eq1: 0*0 + 0 + 1 = 1 ✗
    assert_eq!(p.evaluate(&[0, 0, 0]), Or(false));

    // config [1,1,1]:
    //   eq0: 1*1 + 1 = 0 ✓
    //   eq1: 1*1 + 1 + 1 = 1 ✗
    assert_eq!(p.evaluate(&[1, 1, 1]), Or(false));
}

#[test]
fn test_algebraic_equations_over_gf2_evaluate_unsatisfiable() {
    let p = unsatisfiable_problem();
    assert_eq!(p.dims(), vec![2, 2]);
    // All 4 assignments should fail
    assert_eq!(p.evaluate(&[0, 0]), Or(false)); // eq0: 0+0=0 ✓, eq1: 0+0+1=1 ✗
    assert_eq!(p.evaluate(&[0, 1]), Or(false)); // eq0: 0+1=1 ✗
    assert_eq!(p.evaluate(&[1, 0]), Or(false)); // eq0: 1+0=1 ✗
    assert_eq!(p.evaluate(&[1, 1]), Or(false)); // eq0: 1+1=0 ✓, eq1: 1+1+1=1 ✗
}

#[test]
fn test_algebraic_equations_over_gf2_constant_monomial() {
    // Single equation: 1 = 0 (always false)
    let p = AlgebraicEquationsOverGF2::new(1, vec![vec![vec![]]]).unwrap();
    assert_eq!(p.evaluate(&[0]), Or(false));
    assert_eq!(p.evaluate(&[1]), Or(false));

    // Single equation: 1 + 1 = 0 (always true — two constants XOR to 0)
    let p2 = AlgebraicEquationsOverGF2::new(1, vec![vec![vec![], vec![]]]).unwrap();
    assert_eq!(p2.evaluate(&[0]), Or(true));
    assert_eq!(p2.evaluate(&[1]), Or(true));
}

#[test]
fn test_algebraic_equations_over_gf2_empty_equations() {
    // No equations: trivially satisfied
    let p = AlgebraicEquationsOverGF2::new(2, vec![]).unwrap();
    assert_eq!(p.evaluate(&[0, 0]), Or(true));
    assert_eq!(p.evaluate(&[1, 1]), Or(true));
}

#[test]
fn test_algebraic_equations_over_gf2_empty_polynomial() {
    // One equation with no monomials: sum = 0, so satisfied
    let p = AlgebraicEquationsOverGF2::new(2, vec![vec![]]).unwrap();
    assert_eq!(p.evaluate(&[0, 0]), Or(true));
}

#[test]
fn test_algebraic_equations_over_gf2_brute_force_finds_witness() {
    let solver = BruteForce::new();
    let p = satisfiable_problem();
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Or(true));
}

#[test]
fn test_algebraic_equations_over_gf2_brute_force_no_witness() {
    let solver = BruteForce::new();
    assert!(solver.find_witness(&unsatisfiable_problem()).is_none());
}

#[test]
fn test_algebraic_equations_over_gf2_brute_force_finds_all_witnesses() {
    let solver = BruteForce::new();
    let p = satisfiable_problem();
    let all = solver.find_all_witnesses(&p);
    assert!(!all.is_empty());
    assert!(all.iter().all(|sol| p.evaluate(sol) == Or(true)));
}

#[test]
fn test_algebraic_equations_over_gf2_serialization() {
    let p = satisfiable_problem();
    let json = serde_json::to_value(&p).unwrap();
    assert_eq!(json["num_variables"], 3);
    assert_eq!(json["equations"].as_array().unwrap().len(), 3);

    let restored: AlgebraicEquationsOverGF2 = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_variables(), p.num_variables());
    assert_eq!(restored.num_equations(), p.num_equations());
    // Check round-trip preserves evaluation
    assert_eq!(restored.evaluate(&[1, 0, 0]), Or(true));
}

#[test]
fn test_algebraic_equations_over_gf2_deserialization_rejects_invalid() {
    // Variable index out of range
    let r: Result<AlgebraicEquationsOverGF2, _> = serde_json::from_value(serde_json::json!({
        "num_variables": 2,
        "equations": [[[0, 5]]]
    }));
    assert!(r.is_err());

    // Unsorted monomial
    let r: Result<AlgebraicEquationsOverGF2, _> = serde_json::from_value(serde_json::json!({
        "num_variables": 3,
        "equations": [[[1, 0]]]
    }));
    assert!(r.is_err());

    // Duplicate variable in monomial
    let r: Result<AlgebraicEquationsOverGF2, _> = serde_json::from_value(serde_json::json!({
        "num_variables": 3,
        "equations": [[[1, 1]]]
    }));
    assert!(r.is_err());
}

#[test]
fn test_algebraic_equations_over_gf2_validation_errors() {
    // Out of range
    assert!(AlgebraicEquationsOverGF2::new(2, vec![vec![vec![3]]]).is_err());
    // Not sorted
    assert!(AlgebraicEquationsOverGF2::new(3, vec![vec![vec![2, 1]]]).is_err());
    // Duplicate
    assert!(AlgebraicEquationsOverGF2::new(3, vec![vec![vec![1, 1]]]).is_err());
    // Valid
    assert!(AlgebraicEquationsOverGF2::new(3, vec![vec![vec![0, 1, 2]]]).is_ok());
}

#[test]
fn test_algebraic_equations_over_gf2_paper_example() {
    // Canonical example from the issue: n=3, 3 equations, config [1,0,0]
    let p = satisfiable_problem();
    assert_eq!(p.evaluate(&[1, 0, 0]), Or(true));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&p).unwrap();
    assert_eq!(p.evaluate(&witness), Or(true));
}
