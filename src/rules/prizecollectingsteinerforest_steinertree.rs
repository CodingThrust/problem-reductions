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
//! - for every `v in V_p` we add `(v, t_v)` of cost `0` and `(r, t_v)` of
//!   cost `beta * p(v)`,
//! - the terminal set is `{r} cup {t_v : v in V_p}`.
//!
//! The Steiner-tree optimum then equals the PCSF optimum.
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
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing PCSF to SteinerTree.
///
/// Stores the original PCSF source sizes plus the mapping from the target
/// graph's edge list back to the source variables (the original edge index
/// for each "original" edge, and the source vertex index for each gadget
/// include-edge). Other target edges (root-attachment and gadget omit-edges)
/// are not needed for extraction.
#[derive(Debug, Clone)]
pub struct ReductionPCSFToSteinerTree {
    target: SteinerTree<SimpleGraph, i32>,
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
    type Source = PrizeCollectingSteinerForest<SimpleGraph, i32>;
    type Target = SteinerTree<SimpleGraph, i32>;

    fn target_problem(&self) -> &SteinerTree<SimpleGraph, i32> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_source_vertices;
            let m = self.num_source_edges;
            let mut source_config = vec![0usize; n + m];

            // Mark vertices included via their gadget include-edge `(v, t_v)`,
            // and edges via the matching original edge.
            for (target_idx, &selected) in target_solution.iter().enumerate() {
                if selected != 1 {
                    continue;
                }
                if let Some(v) = self.target_to_include_vertex[target_idx] {
                    source_config[v] = 1;
                } else if let Some(src_edge) = self.target_to_source_edge[target_idx] {
                    source_config[n + src_edge] = 1;
                }
            }

            // Any original edge selected in `T*` forces both endpoints into
            // `V_F`. The PCSF model rejects configurations where a selected
            // edge has an unselected endpoint, so we mark endpoints explicitly
            // (this also covers prize-zero endpoints, which have no gadget).
            let edges = self.target.graph().edges();
            for (target_idx, &(_, _)) in edges.iter().enumerate() {
                if target_solution[target_idx] != 1 {
                    continue;
                }
                if let Some(src_edge) = self.target_to_source_edge[target_idx] {
                    let (u, v) = self.source_edge_pair(src_edge);
                    source_config[u] = 1;
                    source_config[v] = 1;
                }
            }

            source_config
        })
    }
}

impl ReductionPCSFToSteinerTree {
    /// Look up the endpoint pair of the `idx`-th source edge in the target
    /// graph's edge list (source edges are placed first by construction).
    fn source_edge_pair(&self, src_edge_idx: usize) -> (usize, usize) {
        self.target.graph().edges()[src_edge_idx]
    }
}

#[reduction(
    exact = {
        num_vertices = "num_vertices + num_vertices_with_prize + 1",
        num_edges = "num_edges + num_vertices + 2 * num_vertices_with_prize",
        num_terminals = "num_vertices_with_prize + 1",
    }
)]
impl ReduceTo<SteinerTree<SimpleGraph, i32>> for PrizeCollectingSteinerForest<SimpleGraph, i32> {
    type Result = ReductionPCSFToSteinerTree;

    fn reduce_to(&self) -> Self::Result {
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
        let mut target_edge_weights: Vec<i32> = Vec::with_capacity(m + n + 2 * k);
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

        // 3. Per-prized-vertex gadget: (v, t_v) of cost 0 and (r, t_v) of
        // cost beta * p(v).
        for (gadget_pos, &v) in prized.iter().enumerate() {
            let t_v = gadget_terminal(gadget_pos);
            // include-edge: marks "v is in V_F" with cost 0.
            target_edges.push((v, t_v));
            target_edge_weights.push(0);
            target_to_source_edge.push(None);
            target_to_include_vertex.push(Some(v));
            // omit-edge: pays beta * p(v) when v is excluded from V_F.
            target_edges.push((root, t_v));
            target_edge_weights.push(beta * source_prizes[v]);
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
            SteinerTree::<SimpleGraph, i32>::new(target_graph, target_edge_weights, terminals);

        ReductionPCSFToSteinerTree {
            target,
            num_source_vertices: n,
            num_source_edges: m,
            target_to_source_edge,
            target_to_include_vertex,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::RuleExampleSpec;
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;

    vec![RuleExampleSpec {
        id: "prize_collecting_steiner_forest_to_steiner_tree",
        build: || {
            // Issue #1027 canonical instance with the omit-edge actually
            // selected at the optimum: path 0 - 1 - 2 with c(0,1)=10,
            // c(1,2)=10, prizes p = (5, 1, 5), beta = 1, omega = 1. The
            // optimum drops vertex 1 (paying p(1) = 1) rather than paying a
            // size-10 edge to reach it.
            let source = PrizeCollectingSteinerForest::<SimpleGraph, i32>::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![5, 1, 5],
                vec![10, 10],
                1,
                1,
            );
            let reduction = <PrizeCollectingSteinerForest<SimpleGraph, i32> as ReduceTo<
                SteinerTree<SimpleGraph, i32>,
            >>::reduce_to(&source);
            let target = reduction.target_problem();
            let target_config = BruteForce::new()
                .find_witness(target)
                .expect("canonical PCSF -> SteinerTree example must have an optimal target tree");
            let source_config = reduction.extract_solution(&target_config).unwrap();
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
#[path = "../unit_tests/rules/prizecollectingsteinerforest_steinertree.rs"]
mod tests;
