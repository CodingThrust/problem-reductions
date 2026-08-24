//! Closest String problem implementation.
//!
//! Given a finite alphabet `{0, ..., alphabet_size - 1}` and a list of input
//! strings of equal length `m`, find a center string `c` of length `m` over the
//! same alphabet that minimizes the maximum Hamming distance from `c` to any
//! input string.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ClosestString",
        display_name: "Closest String",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Find a center string of fixed length that minimizes the maximum Hamming distance to a list of equal-length input strings",
        fields: &[
            FieldInfo {
                name: "alphabet_size",
                type_name: "usize",
                description: "Size q of the finite alphabet {0, ..., q-1}",
            },
            FieldInfo {
                name: "strings",
                type_name: "Vec<Vec<usize>>",
                description: "Input strings s_1, ..., s_n over the alphabet, all of equal length m",
            },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "ClosestString",
        fields: &["alphabet_size", "num_strings", "string_length", "total_length"],
    }
}

/// The Closest String problem.
///
/// Given a finite alphabet `Sigma = {0, ..., q - 1}` and `n` input strings
/// `s_1, ..., s_n` in `Sigma^m` (all of common length `m`), find a center
/// string `c` in `Sigma^m` minimizing
///
/// `max_{1 <= i <= n} d_H(c, s_i)`,
///
/// where `d_H` is the Hamming distance. Every center in the discrete cube is
/// syntactically feasible; the objective is its worst-case Hamming distance
/// to the input strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosestString {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
}

impl ClosestString {
    /// Create a new `ClosestString` instance.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `strings` is empty (the problem requires at least one input string),
    /// - input strings do not all have the same length,
    /// - `alphabet_size == 0` while any input string is non-empty,
    /// - any symbol in any input string is `>= alphabet_size`.
    pub fn new(alphabet_size: usize, strings: Vec<Vec<usize>>) -> Self {
        assert!(
            !strings.is_empty(),
            "ClosestString requires at least one input string"
        );
        let string_length = strings[0].len();
        assert!(
            strings.iter().all(|s| s.len() == string_length),
            "all input strings must have the same length"
        );
        assert!(
            alphabet_size > 0 || string_length == 0,
            "alphabet_size must be > 0 when input strings are non-empty"
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

    /// Returns the common string length `m`.
    pub fn string_length(&self) -> usize {
        self.strings[0].len()
    }

    /// Returns the total input length `num_strings * string_length`.
    pub fn total_length(&self) -> usize {
        self.num_strings() * self.string_length()
    }
}

impl Problem for ClosestString {
    const NAME: &'static str = "ClosestString";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.alphabet_size; self.string_length()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            let m = self.string_length();
            if config.len() != m {
                return Ok(Min(None));
            }
            if config.iter().any(|&symbol| symbol >= self.alphabet_size) {
                return Ok(Min(None));
            }
            // Maximum Hamming distance from the center to any input string.
            let mut max_distance = 0_i64;
            for string in &self.strings {
                let distance = i64::try_from(
                    config
                        .iter()
                        .zip(string.iter())
                        .filter(|(center, target)| center != target)
                        .count(),
                )
                .map_err(|_| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "converting Hamming distance to i64".into(),
                    )
                })?;
                max_distance = max_distance.max(distance);
            }
            Min(Some(max_distance))
        })
    }
}

crate::declare_variants! {
    default ClosestString => "alphabet_size ^ string_length",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "closest_string",
        instance: Box::new(ClosestString::new(
            2,
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]],
        )),
        optimal_config: vec![0, 0, 0],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/closest_string.rs"]
mod tests;
