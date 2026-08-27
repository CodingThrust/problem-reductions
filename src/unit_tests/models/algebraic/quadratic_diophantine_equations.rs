use crate::models::algebraic::QuadraticDiophantineEquations;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Or;
use num_bigint::BigUint;

fn yes_problem() -> QuadraticDiophantineEquations {
    // a=3, b=5, c=53: x=1 gives y=10, x=4 gives y=1
    QuadraticDiophantineEquations::new(3, 5, 53)
}

fn no_problem() -> QuadraticDiophantineEquations {
    // a=3, b=5, c=10: x=1 gives 5y=7, not integer
    QuadraticDiophantineEquations::new(3, 5, 10)
}

fn bu(n: u32) -> BigUint {
    BigUint::from(n)
}

fn config_for_x(_problem: &QuadraticDiophantineEquations, x: u32) -> BigUint {
    bu(x)
}

#[test]
fn test_quadratic_diophantine_equations_creation_and_accessors() {
    let problem = yes_problem();
    assert_eq!(problem.a(), &bu(3));
    assert_eq!(problem.b(), &bu(5));
    assert_eq!(problem.c(), &bu(53));
    assert_eq!(problem.bit_length_a(), 2);
    assert_eq!(problem.bit_length_b(), 3);
    assert_eq!(problem.bit_length_c(), 6);
    // max_x = floor(sqrt(53 / 3)) = 4, encoded in 3 binary digits.
    assert_eq!(problem.dimensions(), vec![2, 2, 2]);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(
        <QuadraticDiophantineEquations as Problem>::NAME,
        "QuadraticDiophantineEquations"
    );
    assert_eq!(
        <QuadraticDiophantineEquations as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_quadratic_diophantine_equations_evaluate_yes() {
    let problem = yes_problem();
    assert_eq!(
        problem.evaluate(&config_for_x(&problem, 1)).unwrap(),
        Or(true)
    );
    assert_eq!(
        problem.evaluate(&config_for_x(&problem, 2)).unwrap(),
        Or(false)
    );
    assert_eq!(
        problem.evaluate(&config_for_x(&problem, 3)).unwrap(),
        Or(false)
    );
    assert_eq!(
        problem.evaluate(&config_for_x(&problem, 4)).unwrap(),
        Or(true)
    );
}

#[test]
fn test_quadratic_diophantine_equations_evaluate_no() {
    let problem = no_problem();
    assert_eq!(problem.dimensions(), vec![2]);
    assert_eq!(
        problem.evaluate(&config_for_x(&problem, 1)).unwrap(),
        Or(false)
    );
}

#[test]
fn test_quadratic_diophantine_equations_evaluate_invalid_config() {
    let problem = yes_problem();
    assert_eq!(problem.evaluate(&BigUint::default()).unwrap(), Or(false));
    assert_eq!(problem.evaluate(&bu(5)).unwrap(), Or(false));
}

#[test]
fn test_quadratic_diophantine_equations_c_le_a() {
    let problem = QuadraticDiophantineEquations::new(10, 1, 5);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&BigUint::default()).unwrap(), Or(false));
}

#[test]
fn test_quadratic_diophantine_equations_bigint_witness_encoding_round_trip() {
    let x = (BigUint::from(1u32) << 100usize) + BigUint::from(1u32);
    let serialized = serde_json::to_value(&x).unwrap();
    let restored: BigUint = serde_json::from_value(serialized).unwrap();
    assert_eq!(restored, x);
}

#[test]
fn test_quadratic_diophantine_equations_solver_finds_witness() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let witness = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&witness).unwrap(), Or(true));
    let x = witness;
    assert!(matches!(x, v if v == bu(1) || v == bu(4)));
}

#[test]
fn test_quadratic_diophantine_equations_solver_finds_all_witnesses() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(all.len(), 2);
    assert!(all
        .iter()
        .all(|sol| problem.evaluate(sol).unwrap() == Or(true)));
    let decoded = all
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(decoded, std::collections::BTreeSet::from([bu(1), bu(4)]));
}

#[test]
fn test_quadratic_diophantine_equations_solver_no_witness() {
    let problem = no_problem();
    let solver = BruteForce::new();
    assert!(solver.solve(&problem).unwrap().is_none());
}

#[test]
fn test_quadratic_diophantine_equations_serialization() {
    let problem = yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(json, serde_json::json!({"a": "3", "b": "5", "c": "53"}));

    let restored: QuadraticDiophantineEquations = serde_json::from_value(json).unwrap();
    assert_eq!(restored.a(), problem.a());
    assert_eq!(restored.b(), problem.b());
    assert_eq!(restored.c(), problem.c());
}

#[test]
fn test_quadratic_diophantine_equations_deserialization_rejects_invalid() {
    let result: Result<QuadraticDiophantineEquations, _> =
        serde_json::from_value(serde_json::json!({"a": 0, "b": 5, "c": 53}));
    assert!(result.is_err());

    let result: Result<QuadraticDiophantineEquations, _> =
        serde_json::from_value(serde_json::json!({"a": 3, "b": 0, "c": 53}));
    assert!(result.is_err());

    let result: Result<QuadraticDiophantineEquations, _> =
        serde_json::from_value(serde_json::json!({"a": 3, "b": 5, "c": 0}));
    assert!(result.is_err());
}

#[test]
fn test_quadratic_diophantine_equations_check_x() {
    let problem = yes_problem();
    assert_eq!(problem.check_x(&bu(1)), Some(bu(10)));
    assert_eq!(problem.check_x(&bu(2)), None);
    assert_eq!(problem.check_x(&bu(3)), None);
    assert_eq!(problem.check_x(&bu(4)), Some(bu(1)));
    assert_eq!(problem.check_x(&bu(5)), None);
    assert_eq!(problem.check_x(&BigUint::default()), None);
}

#[test]
fn test_quadratic_diophantine_equations_paper_example() {
    let problem = QuadraticDiophantineEquations::new(3, 5, 53);
    let config = config_for_x(&problem, 1);
    assert_eq!(problem.evaluate(&config).unwrap(), Or(true));

    let solver = BruteForce::new();
    let witness = solver.solve(&problem).unwrap().unwrap();
    assert_eq!(problem.evaluate(&witness).unwrap(), Or(true));
}

#[test]
#[should_panic(expected = "Coefficient a must be positive")]
fn test_quadratic_diophantine_equations_panics_on_zero_a() {
    QuadraticDiophantineEquations::new(0, 5, 53);
}

#[test]
#[should_panic(expected = "Coefficient b must be positive")]
fn test_quadratic_diophantine_equations_panics_on_zero_b() {
    QuadraticDiophantineEquations::new(3, 0, 53);
}

#[test]
#[should_panic(expected = "Right-hand side c must be positive")]
fn test_quadratic_diophantine_equations_panics_on_zero_c() {
    QuadraticDiophantineEquations::new(3, 5, 0);
}
