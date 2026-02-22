//! Runtime reduction graph for discovering and executing reduction paths.
//!
//! The graph uses variant-level nodes: each node is a unique `(problem_name, variant)` pair.
//! Edges come exclusively from `#[reduction]` registrations via `inventory::iter::<ReductionEntry>`.
//!
//! This module implements:
//! - Variant-level graph construction from `ReductionEntry` inventory
//! - Dijkstra's algorithm with custom cost functions for optimal paths
//! - JSON export for documentation and visualization

use crate::rules::cost::PathCostFn;
use crate::rules::registry::{ReductionEntry, ReductionOverhead};
use crate::rules::traits::DynReductionResult;
use crate::types::ProblemSize;
use ordered_float::OrderedFloat;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;
use std::any::Any;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

/// A source/target pair from the reduction graph, returned by
/// [`ReductionGraph::outgoing_reductions`] and [`ReductionGraph::incoming_reductions`].
#[derive(Debug, Clone)]
pub struct ReductionEdgeInfo {
    pub source_name: &'static str,
    pub source_variant: BTreeMap<String, String>,
    pub target_name: &'static str,
    pub target_variant: BTreeMap<String, String>,
}

/// Internal edge data combining overhead and executable reduce function.
#[derive(Clone)]
pub(crate) struct ReductionEdgeData {
    pub overhead: ReductionOverhead,
    pub reduce_fn: fn(&dyn Any) -> Box<dyn DynReductionResult>,
}

/// JSON-serializable representation of the reduction graph.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct ReductionGraphJson {
    /// List of problem type nodes.
    pub(crate) nodes: Vec<NodeJson>,
    /// List of reduction edges.
    pub(crate) edges: Vec<EdgeJson>,
}

impl ReductionGraphJson {
    /// Get the source node of an edge.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn source_node(&self, edge: &EdgeJson) -> &NodeJson {
        &self.nodes[edge.source]
    }

    /// Get the target node of an edge.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn target_node(&self, edge: &EdgeJson) -> &NodeJson {
        &self.nodes[edge.target]
    }
}

/// A node in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct NodeJson {
    /// Base problem name (e.g., "MaximumIndependentSet").
    pub(crate) name: String,
    /// Variant attributes as key-value pairs.
    pub(crate) variant: BTreeMap<String, String>,
    /// Category of the problem (e.g., "graph", "set", "optimization", "satisfiability", "specialized").
    pub(crate) category: String,
    /// Relative rustdoc path (e.g., "models/graph/maximum_independent_set").
    pub(crate) doc_path: String,
}

/// Internal reference to a problem variant, used as HashMap key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VariantRef {
    name: String,
    variant: BTreeMap<String, String>,
}

/// A single output field in the reduction overhead.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct OverheadFieldJson {
    /// Output field name (e.g., "num_vars").
    pub(crate) field: String,
    /// Formula as a human-readable string (e.g., "num_vertices").
    pub(crate) formula: String,
}

/// An edge in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct EdgeJson {
    /// Index into the `nodes` array for the source problem variant.
    pub(crate) source: usize,
    /// Index into the `nodes` array for the target problem variant.
    pub(crate) target: usize,
    /// Reduction overhead: output size as polynomials of input size.
    pub(crate) overhead: Vec<OverheadFieldJson>,
    /// Relative rustdoc path for the reduction module.
    pub(crate) doc_path: String,
}

/// A path through the variant-level reduction graph.
#[derive(Debug, Clone)]
pub struct ReductionPath {
    /// Variant-level steps in the path.
    pub steps: Vec<ReductionStep>,
}

impl ReductionPath {
    /// Number of edges (reductions) in the path.
    pub fn len(&self) -> usize {
        if self.steps.is_empty() {
            0
        } else {
            self.steps.len() - 1
        }
    }

    /// Whether the path is empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Source problem name.
    pub fn source(&self) -> Option<&str> {
        self.steps.first().map(|s| s.name.as_str())
    }

    /// Target problem name.
    pub fn target(&self) -> Option<&str> {
        self.steps.last().map(|s| s.name.as_str())
    }

    /// Name-level path (deduplicated consecutive same-name steps).
    pub fn type_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = Vec::new();
        for step in &self.steps {
            if names.last() != Some(&step.name.as_str()) {
                names.push(&step.name);
            }
        }
        names
    }
}

impl std::fmt::Display for ReductionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev_name = "";
        for step in &self.steps {
            if step.name != prev_name {
                if prev_name.is_empty() {
                    write!(f, "{step}")?;
                } else {
                    write!(f, " → {step}")?;
                }
                prev_name = &step.name;
            }
        }
        Ok(())
    }
}

