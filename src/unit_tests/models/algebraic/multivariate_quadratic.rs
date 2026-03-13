use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_multivariate_quadratic_basic() {
    // F_2, 3 variables, 2 equations
    let eq1 = QuadraticPoly {
        quadratic_terms: vec![(0, 1)], // x0 * x1
        linear_terms: vec![2],         // + x2
        constant: false,
    };
    let eq2 = QuadraticPoly {
        quadratic_terms: vec![(1, 2)], // x1 * x2
        linear_terms: vec![0],         // + x0
        constant: false,
    };
    let problem = MultivariateQuadratic::new(3, vec![eq1, eq2]);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.num_equations(), 2);
    assert_eq!(problem.dims(), vec![2, 2, 2]);
}

#[test]
fn test_multivariate_quadratic_evaluate_f2() {
    // f1: x0*x1 + x2 = 0, f2: x1*x2 + x0 = 0 over F_2
    // Solutions: (0,0,0), (0,1,0), (1,1,1)
    let eq1 = QuadraticPoly {
        quadratic_terms: vec![(0, 1)],
        linear_terms: vec![2],
        constant: false,
    };
    let eq2 = QuadraticPoly {
        quadratic_terms: vec![(1, 2)],
        linear_terms: vec![0],
        constant: false,
    };
    let problem = MultivariateQuadratic::new(3, vec![eq1, eq2]);

    // Test known solutions
    assert!(problem.evaluate(&[0, 0, 0])); // 0*0+0=0, 0*0+0=0
    assert!(problem.evaluate(&[0, 1, 0])); // 0*1+0=0, 1*0+0=0
    assert!(problem.evaluate(&[1, 1, 1])); // 1*1+1=0 mod 2, 1*1+1=0 mod 2

    // Test non-solutions
    assert!(!problem.evaluate(&[1, 0, 0])); // 1*0+0=0, 0*0+1=1 != 0
    assert!(!problem.evaluate(&[1, 0, 1])); // 1*0+1=1 != 0
}

#[test]
fn test_multivariate_quadratic_no_solution() {
    // f1: x0+x1=0, f2: x0+x1+1=0 over F_2 (contradictory)
    let eq1 = QuadraticPoly {
        quadratic_terms: vec![],
        linear_terms: vec![0, 1],
        constant: false,
    };
    let eq2 = QuadraticPoly {
        quadratic_terms: vec![],
        linear_terms: vec![0, 1],
        constant: true,
    };
    let problem = MultivariateQuadratic::new(2, vec![eq1, eq2]);

    // No configuration should satisfy both
    assert!(!problem.evaluate(&[0, 0]));
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_multivariate_quadratic_brute_force() {
    // Same F_2 instance
    let eq1 = QuadraticPoly {
        quadratic_terms: vec![(0, 1)],
        linear_terms: vec![2],
        constant: false,
    };
    let eq2 = QuadraticPoly {
        quadratic_terms: vec![(1, 2)],
        linear_terms: vec![0],
        constant: false,
    };
    let problem = MultivariateQuadratic::new(3, vec![eq1, eq2]);

    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));

    let all_solutions = solver.find_all_satisfying(&problem);
    assert_eq!(all_solutions.len(), 3);
    for sol in &all_solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_multivariate_quadratic_brute_force_no_solution() {
    let eq1 = QuadraticPoly {
        quadratic_terms: vec![],
        linear_terms: vec![0, 1],
        constant: false,
    };
    let eq2 = QuadraticPoly {
        quadratic_terms: vec![],
        linear_terms: vec![0, 1],
        constant: true,
    };
    let problem = MultivariateQuadratic::new(2, vec![eq1, eq2]);

    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
    assert!(solver.find_all_satisfying(&problem).is_empty());
}

#[test]
fn test_multivariate_quadratic_serialization() {
    let eq1 = QuadraticPoly {
        quadratic_terms: vec![(0, 1)],
        linear_terms: vec![2],
        constant: false,
    };
    let problem = MultivariateQuadratic::new(3, vec![eq1]);

    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MultivariateQuadratic = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_variables(), problem.num_variables());
    assert_eq!(deserialized.num_equations(), problem.num_equations());
}

#[test]
fn test_multivariate_quadratic_empty_equations() {
    // No equations = every assignment satisfies
    let problem = MultivariateQuadratic::new(2, vec![]);
    assert!(problem.evaluate(&[0, 0]));
    assert!(problem.evaluate(&[1, 1]));
}

#[test]
fn test_multivariate_quadratic_constant_equation() {
    // f = 1 (constant polynomial that always evaluates to 1, never 0)
    let eq = QuadraticPoly {
        quadratic_terms: vec![],
        linear_terms: vec![],
        constant: true,
    };
    let problem = MultivariateQuadratic::new(2, vec![eq]);
    assert!(!problem.evaluate(&[0, 0]));
    assert!(!problem.evaluate(&[1, 1]));

    // f = 0 (constant polynomial that always evaluates to 0, always satisfied)
    let eq = QuadraticPoly {
        quadratic_terms: vec![],
        linear_terms: vec![],
        constant: false,
    };
    let problem = MultivariateQuadratic::new(2, vec![eq]);
    assert!(problem.evaluate(&[0, 0]));
    assert!(problem.evaluate(&[1, 1]));
}
