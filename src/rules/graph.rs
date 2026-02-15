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
use crate::types::ProblemSize;
use ordered_float::OrderedFloat;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

/// JSON-serializable representation of the reduction graph.
#[derive(Debug, Clone, Serialize)]
pub struct ReductionGraphJson {
    /// List of problem type nodes.
    pub nodes: Vec<NodeJson>,
    /// List of reduction edges.
    pub edges: Vec<EdgeJson>,
}

impl ReductionGraphJson {
    /// Get the source node of an edge.
    pub fn source_node(&self, edge: &EdgeJson) -> &NodeJson {
        &self.nodes[edge.source]
    }

    /// Get the target node of an edge.
    pub fn target_node(&self, edge: &EdgeJson) -> &NodeJson {
        &self.nodes[edge.target]
    }
}

/// A node in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub struct NodeJson {
    /// Base problem name (e.g., "MaximumIndependentSet").
    pub name: String,
    /// Variant attributes as key-value pairs.
    pub variant: BTreeMap<String, String>,
    /// Category of the problem (e.g., "graph", "set", "optimization", "satisfiability", "specialized").
    pub category: String,
    /// Relative rustdoc path (e.g., "models/graph/maximum_independent_set").
    pub doc_path: String,
}

/// Internal reference to a problem variant, used as HashMap key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VariantRef {
    name: String,
    variant: BTreeMap<String, String>,
}

/// A single output field in the reduction overhead.
#[derive(Debug, Clone, Serialize)]
pub struct OverheadFieldJson {
    /// Output field name (e.g., "num_vars").
    pub field: String,
    /// Formula as a human-readable string (e.g., "num_vertices").
    pub formula: String,
}

/// An edge in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub struct EdgeJson {
    /// Index into the `nodes` array for the source problem variant.
    pub source: usize,
    /// Index into the `nodes` array for the target problem variant.
    pub target: usize,
    /// Reduction overhead: output size as polynomials of input size.
    pub overhead: Vec<OverheadFieldJson>,
    /// Relative rustdoc path for the reduction module.
    pub doc_path: String,
}

/// A path through the reduction graph.
#[derive(Debug, Clone)]
pub struct ReductionPath {
    /// Human-readable type names in the path (base names without type parameters).
    pub type_names: Vec<&'static str>,
}

impl ReductionPath {
    /// Get the length of the path (number of reductions).
    pub fn len(&self) -> usize {
        if self.type_names.is_empty() {
            0
        } else {
            self.type_names.len() - 1
        }
    }

    /// Check if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.type_names.is_empty()
    }

    /// Get the source type name.
    pub fn source(&self) -> Option<&'static str> {
        self.type_names.first().copied()
    }

    /// Get the target type name.
    pub fn target(&self) -> Option<&'static str> {
        self.type_names.last().copied()
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

/// The kind of transition between adjacent steps in a resolved path.
#[derive(Debug, Clone, Serialize)]
pub enum EdgeKind {
    /// A registered reduction (backed by a ReduceTo impl).
    Reduction {
        /// Overhead from the matching ReductionEntry.
        overhead: ReductionOverhead,
    },
}

/// A fully resolved reduction path with variant information at each node.
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedPath {
    /// Sequence of (name, variant) nodes.
    pub steps: Vec<ReductionStep>,
    /// Edge kinds between adjacent steps. Length = steps.len() - 1.
    pub edges: Vec<EdgeKind>,
}

impl ResolvedPath {
    /// Number of edges in the path.
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Whether the path is empty.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Number of registered reduction steps.
    pub fn num_reductions(&self) -> usize {
        self.edges
            .iter()
            .filter(|e| matches!(e, EdgeKind::Reduction { .. }))
            .count()
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
    /// Graph with node indices as node data, edge weights as ReductionOverhead.
    graph: DiGraph<usize, ReductionOverhead>,
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
                           graph: &mut DiGraph<usize, ReductionOverhead>,
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

            // Check if edge already exists (avoid duplicates)
            if graph.find_edge(src_idx, dst_idx).is_none() {
                graph.add_edge(src_idx, dst_idx, entry.overhead());
            }
        }

