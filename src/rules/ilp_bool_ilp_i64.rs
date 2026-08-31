//! Natural embedding of binary ILP into general integer ILP.
//!
//! The stored `[0, 1]` bounds, constraints, and objective carry over unchanged.
//!
//! This same-name variant reduction preserves the witness representation.

use crate::models::algebraic::ILP;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
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
