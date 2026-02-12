use super::*;
use crate::solvers::Solver;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

// Simple maximization problem
#[derive(Clone)]
struct MaxSumOpt {
    weights: Vec<i32>,
}

impl Problem for MaxSumOpt {
    const NAME: &'static str = "MaxSumOpt";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0 })
            .sum()
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

impl OptimizationProblem for MaxSumOpt {
    fn direction(&self) -> Direction {
        Direction::Maximize
    }
    fn is_better(&self, a: &Self::Metric, b: &Self::Metric) -> bool {
        a > b
    }
}

// Simple minimization problem
#[derive(Clone)]
struct MinSumOpt {
    weights: Vec<i32>,
}

impl Problem for MinSumOpt {
    const NAME: &'static str = "MinSumOpt";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0 })
            .sum()
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

impl OptimizationProblem for MinSumOpt {
    fn direction(&self) -> Direction {
        Direction::Minimize
    }
    fn is_better(&self, a: &Self::Metric, b: &Self::Metric) -> bool {
        a < b
    }
}

// Satisfaction problem (Metric = bool)
#[derive(Clone)]
struct SatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl Problem for SatProblem {
    const NAME: &'static str = "SatProblem";
    type Metric = bool;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }
    fn evaluate(&self, config: &[usize]) -> bool {
        self.satisfying.iter().any(|s| s == config)
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "bool")]
    }
}

#[test]
fn test_solver_maximization() {
    let problem = MaxSumOpt {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![1, 1, 1]); // Select all for max sum = 6
}

#[test]
fn test_solver_minimization() {
    let problem = MinSumOpt {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![0, 0, 0]); // Select none for min sum = 0
}

#[test]
fn test_solver_multiple_optimal() {
    // Two variables with equal weights -> multiple optima
    let problem = MaxSumOpt {
        weights: vec![5, 5],
    };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![1, 1]); // Only one optimal: select both = 10
}

#[test]
fn test_solver_empty() {
    let problem = MaxSumOpt { weights: vec![] };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert!(best.is_empty());
}

#[test]
fn test_solver_find_satisfying() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_solver_find_satisfying_unsat() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![], // No satisfying assignment
    };
    let solver = BruteForce::new();

    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_solver_find_all_satisfying() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions.len(), 2);
    assert!(solutions.contains(&vec![1, 0]));
    assert!(solutions.contains(&vec![0, 1]));
}

#[test]
fn test_solver_find_satisfying_empty() {
    let problem = SatProblem {
        num_vars: 0,
        satisfying: vec![],
    };
    let solver = BruteForce::new();

    assert!(solver.find_satisfying(&problem).is_none());
    assert!(solver.find_all_satisfying(&problem).is_empty());
}

#[test]
fn test_solver_with_real_mis() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    // Triangle graph: MIS = 1
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 3); // Three single-vertex solutions
    for sol in &best {
        assert_eq!(sol.iter().sum::<usize>(), 1);
        assert!(problem.evaluate(sol).is_valid());
    }
}

#[test]
fn test_solver_with_real_sat() {
    use crate::models::satisfiability::{CNFClause, Satisfiability};
    use crate::traits::Problem;

    // (x1 OR x2) AND (NOT x1 OR NOT x2)
    let problem = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}
