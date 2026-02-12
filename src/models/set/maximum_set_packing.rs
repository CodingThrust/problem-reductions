//! Set Packing problem implementation.
//!
//! The Set Packing problem asks for a maximum weight collection of
//! pairwise disjoint sets.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumSetPacking",
        category: "set",
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
/// let solutions = solver.find_best(&problem);
///
/// // Verify solutions are pairwise disjoint
/// for sol in solutions {
///     assert!(problem.solution_size(&sol).is_valid);
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
}

impl<W> Problem for MaximumSetPacking<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "MaximumSetPacking";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", short_type_name::<W>())]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.sets.len()
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("num_sets", self.sets.len())])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter // Maximize total weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = is_valid_packing(&self.sets, config);
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<W> ConstraintSatisfactionProblem for MaximumSetPacking<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        // For each pair of overlapping sets, at most one can be selected
        self.overlapping_pairs()
            .into_iter()
            .map(|(i, j)| {
                LocalConstraint::new(
                    2,
                    vec![i, j],
                    vec![true, true, true, false], // (0,0), (0,1), (1,0) OK; (1,1) invalid
                )
            })
            .collect()
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        self.weights
            .iter()
            .enumerate()
            .map(|(i, w)| LocalSolutionSize::new(2, vec![i], vec![W::zero(), w.clone()]))
            .collect()
    }

    fn weights(&self) -> Vec<Self::Size> {
        self.weights.clone()
    }

    fn set_weights(&mut self, weights: Vec<Self::Size>) {
        assert_eq!(weights.len(), self.num_variables());
        self.weights = weights;
    }

    fn is_weighted(&self) -> bool {
        if self.weights.is_empty() {
            return false;
        }
        let first = &self.weights[0];
        !self.weights.iter().all(|w| w == first)
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

// === ProblemV2 / OptimizationProblemV2 implementations ===

impl<W> crate::traits::ProblemV2 for MaximumSetPacking<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "MaximumSetPacking";
    type Metric = W;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.sets.len()]
    }

    fn evaluate(&self, config: &[usize]) -> W {
        if !is_valid_packing(&self.sets, config) {
            return W::min_value();
        }
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        total
    }
}

impl<W> crate::traits::OptimizationProblemV2 for MaximumSetPacking<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + 'static,
{
    fn direction(&self) -> crate::types::Direction {
        crate::types::Direction::Maximize
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/maximum_set_packing.rs"]
mod tests;
