use super::*;
use crate::solvers::Solver;
use crate::traits::Problem;
use crate::types::{AggregationError, Max, Min, Or, Sum};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
struct MaxSumProblem {
    weights: Vec<i64>,
}

impl Problem for MaxSumProblem {
    const NAME: &'static str = "MaxSumProblem";
    type Value = Max<i64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Max(Some(
                config
                    .iter()
                    .zip(&self.weights)
                    .map(|(&c, &w)| if c == 1 { w } else { 0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

#[derive(Clone)]
struct MinSumProblem {
    weights: Vec<i64>,
}

impl Problem for MinSumProblem {
    const NAME: &'static str = "MinSumProblem";
    type Value = Min<i64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Min(Some(
                config
                    .iter()
                    .zip(&self.weights)
                    .map(|(&c, &w)| if c == 1 { w } else { 0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
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

    fn evaluate(&self, config: &[usize]) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Or(self.satisfying.iter().any(|s| s == config)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "bool")]
    }
}

#[derive(Clone)]
struct SumProblem {
    weights: Vec<u64>,
}

#[derive(Clone)]
struct EvaluationFailureProblem;

impl Problem for EvaluationFailureProblem {
    const NAME: &'static str = "EvaluationFailureProblem";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![2]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Or, crate::traits::EvaluationError> {
        if config == [1] {
            Err(crate::traits::EvaluationError::IntegerOverflow(
                "evaluating test configuration".to_string(),
            ))
        } else {
            Ok(Or(false))
        }
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

#[derive(Clone)]
struct AggregationFailureProblem;

impl Problem for AggregationFailureProblem {
    const NAME: &'static str = "AggregationFailureProblem";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![2]
    }

    fn evaluate(&self, _: &[usize]) -> Result<Sum<u64>, crate::traits::EvaluationError> {
        Ok(Sum(u64::MAX))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

#[derive(Clone)]
struct CountingSatProblem {
    evaluations: Rc<Cell<usize>>,
}

impl Problem for CountingSatProblem {
    const NAME: &'static str = "CountingSatProblem";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![2, 2]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            self.evaluations.set(self.evaluations.get() + 1);
            Or(config == [0, 0])
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

impl Problem for SumProblem {
    const NAME: &'static str = "SumProblem";
    type Value = Sum<u64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Sum(config
                .iter()
                .zip(&self.weights)
                .map(|(&c, &w)| if c == 1 { w } else { 0 })
                .sum())
        })
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

    assert_eq!(solver.solve(&problem).unwrap(), Max(Some(6)));
}

#[test]
fn test_solver_solves_min_value() {
    let problem = MinSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.solve(&problem).unwrap(), Min(Some(0)));
}

#[test]
fn test_solver_solves_satisfaction_value() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.solve(&problem).unwrap(), Or(true));
}

#[test]
fn test_solver_find_witness() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.find_witness(&problem).unwrap(), Some(vec![1, 1, 1]));
}

#[test]
fn test_solver_find_witness_for_satisfaction_problem() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let witness = solver.find_witness(&problem).unwrap();
    assert!(witness.is_some());
    assert_eq!(problem.evaluate(&witness.unwrap()).unwrap(), Or(true));
}

#[test]
fn test_solver_find_witness_stops_after_first_optimal_configuration() {
    let evaluations = Rc::new(Cell::new(0));
    let problem = CountingSatProblem {
        evaluations: Rc::clone(&evaluations),
    };

    assert_eq!(
        BruteForce::new().find_witness(&problem).unwrap(),
        Some(vec![0, 0])
    );
    // Four evaluations compute the aggregate; the witness pass stops at the
    // first configuration instead of collecting every optimal witness.
    assert_eq!(evaluations.get(), 5);
}

#[test]
fn test_solver_find_witness_returns_none_for_sum_problem() {
    let problem = SumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(solver.find_witness(&problem).unwrap(), None);
}

#[test]
fn test_solver_find_all_witnesses() {
    let problem = SatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    let solver = BruteForce::new();

    let witnesses = solver.find_all_witnesses(&problem).unwrap();
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

    assert!(solver.find_all_witnesses(&problem).unwrap().is_empty());
}

#[test]
fn test_solver_with_real_mis() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    let problem = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1i64; 3],
    );
    let solver = BruteForce::new();

    let best = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(best.len(), 3);
    for sol in &best {
        assert_eq!(sol.iter().sum::<usize>(), 1);
        assert!(problem.evaluate(sol).unwrap().is_valid());
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

    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(solutions.len(), 2);
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_solve_with_witnesses_max() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let (value, witnesses) = solver.solve_with_witnesses(&problem).unwrap();
    assert_eq!(value, Max(Some(6)));
    assert_eq!(witnesses, vec![vec![1, 1, 1]]);
}

#[test]
fn test_solve_with_witnesses_sum_returns_empty() {
    let problem = SumProblem {
        weights: vec![1, 2],
    };
    let solver = BruteForce::new();

    let (value, witnesses) = solver.solve_with_witnesses(&problem).unwrap();
    assert_eq!(value, Sum(6)); // 0+0 + 0+2 + 1+0 + 1+2 = 6
    assert!(witnesses.is_empty());
}

#[test]
fn test_solver_trait_solve() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    assert_eq!(Solver::solve(&solver, &problem).unwrap(), Max(Some(6)));
}

#[test]
fn test_solver_preserves_evaluation_errors() {
    let error = BruteForce::new()
        .solve(&EvaluationFailureProblem)
        .unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::SolveError::Evaluation(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
}

#[test]
fn test_solver_preserves_aggregation_errors() {
    let error = BruteForce::new()
        .solve(&AggregationFailureProblem)
        .unwrap_err();
    assert!(matches!(
        error,
        crate::solvers::SolveError::Aggregation(AggregationError::ArithmeticOverflow)
    ));
}
