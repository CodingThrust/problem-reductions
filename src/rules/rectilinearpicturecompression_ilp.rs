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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

#[reduction(
    size = upper_bound {
        num_vars = "num_rows^2 * num_cols^2",
        num_constraints = "num_rows * num_cols + 1",
    }
)]
impl ReduceTo<ILP<bool>> for RectilinearPictureCompression {
    type Result = ReductionRPCToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let rects = self.maximal_rectangles();
        let num_vars = rects.len();
        let mut constraints = Vec::new();

        // For each 1-cell, require at least one covering rectangle selected
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                if self.matrix()[i][j] {
                    let terms: Vec<(usize, i64)> = rects
                        .iter()
                        .enumerate()
                        .filter(|(_, &(r1, c1, r2, c2))| i >= r1 && i <= r2 && j >= c1 && j <= c2)
                        .map(|(idx, _)| (idx, 1))
                        .collect();
                    constraints.push(LinearConstraint::ge(terms, 1));
                }
            }
        }

        // Bound constraint: Σ x_r ≤ bound
        let bound_terms: Vec<(usize, i64)> = (0..num_vars).map(|i| (i, 1)).collect();
        constraints.push(LinearConstraint::le(bound_terms, self.bound()));

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionRPCToILP { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "rectilinearpicturecompression_to_ilp",
        build: || {
            let source =
                RectilinearPictureCompression::new(vec![vec![true, true], vec![true, true]], 1);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/rectilinearpicturecompression_ilp.rs"]
mod tests;
