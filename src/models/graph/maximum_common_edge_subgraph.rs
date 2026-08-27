//! Maximum Common Edge Subgraph problem implementation.
//!
//! Given two finite directed edge-labelled graphs `G1 = (V1, E1)` and
//! `G2 = (V2, E2)` with `E_i subset.eq V_i x Sigma x V_i`, find a partial
//! injective map `f: U1 -> V2`, where `U1 subset.eq V1`, that maximizes the
//! number of labelled arcs `(u, lambda, v) in E1` such that `u, v in U1` and
//! `(f(u), lambda, f(v)) in E2`. Edge labels must match exactly and the model
//! uses set semantics (each preserved arc contributes `1`).
//!
//! The configuration vector has length `|V1|`. For each source vertex `u`, the
//! value `config[u] in {0, ..., |V2| - 1, |V2|}` records which target vertex
//! `u` is matched to, with the sentinel value `|V2|` denoting "unmatched"
//! (`bottom`). Feasibility requires injectivity on the matched vertices.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumCommonEdgeSubgraph",
        display_name: "Maximum Common Edge Subgraph",
        aliases: &["MCES"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Maximize the number of preserved labelled directed arcs under a partial injective vertex map from G1 into G2",
        fields: &[
            FieldInfo {
                name: "graph_1",
                type_name: "LabelledDigraph",
                description: "Source directed edge-labelled graph G1 = (V1, E1) whose vertices are mapped",
            },
            FieldInfo {
                name: "graph_2",
                type_name: "LabelledDigraph",
                description: "Target directed edge-labelled graph G2 = (V2, E2) receiving the partial injective map",
            },
        ],
    }
}

/// A directed labelled arc `(src, label, dst)` in a [`LabelledDigraph`].
///
/// Labels are unsigned integers; the alphabet `Sigma` is encoded by mapping
/// each symbol to a distinct nonnegative integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LabelledArc {
    /// Source vertex index.
    pub src: usize,
    /// Edge label.
    pub label: usize,
    /// Destination vertex index.
    pub dst: usize,
}

impl LabelledArc {
    /// Construct a new labelled arc.
    pub fn new(src: usize, label: usize, dst: usize) -> Self {
        Self { src, label, dst }
    }
}

/// A finite directed edge-labelled graph used by
/// [`MaximumCommonEdgeSubgraph`].
///
/// Vertices are the indices `0..num_vertices`. Arcs are stored as a flat
/// vector and treated as a set (duplicates are deduplicated by the
/// constructor).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelledDigraph {
    /// Number of vertices `|V|`.
    pub num_vertices: usize,
    /// Labelled directed arcs `(u, label, v)`.
    pub arcs: Vec<LabelledArc>,
}

impl LabelledDigraph {
    /// Construct a new labelled digraph.
    ///
    /// # Panics
    /// Panics if any arc references a vertex index outside `0..num_vertices`.
    pub fn new(num_vertices: usize, arcs: Vec<LabelledArc>) -> Self {
        for arc in &arcs {
            assert!(
                arc.src < num_vertices,
                "labelled arc source {} out of range for num_vertices = {}",
                arc.src,
                num_vertices
            );
            assert!(
                arc.dst < num_vertices,
                "labelled arc destination {} out of range for num_vertices = {}",
                arc.dst,
                num_vertices
            );
        }
        // Deduplicate while preserving order so set semantics hold.
        let mut seen = std::collections::HashSet::new();
        let mut deduped = Vec::with_capacity(arcs.len());
        for arc in arcs {
            if seen.insert((arc.src, arc.label, arc.dst)) {
                deduped.push(arc);
            }
        }
        Self {
            num_vertices,
            arcs: deduped,
        }
    }

    /// Number of vertices `|V|`.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Number of distinct labelled arcs `|E|`.
    pub fn num_arcs(&self) -> usize {
        self.arcs.len()
    }

    /// Labelled directed arcs.
    pub fn arcs(&self) -> &[LabelledArc] {
        &self.arcs
    }
}

/// The Maximum Common Edge Subgraph problem.
///
/// Given two finite directed edge-labelled graphs `G1 = (V1, E1)` and
/// `G2 = (V2, E2)`, find a partial injective map `f: U1 -> V2` with
/// `U1 subset.eq V1` that maximizes
///
/// `|{(u, lambda, v) in E1 : u, v in U1 and (f(u), lambda, f(v)) in E2}|`.
///
/// # Configuration encoding
///
/// `dims()` returns `vec![graph_2.num_vertices + 1; graph_1.num_vertices]`.
/// For each source vertex `u in V1`, `config[u]` is either an index in
/// `0..graph_2.num_vertices` (the matched target vertex) or the sentinel
/// value `graph_2.num_vertices` denoting `bottom` (unmatched). Feasibility
/// requires the matched values (everything not equal to the sentinel) to be
/// pairwise distinct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaximumCommonEdgeSubgraph {
    graph_1: LabelledDigraph,
    graph_2: LabelledDigraph,
}

impl MaximumCommonEdgeSubgraph {
    /// Construct a new instance from two labelled digraphs.
    pub fn new(graph_1: LabelledDigraph, graph_2: LabelledDigraph) -> Self {
        Self { graph_1, graph_2 }
    }

    /// Source graph `G1`.
    pub fn graph_1(&self) -> &LabelledDigraph {
        &self.graph_1
    }

