//! Runtime reduction graph for discovering and executing reduction paths.
//!
//! The graph uses type-erased names (e.g., "SpinGlass" instead of "SpinGlass<SimpleGraph, i32>")
//! for topology, allowing path finding regardless of weight type parameters.
//!
//! This module implements set-theoretic validation for path finding:
//! - Graph hierarchy is built from `GraphSubtypeEntry` registrations
//! - Reduction applicability uses subtype relationships: A <= C and D <= B
//! - Dijkstra's algorithm with custom cost functions for optimal paths

use crate::graph_types::{GraphSubtypeEntry, WeightSubtypeEntry};
use crate::rules::cost::PathCostFn;
use crate::rules::registry::{ConcreteVariantEntry, ReductionEntry, ReductionOverhead};
use crate::types::ProblemSize;
use ordered_float::OrderedFloat;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};


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
    pub variant: std::collections::BTreeMap<String, String>,
    /// Category of the problem (e.g., "graph", "set", "optimization", "satisfiability", "specialized").
    pub category: String,
    /// Relative rustdoc path (e.g., "models/graph/maximum_independent_set").
    pub doc_path: String,
}

/// Internal reference to a problem variant, used during edge construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VariantRef {
    name: String,
    variant: std::collections::BTreeMap<String, String>,
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

/// Edge data for a reduction.
#[derive(Clone, Debug)]
pub struct ReductionEdge {
    /// Source variant attributes as key-value pairs.
    pub source_variant: Vec<(&'static str, &'static str)>,
    /// Target variant attributes as key-value pairs.
    pub target_variant: Vec<(&'static str, &'static str)>,
    /// Overhead information for cost calculations.
    pub overhead: ReductionOverhead,
}

impl ReductionEdge {
    /// Get the graph type from the source variant, or "SimpleGraph" as default.
    /// Empty strings are treated as missing and default to "SimpleGraph".
    pub fn source_graph(&self) -> &str {
        self.source_variant
            .iter()
            .find(|(k, _)| *k == "graph")
            .map(|(_, v)| *v)
            .filter(|v| !v.is_empty())
            .unwrap_or("SimpleGraph")
    }

    /// Get the graph type from the target variant, or "SimpleGraph" as default.
    /// Empty strings are treated as missing and default to "SimpleGraph".
    pub fn target_graph(&self) -> &str {
        self.target_variant
            .iter()
            .find(|(k, _)| *k == "graph")
            .map(|(_, v)| *v)
            .filter(|v| !v.is_empty())
            .unwrap_or("SimpleGraph")
    }
}

/// Runtime graph of all registered reductions.
///
/// Uses type-erased names for the graph topology, so `MaxCut<SimpleGraph, i32>` and `MaxCut<SimpleGraph, f64>`
/// map to the same node "MaxCut". This allows finding reduction paths regardless
/// of weight type parameters.
///
/// The graph supports:
/// - Auto-discovery of reductions from `inventory::iter::<ReductionEntry>`
/// - Graph hierarchy from `inventory::iter::<GraphSubtypeEntry>`
/// - Set-theoretic validation for path finding
/// - Dijkstra with custom cost functions
pub struct ReductionGraph {
    /// Graph with base type names as node data.
    graph: DiGraph<&'static str, ReductionEdge>,
    /// Map from base type name to node index.
    name_indices: HashMap<&'static str, NodeIndex>,
    /// Graph hierarchy: subtype -> set of supertypes (transitively closed).
    graph_hierarchy: HashMap<&'static str, HashSet<&'static str>>,
    /// Weight hierarchy: subtype -> set of supertypes (transitively closed).
    weight_hierarchy: HashMap<&'static str, HashSet<&'static str>>,
}

impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions from inventory.
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut name_indices = HashMap::new();

        // Build graph hierarchy from GraphSubtypeEntry registrations
        let graph_hierarchy = Self::build_graph_hierarchy();

        // Build weight hierarchy from WeightSubtypeEntry registrations
        let weight_hierarchy = Self::build_weight_hierarchy();

        // Register reductions from inventory (auto-discovery)
        for entry in inventory::iter::<ReductionEntry> {
            // Ensure source node exists
            if !name_indices.contains_key(entry.source_name) {
                let idx = graph.add_node(entry.source_name);
                name_indices.insert(entry.source_name, idx);
            }
            // Ensure target node exists
            if !name_indices.contains_key(entry.target_name) {
                let idx = graph.add_node(entry.target_name);
                name_indices.insert(entry.target_name, idx);
            }

            // Add edge with metadata
            let src = name_indices[entry.source_name];
            let dst = name_indices[entry.target_name];

            // Check if edge already exists (avoid duplicates)
            if graph.find_edge(src, dst).is_none() {
                graph.add_edge(
                    src,
                    dst,
                    ReductionEdge {
                        source_variant: entry.source_variant(),
                        target_variant: entry.target_variant(),
                        overhead: entry.overhead(),
                    },
                );
            }
        }

        Self {
            graph,
            name_indices,
            graph_hierarchy,
            weight_hierarchy,
        }
    }

    /// Build graph hierarchy from GraphSubtypeEntry registrations.
    /// Computes the transitive closure of the subtype relationship.
    fn build_graph_hierarchy() -> HashMap<&'static str, HashSet<&'static str>> {
        let mut supertypes: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        // Collect direct subtype relationships
        for entry in inventory::iter::<GraphSubtypeEntry> {
            supertypes
                .entry(entry.subtype)
                .or_default()
                .insert(entry.supertype);
        }

        // Compute transitive closure
        loop {
            let mut changed = false;
            let types: Vec<_> = supertypes.keys().copied().collect();

            for sub in &types {
                let current: Vec<_> = supertypes
                    .get(sub)
                    .map(|s| s.iter().copied().collect())
                    .unwrap_or_default();

                for sup in current {
                    if let Some(sup_supers) = supertypes.get(sup).cloned() {
                        for ss in sup_supers {
                            if supertypes.entry(sub).or_default().insert(ss) {
                                changed = true;
                            }
                        }
                    }
                }
            }

            if !changed {
                break;
            }
        }

        supertypes
    }

    /// Build weight hierarchy from WeightSubtypeEntry registrations.
    /// Computes the transitive closure of the subtype relationship.
    fn build_weight_hierarchy() -> HashMap<&'static str, HashSet<&'static str>> {
        let mut supertypes: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();

        // Collect direct subtype relationships
        for entry in inventory::iter::<WeightSubtypeEntry> {
            supertypes
                .entry(entry.subtype)
                .or_default()
                .insert(entry.supertype);
        }

        // Compute transitive closure
        loop {
            let mut changed = false;
            let types: Vec<_> = supertypes.keys().copied().collect();

            for sub in &types {
                let current: Vec<_> = supertypes
                    .get(sub)
                    .map(|s| s.iter().copied().collect())
                    .unwrap_or_default();

                for sup in current {
                    if let Some(sup_supers) = supertypes.get(sup).cloned() {
                        for ss in sup_supers {
                            if supertypes.entry(sub).or_default().insert(ss) {
                                changed = true;
                            }
                        }
                    }
                }
            }

            if !changed {
                break;
            }
        }

        supertypes
    }

    /// Check if `sub` is a subtype of `sup` (or equal).
    pub fn is_graph_subtype(&self, sub: &str, sup: &str) -> bool {
        sub == sup
            || self
                .graph_hierarchy
                .get(sub)
                .map(|s| s.contains(sup))
                .unwrap_or(false)
    }

    /// Get the weight hierarchy (for inspection/testing).
    pub fn weight_hierarchy(&self) -> &HashMap<&'static str, HashSet<&'static str>> {
        &self.weight_hierarchy
    }

    /// Check if `sub` is a weight subtype of `sup` (or equal).
    pub fn is_weight_subtype(&self, sub: &str, sup: &str) -> bool {
        sub == sup
            || self
                .weight_hierarchy
                .get(sub)
                .map(|s| s.contains(sup))
                .unwrap_or(false)
    }

    /// Check if a reduction rule can be used.
    ///
    /// For a reduction from problem A (on graph type G_A) to problem B (on graph type G_B),
    /// using a rule that reduces C (on G_C) to D (on G_D):
    ///
    /// The rule is applicable if:
    /// - G_A is a subtype of G_C (our source graph is more specific than rule requires)
    /// - G_D is a subtype of G_B (rule produces a graph that fits our target requirement)
    pub fn rule_applicable(
        &self,
        want_source_graph: &str,
        want_target_graph: &str,
        rule_source_graph: &str,
        rule_target_graph: &str,
    ) -> bool {
        // A <= C: our source must be subtype of rule's source (or equal)
        // D <= B: rule's target must be subtype of our target (or equal)
        self.is_graph_subtype(want_source_graph, rule_source_graph)
            && self.is_graph_subtype(rule_target_graph, want_target_graph)
    }

    /// Find the cheapest path using a custom cost function.
    ///
    /// Uses Dijkstra's algorithm with set-theoretic validation.
    ///
    /// # Arguments
    /// - `source`: (problem_name, graph_type) for source
    /// - `target`: (problem_name, graph_type) for target
    /// - `input_size`: Initial problem size for cost calculations
    /// - `cost_fn`: Custom cost function for path optimization
    ///
    /// # Returns
    /// The cheapest path if one exists that satisfies the graph type constraints.
    pub fn find_cheapest_path<C: PathCostFn>(
        &self,
        source: (&str, &str),
        target: (&str, &str),
        input_size: &ProblemSize,
        cost_fn: &C,
    ) -> Option<ReductionPath> {
        let src_idx = *self.name_indices.get(source.0)?;
        let dst_idx = *self.name_indices.get(target.0)?;

        let mut costs: HashMap<NodeIndex, f64> = HashMap::new();
        let mut sizes: HashMap<NodeIndex, ProblemSize> = HashMap::new();
        let mut prev: HashMap<NodeIndex, (NodeIndex, petgraph::graph::EdgeIndex)> = HashMap::new();
        let mut heap = BinaryHeap::new();

        costs.insert(src_idx, 0.0);
        sizes.insert(src_idx, input_size.clone());
        heap.push(Reverse((OrderedFloat(0.0), src_idx)));

        while let Some(Reverse((cost, node))) = heap.pop() {
            if node == dst_idx {
                return Some(self.reconstruct_path(&prev, src_idx, dst_idx));
            }

            if cost.0 > *costs.get(&node).unwrap_or(&f64::INFINITY) {
                continue;
            }

            let current_size = match sizes.get(&node) {
                Some(s) => s.clone(),
                None => continue,
            };

            for edge_ref in self.graph.edges(node) {
                let edge = edge_ref.weight();
                let next = edge_ref.target();

                // Check set-theoretic applicability
                if !self.rule_applicable(
                    source.1,
                    target.1,
                    edge.source_graph(),
                    edge.target_graph(),
                ) {
                    continue;
                }

                let edge_cost = cost_fn.edge_cost(&edge.overhead, &current_size);
                let new_cost = cost.0 + edge_cost;
                let new_size = edge.overhead.evaluate_output_size(&current_size);

                if new_cost < *costs.get(&next).unwrap_or(&f64::INFINITY) {
                    costs.insert(next, new_cost);
                    sizes.insert(next, new_size);
                    prev.insert(next, (node, edge_ref.id()));
                    heap.push(Reverse((OrderedFloat(new_cost), next)));
                }
            }
        }

        None
    }

    /// Reconstruct a path from the predecessor map.
    fn reconstruct_path(
        &self,
        prev: &HashMap<NodeIndex, (NodeIndex, petgraph::graph::EdgeIndex)>,
        src: NodeIndex,
        dst: NodeIndex,
    ) -> ReductionPath {
        let mut path = vec![self.graph[dst]];
        let mut current = dst;

        while current != src {
            if let Some(&(prev_node, _)) = prev.get(&current) {
                path.push(self.graph[prev_node]);
                current = prev_node;
            } else {
                break;
            }
        }

        path.reverse();
        ReductionPath { type_names: path }
    }

    /// Find all paths from source to target type.
    ///
    /// Uses `Problem::NAME` for lookup, so `find_paths::<MaxCut<SimpleGraph, i32>, SpinGlass<SimpleGraph, f64>>()`
    /// will find paths even though the weight types differ.
    pub fn find_paths<S: crate::traits::Problem, T: crate::traits::Problem>(
        &self,
    ) -> Vec<ReductionPath> {
        self.find_paths_by_name(S::NAME, T::NAME)
    }

    /// Find all paths between problem types by name.
    pub fn find_paths_by_name(&self, src: &str, dst: &str) -> Vec<ReductionPath> {
        let src_idx = match self.name_indices.get(src) {
            Some(&idx) => idx,
            None => return vec![],
        };
        let dst_idx = match self.name_indices.get(dst) {
            Some(&idx) => idx,
            None => return vec![],
        };

        let paths: Vec<Vec<NodeIndex>> = all_simple_paths::<
            Vec<NodeIndex>,
            _,
            std::hash::RandomState,
        >(&self.graph, src_idx, dst_idx, 0, None)
        .collect();

        paths
            .into_iter()
            .map(|path| {
                let type_names: Vec<&'static str> =
                    path.iter().map(|&idx| self.graph[idx]).collect();
                ReductionPath { type_names }
            })
            .collect()
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
        if let (Some(&src_idx), Some(&dst_idx)) =
            (self.name_indices.get(src), self.name_indices.get(dst))
        {
            self.graph.find_edge(src_idx, dst_idx).is_some()
        } else {
            false
        }
    }

    /// Get all registered problem type names (base names).
    pub fn problem_types(&self) -> Vec<&'static str> {
        self.name_indices.keys().copied().collect()
    }

    /// Get the number of registered problem types.
    pub fn num_types(&self) -> usize {
        self.name_indices.len()
    }

    /// Get the number of registered reductions.
    pub fn num_reductions(&self) -> usize {
        self.graph.edge_count()
    }

    /// Get the graph hierarchy (for inspection/testing).
    pub fn graph_hierarchy(&self) -> &HashMap<&'static str, HashSet<&'static str>> {
        &self.graph_hierarchy
    }
}

