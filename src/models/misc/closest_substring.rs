//! Closest Substring problem implementation.
//!
//! Given a finite alphabet `{0, ..., alphabet_size - 1}`, a list of input
//! strings (not necessarily of equal length), and a substring length `ell`,
//! find a center string `c` of length `ell` and one length-`ell` window per
//! input string that together minimize the maximum Hamming distance from `c`
//! to any selected window.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestSubstring",
        display_name: "Closest Substring",
        aliases: &[],
        dimensions: &[],
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

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "ClosestSubstring",
        fields: &[
            "alphabet_size",
            "num_strings",
            "substring_length",
            "total_length",
            "total_num_windows",
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosestSubstring {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    substring_length: usize,
}

impl ClosestSubstring {
    /// Create a new `ClosestSubstring` instance.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `strings` is empty (the problem requires at least one input string),
    /// - `substring_length > |s_i|` for any input string,
    /// - `alphabet_size == 0` while `substring_length > 0`,
    /// - any symbol in any input string is `>= alphabet_size`.
    pub fn new(alphabet_size: usize, strings: Vec<Vec<usize>>, substring_length: usize) -> Self {
        assert!(
            !strings.is_empty(),
            "ClosestSubstring requires at least one input string"
        );
        assert!(
            strings.iter().all(|s| s.len() >= substring_length),
            "substring_length must be <= |s_i| for every input string"
        );
        assert!(
            alphabet_size > 0 || substring_length == 0,
            "alphabet_size must be > 0 when substring_length > 0"
        );
        assert!(
            strings
                .iter()
                .flat_map(|s| s.iter())
                .all(|&symbol| symbol < alphabet_size),
            "input symbols must be less than alphabet_size"
        );
        Self {
            alphabet_size,
            strings,
            substring_length,
        }
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
    /// Uses saturating multiplication so the value cannot overflow; callers
    /// should treat a return of `usize::MAX` as "very large".
    pub fn num_window_choice_product(&self) -> usize {
        self.strings
            .iter()
            .map(|s| s.len() - self.substring_length + 1)
            .fold(1usize, |acc, w| acc.saturating_mul(w))
    }
}

impl Problem for ClosestSubstring {
    const NAME: &'static str = "ClosestSubstring";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let ell = self.substring_length;
        let mut dims = vec![self.alphabet_size; ell];
        dims.extend(self.strings.iter().map(|s| s.len() - ell + 1));
        dims
    }

    fn evaluate(&self, config: &[usize]) -> Min<i64> {
        let ell = self.substring_length;
        let n = self.num_strings();
        if config.len() != ell + n {
            return Min(None);
        }
        let (center, window_starts) = config.split_at(ell);
        if center.iter().any(|&symbol| symbol >= self.alphabet_size) {
            return Min(None);
        }
        for (i, &start) in window_starts.iter().enumerate() {
            let w_i = self.strings[i].len() - ell + 1;
            if start >= w_i {
                return Min(None);
            }
        }
        // Maximum Hamming distance from the center to the chosen window of each string.
        let max_distance = window_starts
            .iter()
            .enumerate()
            .map(|(i, &start)| {
                let window = &self.strings[i][start..start + ell];
                center
                    .iter()
                    .zip(window.iter())
                    .filter(|(c, t)| c != t)
                    .count() as i64
            })
            .max()
            .unwrap_or(0);
        Min(Some(max_distance))
    }
}

crate::declare_variants! {
    default ClosestSubstring => "alphabet_size ^ substring_length * num_window_choice_product",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "closest_substring",
        instance: Box::new(ClosestSubstring::new(
            2,
            vec![
                vec![0, 0, 0, 1, 1],
                vec![1, 0, 1, 0, 0],
                vec![1, 1, 0, 0, 1],
            ],
            3,
        )),
        // Center c = [0, 1, 0]; windows (0, 1, 0) selecting s_1[0..3] = 000,
        // s_2[1..4] = 010, s_3[0..3] = 110 with distances 1, 0, 1 and radius 1.
        optimal_config: vec![0, 1, 0, 0, 1, 0],
        optimal_value: serde_json::json!(1),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/closest_substring.rs"]
mod tests;
