//! Reduction from KColoring (K3) to TwoDimensionalConsecutiveSets.
//!
//! Given a graph G = (V, E) with |V| = n and |E| = m, construct:
//!
//! - Alphabet: V union {d_e : e in E}, size n + m
//! - For each edge e = {u, v}, one subset {u, v, d_e} of size 3
//!
//! A valid 3-coloring extends to at most three groups. Conversely, each edge
//! triple spans three consecutive groups. After retaining only vertex groups,
//! endpoint ranks differ by one or two, so rank modulo three gives a coloring.
//! Empty sources map to a one-symbol YES instance; loops to a fixed NO instance.
//!
//! Definition: Lipski, CSL Report T-67 (1978), Problem 5; see the paper for proof.

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
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target grouping is not a consecutive-set partition",
            ));
        }

        Ok({
            // The target solution is config[symbol] = group_index.
            // Vertex symbols are indices 0..num_vertices.
            // Removing dummy-only groups cannot increase the separation of
            // edge endpoints: their distinct ranks differ by one or two.
            // Taking these ranks modulo three therefore preserves every edge.

            let vertex_groups = &target_solution[..self.num_vertices];

            // Collect distinct group indices used by vertices and map to 0..k-1
            let mut used: Vec<usize> = vertex_groups.to_vec();
            used.sort();
            used.dedup();

            let group_to_color: std::collections::HashMap<usize, usize> = used
                .into_iter()
                .enumerate()
                .map(|(color, group)| (group, color % 3))
                .collect();

            vertex_groups.iter().map(|&g| group_to_color[&g]).collect()
        })
    }
}

#[reduction(
    transform = upper_bound {
        alphabet_size = "num_vertices + num_edges + 3",
        num_subsets = "num_edges + 3",
    }
)]
impl ReduceTo<TwoDimensionalConsecutiveSets> for KColoring<K3, SimpleGraph> {
    type Result = ReductionKColoringToTDCS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let edges: Vec<(usize, usize)> = self.graph().edges();
        let m = edges.len();
        let (alphabet_size, subsets) = if edges.iter().any(|&(u, v)| u == v) {
            // Three symbols cannot occupy pairwise consecutive distinct groups:
            // the first and last groups are not adjacent. This is a fixed NO.
            (3, vec![vec![0, 1], vec![1, 2], vec![0, 2]])
        } else {
            // Native node and edge Vec allocation bounds make n + m fit usize.
            let alphabet_size = n + m;
            let subsets = edges
                .iter()
                .enumerate()
                .map(|(i, &(u, v))| vec![u, v, n + i])
                .collect();
            // The empty graph is colorable, while the target requires a
            // positive alphabet. One unconstrained symbol preserves YES.
            (alphabet_size.max(1), subsets)
        };
        let target = TwoDimensionalConsecutiveSets::try_new(alphabet_size, subsets).map_err(
            crate::rules::ReductionError::construction::<Self, TwoDimensionalConsecutiveSets>,
        )?;

        Ok(ReductionKColoringToTDCS {
            target,
            num_vertices: n,
        })
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
            >>::reduce_to(&source)
            .expect("reduction should succeed");
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
                target
                    .evaluate(&target_config)
                    .expect("canonical target evaluation must succeed")
                    .0,
                "canonical example target config must be valid"
            );

            crate::example_db::specs::assemble_rule_example(
                &source,
                target,
                vec![SolutionPair {
                    source_config: serde_json::to_value(source_config)
                        .expect("solution serialization must succeed"),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_twodimensionalconsecutivesets.rs"]
mod tests;
