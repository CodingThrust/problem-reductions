//! Reductions between IndependentSet and SetPacking problems.
//!
//! IS → SetPacking: Each vertex becomes a set containing its incident edge indices.
//! SetPacking → IS: Each set becomes a vertex; two vertices are adjacent if their sets overlap.

use crate::models::graph::IndependentSet;
use crate::models::set::SetPacking;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::collections::HashSet;
use std::ops::AddAssign;

/// Result of reducing IndependentSet to SetPacking.
#[derive(Debug, Clone)]
pub struct ReductionISToSP<W> {
    target: SetPacking<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionISToSP<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Source = IndependentSet<W>;
    type Target = SetPacking<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly: vertex selection = set selection.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<SetPacking<W>> for IndependentSet<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32>,
{
    type Result = ReductionISToSP<W>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.edges();
        let n = self.num_vertices();

        // For each vertex, collect the indices of its incident edges
        let mut sets: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            sets[u].push(edge_idx);
            sets[v].push(edge_idx);
        }

        let target = SetPacking::with_weights(sets, self.weights_ref().clone());

        ReductionISToSP {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SetPacking to IndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionSPToIS<W> {
    target: IndependentSet<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionSPToIS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Source = SetPacking<W>;
    type Target = IndependentSet<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<IndependentSet<W>> for SetPacking<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32>,
{
    type Result = ReductionSPToIS<W>;

    fn reduce_to(&self) -> Self::Result {
        let sets = self.sets();
        let n = sets.len();

        // Create edges between sets that overlap
        let mut edges = Vec::new();
        for i in 0..n {
            let set_i: HashSet<_> = sets[i].iter().collect();
            for j in (i + 1)..n {
                // Check if sets[i] and sets[j] overlap
                if sets[j].iter().any(|elem| set_i.contains(elem)) {
                    edges.push((i, j));
                }
            }
        }

        let target = IndependentSet::with_weights(n, edges, self.weights_ref().clone());

        ReductionSPToIS {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_is_to_setpacking() {
        // Triangle graph
        let is_problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&is_problem);
        let sp_problem = reduction.target_problem();

        let solver = BruteForce::new();
        let sp_solutions = solver.find_best(sp_problem);

        // Extract back
        let is_solutions: Vec<_> = sp_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Max IS in triangle = 1
        for sol in &is_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 1);
        }
    }

    #[test]
    fn test_setpacking_to_is() {
        // Two disjoint sets and one overlapping
        let sets = vec![
            vec![0, 1],
            vec![2, 3],
            vec![1, 2], // overlaps with both
        ];
        let sp_problem = SetPacking::<i32>::new(sets);
        let reduction: ReductionSPToIS<i32> = sp_problem.reduce_to();
        let is_problem = reduction.target_problem();

        let solver = BruteForce::new();
        let is_solutions = solver.find_best(is_problem);

        // Max packing = 2 (sets 0 and 1)
        for sol in &is_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 2);
        }
    }

    #[test]
    fn test_roundtrip_is_sp_is() {
        let original = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let original_solutions = solver.find_best(&original);

        // IS -> SP -> IS
        let reduction1 = ReduceTo::<SetPacking<i32>>::reduce_to(&original);
        let sp = reduction1.target_problem().clone();
        let reduction2: ReductionSPToIS<i32> = sp.reduce_to();
        let roundtrip = reduction2.target_problem();

        let roundtrip_solutions = solver.find_best(roundtrip);

        // Solutions should have same objective value
        let orig_size: usize = original_solutions[0].iter().sum();
        let rt_size: usize = roundtrip_solutions[0].iter().sum();
        assert_eq!(orig_size, rt_size);
    }

    #[test]
    fn test_weighted_reduction() {
        let is_problem =
            IndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 20, 30]);
        let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&is_problem);
        let sp_problem = reduction.target_problem();

        // Weights should be preserved
        assert_eq!(sp_problem.weights_ref(), &vec![10, 20, 30]);
    }

    #[test]
    fn test_empty_graph() {
        // No edges means all sets are empty (or we need to handle it)
        let is_problem = IndependentSet::<i32>::new(3, vec![]);
        let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&is_problem);
        let sp_problem = reduction.target_problem();

        // All sets should be empty (no edges to include)
        assert_eq!(sp_problem.num_sets(), 3);

        let solver = BruteForce::new();
        let solutions = solver.find_best(sp_problem);

        // With no overlaps, we can select all sets
        assert_eq!(solutions[0].iter().sum::<usize>(), 3);
    }

    #[test]
    fn test_disjoint_sets() {
        // Completely disjoint sets
        let sets = vec![vec![0], vec![1], vec![2]];
        let sp_problem = SetPacking::<i32>::new(sets);
        let reduction: ReductionSPToIS<i32> = sp_problem.reduce_to();
        let is_problem = reduction.target_problem();

        // No edges in the intersection graph
        assert_eq!(is_problem.num_edges(), 0);
    }
}
