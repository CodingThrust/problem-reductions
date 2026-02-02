//! Boolean Satisfiability (SAT) problem implementation.
//!
//! SAT is the problem of determining if there exists an assignment of
//! Boolean variables that makes a given Boolean formula true.

use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::variant::short_type_name;
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};

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
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
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
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_cnf_clause_creation() {
        let clause = CNFClause::new(vec![1, -2, 3]);
        assert_eq!(clause.len(), 3);
        assert!(!clause.is_empty());
        assert_eq!(clause.variables(), vec![0, 1, 2]);
    }

    #[test]
    fn test_cnf_clause_satisfaction() {
        let clause = CNFClause::new(vec![1, 2]); // x1 OR x2

        assert!(clause.is_satisfied(&[true, false])); // x1 = T
        assert!(clause.is_satisfied(&[false, true])); // x2 = T
        assert!(clause.is_satisfied(&[true, true])); // Both T
        assert!(!clause.is_satisfied(&[false, false])); // Both F
    }

    #[test]
    fn test_cnf_clause_negation() {
        let clause = CNFClause::new(vec![-1, 2]); // NOT x1 OR x2

        assert!(clause.is_satisfied(&[false, false])); // NOT x1 = T
        assert!(clause.is_satisfied(&[false, true])); // Both true
        assert!(clause.is_satisfied(&[true, true])); // x2 = T
        assert!(!clause.is_satisfied(&[true, false])); // Both false
    }

    #[test]
    fn test_sat_creation() {
        let problem = Satisfiability::<i32>::new(
            3,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
        );
        assert_eq!(problem.num_vars(), 3);
        assert_eq!(problem.num_clauses(), 2);
        assert_eq!(problem.num_variables(), 3);
    }

    #[test]
    fn test_sat_with_weights() {
        let problem = Satisfiability::with_weights(
            2,
            vec![CNFClause::new(vec![1]), CNFClause::new(vec![2])],
            vec![5, 10],
        );
        assert_eq!(problem.weights(), vec![5, 10]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_is_satisfying() {
        // (x1 OR x2) AND (NOT x1 OR NOT x2)
        let problem = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
        );

        assert!(problem.is_satisfying(&[true, false])); // Satisfies both
        assert!(problem.is_satisfying(&[false, true])); // Satisfies both
        assert!(!problem.is_satisfying(&[true, true])); // Fails second clause
        assert!(!problem.is_satisfying(&[false, false])); // Fails first clause
    }

    #[test]
    fn test_count_satisfied() {
        let problem = Satisfiability::<i32>::new(
            2,
            vec![
                CNFClause::new(vec![1]),
                CNFClause::new(vec![2]),
                CNFClause::new(vec![-1, -2]),
            ],
        );

        assert_eq!(problem.count_satisfied(&[true, true]), 2); // x1, x2 satisfied
        assert_eq!(problem.count_satisfied(&[false, false]), 1); // Only last
        assert_eq!(problem.count_satisfied(&[true, false]), 2); // x1 and last
    }

    #[test]
    fn test_solution_size() {
        let problem = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
        );

        let sol = problem.solution_size(&[1, 0]); // true, false
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2); // Both clauses satisfied

        let sol = problem.solution_size(&[1, 1]); // true, true
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 1); // Only first clause satisfied
    }

    #[test]
    fn test_brute_force_satisfiable() {
        // (x1) AND (x2) AND (NOT x1 OR NOT x2) - UNSAT
        let problem = Satisfiability::<i32>::new(
            2,
            vec![
                CNFClause::new(vec![1]),
                CNFClause::new(vec![2]),
                CNFClause::new(vec![-1, -2]),
            ],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // This is unsatisfiable, so no valid solutions
        // BruteForce will return configs with max satisfied clauses
        for sol in &solutions {
            // Best we can do is satisfy 2 out of 3 clauses
            assert!(!problem.solution_size(sol).is_valid);
            assert_eq!(problem.solution_size(sol).size, 2);
        }
    }

    #[test]
    fn test_brute_force_simple_sat() {
        // (x1 OR x2) - many solutions
        let problem = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, 2])]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // 3 satisfying assignments
        assert_eq!(solutions.len(), 3);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_max_sat() {
        // Weighted: clause 1 has weight 10, clause 2 has weight 1
        // They conflict, so we prefer satisfying clause 1
        let problem = Satisfiability::with_weights(
            1,
            vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
            vec![10, 1],
        );
        let solver = BruteForce::new().valid_only(false); // Allow invalid (partial) solutions

        let solutions = solver.find_best(&problem);
        // Should select x1 = true (weight 10)
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1]);
    }

    #[test]
    fn test_is_satisfying_assignment() {
        let clauses = vec![vec![1, 2], vec![-1, 3]];

        assert!(is_satisfying_assignment(3, &clauses, &[true, false, true]));
        assert!(is_satisfying_assignment(3, &clauses, &[false, true, false]));
        assert!(!is_satisfying_assignment(
            3,
            &clauses,
            &[true, false, false]
        ));
    }

    #[test]
    fn test_constraints() {
        let problem = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
        );
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2);
    }

    #[test]
    fn test_energy_mode() {
        let problem = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1])]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_empty_formula() {
        let problem = Satisfiability::<i32>::new(2, vec![]);
        let sol = problem.solution_size(&[0, 0]);
        assert!(sol.is_valid); // Empty formula is trivially satisfied
    }

    #[test]
    fn test_single_literal_clauses() {
        // Unit propagation scenario: x1 AND NOT x2
        let problem =
            Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-2])]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 0]); // x1=T, x2=F
    }

    #[test]
    fn test_get_clause() {
        let problem = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
        );
        assert_eq!(problem.get_clause(0), Some(&CNFClause::new(vec![1, 2])));
        assert_eq!(problem.get_clause(2), None);
    }

    #[test]
    fn test_three_sat_example() {
        // (x1 OR x2 OR x3) AND (NOT x1 OR NOT x2 OR x3) AND (x1 OR NOT x2 OR NOT x3)
        let problem = Satisfiability::<i32>::new(
            3,
            vec![
                CNFClause::new(vec![1, 2, 3]),
                CNFClause::new(vec![-1, -2, 3]),
                CNFClause::new(vec![1, -2, -3]),
            ],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_is_satisfied_csp() {
        let problem = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
        );

        assert!(problem.is_satisfied(&[1, 0]));
        assert!(problem.is_satisfied(&[0, 1]));
        assert!(!problem.is_satisfied(&[1, 1]));
        assert!(!problem.is_satisfied(&[0, 0]));
    }

    #[test]
    fn test_objectives() {
        let problem = Satisfiability::with_weights(2, vec![CNFClause::new(vec![1, 2])], vec![5]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 1);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
        );
        assert!(!problem.is_weighted()); // Initially uniform
        problem.set_weights(vec![1, 2]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights(), vec![1, 2]);
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = Satisfiability::<i32>::new(2, vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_satisfying_assignment_defaults() {
        // When assignment is shorter than needed, missing vars default to false
        let clauses = vec![vec![1, 2]];
        // assignment is [true], var 0 = true satisfies literal 1
        assert!(is_satisfying_assignment(3, &clauses, &[true]));
        // assignment is [false], var 0 = false, var 1 defaults to false
        // Neither literal 1 (var0=false) nor literal 2 (var1=false) satisfied
        assert!(!is_satisfying_assignment(3, &clauses, &[false]));
    }

    #[test]
    fn test_problem_size() {
        let problem = Satisfiability::<i32>::new(
            3,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
        );
        let size = problem.problem_size();
        assert_eq!(size.get("num_vars"), Some(3));
        assert_eq!(size.get("num_clauses"), Some(2));
    }

    #[test]
    fn test_num_variables_flavors() {
        let problem = Satisfiability::<i32>::new(5, vec![CNFClause::new(vec![1])]);
        assert_eq!(problem.num_variables(), 5);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_clause_variables() {
        let clause = CNFClause::new(vec![1, -2, 3]);
        let vars = clause.variables();
        assert_eq!(vars, vec![0, 1, 2]); // 0-indexed
    }

    #[test]
    fn test_clause_debug() {
        let clause = CNFClause::new(vec![1, -2, 3]);
        let debug = format!("{:?}", clause);
        assert!(debug.contains("CNFClause"));
    }
}
