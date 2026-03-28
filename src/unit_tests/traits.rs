use crate::traits::Problem;
use crate::types::{Max, Min, Or, Sum};

#[derive(Clone)]
struct TestSatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl Problem for TestSatProblem {
    const NAME: &'static str = "TestSat";
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

#[test]
fn test_problem_sat() {
    let p = TestSatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };

    assert_eq!(p.dims(), vec![2, 2]);
    assert_eq!(p.evaluate(&[1, 0]), Or(true));
    assert_eq!(p.evaluate(&[0, 0]), Or(false));
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

#[derive(Clone)]
struct TestMaxProblem {
    weights: Vec<i32>,
}

impl Problem for TestMaxProblem {
    const NAME: &'static str = "TestMax";
    type Value = Max<i32>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Max(Some(
            config
                .iter()
                .enumerate()
                .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0 })
                .sum(),
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

#[derive(Clone)]
struct TestMinProblem {
    costs: Vec<i32>,
}

impl Problem for TestMinProblem {
    const NAME: &'static str = "TestMin";
    type Value = Min<i32>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.costs.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Min(Some(
            config
                .iter()
                .enumerate()
                .map(|(i, &v)| if v == 1 { self.costs[i] } else { 0 })
                .sum(),
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

#[test]
fn test_problem_max_value() {
    let p = TestMaxProblem {
        weights: vec![3, 1, 4],
    };

    assert_eq!(p.evaluate(&[1, 0, 1]), Max(Some(7)));
    assert_eq!(p.evaluate(&[0, 0, 0]), Max(Some(0)));
    assert_eq!(p.evaluate(&[1, 1, 1]), Max(Some(8)));
}

#[test]
fn test_problem_min_value() {
    let p = TestMinProblem {
        costs: vec![5, 2, 3],
    };

    assert_eq!(p.evaluate(&[1, 0, 0]), Min(Some(5)));
    assert_eq!(p.evaluate(&[0, 1, 1]), Min(Some(5)));
    assert_eq!(p.evaluate(&[0, 0, 0]), Min(Some(0)));
}

#[derive(Clone)]
struct MultiDimProblem {
    dims: Vec<usize>,
}

impl Problem for MultiDimProblem {
    const NAME: &'static str = "MultiDim";
    type Value = Sum<i32>;

    fn dims(&self) -> Vec<usize> {
        self.dims.clone()
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Sum(config.iter().map(|&c| c as i32).sum())
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}

#[test]
fn test_multi_dim_problem() {
    let p = MultiDimProblem {
        dims: vec![2, 3, 4],
    };

    assert_eq!(p.dims(), vec![2, 3, 4]);
    assert_eq!(p.num_variables(), 3);
    assert_eq!(p.evaluate(&[0, 0, 0]), Sum(0));
    assert_eq!(p.evaluate(&[1, 2, 3]), Sum(6));
}

#[test]
fn test_problem_name() {
    assert_eq!(TestSatProblem::NAME, "TestSat");
    assert_eq!(TestMaxProblem::NAME, "TestMax");
    assert_eq!(TestMinProblem::NAME, "TestMin");
    assert_eq!(MultiDimProblem::NAME, "MultiDim");
}

#[derive(Clone)]
struct FloatProblem {
    weights: Vec<f64>,
}

impl Problem for FloatProblem {
    const NAME: &'static str = "FloatProblem";
    type Value = Max<f64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Self::Value {
        Max(Some(
            config
                .iter()
                .enumerate()
                .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0.0 })
                .sum(),
        ))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "f64")]
    }
}

#[test]
fn test_float_value_problem() {
    let p = FloatProblem {
        weights: vec![1.5, 2.5, 3.0],
    };

    assert_eq!(p.dims(), vec![2, 2, 2]);
    assert!((p.evaluate(&[1, 1, 0]).0.unwrap() - 4.0).abs() < 1e-10);
    assert!((p.evaluate(&[1, 1, 1]).0.unwrap() - 7.0).abs() < 1e-10);
}

#[test]
fn problem_type_bridge_returns_catalog_entry_for_registered_type() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let pt = MaximumIndependentSet::<SimpleGraph, i32>::problem_type();
    assert_eq!(pt.canonical_name, "MaximumIndependentSet");
    assert!(!pt.display_name.is_empty());
    assert!(!pt.dimensions.is_empty());
}

#[test]
fn test_problem_is_clone() {
    let p1 = TestSatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0]],
    };
    let p2 = p1.clone();

    assert_eq!(p2.dims(), vec![2, 2]);
    assert_eq!(p2.evaluate(&[1, 0]), Or(true));
}
