//! Minimum Hitting Set problem implementation.
//!
//! The Minimum Hitting Set problem asks for a minimum-size subset of universe
//! elements that intersects every set in a collection.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumHittingSet",
        display_name: "Minimum Hitting Set",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a minimum-size subset of universe elements that hits every set",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Size of the universe U" },
            FieldInfo { name: "sets", type_name: "Vec<Vec<usize>>", description: "Collection of subsets of U that must each be hit" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MinimumHittingSet",
        fields: &["num_sets", "universe_size"],
    }
}

/// The Minimum Hitting Set problem.
///
/// Given a universe `U` and a collection of subsets of `U`, find a minimum-size
/// subset `H ⊆ U` such that `H` intersects every set in the collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumHittingSet {
    universe_size: usize,
    sets: Vec<Vec<usize>>,
}

impl MinimumHittingSet {
    /// Create a new Minimum Hitting Set instance.
    ///
    /// # Panics
    ///
    /// Panics if any set contains an element outside `0..universe_size`.
    pub fn new(universe_size: usize, sets: Vec<Vec<usize>>) -> Self {
        let mut sets = sets;
        for (set_index, set) in sets.iter_mut().enumerate() {
            set.sort_unstable();
            set.dedup();
            for &element in set.iter() {
                assert!(
                    element < universe_size,
                    "Set {set_index} contains element {element} which is outside universe of size {universe_size}"
                );
            }
        }

        Self {
            universe_size,
            sets,
        }
    }

    /// Get the universe size.
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    /// Get the number of sets.
    pub fn num_sets(&self) -> usize {
        self.sets.len()
    }

    /// Get the sets.
    pub fn sets(&self) -> &[Vec<usize>] {
        &self.sets
    }

    /// Get a specific set.
    pub fn get_set(&self, index: usize) -> Option<&Vec<usize>> {
        self.sets.get(index)
    }

    /// Decode the selected universe elements from a binary configuration.
    pub fn selected_elements(&self, config: &[usize]) -> Option<Vec<usize>> {
        if config.len() != self.universe_size {
            return None;
        }

        let mut selected = Vec::new();
        for (element, &value) in config.iter().enumerate() {
            match value {
                0 => {}
                1 => selected.push(element),
                _ => return None,
            }
        }
        Some(selected)
    }

    /// Check whether a configuration hits every set in the collection.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        let Some(selected) = self.selected_elements(config) else {
            return false;
        };

        self.sets.iter().all(|set| {
            set.iter()
                .any(|element| selected.binary_search(element).is_ok())
        })
    }
}

impl Problem for MinimumHittingSet {
    const NAME: &'static str = "MinimumHittingSet";
    type Value = SolutionSize<usize>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.universe_size]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<usize> {
        let Some(selected) = self.selected_elements(config) else {
            return SolutionSize::Invalid;
        };

        if self.sets.iter().all(|set| {
            set.iter()
                .any(|element| selected.binary_search(element).is_ok())
        }) {
            SolutionSize::Valid(selected.len())
        } else {
            SolutionSize::Invalid
        }
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl OptimizationProblem for MinimumHittingSet {
    type Objective = usize;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt MinimumHittingSet => "2^universe_size",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_hitting_set",
        instance: Box::new(MinimumHittingSet::new(
            6,
            vec![
                vec![0, 1, 2],
                vec![0, 3, 4],
                vec![1, 3, 5],
                vec![2, 4, 5],
                vec![0, 1, 5],
                vec![2, 3],
                vec![1, 4],
            ],
        )),
        optimal_config: vec![0, 1, 0, 1, 1, 0],
        optimal_value: serde_json::json!({"Valid": 3}),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/minimum_hitting_set.rs"]
mod tests;
