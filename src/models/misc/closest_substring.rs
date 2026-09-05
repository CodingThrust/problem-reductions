//! Closest Substring problem implementation.
//!
//! Given a finite alphabet `{0, ..., alphabet_size - 1}`, a list of input
//! strings (not necessarily of equal length), and a substring length `ell`,
//! find a center string `c` of length `ell` and one length-`ell` window per
//! input string that together minimize the maximum Hamming distance from `c`
//! to any selected window.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestSubstring",
        display_name: "Closest Substring",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Find a center string of fixed length and one length-ell window per input string that minimize the maximum Hamming distance between the center and any selected window",
        fields: &[
            FieldInfo {
                name: "alphabet_size",
                type_name: "usize",
                description: "Size q of the finite alphabet {0, ..., q-1}",
            },
            FieldInfo {
                name: "strings",
                type_name: "Vec<Vec<usize>>",
                description: "Input strings s_1, ..., s_n over the alphabet (possibly of different lengths)",
            },
            FieldInfo {
                name: "substring_length",
                type_name: "usize",
                description: "Common window length ell; every input string must have length at least substring_length",
            },
        ],
    }
}

/// The Closest Substring problem.
///
/// Given a finite alphabet `Sigma = {0, ..., q - 1}`, `n` input strings
/// `s_1, ..., s_n` over `Sigma` (not necessarily of equal length), and a
/// window length `ell` with `ell <= |s_i|` for every `i`, find a center
/// `c in Sigma^ell` and per-string window start positions `p_i in {0, ..., W_i - 1}`
/// (where `W_i = |s_i| - ell + 1`) minimizing
///
/// `max_{1 <= i <= n} d_H(c, s_i[p_i .. p_i + ell))`,
///
/// where `d_H` is the Hamming distance. Every choice in the discrete cube is
/// syntactically feasible.
#[derive(Debug, Clone, Serialize)]
pub struct ClosestSubstring {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    substring_length: usize,
}

#[derive(Deserialize)]
struct ClosestSubstringData {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    substring_length: usize,
}

impl<'de> Deserialize<'de> for ClosestSubstring {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = ClosestSubstringData::deserialize(deserializer)?;
        Self::new(data.alphabet_size, data.strings, data.substring_length)
            .map_err(serde::de::Error::custom)
    }
}

impl ClosestSubstring {
    /// Create a new `ClosestSubstring` instance.
    ///
    pub fn new(
        alphabet_size: usize,
        strings: Vec<Vec<usize>>,
        substring_length: usize,
    ) -> Result<Self, crate::registry::ConstructionError> {
        if strings.is_empty() {
            return Err("ClosestSubstring requires at least one input string".into());
        }
        if strings.iter().any(|s| s.len() < substring_length) {
            return Err("substring_length must be <= |s_i| for every input string".into());
        }
        if alphabet_size == 0 && substring_length > 0 {
            return Err("alphabet_size must be > 0 when substring_length > 0".into());
        }
        if strings
            .iter()
            .flat_map(|s| s.iter())
            .any(|&symbol| symbol >= alphabet_size)
        {
            return Err("input symbols must be less than alphabet_size".into());
        }
        substring_length
            .checked_add(strings.len())
            .ok_or("configuration length exceeds usize")?;
        strings
            .iter()
            .try_fold(0_usize, |total, string| total.checked_add(string.len()))
            .ok_or("total input length exceeds usize")?;
        strings
            .iter()
            .map(|string| string.len() - substring_length + 1)
            .try_fold(0_usize, usize::checked_add)
            .ok_or("total number of windows exceeds usize")?;
        strings
            .iter()
            .map(|string| string.len() - substring_length + 1)
            .try_fold(1_usize, usize::checked_mul)
            .ok_or("window-choice count exceeds usize")?;
        Ok(Self {
            alphabet_size,
            strings,
            substring_length,
        })
    }

