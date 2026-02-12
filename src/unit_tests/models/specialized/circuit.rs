use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_boolean_expr_var() {
    let expr = BooleanExpr::var("x");
    let mut assignments = HashMap::new();
    assignments.insert("x".to_string(), true);
    assert!(expr.evaluate(&assignments));

    assignments.insert("x".to_string(), false);
    assert!(!expr.evaluate(&assignments));
}

#[test]
fn test_boolean_expr_const() {
    let t = BooleanExpr::constant(true);
    let f = BooleanExpr::constant(false);
    let assignments = HashMap::new();
    assert!(t.evaluate(&assignments));
    assert!(!f.evaluate(&assignments));
}

#[test]
fn test_boolean_expr_not() {
    let expr = BooleanExpr::not(BooleanExpr::var("x"));
    let mut assignments = HashMap::new();
    assignments.insert("x".to_string(), true);
    assert!(!expr.evaluate(&assignments));

    assignments.insert("x".to_string(), false);
    assert!(expr.evaluate(&assignments));
}

#[test]
fn test_boolean_expr_and() {
    let expr = BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]);
    let mut assignments = HashMap::new();

    assignments.insert("x".to_string(), true);
    assignments.insert("y".to_string(), true);
    assert!(expr.evaluate(&assignments));

    assignments.insert("y".to_string(), false);
    assert!(!expr.evaluate(&assignments));
}

#[test]
fn test_boolean_expr_or() {
    let expr = BooleanExpr::or(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]);
    let mut assignments = HashMap::new();

    assignments.insert("x".to_string(), false);
    assignments.insert("y".to_string(), false);
    assert!(!expr.evaluate(&assignments));

    assignments.insert("y".to_string(), true);
    assert!(expr.evaluate(&assignments));
}

#[test]
fn test_boolean_expr_xor() {
    let expr = BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]);
    let mut assignments = HashMap::new();

    assignments.insert("x".to_string(), true);
    assignments.insert("y".to_string(), true);
    assert!(!expr.evaluate(&assignments)); // XOR(T, T) = F

    assignments.insert("y".to_string(), false);
    assert!(expr.evaluate(&assignments)); // XOR(T, F) = T
}

#[test]
fn test_boolean_expr_variables() {
    let expr = BooleanExpr::and(vec![
        BooleanExpr::var("x"),
        BooleanExpr::or(vec![BooleanExpr::var("y"), BooleanExpr::var("z")]),
    ]);
    let vars = expr.variables();
    assert_eq!(vars, vec!["x", "y", "z"]);
}

#[test]
fn test_assignment_satisfied() {
    let assign = Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    );

    let mut assignments = HashMap::new();
    assignments.insert("x".to_string(), true);
    assignments.insert("y".to_string(), true);
    assignments.insert("c".to_string(), true);
    assert!(assign.is_satisfied(&assignments));

    assignments.insert("c".to_string(), false);
    assert!(!assign.is_satisfied(&assignments));
}

#[test]
fn test_circuit_variables() {
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        ),
        Assignment::new(
            vec!["d".to_string()],
            BooleanExpr::or(vec![BooleanExpr::var("c"), BooleanExpr::var("z")]),
        ),
    ]);
    let vars = circuit.variables();
    assert_eq!(vars, vec!["c", "d", "x", "y", "z"]);
}

#[test]
fn test_circuit_sat_creation() {
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::<i32>::new(circuit);
    assert_eq!(problem.num_variables(), 3); // c, x, y
    assert_eq!(problem.num_flavors(), 2);
}

