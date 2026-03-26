//! Comparative Containment problem implementation.
//!
//! Given two weighted families of sets over a common universe, determine
//! whether there exists a subset of the universe whose containment weight
//! in the first family is at least its containment weight in the second.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ComparativeContainment",
        display_name: "Comparative Containment",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i32", &["One", "i32", "f64"])],
        module_path: module_path!(),
        description: "Compare containment-weight sums for two set families over a shared universe",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Size of the universe X" },
            FieldInfo { name: "r_sets", type_name: "Vec<Vec<usize>>", description: "First set family R over X" },
            FieldInfo { name: "s_sets", type_name: "Vec<Vec<usize>>", description: "Second set family S over X" },
            FieldInfo { name: "r_weights", type_name: "Vec<W>", description: "Positive weights for sets in R" },
            FieldInfo { name: "s_weights", type_name: "Vec<W>", description: "Positive weights for sets in S" },
        ],
    }
}

/// Comparative Containment.
///
/// Given a universe `X`, two set families `R` and `S`, and positive weights
/// on those sets, determine whether there exists a subset `Y ⊆ X` such that
/// the total weight of `R`-sets containing `Y` is at least the total weight
/// of `S`-sets containing `Y`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeContainment<W = i32> {
    universe_size: usize,
    r_sets: Vec<Vec<usize>>,
    s_sets: Vec<Vec<usize>>,
    r_weights: Vec<W>,
    s_weights: Vec<W>,
}

impl<W: WeightElement> ComparativeContainment<W> {
    /// Create a new instance with unit weights.
    pub fn new(universe_size: usize, r_sets: Vec<Vec<usize>>, s_sets: Vec<Vec<usize>>) -> Self
    where
        W: From<i32>,
    {
        let r_weights = vec![W::from(1); r_sets.len()];
        let s_weights = vec![W::from(1); s_sets.len()];
        Self::with_weights(universe_size, r_sets, s_sets, r_weights, s_weights)
    }

    /// Create a new instance with explicit weights.
    pub fn with_weights(
        universe_size: usize,
        r_sets: Vec<Vec<usize>>,
        s_sets: Vec<Vec<usize>>,
        r_weights: Vec<W>,
        s_weights: Vec<W>,
    ) -> Self {
        assert_eq!(
            r_sets.len(),
            r_weights.len(),
            "number of R sets and R weights must match"
        );
        assert_eq!(
            s_sets.len(),
            s_weights.len(),
            "number of S sets and S weights must match"
        );
        validate_set_family("R", universe_size, &r_sets);
        validate_set_family("S", universe_size, &s_sets);
        validate_weight_family("R", &r_weights);
        validate_weight_family("S", &s_weights);
        Self {
            universe_size,
            r_sets,
            s_sets,
            r_weights,
            s_weights,
        }
    }

    /// Get the size of the universe.
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    /// Get the number of sets in the R family.
    pub fn num_r_sets(&self) -> usize {
        self.r_sets.len()
    }

    /// Get the number of sets in the S family.
    pub fn num_s_sets(&self) -> usize {
        self.s_sets.len()
    }

    /// Get the R family.
    pub fn r_sets(&self) -> &[Vec<usize>] {
        &self.r_sets
    }

    /// Get the S family.
    pub fn s_sets(&self) -> &[Vec<usize>] {
        &self.s_sets
    }

    /// Get the R-family weights.
    pub fn r_weights(&self) -> &[W] {
        &self.r_weights
    }

    /// Get the S-family weights.
    pub fn s_weights(&self) -> &[W] {
        &self.s_weights
    }

    /// Check whether the subset selected by `config` is contained in `set`.
    pub fn contains_selected_subset(&self, config: &[usize], set: &[usize]) -> bool {
        self.valid_config(config) && contains_selected_subset_unchecked(config, set)
    }

    fn valid_config(&self, config: &[usize]) -> bool {
        config.len() == self.universe_size && config.iter().all(|&value| value <= 1)
    }
}

impl<W> ComparativeContainment<W>
where
    W: WeightElement,
{
    /// Total R-family weight for sets containing the selected subset.
    pub fn r_weight_sum(&self, config: &[usize]) -> Option<W::Sum> {
        self.sum_containing_weights(config, &self.r_sets, &self.r_weights)
    }

    /// Total S-family weight for sets containing the selected subset.
    pub fn s_weight_sum(&self, config: &[usize]) -> Option<W::Sum> {
        self.sum_containing_weights(config, &self.s_sets, &self.s_weights)
    }

    /// Check if a configuration is a satisfying solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        match (self.r_weight_sum(config), self.s_weight_sum(config)) {
            (Some(r_total), Some(s_total)) => r_total >= s_total,
            _ => false,
        }
    }

    fn sum_containing_weights(
        &self,
        config: &[usize],
        sets: &[Vec<usize>],
        weights: &[W],
    ) -> Option<W::Sum> {
        if !self.valid_config(config) {
            return None;
        }

        let mut total = W::Sum::zero();
        for (set, weight) in sets.iter().zip(weights.iter()) {
            if contains_selected_subset_unchecked(config, set) {
                total += weight.to_sum();
            }
        }
        Some(total)
    }
}

impl<W> Problem for ComparativeContainment<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "ComparativeContainment";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.universe_size]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(self.is_valid_solution(config))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }
}

crate::declare_variants! {
    ComparativeContainment<One> => "2^universe_size",
    default ComparativeContainment<i32> => "2^universe_size",
    ComparativeContainment<f64> => "2^universe_size",
}

fn validate_set_family(label: &str, universe_size: usize, sets: &[Vec<usize>]) {
    for (set_index, set) in sets.iter().enumerate() {
        for &element in set {
            assert!(
                element < universe_size,
                "{label} set {set_index} contains element {element} outside universe of size {universe_size}"
            );
        }
    }
}

fn validate_weight_family<W: WeightElement>(label: &str, weights: &[W]) {
    for (index, weight) in weights.iter().enumerate() {
        let sum = weight.to_sum();
        assert!(
            sum.partial_cmp(&W::Sum::zero()) == Some(std::cmp::Ordering::Greater),
            "{label} weights must be finite and positive; weight at index {index} is not"
        );
    }
}

fn contains_selected_subset_unchecked(config: &[usize], set: &[usize]) -> bool {
    config
        .iter()
        .enumerate()
        .all(|(element, &selected)| selected == 0 || set.contains(&element))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "comparative_containment_i32",
        instance: Box::new(ComparativeContainment::with_weights(
            4,
            vec![vec![0, 1, 2, 3], vec![0, 1]],
            vec![vec![0, 1, 2, 3], vec![2, 3]],
            vec![2, 5],
            vec![3, 6],
        )),
        optimal_config: vec![0, 1, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/comparative_containment.rs"]
mod tests;
