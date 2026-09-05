//! Planar 3-Satisfiability (Planar 3-SAT) problem implementation.
//!
//! Planar 3-SAT is a restricted variant of 3-SAT where the variable-clause
//! incidence graph is planar. Each clause has exactly 3 literals. This
//! restriction preserves NP-completeness while enabling reductions to
//! geometric and planar problems.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

use super::{sat::validate_cnf_literals, CNFClause};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Planar3Satisfiability",
        display_name: "Planar 3-Satisfiability",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Formula,
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
/// use problemreductions::{Problem, BruteForce};
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
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "Planar3SatisfiabilityDef")]
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
        Self::try_new(num_vars, clauses).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Create a new Planar 3-SAT problem after validating its clauses.
    pub fn try_new(
        num_vars: usize,
        clauses: Vec<CNFClause>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        validate_cnf_literals(num_vars, &clauses)?;
        for (i, clause) in clauses.iter().enumerate() {
            if clause.len() != 3 {
                return Err(format!("Clause {i} has {} literals, expected 3", clause.len()).into());
            }
        }
        Ok(Self { num_vars, clauses })
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
}

impl Problem for Planar3Satisfiability {
    const NAME: &'static str = "Planar3Satisfiability";
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_vars", num_vars), ("num_clauses", num_clauses),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.num_vars {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "assignment length does not match the formula variables".into(),
            ));
        }
        Ok(crate::types::Or(self.is_satisfying(config)))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for Planar3Satisfiability {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }
}

crate::declare_variants! {
    default Planar3Satisfiability => "1.307^num_vars",
}

crate::register_brute_force! {
    Planar3Satisfiability decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[derive(Deserialize)]
struct Planar3SatisfiabilityDef {
    num_vars: usize,
    clauses: Vec<CNFClause>,
}

impl TryFrom<Planar3SatisfiabilityDef> for Planar3Satisfiability {
    type Error = crate::registry::ConstructionError;

    fn try_from(value: Planar3SatisfiabilityDef) -> Result<Self, Self::Error> {
        Self::try_new(value.num_vars, value.clauses)
    }
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
        optimal_config: serde_json::json!(vec![true, true, true, false]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/planar_3_satisfiability.rs"]
mod tests;
