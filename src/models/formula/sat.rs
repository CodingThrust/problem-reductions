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
        display_name: "Satisfiability",
        aliases: &["SAT"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Formula,
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
    pub literals: Vec<i64>,
}

impl CNFClause {
    /// Create a new clause from literals.
    ///
    /// Literals are signed integers where positive means the variable
    /// and negative means its negation. Variables are 1-indexed.
    pub fn new(literals: Vec<i64>) -> Self {
        Self { literals }
    }

    /// Check if the clause is satisfied by an assignment.
    ///
    /// # Arguments
    /// * `assignment` - Boolean assignment, 0-indexed
    pub fn is_satisfied(&self, assignment: &[bool]) -> bool {
        self.literals.iter().any(|&lit| {
            let var = usize::try_from(lit.unsigned_abs())
                .expect("i64 literal magnitude must fit usize")
                .checked_sub(1)
                .expect("CNF literal 0 is invalid");
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
            .map(|&lit| {
                usize::try_from(lit.unsigned_abs())
                    .expect("i64 literal magnitude must fit usize")
                    .checked_sub(1)
                    .expect("CNF literal 0 is invalid")
            })
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
/// use problemreductions::models::formula::{Satisfiability, CNFClause};
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
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Verify solutions satisfy all clauses
/// for sol in solutions {
///     assert!(problem.evaluate(&sol).unwrap());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "SatisfiabilityDef")]
pub struct Satisfiability {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF.
    clauses: Vec<CNFClause>,
}

impl Satisfiability {
    /// Create a new SAT problem.
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self {
        Self::try_new(num_vars, clauses).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Create a new SAT problem after validating its literal encoding.
    pub fn try_new(
        num_vars: usize,
        clauses: Vec<CNFClause>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        validate_cnf_literals(num_vars, &clauses)?;
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

    /// Count satisfied clauses for an assignment.
    pub fn count_satisfied(
        &self,
        assignment: &[bool],
    ) -> Result<i64, crate::traits::EvaluationError> {
        let count = self
            .clauses
            .iter()
            .filter(|c| c.is_satisfied(assignment))
            .count();
        i64::try_from(count).map_err(|_| {
            crate::traits::EvaluationError::IntegerOverflow(
                "converting satisfied-clause count to i64".into(),
            )
        })
    }

    /// Check if an assignment satisfies all clauses.
    pub fn is_satisfying(&self, assignment: &[bool]) -> bool {
        self.clauses.iter().all(|c| c.is_satisfied(assignment))
    }

    /// Check if a solution (config) is valid.
    ///
    /// For SAT, a valid solution is one that satisfies all clauses.
    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(self.evaluate(config)?.0)
    }
}

impl Problem for Satisfiability {
    const NAME: &'static str = "Satisfiability";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                let assignment = super::config_to_assignment(config);
                self.is_satisfying(&assignment)
            })
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default Satisfiability => "2^num_variables",
}

#[derive(Deserialize)]
struct SatisfiabilityDef {
    num_vars: usize,
    clauses: Vec<CNFClause>,
}

impl TryFrom<SatisfiabilityDef> for Satisfiability {
    type Error = crate::registry::ConstructionError;

    fn try_from(value: SatisfiabilityDef) -> Result<Self, Self::Error> {
        Self::try_new(value.num_vars, value.clauses)
    }
}

pub(super) fn validate_cnf_literals(
    num_vars: usize,
    clauses: &[CNFClause],
) -> Result<(), crate::registry::ConstructionError> {
    if num_vars > i64::MAX as usize {
        return Err(format!(
            "num_vars {num_vars} exceeds the SAT literal limit {}",
            i64::MAX
        )
        .into());
    }

    for (clause_index, clause) in clauses.iter().enumerate() {
        for &literal in &clause.literals {
            if literal == 0 || literal == i64::MIN {
                return Err(format!(
                    "clause {clause_index} contains invalid literal {literal}; allowed variable numbers are 1..={num_vars} with either sign"
                ).into());
            }
            let magnitude = usize::try_from(literal.unsigned_abs()).map_err(|_| {
                format!("clause {clause_index} literal {literal} magnitude does not fit usize")
            })?;
            if magnitude > num_vars {
                return Err(format!(
                    "clause {clause_index} contains invalid literal {literal}; allowed variable numbers are 1..={num_vars} with either sign"
                ).into());
            }
        }
    }

    Ok(())
}

/// Check if an assignment satisfies a SAT formula.
///
/// # Arguments
/// * `num_vars` - Number of variables
/// * `clauses` - Clauses as vectors of literals (1-indexed, signed)
/// * `assignment` - Boolean assignment (0-indexed)
#[cfg(test)]
pub(crate) fn is_satisfying_assignment(
    _num_vars: usize,
    clauses: &[Vec<i64>],
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

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "satisfiability",
        instance: Box::new(Satisfiability::new(
            3,
            vec![
                CNFClause::new(vec![1, 2]),
                CNFClause::new(vec![-1, 3]),
                CNFClause::new(vec![-2, -3]),
            ],
        )),
        optimal_config: vec![0, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/sat.rs"]
mod tests;
