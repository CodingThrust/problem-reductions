//! QUBO (Quadratic Unconstrained Binary Optimization) problem implementation.
//!
//! QUBO minimizes a quadratic function over binary variables.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QUBO",
        description: "Minimize quadratic unconstrained binary objective",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of binary variables" },
            FieldInfo { name: "matrix", type_name: "Vec<Vec<W>>", description: "Upper-triangular Q matrix" },
        ],
    }
}

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

impl<W> Problem for QUBO<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + 'static,
{
    const NAME: &'static str = "QUBO";
    type Metric = W;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> W {
        self.evaluate(config)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }
}

impl<W> OptimizationProblem for QUBO<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + 'static,
{
    fn direction(&self) -> Direction {
        Direction::Minimize
    }

    fn is_better(&self, a: &Self::Metric, b: &Self::Metric) -> bool {
        a < b // Minimize
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/optimization/qubo.rs"]
mod tests;
