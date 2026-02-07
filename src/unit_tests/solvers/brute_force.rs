use super::*;
use crate::types::{EnergyMode, ProblemSize};

// Simple maximization problem: maximize sum of selected weights
#[derive(Clone)]
struct MaxSumProblem {
    weights: Vec<i32>,
}

impl Problem for MaxSumProblem {
    const NAME: &'static str = "MaxSumProblem";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.weights.len()
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("variables", self.weights.len())])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let sum: i32 = config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0 })
            .sum();
        SolutionSize::valid(sum)
    }
}

// Simple minimization problem: minimize sum of selected weights
#[derive(Clone)]
struct MinSumProblem {
    weights: Vec<i32>,
}

impl Problem for MinSumProblem {
    const NAME: &'static str = "MinSumProblem";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.weights.len()
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("variables", self.weights.len())])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let sum: i32 = config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0 })
            .sum();
        SolutionSize::valid(sum)
    }
}

// Problem with validity constraint: select at most one
#[derive(Clone)]
struct SelectAtMostOneProblem {
    weights: Vec<i32>,
}

impl Problem for SelectAtMostOneProblem {
    const NAME: &'static str = "SelectAtMostOneProblem";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.weights.len()
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("variables", self.weights.len())])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let selected: usize = config.iter().sum();
        let sum: i32 = config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0 })
            .sum();
        SolutionSize::new(sum, selected <= 1)
    }
}

#[test]
fn test_variant_for_test_problems() {
    // Test that variant() works for all test problems
    let v = MaxSumProblem::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], ("graph", "SimpleGraph"));
    assert_eq!(v[1], ("weight", "i32"));

    let v = MinSumProblem::variant();
    assert_eq!(v.len(), 2);

    let v = SelectAtMostOneProblem::variant();
    assert_eq!(v.len(), 2);

    let v = FloatProblem::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[1], ("weight", "f64"));
}

#[test]
fn test_brute_force_maximization() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![1, 1, 1]); // Select all for max sum = 6
}

#[test]
fn test_brute_force_minimization() {
    let problem = MinSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![0, 0, 0]); // Select none for min sum = 0
}

#[test]
fn test_brute_force_with_validity() {
    let problem = SelectAtMostOneProblem {
        weights: vec![1, 5, 3],
    };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![0, 1, 0]); // Select weight 5 (max single)
}

#[test]
fn test_brute_force_multiple_optimal() {
    let problem = MaxSumProblem {
        weights: vec![1, 1, 1],
    };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![1, 1, 1]); // All equal, so only one optimal

    // Problem with multiple optimal solutions
    let problem2 = SelectAtMostOneProblem {
        weights: vec![5, 5, 3],
    };
    let best2 = solver.find_best(&problem2);
    assert_eq!(best2.len(), 2); // Both [1,0,0] and [0,1,0] give weight 5
}

#[test]
fn test_brute_force_with_size() {
    let problem = MaxSumProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new();

    let best = solver.find_best_with_size(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0].0, vec![1, 1, 1]);
    assert_eq!(best[0].1.size, 6);
    assert!(best[0].1.is_valid);
}

#[test]
fn test_brute_force_empty_problem() {
    let problem = MaxSumProblem { weights: vec![] };
    let solver = BruteForce::new();

    let best = solver.find_best(&problem);
    assert!(best.is_empty());
}

#[test]
fn test_brute_force_valid_only_false() {
    let problem = SelectAtMostOneProblem {
        weights: vec![1, 2, 3],
    };
    let solver = BruteForce::new().valid_only(false);

    let best = solver.find_best(&problem);
    // With valid_only=false, the best is selecting all (sum=6) even though invalid
    assert_eq!(best.len(), 1);
    assert_eq!(best[0], vec![1, 1, 1]);
}

#[test]
fn test_brute_force_with_tolerance() {
    let solver = BruteForce::with_tolerance(0.01, 0.01);
    assert_eq!(solver.atol, 0.01);
    assert_eq!(solver.rtol, 0.01);
}

// Float problem for testing BruteForceFloat
#[derive(Clone)]
struct FloatProblem {
    weights: Vec<f64>,
}

impl Problem for FloatProblem {
    const NAME: &'static str = "FloatProblem";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "f64")]
    }

    type Size = f64;

    fn num_variables(&self) -> usize {
        self.weights.len()
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("variables", self.weights.len())])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let sum: f64 = config
            .iter()
            .zip(&self.weights)
            .map(|(&c, &w)| if c == 1 { w } else { 0.0 })
            .sum();
        SolutionSize::valid(sum)
    }
}

#[test]
fn test_brute_force_float() {
    use super::BruteForceFloat;

    let problem = FloatProblem {
        weights: vec![1.0, 2.0, 3.0],
    };
    let solver = BruteForce::new();

    let best = solver.find_best_float(&problem);
    assert_eq!(best.len(), 1);
    assert_eq!(best[0].0, vec![1, 1, 1]);
    assert!((best[0].1.size - 6.0).abs() < 1e-10);
}

#[test]
fn test_brute_force_float_tolerance() {
    use super::BruteForceFloat;

    // Problem where multiple solutions have nearly equal values
    #[derive(Clone)]
    struct NearlyEqualProblem;

    impl Problem for NearlyEqualProblem {
        const NAME: &'static str = "NearlyEqualProblem";

        fn variant() -> Vec<(&'static str, &'static str)> {
            vec![("graph", "SimpleGraph"), ("weight", "f64")]
        }

        type Size = f64;

        fn num_variables(&self) -> usize {
            2
        }

        fn num_flavors(&self) -> usize {
            2
        }

        fn problem_size(&self) -> ProblemSize {
            ProblemSize::new(vec![("variables", 2)])
        }

        fn energy_mode(&self) -> EnergyMode {
            EnergyMode::LargerSizeIsBetter
        }

        fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
            let size = match (config.first(), config.get(1)) {
                (Some(1), Some(0)) => 10.0,
                (Some(0), Some(1)) => 10.0 + 1e-12, // Nearly equal
                _ => 0.0,
            };
            SolutionSize::valid(size)
        }
    }

    let problem = NearlyEqualProblem;
    let solver = BruteForce::with_tolerance(1e-10, 1e-10);

    let best = solver.find_best_float(&problem);
    // Both should be considered optimal due to tolerance
    assert_eq!(best.len(), 2);

    // Test variant for NearlyEqualProblem
    let v = NearlyEqualProblem::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], ("graph", "SimpleGraph"));
    assert_eq!(v[1], ("weight", "f64"));
}

#[test]
fn test_brute_force_float_empty() {
    use super::BruteForceFloat;

    let problem = FloatProblem { weights: vec![] };
    let solver = BruteForce::new();

    let best = solver.find_best_float(&problem);
    assert!(best.is_empty());
}
