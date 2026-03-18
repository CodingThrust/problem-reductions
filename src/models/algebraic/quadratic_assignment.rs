//! Quadratic Assignment Problem (QAP) implementation.
//!
//! The QAP assigns facilities to locations to minimize the total cost,
//! where cost depends on both inter-facility flows and inter-location distances.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QuadraticAssignment",
        display_name: "Quadratic Assignment",
        aliases: &["QAP"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Minimize total cost of assigning facilities to locations",
        fields: &[
            FieldInfo { name: "cost_matrix", type_name: "Vec<Vec<i64>>", description: "Flow/cost matrix between facilities" },
            FieldInfo { name: "distance_matrix", type_name: "Vec<Vec<i64>>", description: "Distance matrix between locations" },
        ],
    }
}

/// The Quadratic Assignment Problem (QAP).
///
/// Given n facilities and m locations, a cost matrix C (n x n) representing
/// flows between facilities, and a distance matrix D (m x m) representing
/// distances between locations, find an injective assignment of facilities
/// to locations that minimizes:
///
/// f(p) = sum_{i != j} C[i][j] * D[p(i)][p(j)]
///
/// where p is an injective mapping from facilities to locations (a permutation when n == m).
///
/// # Example
///
/// ```
/// use problemreductions::models::algebraic::QuadraticAssignment;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let cost_matrix = vec![
///     vec![0, 1, 2],
///     vec![1, 0, 3],
///     vec![2, 3, 0],
/// ];
/// let distance_matrix = vec![
///     vec![0, 5, 8],
///     vec![5, 0, 3],
///     vec![8, 3, 0],
/// ];
/// let problem = QuadraticAssignment::new(cost_matrix, distance_matrix);
///
/// let solver = BruteForce::new();
/// let best = solver.find_best(&problem);
/// assert!(best.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadraticAssignment {
    /// Cost/flow matrix between facilities (n x n).
    cost_matrix: Vec<Vec<i64>>,
    /// Distance matrix between locations (m x m).
    distance_matrix: Vec<Vec<i64>>,
}

impl QuadraticAssignment {
    /// Create a new Quadratic Assignment Problem.
    ///
    /// # Arguments
    /// * `cost_matrix` - n x n matrix of flows/costs between facilities
    /// * `distance_matrix` - m x m matrix of distances between locations
    ///
    /// # Panics
    /// Panics if either matrix is not square, or if num_facilities > num_locations.
    pub fn new(cost_matrix: Vec<Vec<i64>>, distance_matrix: Vec<Vec<i64>>) -> Self {
        let n = cost_matrix.len();
        for row in &cost_matrix {
            assert_eq!(row.len(), n, "cost_matrix must be square");
        }
        let m = distance_matrix.len();
        for row in &distance_matrix {
            assert_eq!(row.len(), m, "distance_matrix must be square");
        }
        assert!(
            n <= m,
            "num_facilities ({n}) must be <= num_locations ({m})"
        );
        Self {
            cost_matrix,
            distance_matrix,
        }
    }

    /// Get the cost/flow matrix.
    pub fn cost_matrix(&self) -> &[Vec<i64>] {
        &self.cost_matrix
    }

    /// Get the distance matrix.
    pub fn distance_matrix(&self) -> &[Vec<i64>] {
        &self.distance_matrix
    }

    /// Get the number of facilities.
    pub fn num_facilities(&self) -> usize {
        self.cost_matrix.len()
    }

    /// Get the number of locations.
    pub fn num_locations(&self) -> usize {
        self.distance_matrix.len()
    }
}

impl Problem for QuadraticAssignment {
    const NAME: &'static str = "QuadraticAssignment";
    type Metric = SolutionSize<i64>;

    fn dims(&self) -> Vec<usize> {
        vec![self.num_locations(); self.num_facilities()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i64> {
        let n = self.num_facilities();
        let m = self.num_locations();

        // Check config length matches number of facilities
        if config.len() != n {
            return SolutionSize::Invalid;
        }

        // Check that all assignments are valid locations
        for &loc in config {
            if loc >= m {
                return SolutionSize::Invalid;
            }
        }

        // Check injectivity: no two facilities assigned to the same location
        let mut used = vec![false; m];
        for &loc in config {
            if used[loc] {
                return SolutionSize::Invalid;
            }
            used[loc] = true;
        }

        // Compute objective: sum_{i != j} cost_matrix[i][j] * distance_matrix[config[i]][config[j]]
        let mut total: i64 = 0;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    total += self.cost_matrix[i][j] * self.distance_matrix[config[i]][config[j]];
                }
            }
        }

        SolutionSize::Valid(total)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl OptimizationProblem for QuadraticAssignment {
    type Value = i64;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt QuadraticAssignment => "factorial(num_facilities)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/quadratic_assignment.rs"]
mod tests;
