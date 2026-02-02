//! QUBO (Quadratic Unconstrained Binary Optimization) problem implementation.
//!
//! QUBO minimizes a quadratic function over binary variables.

use crate::traits::Problem;
use crate::variant::short_type_name;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

/// The QUBO (Quadratic Unconstrained Binary Optimization) problem.
///
/// Given n binary variables x_i ∈ {0, 1} and a matrix Q,
/// minimize the quadratic form:
///
/// f(x) = Σ_i Σ_j Q_ij * x_i * x_j = x^T Q x
///
/// The matrix Q is typically upper triangular, with diagonal elements
/// representing linear terms and off-diagonal elements representing
/// quadratic interactions.
///
/// # Example
///
/// ```
/// use problemreductions::models::optimization::QUBO;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Q matrix: minimize x0 - 2*x1 + x0*x1
/// // Q = [[1, 1], [0, -2]]
/// let problem = QUBO::from_matrix(vec![
///     vec![1.0, 1.0],
///     vec![0.0, -2.0],
/// ]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Optimal is x = [0, 1] with value -2
/// assert!(solutions.contains(&vec![0, 1]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QUBO<W = f64> {
    /// Number of variables.
    num_vars: usize,
    /// Q matrix stored as upper triangular (row-major).
    /// `Q[i][j]` for i <= j represents the coefficient of x_i * x_j
    matrix: Vec<Vec<W>>,
}

impl<W: Clone + Default> QUBO<W> {
    /// Create a QUBO problem from a full matrix.
    ///
    /// The matrix should be square. Only the upper triangular part
    /// (including diagonal) is used.
    pub fn from_matrix(matrix: Vec<Vec<W>>) -> Self {
        let num_vars = matrix.len();
        Self { num_vars, matrix }
    }

    /// Create a QUBO from linear and quadratic terms.
    ///
    /// # Arguments
    /// * `linear` - Linear coefficients (diagonal of Q)
    /// * `quadratic` - Quadratic coefficients as ((i, j), value) for i < j
    pub fn new(linear: Vec<W>, quadratic: Vec<((usize, usize), W)>) -> Self
    where
        W: num_traits::Zero,
    {
        let num_vars = linear.len();
        let mut matrix = vec![vec![W::zero(); num_vars]; num_vars];

        // Set diagonal (linear terms)
        for (i, val) in linear.into_iter().enumerate() {
            matrix[i][i] = val;
        }

        // Set off-diagonal (quadratic terms)
        for ((i, j), val) in quadratic {
            if i < j {
                matrix[i][j] = val;
            } else {
                matrix[j][i] = val;
            }
        }

        Self { num_vars, matrix }
    }

    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Get the Q matrix.
    pub fn matrix(&self) -> &[Vec<W>] {
        &self.matrix
    }

    /// Get a specific matrix element `Q[i][j]`.
    pub fn get(&self, i: usize, j: usize) -> Option<&W> {
        self.matrix.get(i).and_then(|row| row.get(j))
    }
}

impl<W> Problem for QUBO<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + 'static,
{
    const NAME: &'static str = "QUBO";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.num_vars
    }

    fn num_flavors(&self) -> usize {
        2 // Binary
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("num_vars", self.num_vars)])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let value = self.evaluate(config);
        SolutionSize::valid(value)
    }
}

