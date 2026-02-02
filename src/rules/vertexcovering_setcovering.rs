//! Reduction from VertexCovering to SetCovering.
//!
//! Each vertex becomes a set containing the edges it covers.
//! The universe is the set of all edges (labeled 0 to num_edges-1).

use crate::models::graph::VertexCovering;
use crate::models::set::SetCovering;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing VertexCovering to SetCovering.
#[derive(Debug, Clone)]
pub struct ReductionVCToSC<W> {
    target: SetCovering<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionVCToSC<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = VertexCovering<W>;
    type Target = SetCovering<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: variables correspond 1:1.
    /// Vertex i in VC corresponds to set i in SC.
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

impl<W> ReduceTo<SetCovering<W>> for VertexCovering<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionVCToSC<W>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.edges();
        let num_edges = edges.len();
        let num_vertices = self.num_vertices();

        // For each vertex, create a set of edge indices that it covers.
        // An edge (u, v) with index i is covered by vertex j if j == u or j == v.
        let sets: Vec<Vec<usize>> = (0..num_vertices)
            .map(|vertex| {
                edges
                    .iter()
                    .enumerate()
                    .filter(|(_, (u, v))| *u == vertex || *v == vertex)
                    .map(|(edge_idx, _)| edge_idx)
                    .collect()
            })
            .collect();

        let target = SetCovering::with_weights(num_edges, sets, self.weights_ref().clone());

        ReductionVCToSC {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};
    use crate::traits::ConstraintSatisfactionProblem;

    #[test]
    fn test_vc_to_sc_basic() {
        // Path graph 0-1-2 with edges (0,1) and (1,2)
        // Vertex 0 covers edge 0
        // Vertex 1 covers edges 0 and 1
        // Vertex 2 covers edge 1
        let vc_problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_problem = reduction.target_problem();

        // Check the sets are constructed correctly
        assert_eq!(sc_problem.universe_size(), 2); // 2 edges
        assert_eq!(sc_problem.num_sets(), 3); // 3 vertices

        // Set 0 (vertex 0): should contain edge 0
        assert_eq!(sc_problem.get_set(0), Some(&vec![0]));
        // Set 1 (vertex 1): should contain edges 0 and 1
        assert_eq!(sc_problem.get_set(1), Some(&vec![0, 1]));
        // Set 2 (vertex 2): should contain edge 1
        assert_eq!(sc_problem.get_set(2), Some(&vec![1]));
    }

    #[test]
    fn test_vc_to_sc_triangle() {
        // Triangle graph: 3 vertices, 3 edges
        // Edge indices: (0,1)->0, (1,2)->1, (0,2)->2
        let vc_problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_problem = reduction.target_problem();

        assert_eq!(sc_problem.universe_size(), 3);
        assert_eq!(sc_problem.num_sets(), 3);

        // Verify each vertex covers exactly 2 edges
        for i in 0..3 {
            let set = sc_problem.get_set(i).unwrap();
            assert_eq!(set.len(), 2);
        }
    }

    #[test]
    fn test_vc_to_sc_solution_extraction() {
        let vc_problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_problem = reduction.target_problem();

        // Solve the SetCovering problem
        let solver = BruteForce::new();
        let sc_solutions = solver.find_best(sc_problem);

        // Extract solutions back to VertexCovering
        let vc_solutions: Vec<_> = sc_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Verify extracted solutions are valid vertex covers
        for sol in &vc_solutions {
            assert!(vc_problem.solution_size(sol).is_valid);
        }

        // The minimum should be selecting just vertex 1 (covers both edges)
        let min_size: usize = vc_solutions[0].iter().sum();
        assert_eq!(min_size, 1);
    }

