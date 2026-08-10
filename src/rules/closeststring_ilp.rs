//! Reduction from ClosestString to ILP (Integer Linear Programming).
//!
//! Given an alphabet of size `q`, `n` input strings of common length `m`, and
//! the goal of finding a center `c in Sigma^m` that minimizes the maximum
//! Hamming distance to every input string, the natural encoding picks one
//! alphabet symbol at every center position and constrains a radius variable
//! to upper-bound every per-string Hamming distance:
//!
//! - Binary `x_{j, a} in {0, 1}` for `j in {0, ..., m - 1}` and `a in
//!   {0, ..., q - 1}`: `x_{j, a} = 1` iff the chosen center has symbol `a` at
//!   position `j`.
//! - Nonnegative integer radius variable `R`.
//! - Assignment constraint: `sum_a x_{j, a} = 1` for every position `j`.
//!   Because every ILP variable is a nonnegative integer, this also forces
//!   each `x_{j, a} in {0, 1}`.
//! - Radius constraint per input string `s_i`:
//!   `R + sum_j x_{j, s_i[j]} >= m`, which is equivalent to `R >= d_H(c, s_i)`.
//! - Objective: minimize `R`.
//!
//! Reference: Ming Li, Bin Ma, and Lusheng Wang, "On the closest string and
//! substring problems," Journal of the ACM 49(2):157-171, 2002.
//! <https://doi.org/10.1145/506147.506150>

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ClosestString;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ClosestString to ILP.
///
/// Variable layout (`ILP<i32>`, all non-negative):
/// - `x_{j, a}` at index `j * alphabet_size + a` for `j in [0, m)` and
///   `a in [0, q)`, bounded to `{0, 1}`.
/// - `R` (radius) at index `m * q`, an integer in `[0, m]`.
#[derive(Debug, Clone)]
pub struct ReductionClosestStringToILP {
    target: ILP<i32>,
    alphabet_size: usize,
    string_length: usize,
}

impl ReductionResult for ReductionClosestStringToILP {
    type Source = ClosestString;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Decode the integer ILP assignment into the source center config.
    ///
    /// For every position `j`, choose the unique alphabet symbol `a` with
    /// `x_{j, a} = 1`.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let q = self.alphabet_size;
        let mut center = Vec::with_capacity(self.string_length);
        for position in 0..self.string_length {
            let block = &target_solution[position * q..(position + 1) * q];
            let mut selected = block.iter().enumerate().filter(|(_, value)| **value == 1);
            let symbol = selected.next().map(|(symbol, _)| symbol).ok_or_else(|| {
                crate::rules::ExtractionError::invalid(format!(
                    "center position {position} has no selected symbol"
                ))
            })?;
            if selected.next().is_some() || block.iter().any(|&value| value > 1) {
                return Err(crate::rules::ExtractionError::invalid(format!(
                    "center position {position} is not one-hot"
                )));
            }
            center.push(symbol);
        }
        Ok(center)
    }
}

#[reduction(
    exact = {
        num_vars = "alphabet_size * string_length + 1",
        num_constraints = "string_length + num_strings",
    },
    unavailable = {
        coefficient_encoding_bits = "the source size vector omits coefficient magnitudes and sparsity needed to bound the encoded coefficients",
    }
)]
impl ReduceTo<ILP<i32>> for ClosestString {
    type Result = ReductionClosestStringToILP;

    fn reduce_to(&self) -> Self::Result {
        let q = self.alphabet_size();
        let m = self.string_length();
        let strings = self.strings();
        let n = strings.len();

        let x_idx = |j: usize, a: usize| -> usize { j * q + a };
        let r_idx = q * m;
        let num_vars = q * m + 1;

        let mut constraints: Vec<LinearConstraint> = Vec::with_capacity(m + n);

        // Assignment constraints: exactly one symbol per center position.
        // Together with the non-negativity built into ILP<i32>, this also
        // forces every x_{j, a} to lie in {0, 1}.
        for j in 0..m {
            let terms: Vec<(usize, f64)> = (0..q).map(|a| (x_idx(j, a), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Radius constraints: R + sum_j x_{j, s_i[j]} >= m.
        // Equivalently, R >= m - sum_j x_{j, s_i[j]} = d_H(c, s_i).
        for s in strings.iter() {
            let mut terms: Vec<(usize, f64)> = Vec::with_capacity(m + 1);
            terms.push((r_idx, 1.0));
            for (j, &symbol) in s.iter().enumerate() {
                terms.push((x_idx(j, symbol), 1.0));
            }
            constraints.push(LinearConstraint::ge(terms, m as f64));
        }

        // Objective: minimize R.
        let objective = vec![(r_idx, 1.0)];

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionClosestStringToILP {
            target,
            alphabet_size: q,
            string_length: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "closeststring_to_ilp",
        build: || {
            // Canonical issue #1032 instance: binary alphabet, the four length-3
            // strings 000, 011, 101, 110. Optimum radius is 2 (achieved by any
            // binary length-3 center, e.g. 000).
            let source = ClosestString::new(
                2,
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/closeststring_ilp.rs"]
mod tests;
