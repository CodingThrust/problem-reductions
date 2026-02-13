//! Boolean Satisfiability (SAT) problem implementation.
//!
//! SAT is the problem of determining if there exists an assignment of
//! Boolean variables that makes a given Boolean formula true. This is
//! the decision version - for the optimization variant (MAX-SAT), see
//! the separate MaxSatisfiability type (if available).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Satisfiability",
        module_path: module_path!(),
        description: "Find satisfying assignment for CNF formula",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses in conjunctive normal form" },
        ],
    }
}

/// A clause in conjunctive normal form (CNF).
///
/// A clause is a disjunction (OR) of literals.
/// Literals are represented as signed integers:
/// - Positive i means variable i
/// - Negative -i means NOT variable i
///
/// Variables are 1-indexed in the external representation but
/// 0-indexed internally.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CNFClause {
    /// Literals in this clause (signed integers, 1-indexed).
    pub literals: Vec<i32>,
}

impl CNFClause {
    /// Create a new clause from literals.
    ///
    /// Literals are signed integers where positive means the variable
    /// and negative means its negation. Variables are 1-indexed.
    pub fn new(literals: Vec<i32>) -> Self {
        Self { literals }
    }

    /// Check if the clause is satisfied by an assignment.
    ///
    /// # Arguments
    /// * `assignment` - Boolean assignment, 0-indexed
    pub fn is_satisfied(&self, assignment: &[bool]) -> bool {
        self.literals.iter().any(|&lit| {
            let var = lit.unsigned_abs() as usize - 1; // Convert to 0-indexed
            let value = assignment.get(var).copied().unwrap_or(false);
            if lit > 0 {
                value
            } else {
                !value
            }
        })
    }

    /// Get the variables involved in this clause (0-indexed).
    pub fn variables(&self) -> Vec<usize> {
        self.literals
            .iter()
            .map(|&lit| lit.unsigned_abs() as usize - 1)
            .collect()
    }

    /// Get the number of literals.
    pub fn len(&self) -> usize {
        self.literals.len()
    }

    /// Check if the clause is empty.
    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
}

/// Boolean Satisfiability (SAT) problem in CNF form.
///
/// Given a Boolean formula in conjunctive normal form (CNF),
/// determine if there exists an assignment that satisfies all clauses.
/// This is the decision version of the problem.
///
/// # Example
///
/// ```
/// use problemreductions::models::satisfiability::{Satisfiability, CNFClause};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Formula: (x1 OR x2) AND (NOT x1 OR x3) AND (NOT x2 OR NOT x3)
/// let problem = Satisfiability::new(
///     3,
///     vec![
///         CNFClause::new(vec![1, 2]),      // x1 OR x2
///         CNFClause::new(vec![-1, 3]),     // NOT x1 OR x3
///         CNFClause::new(vec![-2, -3]),    // NOT x2 OR NOT x3
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_satisfying(&problem);
///
/// // Verify solutions satisfy all clauses
/// for sol in solutions {
///     assert!(problem.evaluate(&sol));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Satisfiability {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF.
    clauses: Vec<CNFClause>,
}

impl Satisfiability {
    /// Create a new SAT problem.
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
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

impl Problem for Satisfiability {
    const NAME: &'static str = "Satisfiability";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let assignment = Self::config_to_assignment(config);
        self.is_satisfying(&assignment)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
}

/// Check if an assignment satisfies a SAT formula.
///
/// # Arguments
/// * `num_vars` - Number of variables
/// * `clauses` - Clauses as vectors of literals (1-indexed, signed)
/// * `assignment` - Boolean assignment (0-indexed)
pub fn is_satisfying_assignment(
    _num_vars: usize,
    clauses: &[Vec<i32>],
    assignment: &[bool],
) -> bool {
    clauses.iter().all(|clause| {
        clause.iter().any(|&lit| {
            let var = lit.unsigned_abs() as usize - 1;
            let value = assignment.get(var).copied().unwrap_or(false);
            if lit > 0 {
                value
            } else {
                !value
            }
        })
    })
}

#[cfg(test)]
#[path = "../../unit_tests/models/satisfiability/sat.rs"]
mod tests;
