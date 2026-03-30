//! Non-Tautology problem implementation.
//!
//! Given a Boolean formula in disjunctive normal form (DNF), determine whether
//! there exists a truth assignment that makes the formula false — i.e., the
//! formula is not a tautology.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "NonTautology",
        display_name: "Non-Tautology",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a falsifying assignment for a DNF formula (proving it is not a tautology)",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "disjuncts", type_name: "Vec<Vec<i32>>", description: "Disjuncts (each a conjunction of literals) in disjunctive normal form" },
        ],
    }
}

/// Non-Tautology problem.
///
/// Given a Boolean formula in DNF (disjunctive normal form) with disjuncts
/// D_1, ..., D_m, find a truth assignment that makes ALL disjuncts false
/// (i.e., the formula is not a tautology).
///
/// A disjunct is a conjunction (AND) of literals. The DNF formula is the
/// disjunction (OR) of all disjuncts. The formula is false when every
/// disjunct is false, which happens when each disjunct has at least one
/// false literal.
///
/// # Example
///
/// ```
/// use problemreductions::models::formula::NonTautology;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // (x1 AND x2 AND x3) OR (NOT x1 AND NOT x2 AND NOT x3)
/// let problem = NonTautology::new(
///     3,
///     vec![vec![1, 2, 3], vec![-1, -2, -3]],
/// );
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonTautology {
    /// Number of variables.
    num_vars: usize,
    /// Disjuncts in DNF. Each disjunct is a conjunction of literals
    /// represented as signed integers (positive = variable, negative = negation).
    disjuncts: Vec<Vec<i32>>,
}

impl NonTautology {
    /// Create a new Non-Tautology problem.
    ///
    /// # Panics
    /// Panics if any literal references a variable outside the range [1, num_vars].
    pub fn new(num_vars: usize, disjuncts: Vec<Vec<i32>>) -> Self {
        for (i, disjunct) in disjuncts.iter().enumerate() {
            for &lit in disjunct {
                let var = lit.unsigned_abs() as usize;
                assert!(
                    var >= 1 && var <= num_vars,
                    "Disjunct {} contains literal {} referencing variable {} outside range [1, {}]",
                    i,
                    lit,
                    var,
                    num_vars
                );
            }
        }
        Self {
            num_vars,
            disjuncts,
        }
    }

    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// Get the number of disjuncts.
    pub fn num_disjuncts(&self) -> usize {
        self.disjuncts.len()
    }

    /// Get the disjuncts.
    pub fn disjuncts(&self) -> &[Vec<i32>] {
        &self.disjuncts
    }

    /// Check if a literal is true under the given assignment.
    fn literal_is_true(lit: i32, assignment: &[bool]) -> bool {
        let var = lit.unsigned_abs() as usize - 1;
        let value = assignment.get(var).copied().unwrap_or(false);
        if lit > 0 {
            value
        } else {
            !value
        }
    }

    /// Check if all disjuncts are false (the formula evaluates to false).
    ///
    /// A disjunct (conjunction of literals) is true iff ALL its literals are true.
    /// The DNF formula is false iff ALL disjuncts are false, i.e., each disjunct
    /// has at least one false literal.
    pub fn is_falsifying(&self, assignment: &[bool]) -> bool {
        self.disjuncts.iter().all(|disjunct| {
            // A disjunct is false if at least one literal is false
            !disjunct
                .iter()
                .all(|&lit| Self::literal_is_true(lit, assignment))
        })
    }

    /// Convert a usize config to boolean assignment.
    fn config_to_assignment(config: &[usize]) -> Vec<bool> {
        config.iter().map(|&v| v == 1).collect()
    }
}

impl Problem for NonTautology {
    const NAME: &'static str = "NonTautology";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            let assignment = Self::config_to_assignment(config);
            self.is_falsifying(&assignment)
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default NonTautology => "1.307^num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "non_tautology",
        instance: Box::new(NonTautology::new(3, vec![vec![1, 2, 3], vec![-1, -2, -3]])),
        optimal_config: vec![1, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/non_tautology.rs"]
mod tests;