    #[test]
    fn test_vc_to_sc_optimality_preservation() {
        // Test that optimal solutions are preserved through reduction
        let vc_problem = VertexCovering::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();

        // Solve VC directly
        let direct_solutions = solver.find_best(&vc_problem);
        let direct_size = direct_solutions[0].iter().sum::<usize>();

        // Solve via reduction
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_solutions = solver.find_best(reduction.target_problem());
        let reduced_solutions: Vec<_> = sc_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();
        let reduced_size = reduced_solutions[0].iter().sum::<usize>();

        // Optimal sizes should match
        assert_eq!(direct_size, reduced_size);
    }

    #[test]
    fn test_vc_to_sc_weighted() {
        // Weighted problem: weights should be preserved
        let vc_problem = VertexCovering::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 1, 10]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_problem = reduction.target_problem();

        // Weights should be preserved
        assert_eq!(sc_problem.weights(), vec![10, 1, 10]);

        // Solve both ways
        let solver = BruteForce::new();
        let vc_solutions = solver.find_best(&vc_problem);
        let sc_solutions = solver.find_best(sc_problem);

        // Both should select vertex 1 (weight 1)
        assert_eq!(vc_solutions[0], vec![0, 1, 0]);
        assert_eq!(sc_solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_vc_to_sc_empty_graph() {
        // Graph with no edges
        let vc_problem = VertexCovering::<i32>::new(3, vec![]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_problem = reduction.target_problem();

        assert_eq!(sc_problem.universe_size(), 0);
        assert_eq!(sc_problem.num_sets(), 3);

        // All sets should be empty
        for i in 0..3 {
            assert!(sc_problem.get_set(i).unwrap().is_empty());
        }
    }

    #[test]
    fn test_vc_to_sc_source_target_size() {
        let vc_problem = VertexCovering::<i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vertices"), Some(5));
        assert_eq!(source_size.get("num_edges"), Some(4));
        assert_eq!(target_size.get("universe_size"), Some(4)); // edges become universe
        assert_eq!(target_size.get("num_sets"), Some(5)); // vertices become sets
    }

    #[test]
    fn test_vc_to_sc_star_graph() {
        // Star graph: center vertex 0 connected to all others
        // Edges: (0,1), (0,2), (0,3)
        let vc_problem = VertexCovering::<i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_problem = reduction.target_problem();

        // Vertex 0 should cover all 3 edges
        assert_eq!(sc_problem.get_set(0), Some(&vec![0, 1, 2]));
        // Other vertices cover only 1 edge each
        assert_eq!(sc_problem.get_set(1), Some(&vec![0]));
        assert_eq!(sc_problem.get_set(2), Some(&vec![1]));
        assert_eq!(sc_problem.get_set(3), Some(&vec![2]));

        // Minimum cover should be just vertex 0
        let solver = BruteForce::new();
        let solutions = solver.find_best(&vc_problem);
        assert_eq!(solutions[0], vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_vc_to_sc_all_solutions_valid() {
        // Ensure all solutions extracted from SC are valid VC solutions
        let vc_problem = VertexCovering::<i32>::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]);
        let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&vc_problem);
        let sc_problem = reduction.target_problem();

        let solver = BruteForce::new();
        let sc_solutions = solver.find_best(sc_problem);

        for sc_sol in &sc_solutions {
            let vc_sol = reduction.extract_solution(sc_sol);
            let sol_size = vc_problem.solution_size(&vc_sol);
            assert!(
                sol_size.is_valid,
                "Extracted solution {:?} should be valid",
                vc_sol
            );
        }
    }
}

// Register reduction with inventory for auto-discovery
use crate::poly;
use crate::rules::registry::{ReductionEntry, ReductionOverhead};

inventory::submit! {
    ReductionEntry {
        source_name: "VertexCovering",
        target_name: "SetCovering",
        source_graph: "SimpleGraph",
        target_graph: "SetSystem",
        source_weighted: false,
        target_weighted: false,
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_sets", poly!(num_vertices)),
            ("num_elements", poly!(num_edges)),
        ]),
    }
}
