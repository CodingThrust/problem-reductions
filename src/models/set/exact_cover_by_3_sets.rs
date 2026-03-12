//! Exact Cover by 3-Sets (X3C) problem implementation.
//!
//! Given a universe X with |X| = 3q elements and a collection C of 3-element
//! subsets of X, determine if C contains an exact cover -- a subcollection of
//! q disjoint triples covering every element exactly once.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ExactCoverBy3Sets",
        module_path: module_path!(),
        description: "Determine if a collection of 3-element subsets contains an exact cover",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Size of universe X (must be divisible by 3)" },
            FieldInfo { name: "subsets", type_name: "Vec<[usize; 3]>", description: "Collection C of 3-element subsets of X" },
        ],
    }
}

/// Exact Cover by 3-Sets (X3C) problem.
///
/// Given a universe X = {0, 1, ..., 3q-1} and a collection C of 3-element
/// subsets of X, determine if there exists a subcollection C' of exactly q
/// subsets such that every element of X appears in exactly one member of C'.
///
/// This is a classical NP-complete problem (Karp, 1972), widely used as
/// a source problem for NP-completeness reductions.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::ExactCoverBy3Sets;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Universe: {0, 1, 2, 3, 4, 5} (q = 2)
/// // Subsets: S0={0,1,2}, S1={3,4,5}, S2={0,3,4}
/// let problem = ExactCoverBy3Sets::new(
///     6,
///     vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]],
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_satisfying(&problem);
///
/// // S0 and S1 form an exact cover
/// assert_eq!(solutions.len(), 1);
/// assert!(problem.evaluate(&solutions[0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExactCoverBy3Sets {
    /// Size of the universe (elements are 0..universe_size, must be divisible by 3).
    universe_size: usize,
    /// Collection of 3-element subsets, each represented as a sorted triple of elements.
    subsets: Vec<[usize; 3]>,
}

impl ExactCoverBy3Sets {
    /// Create a new X3C problem.
    ///
    /// # Panics
    ///
    /// Panics if `universe_size` is not divisible by 3, or if any subset
    /// contains elements outside the universe.
    pub fn new(universe_size: usize, subsets: Vec<[usize; 3]>) -> Self {
        assert!(
            universe_size.is_multiple_of(3),
            "Universe size must be divisible by 3, got {}",
            universe_size
        );
        for (i, subset) in subsets.iter().enumerate() {
            for &elem in subset {
                assert!(
                    elem < universe_size,
                    "Subset {} contains element {} which is outside universe of size {}",
                    i,
                    elem,
                    universe_size
                );
            }
        }
        Self {
            universe_size,
            subsets,
        }
    }

    /// Get the universe size.
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    /// Get the number of subsets in the collection.
    pub fn num_subsets(&self) -> usize {
        self.subsets.len()
    }

    /// Get the subsets.
    pub fn subsets(&self) -> &[[usize; 3]] {
        &self.subsets
    }

    /// Get a specific subset.
    pub fn get_subset(&self, index: usize) -> Option<&[usize; 3]> {
        self.subsets.get(index)
    }

    /// Check if a configuration is a valid exact cover.
    ///
    /// A valid exact cover selects exactly q = universe_size/3 subsets
    /// that are pairwise disjoint and whose union equals the universe.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }

    /// Get the elements covered by the selected subsets.
    pub fn covered_elements(&self, config: &[usize]) -> HashSet<usize> {
        let mut covered = HashSet::new();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                if let Some(subset) = self.subsets.get(i) {
                    covered.extend(subset.iter().copied());
                }
            }
        }
        covered
    }
}

impl Problem for ExactCoverBy3Sets {
    const NAME: &'static str = "ExactCoverBy3Sets";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.subsets.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let q = self.universe_size / 3;

        // Count selected subsets
        let selected_count: usize = config.iter().filter(|&&v| v == 1).sum();
        if selected_count != q {
            return false;
        }

        // Check that selected subsets are pairwise disjoint and cover everything
        let mut covered = HashSet::with_capacity(self.universe_size);
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                if let Some(subset) = self.subsets.get(i) {
                    for &elem in subset {
                        if !covered.insert(elem) {
                            // Element already covered -- not disjoint
                            return false;
                        }
                    }
                }
            }
        }

        // Check all elements are covered
        covered.len() == self.universe_size
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for ExactCoverBy3Sets {}

crate::declare_variants! {
    ExactCoverBy3Sets => "2^num_subsets",
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/exact_cover_by_3_sets.rs"]
mod tests;
