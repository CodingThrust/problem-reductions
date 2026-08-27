use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::{Max, Min, Or, Sum};

#[derive(Clone)]
struct TestSatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl Problem for TestSatProblem {
    const NAME: &'static str = "TestSat";
    type Solution = Vec<usize>;
    type Value = Or;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Or(self.satisfying.iter().any(|s| s == config)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "bool")]
    }
}

impl crate::solvers::BruteForceProblem for TestSatProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }
}

#[test]
fn test_problem_sat() {
    let p = TestSatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };

    assert_eq!(p.dimensions(), vec![2, 2]);
    assert_eq!(p.evaluate(&vec![1, 0]).unwrap(), Or(true));
    assert_eq!(p.evaluate(&vec![0, 0]).unwrap(), Or(false));
}

#[test]
fn test_problem_num_variables() {
    let p = TestSatProblem {
        num_vars: 5,
        satisfying: vec![],
    };

    assert_eq!(p.num_variables(), 5);
    assert_eq!(p.dimensions().len(), 5);
}

#[test]
fn test_problem_empty() {
    let p = TestSatProblem {
        num_vars: 0,
        satisfying: vec![],
    };

    assert_eq!(p.num_variables(), 0);
    assert!(p.dimensions().is_empty());
}

#[derive(Clone)]
struct TestMaxProblem {
    weights: Vec<i64>,
}

impl Problem for TestMaxProblem {
    const NAME: &'static str = "TestMax";
    type Solution = Vec<usize>;
    type Value = Max<i64>;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Max(Some(
                config
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl crate::solvers::BruteForceProblem for TestMaxProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
}

#[derive(Clone)]
struct TestMinProblem {
    costs: Vec<i64>,
}

impl Problem for TestMinProblem {
    const NAME: &'static str = "TestMin";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Min(Some(
                config
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| if v == 1 { self.costs[i] } else { 0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl crate::solvers::BruteForceProblem for TestMinProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.costs.len()]
    }
}

#[test]
fn test_problem_max_value() {
    let p = TestMaxProblem {
        weights: vec![3, 1, 4],
    };

    assert_eq!(p.evaluate(&vec![1, 0, 1]).unwrap(), Max(Some(7)));
    assert_eq!(p.evaluate(&vec![0, 0, 0]).unwrap(), Max(Some(0)));
    assert_eq!(p.evaluate(&vec![1, 1, 1]).unwrap(), Max(Some(8)));
}

#[test]
fn test_problem_min_value() {
    let p = TestMinProblem {
        costs: vec![5, 2, 3],
    };

    assert_eq!(p.evaluate(&vec![1, 0, 0]).unwrap(), Min(Some(5)));
    assert_eq!(p.evaluate(&vec![0, 1, 1]).unwrap(), Min(Some(5)));
    assert_eq!(p.evaluate(&vec![0, 0, 0]).unwrap(), Min(Some(0)));
}

#[derive(Clone)]
struct MultiDimProblem {
    dims: Vec<usize>,
}

impl Problem for MultiDimProblem {
    const NAME: &'static str = "MultiDim";
    type Solution = Vec<usize>;
    type Value = Sum<i64>;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok(Sum(config.iter().map(|&c| c as i64).sum()))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i64")]
    }
}

impl crate::solvers::BruteForceProblem for MultiDimProblem {
    fn dimensions(&self) -> Vec<usize> {
        self.dims.clone()
    }
}

#[test]
fn test_multi_dim_problem() {
    let p = MultiDimProblem {
        dims: vec![2, 3, 4],
    };

    assert_eq!(p.dimensions(), vec![2, 3, 4]);
    assert_eq!(p.num_variables(), 3);
    assert_eq!(p.evaluate(&vec![0, 0, 0]).unwrap(), Sum(0));
    assert_eq!(p.evaluate(&vec![1, 2, 3]).unwrap(), Sum(6));
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
    type Solution = Vec<usize>;
    type Value = Max<f64>;

    crate::problem_size![("num_variables", num_variables)];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Self::Value, crate::traits::EvaluationError> {
        Ok({
            Max(Some(
                config
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0.0 })
                    .sum(),
            ))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "f64")]
    }
}

impl crate::solvers::BruteForceProblem for FloatProblem {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
}

#[test]
fn test_float_value_problem() {
    let p = FloatProblem {
        weights: vec![1.5, 2.5, 3.0],
    };

    assert_eq!(p.dimensions(), vec![2, 2, 2]);
    assert!((p.evaluate(&vec![1, 1, 0]).unwrap().0.unwrap() - 4.0).abs() < 1e-10);
    assert!((p.evaluate(&vec![1, 1, 1]).unwrap().0.unwrap() - 7.0).abs() < 1e-10);
}

#[test]
fn problem_type_bridge_returns_catalog_entry_for_registered_type() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let pt = MaximumIndependentSet::<SimpleGraph, i64>::problem_type();
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

    assert_eq!(p2.dimensions(), vec![2, 2]);
    assert_eq!(p2.evaluate(&vec![1, 0]).unwrap(), Or(true));
}
