//! Reduction from ClosestSubstring to ILP (Integer Linear Programming).
//!
//! Given an alphabet of size `q`, `n` input strings `s_1, ..., s_n` (not
//! necessarily of equal length), and a window length `ell`, the goal is to
//! pick a center `c in Sigma^ell` and one length-`ell` window from each input
//! string that together minimize the worst-case Hamming distance between the
//! center and any chosen window. The ILP encoding combines the
//! center-selection variables of ClosestString with one-hot window-choice
//! indicators, plus a radius variable that is active only on each selected
//! window.
//!
//! - Integer `x_{r, a}` for `r in {0, ..., ell - 1}` and
//!   `a in {0, ..., q - 1}`: `x_{r, a} = 1` iff the center has symbol `a` at
//!   position `r`. The non-negativity of ILP variables together with the
//!   assignment constraint forces every `x_{r, a} in {0, 1}`.
//! - Integer `y_{i, p}` for input string `s_i` and window start
//!   `p in {0, ..., W_i - 1}` where `W_i = |s_i| - ell + 1`: `y_{i, p} = 1` iff
//!   window `p` is selected from string `s_i`.
//! - Nonnegative integer radius variable `R`.
//! - Assignment constraint: `sum_a x_{r, a} = 1` for every position `r`.
//! - Window-choice constraint: `sum_p y_{i, p} = 1` for every input string.
//! - Conditional radius constraint per `(i, p)`:
//!   `R + sum_{r} x_{r, s_i[p + r]} - ell * y_{i, p} >= 0`.
//!   When `y_{i, p} = 1`, this becomes `R >= d_H(c, s_i[p..p + ell))`; when
//!   `y_{i, p} = 0`, the constraint is automatically satisfied.
//! - Objective: minimize `R`.
//!
//! Reference: Ming Li, Bin Ma, and Lusheng Wang, "On the closest string and
//! substring problems," Journal of the ACM 49(2):157-171, 2002.
//! <https://doi.org/10.1145/506147.506150>

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ClosestSubstring;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ClosestSubstring to ILP.
///
/// Variable layout (`ILP<i64>`, all non-negative):
/// - `x_{r, a}` at index `r * alphabet_size + a` for `r in [0, ell)` and
///   `a in [0, q)`, forced into `{0, 1}` by the assignment constraints.
/// - `y_{i, p}` at index `q * ell + window_offsets[i] + p` for input string
///   `s_i` and window start `p in [0, W_i)`, forced into `{0, 1}` by the
///   window-choice constraints.
/// - `R` (radius) at index `q * ell + total_num_windows`, a non-negative
///   integer in `[0, ell]`.
#[derive(Debug, Clone)]
pub struct ReductionClosestSubstringToILP {
    target: ILP<i64>,
    alphabet_size: usize,
    substring_length: usize,
    /// Prefix sums of per-string window counts: `window_offsets[i]` is the
    /// number of `y_{j, p}` variables for `j < i`. Has length `num_strings`.
    window_offsets: Vec<usize>,
    /// `window_counts[i] = W_i = |s_i| - ell + 1`.
    window_counts: Vec<usize>,
}

impl ReductionResult for ReductionClosestSubstringToILP {
    type Source = ClosestSubstring;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    /// Decode the integer ILP assignment into the source config layout.
    ///
    /// `ClosestSubstring::evaluate` expects `config` of length `ell + n`: the
    /// first `ell` entries are the center symbols, the remaining `n` entries
    /// are per-string window starts. For each center position `r`, we pick the
    /// unique alphabet symbol `a` with `x_{r, a} = 1`; for each input string
    /// `s_i`, we pick the unique window start `p` with `y_{i, p} = 1`.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let q = self.alphabet_size;
        let ell = self.substring_length;
        let y_base = q * ell;
        let mut out = Vec::with_capacity(ell + self.window_counts.len());

        for position in 0..ell {
            let block = &target_solution[position * q..(position + 1) * q];
            out.push(decode_one_hot(block, "center position", position)?);
        }
        for (string, &window_count) in self.window_counts.iter().enumerate() {
            let start = y_base + self.window_offsets[string];
            out.push(decode_one_hot(
                &target_solution[start..start + window_count],
                "string window",
                string,
            )?);
        }

        Ok(out)
    }
}

