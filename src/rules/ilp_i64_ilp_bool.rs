//! Encode finitely bounded integer ILP variables as binary variables.

use crate::models::algebraic::{Bounded, Comparison, LinearConstraint, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::ReductionError;

#[derive(Debug, Clone)]
struct VarEncoding {
    lower_bound: i64,
    start: usize,
    weights: Vec<i64>,
}

fn overflow(operation: impl Into<String>) -> ReductionError {
    ReductionError::integer_overflow::<ILP<i64, i64, Bounded>, ILP<bool>>(operation)
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
    type Source = ILP<i64, i64, Bounded>;
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

// If all finite endpoints and row entries have magnitude below 2^h, widths
// need at most h+1 bits. Encoded coefficients are below 2^(2h+1), and the
// lower-bound shift gives |b'| < 2^h + n*2^(2h) < 2^(2h+n+1).
#[reduction(
    transform = {
        exact {
            num_constraints = "num_constraints",
        },
        upper_bound {
            num_vars = "num_vars * (max_constraint_magnitude_bits + 1)",
            num_nonzeros = "num_nonzeros * (max_constraint_magnitude_bits + 1)",
            max_constraint_magnitude_bits = "2 * max_constraint_magnitude_bits + num_vars + 1",
        },
    },
)]
impl ReduceTo<ILP<bool>> for ILP<i64, i64, Bounded> {
    type Result = ReductionIntILPToBinaryILP;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let mut encodings = Vec::with_capacity(self.num_vars());
        let mut num_binary_variables = 0_usize;
        for variable in self.variables() {
            let lower_bound = variable.lower_bound().expect("bounded ILP lower bound");
            let upper_bound = variable.upper_bound().expect("bounded ILP upper bound");
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
                    .checked_mul(weight)
                    .ok_or_else(|| overflow("encoding an integer ILP objective coefficient"))?;
                objective.push((encoding.start + offset, encoded_coefficient));
            }
        }

        Ok(ReductionIntILPToBinaryILP {
            target: ILP::<bool>::new(num_binary_variables, constraints, objective, self.sense())
                .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?,
            encodings,
        })
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_i64_ilp_bool.rs"]
mod tests;
