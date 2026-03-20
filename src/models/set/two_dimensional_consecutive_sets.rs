//! 2-Dimensional Consecutive Sets problem implementation.
//!
//! Given a finite alphabet Σ and a collection C of subsets of Σ, determine whether
//! Σ can be partitioned into disjoint ordered groups X₁, ..., Xₖ such that each
//! group has at most one element from each subset, and each subset's elements
//! are spread across consecutive groups.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::de::Error as _;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "TwoDimensionalConsecutiveSets",
        display_name: "2-Dimensional Consecutive Sets",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine if alphabet can be partitioned into ordered groups with intersection and consecutiveness constraints",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet (elements are 0..alphabet_size-1)" },
            FieldInfo { name: "subsets", type_name: "Vec<Vec<usize>>", description: "Collection of subsets of the alphabet" },
        ],
    }
}

/// 2-Dimensional Consecutive Sets problem.
///
/// Given a finite alphabet Σ = {0, 1, ..., n-1} and a collection C = {Σ₁, ..., Σₘ}
/// of subsets of Σ, determine whether Σ can be partitioned into disjoint sets
/// X₁, X₂, ..., Xₖ such that:
/// 1. Each Xᵢ has at most one element in common with each Σⱼ (intersection constraint)
/// 2. For each Σⱼ, its elements are spread across |Σⱼ| consecutive groups (consecutiveness)
///
/// This is NP-complete (Lipski, 1977) via transformation from Graph 3-Colorability.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::TwoDimensionalConsecutiveSets;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Alphabet: {0,1,2,3,4,5}
/// // Subsets: {0,1,2}, {3,4,5}, {1,3}, {2,4}, {0,5}
/// let problem = TwoDimensionalConsecutiveSets::new(
///     6,
///     vec![vec![0, 1, 2], vec![3, 4, 5], vec![1, 3], vec![2, 4], vec![0, 5]],
/// );
///
/// // Partition: X0={0}, X1={1,5}, X2={2,3}, X3={4}
/// // config[i] = group index of symbol i
/// assert!(problem.evaluate(&[0, 1, 2, 2, 3, 1]));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct TwoDimensionalConsecutiveSets {
    /// Size of the alphabet (elements are 0..alphabet_size-1).
    alphabet_size: usize,
    /// Collection of subsets, each a sorted list of alphabet elements.
    subsets: Vec<Vec<usize>>,
}

#[derive(Debug, Deserialize)]
struct TwoDimensionalConsecutiveSetsUnchecked {
    alphabet_size: usize,
    subsets: Vec<Vec<usize>>,
}

fn validate(alphabet_size: usize, subsets: &[Vec<usize>]) -> Result<(), String> {
    if alphabet_size == 0 {
        return Err("Alphabet size must be positive".to_string());
    }

    for (i, subset) in subsets.iter().enumerate() {
        let mut seen = HashSet::new();
        for &elem in subset {
            if elem >= alphabet_size {
                return Err(format!(
                    "Subset {} contains element {} which is outside alphabet of size {}",
                    i, elem, alphabet_size
                ));
            }
            if !seen.insert(elem) {
                return Err(format!("Subset {} contains duplicate element {}", i, elem));
            }
        }
    }

    Ok(())
}

impl<'de> Deserialize<'de> for TwoDimensionalConsecutiveSets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let unchecked = TwoDimensionalConsecutiveSetsUnchecked::deserialize(deserializer)?;
        Self::try_new(unchecked.alphabet_size, unchecked.subsets).map_err(D::Error::custom)
    }
}

impl TwoDimensionalConsecutiveSets {
    /// Create a new 2-Dimensional Consecutive Sets instance, returning validation errors.
    pub fn try_new(alphabet_size: usize, subsets: Vec<Vec<usize>>) -> Result<Self, String> {
        validate(alphabet_size, &subsets)?;
        let subsets = subsets
            .into_iter()
            .map(|mut s| {
                s.sort();
                s
            })
            .collect();
        Ok(Self {
            alphabet_size,
            subsets,
        })
    }

    /// Create a new 2-Dimensional Consecutive Sets instance.
    ///
    /// # Panics
    ///
    /// Panics if `alphabet_size` is 0, if any subset contains elements
    /// outside the alphabet, or if any subset has duplicate elements.
    pub fn new(alphabet_size: usize, subsets: Vec<Vec<usize>>) -> Self {
        Self::try_new(alphabet_size, subsets).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Get the alphabet size.
    pub fn alphabet_size(&self) -> usize {
        self.alphabet_size
    }

    /// Get the number of subsets.
    pub fn num_subsets(&self) -> usize {
        self.subsets.len()
    }

    /// Get the subsets.
    pub fn subsets(&self) -> &[Vec<usize>] {
        &self.subsets
    }
}

impl Problem for TwoDimensionalConsecutiveSets {
    const NAME: &'static str = "TwoDimensionalConsecutiveSets";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![self.alphabet_size; self.alphabet_size]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.alphabet_size {
            return false;
        }
        if config.iter().any(|&v| v >= self.alphabet_size) {
            return false;
        }

        // Empty labels do not create gaps in the partition order, so compress used labels first.
        let mut used = vec![false; self.alphabet_size];
        for &group in config {
            used[group] = true;
        }
        let mut dense_labels = vec![0; self.alphabet_size];
        let mut next_label = 0;
        for (label, is_used) in used.into_iter().enumerate() {
            if is_used {
                dense_labels[label] = next_label;
                next_label += 1;
            }
        }

        for subset in &self.subsets {
            if subset.is_empty() {
                continue;
            }
            let groups: Vec<usize> = subset.iter().map(|&s| dense_labels[config[s]]).collect();

            // Intersection constraint: all group indices must be distinct
            let unique: HashSet<usize> = groups.iter().copied().collect();
            if unique.len() != subset.len() {
                return false;
            }

            // Consecutiveness: group indices must form a contiguous range
            let min_g = *unique.iter().min().unwrap();
            let max_g = *unique.iter().max().unwrap();
            if max_g - min_g + 1 != subset.len() {
                return false;
            }
        }

        true
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for TwoDimensionalConsecutiveSets {}

crate::declare_variants! {
    default sat TwoDimensionalConsecutiveSets => "alphabet_size^alphabet_size",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "two_dimensional_consecutive_sets",
        instance: Box::new(TwoDimensionalConsecutiveSets::new(
            6,
            vec![
                vec![0, 1, 2],
                vec![3, 4, 5],
                vec![1, 3],
                vec![2, 4],
                vec![0, 5],
            ],
        )),
        optimal_config: vec![0, 1, 2, 2, 3, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/two_dimensional_consecutive_sets.rs"]
mod tests;