impl<W> QUBO<W>
where
    W: Clone + num_traits::Zero + std::ops::AddAssign + std::ops::Mul<Output = W>,
{
    /// Evaluate the QUBO objective for a configuration.
    pub fn evaluate(&self, config: &[usize]) -> W {
        let mut value = W::zero();

        for i in 0..self.num_vars {
            let x_i = config.get(i).copied().unwrap_or(0);
            if x_i == 0 {
                continue;
            }

            for j in i..self.num_vars {
                let x_j = config.get(j).copied().unwrap_or(0);
                if x_j == 0 {
                    continue;
                }

                if let Some(q_ij) = self.matrix.get(i).and_then(|row| row.get(j)) {
                    value += q_ij.clone();
                }
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_qubo_from_matrix() {
        let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);
        assert_eq!(problem.num_vars(), 2);
        assert_eq!(problem.get(0, 0), Some(&1.0));
        assert_eq!(problem.get(0, 1), Some(&2.0));
        assert_eq!(problem.get(1, 1), Some(&3.0));
    }

    #[test]
    fn test_qubo_new() {
        let problem = QUBO::new(vec![1.0, 2.0], vec![((0, 1), 3.0)]);
        assert_eq!(problem.get(0, 0), Some(&1.0));
        assert_eq!(problem.get(1, 1), Some(&2.0));
        assert_eq!(problem.get(0, 1), Some(&3.0));
    }

    #[test]
    fn test_evaluate() {
        // Q = [[1, 2], [0, 3]]
        // f(x) = x0 + 3*x1 + 2*x0*x1
        let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);

        assert_eq!(problem.evaluate(&[0, 0]), 0.0);
        assert_eq!(problem.evaluate(&[1, 0]), 1.0);
        assert_eq!(problem.evaluate(&[0, 1]), 3.0);
        assert_eq!(problem.evaluate(&[1, 1]), 6.0); // 1 + 3 + 2 = 6
    }

    #[test]
    fn test_solution_size() {
        let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);

        let sol = problem.solution_size(&[0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0.0);

        let sol = problem.solution_size(&[1, 1]);
        assert_eq!(sol.size, 6.0);
    }

    #[test]
    fn test_brute_force_minimize() {
        // Q = [[1, 0], [0, -2]]
        // f(x) = x0 - 2*x1
        // Minimum at x = [0, 1] with value -2
        let problem = QUBO::from_matrix(vec![vec![1.0, 0.0], vec![0.0, -2.0]]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1]);
        assert_eq!(problem.solution_size(&solutions[0]).size, -2.0);
    }

    #[test]
    fn test_brute_force_with_interaction() {
        // Q = [[-1, 2], [0, -1]]
        // f(x) = -x0 - x1 + 2*x0*x1
        // x=[0,0] -> 0, x=[1,0] -> -1, x=[0,1] -> -1, x=[1,1] -> 0
        let problem = QUBO::from_matrix(vec![vec![-1.0, 2.0], vec![0.0, -1.0]]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Minimum is -1 at [1,0] or [0,1]
        assert_eq!(solutions.len(), 2);
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, -1.0);
        }
    }

    #[test]
    fn test_energy_mode() {
        let problem = QUBO::<f64>::from_matrix(vec![vec![1.0]]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_num_variables_flavors() {
        let problem = QUBO::<f64>::from_matrix(vec![vec![0.0; 5]; 5]);
        assert_eq!(problem.num_variables(), 5);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_problem_size() {
        let problem = QUBO::<f64>::from_matrix(vec![vec![0.0; 3]; 3]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vars"), Some(3));
    }

    #[test]
    fn test_matrix_access() {
        let problem = QUBO::from_matrix(vec![
            vec![1.0, 2.0, 3.0],
            vec![0.0, 4.0, 5.0],
            vec![0.0, 0.0, 6.0],
        ]);
        let matrix = problem.matrix();
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0], vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_empty_qubo() {
        let problem = QUBO::<f64>::from_matrix(vec![]);
        assert_eq!(problem.num_vars(), 0);
        assert_eq!(problem.evaluate(&[]), 0.0);
    }

    #[test]
    fn test_single_variable() {
        let problem = QUBO::from_matrix(vec![vec![-5.0]]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1]); // x=1 gives -5, x=0 gives 0
    }

    #[test]
    fn test_qubo_new_reverse_indices() {
        // Test the case where (j, i) is provided with i < j
        let problem = QUBO::new(vec![1.0, 2.0], vec![((1, 0), 3.0)]); // j > i
        assert_eq!(problem.get(0, 1), Some(&3.0)); // Should be stored at (0, 1)
    }

    #[test]
    fn test_get_out_of_bounds() {
        let problem = QUBO::from_matrix(vec![vec![1.0, 2.0], vec![0.0, 3.0]]);
        assert_eq!(problem.get(5, 5), None);
        assert_eq!(problem.get(0, 5), None);
    }
}
