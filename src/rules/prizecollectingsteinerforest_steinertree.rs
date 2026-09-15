//! Reduction from Prize-Collecting Steiner Forest (PCSF) to Steiner Tree
//! via the artificial-root + per-vertex prize gadget construction.
//!
//! The PCSF objective on `(V, E)` with vertex prizes `p`, edge costs `c`,
//! tradeoff `beta`, and per-component penalty `omega` is
//!
//! ```text
//! beta * sum_{v notin V_F} p(v) + sum_{e in E_F} c(e) + omega * kappa(F).
//! ```
//!
//! We build a Steiner-tree instance on the augmented graph
//! `H = (V cup {r} cup {t_v : v in V_p}, E_H)` where `V_p` collects the
//! vertices with `p(v) > 0`, and
//!
//! - every original edge keeps its cost,
//! - every `v in V` is attached to `r` by an edge of cost `omega` (so each
//!   tree component of `F` is paid by exactly one root-attachment edge in
//!   `T*`),
//! - with `M = omega + 1`, for every `v in V_p` add `(v, t_v)` of cost `M`
//!   and `(r, t_v)` of cost `M + beta * p(v)`,
//! - the terminal set is `{r} cup {t_v : v in V_p}`.
//!
//! The Steiner-tree optimum equals the PCSF optimum plus `M * |V_p|`.
//! In an optimum each gadget terminal is a leaf: replacing both gadget edges
//! by the include edge and a root attachment strictly reduces cost.
//!
//! References:
//! - Bienstock, Goemans, Simchi-Levi, Williamson, "A note on the prize
//!   collecting traveling salesman problem," Math. Programming 59 (1993).
//!   <https://doi.org/10.1007/BF01581256>
//! - Tuncbag et al., "Simultaneous Reconstruction of Multiple Signaling
//!   Pathways via the Prize-Collecting Steiner Forest Problem,"
//!   J. Comput. Biol. 20(2):124--136, 2013.
//!   <https://doi.org/10.1089/cmb.2012.0092>

use crate::models::graph::{PrizeCollectingSteinerForest, SteinerTree};
use crate::reduction;
use crate::rules::traits::{recover_preserving_status, ReduceTo, ReductionResult};
use crate::solvers::ProblemOutcome;
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing PCSF to SteinerTree.
///
/// Stores the original PCSF source parameters plus the mapping from the target
/// graph's edge list back to the source variables (the original edge index
/// for each "original" edge, and the source vertex index for each gadget
/// include-edge). Other target edges (root-attachment and gadget omit-edges)
/// are not needed for extraction.
#[derive(Debug, Clone)]
pub struct ReductionPCSFToSteinerTree {
    target: SteinerTree<SimpleGraph, i64>,
    /// Number of vertices in the source graph (also the prefix size of the
    /// source configuration's vertex-selector segment).
    num_source_vertices: usize,
    /// Number of edges in the source graph (length of the edge-selector
    /// segment of the source configuration).
    num_source_edges: usize,
    /// `target_to_source_edge[i] = Some(j)` iff target edge `i` is the same
    /// pair as source edge `j`; otherwise the target edge is a gadget edge.
    target_to_source_edge: Vec<Option<usize>>,
    /// `target_to_include_vertex[i] = Some(v)` iff target edge `i` is the
    /// include-edge `(v, t_v)` of the per-vertex prize gadget. Original
    /// edges and other gadget edges store `None`.
    target_to_include_vertex: Vec<Option<usize>>,
}

impl ReductionResult for ReductionPCSFToSteinerTree {
    type Source = PrizeCollectingSteinerForest<SimpleGraph, i64>;
    type Target = SteinerTree<SimpleGraph, i64>;

    fn target_problem(&self) -> &SteinerTree<SimpleGraph, i64> {
        &self.target
    }

    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        recover_preserving_status(source, target, |solution| self.map_solution(solution))
    }
}

impl ReductionPCSFToSteinerTree {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        Ok({
            let n = self.num_source_vertices;
            let m = self.num_source_edges;
            let mut selected_vertices = vec![false; n];
            let mut selected_edges = vec![false; m];
            let edges = self.target.graph().edges();

            // Mark vertices included via their gadget include-edge `(v, t_v)`,
            // and edges via the matching original edge.
            for (target_idx, &selected) in target_solution.iter().enumerate() {
                if !selected {
                    continue;
                }
                if let Some(v) = self.target_to_include_vertex[target_idx] {
                    selected_vertices[v] = true;
                } else if let Some(src_edge) = self.target_to_source_edge[target_idx] {
                    selected_edges[src_edge] = true;
                    // Include both endpoints, including prize-zero vertices
                    // that have no inclusion gadget.
                    let (u, v) = edges[target_idx];
                    selected_vertices[u] = true;
                    selected_vertices[v] = true;
                }
            }

            (selected_vertices, selected_edges)
        })
    }
}

