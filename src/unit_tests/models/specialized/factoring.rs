use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_factoring_creation() {
    let problem = Factoring::new(3, 3, 15);
    assert_eq!(problem.m(), 3);
    assert_eq!(problem.n(), 3);
    assert_eq!(problem.target(), 15);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.num_flavors(), 2);
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
fn test_solution_size_valid() {
    let problem = Factoring::new(2, 2, 6);
    // 2 * 3 = 6
    let sol = problem.solution_size(&[0, 1, 1, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0); // Exact match

    // 3 * 2 = 6
    let sol = problem.solution_size(&[1, 1, 0, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);
}

#[test]
fn test_solution_size_invalid() {
    let problem = Factoring::new(2, 2, 6);
    // 2 * 2 = 4 != 6
    let sol = problem.solution_size(&[0, 1, 0, 1]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 2); // Distance from 6

    // 1 * 1 = 1 != 6
    let sol = problem.solution_size(&[1, 0, 1, 0]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 5); // Distance from 6
}

#[test]
fn test_brute_force_factor_6() {
    let problem = Factoring::new(2, 2, 6);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Should find 2*3 and 3*2
    assert!(!solutions.is_empty());
    for sol in &solutions {
        let (a, b) = problem.read_factors(sol);
        assert_eq!(a * b, 6);
    }
}

#[test]
fn test_brute_force_factor_15() {
    let problem = Factoring::new(3, 3, 15);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Should find 3*5, 5*3, 1*15, 15*1
    for sol in &solutions {
        let (a, b) = problem.read_factors(sol);
        assert_eq!(a * b, 15);
    }
}

#[test]
fn test_brute_force_prime() {
    // 7 is prime, only 1*7 and 7*1 work
    let problem = Factoring::new(3, 3, 7);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    let factor_pairs: Vec<_> = solutions.iter().map(|s| problem.read_factors(s)).collect();

    // Should find at least one of (1,7) or (7,1)
    assert!(factor_pairs.contains(&(1, 7)) || factor_pairs.contains(&(7, 1)));
}

#[test]
fn test_is_factoring_function() {
    assert!(is_factoring(6, 2, 3));
    assert!(is_factoring(6, 3, 2));
    assert!(is_factoring(15, 3, 5));
    assert!(!is_factoring(6, 2, 2));
}

#[test]
fn test_energy_mode() {
    let problem = Factoring::new(2, 2, 6);
    assert!(problem.energy_mode().is_minimization());
}

#[test]
fn test_problem_size() {
    let problem = Factoring::new(3, 4, 12);
    let size = problem.problem_size();
    assert_eq!(size.get("num_bits_first"), Some(3));
    assert_eq!(size.get("num_bits_second"), Some(4));
    assert_eq!(size.get("target"), Some(12));
}

#[test]
fn test_is_valid_factorization() {
    let problem = Factoring::new(2, 2, 6);
    assert!(problem.is_valid_factorization(&[0, 1, 1, 1])); // 2*3=6
    assert!(!problem.is_valid_factorization(&[0, 1, 0, 1])); // 2*2=4
}

#[test]
fn test_factor_one() {
    // Factor 1: only 1*1 works
    let problem = Factoring::new(2, 2, 1);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        let (a, b) = problem.read_factors(sol);
        assert_eq!(a * b, 1);
    }
}

#[test]
fn test_factoring_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // Factor 6 with 2-bit factors
    let p = Factoring::new(2, 2, 6);
    assert_eq!(p.dims(), vec![2, 2, 2, 2]);

    // Bits [0,1, 1,1] = a=2, b=3, product=6, distance=0
    assert_eq!(p.evaluate(&[0, 1, 1, 1]), 0);
    // Bits [1,1, 0,1] = a=3, b=2, product=6, distance=0
    assert_eq!(p.evaluate(&[1, 1, 0, 1]), 0);
    // Bits [0,0, 0,0] = a=0, b=0, product=0, distance=6
    assert_eq!(p.evaluate(&[0, 0, 0, 0]), 6);

    assert_eq!(p.direction(), Direction::Minimize);
}
