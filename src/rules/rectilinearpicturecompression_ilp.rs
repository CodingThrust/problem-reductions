//! Reduction from RectilinearPictureCompression to ILP (Integer Linear Programming).
//!
//! Binary variable x_r per maximal rectangle. For each 1-cell, require at least
//! one covering rectangle selected. Total selected ≤ bound.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::RectilinearPictureCompression;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionRPCToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionRPCToILP {
    type Source = RectilinearPictureCompression;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_rows * num_cols",
        num_constraints = "num_rows * num_cols + 1",
    }
)]
impl ReduceTo<ILP<bool>> for RectilinearPictureCompression {
    type Result = ReductionRPCToILP;

    fn reduce_to(&self) -> Self::Result {
        let rects = self.maximal_rectangles();
        let num_vars = rects.len();
        let mut constraints = Vec::new();

        // For each 1-cell, require at least one covering rectangle selected
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                if self.matrix()[i][j] {
                    let terms: Vec<(usize, f64)> = rects
                        .iter()
                        .enumerate()
                        .filter(|(_, &(r1, c1, r2, c2))| i >= r1 && i <= r2 && j >= c1 && j <= c2)
                        .map(|(idx, _)| (idx, 1.0))
                        .collect();
                    constraints.push(LinearConstraint::ge(terms, 1.0));
                }
            }
        }

        // Bound constraint: Σ x_r ≤ bound
        let bound_terms: Vec<(usize, f64)> = (0..num_vars).map(|i| (i, 1.0)).collect();
        constraints.push(LinearConstraint::le(bound_terms, self.bound() as f64));

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionRPCToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "rectilinearpicturecompression_to_ilp",
        build: || {
            let source =
                RectilinearPictureCompression::new(vec![vec![true, true], vec![true, true]], 1);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1],
                    target_config: vec![1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/rectilinearpicturecompression_ilp.rs"]
mod tests;
