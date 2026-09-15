//! Reduction from HamiltonianPathBetweenTwoVertices to LongestPath.
//!
//! A Hamiltonian s-t path in G has length n-1 edges (the maximum possible for
//! any simple path). Setting all edge lengths to unit weight and the same
//! source/target vertices, the longest path of length n-1 exactly corresponds
//! to a Hamiltonian s-t path.

use crate::models::decision::Decision;
use crate::models::graph::{HamiltonianPathBetweenTwoVertices, LongestPath};
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing HamiltonianPathBetweenTwoVertices to LongestPath.
#[derive(Debug, Clone)]
pub struct ReductionHPBTVToLP {
    target: Decision<LongestPath<SimpleGraph, One>>,
}

impl ReductionResult for ReductionHPBTVToLP {
    type Source = HamiltonianPathBetweenTwoVertices<SimpleGraph>;
    type Target = Decision<LongestPath<SimpleGraph, One>>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a vertex-permutation solution from an edge-selection solution.
    ///
    /// The target solution is a binary vector over edges. We walk the selected
    /// edges from the source vertex to reconstruct the vertex ordering.
    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| self.map_solution(solution))
    }
}

impl ReductionHPBTVToLP {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        let mut adjacency = vec![Vec::new(); self.target.inner().num_vertices()];
        for (&selected, (u, v)) in target_solution
            .iter()
            .zip(self.target.inner().graph().edges())
        {
            if selected {
                adjacency[u].push(v);
                adjacency[v].push(u);
            }
        }

        // Target feasibility guarantees a single simple path with these endpoints.
        // Its certified n-1 edges visit every vertex; walking away from the
        // previous vertex terminates at the target without repetitions.
        let mut current = self.target.inner().source_vertex();
        let mut previous = None;
        let mut path = Vec::with_capacity(self.target.inner().num_vertices());
        path.push(current);
        while let Some(&next) = adjacency[current]
            .iter()
            .find(|&&neighbor| Some(neighbor) != previous)
        {
            previous = Some(current);
            current = next;
            path.push(current);
        }
        Ok(path)
    }
}

#[reduction(
    transform = exact {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    })]
impl ReduceTo<Decision<LongestPath<SimpleGraph, One>>>
    for HamiltonianPathBetweenTwoVertices<SimpleGraph>
{
    type Result = ReductionHPBTVToLP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let graph = self.graph().clone();
        let num_edges = graph.num_edges();
        let edge_lengths = vec![One; num_edges];

        let target = LongestPath::new(
            graph,
            edge_lengths,
            self.source_vertex(),
            self.target_vertex(),
        );

        Ok(ReductionHPBTVToLP {
            target: Decision::new(
                target,
                <Self as ReduceTo<Decision<LongestPath<SimpleGraph, One>>>>::exact_i64(
                    self.num_vertices() - 1,
                    "encoding the path bound",
                )?,
            ),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpathbetweentwovertices_to_longestpath",
        build: || {
            // Path graph 0-1-2-3-4 with s=0, t=4
            let source = HamiltonianPathBetweenTwoVertices::new(
                SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                0,
                4,
            );
            crate::example_db::specs::rule_example_with_witness::<
                _,
                Decision<LongestPath<SimpleGraph, One>>,
            >(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![0, 1, 2, 3, 4]),
                    target_config: serde_json::json!(vec![true, true, true, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpathbetweentwovertices_longestpath.rs"]
mod tests;
