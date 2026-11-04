//! Unit vertex cover to maximum-leaf spanning tree via a root and subdivisions.

use crate::models::{Decision, MaximumLeafSpanningTree, MinimumVertexCover};
use crate::rules::{ReduceTo, ReductionError, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

type Source = Decision<MinimumVertexCover<SimpleGraph, One>>;
type Target = Decision<MaximumLeafSpanningTree<SimpleGraph>>;

/// A target decision instance and the original vertex prefix used in extraction.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMinimumVertexCoverToDecisionMaximumLeafSpanningTree {
    target: Target,
    source_vertices: usize,
}

impl ReductionResult for ReductionDecisionMinimumVertexCoverToDecisionMaximumLeafSpanningTree {
    type Source = Source;
    type Target = Target;

    fn target_problem(&self) -> &Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.0,
            "target selection is not a spanning tree meeting the leaf bound",
        )?;
        let mut degrees = vec![0_usize; self.target.num_vertices()];
        for ((u, v), selected) in self
            .target
            .inner()
            .graph()
            .edges()
            .into_iter()
            .zip(solution)
        {
            if *selected {
                degrees[u] += 1;
                degrees[v] += 1;
            }
        }
        Ok(degrees[..self.source_vertices]
            .iter()
            .map(|&degree| degree >= 2)
            .collect())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult
    for ReductionDecisionMinimumVertexCoverToDecisionMaximumLeafSpanningTree
{
}

fn target_sizes(n: usize, m: usize) -> Result<(usize, usize), ReductionError> {
    let overflow =
        || ReductionError::integer_overflow::<Source, Target>("counting target vertices and edges");
    let vertices = n
        .checked_add(m)
        .and_then(|value| value.checked_add(3))
        .ok_or_else(overflow)?;
    let edges = m
        .checked_mul(2)
        .and_then(|value| value.checked_add(n))
        .and_then(|value| value.checked_add(2))
        .ok_or_else(overflow)?;
    Ok((vertices, edges))
}

fn leaf_bound(n: usize, vertices: usize, k: i64) -> Result<i64, ReductionError> {
    let n = <Source as ReduceTo<Target>>::exact_i64(n, "converting the cover-size limit")?;
    let non_root =
        <Source as ReduceTo<Target>>::exact_i64(vertices - 1, "converting the target tree size")?;
    let normalized = k.clamp(-1, n);
    non_root.checked_sub(normalized).ok_or_else(|| {
        ReductionError::integer_overflow::<Source, Target>("computing the target leaf bound")
    })
}

#[crate::reduction(transform = exact {
    num_vertices = "num_vertices + num_edges + 3",
    num_edges = "num_vertices + 2 * num_edges + 2",
})]
impl ReduceTo<Decision<MaximumLeafSpanningTree<SimpleGraph>>>
    for Decision<MinimumVertexCover<SimpleGraph, One>>
{
    type Result = ReductionDecisionMinimumVertexCoverToDecisionMaximumLeafSpanningTree;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let graph = self.inner().graph();
        let n = graph.num_vertices();
        let m = graph.num_edges();
        let (vertices, edge_count) = target_sizes(n, m)?;
        let bound = leaf_bound(n, vertices, *self.bound())?;
        let root = vertices - 3;
        let mut edges = Vec::with_capacity(edge_count);
        edges.extend((0..n).map(|v| (root, v)));
        for (j, (u, v)) in graph.edges().into_iter().enumerate() {
            let subdivision = n + j;
            edges.extend([(u, subdivision), (v, subdivision)]);
        }
        edges.extend([(root, root + 1), (root, root + 2)]);
        let graph = SimpleGraph::try_new(vertices, edges)
            .map_err(<Self as ReduceTo<Target>>::target_construction)?;
        let inner = MaximumLeafSpanningTree::try_new(graph)
            .map_err(<Self as ReduceTo<Target>>::target_construction)?;
        Ok(
            ReductionDecisionMinimumVertexCoverToDecisionMaximumLeafSpanningTree {
                target: Decision::new(inner, bound),
                source_vertices: n,
            },
        )
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionminimumvertexcover_to_decisionmaximumleafspanningtree",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Target>(
                Decision::new(
                    MinimumVertexCover::new(SimpleGraph::cycle(4), vec![One; 4]),
                    2,
                ),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([true, false, true, false]),
                    target_config: serde_json::json!([
                        true, true, true, true, true, false, false, true, true, false, false, true,
                        true, true,
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionminimumvertexcover_decisionmaximumleafspanningtree.rs"]
mod tests;