        Self {
            graph,
            nodes,
            name_to_nodes,
        }
    }

    /// Helper to convert a variant slice to a BTreeMap.
    /// Normalizes empty "graph" values to "SimpleGraph" for consistency.
    fn variant_to_map(variant: &[(&str, &str)]) -> BTreeMap<String, String> {
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

    /// Find the cheapest path using a custom cost function.
    ///
    /// Uses Dijkstra's algorithm on the variant-level graph.
    ///
    /// # Arguments
    /// - `source`: (problem_name, graph_type) for source — used to look up the variant node
    /// - `target`: (problem_name, graph_type) for target
    /// - `input_size`: Initial problem size for cost calculations
    /// - `cost_fn`: Custom cost function for path optimization
    ///
    /// # Returns
    /// The cheapest path if one exists.
    pub fn find_cheapest_path<C: PathCostFn>(
        &self,
        source: (&str, &str),
        target: (&str, &str),
        input_size: &ProblemSize,
        cost_fn: &C,
    ) -> Option<ReductionPath> {
        // Find source nodes matching the name (we try all variant nodes for that name)
        let src_nodes = self.name_to_nodes.get(source.0)?;
        let dst_nodes = self.name_to_nodes.get(target.0)?;

        // Build set of target node indices for quick lookup
        let dst_set: HashSet<NodeIndex> = dst_nodes.iter().copied().collect();

        let mut best_path: Option<(f64, ReductionPath)> = None;

        // Try from each source node
        for &src_idx in src_nodes {
            let mut costs: HashMap<NodeIndex, f64> = HashMap::new();
            let mut sizes: HashMap<NodeIndex, ProblemSize> = HashMap::new();
            let mut prev: HashMap<NodeIndex, (NodeIndex, petgraph::graph::EdgeIndex)> =
                HashMap::new();
            let mut heap = BinaryHeap::new();

            costs.insert(src_idx, 0.0);
            sizes.insert(src_idx, input_size.clone());
            heap.push(Reverse((OrderedFloat(0.0), src_idx)));

            while let Some(Reverse((cost, node))) = heap.pop() {
                if dst_set.contains(&node) {
                    let path = self.reconstruct_path_names(&prev, src_idx, node);
                    if best_path.as_ref().map(|(c, _)| cost.0 < *c).unwrap_or(true) {
                        best_path = Some((cost.0, path));
                    }
                    continue;
                }

                if cost.0 > *costs.get(&node).unwrap_or(&f64::INFINITY) {
                    continue;
                }

                let current_size = match sizes.get(&node) {
                    Some(s) => s.clone(),
                    None => continue,
                };

                for edge_ref in self.graph.edges(node) {
                    let overhead = edge_ref.weight();
                    let next = edge_ref.target();

                    let edge_cost = cost_fn.edge_cost(overhead, &current_size);
                    let new_cost = cost.0 + edge_cost;
                    let new_size = overhead.evaluate_output_size(&current_size);

                    if new_cost < *costs.get(&next).unwrap_or(&f64::INFINITY) {
                        costs.insert(next, new_cost);
                        sizes.insert(next, new_size);
                        prev.insert(next, (node, edge_ref.id()));
                        heap.push(Reverse((OrderedFloat(new_cost), next)));
                    }
                }
            }
        }

        best_path.map(|(_, p)| p)
    }

    /// Reconstruct a name-level path from the predecessor map.
    fn reconstruct_path_names(
        &self,
        prev: &HashMap<NodeIndex, (NodeIndex, petgraph::graph::EdgeIndex)>,
        src: NodeIndex,
        dst: NodeIndex,
    ) -> ReductionPath {
        let mut path = vec![self.nodes[self.graph[dst]].name];
        let mut current = dst;

        while current != src {
            if let Some(&(prev_node, _)) = prev.get(&current) {
                path.push(self.nodes[self.graph[prev_node]].name);
                current = prev_node;
            } else {
                break;
            }
        }

        path.reverse();
        // Deduplicate consecutive names (variant-level hops within same problem)
        let mut deduped = Vec::new();
        for name in &path {
            if deduped.last() != Some(name) {
                deduped.push(*name);
            }
        }
        ReductionPath {
            type_names: deduped,
        }
    }

    /// Find all paths from source to target type.
    ///
    /// Uses `Problem::NAME` for lookup. With variant-level nodes, this finds
    /// paths between any variant of S and any variant of T.
    pub fn find_paths<S: crate::traits::Problem, T: crate::traits::Problem>(
        &self,
    ) -> Vec<ReductionPath> {
        self.find_paths_by_name(S::NAME, T::NAME)
    }

    /// Find all paths between problem types by name.
    pub fn find_paths_by_name(&self, src: &str, dst: &str) -> Vec<ReductionPath> {
        let src_nodes = match self.name_to_nodes.get(src) {
            Some(nodes) => nodes.clone(),
            None => return vec![],
        };
        let dst_nodes = match self.name_to_nodes.get(dst) {
            Some(nodes) => nodes.clone(),
            None => return vec![],
        };

        let mut all_paths = Vec::new();
        let mut seen_name_paths: HashSet<Vec<&'static str>> = HashSet::new();

        for &src_idx in &src_nodes {
            for &dst_idx in &dst_nodes {
                let paths: Vec<Vec<NodeIndex>> = all_simple_paths::<
                    Vec<NodeIndex>,
                    _,
                    std::hash::RandomState,
                >(
                    &self.graph, src_idx, dst_idx, 0, None
                )
                .collect();

                for path in paths {
                    // Convert variant-level path to name-level path (dedup consecutive same-name nodes)
                    let mut type_names: Vec<&'static str> = Vec::new();
                    for &idx in &path {
                        let name = self.nodes[self.graph[idx]].name;
                        if type_names.last() != Some(&name) {
                            type_names.push(name);
                        }
                    }
                    if !seen_name_paths.contains(&type_names) {
                        seen_name_paths.insert(type_names.clone());
                        all_paths.push(ReductionPath { type_names });
                    }
                }
            }
        }

        all_paths
    }

    /// Find the shortest path from source to target type.
    pub fn find_shortest_path<S: crate::traits::Problem, T: crate::traits::Problem>(
        &self,
    ) -> Option<ReductionPath> {
        let paths = self.find_paths::<S, T>();
        paths.into_iter().min_by_key(|p| p.len())
    }

    /// Find the shortest path by name.
    pub fn find_shortest_path_by_name(&self, src: &str, dst: &str) -> Option<ReductionPath> {
        let paths = self.find_paths_by_name(src, dst);
        paths.into_iter().min_by_key(|p| p.len())
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
    pub fn to_json(&self) -> ReductionGraphJson {
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
            let overhead = edge_ref.weight();

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
    /// target names match). This allows looking up overhead for specific variants
    /// (e.g., `K3`) when only the general variant (e.g., `KN`) is registered.
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

#[cfg(test)]
#[path = "../unit_tests/rules/graph.rs"]
mod tests;
