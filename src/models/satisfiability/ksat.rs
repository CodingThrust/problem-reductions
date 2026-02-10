//! K-Satisfiability (K-SAT) problem implementation.
//!
//! K-SAT is a special case of SAT where each clause has exactly K literals.
//! Common variants include 3-SAT (K=3) and 2-SAT (K=2).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
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
/// let solutions = solver.find_best(&problem);
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
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "KSatisfiability";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", short_type_name::<W>())]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.num_vars
    }

    fn num_flavors(&self) -> usize {
        2 // Boolean
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("k", K),
            ("num_vars", self.num_vars),
            ("num_clauses", self.clauses.len()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let assignment = Self::config_to_assignment(config);
        let is_valid = self.is_satisfying(&assignment);

        let mut total = W::zero();
        for (clause, weight) in self.clauses.iter().zip(&self.weights) {
            if clause.is_satisfied(&assignment) {
                total += weight.clone();
            }
        }

        SolutionSize::new(total, is_valid)
    }
}

impl<const K: usize, W> ConstraintSatisfactionProblem for KSatisfiability<K, W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        self.clauses
            .iter()
            .map(|clause| {
                let vars = clause.variables();
                let num_configs = 2usize.pow(vars.len() as u32);

                let spec: Vec<bool> = (0..num_configs)
                    .map(|config_idx| {
                        let local_assignment: Vec<bool> = (0..vars.len())
                            .map(|i| (config_idx >> (vars.len() - 1 - i)) & 1 == 1)
                            .collect();

                        let mut full_assignment = vec![false; self.num_vars];
                        for (i, &var) in vars.iter().enumerate() {
                            full_assignment[var] = local_assignment[i];
                        }

                        clause.is_satisfied(&full_assignment)
                    })
                    .collect();

                LocalConstraint::new(2, vars, spec)
            })
            .collect()
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        self.clauses
            .iter()
            .zip(&self.weights)
            .map(|(clause, weight)| {
                let vars = clause.variables();
                let num_configs = 2usize.pow(vars.len() as u32);

                let spec: Vec<W> = (0..num_configs)
                    .map(|config_idx| {
                        let local_assignment: Vec<bool> = (0..vars.len())
                            .map(|i| (config_idx >> (vars.len() - 1 - i)) & 1 == 1)
                            .collect();

                        let mut full_assignment = vec![false; self.num_vars];
                        for (i, &var) in vars.iter().enumerate() {
                            full_assignment[var] = local_assignment[i];
                        }

                        if clause.is_satisfied(&full_assignment) {
                            weight.clone()
                        } else {
                            W::zero()
                        }
                    })
                    .collect();

                LocalSolutionSize::new(2, vars, spec)
            })
            .collect()
    }

    fn weights(&self) -> Vec<Self::Size> {
        self.weights.clone()
    }

    fn set_weights(&mut self, weights: Vec<Self::Size>) {
        assert_eq!(weights.len(), self.clauses.len());
        self.weights = weights;
    }

    fn is_weighted(&self) -> bool {
        if self.weights.is_empty() {
            return false;
        }
        let first = &self.weights[0];
        !self.weights.iter().all(|w| w == first)
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/satisfiability/ksat.rs"]
mod tests;
