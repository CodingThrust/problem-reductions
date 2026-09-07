//! Set Covering problem implementation.
//!
//! The Set Covering problem asks for a minimum weight collection of sets
//! that covers all elements in the universe.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumSetCovering",
        display_name: "Minimum Set Covering",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i64", &["i64"])],
        category: crate::registry::ProblemCategory::Set,
        module_path: module_path!(),
        description: "Find minimum weight collection covering the universe",
        fields: MinimumSetCoveringCreateSpec::FIELDS,
    }
}

/// The Set Covering problem.
///
/// Given a universe U of elements and a collection S of subsets of U,
/// each with a weight, find a minimum weight subcollection of S
/// that covers all elements in U.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::MinimumSetCovering;
/// use problemreductions::{Problem, BruteForce};
///
/// // Universe: {0, 1, 2, 3}
/// // Sets: S0={0,1}, S1={1,2}, S2={2,3}, S3={0,3}
/// let problem = MinimumSetCovering::<i64>::new(
///     4, // universe size
///     vec![
///         vec![0, 1],
///         vec![1, 2],
///         vec![2, 3],
///         vec![0, 3],
///     ],
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem).unwrap();
///
/// // Verify solutions cover all elements
/// for sol in solutions {
///     assert!(problem.evaluate(&sol).unwrap().is_valid());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumSetCovering<W = i64> {
    /// Size of the universe (elements are 0..universe_size).
    universe_size: usize,
    /// Collection of sets, each represented as a vector of elements.
    sets: Vec<Vec<usize>>,
    /// Weights for each set.
    weights: Vec<W>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumSetCoveringCreateSpec {
    /// Size of the universe U.
    universe_size: usize,
    /// Collection of subsets of U.
    subsets: Vec<Vec<usize>>,
    /// Weight for each subset.
    weights: Vec<i64>,
}

impl TryFrom<MinimumSetCoveringCreateSpec> for MinimumSetCovering<i64> {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: MinimumSetCoveringCreateSpec) -> Result<Self, Self::Error> {
        if spec.subsets.len() != spec.weights.len() {
            return Err(format!(
                "weights has {} entries, expected one for each of {} subsets",
                spec.weights.len(),
                spec.subsets.len()
            )
            .into());
        }
        for (set_index, set) in spec.subsets.iter().enumerate() {
            if let Some(&element) = set.iter().find(|&&element| element >= spec.universe_size) {
                return Err(format!(
                    "subsets[{set_index}] contains element {element} outside universe of size {}",
                    spec.universe_size
                )
                .into());
            }
        }
        Ok(Self::with_weights(
            spec.universe_size,
            spec.subsets,
            spec.weights,
        ))
    }
}

impl<W: Clone + Default> MinimumSetCovering<W> {
    /// Create a new Set Covering problem with unit weights.
    pub fn new(universe_size: usize, sets: Vec<Vec<usize>>) -> Self
    where
        W: WeightElement,
    {
        let num_sets = sets.len();
        let weights = vec![W::unit(); num_sets];
        Self {
            universe_size,
            sets,
            weights,
        }
    }

    /// Create a new Set Covering problem with custom weights.
    pub fn with_weights(universe_size: usize, sets: Vec<Vec<usize>>, weights: Vec<W>) -> Self {
        assert_eq!(sets.len(), weights.len());
        Self {
            universe_size,
            sets,
            weights,
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

    /// Get a reference to the weights.
    pub fn weights_ref(&self) -> &[W] {
        &self.weights
    }

    /// Check if a configuration is a valid set cover.
    pub fn is_valid_solution(&self, config: &[bool]) -> bool {
        let covered = self.covered_elements(config);
        covered.len() == self.universe_size && (0..self.universe_size).all(|e| covered.contains(&e))
    }

    /// Check which elements are covered by selected sets.
    pub fn covered_elements(&self, config: &[bool]) -> HashSet<usize> {
        let mut covered = HashSet::new();
        for (i, &selected) in config.iter().enumerate() {
            if selected {
                if let Some(set) = self.sets.get(i) {
                    covered.extend(set.iter().copied());
                }
            }
        }
        covered
    }
}

impl<W> Problem for MinimumSetCovering<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumSetCovering";
    type Solution = Vec<bool>;
    type Value = Min<W::Sum>;

    crate::problem_parameters![("num_sets", num_sets), ("universe_size", universe_size),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.sets.len() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "set-selection length does not match the family".into(),
            ));
        }
        Ok({
            let covered = self.covered_elements(config);
            let is_valid = covered.len() == self.universe_size
                && (0..self.universe_size).all(|e| covered.contains(&e));
            if !is_valid {
                return Ok(Min(None));
            }
            let mut total = W::Sum::zero();
            for (i, &selected) in config.iter().enumerate() {
                if selected {
                    total = W::checked_add_to_sum(
                        total,
                        self.weights[i].to_sum(),
                        "summing selected set-cover weights",
                    )?;
                }
            }
            Min(Some(total))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }
}

impl<W> crate::solvers::BruteForceProblem for MinimumSetCovering<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.sets.len()]
    }
}

crate::declare_variants! {
    default MinimumSetCovering<i64> => "2^num_sets" create MinimumSetCoveringCreateSpec,
}

crate::register_brute_force! {
    MinimumSetCovering<i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

/// Check if a selection of sets forms a valid set cover.
#[cfg(test)]
pub(crate) fn is_set_cover(universe_size: usize, sets: &[Vec<usize>], selected: &[bool]) -> bool {
    if selected.len() != sets.len() {
        return false;
    }

    let mut covered = HashSet::new();
    for (i, &sel) in selected.iter().enumerate() {
        if sel {
            covered.extend(sets[i].iter().copied());
        }
    }

    (0..universe_size).all(|e| covered.contains(&e))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_set_covering",
        instance: Box::new(MinimumSetCovering::<i64>::new(
            5,
            vec![vec![0, 1, 2], vec![1, 3], vec![2, 3, 4]],
        )),
        optimal_config: serde_json::json!(vec![true, false, true]),
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/minimum_set_covering.rs"]
mod tests;