fn decode_one_hot(
    block: &[i64],
    block_name: &str,
    block_index: usize,
) -> crate::rules::ExtractionResult<usize> {
    let mut selected = block.iter().enumerate().filter(|(_, value)| **value == 1);
    let index = selected.next().map(|(index, _)| index).ok_or_else(|| {
        crate::rules::ExtractionError::invalid(format!(
            "{block_name} {block_index} has no selected value"
        ))
    })?;
    if selected.next().is_some() || block.iter().any(|&value| value > 1) {
        return Err(crate::rules::ExtractionError::invalid(format!(
            "{block_name} {block_index} is not one-hot"
        )));
    }
    Ok(index)
}

#[reduction(
    transform = exact {
        num_vars = "alphabet_size * substring_length + total_num_windows + 1",
        num_constraints = "substring_length + num_strings + total_num_windows + 1",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for ClosestSubstring {
    type Result = ReductionClosestSubstringToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let q = self.alphabet_size();
        let ell = self.substring_length();
        let strings = self.strings();
        let n = strings.len();

        let window_counts: Vec<usize> = strings.iter().map(|s| s.len() - ell + 1).collect();
        let mut window_offsets: Vec<usize> = Vec::with_capacity(n);
        {
            let mut acc = 0usize;
            for &w in &window_counts {
                window_offsets.push(acc);
                acc += w;
            }
        }
        let total_windows: usize = window_counts.iter().sum();

        let x_idx = |r: usize, a: usize| -> usize { r * q + a };
        let y_base = q * ell;
        let y_idx = |i: usize, p: usize| -> usize { y_base + window_offsets[i] + p };
        let r_idx = y_base + total_windows;
        let num_vars = r_idx + 1;
        let ell_i64 = Self::exact_i64(ell, "encoding the substring length")?;

        let mut constraints: Vec<LinearConstraint> =
            Vec::with_capacity(ell + n + total_windows + 1);

        // Assignment constraints: exactly one symbol per center position.
        // Together with the non-negativity built into `ILP<i64>`, this also
        // forces every x_{r, a} to lie in {0, 1}.
        for r in 0..ell {
            let terms: Vec<(usize, i64)> = (0..q).map(|a| (x_idx(r, a), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Tight upper bound on R: the worst-case Hamming distance over a
        // length-ell window is at most ell. Added as a single-term `<=`
        // constraint so the solver's bound-tightening pass (which scans for
        // exactly this pattern) picks it up. Without this, R defaults to the
        // full i64 domain, which severely degrades HiGHS performance even on
        // tiny instances.
        constraints.push(LinearConstraint::le(vec![(r_idx, 1)], ell_i64));

        // Window-choice constraints: exactly one window per input string.
        // Combined with non-negativity, this forces every y_{i, p} in {0, 1}.
        for (i, &w_i) in window_counts.iter().enumerate() {
            let terms: Vec<(usize, i64)> = (0..w_i).map(|p| (y_idx(i, p), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Conditional radius constraints: for every (input string, window
        // start) pair, R + sum_r x_{r, s_i[p + r]} - ell * y_{i, p} >= 0.
        // - If y_{i, p} = 1: R >= ell - sum_r x_{r, s_i[p + r]} = d_H(c, window).
        // - If y_{i, p} = 0: the LHS is R + (nonneg match count) >= 0,
        //   automatically satisfied because R >= 0.
        for (i, s) in strings.iter().enumerate() {
            for p in 0..window_counts[i] {
                let mut terms: Vec<(usize, i64)> = Vec::with_capacity(ell + 2);
                terms.push((r_idx, 1));
                for r in 0..ell {
                    terms.push((x_idx(r, s[p + r]), 1));
                }
                terms.push((y_idx(i, p), -ell_i64));
                constraints.push(LinearConstraint::ge(terms, 0));
            }
        }

        // Objective: minimize R.
        let objective = vec![(r_idx, 1)];

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionClosestSubstringToILP {
            target,
            alphabet_size: q,
            substring_length: ell,
            window_offsets,
            window_counts,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "closestsubstring_to_ilp",
        build: || {
            // Canonical issue #1033 instance: binary alphabet, length-3
            // windows on three length-5 strings. Optimum radius is 1; one
            // optimal center is 010 with windows (0, 1, 0) selecting 000,
            // 010, 110 from s_1, s_2, s_3 respectively.
            let source = ClosestSubstring::new(
                2,
                vec![
                    vec![0, 0, 0, 1, 1],
                    vec![1, 0, 1, 0, 0],
                    vec![1, 1, 0, 0, 1],
                ],
                3,
            )
            .unwrap();
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/closestsubstring_ilp.rs"]
mod tests;
