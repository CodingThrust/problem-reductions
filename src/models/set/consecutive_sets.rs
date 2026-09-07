//! Consecutive Sets problem implementation.
//!
//! Given an alphabet of size n, a collection of subsets of the alphabet, and a
//! bound K, determine if there exists a string of length at most K over the
//! alphabet such that the elements of each subset appear consecutively (as a
//! contiguous block in some order) within the string.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConsecutiveSets",
        display_name: "Consecutive Sets",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Set,
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
/// Solutions use `bound_k` positions. `Some(symbol)` represents an alphabet
/// symbol and trailing `None` positions mark the unused suffix of a shorter
/// string.
///
/// This problem is NP-complete and arises in physical mapping of DNA and in
/// consecutive arrangements of hypergraph vertices.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::ConsecutiveSets;
/// use problemreductions::{Problem, BruteForce};
///
/// // Alphabet: {0, 1, 2, 3, 4, 5}, subsets that must appear consecutively
/// let problem = ConsecutiveSets::new(
///     6,
///     vec![vec![0, 4], vec![2, 4], vec![2, 5], vec![1, 5], vec![1, 3]],
///     6,
/// );
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
///
/// // w = [0, 4, 2, 5, 1, 3] is a valid solution
/// assert!(solution.is_some());
/// assert!(problem.evaluate(&solution.unwrap()).unwrap());
///
/// // Shorter strings use trailing `None` positions.
/// let shorter = ConsecutiveSets::new(3, vec![vec![0, 1]], 4);
/// assert!(shorter
///     .evaluate(&vec![Some(0), Some(1), None, None])
///     .unwrap());
/// assert!(!shorter
///     .evaluate(&vec![Some(0), None, Some(1), None])
///     .unwrap());
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
    type Solution = Vec<Option<usize>>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("alphabet_size", alphabet_size),
        ("num_subsets", num_subsets),
        ("bound_k", bound_k),
    ];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.bound_k {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "ordering representation length does not match the bound".into(),
            ));
        }
        if config
            .iter()
            .any(|symbol| symbol.is_some_and(|value| value >= self.alphabet_size))
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "ordering representation contains an out-of-range symbol".into(),
            ));
        }
        let config = config
            .iter()
            .map(|symbol| symbol.unwrap_or(self.alphabet_size))
            .collect::<Vec<_>>();
        Ok({
            crate::types::Or({
                // 2. Build string: find the actual string length (strip trailing "unused")
                let unused = self.alphabet_size;
                let str_len = config
                    .iter()
                    .rposition(|&v| v != unused)
                    .map_or(0, |p| p + 1);

                // 3. Check no internal "unused" symbols
                let w = &config[..str_len];
                if w.contains(&unused) {
                    return Ok(crate::types::Or(false));
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
                        return Ok(crate::types::Or(false)); // can't fit
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
                        return Ok(crate::types::Or(false));
                    }

                    subset_stamp += 1;
                }

                true
            })
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for ConsecutiveSets {
    fn dimensions(&self) -> Vec<usize> {
        // Each position can be any symbol (0..alphabet_size-1) or "unused" (alphabet_size)
        vec![self.alphabet_size + 1; self.bound_k]
    }
}

crate::declare_variants! {
    default ConsecutiveSets => "alphabet_size^bound_k * num_subsets",
}

crate::register_brute_force! {
    ConsecutiveSets decode |problem: &ConsecutiveSets, indices: Vec<usize>| indices.into_iter().map(|value| (value != problem.alphabet_size()).then_some(value)).collect(),
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
        optimal_config: serde_json::json!(vec![0, 4, 2, 5, 1, 3]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/consecutive_sets.rs"]
mod tests;