#[test]
fn test_circuit_sat_solution_size() {
    // c = x AND y
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::<i32>::new(circuit);

    // Variables sorted: c, x, y
    // c=1, x=1, y=1 -> c = 1 AND 1 = 1, valid
    let sol = problem.solution_size(&[1, 1, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    // c=0, x=0, y=0 -> c = 0 AND 0 = 0, valid
    let sol = problem.solution_size(&[0, 0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    // c=1, x=0, y=0 -> c should be 0, but c=1, invalid
    let sol = problem.solution_size(&[1, 0, 0]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 0);
}

#[test]
fn test_circuit_sat_brute_force() {
    // c = x AND y
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::<i32>::new(circuit);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // All satisfying: c matches x AND y
    // 4 valid configs: (0,0,0), (0,0,1), (0,1,0), (1,1,1)
    assert_eq!(solutions.len(), 4);
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_circuit_sat_complex() {
    // c = x AND y
    // d = c OR z
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        ),
        Assignment::new(
            vec!["d".to_string()],
            BooleanExpr::or(vec![BooleanExpr::var("c"), BooleanExpr::var("z")]),
        ),
    ]);
    let problem = CircuitSAT::<i32>::new(circuit);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // All valid solutions satisfy both assignments
    for sol in &solutions {
        let sol_size = problem.solution_size(sol);
        assert!(sol_size.is_valid);
        assert_eq!(sol_size.size, 2);
    }
}

#[test]
fn test_is_circuit_satisfying() {
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);

    let mut assignments = HashMap::new();
    assignments.insert("x".to_string(), true);
    assignments.insert("y".to_string(), true);
    assignments.insert("c".to_string(), true);
    assert!(is_circuit_satisfying(&circuit, &assignments));

    assignments.insert("c".to_string(), false);
    assert!(!is_circuit_satisfying(&circuit, &assignments));
}

#[test]
fn test_problem_size() {
    let circuit = Circuit::new(vec![
        Assignment::new(vec!["c".to_string()], BooleanExpr::var("x")),
        Assignment::new(vec!["d".to_string()], BooleanExpr::var("y")),
    ]);
    let problem = CircuitSAT::<i32>::new(circuit);
    let size = problem.problem_size();
    assert_eq!(size.get("num_variables"), Some(4));
    assert_eq!(size.get("num_assignments"), Some(2));
}

#[test]
fn test_energy_mode() {
    let circuit = Circuit::new(vec![]);
    let problem = CircuitSAT::<i32>::new(circuit);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_empty_circuit() {
    let circuit = Circuit::new(vec![]);
    let problem = CircuitSAT::<i32>::new(circuit);
    let sol = problem.solution_size(&[]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);
}

#[test]
fn test_weighted_circuit_sat() {
    let circuit = Circuit::new(vec![
        Assignment::new(vec!["c".to_string()], BooleanExpr::var("x")),
        Assignment::new(vec!["d".to_string()], BooleanExpr::var("y")),
    ]);
    let problem = CircuitSAT::with_weights(circuit, vec![10, 1]);

    // Variables sorted: c, d, x, y
    // Config [1, 0, 1, 0]: c=1, d=0, x=1, y=0
    // c=x (1=1) satisfied (weight 10), d=y (0=0) satisfied (weight 1)
    let sol = problem.solution_size(&[1, 0, 1, 0]);
    assert_eq!(sol.size, 11); // Both satisfied: 10 + 1
    assert!(sol.is_valid);

    // Config [1, 0, 0, 0]: c=1, d=0, x=0, y=0
    // c=x (1!=0) not satisfied, d=y (0=0) satisfied (weight 1)
    let sol = problem.solution_size(&[1, 0, 0, 0]);
    assert_eq!(sol.size, 1); // Only d=y satisfied
    assert!(!sol.is_valid);

    // Config [0, 1, 0, 0]: c=0, d=1, x=0, y=0
    // c=x (0=0) satisfied (weight 10), d=y (1!=0) not satisfied
    let sol = problem.solution_size(&[0, 1, 0, 0]);
    assert_eq!(sol.size, 10); // Only c=x satisfied
    assert!(!sol.is_valid);
}

#[test]
fn test_circuit_sat_problem_v2() {
    use crate::traits::ProblemV2;

    // c = x AND y
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let p = CircuitSAT::<i32>::new(circuit);

    // Variables sorted: c, x, y
    assert_eq!(p.dims(), vec![2, 2, 2]);

    // c=1, x=1, y=1: c = 1 AND 1 = 1 => satisfied
    assert!(p.evaluate(&[1, 1, 1]));
    // c=0, x=0, y=0: c = 0 AND 0 = 0 => satisfied (c=0 matches)
    assert!(p.evaluate(&[0, 0, 0]));
    // c=1, x=1, y=0: c = 1 AND 0 = 0 != 1 => not satisfied
    assert!(!p.evaluate(&[1, 1, 0]));
}