    /// Target graph `G2`.
    pub fn graph_2(&self) -> &LabelledDigraph {
        &self.graph_2
    }

    /// Number of vertices in `G1`: `|V1|`.
    pub fn num_vertices_1(&self) -> usize {
        self.graph_1.num_vertices()
    }

    /// Number of vertices in `G2`: `|V2|`.
    pub fn num_vertices_2(&self) -> usize {
        self.graph_2.num_vertices()
    }

    /// Number of labelled arcs in `G1`: `|E1|`.
    pub fn num_arcs_1(&self) -> usize {
        self.graph_1.num_arcs()
    }

    /// Number of labelled arcs in `G2`: `|E2|`.
    pub fn num_arcs_2(&self) -> usize {
        self.graph_2.num_arcs()
    }

    /// Sentinel value encoding `bottom` (unmatched) for any `config[u]`.
    pub fn bottom_index(&self) -> usize {
        self.graph_2.num_vertices()
    }

    /// Check that `config` describes a partial injective map.
    ///
    /// Validity requires: `config.len() == |V1|`, every entry lies in
    /// `0..=|V2|` (with `|V2|` denoting `bottom`), and all entries strictly
    /// less than `|V2|` are pairwise distinct.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        let n1 = self.num_vertices_1();
        let n2 = self.num_vertices_2();
        if config.len() != n1 {
            return false;
        }
        let bottom = n2;
        let mut used = vec![false; n2];
        for &value in config {
            if value > bottom {
                return false;
            }
            if value == bottom {
                continue;
            }
            if used[value] {
                return false;
            }
            used[value] = true;
        }
        true
    }

    /// Count the labelled arcs in `G1` that are preserved by the partial
    /// injective map `config`. Returns `None` if `config` is infeasible.
    pub fn preserved_arc_count(
        &self,
        config: &[usize],
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        if !self.is_valid_solution(config) {
            return Ok(None);
        }
        let bottom = self.bottom_index();
        // Build a lookup set of arcs in G2 for O(1) membership checks.
        let arcs_2: std::collections::HashSet<(usize, usize, usize)> = self
            .graph_2
            .arcs()
            .iter()
            .map(|arc| (arc.src, arc.label, arc.dst))
            .collect();
        let mut count = 0usize;
        for arc in self.graph_1.arcs() {
            let fu = config[arc.src];
            let fv = config[arc.dst];
            if fu == bottom || fv == bottom {
                continue;
            }
            if arcs_2.contains(&(fu, arc.label, fv)) {
                count += 1;
            }
        }
        Ok(Some(i64::try_from(count).map_err(|_| {
            crate::traits::EvaluationError::IntegerOverflow(
                "converting preserved-arc count to i64".into(),
            )
        })?))
    }
}

impl Problem for MaximumCommonEdgeSubgraph {
    const NAME: &'static str = "MaximumCommonEdgeSubgraph";
    type Solution = Vec<usize>;
    type Value = Max<i64>;

    crate::problem_size![
        ("num_arcs_1", num_arcs_1),
        ("num_arcs_2", num_arcs_2),
        ("num_vertices_1", num_vertices_1),
        ("num_vertices_2", num_vertices_2),
    ];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Max<i64>, crate::traits::EvaluationError> {
        if config.len() != self.num_vertices_1() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "vertex mapping length does not match the first graph".into(),
            ));
        }
        if config.iter().any(|&vertex| vertex > self.num_vertices_2()) {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "vertex mapping contains an out-of-range target vertex".into(),
            ));
        }
        Ok({
            match self.preserved_arc_count(config)? {
                Some(count) => Max(Some(count)),
                None => Max(None),
            }
        })
    }
}

impl crate::solvers::BruteForceProblem for MaximumCommonEdgeSubgraph {
    fn dimensions(&self) -> Vec<usize> {
        vec![self.graph_2.num_vertices() + 1; self.graph_1.num_vertices()]
    }
}

crate::declare_variants! {
    default MaximumCommonEdgeSubgraph => "(num_vertices_2 + 1)^num_vertices_1",
}

crate::register_brute_force! {
    MaximumCommonEdgeSubgraph,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_common_edge_subgraph",
        instance: Box::new(MaximumCommonEdgeSubgraph::new(
            LabelledDigraph::new(
                5,
                vec![
                    LabelledArc::new(0, 0, 1),
                    LabelledArc::new(1, 1, 2),
                    LabelledArc::new(0, 2, 2),
                    LabelledArc::new(2, 0, 3),
                    LabelledArc::new(1, 3, 3),
                    LabelledArc::new(3, 1, 4),
                ],
            ),
            LabelledDigraph::new(
                4,
                vec![
                    LabelledArc::new(0, 0, 1),
                    LabelledArc::new(1, 1, 2),
                    LabelledArc::new(0, 2, 2),
                    LabelledArc::new(2, 0, 3),
                    LabelledArc::new(1, 3, 3),
                    LabelledArc::new(0, 1, 3),
                ],
            ),
        )),
        // 4 encodes bottom because |V2| = 4. The map 0->0, 1->1, 2->2, 3->3,
        // 4->bottom preserves the first five source arcs.
        optimal_config: serde_json::json!(vec![0, 1, 2, 3, 4]),
        optimal_value: serde_json::json!(5),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_common_edge_subgraph.rs"]
mod tests;
