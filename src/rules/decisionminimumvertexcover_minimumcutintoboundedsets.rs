//! Reduction from Decision Minimum Vertex Cover to Minimum Cut Into Bounded Sets.
//!
//! A cover of size at most `k` exists exactly when the graph has an independent set of size
//! `T = n - k`, i.e. when the complement graph `H` has a clique of size `T` (`T` is clamped to
//! `0..=n + 1`). Let `I` be the original vertices on the sink's side. The target consists of
//!
//! * the complement edges `{i, j}` of `H`, weight 1;
//! * a source `s = n` and a sink `t = n + 1`, with the edges `{i, s}` of weight 1 and `{i, t}` of
//!   weight `deg_H(i) + 2` for every original vertex `i`;
//! * `q = n - T` padding vertices, each joined to `t` by an edge of weight `M = Phi + 1`, where
//!   `Phi = 2 |E(H)| + 2 n`;
//! * the side bound `n + 1`.
//!
//! A feasible assignment cuts `Phi - 2 e_H(I) - |I| + M p`, where `e_H(I)` counts the edges of `H`
//! inside `I` and `p` the padding vertices on the source's side. Since `M` exceeds the threshold
//! `Phi - T^2`, no padding vertex is on the source's side within the threshold. The padding then
//! fills the sink's side and the side bound gives `|I| <= T`; with `2 e_H(I) + |I| <= |I|^2` the
//! cut is at most the threshold exactly when `|I| = T` and `I` is a clique of `H`, i.e. when the
//! original vertices outside `I` form a cover of size `n - T <= k`.

use crate::models::decision::Decision;
use crate::models::graph::{MinimumCutIntoBoundedSets, MinimumVertexCover};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{Min, One, Or};

type Source = Decision<MinimumVertexCover<SimpleGraph, One>>;
type Target = MinimumCutIntoBoundedSets<SimpleGraph, i64>;

/// Result of reducing Decision Minimum Vertex Cover to Minimum Cut Into Bounded Sets.
///
/// The source is a YES instance exactly when the optimal cut of the target is at most
/// `threshold`.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMinimumVertexCoverToMinimumCutIntoBoundedSets {
    target: Target,
    source_num_vertices: usize,
    threshold: i64,
}

impl ReductionResult for ReductionDecisionMinimumVertexCoverToMinimumCutIntoBoundedSets {
    type Source = Source;
    type Target = Target;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| crate::rules::AggregateReductionResult::extract_value(self, value).0,
            "target partition is infeasible or cuts more than the threshold, so it does not certify a vertex cover within the source bound",
        )?;
        // The cover is the set of original vertices on the source's side (`false`); the
        // source, the sink and the padding vertices follow the original vertices.
        Ok(target_solution[..self.source_num_vertices]
            .iter()
            .map(|on_sink_side| !on_sink_side)
            .collect())
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult
    for ReductionDecisionMinimumVertexCoverToMinimumCutIntoBoundedSets
{
    type Source = Decision<MinimumVertexCover<SimpleGraph, One>>;
    type Target = MinimumCutIntoBoundedSets<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: Min<i64>) -> Or {
        Or(value.0.is_some_and(|cut| cut <= self.threshold))
    }
}

/// Counts and numbers of the construction, computed before anything is allocated.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Parameters {
    /// `T`: the independent-set size sought, `n - bound` clamped to `0..=n + 1`.
    independent_size: usize,
    /// `q = n - T` (0 when `T = n + 1`): the number of padding vertices.
    padding: usize,
    /// `n + 2 + q`.
    target_vertices: usize,
    /// `|E(H)| = n (n - 1) / 2 - m`: the number of complement edges.
    complement_edges: usize,
    /// `|E(H)| + 2 n + q`.
    target_edges: usize,
    /// `Phi + 1`, the weight of a padding edge.
    heavy_weight: i64,
    /// `Phi - T^2`.
    threshold: i64,
}

