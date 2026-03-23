use super::*;
use crate::solvers::Solver;
use crate::traits::Problem;
use crate::types::{Max, Min, Or, Sum};

#[derive(Clone)]
struct MaxSumProblem {
    weights: Vec<i32>,
}

impl Problem for MaxSumProblem {
    const NAME: &'static str = "MaxSumProblem";
    type Value = Max<i32>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Max(Some(
            config
                .iter()
                .zip(&self.weights)
                .map(|(&c, &w)| if c == 1 { w } else { 0 })
                .sum(),
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

#[derive(Clone)]
struct MinSumProblem {
    weights: Vec<i32>,
}

impl Problem for MinSumProblem {
    const NAME: &'static str = "MinSumProblem";
    type Value = Min<i32>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Min(Some(
            config
                .iter()
                .zip(&self.weights)
                .map(|(&c, &w)| if c == 1 { w } else { 0 })
                .sum(),
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

#[derive(Clone)]
struct SatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl Problem for SatProblem {
    const NAME: &'static str = "SatProblem";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Or(self.satisfying.iter().any(|s| s == config))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "bool")]
    }
}

#[derive(Clone)]
struct SumProblem {
    weights: Vec<u64>,
}

impl Problem for SumProblem {
    const NAME: &'static str = "SumProblem";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Sum(config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0 })
            .sum())
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "u64")]
    }
}

#[test]
fn test_solver_solves_max_value() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.solve(&problem), Max(Some(6)));
}

#[test]
fn test_solver_solves_min_value() {
    let problem = MinSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.solve(&problem), Min(Some(0)));
}

#[test]
fn test_solver_solves_satisfaction_value() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.solve(&problem), Or(true));
}

#[test]
fn test_solver_find_witness() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.find_witness(&problem), Some(vec![1, 1, 1]));
}

#[test]
fn test_solver_find_witness_for_satisfaction_problem() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    assert_eq!(problem.evaluate(&witness.unwrap()), Or(true));
}

#[test]
fn test_solver_find_witness_returns_none_for_sum_problem() {
    let problem = SumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.find_witness(&problem), None);
}

#[test]
fn test_solver_find_all_witnesses() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let witnesses = solver.find_all_witnesses(&problem);
    assert_eq!(witnesses.len(), 2);
    assert!(witnesses.contains(&vec![1, 0]));
    assert!(witnesses.contains(&vec![0, 1]));
}

#[test]
fn test_solver_find_all_witnesses_returns_empty_for_sum_problem() {
    let problem = SumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert!(solver.find_all_witnesses(&problem).is_empty());
}

#[test]
fn test_solver_with_real_mis() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i32; 3],
    );
    let solver = BruteForce::new();

    let best = solver.find_all_witnesses(&problem);
    assert_eq!(best.len(), 3);
    for sol in &best {
        assert_eq!(sol.iter().sum::<usize>(), 1);
        assert!(problem.evaluate(sol).is_valid());
    }
}

#[test]
fn test_solver_with_real_sat() {
    use crate::models::formula::{CNFClause, Satisfiability};
    use crate::traits::Problem;

    let problem = Satisfiability::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_solve_with_witnesses_max() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let (value, witnesses) = solver.solve_with_witnesses(&problem);
    assert_eq!(value, Max(Some(6)));
    assert_eq!(witnesses, vec![vec![1, 1, 1]]);
}

#[test]
fn test_solve_with_witnesses_sum_returns_empty() {
    let problem = SumProblem {
        weights: vec![1, 2],
    };
    let solver = BruteForce::new();

    let (value, witnesses) = solver.solve_with_witnesses(&problem);
    assert_eq!(value, Sum(6)); // 0+0 + 0+2 + 1+0 + 1+2 = 6
    assert!(witnesses.is_empty());
}

#[test]
fn test_solver_trait_solve() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(Solver::solve(&solver, &problem), Max(Some(6)));
}
