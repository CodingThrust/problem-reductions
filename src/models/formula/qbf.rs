//! Quantified Boolean Formulas (QBF) problem implementation.
//!
//! QBF is the problem of determining whether a fully quantified Boolean formula
//! with alternating universal and existential quantifiers is true. It is the
//! canonical PSPACE-complete problem (Stockmeyer & Meyer, 1973).
//!
//! Given F = (Q_1 u_1)(Q_2 u_2)...(Q_n u_n) E, where each Q_i is either
//! ∀ (ForAll) or ∃ (Exists) and E is a Boolean expression in CNF,
//! determine whether F is true.

use crate::models::formula::CNFClause;
use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "QuantifiedBooleanFormulas",
        module_path: module_path!(),
        description: "Determine if a quantified Boolean formula is true",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "quantifiers", type_name: "Vec<Quantifier>", description: "Quantifier for each variable (Exists or ForAll)" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "CNF clauses of the Boolean expression E" },
        ],
    }
}

/// Quantifier type for QBF variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quantifier {
    /// Existential quantifier (∃)
    Exists,
    /// Universal quantifier (∀)
    ForAll,
}

/// Quantified Boolean Formulas (QBF) problem.
///
/// Given a fully quantified Boolean formula F = (Q_1 u_1)...(Q_n u_n) E,
/// where each Q_i is ∀ or ∃ and E is in CNF, determine whether F is true.
///
/// # Example
///
/// ```
/// use problemreductions::models::formula::{QuantifiedBooleanFormulas, Quantifier, CNFClause};
/// use problemreductions::Problem;
///
/// // F = ∃u_1 ∀u_2 (u_1 ∨ u_2) ∧ (u_1 ∨ ¬u_2)
/// let problem = QuantifiedBooleanFormulas::new(
///     2,
///     vec![Quantifier::Exists, Quantifier::ForAll],
///     vec![
///         CNFClause::new(vec![1, 2]),   // u_1 OR u_2
///         CNFClause::new(vec![1, -2]),  // u_1 OR NOT u_2
///     ],
/// );
///
/// // With u_1=true, both clauses are satisfied regardless of u_2
/// assert!(problem.is_true());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantifiedBooleanFormulas {
    /// Number of variables.
    num_vars: usize,
    /// Quantifier for each variable (indexed 0..num_vars).
    quantifiers: Vec<Quantifier>,
    /// Clauses in CNF representing the Boolean expression E.
    clauses: Vec<CNFClause>,
}

impl QuantifiedBooleanFormulas {
    /// Create a new QBF problem.
    ///
    /// # Panics
    ///
    /// Panics if `quantifiers.len() != num_vars`.
    pub fn new(num_vars: usize, quantifiers: Vec<Quantifier>, clauses: Vec<CNFClause>) -> Self {
        assert_eq!(
            quantifiers.len(),
            num_vars,
            "quantifiers length ({}) must equal num_vars ({})",
            quantifiers.len(),
            num_vars
        );
        Self {
            num_vars,
            quantifiers,
            clauses,
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

    /// Get the quantifiers.
    pub fn quantifiers(&self) -> &[Quantifier] {
        &self.quantifiers
    }

    /// Get the clauses.
    pub fn clauses(&self) -> &[CNFClause] {
        &self.clauses
    }

    /// Evaluate whether the QBF formula is true using game-tree search.
    ///
    /// This implements a recursive minimax-style evaluation:
    /// - For ∃ quantifiers: true if ANY assignment to the variable leads to true
    /// - For ∀ quantifiers: true if ALL assignments to the variable lead to true
    ///
    /// Runtime is O(2^n) in the worst case.
    pub fn is_true(&self) -> bool {
        let mut assignment = vec![false; self.num_vars];
        self.evaluate_recursive(&mut assignment, 0)
    }

    /// Recursive QBF evaluation.
    fn evaluate_recursive(&self, assignment: &mut Vec<bool>, var_idx: usize) -> bool {
        if var_idx == self.num_vars {
            // All variables assigned — evaluate the CNF matrix
            return self.clauses.iter().all(|c| c.is_satisfied(assignment));
        }

        match self.quantifiers[var_idx] {
            Quantifier::Exists => {
                // Try both values; true if either works
                assignment[var_idx] = false;
                if self.evaluate_recursive(assignment, var_idx + 1) {
                    return true;
                }
                assignment[var_idx] = true;
                self.evaluate_recursive(assignment, var_idx + 1)
            }
            Quantifier::ForAll => {
                // Try both values; true only if both work
                assignment[var_idx] = false;
                if !self.evaluate_recursive(assignment, var_idx + 1) {
                    return false;
                }
                assignment[var_idx] = true;
                self.evaluate_recursive(assignment, var_idx + 1)
            }
        }
    }

    /// Convert a usize config to boolean assignment.
    fn config_to_assignment(config: &[usize]) -> Vec<bool> {
        config.iter().map(|&v| v == 1).collect()
    }
}

impl Problem for QuantifiedBooleanFormulas {
    const NAME: &'static str = "QuantifiedBooleanFormulas";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let assignment = Self::config_to_assignment(config);
        self.clauses.iter().all(|c| c.is_satisfied(&assignment))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for QuantifiedBooleanFormulas {}

crate::declare_variants! {
    QuantifiedBooleanFormulas => "2^num_vars",
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/qbf.rs"]
mod tests;
