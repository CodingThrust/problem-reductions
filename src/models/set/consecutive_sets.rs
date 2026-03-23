//! Consecutive Sets problem implementation.
//!
//! Given an alphabet of size n, a collection of subsets of the alphabet, and a
//! bound K, determine if there exists a string of length at most K over the
//! alphabet such that the elements of each subset appear consecutively (as a
//! contiguous block in some order) within the string.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, WitnessProblem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConsecutiveSets",
        display_name: "Consecutive Sets",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine if a string exists where each subset's elements appear consecutively",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet (elements are 0..alphabet_size-1)" },
            FieldInfo { name: "subsets", type_name: "Vec<Vec<usize>>", description: "Collection of subsets of the alphabet" },
            FieldInfo { name: "bound_k", type_name: "usize", description: "Maximum string length K" },
        ],
    }
}

/// Consecutive Sets problem.
///
/// Given an alphabet {0, 1, ..., n-1}, a collection of subsets, and a bound K,
/// determine if there exists a string w of length at most K over the alphabet
/// such that the elements of each subset appear as a contiguous block (in any
/// order) within w.
///
/// Configurations use `bound_k` positions. Values `0..alphabet_size-1`
/// represent alphabet symbols, and the extra value `alphabet_size` marks
/// unused positions beyond the end of a shorter string. Only trailing unused
/// positions are valid.
///
/// This problem is NP-complete and arises in physical mapping of DNA and in
/// consecutive arrangements of hypergraph vertices.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::ConsecutiveSets;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Alphabet: {0, 1, 2, 3, 4, 5}, subsets that must appear consecutively
/// let problem = ConsecutiveSets::new(
///     6,
///     vec![vec![0, 4], vec![2, 4], vec![2, 5], vec![1, 5], vec![1, 3]],
///     6,
/// );
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
///
/// // w = [0, 4, 2, 5, 1, 3] is a valid solution
/// assert!(solution.is_some());
/// assert!(problem.evaluate(&solution.unwrap()));
///
/// // Shorter strings are encoded with trailing `unused = alphabet_size`.
/// let shorter = ConsecutiveSets::new(3, vec![vec![0, 1]], 4);
/// let unused = shorter.alphabet_size();
/// assert!(shorter.evaluate(&[0, 1, unused, unused]));
/// assert!(!shorter.evaluate(&[0, unused, 1, unused]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveSets {
    /// Size of the alphabet (elements are 0..alphabet_size-1).
    alphabet_size: usize,
    /// Collection of subsets of the alphabet, each sorted in canonical form.
    subsets: Vec<Vec<usize>>,
    /// Maximum string length K.
    bound_k: usize,
}

impl ConsecutiveSets {
    /// Create a new Consecutive Sets problem.
    ///
    /// # Panics
    ///
    /// Panics if `bound_k` is zero, if any subset contains duplicate elements,
    /// or if any element is outside the alphabet.
    pub fn new(alphabet_size: usize, subsets: Vec<Vec<usize>>, bound_k: usize) -> Self {
        assert!(bound_k > 0, "bound_k must be positive, got 0");
        let mut subsets = subsets;
        for (i, subset) in subsets.iter_mut().enumerate() {
            let mut seen = HashSet::with_capacity(subset.len());
            for &elem in subset.iter() {
                assert!(
                    elem < alphabet_size,
                    "Subset {} contains element {} which is outside alphabet of size {}",
                    i,
                    elem,
                    alphabet_size
                );
                assert!(
                    seen.insert(elem),
                    "Subset {} contains duplicate elements",
                    i
                );
            }
            subset.sort();
        }
        Self {
            alphabet_size,
            subsets,
            bound_k,
        }
    }

    /// Get the alphabet size.
    pub fn alphabet_size(&self) -> usize {
        self.alphabet_size
    }

    /// Get the number of subsets in the collection.
    pub fn num_subsets(&self) -> usize {
        self.subsets.len()
    }

    /// Get the bound K.
    pub fn bound_k(&self) -> usize {
        self.bound_k
    }

    /// Get the subsets.
    pub fn subsets(&self) -> &[Vec<usize>] {
        &self.subsets
    }
}

impl Problem for ConsecutiveSets {
    const NAME: &'static str = "ConsecutiveSets";
    type Value = bool;

    fn dims(&self) -> Vec<usize> {
        // Each position can be any symbol (0..alphabet_size-1) or "unused" (alphabet_size)
        vec![self.alphabet_size + 1; self.bound_k]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        // 1. Validate config
        if config.len() != self.bound_k || config.iter().any(|&v| v > self.alphabet_size) {
            return false;
        }

        // 2. Build string: find the actual string length (strip trailing "unused")
        let unused = self.alphabet_size;
        let str_len = config
            .iter()
            .rposition(|&v| v != unused)
            .map_or(0, |p| p + 1);

        // 3. Check no internal "unused" symbols
        let w = &config[..str_len];
        if w.contains(&unused) {
            return false;
        }

        let mut subset_membership = vec![0usize; self.alphabet_size];
        let mut seen_in_window = vec![0usize; self.alphabet_size];
        let mut subset_stamp = 1usize;
        let mut window_stamp = 1usize;

        // 4. Check each subset has a consecutive block
        for subset in &self.subsets {
            let subset_len = subset.len();
            if subset_len == 0 {
                continue; // empty subset trivially satisfied
            }
            if subset_len > str_len {
                return false; // can't fit
            }

            for &elem in subset {
                subset_membership[elem] = subset_stamp;
            }

            let mut found = false;
            for start in 0..=(str_len - subset_len) {
                let window = &w[start..start + subset_len];
                let current_window_stamp = window_stamp;
                window_stamp += 1;

                // Because subsets are validated to contain unique elements,
                // a window matches iff every symbol belongs to the subset and
                // appears at most once.
                if window.iter().all(|&elem| {
                    let is_member = subset_membership[elem] == subset_stamp;
                    let is_new = seen_in_window[elem] != current_window_stamp;
                    if is_member && is_new {
                        seen_in_window[elem] = current_window_stamp;
                        true
                    } else {
                        false
                    }
                }) {
                    // subset is already sorted
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }

            subset_stamp += 1;
        }

        true
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl WitnessProblem for ConsecutiveSets {}

crate::declare_variants! {
    default sat ConsecutiveSets => "alphabet_size^bound_k * num_subsets",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "consecutive_sets",
        // YES instance from issue: w = [0, 4, 2, 5, 1, 3]
        instance: Box::new(ConsecutiveSets::new(
            6,
            vec![vec![0, 4], vec![2, 4], vec![2, 5], vec![1, 5], vec![1, 3]],
            6,
        )),
        optimal_config: vec![0, 4, 2, 5, 1, 3],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/consecutive_sets.rs"]
mod tests;
