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

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "StringToStringCorrection",
        display_name: "String-to-String Correction",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Derive target string from source using at most K deletions and adjacent swaps",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the finite alphabet" },
            FieldInfo { name: "source", type_name: "Vec<usize>", description: "Source string (symbol indices)" },
            FieldInfo { name: "target", type_name: "Vec<usize>", description: "Target string (symbol indices)" },
            FieldInfo { name: "bound_k", type_name: "usize", description: "Maximum number of operations allowed" },
        ],
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
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // source = [0,1,2,3,1,0], target = [0,1,3,2,1], bound = 2
/// let problem = StringToStringCorrection::new(4, vec![0,1,2,3,1,0], vec![0,1,3,2,1], 2);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringToStringCorrection {
    alphabet_size: usize,
    source: Vec<usize>,
    target: Vec<usize>,
    bound_k: usize,
}

impl StringToStringCorrection {
    /// Create a new StringToStringCorrection instance.
    ///
    /// # Panics
    ///
    /// Panics if `alphabet_size` is 0 when the source or target string is
    /// non-empty, or if any symbol in `source` or `target` is
    /// `>= alphabet_size`.
    pub fn new(
        alphabet_size: usize,
        source: Vec<usize>,
        target: Vec<usize>,
        bound_k: usize,
    ) -> Self {
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
            bound_k,
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
    pub fn bound_k(&self) -> usize {
        self.bound_k
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
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2 * self.source.len() + 1; self.bound_k]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.bound_k {
            return false;
        }
        let n = self.source.len();
        let domain = 2 * n + 1;
        if config.iter().any(|&v| v >= domain) {
            return false;
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
                    return false;
                }
            }
        }
        working == self.target
    }
}

impl SatisfactionProblem for StringToStringCorrection {}

crate::declare_variants! {
    default sat StringToStringCorrection => "(2 * source_length + 1) ^ bound_k",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "string_to_string_correction",
        build: || {
            let problem =
                StringToStringCorrection::new(4, vec![0, 1, 2, 3, 1, 0], vec![0, 1, 3, 2, 1], 2);
            // source has length 6. Domain = 2*6+1 = 13. No-op = 12.
            // First operation: swap at positions 2,3 in original 6-element string.
            //   current_len = 6, so swap range starts at 6.
            //   swap_pos = value - current_len. For swap_pos=2, value = 6 + 2 = 8
            //   After swap: [0,1,3,2,1,0]
            // Second operation: delete at position 5 (the trailing 0).
            //   current_len = 6, 5 < 6 → delete index 5
            //   After delete: [0,1,3,2,1] = target
            crate::example_db::specs::satisfaction_example(problem, vec![vec![8, 5]])
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/string_to_string_correction.rs"]
mod tests;
