//! Minimum-Weight Solution to Linear Equations.
//!
//! Given an integer matrix `A`, right-hand side `b`, and sparsity bound `K`,
//! determine whether the binary linear system `Ax = b` has a solution with at
//! most `K` nonzero entries.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumWeightSolutionToLinearEquations",
        display_name: "Minimum Weight Solution to Linear Equations",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a binary linear system has a solution with at most K nonzero entries",
        fields: &[
            FieldInfo { name: "coefficients", type_name: "Vec<Vec<i64>>", description: "Coefficient matrix A, stored row-by-row" },
            FieldInfo { name: "rhs", type_name: "Vec<i64>", description: "Right-hand side vector b" },
            FieldInfo { name: "bound", type_name: "usize", description: "Maximum number of nonzero variables allowed in the solution" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MinimumWeightSolutionToLinearEquations",
        fields: &["num_variables", "num_equations"],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumWeightSolutionToLinearEquations {
    coefficients: Vec<Vec<i64>>,
    rhs: Vec<i64>,
    bound: usize,
}

impl MinimumWeightSolutionToLinearEquations {
    pub fn new(coefficients: Vec<Vec<i64>>, rhs: Vec<i64>, bound: usize) -> Self {
        assert_eq!(
            coefficients.len(),
            rhs.len(),
            "rhs length must match number of equations"
        );

        if let Some(expected_width) = coefficients.first().map(Vec::len) {
            assert!(
                coefficients.iter().all(|row| row.len() == expected_width),
                "coefficient matrix must be rectangular"
            );
        }

        Self {
            coefficients,
            rhs,
            bound,
        }
    }

    pub fn coefficients(&self) -> &[Vec<i64>] {
        &self.coefficients
    }

    pub fn rhs(&self) -> &[i64] {
        &self.rhs
    }

    pub fn bound(&self) -> usize {
        self.bound
    }

    pub fn num_variables(&self) -> usize {
        self.coefficients.first().map_or(0, Vec::len)
    }

    pub fn num_equations(&self) -> usize {
        self.coefficients.len()
    }
}

impl Problem for MinimumWeightSolutionToLinearEquations {
    const NAME: &'static str = "MinimumWeightSolutionToLinearEquations";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_variables()]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if config.len() != self.num_variables() || config.iter().any(|&value| value > 1) {
            return Or(false);
        }

        if config.iter().filter(|&&value| value == 1).count() > self.bound {
            return Or(false);
        }

        let satisfies_equations = self.coefficients.iter().zip(&self.rhs).all(|(row, &rhs)| {
            row.iter()
                .zip(config)
                .map(|(&coefficient, &value)| coefficient * value as i64)
                .sum::<i64>()
                == rhs
        });

        Or(satisfies_equations)
    }
}

crate::declare_variants! {
    default MinimumWeightSolutionToLinearEquations => "2^num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_weight_solution_to_linear_equations",
        instance: Box::new(MinimumWeightSolutionToLinearEquations::new(
            vec![
                vec![1, 0, 1],
                vec![1, 0, 0],
                vec![1, 0, 0],
                vec![0, 1, 1],
                vec![0, 1, 1],
                vec![0, 1, 0],
            ],
            vec![1; 6],
            2,
        )),
        optimal_config: vec![1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/minimum_weight_solution_to_linear_equations.rs"]
mod tests;
