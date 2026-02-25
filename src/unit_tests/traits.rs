use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

// === Problem trait tests ===

#[derive(Clone)]
struct TestSatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl Problem for TestSatProblem {
    const NAME: &'static str = "TestSat";
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
fn test_problem_sat() {
    let p = TestSatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    assert_eq!(p.dims(), vec![2, 2]);
    assert!(p.evaluate(&[1, 0]));
    assert!(!p.evaluate(&[0, 0]));
}

#[test]
fn test_problem_num_variables() {
    let p = TestSatProblem {
        num_vars: 5,
        satisfying: vec![],
    };
    assert_eq!(p.num_variables(), 5);
    assert_eq!(p.dims().len(), 5);
}

#[test]
fn test_problem_empty() {
    let p = TestSatProblem {
        num_vars: 0,
        satisfying: vec![],
    };
    assert_eq!(p.num_variables(), 0);
    assert!(p.dims().is_empty());
}

// === OptimizationProblem trait tests ===

#[derive(Clone)]
struct TestMaxProblem {
    weights: Vec<i32>,
}

impl Problem for TestMaxProblem {
    const NAME: &'static str = "TestMax";
    type Metric = SolutionSize<i32>;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        SolutionSize::Valid(
            config
                .iter()
                .enumerate()
                .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0 })
                .sum(),
        )
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

impl OptimizationProblem for TestMaxProblem {
    type Value = i32;
    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

#[derive(Clone)]
struct TestMinProblem {
    costs: Vec<i32>,
}

impl Problem for TestMinProblem {
    const NAME: &'static str = "TestMin";
    type Metric = SolutionSize<i32>;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.costs.len()]
    }
    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        SolutionSize::Valid(
            config
                .iter()
                .enumerate()
                .map(|(i, &v)| if v == 1 { self.costs[i] } else { 0 })
                .sum(),
        )
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

impl OptimizationProblem for TestMinProblem {
    type Value = i32;
    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

#[test]
fn test_optimization_problem_maximize() {
    let p = TestMaxProblem {
        weights: vec![3, 1, 4],
    };
    assert_eq!(p.evaluate(&[1, 0, 1]), SolutionSize::Valid(7));
    assert_eq!(p.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
    assert_eq!(p.evaluate(&[1, 1, 1]), SolutionSize::Valid(8));
    assert_eq!(p.direction(), Direction::Maximize);
}

#[test]
fn test_optimization_problem_minimize() {
    let p = TestMinProblem {
        costs: vec![5, 2, 3],
    };
    assert_eq!(p.evaluate(&[1, 0, 0]), SolutionSize::Valid(5));
    assert_eq!(p.evaluate(&[0, 1, 1]), SolutionSize::Valid(5));
    assert_eq!(p.evaluate(&[0, 0, 0]), SolutionSize::Valid(0));
    assert_eq!(p.direction(), Direction::Minimize);
}

// === Multi-dimension (non-binary) problems ===

#[derive(Clone)]
struct MultiDimProblem {
    dims: Vec<usize>,
}

impl Problem for MultiDimProblem {
    const NAME: &'static str = "MultiDim";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        self.dims.clone()
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        config.iter().map(|&c| c as i32).sum()
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

#[test]
fn test_multi_dim_problem() {
    // 3 variables with cardinalities [2, 3, 4]
    let p = MultiDimProblem {
        dims: vec![2, 3, 4],
    };
    assert_eq!(p.dims(), vec![2, 3, 4]);
    assert_eq!(p.num_variables(), 3);
    assert_eq!(p.evaluate(&[0, 0, 0]), 0);
    assert_eq!(p.evaluate(&[1, 2, 3]), 6);
}

// === Problem NAME constant ===

#[test]
fn test_problem_name() {
    assert_eq!(TestSatProblem::NAME, "TestSat");
    assert_eq!(TestMaxProblem::NAME, "TestMax");
    assert_eq!(TestMinProblem::NAME, "TestMin");
    assert_eq!(MultiDimProblem::NAME, "MultiDim");
}

// === Problem with f64 metric ===

#[derive(Clone)]
struct FloatProblem {
    weights: Vec<f64>,
}

impl Problem for FloatProblem {
    const NAME: &'static str = "FloatProblem";
    type Metric = SolutionSize<f64>;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
    fn evaluate(&self, config: &[usize]) -> SolutionSize<f64> {
        SolutionSize::Valid(
            config
                .iter()
                .enumerate()
                .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0.0 })
                .sum(),
        )
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "f64")]
    }
}

impl OptimizationProblem for FloatProblem {
    type Value = f64;
    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

#[test]
fn test_float_metric_problem() {
    let p = FloatProblem {
        weights: vec![1.5, 2.5, 3.0],
    };
    assert_eq!(p.dims(), vec![2, 2, 2]);
    assert!((p.evaluate(&[1, 1, 0]).unwrap() - 4.0).abs() < 1e-10);
    assert!((p.evaluate(&[1, 1, 1]).unwrap() - 7.0).abs() < 1e-10);
    assert_eq!(p.direction(), Direction::Maximize);
}

// === Clone constraint ===

#[test]
fn test_problem_is_clone() {
    let p1 = TestSatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0]],
    };
    let p2 = p1.clone();
    assert_eq!(p2.dims(), vec![2, 2]);
    assert!(p2.evaluate(&[1, 0]));
}
