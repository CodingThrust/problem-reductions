//! Reduction from `ILP<i64>` to `ILP<bool>` via truncated binary encoding with FBBT.
//!
//! Uses Feasibility-Based Bound Tightening (Savelsbergh 1994, Achterberg et al. 2020)
//! to infer per-variable upper bounds, then encodes each integer variable into
//! ceil(log2(U+1)) binary variables using truncated binary encoding (Karimi & Rosenberg 2017).

use crate::models::algebraic::{Comparison, LinearConstraint, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::{i64_to_exact_f64, MAX_EXACT_F64_INTEGER};

/// Error type for FBBT failures.
#[derive(Debug, Clone, PartialEq)]
pub enum FbbtError {
    /// At least one variable has an unbounded upper bound after FBBT.
    Unbounded,
    /// The constraint system is provably infeasible.
    Infeasible,
    /// Integer arithmetic or an integer-valued coefficient is outside the i64 domain.
    Arithmetic(String),
    /// A coefficient required by this integer encoding is not a finite i64-valued integer.
    InvalidInteger(String),
}

fn integer_coefficient(value: f64, context: &str) -> Result<i64, FbbtError> {
    let exact_limit = MAX_EXACT_F64_INTEGER as f64;
    if !value.is_finite() || value.fract() != 0.0 || value < -exact_limit || value > exact_limit {
        return Err(FbbtError::InvalidInteger(format!(
            "{context} must be a finite integer representable as i64"
        )));
    }
    Ok(value as i64)
}

fn checked_mul(left: i64, right: i64, context: &str) -> Result<i64, FbbtError> {
    left.checked_mul(right)
        .ok_or_else(|| FbbtError::Arithmetic(context.to_string()))
}

fn checked_add(left: i64, right: i64, context: &str) -> Result<i64, FbbtError> {
    left.checked_add(right)
        .ok_or_else(|| FbbtError::Arithmetic(context.to_string()))
}

fn checked_sub(left: i64, right: i64, context: &str) -> Result<i64, FbbtError> {
    left.checked_sub(right)
        .ok_or_else(|| FbbtError::Arithmetic(context.to_string()))
}

/// Per-variable encoding info: start index in binary variables, weights.
#[derive(Debug, Clone)]
struct VarEncoding {
    /// Index of the first binary variable for this integer variable.
    start: usize,
    /// Weights for each binary variable: [1, 2, 4, ..., remainder].
    weights: Vec<i64>,
}

/// Infer upper bounds for non-negative integer variables via FBBT.
///
/// Returns `Ok(bounds)` with finite upper bounds, or an error if the system
/// is infeasible or unbounded.
fn fbbt(num_vars: usize, constraints: &[LinearConstraint]) -> Result<Vec<i64>, FbbtError> {
    const INF: i64 = i64::MAX / 2; // sentinel for +infinity (safe for addition)

    let mut lower = vec![0i64; num_vars];
    let mut upper = vec![INF; num_vars];

    let max_iters = num_vars + 1;

    for _ in 0..max_iters {
        let mut changed = false;

        for c in constraints {
            // Compute activity bounds: act_min = sum of min contributions, act_max = sum of max contributions
            let mut act_min: i64 = 0;
            let mut act_max: i64 = 0;
            let mut act_min_finite = true;
            let mut act_max_finite = true;

            for &(var, coef) in &c.terms {
                let coef_i = integer_coefficient(coef, "FBBT coefficient")?;
                if coef_i > 0 {
                    act_min = checked_add(
                        act_min,
                        checked_mul(coef_i, lower[var], "multiplying FBBT lower activity")?,
                        "summing FBBT lower activity",
                    )?;
                    if upper[var] >= INF {
                        act_max_finite = false;
                    } else {
                        act_max = checked_add(
                            act_max,
                            checked_mul(coef_i, upper[var], "multiplying FBBT upper activity")?,
                            "summing FBBT upper activity",
                        )?;
                    }
                } else if coef_i < 0 {
                    if upper[var] >= INF {
                        act_min_finite = false;
                    } else {
                        act_min = checked_add(
                            act_min,
                            checked_mul(coef_i, upper[var], "multiplying FBBT lower activity")?,
                            "summing FBBT lower activity",
                        )?;
                    }
                    act_max = checked_add(
                        act_max,
                        checked_mul(coef_i, lower[var], "multiplying FBBT upper activity")?,
                        "summing FBBT upper activity",
                    )?;
                }
            }

            let rhs = integer_coefficient(c.rhs, "FBBT right-hand side")?;

            // Infeasibility checks
            if matches!(c.cmp, Comparison::Le | Comparison::Eq) && act_min_finite && act_min > rhs {
                return Err(FbbtError::Infeasible);
            }
            if matches!(c.cmp, Comparison::Ge | Comparison::Eq) && act_max_finite && act_max < rhs {
                return Err(FbbtError::Infeasible);
            }

            // Tighten each variable
            for &(var, coef) in &c.terms {
                let coef_i = integer_coefficient(coef, "FBBT coefficient")?;
                if coef_i == 0 {
                    continue;
                }

                // From Le or Eq: upper bound tightening for positive coef, lower bound for negative
                if matches!(c.cmp, Comparison::Le | Comparison::Eq) {
                    // Compute residual min = act_min - this variable's min contribution
                    let my_min = if coef_i > 0 {
                        checked_mul(coef_i, lower[var], "multiplying FBBT minimum contribution")?
                    } else {
                        if upper[var] >= INF {
                            continue; // can't compute residual
                        }
                        checked_mul(coef_i, upper[var], "multiplying FBBT minimum contribution")?
                    };
                    if !(act_min_finite || coef_i < 0 && upper[var] >= INF) {
                        // act_min is -inf, residual is -inf, no useful bound
                        continue;
                    }
                    let res_min = if act_min_finite {
                        checked_sub(act_min, my_min, "computing FBBT minimum residual")?
                    } else {
                        // act_min was -inf because of this var's contribution
                        // but my_min was the infinite part, so residual is finite
                        // This case shouldn't produce useful bounds
                        continue;
                    };

                    if coef_i > 0 {
                        // a_i * x_i <= rhs - res_min => x_i <= floor((rhs - res_min) / a_i)
                        let new_u = floor_div(
                            checked_sub(rhs, res_min, "computing FBBT upper-bound numerator")?,
                            coef_i,
                        )?;
                        if new_u < upper[var] {
                            upper[var] = new_u;
                            changed = true;
                        }
                    } else {
                        // a_i * x_i <= rhs - res_min, a_i < 0 => x_i >= ceil((rhs - res_min) / a_i)
                        let new_l = ceil_div(
                            checked_sub(rhs, res_min, "computing FBBT lower-bound numerator")?,
                            coef_i,
                        )?;
                        if new_l > lower[var] {
                            lower[var] = new_l;
                            changed = true;
                        }
                    }
                }

                // From Ge or Eq: lower bound tightening for positive coef, upper for negative
                if matches!(c.cmp, Comparison::Ge | Comparison::Eq) {
                    let my_max = if coef_i > 0 {
                        if upper[var] >= INF {
                            continue;
                        }
                        checked_mul(coef_i, upper[var], "multiplying FBBT maximum contribution")?
                    } else {
                        checked_mul(coef_i, lower[var], "multiplying FBBT maximum contribution")?
                    };
                    if !(act_max_finite || coef_i > 0 && upper[var] >= INF) {
                        continue;
                    }
                    let res_max = if act_max_finite {
                        checked_sub(act_max, my_max, "computing FBBT maximum residual")?
                    } else {
                        continue;
                    };

                    if coef_i > 0 {
                        // a_i * x_i >= rhs - res_max => x_i >= ceil((rhs - res_max) / a_i)
                        let new_l = ceil_div(
                            checked_sub(rhs, res_max, "computing FBBT lower-bound numerator")?,
                            coef_i,
                        )?;
                        if new_l > lower[var] {
                            lower[var] = new_l;
                            changed = true;
                        }
                    } else {
                        // a_i * x_i >= rhs - res_max, a_i < 0 => x_i <= floor((rhs - res_max) / a_i)
                        let new_u = floor_div(
                            checked_sub(rhs, res_max, "computing FBBT upper-bound numerator")?,
                            coef_i,
                        )?;
                        if new_u < upper[var] {
                            upper[var] = new_u;
                            changed = true;
                        }
                    }
                }

                if lower[var] > upper[var] {
                    return Err(FbbtError::Infeasible);
                }
            }
        }

        if !changed {
            break;
        }
    }

    // Check for unbounded variables
    for &u in &upper {
        if u >= INF {
            return Err(FbbtError::Unbounded);
        }
    }

    Ok(upper)
}

/// Floor division that rounds toward negative infinity.
fn floor_div(a: i64, b: i64) -> Result<i64, FbbtError> {
    let d = a
        .checked_div(b)
        .ok_or_else(|| FbbtError::Arithmetic("dividing an FBBT upper-bound numerator".into()))?;
    let r = a
        .checked_rem(b)
        .ok_or_else(|| FbbtError::Arithmetic("taking an FBBT upper-bound remainder".into()))?;
    if (r != 0) && ((r ^ b) < 0) {
        d.checked_sub(1)
            .ok_or_else(|| FbbtError::Arithmetic("rounding an FBBT upper bound".into()))
    } else {
        Ok(d)
    }
}

/// Ceiling division that rounds toward positive infinity.
fn ceil_div(a: i64, b: i64) -> Result<i64, FbbtError> {
    let d = a
        .checked_div(b)
        .ok_or_else(|| FbbtError::Arithmetic("dividing an FBBT lower-bound numerator".into()))?;
    let r = a
        .checked_rem(b)
        .ok_or_else(|| FbbtError::Arithmetic("taking an FBBT lower-bound remainder".into()))?;
    if (r != 0) && ((r ^ b) >= 0) {
        d.checked_add(1)
            .ok_or_else(|| FbbtError::Arithmetic("rounding an FBBT lower bound".into()))
    } else {
        Ok(d)
    }
}

/// Compute the truncated binary encoding weights for a variable with upper bound U.
///
/// Returns weights [1, 2, 4, ..., remainder] such that sum of weights = U.
fn binary_weights(upper_bound: i64) -> Vec<i64> {
    if upper_bound == 0 {
        return vec![]; // fixed at 0, no binary variables needed
    }
    let k = num_bits(upper_bound);
    let mut weights = Vec::with_capacity(k);
    for j in 0..(k - 1) {
        weights.push(1i64 << j);
    }
    // Last weight: U - (2^{K-1} - 1)
    let last = upper_bound - ((1i64 << (k - 1)) - 1);
    weights.push(last);
    weights
}

/// Number of binary variables needed: ceil(log2(U + 1)).
fn num_bits(upper_bound: i64) -> usize {
    if upper_bound <= 0 {
        return 0;
    }
    // ceil(log2(U + 1)) = floor(log2(U)) + 1 = 64 - leading_zeros(U)
    64 - (upper_bound as u64).leading_zeros() as usize
}

/// Reduction result for `ILP<i64>` -> `ILP<bool>`.
#[derive(Debug, Clone)]
pub struct ReductionIntILPToBinaryILP {
    target: ILP<bool>,
    /// Per-source-variable encoding info.
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
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            self.encodings
                .iter()
                .map(|enc| {
                    let val =
                        enc.weights
                            .iter()
                            .enumerate()
                            .try_fold(0_i64, |total, (j, &weight)| {
                                let bit = i64::try_from(target_solution[enc.start + j]).map_err(
                                    |_| {
                                        crate::rules::ExtractionError::invalid(
                                            "binary ILP value cannot be represented as i64",
                                        )
                                    },
                                )?;
                                let term = weight.checked_mul(bit).ok_or_else(|| {
                                    crate::rules::ExtractionError::invalid(
                                        "binary ILP decoding multiplication overflowed i64",
                                    )
                                })?;
                                total.checked_add(term).ok_or_else(|| {
                                    crate::rules::ExtractionError::invalid(
                                        "binary ILP decoding sum overflowed i64",
                                    )
                                })
                            })?;
                    usize::try_from(val).map_err(|_| {
                        crate::rules::ExtractionError::invalid(
                            "decoded ILP value cannot be represented as usize",
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}

#[reduction(
    size = upper_bound {
        num_vars = "31 * num_vars",
        num_constraints = "num_constraints",
    },)]
impl ReduceTo<ILP<bool>> for ILP<i64> {
    type Result = ReductionIntILPToBinaryILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        if self.num_vars == 0 {
            return Ok(ReductionIntILPToBinaryILP {
                target: ILP::<bool>::new(0, vec![], vec![], self.sense),
                encodings: vec![],
            });
        }

        // Step 1: FBBT to infer upper bounds
        let upper_bounds = match fbbt(self.num_vars, &self.constraints) {
            Ok(bounds) => bounds,
            Err(FbbtError::Infeasible) => {
                // Return an infeasible `ILP<bool>`: 1 variable, constraint y0 >= 1 AND y0 <= 0
                return Ok(ReductionIntILPToBinaryILP {
                    target: ILP::<bool>::new(
                        1,
                        vec![
                            LinearConstraint::ge(vec![(0, 1.0)], 1.0),
                            LinearConstraint::le(vec![(0, 1.0)], 0.0),
                        ],
                        vec![],
                        self.sense,
                    ),
                    encodings: (0..self.num_vars)
                        .map(|_| VarEncoding {
                            start: 0,
                            weights: vec![],
                        })
                        .collect(),
                });
            }
            Err(FbbtError::Unbounded) => {
                return Err(crate::rules::ReductionError::invalid_target::<
                    ILP<i64>,
                    ILP<bool>,
                >(
                    "binary encoding requires finite upper bounds for every integer variable",
                ));
            }
            Err(FbbtError::Arithmetic(context)) => {
                return Err(crate::rules::ReductionError::integer_overflow::<
                    ILP<i64>,
                    ILP<bool>,
                >(context));
            }
            Err(FbbtError::InvalidInteger(message)) => {
                return Err(crate::rules::ReductionError::invalid_target::<
                    ILP<i64>,
                    ILP<bool>,
                >(message));
            }
        };

        // Step 2: Build encodings
        let mut encodings = Vec::with_capacity(self.num_vars);
        let mut total_bool_vars = 0;
        for &u in &upper_bounds {
            let weights = binary_weights(u);
            encodings.push(VarEncoding {
                start: total_bool_vars,
                weights: weights.clone(),
            });
            total_bool_vars += weights.len();
        }

        // Step 3: Transform constraints
        let mut constraints = Vec::with_capacity(self.constraints.len());
        for constraint in &self.constraints {
            let mut new_terms = Vec::new();
            for &(var, coefficient) in &constraint.terms {
                let encoding = &encodings[var];
                for (j, &weight) in encoding.weights.iter().enumerate() {
                    let weight =
                        i64_to_exact_f64(weight).map_err(|error| {
                            crate::rules::ReductionError::inexact_float_conversion::<
                                ILP<i64>,
                                ILP<bool>,
                            >(error)
                        })?;
                    let encoded_coefficient = coefficient * weight;
                    if !encoded_coefficient.is_finite() {
                        return Err(crate::rules::ReductionError::invalid_target::<
                            ILP<i64>,
                            ILP<bool>,
                        >(
                            "encoding an integer constraint produced a non-finite coefficient",
                        ));
                    }
                    new_terms.push((encoding.start + j, encoded_coefficient));
                }
            }
            constraints.push(LinearConstraint::new(
                new_terms,
                constraint.cmp,
                constraint.rhs,
            ));
        }

        // Step 4: Transform objective
        let mut new_objective = Vec::new();
        for &(var, coef) in &self.objective {
            let enc = &encodings[var];
            for (j, &w) in enc.weights.iter().enumerate() {
                let weight = i64_to_exact_f64(w).map_err(|error| {
                    crate::rules::ReductionError::inexact_float_conversion::<ILP<i64>, ILP<bool>>(
                        error,
                    )
                })?;
                let encoded_coefficient = coef * weight;
                if !encoded_coefficient.is_finite() {
                    return Err(crate::rules::ReductionError::invalid_target::<
                        ILP<i64>,
                        ILP<bool>,
                    >(
                        "encoding an integer objective produced a non-finite coefficient",
                    ));
                }
                new_objective.push((enc.start + j, encoded_coefficient));
            }
        }

        Ok(ReductionIntILPToBinaryILP {
            target: ILP::<bool>::new(total_bool_vars, constraints, new_objective, self.sense),
            encodings,
        })
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_i64_ilp_bool.rs"]
mod tests;
