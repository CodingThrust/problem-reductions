//! Set Splitting problem implementation.
//!
//! Set Splitting asks whether the universe can be partitioned into two parts so
//! that every subset contains elements from both parts.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SetSplitting",
        display_name: "Set Splitting",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a bipartition of the universe so that no subset is monochromatic",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Number of elements in the finite universe" },
            FieldInfo { name: "subsets", type_name: "Vec<Vec<usize>>", description: "Collection of subsets that must each contain both partition colors" },
        ],
    }
}

/// The Set Splitting decision problem.
///
/// Given a finite universe `S` and a collection `C` of subsets of `S`,
/// determine whether `S` can be partitioned into `S_1` and `S_2` such that
/// every subset in `C` has at least one element in each part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSplitting {
    universe_size: usize,
    subsets: Vec<Vec<usize>>,
}

impl SetSplitting {
    /// Create a new Set Splitting instance.
    ///
    /// # Panics
    ///
    /// Panics if any subset contains an element outside `0..universe_size`.
    pub fn new(universe_size: usize, subsets: Vec<Vec<usize>>) -> Self {
        let mut subsets = subsets;
        for (subset_index, subset) in subsets.iter_mut().enumerate() {
            subset.sort_unstable();
            subset.dedup();
            for &element in subset.iter() {
                assert!(
                    element < universe_size,
                    "Subset {subset_index} contains element {element} which is outside universe of size {universe_size}"
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

    /// Get the subsets.
    pub fn subsets(&self) -> &[Vec<usize>] {
        &self.subsets
    }

    /// Get the number of subsets.
    pub fn num_subsets(&self) -> usize {
        self.subsets.len()
    }

    fn config_is_binary_partition(&self, config: &[usize]) -> bool {
        config.len() == self.universe_size && config.iter().all(|&value| value <= 1)
    }
}

impl Problem for SetSplitting {
    const NAME: &'static str = "SetSplitting";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.universe_size]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if !self.config_is_binary_partition(config) {
            return Or(false);
        }

        for subset in &self.subsets {
            let Some((&first, rest)) = subset.split_first() else {
                return Or(false);
            };
            let first_part = config[first];
            if rest.iter().all(|&element| config[element] == first_part) {
                return Or(false);
            }
        }

        Or(true)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default SetSplitting => "2^universe_size",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "set_splitting",
        instance: Box::new(SetSplitting::new(
            6,
            vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4, 5], vec![1, 3, 5]],
        )),
        optimal_config: vec![0, 1, 0, 1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/set_splitting.rs"]
mod tests;
