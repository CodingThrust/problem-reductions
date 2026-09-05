//! String-to-String Correction problem implementation.
//!
//! Given a source string `s` and a target string `t` over a finite alphabet,
//! and a bound `K`, the problem asks whether `t` can be derived from `s`
//! using at most `K` operations, where each operation is either a deletion
//! of a character or a swap of two adjacent characters.
//!
//! The configuration is a vector of length `K`, where each entry encodes one
//! operation. For a source of length `n`, each entry is in `{0, ..., 2n}`:
//! - `0..current_len` → delete the character at that index
//! - `current_len..2n` → swap the character at position `value - current_len`
//!   with its right neighbor
//! - `2n` → no-op (skip this operation slot)
//!
//! This problem is NP-complete (Wagner, 1975).

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "StringToStringCorrection",
        display_name: "String-to-String Correction",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Derive target string from source using at most K deletions and adjacent swaps",
        fields: StringToStringCorrectionCreateSpec::FIELDS,
    }
}

/// The String-to-String Correction problem.
///
/// Given an alphabet of size `a`, a source string `s` over `{0, ..., a-1}`,
/// a target string `t` over the same alphabet, and a bound `K`, determine
/// whether `t` can be obtained from `s` by applying at most `K` operations,
/// where each operation is either a character deletion or a swap of two
/// adjacent characters.
///
/// # Representation
///
/// The configuration is a vector of length `K`. For a source string of
/// length `n`, each entry is in `{0, ..., 2n}`:
/// - Values `0..current_len` delete the character at that index in the
///   current working string.
/// - Values `current_len..2n` swap the character at position
///   `value - current_len` with its right neighbor.
/// - Value `2n` is a no-op (skip this slot).
///
/// The domain size per slot is fixed at `2n + 1` regardless of how
/// deletions shorten the working string; as the working string shrinks,
/// some encodings that were valid before may become invalid.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::StringToStringCorrection;
/// use problemreductions::{Problem, BruteForce};
///
/// // source = [0,1,2,3,1,0], target = [0,1,3,2,1], bound = 2
/// let problem = StringToStringCorrection::new(4, vec![0,1,2,3,1,0], vec![0,1,3,2,1], 2);
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringToStringCorrection {
    alphabet_size: usize,
    source: Vec<usize>,
    target: Vec<usize>,
    bound: usize,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct StringToStringCorrectionCreateSpec {
    /// Optional alphabet size; omitted values are inferred from both strings.
    alphabet_size: Option<usize>,
    /// Source string.
    #[create(codec = "comma-separated")]
    source_string: Vec<usize>,
    /// Target string.
    #[create(codec = "comma-separated")]
    target_string: Vec<usize>,
    /// Maximum number of correction operations.
    bound: usize,
}

impl TryFrom<StringToStringCorrectionCreateSpec> for StringToStringCorrection {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: StringToStringCorrectionCreateSpec) -> Result<Self, Self::Error> {
        let inferred_alphabet_size = spec
            .source_string
            .iter()
            .chain(&spec.target_string)
            .copied()
            .max()
            .map(|symbol| {
                symbol
                    .checked_add(1)
                    .ok_or_else(|| "inferred alphabet size overflows usize".to_string())
            })
            .transpose()?
            .unwrap_or(0);
        let alphabet_size = spec.alphabet_size.unwrap_or(inferred_alphabet_size);
        if alphabet_size < inferred_alphabet_size {
            return Err(format!(
                "alphabet size {alphabet_size} is smaller than inferred alphabet size {inferred_alphabet_size}"
            ).into());
        }
        if alphabet_size == 0 && (!spec.source_string.is_empty() || !spec.target_string.is_empty())
        {
            return Err("alphabet size must be positive when either string is non-empty".into());
        }

        Ok(Self {
            alphabet_size,
            source: spec.source_string,
            target: spec.target_string,
            bound: spec.bound,
        })
    }
}

impl StringToStringCorrection {
    /// Create a new StringToStringCorrection instance.
    ///
    /// # Panics
    ///
    /// Panics if `alphabet_size` is 0 when the source or target string is
    /// non-empty, or if any symbol in `source` or `target` is
    /// `>= alphabet_size`.
    pub fn new(alphabet_size: usize, source: Vec<usize>, target: Vec<usize>, bound: usize) -> Self {
        assert!(
            alphabet_size > 0 || (source.is_empty() && target.is_empty()),
            "alphabet_size must be > 0 when source or target is non-empty"
        );
        assert!(
            source.iter().all(|&s| s < alphabet_size),
            "all source symbols must be < alphabet_size"
        );
        assert!(
            target.iter().all(|&s| s < alphabet_size),
            "all target symbols must be < alphabet_size"
        );
        Self {
            alphabet_size,
            source,
            target,
            bound,
        }
    }

    /// Returns the alphabet size.
    pub fn alphabet_size(&self) -> usize {
        self.alphabet_size
    }

    /// Returns the source string.
    pub fn source(&self) -> &[usize] {
        &self.source
    }

    /// Returns the target string.
    pub fn target(&self) -> &[usize] {
        &self.target
    }

    /// Returns the operation bound.
    pub fn bound(&self) -> usize {
        self.bound
    }

    /// Returns the length of the source string.
    pub fn source_length(&self) -> usize {
        self.source.len()
    }

    /// Returns the length of the target string.
    pub fn target_length(&self) -> usize {
        self.target.len()
    }
}

impl Problem for StringToStringCorrection {
    const NAME: &'static str = "StringToStringCorrection";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![("bound", bound), ("source_length", source_length),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.bound {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "edit-program length does not match the operation bound".into(),
                    ));
                }
                if self.target.len() > self.source.len()
                    || self.target.len() < self.source.len().saturating_sub(self.bound)
                {
                    return Ok(crate::types::Or(false));
                }
                let n = self.source.len();
                let domain = 2 * n + 1;
                if config.iter().any(|&v| v >= domain) {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "edit program contains an out-of-range operation".into(),
                    ));
                }
                let noop = 2 * n;
                let mut working = self.source.clone();
                for &op in config {
                    if op == noop {
                        // no-op
                        continue;
                    }
                    let current_len = working.len();
                    if op < current_len {
                        // delete at index op
                        working.remove(op);
                    } else {
                        let swap_pos = op - current_len;
                        if swap_pos + 1 < current_len {
                            working.swap(swap_pos, swap_pos + 1);
                        } else {
                            // invalid operation for current string state
                            return Ok(crate::types::Or(false));
                        }
                    }
                }
                working == self.target
            })
        })
    }
}

impl crate::solvers::BruteForceProblem for StringToStringCorrection {
    fn dimensions(&self) -> Vec<usize> {
        vec![2 * self.source.len() + 1; self.bound]
    }
}

crate::declare_variants! {
    default StringToStringCorrection => "(2 * source_length + 1) ^ bound" create StringToStringCorrectionCreateSpec,
}

crate::register_brute_force! {
    StringToStringCorrection,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "string_to_string_correction",
        // source has length 6. Domain = 2*6+1 = 13. No-op = 12.
        // First operation: swap at positions 2,3 → value = 6 + 2 = 8
        // Second operation: delete at position 5
        instance: Box::new(StringToStringCorrection::new(
            4,
            vec![0, 1, 2, 3, 1, 0],
            vec![0, 1, 3, 2, 1],
            2,
        )),
        optimal_config: serde_json::json!(vec![8, 5]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/string_to_string_correction.rs"]
mod tests;
