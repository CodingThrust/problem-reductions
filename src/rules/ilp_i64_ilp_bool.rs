//! Encode finitely bounded integer ILP variables as binary variables.

use crate::models::algebraic::{Comparison, LinearConstraint, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::ReductionError;
use crate::types::i64_to_exact_f64;

#[derive(Debug, Clone)]
struct VarEncoding {
    lower_bound: i64,
    start: usize,
    weights: Vec<i64>,
}

fn overflow(operation: impl Into<String>) -> ReductionError {
    ReductionError::integer_overflow::<ILP<i64>, ILP<bool>>(operation)
}

fn binary_weights(width: i64) -> Vec<i64> {
    if width == 0 {
        return Vec::new();
    }
    let num_bits = 64 - width.leading_zeros() as usize;
    let mut weights = Vec::with_capacity(num_bits);
    for bit in 0..num_bits - 1 {
        weights.push(1_i64 << bit);
    }
    weights.push(width - ((1_i64 << (num_bits - 1)) - 1));
    weights
}

fn encoded_constraint(
    constraint: &LinearConstraint,
    encodings: &[VarEncoding],
) -> Result<LinearConstraint, ReductionError> {
    let mut terms = Vec::new();
    let mut constant = 0_i64;
    for &(variable, coefficient) in constraint.terms() {
        let encoding = &encodings[variable];
        constant = constant
            .checked_add(
                coefficient
                    .checked_mul(encoding.lower_bound)
                    .ok_or_else(|| {
                        overflow("multiplying an ILP row coefficient by a lower bound")
                    })?,
            )
            .ok_or_else(|| overflow("summing the lower-bound shift of an ILP row"))?;
        for (offset, &weight) in encoding.weights.iter().enumerate() {
            terms.push((
                encoding.start + offset,
                coefficient
                    .checked_mul(weight)
                    .ok_or_else(|| overflow("encoding an integer ILP row coefficient"))?,
            ));
        }
    }
    let rhs = constraint
        .rhs()
        .checked_sub(constant)
        .ok_or_else(|| overflow("shifting an integer ILP right-hand side"))?;
    Ok(match constraint.comparison() {
        Comparison::Le => LinearConstraint::le(terms, rhs),
        Comparison::Ge => LinearConstraint::ge(terms, rhs),
        Comparison::Eq => LinearConstraint::eq(terms, rhs),
    })
}

#[derive(Debug, Clone)]
pub struct ReductionIntILPToBinaryILP {
    target: ILP<bool>,
    encodings: Vec<VarEncoding>,
}

impl ReductionResult for ReductionIntILPToBinaryILP {
    type Source = ILP<i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        self.encodings
            .iter()
            .map(|encoding| {
                encoding.weights.iter().enumerate().try_fold(
                    encoding.lower_bound,
                    |value, (offset, &weight)| {
                        let term = weight
                            .checked_mul(target_solution[encoding.start + offset])
                            .ok_or_else(|| {
                                crate::rules::ExtractionError::invalid(
                                    "binary ILP decoding multiplication overflowed i64",
                                )
                            })?;
                        value.checked_add(term).ok_or_else(|| {
                            crate::rules::ExtractionError::invalid(
                                "binary ILP decoding sum overflowed i64",
                            )
                        })
                    },
                )
            })
            .collect()
    }
}

#[reduction(
    transform = unavailable {
        num_vars = "the binary width depends on concrete variable bounds, not registered problem parameters",
        num_constraints = "the exact row count is preserved but the target parameters model is unavailable until all ILP overhead declarations are migrated",
        num_nonzeros = "binary expansion depends on concrete variable bounds and row sparsity",
    },
)]
impl ReduceTo<ILP<bool>> for ILP<i64> {
    type Result = ReductionIntILPToBinaryILP;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let mut encodings = Vec::with_capacity(self.num_vars());
        let mut num_binary_variables = 0_usize;
        for variable in self.variables() {
            let lower_bound = variable.lower_bound().ok_or_else(|| {
                ReductionError::invalid_target::<ILP<i64>, ILP<bool>>(
                    "binary encoding requires a finite lower bound for every integer variable",
                )
            })?;
            let upper_bound = variable.upper_bound().ok_or_else(|| {
                ReductionError::invalid_target::<ILP<i64>, ILP<bool>>(
                    "binary encoding requires a finite upper bound for every integer variable",
                )
            })?;
            let width = upper_bound
                .checked_sub(lower_bound)
                .ok_or_else(|| overflow("computing an integer variable interval width"))?;
            let weights = binary_weights(width);
            let num_weights = weights.len();
            encodings.push(VarEncoding {
                lower_bound,
                start: num_binary_variables,
                weights,
            });
            num_binary_variables = num_binary_variables
                .checked_add(num_weights)
                .ok_or_else(|| overflow("counting binary encoding variables"))?;
        }

        let constraints = self
            .constraints()
            .iter()
            .map(|constraint| encoded_constraint(constraint, &encodings))
            .collect::<Result<Vec<_>, _>>()?;

        let mut objective = Vec::new();
        for &(variable, coefficient) in self.objective() {
            let encoding = &encodings[variable];
            for (offset, &weight) in encoding.weights.iter().enumerate() {
                let encoded_coefficient = coefficient
                    * i64_to_exact_f64(weight).map_err(|error| {
                        ReductionError::inexact_float_conversion::<ILP<i64>, ILP<bool>>(error)
                    })?;
                if !encoded_coefficient.is_finite() {
                    return Err(ReductionError::non_finite_result::<ILP<i64>, ILP<bool>>(
                        "encoding an integer ILP objective coefficient",
                    ));
                }
                objective.push((encoding.start + offset, encoded_coefficient));
            }
        }

        Ok(ReductionIntILPToBinaryILP {
            target: ILP::<bool>::new(num_binary_variables, constraints, objective, self.sense())
                .map_err(Self::target_construction)?,
            encodings,
        })
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_i64_ilp_bool.rs"]
mod tests;
