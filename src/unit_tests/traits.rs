use super::*;

// A simple test problem: select binary variables to maximize sum of weights
#[derive(Clone)]
struct SimpleWeightedProblem {
    weights: Vec<i32>,
}

impl Problem for SimpleWeightedProblem {
    const NAME: &'static str = "SimpleWeightedProblem";

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

// A simple CSP for testing
#[derive(Clone)]
struct SimpleCsp {
    num_vars: usize,
}

impl Problem for SimpleCsp {
    const NAME: &'static str = "SimpleCsp";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.num_vars
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("variables", self.num_vars)])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        csp_solution_size(self, config)
    }
}

impl ConstraintSatisfactionProblem for SimpleCsp {
    fn constraints(&self) -> Vec<LocalConstraint> {
        // Constraint: at most one variable can be 1
        if self.num_vars >= 2 {
            vec![LocalConstraint::new(
                2,
                vec![0, 1],
                vec![true, true, true, false], // (0,0), (0,1), (1,0) OK; (1,1) invalid
            )]
        } else {
            vec![]
        }
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        // Each variable contributes 1 if selected
        (0..self.num_vars)
            .map(|i| LocalSolutionSize::new(2, vec![i], vec![0, 1]))
            .collect()
    }

    fn weights(&self) -> Vec<Self::Size> {
        vec![1; self.num_vars]
    }

    fn set_weights(&mut self, _weights: Vec<Self::Size>) {}

    fn is_weighted(&self) -> bool {
        false
    }
}

#[test]
fn test_variant_for_test_problems() {
    // Test that variant() works for test problems
    let v = SimpleWeightedProblem::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], ("graph", "SimpleGraph"));
    assert_eq!(v[1], ("weight", "i32"));

    let v = SimpleCsp::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], ("graph", "SimpleGraph"));
    assert_eq!(v[1], ("weight", "i32"));

    let v = MultiFlavorProblem::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], ("graph", "SimpleGraph"));
    assert_eq!(v[1], ("weight", "i32"));
}

#[test]
fn test_simple_problem() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2, 3],
    };

    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.num_flavors(), 2);
    assert_eq!(problem.variables(), 0..3);
    assert_eq!(problem.flavors(), vec![0, 1]);

    let sol = problem.solution_size(&[0, 0, 0]);
    assert_eq!(sol.size, 0);
    assert!(sol.is_valid);

    let sol = problem.solution_size(&[1, 1, 1]);
    assert_eq!(sol.size, 6);
    assert!(sol.is_valid);

    let sol = problem.solution_size(&[1, 0, 1]);
    assert_eq!(sol.size, 4);
    assert!(sol.is_valid);
}

#[test]
fn test_valid_config() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2, 3],
    };

    assert!(problem.is_valid_config(&[0, 1, 0]));
    assert!(problem.is_valid_config(&[1, 1, 1]));
    assert!(!problem.is_valid_config(&[0, 2, 0])); // invalid flavor
    assert!(!problem.is_valid_config(&[0, 1])); // wrong length
    assert!(!problem.is_valid_config(&[0, 1, 0, 1])); // wrong length
}

#[test]
fn test_batch_evaluation() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2, 3],
    };

    let configs = vec![vec![0, 0, 0], vec![1, 1, 1], vec![1, 0, 1]];

    let results = problem.solution_size_multiple(&configs);
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].size, 0);
    assert_eq!(results[1].size, 6);
    assert_eq!(results[2].size, 4);
}