/// A node in a variant-level reduction path.
#[derive(Debug, Clone, Serialize)]
pub struct ReductionStep {
    /// Problem name (e.g., "MaximumIndependentSet").
    pub name: String,
    /// Variant at this point (e.g., {"graph": "KingsSubgraph", "weight": "i32"}).
    pub variant: BTreeMap<String, String>,
}

impl std::fmt::Display for ReductionStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.variant.is_empty() {
            let vars: Vec<_> = self
                .variant
                .iter()
                .map(|(k, v)| format!("{k}: {v:?}"))
                .collect();
            write!(f, " {{{}}}", vars.join(", "))?;
        }
        Ok(())
    }
}

/// Classify a problem's category from its module path.
/// Expected format: "problemreductions::models::<category>::<module_name>"
pub(crate) fn classify_problem_category(module_path: &str) -> &str {
    let parts: Vec<&str> = module_path.split("::").collect();
    if parts.len() >= 3 {
        if let Some(pos) = parts.iter().position(|&p| p == "models") {
            if pos + 1 < parts.len() {
                return parts[pos + 1];
            }
        }
    }
    "other"
}

/// Internal node data for the variant-level graph.
#[derive(Debug, Clone)]
struct VariantNode {
    name: &'static str,
    variant: BTreeMap<String, String>,
}

/// Information about a neighbor in the reduction graph.
#[derive(Debug, Clone)]
pub struct NeighborInfo {
    /// Problem name.
    pub name: &'static str,
    /// Variant attributes.
    pub variant: BTreeMap<String, String>,
    /// Hop distance from the source.
    pub hops: usize,
}

/// Direction for graph traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalDirection {
    /// Follow outgoing edges (what can this reduce to?).
    Outgoing,
    /// Follow incoming edges (what can reduce to this?).
    Incoming,
    /// Follow edges in both directions.
    Both,
}

/// A tree node for neighbor traversal results.
#[derive(Debug, Clone)]
pub struct NeighborTree {
    /// Problem name.
    pub name: String,
    /// Variant attributes.
    pub variant: BTreeMap<String, String>,
    /// Child nodes (sorted by name).
    pub children: Vec<NeighborTree>,
}

/// Validate that a reduction's overhead variables are consistent with source/target size names.
///
/// Checks:
/// - Overhead input variables are a subset of `source_size_names`
/// - Overhead output fields are a subset of `target_size_names` (skipped if `target_size_names` is empty)
///
/// Panics with a descriptive message on mismatch.
pub(crate) fn validate_overhead_variables(
    source_name: &str,
    target_name: &str,
    overhead: &ReductionOverhead,
    source_size_names: &[&str],
    target_size_names: &[&str],
) {
    let source_set: HashSet<&str> = source_size_names.iter().copied().collect();
    let overhead_inputs = overhead.input_variable_names();
    let missing_inputs: Vec<_> = overhead_inputs
        .iter()
        .filter(|name| !source_set.contains(*name))
        .collect();
    assert!(
        missing_inputs.is_empty(),
        "Reduction {} -> {}: overhead references input variables {:?} \
         not in source problem_size_names {:?}",
        source_name,
        target_name,
        missing_inputs,
        source_set,
    );

    if !target_size_names.is_empty() {
        let target_set: HashSet<&str> = target_size_names.iter().copied().collect();
        let overhead_outputs: HashSet<&str> =
            overhead.output_size.iter().map(|(name, _)| *name).collect();
        let missing_outputs: Vec<_> = overhead_outputs
            .iter()
            .filter(|name| !target_set.contains(*name))
            .collect();
        assert!(
            missing_outputs.is_empty(),
            "Reduction {} -> {}: overhead output fields {:?} \
             not in target problem_size_names {:?}",
            source_name,
            target_name,
            missing_outputs,
            target_set,
        );
    }
}

/// Runtime graph of all registered reductions.
///
/// Uses variant-level nodes: each node is a unique `(problem_name, variant)` pair.
/// All edges come from `inventory::iter::<ReductionEntry>` registrations.
///
/// The graph supports:
/// - Auto-discovery of reductions from `inventory::iter::<ReductionEntry>`
/// - Dijkstra with custom cost functions
/// - Path finding by problem type or by name
pub struct ReductionGraph {
    /// Graph with node indices as node data, edge weights as ReductionEdgeData.
    graph: DiGraph<usize, ReductionEdgeData>,
    /// All variant nodes, indexed by position.
    nodes: Vec<VariantNode>,
    /// Map from base type name to all NodeIndex values for that name.
    name_to_nodes: HashMap<&'static str, Vec<NodeIndex>>,
}

impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions from inventory.
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut nodes: Vec<VariantNode> = Vec::new();
        let mut node_index: HashMap<VariantRef, NodeIndex> = HashMap::new();
        let mut name_to_nodes: HashMap<&'static str, Vec<NodeIndex>> = HashMap::new();

        // Helper to ensure a variant node exists in the graph.
        let ensure_node = |name: &'static str,
                           variant: BTreeMap<String, String>,
                           nodes: &mut Vec<VariantNode>,
                           graph: &mut DiGraph<usize, ReductionEdgeData>,
                           node_index: &mut HashMap<VariantRef, NodeIndex>,
                           name_to_nodes: &mut HashMap<&'static str, Vec<NodeIndex>>|
         -> NodeIndex {
            let vref = VariantRef {
                name: name.to_string(),
                variant: variant.clone(),
            };
            if let Some(&idx) = node_index.get(&vref) {
                idx
            } else {
                let node_id = nodes.len();
                nodes.push(VariantNode { name, variant });
                let idx = graph.add_node(node_id);
                node_index.insert(vref, idx);
                name_to_nodes.entry(name).or_default().push(idx);
                idx
            }
        };

        // Register reductions from inventory (auto-discovery)
        for entry in inventory::iter::<ReductionEntry> {
            let source_variant = Self::variant_to_map(&entry.source_variant());
            let target_variant = Self::variant_to_map(&entry.target_variant());

            let src_idx = ensure_node(
                entry.source_name,
                source_variant,
                &mut nodes,
                &mut graph,
                &mut node_index,
                &mut name_to_nodes,
            );
            let dst_idx = ensure_node(
                entry.target_name,
                target_variant,
                &mut nodes,
                &mut graph,
                &mut node_index,
                &mut name_to_nodes,
            );

            let overhead = entry.overhead();
            validate_overhead_variables(
                entry.source_name,
                entry.target_name,
                &overhead,
                (entry.source_size_names_fn)(),
                (entry.target_size_names_fn)(),
            );

            // Check if edge already exists (avoid duplicates)
            if graph.find_edge(src_idx, dst_idx).is_none() {
                graph.add_edge(
                    src_idx,
                    dst_idx,
                    ReductionEdgeData {
                        overhead,
                        reduce_fn: entry.reduce_fn,
                    },
                );
            }
        }

        Self {
            graph,
            nodes,
            name_to_nodes,
        }
    }

    /// Convert a variant slice to a BTreeMap.
    /// Normalizes empty "graph" values to "SimpleGraph" for consistency.
    pub fn variant_to_map(variant: &[(&str, &str)]) -> BTreeMap<String, String> {
        variant
            .iter()
            .map(|(k, v)| {
                let value = if *k == "graph" && v.is_empty() {
                    "SimpleGraph".to_string()
                } else {
                    v.to_string()
                };
                (k.to_string(), value)
            })
            .collect()
    }

    /// Look up a variant node by name and variant map.
    fn lookup_node(&self, name: &str, variant: &BTreeMap<String, String>) -> Option<NodeIndex> {
        let nodes = self.name_to_nodes.get(name)?;
        nodes
            .iter()
            .find(|&&idx| self.nodes[self.graph[idx]].variant == *variant)
            .copied()
    }

    /// Find the cheapest path between two specific problem variants.
    ///
    /// Uses Dijkstra's algorithm on the variant-level graph from the exact
    /// source variant node to the exact target variant node.
    pub fn find_cheapest_path<C: PathCostFn>(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        input_size: &ProblemSize,
        cost_fn: &C,
    ) -> Option<ReductionPath> {
        let src = self.lookup_node(source, source_variant)?;

        // Validate: when input_size is non-empty, check outgoing edges
        if !input_size.components.is_empty() {
            let size_names: Vec<&str> = input_size
                .components
                .iter()
                .map(|(k, _)| k.as_str())
                .collect();
            for edge_ref in self.graph.edges(src) {
                let target_node = &self.nodes[self.graph[edge_ref.target()]];
                validate_overhead_variables(
                    source,
                    target_node.name,
                    &edge_ref.weight().overhead,
                    &size_names,
                    &[], // skip output validation at query time
                );
            }
        }

        let dst = self.lookup_node(target, target_variant)?;
        let node_path = self.dijkstra(src, dst, input_size, cost_fn)?;
        Some(self.node_path_to_reduction_path(&node_path))
    }

    /// Core Dijkstra search on node indices.
    fn dijkstra<C: PathCostFn>(
        &self,
        src: NodeIndex,
        dst: NodeIndex,
        input_size: &ProblemSize,
        cost_fn: &C,
    ) -> Option<Vec<NodeIndex>> {
        let mut costs: HashMap<NodeIndex, f64> = HashMap::new();
        let mut sizes: HashMap<NodeIndex, ProblemSize> = HashMap::new();
        let mut prev: HashMap<NodeIndex, NodeIndex> = HashMap::new();
        let mut heap = BinaryHeap::new();

        costs.insert(src, 0.0);
        sizes.insert(src, input_size.clone());
        heap.push(Reverse((OrderedFloat(0.0), src)));

        while let Some(Reverse((cost, node))) = heap.pop() {
            if node == dst {
                let mut path = vec![dst];
                let mut current = dst;
                while current != src {
                    let &prev_node = prev.get(&current)?;
                    path.push(prev_node);
                    current = prev_node;
                }
                path.reverse();
                return Some(path);
            }

            if cost.0 > *costs.get(&node).unwrap_or(&f64::INFINITY) {
                continue;
            }

            let current_size = match sizes.get(&node) {
                Some(s) => s.clone(),
                None => continue,
            };

            for edge_ref in self.graph.edges(node) {
                let overhead = &edge_ref.weight().overhead;
                let next = edge_ref.target();

                let edge_cost = cost_fn.edge_cost(overhead, &current_size);
                let new_cost = cost.0 + edge_cost;
                let new_size = overhead.evaluate_output_size(&current_size);

                if new_cost < *costs.get(&next).unwrap_or(&f64::INFINITY) {
                    costs.insert(next, new_cost);
                    sizes.insert(next, new_size);
                    prev.insert(next, node);
                    heap.push(Reverse((OrderedFloat(new_cost), next)));
                }
            }
        }

        None
    }

    /// Convert a node index path to a `ReductionPath`.
    fn node_path_to_reduction_path(&self, node_path: &[NodeIndex]) -> ReductionPath {
        let steps = node_path
            .iter()
            .map(|&idx| {
                let node = &self.nodes[self.graph[idx]];
                ReductionStep {
                    name: node.name.to_string(),
                    variant: node.variant.clone(),
                }
            })
            .collect();
        ReductionPath { steps }
    }

    /// Find all simple paths between two specific problem variants.
    ///
    /// Uses `all_simple_paths` on the variant-level graph from the exact
    /// source variant node to the exact target variant node.
    pub fn find_all_paths(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
    ) -> Vec<ReductionPath> {
        let src = match self.lookup_node(source, source_variant) {
            Some(idx) => idx,
            None => return vec![],
        };
        let dst = match self.lookup_node(target, target_variant) {
            Some(idx) => idx,
            None => return vec![],
        };

        let paths: Vec<Vec<NodeIndex>> = all_simple_paths::<
            Vec<NodeIndex>,
            _,
            std::hash::RandomState,
        >(&self.graph, src, dst, 0, None)
        .collect();

        paths
            .iter()
            .map(|p| self.node_path_to_reduction_path(p))
            .collect()
    }

    /// Check if a direct reduction exists from S to T.
    pub fn has_direct_reduction<S: crate::traits::Problem, T: crate::traits::Problem>(
        &self,
    ) -> bool {
        self.has_direct_reduction_by_name(S::NAME, T::NAME)
    }

    /// Check if a direct reduction exists by name.
    pub fn has_direct_reduction_by_name(&self, src: &str, dst: &str) -> bool {
        let src_nodes = match self.name_to_nodes.get(src) {
            Some(nodes) => nodes,
            None => return false,
        };
        let dst_nodes = match self.name_to_nodes.get(dst) {
            Some(nodes) => nodes,
            None => return false,
        };

        let dst_set: HashSet<NodeIndex> = dst_nodes.iter().copied().collect();

        for &src_idx in src_nodes {
            for edge_ref in self.graph.edges(src_idx) {
                if dst_set.contains(&edge_ref.target()) {
                    return true;
                }
            }
        }

        false
    }

    /// Get all registered problem type names (base names).
    pub fn problem_types(&self) -> Vec<&'static str> {
        self.name_to_nodes.keys().copied().collect()
    }

    /// Get the number of registered problem types (unique base names).
    pub fn num_types(&self) -> usize {
        self.name_to_nodes.len()
    }

    /// Get the number of registered reductions (edges).
    pub fn num_reductions(&self) -> usize {
        self.graph.edge_count()
    }

    /// Get the number of variant-level nodes.
    pub fn num_variant_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Get the per-edge overhead polynomials along a reduction path.
    ///
    /// Returns one `ReductionOverhead` per edge (i.e., `path.steps.len() - 1` items).
    ///
    /// Panics if any step in the path does not correspond to an edge in the graph.
    pub fn path_overheads(&self, path: &ReductionPath) -> Vec<ReductionOverhead> {
        if path.steps.len() <= 1 {
            return vec![];
        }

        let node_indices: Vec<NodeIndex> = path
            .steps
            .iter()
            .map(|step| {
                self.lookup_node(&step.name, &step.variant)
                    .unwrap_or_else(|| panic!("Node not found: {} {:?}", step.name, step.variant))
            })
            .collect();

        node_indices
            .windows(2)
            .map(|pair| {
                let edge_idx = self.graph.find_edge(pair[0], pair[1]).unwrap_or_else(|| {
                    let src = &self.nodes[self.graph[pair[0]]];
                    let dst = &self.nodes[self.graph[pair[1]]];
                    panic!(
                        "No edge from {} {:?} to {} {:?}",
                        src.name, src.variant, dst.name, dst.variant
                    )
                });
                self.graph[edge_idx].overhead.clone()
            })
            .collect()
    }

    /// Compose overheads along a path symbolically.
    ///
    /// Returns a single `ReductionOverhead` whose polynomials map from the
    /// source problem's size variables directly to the final target's size variables.
    pub fn compose_path_overhead(&self, path: &ReductionPath) -> ReductionOverhead {
        self.path_overheads(path)
            .into_iter()
            .reduce(|acc, oh| acc.compose(&oh))
            .unwrap_or_default()
    }

    /// Get all variant maps registered for a problem name.
    ///
    /// Returns an empty `Vec` if the name is not found.
    pub fn variants_for(&self, name: &str) -> Vec<BTreeMap<String, String>> {
        self.name_to_nodes
            .get(name)
            .map(|indices| {
                indices
                    .iter()
                    .map(|&idx| self.nodes[self.graph[idx]].variant.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all outgoing reductions from a problem (across all its variants).
    pub fn outgoing_reductions(&self, name: &str) -> Vec<ReductionEdgeInfo> {
        let Some(indices) = self.name_to_nodes.get(name) else {
            return vec![];
        };
        let index_set: HashSet<NodeIndex> = indices.iter().copied().collect();
        self.graph
            .edge_references()
            .filter(|e| index_set.contains(&e.source()))
            .map(|e| {
                let src = &self.nodes[self.graph[e.source()]];
                let dst = &self.nodes[self.graph[e.target()]];
                ReductionEdgeInfo {
                    source_name: src.name,
                    source_variant: src.variant.clone(),
                    target_name: dst.name,
                    target_variant: dst.variant.clone(),
                }
            })
            .collect()
    }

    /// Get the problem size field names for a problem type.
    ///
    /// Returns the static `problem_size_names()` by finding a reduction entry
    /// where this problem is the source or target.
    pub fn size_field_names(&self, name: &str) -> &'static [&'static str] {
        for entry in inventory::iter::<ReductionEntry> {
            if entry.source_name == name {
                return (entry.source_size_names_fn)();
            }
            if entry.target_name == name {
                return (entry.target_size_names_fn)();
            }
        }
        &[]
    }

    /// Get all incoming reductions to a problem (across all its variants).
    pub fn incoming_reductions(&self, name: &str) -> Vec<ReductionEdgeInfo> {
        let Some(indices) = self.name_to_nodes.get(name) else {
            return vec![];
        };
        let index_set: HashSet<NodeIndex> = indices.iter().copied().collect();
        self.graph
            .edge_references()
            .filter(|e| index_set.contains(&e.target()))
            .map(|e| {
                let src = &self.nodes[self.graph[e.source()]];
                let dst = &self.nodes[self.graph[e.target()]];
                ReductionEdgeInfo {
                    source_name: src.name,
                    source_variant: src.variant.clone(),
                    target_name: dst.name,
                    target_variant: dst.variant.clone(),
                }
            })
            .collect()
    }

    /// Find all problems reachable within `max_hops` edges from a starting node.
    ///
    /// Returns neighbors sorted by (hops, name). The starting node itself is excluded.
    /// If a node is reachable at multiple distances, it appears at the shortest distance only.
    pub fn k_neighbors(
        &self,
        name: &str,
        variant: &BTreeMap<String, String>,
        max_hops: usize,
        direction: TraversalDirection,
    ) -> Vec<NeighborInfo> {
        use std::collections::VecDeque;

        let Some(start_idx) = self.lookup_node(name, variant) else {
            return vec![];
        };

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        visited.insert(start_idx);
        let mut queue: VecDeque<(NodeIndex, usize)> = VecDeque::new();
        queue.push_back((start_idx, 0));
        let mut results: Vec<NeighborInfo> = Vec::new();

        while let Some((node_idx, hops)) = queue.pop_front() {
            if hops >= max_hops {
                continue;
            }

            let directions: Vec<petgraph::Direction> = match direction {
                TraversalDirection::Outgoing => vec![petgraph::Direction::Outgoing],
                TraversalDirection::Incoming => vec![petgraph::Direction::Incoming],
                TraversalDirection::Both => {
                    vec![petgraph::Direction::Outgoing, petgraph::Direction::Incoming]
                }
            };

            for dir in directions {
                for neighbor_idx in self.graph.neighbors_directed(node_idx, dir) {
                    if visited.insert(neighbor_idx) {
                        let neighbor_node = &self.nodes[self.graph[neighbor_idx]];
                        results.push(NeighborInfo {
                            name: neighbor_node.name,
                            variant: neighbor_node.variant.clone(),
                            hops: hops + 1,
                        });
                        queue.push_back((neighbor_idx, hops + 1));
                    }
                }
            }
        }

        results.sort_by(|a, b| a.hops.cmp(&b.hops).then_with(|| a.name.cmp(b.name)));
        results
    }

    /// Build a tree of neighbors via BFS with parent tracking.
    ///
    /// Returns the children of the starting node as a forest of `NeighborTree` nodes.
    /// Each node appears at most once (shortest-path tree). Children are sorted by name.
    pub fn k_neighbor_tree(
        &self,
        name: &str,
        variant: &BTreeMap<String, String>,
        max_hops: usize,
        direction: TraversalDirection,
    ) -> Vec<NeighborTree> {
        use std::collections::VecDeque;

        let Some(start_idx) = self.lookup_node(name, variant) else {
            return vec![];
        };

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        visited.insert(start_idx);

        let mut queue: VecDeque<(NodeIndex, usize)> = VecDeque::new();
        queue.push_back((start_idx, 0));

        // Map from node_idx -> children node indices
        let mut node_children: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();

        while let Some((node_idx, depth)) = queue.pop_front() {
            if depth >= max_hops {
                continue;
            }

            let directions: Vec<petgraph::Direction> = match direction {
                TraversalDirection::Outgoing => vec![petgraph::Direction::Outgoing],
                TraversalDirection::Incoming => vec![petgraph::Direction::Incoming],
                TraversalDirection::Both => {
                    vec![petgraph::Direction::Outgoing, petgraph::Direction::Incoming]
                }
            };

            let mut children = Vec::new();
            for dir in directions {
                for neighbor_idx in self.graph.neighbors_directed(node_idx, dir) {
                    if visited.insert(neighbor_idx) {
                        children.push(neighbor_idx);
                        queue.push_back((neighbor_idx, depth + 1));
                    }
                }
            }
            children.sort_by(|a, b| {
                self.nodes[self.graph[*a]]
                    .name
                    .cmp(self.nodes[self.graph[*b]].name)
            });
            node_children.insert(node_idx, children);
        }

        // Recursively build NeighborTree from BFS parent map.
        fn build(
            idx: NodeIndex,
            node_children: &HashMap<NodeIndex, Vec<NodeIndex>>,
            nodes: &[VariantNode],
            graph: &DiGraph<usize, ReductionEdgeData>,
        ) -> NeighborTree {
            let children = node_children
                .get(&idx)
                .map(|cs| {
                    cs.iter()
                        .map(|&c| build(c, node_children, nodes, graph))
                        .collect()
                })
                .unwrap_or_default();
            let node = &nodes[graph[idx]];
            NeighborTree {
                name: node.name.to_string(),
                variant: node.variant.clone(),
                children,
            }
        }

        node_children
            .get(&start_idx)
            .map(|cs| {
                cs.iter()
                    .map(|&c| build(c, &node_children, &self.nodes, &self.graph))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Default for ReductionGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ReductionGraph {
    /// Export the reduction graph as a JSON-serializable structure.
    ///
    /// Nodes and edges come directly from the variant-level graph.
    pub(crate) fn to_json(&self) -> ReductionGraphJson {
        use crate::registry::ProblemSchemaEntry;

        // Build name -> module_path lookup from ProblemSchemaEntry inventory
        let schema_modules: HashMap<&str, &str> = inventory::iter::<ProblemSchemaEntry>
            .into_iter()
            .map(|entry| (entry.name, entry.module_path))
            .collect();

        // Build sorted node list from the internal nodes
        let mut json_nodes: Vec<(usize, NodeJson)> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let (category, doc_path) = if let Some(&mod_path) = schema_modules.get(node.name) {
                    (
                        Self::category_from_module_path(mod_path),
                        Self::doc_path_from_module_path(mod_path, node.name),
                    )
                } else {
                    ("other".to_string(), String::new())
                };
                (
                    i,
                    NodeJson {
                        name: node.name.to_string(),
                        variant: node.variant.clone(),
                        category,
                        doc_path,
                    },
                )
            })
            .collect();
        json_nodes.sort_by(|a, b| (&a.1.name, &a.1.variant).cmp(&(&b.1.name, &b.1.variant)));

        // Build old-index -> new-index mapping
        let mut old_to_new: HashMap<usize, usize> = HashMap::new();
        for (new_idx, (old_idx, _)) in json_nodes.iter().enumerate() {
            old_to_new.insert(*old_idx, new_idx);
        }

        let nodes: Vec<NodeJson> = json_nodes.into_iter().map(|(_, n)| n).collect();

        // Build edges from the graph
        let mut edges: Vec<EdgeJson> = Vec::new();
        for edge_ref in self.graph.edge_references() {
            let src_node_id = self.graph[edge_ref.source()];
            let dst_node_id = self.graph[edge_ref.target()];
            let overhead = &edge_ref.weight().overhead;

            let overhead_fields = overhead
                .output_size
                .iter()
                .map(|(field, poly)| OverheadFieldJson {
                    field: field.to_string(),
                    formula: poly.to_string(),
                })
                .collect();

            // Find the doc_path from the matching ReductionEntry
            let src_name = self.nodes[src_node_id].name;
            let dst_name = self.nodes[dst_node_id].name;
            let src_variant = &self.nodes[src_node_id].variant;
            let dst_variant = &self.nodes[dst_node_id].variant;

            let doc_path = self.find_entry_doc_path(src_name, dst_name, src_variant, dst_variant);

            edges.push(EdgeJson {
                source: old_to_new[&src_node_id],
                target: old_to_new[&dst_node_id],
                overhead: overhead_fields,
                doc_path,
            });
        }

        // Sort edges for deterministic output
        edges.sort_by(|a, b| {
            (
                &nodes[a.source].name,
                &nodes[a.source].variant,
                &nodes[a.target].name,
                &nodes[a.target].variant,
            )
                .cmp(&(
                    &nodes[b.source].name,
                    &nodes[b.source].variant,
                    &nodes[b.target].name,
                    &nodes[b.target].variant,
                ))
        });

        ReductionGraphJson { nodes, edges }
    }

    /// Find the doc_path for a reduction entry matching the given source/target.
    fn find_entry_doc_path(
        &self,
        src_name: &str,
        dst_name: &str,
        src_variant: &BTreeMap<String, String>,
        dst_variant: &BTreeMap<String, String>,
    ) -> String {
        for entry in inventory::iter::<ReductionEntry> {
            if entry.source_name == src_name && entry.target_name == dst_name {
                let entry_src = Self::variant_to_map(&entry.source_variant());
                let entry_dst = Self::variant_to_map(&entry.target_variant());
                if &entry_src == src_variant && &entry_dst == dst_variant {
                    return Self::module_path_to_doc_path(entry.module_path);
                }
            }
        }
        String::new()
    }

    /// Export the reduction graph as a JSON string.
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        let json = self.to_json();
        serde_json::to_string_pretty(&json)
    }

    /// Export the reduction graph to a JSON file.
    pub fn to_json_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json_string = self
            .to_json_string()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json_string)
    }

    /// Convert a module path to a rustdoc relative path.
    ///
    /// E.g., `"problemreductions::rules::spinglass_qubo"` -> `"rules/spinglass_qubo/index.html"`.
    fn module_path_to_doc_path(module_path: &str) -> String {
        let stripped = module_path
            .strip_prefix("problemreductions::")
            .unwrap_or(module_path);
        format!("{}/index.html", stripped.replace("::", "/"))
    }

    /// Extract the category from a module path.
    ///
    /// E.g., `"problemreductions::models::graph::maximum_independent_set"` -> `"graph"`.
    fn category_from_module_path(module_path: &str) -> String {
        classify_problem_category(module_path).to_string()
    }

    /// Build the rustdoc path from a module path and problem name.
    ///
    /// E.g., `"problemreductions::models::graph::maximum_independent_set"`, `"MaximumIndependentSet"`
    /// -> `"models/graph/struct.MaximumIndependentSet.html"`.
    fn doc_path_from_module_path(module_path: &str, name: &str) -> String {
        let stripped = module_path
            .strip_prefix("problemreductions::")
            .unwrap_or(module_path);
        if let Some(parent) = stripped.rsplit_once("::").map(|(p, _)| p) {
            format!("{}/struct.{}.html", parent.replace("::", "/"), name)
        } else {
            format!("struct.{}.html", name)
        }
    }

    /// Find the best matching `ReductionEntry` for a (source_name, target_name) pair
    /// given the caller's current source variant.
    ///
    /// First tries an exact match on the source variant. If no exact match is found,
    /// falls back to a name-only match (returning the first entry whose source and
    /// target names match). This is intentional: specific variants (e.g., `K3`) may
    /// not have their own `#[reduction]` entry, but the general variant (`KN`) covers
    /// them with the same overhead polynomial. The fallback is safe because cross-name
    /// reductions share the same overhead regardless of source variant; it is only
    /// used by the JSON export pipeline (`export::lookup_overhead`).
    pub fn find_best_entry(
        &self,
        source_name: &str,
        target_name: &str,
        current_variant: &BTreeMap<String, String>,
    ) -> Option<MatchedEntry> {
        let mut fallback: Option<MatchedEntry> = None;

        for entry in inventory::iter::<ReductionEntry> {
            if entry.source_name != source_name || entry.target_name != target_name {
                continue;
            }

            let entry_source = Self::variant_to_map(&entry.source_variant());
            let entry_target = Self::variant_to_map(&entry.target_variant());

            // Exact match on source variant — return immediately
            if current_variant == &entry_source {
                return Some(MatchedEntry {
                    source_variant: entry_source,
                    target_variant: entry_target,
                    overhead: entry.overhead(),
                });
            }

            // Remember the first name-only match as a fallback
            if fallback.is_none() {
                fallback = Some(MatchedEntry {
                    source_variant: entry_source,
                    target_variant: entry_target,
                    overhead: entry.overhead(),
                });
            }
        }

        fallback
    }
}

/// A matched reduction entry returned by [`ReductionGraph::find_best_entry`].
pub struct MatchedEntry {
    /// The entry's source variant.
    pub source_variant: BTreeMap<String, String>,
    /// The entry's target variant.
    pub target_variant: BTreeMap<String, String>,
    /// The overhead of the reduction.
    pub overhead: ReductionOverhead,
}

/// A composed reduction chain produced by [`ReductionGraph::reduce_along_path`].
///
/// Holds the intermediate reduction results from executing a multi-step
/// reduction path. Provides access to the final target problem and
/// solution extraction back to the source problem space.
pub struct ReductionChain {
    steps: Vec<Box<dyn DynReductionResult>>,
}

impl ReductionChain {
    /// Get the final target problem as a type-erased reference.
    pub fn target_problem_any(&self) -> &dyn Any {
        self.steps
            .last()
            .expect("ReductionChain has no steps")
            .target_problem_any()
    }

    /// Get a typed reference to the final target problem.
    ///
    /// Panics if the actual target type does not match `T`.
    pub fn target_problem<T: 'static>(&self) -> &T {
        self.target_problem_any()
            .downcast_ref::<T>()
            .expect("ReductionChain target type mismatch")
    }

    /// Extract a solution from target space back to source space.
    pub fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.steps
            .iter()
            .rev()
            .fold(target_solution.to_vec(), |sol, step| {
                step.extract_solution_dyn(&sol)
            })
    }
}

