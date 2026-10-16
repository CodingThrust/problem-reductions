//! Reduction from MinimumMaximalMatching (on a bipartite graph) to
//! MaximumAchromaticNumber.
//!
//! Classical reduction of Yannakakis and Gavril (1980) establishing
//! NP-completeness of Achromatic Number (G&J GT5). For a bipartite graph `G`,
//! the identity `ach(complement(G)) = |V| - mm(G)` holds, where `mm(G)` is the
//! minimum maximal matching size of `G`. The decision-version correspondence
//! used in the reduction is `(G, K) -> (complement(G), |V| - K)`.

use crate::models::graph::{MaximumAchromaticNumber, MinimumMaximalMatching};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{BipartiteGraph, Graph, SimpleGraph};

/// Result of reducing `MinimumMaximalMatching<BipartiteGraph>` to
/// `MaximumAchromaticNumber<SimpleGraph>`.
///
/// Stores the target problem along with the source edge list (in unified vertex
/// coordinates) so that `extract_solution` can map a target coloring back to a
/// maximal matching of the source graph.
#[derive(Debug, Clone)]
pub struct ReductionMMMToAchromatic {
    target: MaximumAchromaticNumber<SimpleGraph>,
    /// Source edges in unified vertex coordinates, in the same order as
    /// `source.graph().edges()` (which determines the source solution's edge order).
    source_edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionMMMToAchromatic {
    type Source = MinimumMaximalMatching<BipartiteGraph>;
    type Target = MaximumAchromaticNumber<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a maximal matching of the source graph from an achromatic
    /// coloring of `complement(G)`.
    ///
    /// Each color class of size exactly 2 in `complement(G)` is an independent
    /// set there and thus a clique in `G`. For bipartite `G` such a clique has
    /// size 2, i.e., a source edge. A source edge `(u, v)` belongs to the
    /// extracted matching iff `u` and `v` share a color, which we detect in a
    /// single pass over `source_edges`.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            self.source_edges
                .iter()
                .map(|&(u, v)| target_solution[u] == target_solution[v])
                .collect()
        })
    }
}

#[reduction(
    transform = exact {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumAchromaticNumber<SimpleGraph>> for MinimumMaximalMatching<BipartiteGraph> {
    type Result = ReductionMMMToAchromatic;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let source_edges = self.graph().edges();

        let source_edge_set: std::collections::HashSet<(usize, usize)> = source_edges
            .iter()
            .map(|&(u, v)| if u < v { (u, v) } else { (v, u) })
            .collect();

        // Complement graph: all non-edges of G become edges of H.
        let mut complement_edges = Vec::new();
        for u in 0..n {
            for v in (u + 1)..n {
                if !source_edge_set.contains(&(u, v)) {
                    complement_edges.push((u, v));
                }
            }
        }

        let target = MaximumAchromaticNumber::new(SimpleGraph::new(n, complement_edges));

        Ok(ReductionMMMToAchromatic {
            target,
            source_edges,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimummaximalmatching_to_maximumachromaticnumber",
        build: || {
            // Include suboptimal maximal matchings so the example exercises optimization.
            let source = MinimumMaximalMatching::new(BipartiteGraph::new(
                3,
                2,
                vec![(0, 0), (1, 0), (1, 1), (2, 0)],
            ));
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MaximumAchromaticNumber<SimpleGraph>,
            >(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, false, false]),
                    target_config: serde_json::json!(vec![1, 0, 3, 0, 2]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummaximalmatching_maximumachromaticnumber.rs"]
mod tests;