impl Default for ReductionGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if const value `a` is a subtype of `b`.
/// A specific value (e.g., "3") is a subtype of "N" (generic/any).
fn is_const_subtype(a: &str, b: &str) -> bool {
    a != b && b == "N" && a != "N"
}

impl ReductionGraph {
    /// Check if variant A is strictly more restrictive than variant B (same problem name).
    /// Returns true if every field of A is a subtype of (or equal to) the corresponding field in B,
    /// and at least one field is strictly more restrictive.
    fn is_variant_reducible(
        &self,
        a: &std::collections::BTreeMap<String, String>,
        b: &std::collections::BTreeMap<String, String>,
    ) -> bool {
        if a == b {
            return false; // No self-reduction
        }

        let mut all_compatible = true;

        // Check all fields present in either variant
        let all_keys: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).collect();

        for key in all_keys {
            let a_val = a.get(key.as_str()).map(|s| s.as_str()).unwrap_or("");
            let b_val = b.get(key.as_str()).map(|s| s.as_str()).unwrap_or("");

            if a_val == b_val {
                continue; // Equal on this field
            }

            // Check subtype relationship based on field type
            let is_sub = match key.as_str() {
                "graph" => self.is_graph_subtype(a_val, b_val),
                "weight" => self.is_weight_subtype(a_val, b_val),
                "k" => is_const_subtype(a_val, b_val),
                _ => false, // Unknown fields must be equal
            };

            if !is_sub {
                all_compatible = false;
                break;
            }
        }

