//! Reductions between IndependentSet and VertexCovering problems.
//!
//! These problems are complements: a set S is an independent set iff V\S is a vertex cover.

use crate::models::graph::{IndependentSet, VertexCovering};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing IndependentSet to VertexCovering.
#[derive(Debug, Clone)]
pub struct ReductionISToVC<W> {
    target: VertexCovering<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionISToVC<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Source = IndependentSet<W>;
    type Target = VertexCovering<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    /// If v is in the independent set (1), it's NOT in the vertex cover (0).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.iter().map(|&x| 1 - x).collect()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<VertexCovering<W>> for IndependentSet<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32>,
{
    type Result = ReductionISToVC<W>;

    fn reduce_to(&self) -> Self::Result {
        let target = VertexCovering::with_weights(
            self.num_vertices(),
            self.edges(),
            self.weights_ref().clone(),
        );
        ReductionISToVC {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing VertexCovering to IndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToIS<W> {
    target: IndependentSet<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionVCToIS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Source = VertexCovering<W>;
    type Target = IndependentSet<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.iter().map(|&x| 1 - x).collect()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<IndependentSet<W>> for VertexCovering<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32>,
{
    type Result = ReductionVCToIS<W>;

    fn reduce_to(&self) -> Self::Result {
        let target = IndependentSet::with_weights(
            self.num_vertices(),
            self.edges(),
            self.weights_ref().clone(),
        );
        ReductionVCToIS {
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
    fn test_is_to_vc_reduction() {
        // Triangle graph: max IS = 1, min VC = 2
        let is_problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction = ReduceTo::<VertexCovering<i32>>::reduce_to(&is_problem);
        let vc_problem = reduction.target_problem();

        // Solve the VC problem
        let solver = BruteForce::new();
        let vc_solutions = solver.find_best(vc_problem);

        // Extract back to IS solutions
        let is_solutions: Vec<_> = vc_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Verify IS solutions are valid and optimal
        for sol in &is_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 1, "Max IS in triangle should be 1");
        }
    }

    #[test]
    fn test_vc_to_is_reduction() {
        // Path graph 0-1-2: min VC = 1 (just vertex 1), max IS = 2 (vertices 0 and 2)
        let vc_problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let reduction: ReductionVCToIS<i32> = vc_problem.reduce_to();
        let is_problem = reduction.target_problem();

        let solver = BruteForce::new();
        let is_solutions = solver.find_best(is_problem);

        let vc_solutions: Vec<_> = is_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Verify VC solutions
        for sol in &vc_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 1, "Min VC in path should be 1");
        }
    }

    #[test]
    fn test_roundtrip_is_vc_is() {
        let original = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let original_solutions = solver.find_best(&original);

        // IS -> VC -> IS
        let reduction1 = ReduceTo::<VertexCovering<i32>>::reduce_to(&original);
        let vc = reduction1.target_problem().clone();
        let reduction2: ReductionVCToIS<i32> = vc.reduce_to();
        let roundtrip = reduction2.target_problem();

        let roundtrip_solutions = solver.find_best(roundtrip);

        // Solutions should have same objective value
        let orig_size: usize = original_solutions[0].iter().sum();
        let rt_size: usize = roundtrip_solutions[0].iter().sum();
        assert_eq!(orig_size, rt_size);
    }

    #[test]
    fn test_weighted_reduction() {
        // Test with weighted problems
        let is_problem =
            IndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 20, 30]);
        let reduction = ReduceTo::<VertexCovering<i32>>::reduce_to(&is_problem);
        let vc_problem = reduction.target_problem();

        // Weights should be preserved
        assert_eq!(vc_problem.weights_ref(), &vec![10, 20, 30]);
    }

    #[test]
    fn test_source_and_target_size() {
        let is_problem = IndependentSet::<i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
        let reduction = ReduceTo::<VertexCovering<i32>>::reduce_to(&is_problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vertices"), Some(5));
        assert_eq!(target_size.get("num_vertices"), Some(5));
    }
}
