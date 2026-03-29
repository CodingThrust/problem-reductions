use crate::models::algebraic::QuadraticDiophantineEquations;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn yes_problem() -> QuadraticDiophantineEquations {
    // a=3, b=5, c=53: x=1 gives y=10, x=4 gives y=1
    QuadraticDiophantineEquations::new(3, 5, 53)
}

fn no_problem() -> QuadraticDiophantineEquations {
    // a=3, b=5, c=10: x=1 gives 5y=7, not integer
    QuadraticDiophantineEquations::new(3, 5, 10)
}

#[test]
fn test_quadratic_diophantine_equations_basic() {
    let problem = yes_problem();
    assert_eq!(problem.a(), 3);
    assert_eq!(problem.b(), 5);
    assert_eq!(problem.c(), 53);
    // max_x = isqrt(53/3) = isqrt(17) = 4
    assert_eq!(problem.dims(), vec![4]);
    assert_eq!(problem.num_variables(), 1);
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
    // config[0]=0 -> x=1: 3*1 + 5y = 53, y=10
    assert_eq!(problem.evaluate(&[0]), Or(true));
    // config[0]=1 -> x=2: 3*4 + 5y = 53, 5y=41, not integer
    assert_eq!(problem.evaluate(&[1]), Or(false));
    // config[0]=2 -> x=3: 3*9 + 5y = 53, 5y=26, not integer
    assert_eq!(problem.evaluate(&[2]), Or(false));
    // config[0]=3 -> x=4: 3*16 + 5y = 53, 5y=5, y=1
    assert_eq!(problem.evaluate(&[3]), Or(true));
}

#[test]
fn test_quadratic_diophantine_equations_evaluate_no() {
    let problem = no_problem();
    // max_x = isqrt(10/3) = isqrt(3) = 1
    assert_eq!(problem.dims(), vec![1]);
    // config[0]=0 -> x=1: 3*1 + 5y = 10, 5y=7, not integer
    assert_eq!(problem.evaluate(&[0]), Or(false));
}

#[test]
fn test_quadratic_diophantine_equations_evaluate_invalid_config() {
    let problem = yes_problem();
    // Wrong length
    assert_eq!(problem.evaluate(&[]), Or(false));
    assert_eq!(problem.evaluate(&[0, 1]), Or(false));
}

#[test]
fn test_quadratic_diophantine_equations_solver_finds_witness() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&witness), Or(true));
}

#[test]
fn test_quadratic_diophantine_equations_solver_finds_all_witnesses() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem);
    // Two solutions: x=1 (config[0]=0) and x=4 (config[0]=3)
    assert_eq!(all.len(), 2);
    assert!(all.iter().all(|sol| problem.evaluate(sol) == Or(true)));
}

#[test]
fn test_quadratic_diophantine_equations_solver_no_witness() {
    let problem = no_problem();
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_quadratic_diophantine_equations_serialization() {
    let problem = yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "a": 3,
            "b": 5,
            "c": 53,
        })
    );

    let restored: QuadraticDiophantineEquations = serde_json::from_value(json).unwrap();
    assert_eq!(restored.a(), problem.a());
    assert_eq!(restored.b(), problem.b());
    assert_eq!(restored.c(), problem.c());
}

#[test]
fn test_quadratic_diophantine_equations_deserialization_rejects_invalid() {
    // a=0
    let result: Result<QuadraticDiophantineEquations, _> =
        serde_json::from_value(serde_json::json!({"a": 0, "b": 5, "c": 53}));
    assert!(result.is_err());
    // b=0
    let result: Result<QuadraticDiophantineEquations, _> =
        serde_json::from_value(serde_json::json!({"a": 3, "b": 0, "c": 53}));
    assert!(result.is_err());
    // c=0
    let result: Result<QuadraticDiophantineEquations, _> =
        serde_json::from_value(serde_json::json!({"a": 3, "b": 5, "c": 0}));
    assert!(result.is_err());
}

#[test]
fn test_quadratic_diophantine_equations_check_x() {
    let problem = yes_problem();
    assert_eq!(problem.check_x(1), Some(10)); // 3 + 50 = 53
    assert_eq!(problem.check_x(2), None); // 12 + 5y = 53, 41/5 not integer
    assert_eq!(problem.check_x(3), None); // 27 + 5y = 53, 26/5 not integer
    assert_eq!(problem.check_x(4), Some(1)); // 48 + 5 = 53
    assert_eq!(problem.check_x(5), None); // 75 > 53
    assert_eq!(problem.check_x(0), None); // x must be positive
}

#[test]
fn test_quadratic_diophantine_equations_edge_case_c_less_than_a() {
    // c < a: no valid x since a*1^2 = a > c
    let problem = QuadraticDiophantineEquations::new(10, 1, 5);
    assert_eq!(problem.dims(), vec![0]);
}

#[test]
fn test_quadratic_diophantine_equations_paper_example() {
    // From issue: a=3, b=5, c=53. x=1: y=10, x=4: y=1.
    let problem = QuadraticDiophantineEquations::new(3, 5, 53);
    // Verify the claimed solution x=1 (config[0]=0)
    assert_eq!(problem.evaluate(&[0]), Or(true));
    // Verify x=4 (config[0]=3) also works
    assert_eq!(problem.evaluate(&[3]), Or(true));

    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem);
    assert_eq!(all.len(), 2);
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