/// Compute the construction's parameters with checked arithmetic. Besides the sizes, this checks
/// that every weight and the total weight of all edges (which bounds every cut) fit in `i64`, so
/// no evaluation of the target can overflow.
fn construction_parameters(
    n: usize,
    m: usize,
    bound: i64,
) -> Result<Parameters, crate::rules::ReductionError> {
    let overflow = || {
        crate::rules::ReductionError::integer_overflow::<Source, Target>(
            "encoding the bounded-sets construction parameters",
        )
    };
    let n_plus_one = n.checked_add(1).ok_or_else(overflow)?;
    // T = n - bound clamped to 0..=n + 1; a negative bound gives n + 1.
    let independent_size = if bound < 0 {
        n_plus_one
    } else {
        // A bound beyond `usize` exceeds every vertex count, so it saturates.
        n.saturating_sub(usize::try_from(bound).unwrap_or(usize::MAX))
    };
    let padding = n.saturating_sub(independent_size);
    let target_vertices = n_plus_one
        .checked_add(1)
        .and_then(|v| v.checked_add(padding))
        .ok_or_else(overflow)?;
    let pairs = n.checked_mul(n.saturating_sub(1)).ok_or_else(overflow)? / 2;
    let complement_edges = pairs.checked_sub(m).ok_or_else(overflow)?;
    let target_edges = n
        .checked_mul(2)
        .and_then(|e| e.checked_add(complement_edges))
        .and_then(|e| e.checked_add(padding))
        .ok_or_else(overflow)?;

    let to_i64 = |value: usize| {
        <Source as ReduceTo<Target>>::exact_i64(value, "encoding a construction count")
    };
    let n_i64 = to_i64(n)?;
    let complement_i64 = to_i64(complement_edges)?;
    // Phi = 2 |E(H)| + 2 n.
    let phi = complement_i64
        .checked_mul(2)
        .and_then(|p| p.checked_add(n_i64.checked_mul(2)?))
        .ok_or_else(overflow)?;
    let heavy_weight = phi.checked_add(1).ok_or_else(overflow)?;
    let t_i64 = to_i64(independent_size)?;
    let threshold = t_i64
        .checked_mul(t_i64)
        .and_then(|square| phi.checked_sub(square))
        .ok_or_else(overflow)?;
    // The total weight |E(H)| + n + (2 |E(H)| + 2 n) + q M bounds every cut, so evaluating the
    // target cannot overflow. Every other weight is at most n + 1 <= M.
    to_i64(padding)?
        .checked_mul(heavy_weight)
        .and_then(|padding_weight| {
            padding_weight
                .checked_add(phi)?
                .checked_add(n_i64)?
                .checked_add(complement_i64)
        })
        .ok_or_else(overflow)?;
    Ok(Parameters {
        independent_size,
        padding,
        target_vertices,
        complement_edges,
        target_edges,
        heavy_weight,
        threshold,
    })
}

#[reduction(
    transform = upper_bound {
        num_vertices = "2 * num_vertices + 2",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges + 3 * num_vertices",
    }
)]
impl ReduceTo<MinimumCutIntoBoundedSets<SimpleGraph, i64>>
    for Decision<MinimumVertexCover<SimpleGraph, One>>
{
    type Result = ReductionDecisionMinimumVertexCoverToMinimumCutIntoBoundedSets;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let graph = self.inner().graph();
        let n = graph.num_vertices();
        let parameters = construction_parameters(n, graph.num_edges(), *self.bound())?;
        let source_vertex = n;
        let sink_vertex = n + 1;

        // Complement edges, each of weight 1; `degree[i]` is the degree of `i` in `H`.
        let mut edges = Vec::with_capacity(parameters.target_edges);
        let mut weights = Vec::with_capacity(parameters.target_edges);
        let mut degree = vec![0usize; n];
        for j in 0..n {
            for i in 0..j {
                if !graph.has_edge(i, j) {
                    edges.push((i, j));
                    weights.push(1);
                    degree[i] += 1;
                    degree[j] += 1;
                }
            }
        }
        // {i, s} of weight 1 for every original vertex.
        for i in 0..n {
            edges.push((i, source_vertex));
            weights.push(1);
        }
        // {i, t} of weight deg_H(i) + 2 for every original vertex.
        for (i, degree) in degree.into_iter().enumerate() {
            edges.push((i, sink_vertex));
            // deg_H(i) + 2 <= n + 1 <= M, which `construction_parameters` showed fits in `i64`.
            weights.push(
                <Source as ReduceTo<Target>>::exact_i64(degree, "encoding a complement degree")?
                    + 2,
            );
        }
        // {t, padding} of weight M for every padding vertex.
        for padding_vertex in sink_vertex + 1..parameters.target_vertices {
            edges.push((sink_vertex, padding_vertex));
            weights.push(parameters.heavy_weight);
        }

        let target = Target::try_new(
            SimpleGraph::try_new(parameters.target_vertices, edges)
                .map_err(<Source as ReduceTo<Target>>::target_construction)?,
            weights,
            source_vertex,
            sink_vertex,
            n + 1,
        )
        .map_err(<Source as ReduceTo<Target>>::target_construction)?;
        Ok(
            ReductionDecisionMinimumVertexCoverToMinimumCutIntoBoundedSets {
                target,
                source_num_vertices: n,
                threshold: parameters.threshold,
            },
        )
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionminimumvertexcover_to_minimumcutintoboundedsets",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Target>(
                Decision::new(
                    MinimumVertexCover::new(
                        SimpleGraph::new(
                            6,
                            vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5)],
                        ),
                        vec![One; 6],
                    ),
                    3,
                ),
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, true, false, true, false]),
                    // Independent set {0, 3, 5}, the sink t = 7 and the three padding vertices
                    // on the sink's side; the source s = 6 and the cover {1, 2, 4} on the other.
                    target_config: serde_json::json!(vec![
                        true, false, false, true, false, true, false, true, true, true, true
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionminimumvertexcover_minimumcutintoboundedsets.rs"]
mod tests;
