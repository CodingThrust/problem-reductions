//! Natural embedding of binary ILP into general integer ILP.
//!
//! The stored `[0, 1]` bounds, constraints, and objective carry over unchanged.
//!
//! This same-name variant reduction preserves the witness representation.

use crate::models::algebraic::ILP;
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;

#[derive(Debug, Clone)]
pub struct ReductionBinaryILPToIntILP {
    target: ILP<i64>,
}

impl ReductionResult for ReductionBinaryILPToIntILP {
    type Source = ILP<bool>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| Ok(solution.clone()))
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vars",
        num_constraints = "num_constraints",
        num_nonzeros = "num_nonzeros",
    },)]
impl ReduceTo<ILP<i64>> for ILP<bool> {
    type Result = ReductionBinaryILPToIntILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(ReductionBinaryILPToIntILP {
            target: ILP::<i64>::with_variables(
                self.variables().to_vec(),
                self.constraints().to_vec(),
                self.objective().to_vec(),
                self.sense(),
            )
            .map_err(<Self as ReduceTo<ILP<i64>>>::target_construction)?,
        })
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_bool_ilp_i64.rs"]
mod tests;
