//! Reduction from KColoring (K3) to TwoDimensionalConsecutiveSets.
//!
//! Given a graph G = (V, E) with |V| = n and |E| = m, construct:
//!
//! - Alphabet: V union {d_e : e in E}, size n + m
//! - For each edge e = {u, v}, one subset {u, v, d_e} of size 3
//!
//! A valid 3-coloring corresponds to a partition into 3 groups where each
//! edge-subset spans 3 consecutive groups with one element per group.
//!
//! Reference: Garey & Johnson, Appendix A4.2, p.230 (Lipski 1977).

use crate::models::graph::KColoring;
use crate::models::set::TwoDimensionalConsecutiveSets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::K3;

/// Result of reducing KColoring<K3> to TwoDimensionalConsecutiveSets.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToTDCS {
    target: TwoDimensionalConsecutiveSets,
    /// Number of vertices in the source graph.
    num_vertices: usize,
    /// Edges of the source graph (stored for solution extraction).
    edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionKColoringToTDCS {
    type Source = KColoring<K3, SimpleGraph>;
    type Target = TwoDimensionalConsecutiveSets;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a 3-coloring from a TwoDimensionalConsecutiveSets solution.
    ///
    /// The target solution assigns each alphabet symbol to a group index.
    /// The first `num_vertices` symbols correspond to graph vertices,
    /// so their group assignments directly give a valid 3-coloring
    /// (after remapping to colors 0, 1, 2).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // The target solution is config[symbol] = group_index.
        // Vertex symbols are indices 0..num_vertices.
        // We need to remap the group indices to colors 0, 1, 2.
        // Since there are only 3 groups used, we can directly use the group indices
        // if they are already 0, 1, 2. But the target may use any 3 labels from
        // 0..alphabet_size, so we need to compress them to 0..2.

        let vertex_groups: Vec<usize> = target_solution[..self.num_vertices].to_vec();

        // Collect all distinct group indices used by all symbols and compress to 0..k-1
        let mut used_groups: Vec<usize> = target_solution.to_vec();
        used_groups.sort();
        used_groups.dedup();

        let mut group_to_color = vec![0usize; self.num_vertices + self.edges.len()];
        for (color, &group) in used_groups.iter().enumerate() {
            if group < group_to_color.len() {
                group_to_color[group] = color;
            }
        }

        vertex_groups.iter().map(|&g| group_to_color[g]).collect()
    }
}

#[reduction(
    overhead = {
        alphabet_size = "num_vertices + num_edges",
        num_subsets = "num_edges",
    }
)]
impl ReduceTo<TwoDimensionalConsecutiveSets> for KColoring<K3, SimpleGraph> {
    type Result = ReductionKColoringToTDCS;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges: Vec<(usize, usize)> = self.graph().edges();
        let m = edges.len();
        let alphabet_size = n + m;

        // For each edge e_i = {u, v}, create subset {u, v, n + i}
        let subsets: Vec<Vec<usize>> = edges
            .iter()
            .enumerate()
            .map(|(i, &(u, v))| vec![u, v, n + i])
            .collect();

        let target = TwoDimensionalConsecutiveSets::new(alphabet_size, subsets);

        ReductionKColoringToTDCS {
            target,
            num_vertices: n,
            edges,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::traits::Problem;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_twodimensionalconsecutivesets",
        build: || {
            // Small 3-colorable graph: triangle with pendant
            // 0 -- 1 -- 2 -- 0, plus 2 -- 3
            // 3-coloring: 0->0, 1->1, 2->2, 3->0
            let source =
                KColoring::<K3, _>::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2), (2, 3)]));
            let reduction = <KColoring<K3, SimpleGraph> as ReduceTo<
                TwoDimensionalConsecutiveSets,
            >>::reduce_to(&source);
            let target = reduction.target_problem();

            // Source coloring: 0->0, 1->1, 2->2, 3->0
            // Target config: vertex 0->group 0, vertex 1->group 1, vertex 2->group 2, vertex 3->group 0
            // Dummies:
            //   d_{0,1} (symbol 4): colors used {0,1}, dummy->group 2
            //   d_{1,2} (symbol 5): colors used {1,2}, dummy->group 0
            //   d_{0,2} (symbol 6): colors used {0,2}, dummy->group 1
            //   d_{2,3} (symbol 7): colors used {2,0}, dummy->group 1
            let source_config = vec![0, 1, 2, 0];
            let target_config = vec![0, 1, 2, 0, 2, 0, 1, 1];

            // Verify the target config is valid
            assert!(
                target.evaluate(&target_config).0,
                "canonical example target config must be valid"
            );

            crate::example_db::specs::assemble_rule_example(
                &source,
                target,
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_twodimensionalconsecutivesets.rs"]
mod tests;
