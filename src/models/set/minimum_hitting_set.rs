//! Minimum Hitting Set problem implementation.
//!
//! The Minimum Hitting Set problem asks for a minimum-size subset of universe
//! elements that intersects every set in a collection.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumHittingSet",
        display_name: "Minimum Hitting Set",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Set,
        module_path: module_path!(),
        description: "Find a minimum-size subset of universe elements that hits every set",
        fields: MinimumHittingSetCreateSpec::FIELDS,
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

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumHittingSetCreateSpec {
    /// Size of the universe U.
    universe_size: usize,
    /// Collection of subsets of U that must each be hit.
    subsets: Vec<Vec<usize>>,
}

impl TryFrom<MinimumHittingSetCreateSpec> for MinimumHittingSet {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: MinimumHittingSetCreateSpec) -> Result<Self, Self::Error> {
        for (set_index, set) in spec.subsets.iter().enumerate() {
            if let Some(&element) = set.iter().find(|&&element| element >= spec.universe_size) {
                return Err(format!(
                    "subsets[{set_index}] contains element {element} outside universe of size {}",
                    spec.universe_size
                )
                .into());
            }
        }
        Ok(Self::new(spec.universe_size, spec.subsets))
    }
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
    pub fn selected_elements(&self, config: &[bool]) -> Option<Vec<usize>> {
        if config.len() != self.universe_size {
            return None;
        }

        let mut selected = Vec::new();
        for (element, &is_selected) in config.iter().enumerate() {
            if is_selected {
                selected.push(element);
            }
        }
        Some(selected)
    }

    /// Check whether a configuration hits every set in the collection.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
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
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_sets", num_sets), ("universe_size", universe_size),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            let Some(selected) = self.selected_elements(config) else {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "element-selection length does not match the universe".into(),
                ));
            };

            if self.sets.iter().all(|set| {
                set.iter()
                    .any(|element| selected.binary_search(element).is_ok())
            }) {
                Min(Some(i64::try_from(selected.len()).map_err(|_| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "converting hitting-set cardinality to i64".into(),
                    )
                })?))
            } else {
                Min(None)
            }
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for MinimumHittingSet {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.universe_size]
    }
}

crate::declare_variants! {
    default MinimumHittingSet => "2^universe_size" create MinimumHittingSetCreateSpec,
}

crate::register_brute_force! {
    MinimumHittingSet decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
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
        optimal_config: serde_json::json!(vec![false, true, false, true, true, false]),
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/minimum_hitting_set.rs"]
mod tests;
