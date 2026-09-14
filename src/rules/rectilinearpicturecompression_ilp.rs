//! Reduction from RectilinearPictureCompression to ILP (Integer Linear Programming).
//!
//! Binary variable x_r per maximal rectangle. For each 1-cell, require at least
//! one covering rectangle selected. Total selected ≤ bound.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::RectilinearPictureCompression;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;

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

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        match target {
            SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
            SolveOutcome::Optimal { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::optimal(source, solution)?)
            }
            SolveOutcome::Feasible { solution, .. } => {
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::feasible(source, solution)?)
            }
        }
    }
}

impl ReductionRPCToILP {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "num_rows^2 * num_cols^2",
        num_constraints = "num_rows * num_cols + 1",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
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
