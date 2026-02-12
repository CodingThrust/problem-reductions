//! Boolean Satisfiability (SAT) problem implementation.
//!
//! SAT is the problem of determining if there exists an assignment of
//! Boolean variables that makes a given Boolean formula true.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Satisfiability",
        category: "satisfiability",
        description: "Find satisfying assignment for CNF formula",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "clauses", type_name: "Vec<CNFClause>", description: "Clauses in conjunctive normal form" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Clause weights for MAX-SAT" },
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
///
/// The problem can be weighted, where the goal is to maximize the
/// total weight of satisfied clauses (MAX-SAT).
///
/// # Example
///
/// ```
/// use problemreductions::models::satisfiability::{Satisfiability, CNFClause};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Formula: (x1 OR x2) AND (NOT x1 OR x3) AND (NOT x2 OR NOT x3)
/// let problem = Satisfiability::<i32>::new(
///     3,
///     vec![
///         CNFClause::new(vec![1, 2]),      // x1 OR x2
///         CNFClause::new(vec![-1, 3]),     // NOT x1 OR x3
///         CNFClause::new(vec![-2, -3]),    // NOT x2 OR NOT x3
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Verify solutions satisfy all clauses
/// for sol in solutions {
///     assert!(problem.solution_size(&sol).is_valid);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Satisfiability<W = i32> {
    /// Number of variables.
    num_vars: usize,
    /// Clauses in CNF.
    clauses: Vec<CNFClause>,
    /// Weights for each clause (for MAX-SAT).
    weights: Vec<W>,
}

impl<W: Clone + Default> Satisfiability<W> {
    /// Create a new SAT problem with unit weights.
    pub fn new(num_vars: usize, clauses: Vec<CNFClause>) -> Self
    where
        W: From<i32>,
    {
        let num_clauses = clauses.len();
        let weights = vec![W::from(1); num_clauses];
        Self {
            num_vars,
            clauses,
            weights,
        }
    }

    /// Create a new weighted SAT problem (MAX-SAT).
    pub fn with_weights(num_vars: usize, clauses: Vec<CNFClause>, weights: Vec<W>) -> Self {
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

impl<W> Problem for Satisfiability<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "Satisfiability";

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
            ("num_vars", self.num_vars),
            ("num_clauses", self.clauses.len()),
            ("num_literals", self.num_literals()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        // For standard SAT, we maximize satisfied clauses (all must be satisfied)
        // For MAX-SAT, we maximize weighted sum of satisfied clauses
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let assignment = Self::config_to_assignment(config);
        let is_valid = self.is_satisfying(&assignment);

        // Compute weighted sum of satisfied clauses
        let mut total = W::zero();
        for (clause, weight) in self.clauses.iter().zip(&self.weights) {
            if clause.is_satisfied(&assignment) {
                total += weight.clone();
            }
        }

        SolutionSize::new(total, is_valid)
    }
}

impl<W> ConstraintSatisfactionProblem for Satisfiability<W>
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
        // Each clause is a constraint
        self.clauses
            .iter()
            .map(|clause| {
                let vars = clause.variables();
                let num_configs = 2usize.pow(vars.len() as u32);

                // Build spec: config is valid if clause is satisfied
                let spec: Vec<bool> = (0..num_configs)
                    .map(|config_idx| {
                        // Convert config index to local assignment
                        let local_assignment: Vec<bool> = (0..vars.len())
                            .map(|i| (config_idx >> (vars.len() - 1 - i)) & 1 == 1)
                            .collect();

                        // Build full assignment for clause evaluation
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
        // For MAX-SAT, each clause contributes its weight if satisfied
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

// === ProblemV2 implementation ===

impl<W> crate::traits::ProblemV2 for Satisfiability<W>
where
    W: Clone + Default + 'static,
{
    const NAME: &'static str = "Satisfiability";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let assignment = Self::config_to_assignment(config);
        self.is_satisfying(&assignment)
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