impl ReductionGraph {
    /// Execute a reduction path on a source problem instance.
    ///
    /// Looks up each edge's `reduce_fn`, chains them, and returns the
    /// resulting [`ReductionChain`]. The source must be passed as `&dyn Any`
    /// (use `&problem as &dyn Any` or pass a concrete reference directly).
    ///
    /// # Example
    ///
    /// ```text
    /// let chain = graph.reduce_along_path(&path, &source_problem)?;
    /// let target: &QUBO<f64> = chain.target_problem();
    /// let source_solution = chain.extract_solution(&target_solution);
    /// ```
    pub fn reduce_along_path(
        &self,
        path: &ReductionPath,
        source: &dyn Any,
    ) -> Option<ReductionChain> {
        if path.steps.len() < 2 {
            return None;
        }
        // Collect edge reduce_fns
        let mut edge_fns = Vec::new();
        for window in path.steps.windows(2) {
            let src = self.lookup_node(&window[0].name, &window[0].variant)?;
            let dst = self.lookup_node(&window[1].name, &window[1].variant)?;
            let edge_idx = self.graph.find_edge(src, dst)?;
            edge_fns.push(self.graph[edge_idx].reduce_fn);
        }
        // Execute the chain
        let mut steps: Vec<Box<dyn DynReductionResult>> = Vec::new();
        let step = (edge_fns[0])(source);
        steps.push(step);
        for edge_fn in &edge_fns[1..] {
            let step = {
                let prev_target = steps.last().unwrap().target_problem_any();
                edge_fn(prev_target)
            };
            steps.push(step);
        }
        Some(ReductionChain { steps })
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/graph.rs"]
mod tests;

#[cfg(test)]
#[path = "../unit_tests/rules/reduction_path_parity.rs"]
mod reduction_path_parity_tests;
