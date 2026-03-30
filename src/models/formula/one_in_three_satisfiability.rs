//! One-in-Three Satisfiability (1-in-3 SAT) problem implementation.
//!
//! 1-in-3 SAT is a variant of 3-SAT where each clause must have *exactly one*
//! true literal (rather than *at least one*). This stronger constraint makes
//! the problem NP-complete even without negations (monotone 1-in-3 SAT).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

use super::CNFClause;

inventory::submit! {
    ProblemSchemaEntry {
        name: "OneInThreeSatisfiability",
        display_name: "One-in-Three Satisfiability",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "3-SAT variant where each clause has exactly one true literal",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses each with exactly 3 literals" },
        ],
    }
}

/// One-in-Three Satisfiability problem.
///
/// Given a CNF formula where each clause has exactly 3 literals, find a truth
/// assignment such that each clause has *exactly one* true literal.
///
/// This is a well-known NP-complete problem introduced by Schaefer (1978).
/// Unlike standard 3-SAT which requires at least one true literal per clause,
/// 1-in-3 SAT requires exactly one.
///
/// # Example
///
/// ```
/// use problemreductions::models::formula::{OneInThreeSatisfiability, CNFClause};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // (x1 OR x2 OR x3) AND (NOT x1 OR x3 OR x4) AND (x2 OR NOT x3 OR NOT x4)
/// let problem = OneInThreeSatisfiability::new(
///     4,
///     vec![
///         CNFClause::new(vec![1, 2, 3]),
///         CNFClause::new(vec![-1, 3, 4]),
///         CNFClause::new(vec![2, -3, -4]),
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneInThreeSatisfiability {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF, each with exactly 3 literals.
    clauses: Vec<CNFClause>,
}

impl OneInThreeSatisfiability {
    /// Create a new 1-in-3 SAT problem.
    ///
    /// # Panics
    /// Panics if any clause does not have exactly 3 literals, or if any
    /// literal references a variable outside the range [1, num_vars].
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        for (i, clause) in clauses.iter().enumerate() {
            assert!(
                clause.len() == 3,
                "Clause {} has {} literals, expected 3",
                i,
                clause.len()
            );
            for &lit in &clause.literals {
                let var = lit.unsigned_abs() as usize;
                assert!(
                    var >= 1 && var <= num_vars,
                    "Clause {} contains literal {} referencing variable {} outside range [1, {}]",
                    i,
                    lit,
                    var,
                    num_vars
                );
            }
        }
        Self { num_vars, clauses }
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

    /// Check if exactly one literal is true in each clause.
    pub fn is_one_in_three_satisfying(&self, assignment: &[bool]) -> bool {
        self.clauses.iter().all(|clause| {
            let true_count = clause
                .literals
                .iter()
                .filter(|&&lit| {
                    let var = lit.unsigned_abs() as usize - 1; // Convert to 0-indexed
                    let value = assignment.get(var).copied().unwrap_or(false);
                    if lit > 0 {
                        value
                    } else {
                        !value
                    }
                })
                .count();
            true_count == 1
        })
    }

    /// Convert a usize config to boolean assignment.
    fn config_to_assignment(config: &[usize]) -> Vec<bool> {
        config.iter().map(|&v| v == 1).collect()
    }
}

impl Problem for OneInThreeSatisfiability {
    const NAME: &'static str = "OneInThreeSatisfiability";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            let assignment = Self::config_to_assignment(config);
            self.is_one_in_three_satisfying(&assignment)
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default OneInThreeSatisfiability => "1.307^num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "one_in_three_satisfiability",
        instance: Box::new(OneInThreeSatisfiability::new(
            4,
            vec![
                CNFClause::new(vec![1, 2, 3]),
                CNFClause::new(vec![-1, 3, 4]),
                CNFClause::new(vec![2, -3, -4]),
            ],
        )),
        optimal_config: vec![1, 0, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/one_in_three_satisfiability.rs"]
mod tests;
