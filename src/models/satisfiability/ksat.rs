//! K-Satisfiability (K-SAT) problem implementation.
//!
//! K-SAT is a special case of SAT where each clause has exactly K literals.
//! Common variants include 3-SAT (K=3) and 2-SAT (K=2). This is the decision
//! version - for the optimization variant (MAX-K-SAT), see the separate
//! MaxKSatisfiability type (if available).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::KValue;
use serde::{Deserialize, Serialize};

use super::CNFClause;

inventory::submit! {
    ProblemSchemaEntry {
        name: "KSatisfiability",
        module_path: module_path!(),
        description: "SAT with exactly k literals per clause",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses each with exactly K literals" },
        ],
    }
}

/// K-Satisfiability problem where each clause has exactly K literals.
///
/// This is a restricted form of SAT where every clause must contain
/// exactly K literals. The most famous variant is 3-SAT (K=3), which
/// is NP-complete, while 2-SAT (K=2) is solvable in polynomial time.
/// This is the decision version of the problem.
///
/// # Type Parameters
/// * `K` - A type implementing `KValue` that specifies the number of literals per clause
///
/// # Example
///
/// ```
/// use problemreductions::models::satisfiability::{KSatisfiability, CNFClause};
/// use problemreductions::variant::K3;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 3-SAT formula: (x1 OR x2 OR x3) AND (NOT x1 OR x2 OR NOT x3)
/// let problem = KSatisfiability::<K3>::new(
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
#[serde(bound(deserialize = ""))]
pub struct KSatisfiability<K: KValue> {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF, each with exactly K literals.
    clauses: Vec<CNFClause>,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<K>,
}

impl<K: KValue> KSatisfiability<K> {
    /// Create a new K-SAT problem.
    ///
    /// # Panics
    /// Panics if any clause does not have exactly K literals,
    /// or if K is KN (generic K cannot be instantiated).
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        let k = K::K.expect("KN cannot be instantiated");
        for (i, clause) in clauses.iter().enumerate() {
            assert!(
                clause.len() == k,
                "Clause {} has {} literals, expected {}",
                i,
                clause.len(),
                k
            );
        }
        Self {
            num_vars,
            clauses,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new K-SAT problem allowing clauses with fewer than K literals.
    ///
    /// This is useful when the reduction algorithm produces clauses with
    /// fewer literals (e.g., when allow_less is true in the Julia implementation).
    ///
    /// # Panics
    /// Panics if any clause has more than K literals,
    /// or if K is KN (generic K cannot be instantiated).
    pub fn new_allow_less(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        let k = K::K.expect("KN cannot be instantiated");
        for (i, clause) in clauses.iter().enumerate() {
            assert!(
                clause.len() <= k,
                "Clause {} has {} literals, expected at most {}",
                i,
                clause.len(),
                k
            );
        }
        Self {
            num_vars,
            clauses,
            _phantom: std::marker::PhantomData,
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

impl<K: KValue> Problem for KSatisfiability<K> {
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
        crate::variant_params![K]
    }
}

impl<K: KValue> SatisfactionProblem for KSatisfiability<K> {}

#[cfg(test)]
#[path = "../../unit_tests/models/satisfiability/ksat.rs"]
mod tests;
