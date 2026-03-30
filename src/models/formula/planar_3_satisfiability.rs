//! Planar 3-Satisfiability (Planar 3-SAT) problem implementation.
//!
//! Planar 3-SAT is a restricted variant of 3-SAT where the variable-clause
//! incidence graph is planar. Each clause has exactly 3 literals. This
//! restriction preserves NP-completeness while enabling reductions to
//! geometric and planar problems.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

use super::CNFClause;

inventory::submit! {
    ProblemSchemaEntry {
        name: "Planar3Satisfiability",
        display_name: "Planar 3-Satisfiability",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "3-SAT with planar variable-clause incidence graph",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses each with exactly 3 literals" },
        ],
    }
}

/// Planar 3-Satisfiability problem.
///
/// Given a 3-CNF formula where each clause has exactly 3 literals and the
/// variable-clause incidence graph is planar, find a satisfying assignment.
///
/// The incidence graph H(F) is a bipartite graph with variable nodes and
/// clause nodes, where an edge connects variable v to clause C if v appears
/// (positively or negatively) in C. The formula is a valid Planar 3-SAT
/// instance if H(F) is planar.
///
/// **Note:** Planarity of the incidence graph is NOT validated at construction
/// time. Only the clause width (exactly 3 literals) and variable index range
/// are validated. This is analogous to how `PlanarGraph` does not explicitly
/// validate planarity in this codebase.
///
/// # Example
///
/// ```
/// use problemreductions::models::formula::{Planar3Satisfiability, CNFClause};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Formula: (x1 OR x2 OR x3) AND (NOT x1 OR x2 OR x4)
/// //       AND (x1 OR NOT x3 OR x4) AND (NOT x2 OR x3 OR NOT x4)
/// let problem = Planar3Satisfiability::new(
///     4,
///     vec![
///         CNFClause::new(vec![1, 2, 3]),
///         CNFClause::new(vec![-1, 2, 4]),
///         CNFClause::new(vec![1, -3, 4]),
///         CNFClause::new(vec![-2, 3, -4]),
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planar3Satisfiability {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF, each with exactly 3 literals.
    clauses: Vec<CNFClause>,
}

impl Planar3Satisfiability {
    /// Create a new Planar 3-SAT problem.
    ///
    /// # Panics
    /// Panics if any clause does not have exactly 3 literals, or if any
    /// literal references a variable outside the range [1, num_vars].
    ///
    /// **Note:** Planarity of the incidence graph is not checked.
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

    /// Check if an assignment satisfies all clauses.
    pub fn is_satisfying(&self, assignment: &[bool]) -> bool {
        self.clauses.iter().all(|c| c.is_satisfied(assignment))
    }

    /// Convert a usize config to boolean assignment.
    fn config_to_assignment(config: &[usize]) -> Vec<bool> {
        config.iter().map(|&v| v == 1).collect()
    }
}

impl Problem for Planar3Satisfiability {
    const NAME: &'static str = "Planar3Satisfiability";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            let assignment = Self::config_to_assignment(config);
            self.is_satisfying(&assignment)
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default Planar3Satisfiability => "1.307^num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "planar_3_satisfiability",
        instance: Box::new(Planar3Satisfiability::new(
            4,
            vec![
                CNFClause::new(vec![1, 2, 3]),
                CNFClause::new(vec![-1, 2, 4]),
                CNFClause::new(vec![1, -3, 4]),
                CNFClause::new(vec![-2, 3, -4]),
            ],
        )),
        optimal_config: vec![1, 1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/planar_3_satisfiability.rs"]
mod tests;