    /// Returns the alphabet size `q`.
    pub fn alphabet_size(&self) -> usize {
        self.alphabet_size
    }

    /// Returns the input strings.
    pub fn strings(&self) -> &[Vec<usize>] {
        &self.strings
    }

    /// Returns the number of input strings `n`.
    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }

    /// Returns the common window length `ell`.
    pub fn substring_length(&self) -> usize {
        self.substring_length
    }

    /// Returns the sum of input string lengths.
    pub fn total_length(&self) -> usize {
        self.strings.iter().map(|s| s.len()).sum()
    }

    /// Returns `sum_i W_i`, where `W_i = |s_i| - substring_length + 1`.
    pub fn total_num_windows(&self) -> usize {
        self.strings
            .iter()
            .map(|s| s.len() - self.substring_length + 1)
            .sum()
    }

    /// Returns `prod_i W_i`, the number of distinct window-selection tuples.
    ///
    pub fn num_window_choice_product(&self) -> usize {
        self.strings
            .iter()
            .map(|s| s.len() - self.substring_length + 1)
            .try_fold(1usize, usize::checked_mul)
            .expect("validated window-choice count must fit usize")
    }
}

impl Problem for ClosestSubstring {
    const NAME: &'static str = "ClosestSubstring";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_parameters![
        ("alphabet_size", alphabet_size),
        ("num_strings", num_strings),
        ("substring_length", substring_length),
        ("total_length", total_length),
        ("total_num_windows", total_num_windows),
        ("num_window_choice_product", num_window_choice_product),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            let ell = self.substring_length;
            let n = self.num_strings();
            if config.len() != ell + n {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "substring witness length does not match the instance".into(),
                ));
            }
            let (center, window_starts) = config.split_at(ell);
            if center.iter().any(|&symbol| symbol >= self.alphabet_size) {
                return Ok(Min(None));
            }
            for (i, &start) in window_starts.iter().enumerate() {
                let w_i = self.strings[i].len() - ell + 1;
                if start >= w_i {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "substring witness contains an out-of-range window start".into(),
                    ));
                }
            }
            // Maximum Hamming distance from the center to the chosen window of each string.
            let mut max_distance = 0_i64;
            for (i, &start) in window_starts.iter().enumerate() {
                let window = &self.strings[i][start..start + ell];
                let distance = i64::try_from(
                    center
                        .iter()
                        .zip(window.iter())
                        .filter(|(center_symbol, target_symbol)| center_symbol != target_symbol)
                        .count(),
                )
                .map_err(|_| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "converting substring Hamming distance to i64".into(),
                    )
                })?;
                max_distance = max_distance.max(distance);
            }
            Min(Some(max_distance))
        })
    }
}

impl crate::solvers::BruteForceProblem for ClosestSubstring {
    fn dimensions(&self) -> Vec<usize> {
        let ell = self.substring_length;
        let mut dims = vec![self.alphabet_size; ell];
        dims.extend(self.strings.iter().map(|s| s.len() - ell + 1));
        dims
    }
}

crate::declare_variants! {
    default ClosestSubstring => "alphabet_size ^ substring_length * num_window_choice_product",
}

crate::register_brute_force! {
    ClosestSubstring,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "closest_substring",
        instance: Box::new(
            ClosestSubstring::new(
                2,
                vec![
                    vec![0, 0, 0, 1, 1],
                    vec![1, 0, 1, 0, 0],
                    vec![1, 1, 0, 0, 1],
                ],
                3,
            )
            .unwrap(),
        ),
        // Center c = [0, 1, 0]; windows (0, 1, 0) selecting s_1[0..3] = 000,
        // s_2[1..4] = 010, s_3[0..3] = 110 with distances 1, 0, 1 and radius 1.
        optimal_config: serde_json::json!(vec![0, 1, 0, 0, 1, 0]),
        optimal_value: serde_json::json!(1),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/closest_substring.rs"]
mod tests;
