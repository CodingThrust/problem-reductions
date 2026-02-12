//! K-Satisfiability (K-SAT) problem implementation.
//!
//! K-SAT is a special case of SAT where each clause has exactly K literals.
//! Common variants include 3-SAT (K=3) and 2-SAT (K=2).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

use super::CNFClause;

inventory::submit! {
    ProblemSchemaEntry {
        name: "KSatisfiability",
        category: "satisfiability",
        description: "SAT with exactly k literals per clause",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses each with exactly K literals" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Clause weights for MAX-K-SAT" },
        ],
    }
}

/// K-Satisfiability problem where each clause has exactly K literals.
///
/// This is a restricted form of SAT where every clause must contain
/// exactly K literals. The most famous variant is 3-SAT (K=3), which
/// is NP-complete, while 2-SAT (K=2) is solvable in polynomial time.
///
/// # Type Parameters
/// * `K` - The number of literals per clause (compile-time constant)
/// * `W` - The weight type for MAX-K-SAT
///
/// # Example
///
/// ```
/// use problemreductions::models::satisfiability::{KSatisfiability, CNFClause};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 3-SAT formula: (x1 OR x2 OR x3) AND (NOT x1 OR x2 OR NOT x3)
/// let problem = KSatisfiability::<3, i32>::new(
///     3,
///     vec![
///         CNFClause::new(vec![1, 2, 3]),       // x1 OR x2 OR x3
///         CNFClause::new(vec![-1, 2, -3]),     // NOT x1 OR x2 OR NOT x3
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_satisfying(&problem);
/// assert!(!solutions.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSatisfiability<const K: usize, W = i32> {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF, each with exactly K literals.
    clauses: Vec<CNFClause>,
    /// Weights for each clause (for MAX-K-SAT).
    weights: Vec<W>,
}

impl<const K: usize, W: Clone + Default> KSatisfiability<K, W> {
    /// Create a new K-SAT problem with unit weights.
    ///
    /// # Panics
    /// Panics if any clause does not have exactly K literals.
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self
    where
        W: From<i32>,
    {
        for (i, clause) in clauses.iter().enumerate() {
            assert!(
                clause.len() == K,
                "Clause {} has {} literals, expected {}",
                i,
                clause.len(),
                K
            );
        }
        let num_clauses = clauses.len();
        let weights = vec![W::from(1); num_clauses];
        Self {
            num_vars,
            clauses,
            weights,
        }
    }

    /// Create a new K-SAT problem allowing clauses with fewer than K literals.
    ///
    /// This is useful when the reduction algorithm produces clauses with
    /// fewer literals (e.g., when allow_less is true in the Julia implementation).
    ///
    /// # Panics
    /// Panics if any clause has more than K literals.
    pub fn new_allow_less(num_vars: usize, clauses: Vec<CNFClause>) -> Self
    where
        W: From<i32>,
    {
        for (i, clause) in clauses.iter().enumerate() {
            assert!(
                clause.len() <= K,
                "Clause {} has {} literals, expected at most {}",
                i,
                clause.len(),
                K
            );
        }
        let num_clauses = clauses.len();
        let weights = vec![W::from(1); num_clauses];
        Self {
            num_vars,
            clauses,
            weights,
        }
    }

    /// Create a new weighted K-SAT problem (MAX-K-SAT).
    ///
    /// # Panics
    /// Panics if any clause does not have exactly K literals,
    /// or if the number of weights doesn't match the number of clauses.
    pub fn with_weights(num_vars: usize, clauses: Vec<CNFClause>, weights: Vec<W>) -> Self {
        for (i, clause) in clauses.iter().enumerate() {
            assert!(
                clause.len() == K,
                "Clause {} has {} literals, expected {}",
                i,
                clause.len(),
                K
            );
        }
        assert_eq!(clauses.len(), weights.len());
        Self {
            num_vars,
            clauses,
            weights,
        }
    }

    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Get the number of clauses.
    pub fn num_clauses(&self) -> usize {
        self.clauses.len()
    }

    /// Get the clauses.
    pub fn clauses(&self) -> &[CNFClause] {
        &self.clauses
    }

    /// Get a specific clause.
    pub fn get_clause(&self, index: usize) -> Option<&CNFClause> {
        self.clauses.get(index)
    }

    /// Count satisfied clauses for an assignment.
    pub fn count_satisfied(&self, assignment: &[bool]) -> usize {
        self.clauses
            .iter()
            .filter(|c| c.is_satisfied(assignment))
            .count()
    }

    /// Check if an assignment satisfies all clauses.
    pub fn is_satisfying(&self, assignment: &[bool]) -> bool {
        self.clauses.iter().all(|c| c.is_satisfied(assignment))
    }

    /// Convert a usize config to boolean assignment.
    fn config_to_assignment(config: &[usize]) -> Vec<bool> {
        config.iter().map(|&v| v == 1).collect()
    }
}

impl<const K: usize, W> Problem for KSatisfiability<K, W>
where
    W: Clone + Default + 'static,
{
    const NAME: &'static str = "KSatisfiability";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let assignment = Self::config_to_assignment(config);
        self.is_satisfying(&assignment)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("k", crate::variant::const_usize_str::<K>()),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/satisfiability/ksat.rs"]
mod tests;
