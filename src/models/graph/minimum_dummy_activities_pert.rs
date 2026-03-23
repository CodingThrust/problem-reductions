//! Minimum Dummy Activities in PERT Networks.
//!
//! Given a precedence DAG whose vertices are tasks, select which direct
//! precedence constraints can be represented by merging the predecessor's
//! finish event with the successor's start event. The remaining precedence
//! constraints require dummy activities. A configuration is valid when the
//! resulting event network is acyclic and preserves exactly the same
//! task-to-task reachability relation as the original DAG.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, BTreeSet};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDummyActivitiesPert",
        display_name: "Minimum Dummy Activities in PERT Networks",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a PERT event network for a precedence DAG minimizing dummy activities",
        fields: &[
            FieldInfo {
                name: "graph",
                type_name: "DirectedGraph",
                description: "The precedence DAG G=(V,A) whose vertices are tasks and arcs encode direct precedence constraints",
            },
        ],
    }
}

/// Minimum Dummy Activities in PERT Networks.
///
/// For each precedence arc `u -> v`, the configuration chooses one of two
/// encodings:
/// - `1`: merge `u`'s finish event with `v`'s start event
/// - `0`: keep a dummy activity from `u`'s finish event to `v`'s start event
///
/// A valid configuration must preserve exactly the same reachability relation
/// between task completions and task starts as the original precedence DAG.
#[derive(Debug, Clone, Serialize)]
pub struct MinimumDummyActivitiesPert {
    graph: DirectedGraph,
}

impl MinimumDummyActivitiesPert {
    /// Fallible constructor used by CLI validation and deserialization.
    pub fn try_new(graph: DirectedGraph) -> Result<Self, String> {
        if !graph.is_dag() {
            return Err("MinimumDummyActivitiesPert requires the input graph to be a DAG".into());
        }
        Ok(Self { graph })
    }

    /// Create a new instance.
    ///
    /// # Panics
    ///
    /// Panics if the input graph is not a DAG.
    pub fn new(graph: DirectedGraph) -> Self {
        Self::try_new(graph).unwrap_or_else(|msg| panic!("{msg}"))
    }

    /// Get the precedence DAG.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the number of tasks.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of direct precedence arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Check whether the merge-selection config encodes a valid PERT network.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config).is_valid()
    }

    fn precedence_arcs(&self) -> Vec<(usize, usize)> {
        self.graph.arcs()
    }

    fn build_candidate_network(&self, config: &[usize]) -> Option<CandidatePertNetwork> {
        let num_tasks = self.num_vertices();
        let arcs = self.precedence_arcs();
        if config.len() != arcs.len() || config.iter().any(|&bit| bit > 1) {
            return None;
        }

        let mut uf = UnionFind::new(2 * num_tasks);
        for ((u, v), &merge_bit) in arcs.iter().zip(config.iter()) {
            if merge_bit == 1 {
                uf.union(finish_endpoint(*u), start_endpoint(*v));
            }
        }

        let roots: Vec<usize> = (0..2 * num_tasks)
            .map(|endpoint| uf.find(endpoint))
            .collect();
        let mut root_to_dense = BTreeMap::new();
        for &root in &roots {
            let next = root_to_dense.len();
            root_to_dense.entry(root).or_insert(next);
        }

        let start_events: Vec<usize> = (0..num_tasks)
            .map(|task| root_to_dense[&roots[start_endpoint(task)]])
            .collect();
        let finish_events: Vec<usize> = (0..num_tasks)
            .map(|task| root_to_dense[&roots[finish_endpoint(task)]])
            .collect();

        if start_events
            .iter()
            .zip(finish_events.iter())
            .any(|(start, finish)| start == finish)
        {
            return None;
        }

        let task_arcs: Vec<(usize, usize)> = (0..num_tasks)
            .map(|task| (start_events[task], finish_events[task]))
            .collect();

        let dummy_arcs: BTreeSet<(usize, usize)> = arcs
            .iter()
            .zip(config.iter())
            .filter_map(|((u, v), &merge_bit)| {
                if merge_bit == 1 {
                    return None;
                }
                let source = finish_events[*u];
                let target = start_events[*v];
                (source != target).then_some((source, target))
            })
            .collect();

        let task_arc_set: BTreeSet<(usize, usize)> = task_arcs.iter().copied().collect();
        let num_dummy_arcs = dummy_arcs.difference(&task_arc_set).count();

        let mut event_arcs = task_arcs;
        event_arcs.extend(dummy_arcs.iter().copied());
        let event_graph = DirectedGraph::new(root_to_dense.len(), event_arcs);
        if !event_graph.is_dag() {
            return None;
        }

        Some(CandidatePertNetwork {
            event_graph,
            start_events,
            finish_events,
            num_dummy_arcs,
        })
    }
}

impl Problem for MinimumDummyActivitiesPert {
    const NAME: &'static str = "MinimumDummyActivitiesPert";
    type Value = Min<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_arcs()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i32> {
        let Some(candidate) = self.build_candidate_network(config) else {
            return Min(None);
        };

        let source_reachability = reachability_matrix(&self.graph);
        let event_reachability = reachability_matrix(&candidate.event_graph);

        for source in 0..self.num_vertices() {
            for target in 0..self.num_vertices() {
                let pert_reachable = candidate.finish_events[source]
                    == candidate.start_events[target]
                    || event_reachability[candidate.finish_events[source]]
                        [candidate.start_events[target]];
                if source_reachability[source][target] != pert_reachable {
                    return Min(None);
                }
            }
        }

        Min(Some(
            i32::try_from(candidate.num_dummy_arcs).expect("dummy activity count must fit in i32"),
        ))
    }
}

crate::declare_variants! {
    default MinimumDummyActivitiesPert => "2^num_arcs",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_dummy_activities_pert",
        instance: Box::new(MinimumDummyActivitiesPert::new(DirectedGraph::new(
            6,
            vec![(0, 2), (0, 3), (1, 3), (1, 4), (2, 5)],
        ))),
        optimal_config: vec![1, 0, 0, 1, 1],
        optimal_value: serde_json::json!(2),
    }]
}

#[derive(Deserialize)]
struct MinimumDummyActivitiesPertData {
    graph: DirectedGraph,
}

impl<'de> Deserialize<'de> for MinimumDummyActivitiesPert {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = MinimumDummyActivitiesPertData::deserialize(deserializer)?;
        Self::try_new(data.graph).map_err(serde::de::Error::custom)
    }
}

struct CandidatePertNetwork {
    event_graph: DirectedGraph,
    start_events: Vec<usize>,
    finish_events: Vec<usize>,
    num_dummy_arcs: usize,
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            self.parent[root_b] = root_a;
        }
    }
}

fn start_endpoint(task: usize) -> usize {
    2 * task
}

fn finish_endpoint(task: usize) -> usize {
    2 * task + 1
}

fn reachability_matrix(graph: &DirectedGraph) -> Vec<Vec<bool>> {
    let num_vertices = graph.num_vertices();
    let adjacency: Vec<Vec<usize>> = (0..num_vertices)
        .map(|vertex| graph.successors(vertex))
        .collect();
    let mut reachable = vec![vec![false; num_vertices]; num_vertices];

    for source in 0..num_vertices {
        let mut stack = adjacency[source].clone();
        while let Some(vertex) = stack.pop() {
            if reachable[source][vertex] {
                continue;
            }
            reachable[source][vertex] = true;
            stack.extend(adjacency[vertex].iter().copied());
        }
    }

    reachable
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_dummy_activities_pert.rs"]
mod tests;
