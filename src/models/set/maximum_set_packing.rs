//! Set Packing problem implementation.
//!
//! The Set Packing problem asks for a maximum weight collection of
//! pairwise disjoint sets.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{Max, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumSetPacking",
        display_name: "Maximum Set Packing",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "One", &["One", "i64", "f64"])],
        category: crate::registry::ProblemCategory::Set,
        module_path: module_path!(),
        description: "Find maximum weight collection of disjoint sets",
        fields: MaximumSetPackingCreateSpec::<One>::FIELDS,
    }
}

/// The Set Packing problem.
///
/// Given a collection S of sets, each with a weight, find a maximum weight
/// subcollection of pairwise disjoint sets.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::MaximumSetPacking;
/// use problemreductions::{Problem, BruteForce};
///
/// // Sets: S0={0,1}, S1={1,2}, S2={2,3}, S3={3,4}
/// // S0 and S1 overlap, S2 and S3 are disjoint from S0
/// let problem = MaximumSetPacking::<i64>::new(vec![
///     vec![0, 1],
///     vec![1, 2],
///     vec![2, 3],
///     vec![3, 4],
/// ]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Verify solutions are pairwise disjoint
/// for sol in solutions {
///     assert!(problem.evaluate(&sol).unwrap().is_valid());
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MaximumSetPacking<W = i64> {
    /// Collection of sets.
    sets: Vec<Vec<usize>>,
    /// Weights for each set.
    weights: Vec<W>,
}

#[derive(Deserialize)]
struct MaximumSetPackingData<W> {
    sets: Vec<Vec<usize>>,
    weights: Vec<W>,
}

impl<'de, W> Deserialize<'de> for MaximumSetPacking<W>
where
    W: WeightElement + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = MaximumSetPackingData::deserialize(deserializer)?;
        Self::with_weights(data.sets, data.weights).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MaximumSetPackingCreateSpec<W> {
    /// Collection of sets over a universe.
    subsets: Vec<Vec<usize>>,
    /// Weight for each set.
    weights: Vec<W>,
}

impl<W: WeightElement> TryFrom<MaximumSetPackingCreateSpec<W>> for MaximumSetPacking<W> {
    type Error = ConstructionError;

    fn try_from(spec: MaximumSetPackingCreateSpec<W>) -> Result<Self, Self::Error> {
        Self::with_weights(spec.subsets, spec.weights)
    }
}

impl<W: Clone + Default> MaximumSetPacking<W> {
    /// Create a new Set Packing problem with unit weights.
    pub fn new(sets: Vec<Vec<usize>>) -> Self
    where
        W: WeightElement,
    {
        let num_sets = sets.len();
        let weights = vec![W::unit(); num_sets];
        Self { sets, weights }
    }

    /// Create a new Set Packing problem with custom weights.
    pub fn with_weights(sets: Vec<Vec<usize>>, weights: Vec<W>) -> Result<Self, ConstructionError>
    where
        W: WeightElement,
    {
        if sets.len() != weights.len() {
            return Err(ConstructionError::Conversion(
                "weights length must match number of sets".into(),
            ));
        }
        for (index, weight) in weights.iter().enumerate() {
            weight.validate_element(&format!("set weight at index {index}"))?;
        }
        Ok(Self { sets, weights })
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

    /// Check if two sets overlap.
    pub fn sets_overlap(&self, i: usize, j: usize) -> bool {
        if let (Some(set_i), Some(set_j)) = (self.sets.get(i), self.sets.get(j)) {
            let set_i: HashSet<_> = set_i.iter().collect();
            set_j.iter().any(|e| set_i.contains(e))
        } else {
            false
        }
    }

    /// Get all pairs of overlapping sets.
    pub fn overlapping_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for i in 0..self.sets.len() {
            for j in (i + 1)..self.sets.len() {
                if self.sets_overlap(i, j) {
                    pairs.push((i, j));
                }
            }
        }
        pairs
    }

    /// Get the universe size (one more than the maximum element across all sets).
    pub fn universe_size(&self) -> usize {
        self.sets()
            .iter()
            .flat_map(|s| s.iter())
            .max()
            .map_or(0, |&m| m + 1)
    }

    /// Get a reference to the weights vector.
    pub fn weights_ref(&self) -> &Vec<W> {
        &self.weights
    }

    /// Check if a configuration is a valid set packing.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        is_valid_packing(&self.sets, config)
    }
}

impl<W> Problem for MaximumSetPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumSetPacking";
    type Solution = Vec<bool>;
    type Value = Max<W::Sum>;

    crate::problem_size![("num_sets", num_sets), ("universe_size", universe_size),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Max<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.sets.len() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "set-selection length does not match the family".into(),
            ));
        }
        Ok({
            if !is_valid_packing(&self.sets, config) {
                return Ok(Max(None));
            }
            let mut total = W::Sum::zero();
            for (i, &selected) in config.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.weights[i].to_sum(),
                        "summing selected set-packing weights",
                    )?;
                }
            }
            Max(Some(total))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }
}

impl<W> crate::solvers::BruteForceProblem for MaximumSetPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.sets.len()]
    }
}

crate::declare_variants! {
    default MaximumSetPacking<One> => "2^num_sets" create MaximumSetPackingCreateSpec<One>,
    MaximumSetPacking<i64> => "2^num_sets" create MaximumSetPackingCreateSpec<i64>,
    MaximumSetPacking<f64> => "2^num_sets" create MaximumSetPackingCreateSpec<f64>,
}

crate::register_brute_force! {
    MaximumSetPacking<One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumSetPacking<i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    MaximumSetPacking<f64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

/// Check if a selection forms a valid set packing (pairwise disjoint).
fn is_valid_packing(sets: &[Vec<usize>], config: &[bool]) -> bool {
    let selected_sets: Vec<_> = config
        .iter()
        .enumerate()
        .filter(|(_, &selected)| selected)
        .map(|(i, _)| i)
        .collect();

    // Check all pairs of selected sets are disjoint
    for i in 0..selected_sets.len() {
        for j in (i + 1)..selected_sets.len() {
            let set_i: HashSet<_> = sets[selected_sets[i]].iter().collect();
            if sets[selected_sets[j]].iter().any(|e| set_i.contains(e)) {
                return false;
            }
        }
    }
    true
}

/// Check if a selection of sets forms a valid set packing.
#[cfg(test)]
pub(crate) fn is_set_packing(sets: &[Vec<usize>], selected: &[bool]) -> bool {
    if selected.len() != sets.len() {
        return false;
    }

    is_valid_packing(sets, selected)
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_set_packing",
        instance: Box::new(MaximumSetPacking::<i64>::new(vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
        ])),
        optimal_config: serde_json::json!(vec![false, true, false, true]),
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/maximum_set_packing.rs"]
mod tests;
