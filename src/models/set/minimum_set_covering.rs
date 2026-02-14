//! Set Covering problem implementation.
//!
//! The Set Covering problem asks for a minimum weight collection of sets
//! that covers all elements in the universe.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumSetCovering",
        module_path: module_path!(),
        description: "Find minimum weight collection covering the universe",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Size of the universe U" },
            FieldInfo { name: "sets", type_name: "Vec<Vec<usize>>", description: "Collection of subsets of U" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Weight for each set" },
        ],
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
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Universe: {0, 1, 2, 3}
/// // Sets: S0={0,1}, S1={1,2}, S2={2,3}, S3={0,3}
/// let problem = MinimumSetCovering::<i32>::new(
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
/// let solutions = solver.find_all_best(&problem);
///
/// // Verify solutions cover all elements
/// for sol in solutions {
///     assert!(problem.evaluate(&sol).is_valid());
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumSetCovering<W = i32> {
    /// Size of the universe (elements are 0..universe_size).
    universe_size: usize,
    /// Collection of sets, each represented as a vector of elements.
    sets: Vec<Vec<usize>>,
    /// Weights for each set.
    weights: Vec<W>,
}

impl<W: Clone + Default> MinimumSetCovering<W> {
    /// Create a new Set Covering problem with unit weights.
    pub fn new(universe_size: usize, sets: Vec<Vec<usize>>) -> Self
    where
        W: From<i32>,
    {
        let num_sets = sets.len();
        let weights = vec![W::from(1); num_sets];
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

    /// Check which elements are covered by selected sets.
    pub fn covered_elements(&self, config: &[usize]) -> HashSet<usize> {
        let mut covered = HashSet::new();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
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
    W: WeightElement,
{
    const NAME: &'static str = "MinimumSetCovering";
    type Metric = SolutionSize<W::Sum>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.sets.len()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        let covered = self.covered_elements(config);
        let is_valid = covered.len() == self.universe_size
            && (0..self.universe_size).all(|e| covered.contains(&e));
        if !is_valid {
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
        vec![("weight", crate::variant::short_type_name::<W>())]
    }
}

impl<W> OptimizationProblem for MinimumSetCovering<W>
where
    W: WeightElement,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a selection of sets forms a valid set cover.
pub fn is_set_cover(universe_size: usize, sets: &[Vec<usize>], selected: &[bool]) -> bool {
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

#[cfg(test)]
#[path = "../../unit_tests/models/set/minimum_set_covering.rs"]
mod tests;
