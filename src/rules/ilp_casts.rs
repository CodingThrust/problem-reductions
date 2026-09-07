//! Numeric variant reductions for ILP.

use crate::models::algebraic::{Comparison, LinearConstraint, VariableDomain, ILP};
use crate::reduction;
use crate::rules::{ReduceTo, ReductionError, ReductionResult};
use crate::types::i64_to_exact_f64;

#[derive(Debug, Clone)]
pub struct ReductionILPToFloat<V: VariableDomain> {
    source: ILP<V>,
    target: ILP<V, f64>,
}

impl<V: VariableDomain> ReductionILPToFloat<V> {
    fn new(source: &ILP<V>) -> Result<Self, ReductionError> {
        let convert = |coefficient: i64| {
            i64_to_exact_f64(coefficient)
                .map_err(ReductionError::inexact_float_conversion::<ILP<V>, ILP<V, f64>>)
        };
        let constraints = source
            .constraints()
            .iter()
            .map(|constraint| {
                let terms = constraint
                    .terms()
                    .iter()
                    .map(|&(variable, coefficient)| Ok((variable, convert(coefficient)?)))
                    .collect::<Result<Vec<_>, ReductionError>>()?;
                let rhs = convert(constraint.rhs())?;
                Ok(match constraint.comparison() {
                    Comparison::Le => LinearConstraint::le(terms, rhs),
                    Comparison::Ge => LinearConstraint::ge(terms, rhs),
                    Comparison::Eq => LinearConstraint::eq(terms, rhs),
                })
            })
            .collect::<Result<Vec<_>, ReductionError>>()?;
        let objective = source
            .objective()
            .iter()
            .map(|&(variable, coefficient)| Ok((variable, convert(coefficient)?)))
            .collect::<Result<Vec<_>, ReductionError>>()?;
        let target = ILP::with_variables(
            source.variables().to_vec(),
            constraints,
            objective,
            source.sense(),
        )
        .map_err(ReductionError::construction::<ILP<V>, ILP<V, f64>>)?;
        Ok(Self {
            source: source.clone(),
            target,
        })
    }
}

impl<V: VariableDomain> ReductionResult for ReductionILPToFloat<V> {
    type Source = ILP<V>;
    type Target = ILP<V, f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !self.source.is_feasible(target_solution)? {
            return Err(crate::rules::ExtractionError::invalid(
                "the floating-point assignment violates the source integer ILP",
            ));
        }
        Ok(target_solution.clone())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vars",
        num_constraints = "num_constraints",
        num_nonzeros = "num_nonzeros",
    },
)]
impl ReduceTo<ILP<bool, f64>> for ILP<bool> {
    type Result = ReductionILPToFloat<bool>;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        ReductionILPToFloat::new(self)
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vars",
        num_constraints = "num_constraints",
        num_nonzeros = "num_nonzeros",
    },
)]
impl ReduceTo<ILP<i64, f64>> for ILP<i64> {
    type Result = ReductionILPToFloat<i64>;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        ReductionILPToFloat::new(self)
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_casts.rs"]
mod tests;
