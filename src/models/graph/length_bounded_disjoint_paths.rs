//! Length-Bounded Disjoint Paths problem implementation.
//!
//! The problem asks whether a graph contains at least `J` internally
//! vertex-disjoint `s-t` paths, each using at most `K` edges.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "LengthBoundedDisjointPaths",
        display_name: "Length-Bounded Disjoint Paths",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Find J internally vertex-disjoint s-t paths of length at most K",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "source", type_name: "usize", description: "The shared source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "The shared sink vertex t" },
            FieldInfo { name: "num_paths_required", type_name: "usize", description: "Required number J of disjoint s-t paths" },
            FieldInfo { name: "max_length", type_name: "usize", description: "Maximum path length K in edges" },
        ],
    }
}

/// Length-Bounded Disjoint Paths on an undirected graph.
///
/// A configuration uses `J * |V|` binary choices. For each path slot `j` and
/// vertex `v`, `x_{j,v} = 1` means that `v` belongs to slot `j`'s path. Each
/// slot must induce a simple `s-t` path, and the internal vertices of
/// different slots must be disjoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct LengthBoundedDisjointPaths<G> {
    graph: G,
    source: usize,
    sink: usize,
    num_paths_required: usize,
    max_length: usize,
}

impl<G: Graph> LengthBoundedDisjointPaths<G> {
    /// Create a new Length-Bounded Disjoint Paths instance.
    pub fn new(
        graph: G,
        source: usize,
        sink: usize,
        num_paths_required: usize,
        max_length: usize,
    ) -> Self {
        assert!(
            source < graph.num_vertices(),
            "source must be a valid graph vertex"
        );
        assert!(
            sink < graph.num_vertices(),
            "sink must be a valid graph vertex"
        );
        assert_ne!(source, sink, "source and sink must be distinct");
        assert!(
            num_paths_required > 0,
            "num_paths_required must be positive"
        );
        assert!(max_length > 0, "max_length must be positive");
        Self {
            graph,
            source,
            sink,
            num_paths_required,
            max_length,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the shared source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the shared sink vertex.
    pub fn sink(&self) -> usize {
        self.sink
    }

    /// Get the required number of paths.
    pub fn num_paths_required(&self) -> usize {
        self.num_paths_required
    }

    /// Get the maximum permitted path length in edges.
    pub fn max_length(&self) -> usize {
        self.max_length
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether a configuration is a valid solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_valid_path_collection(
            &self.graph,
            self.source,
            self.sink,
            self.num_paths_required,
            self.max_length,
            config,
        )
    }
}

impl<G> Problem for LengthBoundedDisjointPaths<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "LengthBoundedDisjointPaths";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_paths_required * self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_solution(config)
    }
}

impl<G: Graph + VariantParam> SatisfactionProblem for LengthBoundedDisjointPaths<G> {}

fn is_valid_path_collection<G: Graph>(
    graph: &G,
    source: usize,
    sink: usize,
    num_paths_required: usize,
    max_length: usize,
    config: &[usize],
) -> bool {
    let num_vertices = graph.num_vertices();
    if config.len() != num_paths_required * num_vertices {
        return false;
    }
    if config.iter().any(|&value| value > 1) {
        return false;
    }

    let mut used_internal = vec![false; num_vertices];
    let mut used_direct_path = false;
    for slot in config.chunks(num_vertices) {
        if !is_valid_path_slot(
            graph,
            source,
            sink,
            max_length,
            slot,
            &mut used_internal,
            &mut used_direct_path,
        ) {
            return false;
        }
    }
    true
}

fn is_valid_path_slot<G: Graph>(
    graph: &G,
    source: usize,
    sink: usize,
    max_length: usize,
    slot: &[usize],
    used_internal: &mut [bool],
    used_direct_path: &mut bool,
) -> bool {
    if slot.len() != graph.num_vertices()
        || slot.get(source) != Some(&1)
        || slot.get(sink) != Some(&1)
    {
        return false;
    }

    let selected = slot
        .iter()
        .enumerate()
        .filter_map(|(vertex, &chosen)| (chosen == 1).then_some(vertex))
        .collect::<Vec<_>>();
    if selected.len() < 2 {
        return false;
    }

    let mut in_path = vec![false; graph.num_vertices()];
    for &vertex in &selected {
        in_path[vertex] = true;
        if vertex != source && vertex != sink && used_internal[vertex] {
            return false;
        }
    }

    let mut degree_sum = 0usize;
    for &vertex in &selected {
        let degree = graph
            .neighbors(vertex)
            .into_iter()
            .filter(|&neighbor| in_path[neighbor])
            .count();
        degree_sum += degree;

        if vertex == source || vertex == sink {
            if degree != 1 {
                return false;
            }
        } else if degree != 2 {
            return false;
        }
    }

    let edge_count = degree_sum / 2;
    if edge_count + 1 != selected.len() || edge_count > max_length {
        return false;
    }
    if edge_count == 1 {
        if *used_direct_path {
            return false;
        }
        *used_direct_path = true;
    }

    let mut seen = vec![false; graph.num_vertices()];
    let mut stack = vec![source];
    seen[source] = true;
    let mut seen_count = 0usize;
    while let Some(vertex) = stack.pop() {
        seen_count += 1;
        for neighbor in graph.neighbors(vertex) {
            if in_path[neighbor] && !seen[neighbor] {
                seen[neighbor] = true;
                stack.push(neighbor);
            }
        }
    }

    if !seen[sink] || seen_count != selected.len() {
        return false;
    }

    for &vertex in &selected {
        if vertex != source && vertex != sink {
            used_internal[vertex] = true;
        }
    }
    true
}

#[cfg(feature = "example-db")]
fn encode_paths(num_vertices: usize, slots: &[&[usize]]) -> Vec<usize> {
    let mut config = vec![0; num_vertices * slots.len()];
    for (slot_index, slot_vertices) in slots.iter().enumerate() {
        let offset = slot_index * num_vertices;
        for &vertex in *slot_vertices {
            config[offset + vertex] = 1;
        }
    }
    config
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "length_bounded_disjoint_paths_simplegraph",
        build: || {
            let problem = LengthBoundedDisjointPaths::new(
                SimpleGraph::new(
                    7,
                    vec![
                        (0, 1),
                        (1, 6),
                        (0, 2),
                        (2, 3),
                        (3, 6),
                        (0, 4),
                        (4, 5),
                        (5, 6),
                    ],
                ),
                0,
                6,
                2,
                3,
            );
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![encode_paths(7, &[&[0, 1, 6], &[0, 2, 3, 6]])],
            )
        },
    }]
}

crate::declare_variants! {
    default sat LengthBoundedDisjointPaths<SimpleGraph> => "2^(num_paths_required * num_vertices)",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/length_bounded_disjoint_paths.rs"]
mod tests;
