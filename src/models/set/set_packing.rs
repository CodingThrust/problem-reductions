//! Set Packing problem implementation.
//!
//! The Set Packing problem asks for a maximum weight collection of
//! pairwise disjoint sets.

use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The Set Packing problem.
///
/// Given a collection S of sets, each with a weight, find a maximum weight
/// subcollection of pairwise disjoint sets.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::SetPacking;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Sets: S0={0,1}, S1={1,2}, S2={2,3}, S3={3,4}
/// // S0 and S1 overlap, S2 and S3 are disjoint from S0
/// let problem = SetPacking::<i32>::new(vec![
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
pub struct SetPacking<W = i32> {
    /// Collection of sets.
    sets: Vec<Vec<usize>>,
    /// Weights for each set.
    weights: Vec<W>,
}

impl<W: Clone + Default> SetPacking<W> {
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

impl<W> Problem for SetPacking<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "SetPacking";

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

impl<W> ConstraintSatisfactionProblem for SetPacking<W>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_set_packing_creation() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
        assert_eq!(problem.num_sets(), 3);
        assert_eq!(problem.num_variables(), 3);
    }

    #[test]
    fn test_set_packing_with_weights() {
        let problem = SetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]);
        assert_eq!(problem.weights(), vec![5, 10]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_sets_overlap() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

        assert!(problem.sets_overlap(0, 1)); // Share element 1
        assert!(!problem.sets_overlap(0, 2)); // No overlap
        assert!(!problem.sets_overlap(1, 2)); // No overlap
    }

    #[test]
    fn test_overlapping_pairs() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

        let pairs = problem.overlapping_pairs();
        assert_eq!(pairs.len(), 2);
        assert!(pairs.contains(&(0, 1)));
        assert!(pairs.contains(&(1, 2)));
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![4, 5]]);

        // All disjoint, can select all
        let sol = problem.solution_size(&[1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 3);

        // Select none
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

        // Sets 0 and 1 overlap
        let sol = problem.solution_size(&[1, 1, 0]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_brute_force_chain() {
        // Chain: {0,1}, {1,2}, {2,3} - can select at most 2 non-adjacent sets
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Max is 2: select {0,1} and {2,3}
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 2);
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Weighted: single heavy set vs multiple light sets
        let problem = SetPacking::with_weights(
            vec![vec![0, 1, 2, 3], vec![0, 1], vec![2, 3]],
            vec![5, 3, 3],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Should select sets 1 and 2 (total 6) over set 0 (total 5)
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1, 1]);
    }

    #[test]
    fn test_is_set_packing_function() {
        let sets = vec![vec![0, 1], vec![1, 2], vec![3, 4]];

        assert!(is_set_packing(&sets, &[true, false, true])); // Disjoint
        assert!(is_set_packing(&sets, &[false, true, true])); // Disjoint
        assert!(!is_set_packing(&sets, &[true, true, false])); // Overlap on 1
        assert!(is_set_packing(&sets, &[false, false, false])); // Empty is valid
    }

    #[test]
    fn test_constraints() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
        let constraints = problem.constraints();
        // Only one overlapping pair
        assert_eq!(constraints.len(), 1);
    }

    #[test]
    fn test_energy_mode() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1]]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_disjoint_sets() {
        let problem = SetPacking::<i32>::new(vec![vec![0], vec![1], vec![2], vec![3]]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All sets are disjoint, so select all
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_all_overlapping() {
        // All sets share element 0
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![0, 3]]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Can only select one set
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
        }
    }

    #[test]
    fn test_is_satisfied() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

        assert!(problem.is_satisfied(&[1, 0, 1])); // Disjoint selection
        assert!(problem.is_satisfied(&[0, 1, 1])); // Disjoint selection
        assert!(!problem.is_satisfied(&[1, 1, 0])); // Overlapping selection
    }

    #[test]
    fn test_empty_sets() {
        let problem = SetPacking::<i32>::new(vec![]);
        let sol = problem.solution_size(&[]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_get_set() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3]]);
        assert_eq!(problem.get_set(0), Some(&vec![0, 1]));
        assert_eq!(problem.get_set(1), Some(&vec![2, 3]));
        assert_eq!(problem.get_set(2), None);
    }

    #[test]
    fn test_relationship_to_independent_set() {
        // SetPacking on sets is equivalent to IndependentSet on the intersection graph
        use crate::models::graph::IndependentSet;
        use crate::topology::SimpleGraph;

        let sets = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let sp_problem = SetPacking::<i32>::new(sets.clone());

        // Build intersection graph
        let edges = sp_problem.overlapping_pairs();
        let is_problem = IndependentSet::<SimpleGraph, i32>::new(sets.len(), edges);

        let solver = BruteForce::new();

        let sp_solutions = solver.find_best(&sp_problem);
        let is_solutions = solver.find_best(&is_problem);

        // Should have same optimal value
        let sp_size: usize = sp_solutions[0].iter().sum();
        let is_size: usize = is_solutions[0].iter().sum();
        assert_eq!(sp_size, is_size);
    }

    #[test]
    fn test_objectives() {
        let problem = SetPacking::with_weights(vec![vec![0, 1], vec![1, 2]], vec![5, 10]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 2);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2]]);
        assert!(!problem.is_weighted()); // Initially uniform
        problem.set_weights(vec![1, 2]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights(), vec![1, 2]);
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = SetPacking::<i32>::new(vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_set_packing_wrong_len() {
        let sets = vec![vec![0, 1], vec![1, 2]];
        assert!(!is_set_packing(&sets, &[true])); // Wrong length
    }

    #[test]
    fn test_problem_size() {
        let problem = SetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_sets"), Some(3));
    }
}
