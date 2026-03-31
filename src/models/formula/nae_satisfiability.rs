//! Not-All-Equal Boolean Satisfiability (NAE-SAT) problem implementation.
//!
//! NAE-SAT asks whether a CNF formula has an assignment such that each clause
//! contains at least one true literal and at least one false literal.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

use super::CNFClause;

inventory::submit! {
    ProblemSchemaEntry {
        name: "NAESatisfiability",
        display_name: "Not-All-Equal Satisfiability",
        aliases: &["NAESAT"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find an assignment where every CNF clause has both a true and a false literal",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses in conjunctive normal form with at least two literals each" },
        ],
    }
}

/// Not-All-Equal Boolean Satisfiability (NAE-SAT) in CNF form.
///
/// Given a Boolean formula in conjunctive normal form (CNF), determine whether
/// there exists an assignment such that every clause contains at least one
/// true literal and at least one false literal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "NAESatisfiabilityDef")]
pub struct NAESatisfiability {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF, each with at least two literals.
    clauses: Vec<CNFClause>,
}

impl NAESatisfiability {
    /// Create a new NAE-SAT problem.
    ///
    /// # Panics
    /// Panics if any clause has fewer than two literals.
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        Self::try_new(num_vars, clauses).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Create a new NAE-SAT problem, returning an error instead of panicking
    /// when a clause has fewer than two literals.
    pub fn try_new(num_vars: usize, clauses: Vec<CNFClause>) -> Result<Self, String> {
        validate_clause_lengths(&clauses)?;
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

    /// Get the total number of literal occurrences across all clauses.
    pub fn num_literals(&self) -> usize {
        self.clauses.iter().map(|c| c.len()).sum()
    }

    /// Get the clauses.
    pub fn clauses(&self) -> &[CNFClause] {
        &self.clauses
    }

    /// Get a specific clause.
    pub fn get_clause(&self, index: usize) -> Option<&CNFClause> {
        self.clauses.get(index)
    }

    /// Count how many clauses satisfy the NAE condition under an assignment.
    pub fn count_nae_satisfied(&self, assignment: &[bool]) -> usize {
        self.clauses
            .iter()
            .filter(|clause| Self::clause_is_nae_satisfied(clause, assignment))
            .count()
    }

    /// Check whether all clauses satisfy the NAE condition under an assignment.
    pub fn is_nae_satisfying(&self, assignment: &[bool]) -> bool {
        self.clauses
            .iter()
            .all(|clause| Self::clause_is_nae_satisfied(clause, assignment))
    }

    /// Check if a solution (config) is valid.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config).0
    }

    fn literal_value(lit: i32, assignment: &[bool]) -> bool {
        let var = lit.unsigned_abs() as usize - 1;
        let value = assignment.get(var).copied().unwrap_or(false);
        if lit > 0 {
            value
        } else {
            !value
        }
    }

    fn clause_is_nae_satisfied(clause: &CNFClause, assignment: &[bool]) -> bool {
        let mut has_true = false;
        let mut has_false = false;

        for &lit in &clause.literals {
            if Self::literal_value(lit, assignment) {
                has_true = true;
            } else {
                has_false = true;
            }

            if has_true && has_false {
                return true;
            }
        }

        false
    }
}

impl Problem for NAESatisfiability {
    const NAME: &'static str = "NAESatisfiability";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            let assignment = super::config_to_assignment(config);
            self.is_nae_satisfying(&assignment)
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default NAESatisfiability => "2^num_variables",
}

#[derive(Debug, Clone, Deserialize)]
struct NAESatisfiabilityDef {
    num_vars: usize,
    clauses: Vec<CNFClause>,
}

impl TryFrom<NAESatisfiabilityDef> for NAESatisfiability {
    type Error = String;

    fn try_from(value: NAESatisfiabilityDef) -> Result<Self, Self::Error> {
        Self::try_new(value.num_vars, value.clauses)
    }
}

fn validate_clause_lengths(clauses: &[CNFClause]) -> Result<(), String> {
    for (index, clause) in clauses.iter().enumerate() {
        if clause.len() < 2 {
            return Err(format!(
                "Clause {} has {} literals, expected at least 2",
                index,
                clause.len()
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "nae_satisfiability",
        instance: Box::new(NAESatisfiability::new(
            5,
            vec![
                CNFClause::new(vec![1, 2, -3]),
                CNFClause::new(vec![-1, 3, 4]),
                CNFClause::new(vec![2, -4, 5]),
                CNFClause::new(vec![-2, 3, -5]),
                CNFClause::new(vec![1, -3, 5]),
            ],
        )),
        optimal_config: vec![0, 0, 0, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/nae_satisfiability.rs"]
mod tests;
