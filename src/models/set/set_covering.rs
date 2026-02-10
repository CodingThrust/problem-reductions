//! Set Covering problem implementation.
//!
//! The Set Covering problem asks for a minimum weight collection of sets
//! that covers all elements in the universe.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "SetCovering",
        category: "set",
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
/// use problemreductions::models::set::SetCovering;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Universe: {0, 1, 2, 3}
/// // Sets: S0={0,1}, S1={1,2}, S2={2,3}, S3={0,3}
/// let problem = SetCovering::<i32>::new(
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
/// let solutions = solver.find_best(&problem);
///
/// // Verify solutions cover all elements
/// for sol in solutions {
///     assert!(problem.solution_size(&sol).is_valid);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCovering<W = i32> {
    /// Size of the universe (elements are 0..universe_size).
    universe_size: usize,
    /// Collection of sets, each represented as a vector of elements.
    sets: Vec<Vec<usize>>,
    /// Weights for each set.
    weights: Vec<W>,
}

impl<W: Clone + Default> SetCovering<W> {
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

impl<W> Problem for SetCovering<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "SetCovering";

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
        ProblemSize::new(vec![
            ("universe_size", self.universe_size),
            ("num_sets", self.sets.len()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize total weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let covered = self.covered_elements(config);
        let is_valid = covered.len() == self.universe_size
            && (0..self.universe_size).all(|e| covered.contains(&e));

        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<W> ConstraintSatisfactionProblem for SetCovering<W>
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
        // For each element, at least one set containing it must be selected
        (0..self.universe_size)
            .map(|element| {
                // Find all sets containing this element
                let containing_sets: Vec<usize> = self
                    .sets
                    .iter()
                    .enumerate()
                    .filter(|(_, set)| set.contains(&element))
                    .map(|(i, _)| i)
                    .collect();

                // Create constraint: at least one must be selected
                let num_vars = containing_sets.len();
                let num_configs = 2usize.pow(num_vars as u32);

                // All configs are valid except all-zeros
                let mut spec = vec![true; num_configs];
                spec[0] = false; // (0, 0, ..., 0) is invalid

                LocalConstraint::new(2, containing_sets, spec)
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
#[path = "../../unit_tests/models/set/set_covering.rs"]
mod tests;