        // all_compatible is true and a != b means at least one field is strictly more restrictive
        all_compatible
    }

    /// Helper to convert a variant slice to a BTreeMap.
    /// Normalizes empty "graph" values to "SimpleGraph" for consistency.
    fn variant_to_map(variant: &[(&str, &str)]) -> std::collections::BTreeMap<String, String> {
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

    /// Helper to create a VariantRef from name and variant slice.
    fn make_variant_ref(name: &str, variant: &[(&str, &str)]) -> VariantRef {
        VariantRef {
            name: name.to_string(),
            variant: Self::variant_to_map(variant),
        }
    }

    /// Export the reduction graph as a JSON-serializable structure.
    ///
    /// This method generates nodes for each variant based on the registered reductions.
    pub fn to_json(&self) -> ReductionGraphJson {
        use crate::registry::ProblemSchemaEntry;
        use crate::rules::registry::ReductionEntry;

        // Build name → module_path lookup from ProblemSchemaEntry inventory
        let schema_modules: HashMap<&str, &str> = inventory::iter::<ProblemSchemaEntry>
            .into_iter()
            .map(|entry| (entry.name, entry.module_path))
            .collect();

        // Collect all unique nodes (name + variant combination)
        let mut node_set: HashSet<(String, std::collections::BTreeMap<String, String>)> =
            HashSet::new();

        // First, add base nodes from the graph
        for &name in self.name_indices.keys() {
            node_set.insert((name.to_string(), std::collections::BTreeMap::new()));
        }

        // Then, collect variants from reduction entries
        for entry in inventory::iter::<ReductionEntry> {
            let source_variant = entry.source_variant();
            let target_variant = entry.target_variant();
            node_set.insert((
                entry.source_name.to_string(),
                Self::variant_to_map(&source_variant),
            ));
            node_set.insert((
                entry.target_name.to_string(),
                Self::variant_to_map(&target_variant),
            ));
        }

        // Also collect nodes from ConcreteVariantEntry registrations
        for entry in inventory::iter::<ConcreteVariantEntry> {
            let variant = (entry.variant_fn)();
            node_set.insert((entry.name.to_string(), Self::variant_to_map(&variant)));
        }

        // Build nodes with categories and doc paths derived from ProblemSchemaEntry.module_path
        let mut nodes: Vec<NodeJson> = node_set
            .iter()
            .map(|(name, variant)| {
                let (category, doc_path) = if let Some(&mod_path) = schema_modules.get(name.as_str()) {
                    (
                        Self::category_from_module_path(mod_path),
                        Self::doc_path_from_module_path(mod_path, name),
                    )
                } else {
                    ("other".to_string(), String::new())
                };
                NodeJson {
                    name: name.clone(),
                    variant: variant.clone(),
                    category,
                    doc_path,
                }
            })
            .collect();
        nodes.sort_by(|a, b| (&a.name, &a.variant).cmp(&(&b.name, &b.variant)));

        // Build node index lookup: (name, variant) -> index in sorted nodes vec
        let node_index: HashMap<(&str, &std::collections::BTreeMap<String, String>), usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, n)| ((n.name.as_str(), &n.variant), i))
            .collect();

        // Collect edges as (VariantRef, VariantRef) pairs first, then resolve to indices
        let mut edge_set: HashSet<(VariantRef, VariantRef)> = HashSet::new();
        let mut edge_data: Vec<(VariantRef, VariantRef, Vec<OverheadFieldJson>, String)> =
            Vec::new();

        for entry in inventory::iter::<ReductionEntry> {
            let source_variant = entry.source_variant();
            let target_variant = entry.target_variant();
            let src_ref = Self::make_variant_ref(entry.source_name, &source_variant);
            let dst_ref = Self::make_variant_ref(entry.target_name, &target_variant);
            let key = (src_ref.clone(), dst_ref.clone());
            if edge_set.insert(key) {
                let overhead = entry.overhead();
                let doc_path = Self::module_path_to_doc_path(entry.module_path);
                let overhead_fields = overhead
                    .output_size
                    .iter()
                    .map(|(field, poly)| OverheadFieldJson {
                        field: field.to_string(),
                        formula: poly.to_string(),
                    })
                    .collect();
                edge_data.push((src_ref, dst_ref, overhead_fields, doc_path));
            }
        }

        // Auto-generate natural edges between same-name variant nodes.
        // A natural edge exists from A to B when all variant fields of A are
        // at least as restrictive as B's (and at least one is strictly more restrictive).
        // The overhead is identity: p(x) = x for each field.
        {
            // Group non-empty-variant nodes by problem name
            let mut nodes_by_name: HashMap<&str, Vec<&std::collections::BTreeMap<String, String>>> =
                HashMap::new();
            for (name, variant) in &node_set {
                if !variant.is_empty() {
                    nodes_by_name
                        .entry(name.as_str())
                        .or_default()
                        .push(variant);
                }
            }

            // Collect overhead field names per problem from existing edges.
            // Use edges where the problem is the TARGET, since the overhead fields
            // describe the target problem's size dimensions.
            let mut fields_by_problem: HashMap<String, Vec<String>> = HashMap::new();
            for (_, dst, overhead, _) in &edge_data {
                if !overhead.is_empty() {
                    fields_by_problem
                        .entry(dst.name.clone())
                        .or_insert_with(|| overhead.iter().map(|o| o.field.clone()).collect());
                }
            }

            // For each pair of same-name nodes, check transitive reducibility
            for (name, variants) in &nodes_by_name {
                for a in variants {
                    for b in variants {
                        if self.is_variant_reducible(a, b) {
                            let src_ref = VariantRef {
                                name: name.to_string(),
                                variant: (*a).clone(),
                            };
                            let dst_ref = VariantRef {
                                name: name.to_string(),
                                variant: (*b).clone(),
                            };
                            let key = (src_ref.clone(), dst_ref.clone());
                            if edge_set.insert(key) {
                                // Identity overhead: each field maps to itself, p(x) = x
                                let overhead = fields_by_problem
                                    .get(*name)
                                    .map(|fields| {
                                        fields
                                            .iter()
                                            .map(|f| OverheadFieldJson {
                                                field: f.clone(),
                                                formula: f.clone(),
                                            })
                                            .collect()
                                    })
                                    .unwrap_or_default();

                                edge_data.push((src_ref, dst_ref, overhead, String::new()));
                            }
                        }
                    }
                }
            }
        }

        // Sort edge data by source/target names for deterministic output
        edge_data.sort_by(|a, b| {
            (&a.0.name, &a.0.variant, &a.1.name, &a.1.variant)
                .cmp(&(&b.0.name, &b.0.variant, &b.1.name, &b.1.variant))
        });

        // Resolve VariantRefs to node indices
        let edges: Vec<EdgeJson> = edge_data
            .into_iter()
            .map(|(src, dst, overhead, doc_path)| {
                let src_idx = node_index[&(src.name.as_str(), &src.variant)];
                let dst_idx = node_index[&(dst.name.as_str(), &dst.variant)];
                EdgeJson {
                    source: src_idx,
                    target: dst_idx,
                    overhead,
                    doc_path,
                }
            })
            .collect();

        ReductionGraphJson { nodes, edges }
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
    /// E.g., `"problemreductions::rules::spinglass_qubo"` → `"rules/spinglass_qubo/index.html"`.
    fn module_path_to_doc_path(module_path: &str) -> String {
        let stripped = module_path
            .strip_prefix("problemreductions::")
            .unwrap_or(module_path);
        format!("{}/index.html", stripped.replace("::", "/"))
    }

    /// Extract the category from a module path.
    ///
    /// E.g., `"problemreductions::models::graph::maximum_independent_set"` → `"graph"`.
    fn category_from_module_path(module_path: &str) -> String {
        // Expected format: "problemreductions::models::<category>::<module_name>"
        let parts: Vec<&str> = module_path.split("::").collect();
        // parts = ["problemreductions", "models", "graph", "maximum_independent_set"]
        if parts.len() >= 3 {
            parts[2].to_string()
        } else {
            "other".to_string()
        }
    }

    /// Build the rustdoc path from a module path and problem name.
    ///
    /// E.g., `"problemreductions::models::graph::maximum_independent_set"`, `"MaximumIndependentSet"`
    /// → `"models/graph/struct.MaximumIndependentSet.html"`.
    fn doc_path_from_module_path(module_path: &str, name: &str) -> String {
        let stripped = module_path
            .strip_prefix("problemreductions::")
            .unwrap_or(module_path);
        // stripped = "models::graph::maximum_independent_set"
        // We need "models/graph/struct.MaximumIndependentSet.html"
        if let Some(parent) = stripped.rsplit_once("::").map(|(p, _)| p) {
            format!("{}/struct.{}.html", parent.replace("::", "/"), name)
        } else {
            format!("struct.{}.html", name)
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/graph.rs"]
mod tests;