#[reduction(
    transform = exact {
        num_vertices = "num_vertices + num_vertices_with_prize + 1",
        num_edges = "num_edges + num_vertices + 2 * num_vertices_with_prize",
        num_terminals = "num_vertices_with_prize + 1",
    }
)]
impl ReduceTo<SteinerTree<SimpleGraph, i64>> for PrizeCollectingSteinerForest<SimpleGraph, i64> {
    type Result = ReductionPCSFToSteinerTree;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let m = self.num_edges();
        let source_edges = self.graph().edges();
        let source_edge_costs = self.edge_costs();
        let source_prizes = self.vertex_prizes();
        let beta = *self.beta();
        let omega = *self.omega();

        // Augmented vertex layout:
        //   indices 0..n           -- original vertices
        //   index   n               -- artificial root r
        //   indices n+1..n+1+k      -- gadget terminals t_v for v in V_p,
        //                              listed in increasing order of v.
        let prized: Vec<usize> = (0..n).filter(|&v| source_prizes[v] > 0).collect();
        let k = prized.len();
        let root = n;
        let gadget_terminal = |gadget_pos: usize| -> usize { n + 1 + gadget_pos };

        let target_num_vertices = n + 1 + k;
        let mut target_edges: Vec<(usize, usize)> = Vec::with_capacity(m + n + 2 * k);
        let mut target_edge_weights: Vec<i64> = Vec::with_capacity(m + n + 2 * k);
        let mut target_to_source_edge: Vec<Option<usize>> = Vec::with_capacity(m + n + 2 * k);
        let mut target_to_include_vertex: Vec<Option<usize>> = Vec::with_capacity(m + n + 2 * k);

        // 1. Original edges keep their cost.
        for (idx, &(u, v)) in source_edges.iter().enumerate() {
            target_edges.push((u, v));
            target_edge_weights.push(source_edge_costs[idx]);
            target_to_source_edge.push(Some(idx));
            target_to_include_vertex.push(None);
        }

        // 2. Root-attachment edge (r, v) of cost omega for every v in V.
        for v in 0..n {
            target_edges.push((v, root));
            target_edge_weights.push(omega);
            target_to_source_edge.push(None);
            target_to_include_vertex.push(None);
        }

        // 3. Both gadget edges carry M = omega + 1. Using both is strictly
        // more expensive than replacing the omit edge with a root attachment.
        for (gadget_pos, &v) in prized.iter().enumerate() {
            let t_v = gadget_terminal(gadget_pos);
            let include_cost =
                omega.checked_add(1).ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<
                        Self,
                        SteinerTree<SimpleGraph, i64>,
                    >("forming the Steiner gadget inclusion cost")
                })?;
            let omit_cost = beta
                .checked_mul(source_prizes[v])
                .and_then(|penalty| penalty.checked_add(include_cost))
                .ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<
                        Self,
                        SteinerTree<SimpleGraph, i64>,
                    >("forming the Steiner gadget omission cost")
                })?;
            // The include edge records a selected prized vertex.
            target_edges.push((v, t_v));
            target_edge_weights.push(include_cost);
            target_to_source_edge.push(None);
            target_to_include_vertex.push(Some(v));
            // The omit edge pays the extra beta * p(v).
            target_edges.push((root, t_v));
            target_edge_weights.push(omit_cost);
            target_to_source_edge.push(None);
            target_to_include_vertex.push(None);
        }

        // 4. Terminal set: r plus every gadget terminal t_v.
        let mut terminals: Vec<usize> = Vec::with_capacity(k + 1);
        terminals.push(root);
        for gadget_pos in 0..k {
            terminals.push(gadget_terminal(gadget_pos));
        }

        let target_graph = SimpleGraph::new(target_num_vertices, target_edges);
        let target =
            SteinerTree::<SimpleGraph, i64>::new(target_graph, target_edge_weights, terminals);

        Ok(ReductionPCSFToSteinerTree {
            target,
            num_source_vertices: n,
            num_source_edges: m,
            target_to_source_edge,
            target_to_include_vertex,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::RuleExampleSpec;
    use crate::export::SolutionPair;

    vec![RuleExampleSpec {
        id: "prize_collecting_steiner_forest_to_steiner_tree",
        build: || {
            let source = PrizeCollectingSteinerForest::<SimpleGraph, i64>::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![5, 1, 5],
                vec![10, 10],
                1,
                1,
            )
            .unwrap();
            crate::example_db::specs::rule_example_with_witness::<_, SteinerTree<SimpleGraph, i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!([[true, false, true], [false, false]]),
                    target_config: serde_json::json!([
                        false, false, true, false, true, true, false, false, true, true, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/prizecollectingsteinerforest_steinertree.rs"]
mod tests;