#[test]
fn test_csp_solution_size() {
    let problem = SimpleCsp { num_vars: 3 };

    // Test valid configurations
    let sol = problem.solution_size(&[0, 0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);

    let sol = problem.solution_size(&[1, 0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    let sol = problem.solution_size(&[0, 1, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    // Test invalid configuration (both 0 and 1 are 1)
    let sol = problem.solution_size(&[1, 1, 0]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 2);
}

#[test]
fn test_csp_is_satisfied() {
    let problem = SimpleCsp { num_vars: 3 };

    assert!(problem.is_satisfied(&[0, 0, 0]));
    assert!(problem.is_satisfied(&[1, 0, 0]));
    assert!(problem.is_satisfied(&[0, 1, 0]));
    assert!(!problem.is_satisfied(&[1, 1, 0]));
}

#[test]
fn test_csp_compute_objective() {
    let problem = SimpleCsp { num_vars: 3 };

    assert_eq!(problem.compute_objective(&[0, 0, 0]), 0);
    assert_eq!(problem.compute_objective(&[1, 0, 0]), 1);
    assert_eq!(problem.compute_objective(&[1, 1, 0]), 2);
    assert_eq!(problem.compute_objective(&[1, 1, 1]), 3);
}

#[test]
fn test_csp_single_variable() {
    // Test CSP with num_vars = 1 (no constraints, empty constraint list)
    let problem = SimpleCsp { num_vars: 1 };

    assert!(problem.constraints().is_empty());
    assert!(problem.is_satisfied(&[0])); // Always satisfied with no constraints
    assert!(problem.is_satisfied(&[1]));

    let sol = problem.solution_size(&[0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);

    let sol = problem.solution_size(&[1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);
}

#[test]
fn test_csp_weights_and_weighted() {
    let problem = SimpleCsp { num_vars: 3 };
    assert_eq!(problem.weights(), vec![1, 1, 1]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_csp_set_weights() {
    let mut problem = SimpleCsp { num_vars: 3 };
    problem.set_weights(vec![10, 20, 30]);
    // For SimpleCsp, set_weights is a no-op, so this just tests the call works
    assert!(!problem.is_weighted());
}

#[test]
fn test_problem_size_metadata() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2, 3, 4, 5],
    };

    let size = problem.problem_size();
    assert_eq!(size.get("variables"), Some(5));
}

#[test]
fn test_energy_mode() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2, 3],
    };
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_batch_evaluation_empty() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2, 3],
    };

    let configs: Vec<Vec<usize>> = vec![];
    let results = problem.solution_size_multiple(&configs);
    assert!(results.is_empty());
}

#[test]
fn test_is_valid_config_empty_problem() {
    let problem = SimpleWeightedProblem { weights: vec![] };

    assert_eq!(problem.num_variables(), 0);
    assert!(problem.is_valid_config(&[])); // Empty config for empty problem
    assert!(!problem.is_valid_config(&[0])); // Non-empty config is invalid
}

#[test]
fn test_variables_range() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2, 3, 4, 5],
    };

    let vars: Vec<usize> = problem.variables().collect();
    assert_eq!(vars, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_flavors_list() {
    let problem = SimpleWeightedProblem {
        weights: vec![1, 2],
    };

    assert_eq!(problem.flavors(), vec![0, 1]);
}

#[test]
fn test_csp_objectives() {
    let problem = SimpleCsp { num_vars: 3 };
    let objectives = problem.objectives();

    assert_eq!(objectives.len(), 3);
    // Test that each objective evaluates correctly
    assert_eq!(objectives[0].evaluate(&[0, 0, 0]), 0);
    assert_eq!(objectives[0].evaluate(&[1, 0, 0]), 1);
    assert_eq!(objectives[1].evaluate(&[0, 1, 0]), 1);
    assert_eq!(objectives[2].evaluate(&[0, 0, 1]), 1);
}

#[test]
fn test_csp_solution_size_helper_function() {
    let problem = SimpleCsp { num_vars: 2 };

    // Test via the helper function directly
    let sol = csp_solution_size(&problem, &[0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);

    let sol = csp_solution_size(&problem, &[1, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 1);

    let sol = csp_solution_size(&problem, &[1, 1]);
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 2);
}

// Test problem with more than 2 flavors
#[derive(Clone)]
struct MultiFlavorProblem {
    num_vars: usize,
    num_flavors: usize,
}

impl Problem for MultiFlavorProblem {
    const NAME: &'static str = "MultiFlavorProblem";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.num_vars
    }

    fn num_flavors(&self) -> usize {
        self.num_flavors
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("variables", self.num_vars),
            ("flavors", self.num_flavors),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let sum: i32 = config.iter().map(|&c| c as i32).sum();
        SolutionSize::valid(sum)
    }
}

#[test]
fn test_multi_flavor_problem() {
    let problem = MultiFlavorProblem {
        num_vars: 3,
        num_flavors: 4,
    };

    assert_eq!(problem.num_flavors(), 4);
    assert_eq!(problem.flavors(), vec![0, 1, 2, 3]);
    assert!(problem.energy_mode().is_minimization());

    // Valid configs
    assert!(problem.is_valid_config(&[0, 1, 2]));
    assert!(problem.is_valid_config(&[3, 3, 3]));

    // Invalid: flavor out of range
    assert!(!problem.is_valid_config(&[0, 4, 0]));
    assert!(!problem.is_valid_config(&[5, 0, 0]));

    let sol = problem.solution_size(&[0, 1, 2]);
    assert_eq!(sol.size, 3);

    let sol = problem.solution_size(&[3, 3, 3]);
    assert_eq!(sol.size, 9);
}

#[test]
fn test_batch_evaluation_with_multi_flavor() {
    let problem = MultiFlavorProblem {
        num_vars: 2,
        num_flavors: 3,
    };

    let configs = vec![vec![0, 0], vec![1, 1], vec![2, 2], vec![0, 2]];
    let results = problem.solution_size_multiple(&configs);

    assert_eq!(results.len(), 4);
    assert_eq!(results[0].size, 0);
    assert_eq!(results[1].size, 2);
    assert_eq!(results[2].size, 4);
    assert_eq!(results[3].size, 2);
}

// === ProblemV2 / OptimizationProblemV2 tests ===

use crate::types::Direction;

#[derive(Clone)]
struct TestSatProblem {
    num_vars: usize,
    satisfying: Vec<Vec<usize>>,
}

impl crate::traits::ProblemV2 for TestSatProblem {
    const NAME: &'static str = "TestSat";
    type Metric = bool;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }
    fn evaluate(&self, config: &[usize]) -> bool {
        self.satisfying.iter().any(|s| s == config)
    }
}

#[test]
fn test_problem_v2_sat() {
    let p = TestSatProblem {
        num_vars: 2,
        satisfying: vec![vec![1, 0], vec![0, 1]],
    };
    assert_eq!(p.dims(), vec![2, 2]);
    assert!(p.evaluate(&[1, 0]));
    assert!(!p.evaluate(&[0, 0]));
}

#[derive(Clone)]
struct TestOptProblem {
    weights: Vec<i32>,
}

impl crate::traits::ProblemV2 for TestOptProblem {
    const NAME: &'static str = "TestOpt";
    type Metric = i32;
    fn dims(&self) -> Vec<usize> {
        vec![2; self.weights.len()]
    }
    fn evaluate(&self, config: &[usize]) -> i32 {
        config
            .iter()
            .enumerate()
            .map(|(i, &v)| if v == 1 { self.weights[i] } else { 0 })
            .sum()
    }
}

impl crate::traits::OptimizationProblemV2 for TestOptProblem {
    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

#[test]
fn test_optimization_problem_v2() {
    let p = TestOptProblem {
        weights: vec![3, 1, 4],
    };
    assert_eq!(p.evaluate(&[1, 0, 1]), 7);
    assert_eq!(p.direction(), Direction::Maximize);
}
