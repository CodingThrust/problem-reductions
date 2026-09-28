//! Forget the bounded-interval certificate while retaining the entire ILP.

use crate::models::algebraic::{Bounded, ILP};
use crate::reduction;
use crate::rules::{ReduceTo, ReductionResult};

/// Identity embedding of bounded integer ILP into general integer ILP.
#[derive(Debug, Clone)]
pub struct ReductionBoundedILPToILP {
    target: ILP<i64>,
}

impl ReductionResult for ReductionBoundedILPToILP {
    type Source = ILP<i64, i64, Bounded>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<i64>> {
        crate::rules::traits::validate_target_witness(
            ReductionResult::target_problem(self),
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        Ok(solution.clone())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionBoundedILPToILP {}

#[reduction(transform = exact {
    max_constraint_magnitude_bits = "max_constraint_magnitude_bits",
    num_vars = "num_vars",
    num_constraints = "num_constraints",
    num_nonzeros = "num_nonzeros",
})]
impl ReduceTo<ILP<i64>> for ILP<i64, i64, Bounded> {
    type Result = ReductionBoundedILPToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(ReductionBoundedILPToILP {
            target: ILP::with_variables(
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
#[path = "../unit_tests/rules/ilp_bounded_ilp.rs"]
mod tests;
