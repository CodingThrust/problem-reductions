//! Maximum 2-Satisfiability (MAX-2-SAT) problem implementation.
//!
//! MAX-2-SAT is an optimization variant of 2-SAT where each clause has exactly
//! 2 literals, and the goal is to maximize the number of satisfied clauses.
//! While 2-SAT (decision) is solvable in polynomial time, MAX-2-SAT is NP-hard.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

use super::CNFClause;

inventory::submit! {
    ProblemSchemaEntry {
        name: "Maximum2Satisfiability",
        display_name: "Maximum 2-Satisfiability",
        aliases: &["MAX2SAT"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Maximize the number of satisfied 2-literal clauses",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Collection of 2-literal clauses" },
        ],
    }
}

/// Maximum 2-Satisfiability problem where each clause has exactly 2 literals.
///
/// Given a set of Boolean variables and a collection of clauses, each containing
/// exactly 2 literals, find a truth assignment that maximizes the number of
/// simultaneously satisfied clauses.
///
/// # Example
///
/// ```
/// use problemreductions::models::formula::{Maximum2Satisfiability, CNFClause};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = Maximum2Satisfiability::new(
///     3,
///     vec![
///         CNFClause::new(vec![1, 2]),    // x1 OR x2
///         CNFClause::new(vec![-1, -2]),  // NOT x1 OR NOT x2
///         CNFClause::new(vec![1, 3]),    // x1 OR x3
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let value = solver.solve(&problem);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maximum2Satisfiability {
    /// Number of Boolean variables.
    num_vars: usize,
    /// Clauses in CNF, each with exactly 2 literals.
    clauses: Vec<CNFClause>,
}

impl Maximum2Satisfiability {
    /// Create a new MAX-2-SAT problem.
    ///
    /// # Panics
    /// Panics if any clause does not have exactly 2 literals.
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        for (i, clause) in clauses.iter().enumerate() {
            assert!(
                clause.len() == 2,
                "Clause {} has {} literals, expected 2",
                i,
                clause.len()
            );
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

    /// Count satisfied clauses for an assignment.
    pub fn count_satisfied(&self, assignment: &[bool]) -> usize {
        self.clauses
            .iter()
            .filter(|c| c.is_satisfied(assignment))
            .count()
    }
}

impl Problem for Maximum2Satisfiability {
    const NAME: &'static str = "Maximum2Satisfiability";
    type Value = Max<usize>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> Max<usize> {
        let assignment = super::config_to_assignment(config);
        Max(Some(self.count_satisfied(&assignment)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default Maximum2Satisfiability => "2^(0.7905 * num_variables)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_2_satisfiability",
        instance: Box::new(Maximum2Satisfiability::new(
            4,
            vec![
                CNFClause::new(vec![1, 2]),
                CNFClause::new(vec![1, -2]),
                CNFClause::new(vec![-1, 3]),
                CNFClause::new(vec![-1, -3]),
                CNFClause::new(vec![2, 4]),
                CNFClause::new(vec![-3, -4]),
                CNFClause::new(vec![3, 4]),
            ],
        )),
        optimal_config: vec![1, 1, 0, 1],
        optimal_value: serde_json::json!(6),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/maximum_2_satisfiability.rs"]
mod tests;
