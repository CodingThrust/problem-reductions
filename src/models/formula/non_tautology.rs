//! Non-Tautology problem for DNF formulas.
//!
//! Given a formula in disjunctive normal form (DNF), determine whether there
//! exists an assignment that falsifies the formula.

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
        description: "Find a falsifying assignment for a DNF formula",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "disjuncts", type_name: "Vec<Vec<i32>>", description: "DNF disjuncts, each a conjunction of signed literals" },
        ],
    }
}

/// Non-Tautology for Boolean formulas in disjunctive normal form (DNF).
///
/// The instance asks whether there exists an assignment under which the DNF
/// formula evaluates to false.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonTautology {
    /// Number of Boolean variables.
    num_vars: usize,
    /// DNF disjuncts, each represented as a conjunction of signed literals.
    disjuncts: Vec<Vec<i32>>,
}

impl NonTautology {
    /// Create a new NonTautology instance.
    pub fn new(num_vars: usize, disjuncts: Vec<Vec<i32>>) -> Self {
        Self {
            num_vars,
            disjuncts,
        }
    }

    /// Get the number of Boolean variables.
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

    fn literal_is_true(lit: i32, config: &[usize]) -> bool {
        let var = lit.unsigned_abs() as usize - 1;
        let value = config.get(var).copied().unwrap_or(0);
        if lit > 0 {
            value == 1
        } else {
            value == 0
        }
    }
}

impl Problem for NonTautology {
    const NAME: &'static str = "NonTautology";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        let e_value = self.disjuncts.iter().any(|disjunct| {
            disjunct
                .iter()
                .all(|&lit| Self::literal_is_true(lit, config))
        });
        crate::types::Or(!e_value)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default NonTautology => "1.307^num_vars",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "non_tautology",
        instance: Box::new(NonTautology::new(2, vec![vec![1, 2], vec![-1, -2]])),
        optimal_config: vec![1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/formula/non_tautology.rs"]
mod tests;
