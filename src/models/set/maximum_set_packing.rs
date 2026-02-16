//! Set Packing problem implementation.
//!
//! The Set Packing problem asks for a maximum weight collection of
//! pairwise disjoint sets.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumSetPacking",
        module_path: module_path!(),
        description: "Find maximum weight collection of disjoint sets",
        fields: &[
            FieldInfo { name: "sets", type_name: "Vec<Vec<usize>>", description: "Collection of sets over a universe" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Weight for each set" },
        ],
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
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Sets: S0={0,1}, S1={1,2}, S2={2,3}, S3={3,4}
/// // S0 and S1 overlap, S2 and S3 are disjoint from S0
/// let problem = MaximumSetPacking::<i32>::new(vec![
///     vec![0, 1],
///     vec![1, 2],
///     vec![2, 3],
///     vec![3, 4],
/// ]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Verify solutions are pairwise disjoint
/// for sol in solutions {
///     assert!(problem.evaluate(&sol).is_valid());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumSetPacking<W = i32> {
    /// Collection of sets.
    sets: Vec<Vec<usize>>,
    /// Weights for each set.
    weights: Vec<W>,
}

impl<W: Clone + Default> MaximumSetPacking<W> {
    /// Create a new Set Packing problem with unit weights.
    pub fn new(sets: Vec<Vec<usize>>) -> Self
    where
        W: From<i32>,
    {
        let num_sets = sets.len();
        let weights = vec![W::from(1); num_sets];
        Self { sets, weights }
    }

    /// Create a new Set Packing problem with custom weights.
    pub fn with_weights(sets: Vec<Vec<usize>>, weights: Vec<W>) -> Self {
        assert_eq!(sets.len(), weights.len());
        Self { sets, weights }
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

    /// Get a reference to the weights vector.
    pub fn weights_ref(&self) -> &Vec<W> {
        &self.weights
    }

    /// Check if a configuration is a valid set packing.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_packing(&self.sets, config)
    }
}

impl<W> Problem for MaximumSetPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumSetPacking";
    type Metric = SolutionSize<W::Sum>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.sets.len()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !is_valid_packing(&self.sets, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }

    fn problem_size_names() -> &'static [&'static str] {
        &["num_sets", "universe_size"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        let universe_size = self
            .sets()
            .iter()
            .flat_map(|s| s.iter())
            .max()
            .map_or(0, |&m| m + 1);
        vec![self.num_sets(), universe_size]
    }
}

impl<W> OptimizationProblem for MaximumSetPacking<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Check if a selection forms a valid set packing (pairwise disjoint).
fn is_valid_packing(sets: &[Vec<usize>], config: &[usize]) -> bool {
    let selected_sets: Vec<_> = config
        .iter()
        .enumerate()
        .filter(|(_, &s)| s == 1)
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
pub fn is_set_packing(sets: &[Vec<usize>], selected: &[bool]) -> bool {
    if selected.len() != sets.len() {
        return false;
    }

    let config: Vec<usize> = selected.iter().map(|&b| if b { 1 } else { 0 }).collect();
    is_valid_packing(sets, &config)
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/maximum_set_packing.rs"]
mod tests;
