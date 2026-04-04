//! Algebraic Equations over GF(2).
//!
//! Given Boolean variables and a system of polynomial equations over GF(2),
//! determine whether there is a binary assignment satisfying every equation.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "AlgebraicEquationsOverGF2",
        display_name: "Algebraic Equations over GF(2)",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a system of polynomial equations over GF(2) has a satisfying binary assignment",
        fields: &[
            FieldInfo { name: "num_vars", type_name: "usize", description: "Number of binary variables" },
            FieldInfo { name: "equations", type_name: "Vec<Vec<Vec<usize>>>", description: "Equations represented as XORs of AND-terms over variable indices; an empty term denotes the constant 1" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "AlgebraicEquationsOverGF2",
        fields: &["num_vars", "num_equations"],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgebraicEquationsOverGF2 {
    num_vars: usize,
    equations: Vec<Vec<Vec<usize>>>,
}

impl AlgebraicEquationsOverGF2 {
    pub fn new(num_vars: usize, equations: Vec<Vec<Vec<usize>>>) -> Self {
        for (equation_index, equation) in equations.iter().enumerate() {
            for (term_index, term) in equation.iter().enumerate() {
                for &var in term {
                    assert!(
                        var < num_vars,
                        "Equation {} term {} references variable {} outside 0..{}",
                        equation_index,
                        term_index,
                        var,
                        num_vars
                    );
                }
            }
        }

        Self {
            num_vars,
            equations,
        }
    }

    pub fn num_vars(&self) -> usize {
        self.num_vars
    }

    pub fn equations(&self) -> &[Vec<Vec<usize>>] {
        &self.equations
    }

    pub fn num_equations(&self) -> usize {
        self.equations.len()
    }
}

impl Problem for AlgebraicEquationsOverGF2 {
    const NAME: &'static str = "AlgebraicEquationsOverGF2";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if config.len() != self.num_vars || config.iter().any(|&value| value > 1) {
            return Or(false);
        }

        let all_equations_vanish = self.equations.iter().all(|equation| {
            let value = equation.iter().fold(0usize, |acc, term| {
                let term_value = if term.is_empty() {
                    1
                } else {
                    term.iter()
                        .fold(1usize, |product, &var| product * config[var])
                };
                acc ^ term_value
            });
            value == 0
        });

        Or(all_equations_vanish)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default AlgebraicEquationsOverGF2 => "2^num_vars",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "algebraic_equations_over_gf2",
        instance: Box::new(AlgebraicEquationsOverGF2::new(
            3,
            vec![vec![vec![0], vec![]], vec![vec![1]], vec![vec![2], vec![]]],
        )),
        optimal_config: vec![1, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/algebraic_equations_over_gf2.rs"]
mod tests;
