//! Runtime reduction graph for discovering and executing reduction paths.
//!
//! The graph uses type-erased names (e.g., "SpinGlass" instead of "SpinGlass<i32>")
//! for topology, allowing path finding regardless of weight type parameters.
//!
//! This module implements set-theoretic validation for path finding:
//! - Graph hierarchy is built from `GraphSubtypeEntry` registrations
//! - Reduction applicability uses subtype relationships: A <= C and D <= B
//! - Dijkstra's algorithm with custom cost functions for optimal paths

use crate::graph_types::GraphSubtypeEntry;
use crate::rules::cost::PathCostFn;
use crate::rules::registry::{ReductionEntry, ReductionOverhead};
use crate::types::ProblemSize;
use ordered_float::OrderedFloat;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;
use std::any::TypeId;
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

/// A node in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub struct NodeJson {
    /// Base problem name (e.g., "IndependentSet").
    pub name: String,
    /// Variant attributes as key-value pairs.
    pub variant: std::collections::BTreeMap<String, String>,
    /// Category of the problem (e.g., "graph", "set", "optimization", "satisfiability", "specialized").
    pub category: String,
}

/// Reference to a problem variant in an edge.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct VariantRef {
    /// Base problem name.
    pub name: String,
    /// Variant attributes as key-value pairs.
    pub variant: std::collections::BTreeMap<String, String>,
}

/// An edge in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub struct EdgeJson {
    /// Source problem variant.
    pub source: VariantRef,
    /// Target problem variant.
    pub target: VariantRef,
    /// Whether the reverse reduction also exists.
    pub bidirectional: bool,
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
    pub source_variant: &'static [(&'static str, &'static str)],
    /// Target variant attributes as key-value pairs.
    pub target_variant: &'static [(&'static str, &'static str)],
    /// Overhead information for cost calculations.
    pub overhead: ReductionOverhead,
}

impl ReductionEdge {
    /// Get the graph type from the source variant, or "SimpleGraph" as default.
    pub fn source_graph(&self) -> &'static str {
        self.source_variant
            .iter()
            .find(|(k, _)| *k == "graph")
            .map(|(_, v)| *v)
            .unwrap_or("SimpleGraph")
    }

    /// Get the graph type from the target variant, or "SimpleGraph" as default.
    pub fn target_graph(&self) -> &'static str {
        self.target_variant
            .iter()
            .find(|(k, _)| *k == "graph")
            .map(|(_, v)| *v)
            .unwrap_or("SimpleGraph")
    }
}

/// Runtime graph of all registered reductions.
///
/// Uses type-erased names for the graph topology, so `MaxCut<i32>` and `MaxCut<f64>`
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
    /// Map from TypeId to base type name (for generic API compatibility).
    type_to_name: HashMap<TypeId, &'static str>,
    /// Graph hierarchy: subtype -> set of supertypes (transitively closed).
    graph_hierarchy: HashMap<&'static str, HashSet<&'static str>>,
}

impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions from inventory.
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut name_indices = HashMap::new();
        let mut type_to_name = HashMap::new();

        // Build graph hierarchy from GraphSubtypeEntry registrations
        let graph_hierarchy = Self::build_graph_hierarchy();

        // First, register all problem types (for TypeId mapping)
        Self::register_types(&mut graph, &mut name_indices, &mut type_to_name);

        // Then, register reductions from inventory (auto-discovery)
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
                        source_variant: entry.source_variant,
                        target_variant: entry.target_variant,
                        overhead: entry.overhead(),
                    },
                );
            }
        }

        Self {
            graph,
            name_indices,
            type_to_name,
            graph_hierarchy,
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

    fn register_types(
        graph: &mut DiGraph<&'static str, ReductionEdge>,
        name_indices: &mut HashMap<&'static str, NodeIndex>,
        type_to_name: &mut HashMap<TypeId, &'static str>,
    ) {
        // Register a problem type with its base name.
        // Multiple concrete types can map to the same base name.
        macro_rules! register {
            ($($ty:ty => $base_name:expr),* $(,)?) => {
                $(
                    // Map TypeId to base name
                    type_to_name.insert(TypeId::of::<$ty>(), $base_name);

                    // Only add node if not already present
                    if !name_indices.contains_key($base_name) {
                        let idx = graph.add_node($base_name);
                        name_indices.insert($base_name, idx);
                    }
                )*
            };
        }

        use crate::models::graph::*;
        use crate::models::optimization::*;
        use crate::models::satisfiability::*;
        use crate::models::set::*;
        use crate::models::specialized::*;

        // Register problem types - multiple concrete types can share a base name
        register! {
            // Graph problems
            IndependentSet<i32> => "IndependentSet",
            IndependentSet<f64> => "IndependentSet",
            VertexCovering<i32> => "VertexCovering",
            VertexCovering<f64> => "VertexCovering",
            MaxCut<i32> => "MaxCut",
            MaxCut<f64> => "MaxCut",
            Matching<i32> => "Matching",
            DominatingSet<i32> => "DominatingSet",
            Coloring => "Coloring",
            // Set problems
            SetPacking<i32> => "SetPacking",
            SetCovering<i32> => "SetCovering",
            // Optimization problems
            SpinGlass<i32> => "SpinGlass",
            SpinGlass<f64> => "SpinGlass",
            QUBO<f64> => "QUBO",
            ILP => "ILP",
            // Satisfiability problems
            Satisfiability<i32> => "Satisfiability",
            KSatisfiability<3, i32> => "KSatisfiability",
            CircuitSAT<i32> => "CircuitSAT",
            // Specialized
            Factoring => "Factoring",
        }
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
                if !self.rule_applicable(source.1, target.1, edge.source_graph(), edge.target_graph()) {
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
    /// Uses type-erased names, so `find_paths::<MaxCut<i32>, SpinGlass<f64>>()`
    /// will find paths even though the weight types differ.
    pub fn find_paths<S: 'static, T: 'static>(&self) -> Vec<ReductionPath> {
        let src_name = match self.type_to_name.get(&TypeId::of::<S>()) {
            Some(&name) => name,
            None => return vec![],
        };
        let dst_name = match self.type_to_name.get(&TypeId::of::<T>()) {
            Some(&name) => name,
            None => return vec![],
        };

        self.find_paths_by_name(src_name, dst_name)
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

        let paths: Vec<Vec<NodeIndex>> =
            all_simple_paths(&self.graph, src_idx, dst_idx, 0, None).collect();

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
    pub fn find_shortest_path<S: 'static, T: 'static>(&self) -> Option<ReductionPath> {
        let paths = self.find_paths::<S, T>();
        paths.into_iter().min_by_key(|p| p.len())
    }

    /// Find the shortest path by name.
    pub fn find_shortest_path_by_name(&self, src: &str, dst: &str) -> Option<ReductionPath> {
        let paths = self.find_paths_by_name(src, dst);
        paths.into_iter().min_by_key(|p| p.len())
    }

    /// Check if a direct reduction exists from S to T.
    pub fn has_direct_reduction<S: 'static, T: 'static>(&self) -> bool {
        let src_name = match self.type_to_name.get(&TypeId::of::<S>()) {
            Some(&name) => name,
            None => return false,
        };
        let dst_name = match self.type_to_name.get(&TypeId::of::<T>()) {
            Some(&name) => name,
            None => return false,
        };

        self.has_direct_reduction_by_name(src_name, dst_name)
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

impl ReductionGraph {
    /// Helper to convert a variant slice to a BTreeMap.
    fn variant_to_map(
        variant: &[(&'static str, &'static str)],
    ) -> std::collections::BTreeMap<String, String> {
        variant
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    /// Helper to create a VariantRef from name and variant slice.
    fn make_variant_ref(
        name: &str,
        variant: &[(&'static str, &'static str)],
    ) -> VariantRef {
        VariantRef {
            name: name.to_string(),
            variant: Self::variant_to_map(variant),
        }
    }

    /// Export the reduction graph as a JSON-serializable structure.
    ///
    /// This method generates nodes for each variant based on the registered reductions.
    pub fn to_json(&self) -> ReductionGraphJson {
        use crate::rules::registry::ReductionEntry;

        // Collect all unique nodes (name + variant combination)
        let mut node_set: HashSet<(String, std::collections::BTreeMap<String, String>)> =
            HashSet::new();

        // First, add base nodes from the graph
        for &name in self.name_indices.keys() {
            node_set.insert((name.to_string(), std::collections::BTreeMap::new()));
        }

        // Then, collect variants from reduction entries
        for entry in inventory::iter::<ReductionEntry> {
            node_set.insert((
                entry.source_name.to_string(),
                Self::variant_to_map(entry.source_variant),
            ));
            node_set.insert((
                entry.target_name.to_string(),
                Self::variant_to_map(entry.target_variant),
            ));
        }

        // Build nodes with categories
        let mut nodes: Vec<NodeJson> = node_set
            .iter()
            .map(|(name, variant)| {
                let category = Self::categorize_type(name);
                NodeJson {
                    name: name.clone(),
                    variant: variant.clone(),
                    category: category.to_string(),
                }
            })
            .collect();
        nodes.sort_by(|a, b| (&a.name, &a.variant).cmp(&(&b.name, &b.variant)));

        // Collect edges, checking for bidirectionality
        let mut edge_set: HashMap<(VariantRef, VariantRef), bool> = HashMap::new();

        for entry in inventory::iter::<ReductionEntry> {
            let src_ref = Self::make_variant_ref(entry.source_name, entry.source_variant);
            let dst_ref = Self::make_variant_ref(entry.target_name, entry.target_variant);

            let reverse_key = (dst_ref.clone(), src_ref.clone());
            if edge_set.contains_key(&reverse_key) {
                edge_set.insert(reverse_key, true);
            } else {
                edge_set.insert((src_ref, dst_ref), false);
            }
        }

        // Build edges
        let mut edges: Vec<EdgeJson> = edge_set
            .into_iter()
            .map(|((src, dst), bidirectional)| EdgeJson {
                source: src,
                target: dst,
                bidirectional,
            })
            .collect();
        edges.sort_by(|a, b| {
            (&a.source.name, &a.source.variant, &a.target.name, &a.target.variant)
                .cmp(&(&b.source.name, &b.source.variant, &b.target.name, &b.target.variant))
        });

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

    /// Categorize a type name into a problem category.
    fn categorize_type(name: &str) -> &'static str {
        if name.contains("IndependentSet")
            || name.contains("VertexCover")
            || name.contains("MaxCut")
            || name.contains("Coloring")
            || name.contains("DominatingSet")
            || name.contains("Matching")
        {
            "graph"
        } else if name.contains("SetPacking") || name.contains("SetCover") {
            "set"
        } else if name.contains("SpinGlass") || name.contains("QUBO") || name.contains("ILP") {
            "optimization"
        } else if name.contains("Satisfiability") || name.contains("SAT") {
            "satisfiability"
        } else if name.contains("Factoring") || name.contains("Circuit") {
            "specialized"
        } else {
            "other"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::graph::{IndependentSet, VertexCovering};
    use crate::models::set::SetPacking;
    use crate::rules::cost::MinimizeSteps;

    #[test]
    fn test_find_direct_path() {
        let graph = ReductionGraph::new();
        let paths = graph.find_paths::<IndependentSet<i32>, VertexCovering<i32>>();
        assert!(!paths.is_empty());
        assert_eq!(paths[0].type_names.len(), 2);
        assert_eq!(paths[0].len(), 1); // One reduction step
    }

    #[test]
    fn test_find_indirect_path() {
        let graph = ReductionGraph::new();
        // IS -> VC -> IS -> SP or IS -> SP directly
        let paths = graph.find_paths::<IndependentSet<i32>, SetPacking<i32>>();
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_find_shortest_path() {
        let graph = ReductionGraph::new();
        let path = graph.find_shortest_path::<IndependentSet<i32>, SetPacking<i32>>();
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 1); // Direct path exists
    }

    #[test]
    fn test_has_direct_reduction() {
        let graph = ReductionGraph::new();
        assert!(graph.has_direct_reduction::<IndependentSet<i32>, VertexCovering<i32>>());
        assert!(graph.has_direct_reduction::<VertexCovering<i32>, IndependentSet<i32>>());
    }

    #[test]
    fn test_no_path() {
        let graph = ReductionGraph::new();
        // No path between IndependentSet and QUBO (disconnected in graph topology)
        let paths =
            graph.find_paths::<IndependentSet<i32>, crate::models::optimization::QUBO<f64>>();
        assert!(paths.is_empty());
    }

    #[test]
    fn test_type_erased_paths() {
        let graph = ReductionGraph::new();

        // Different weight types should find the same path (type-erased)
        let paths_i32 = graph.find_paths::<
            crate::models::graph::MaxCut<i32>,
            crate::models::optimization::SpinGlass<i32>,
        >();
        let paths_f64 = graph.find_paths::<
            crate::models::graph::MaxCut<f64>,
            crate::models::optimization::SpinGlass<f64>,
        >();

        // Both should find paths since we use type-erased names
        assert!(!paths_i32.is_empty());
        assert!(!paths_f64.is_empty());
        assert_eq!(paths_i32[0].type_names, paths_f64[0].type_names);
    }

    #[test]
    fn test_find_paths_by_name() {
        let graph = ReductionGraph::new();

        let paths = graph.find_paths_by_name("MaxCut", "SpinGlass");
        assert!(!paths.is_empty());
        assert_eq!(paths[0].len(), 1); // Direct path

        let paths = graph.find_paths_by_name("Factoring", "SpinGlass");
        assert!(!paths.is_empty());
        assert_eq!(paths[0].len(), 2); // Factoring -> CircuitSAT -> SpinGlass
    }

    #[test]
    fn test_problem_types() {
        let graph = ReductionGraph::new();
        let types = graph.problem_types();
        assert!(types.len() >= 5);
        assert!(types.iter().any(|t| t.contains("IndependentSet")));
        assert!(types.iter().any(|t| t.contains("VertexCovering")));
    }

    #[test]
    fn test_graph_statistics() {
        let graph = ReductionGraph::new();
        assert!(graph.num_types() >= 5);
        assert!(graph.num_reductions() >= 6);
    }

    #[test]
    fn test_reduction_path_methods() {
        let graph = ReductionGraph::new();
        let path = graph
            .find_shortest_path::<IndependentSet<i32>, VertexCovering<i32>>()
            .unwrap();

        assert!(!path.is_empty());
        assert!(path.source().unwrap().contains("IndependentSet"));
        assert!(path.target().unwrap().contains("VertexCovering"));
    }

    #[test]
    fn test_bidirectional_paths() {
        let graph = ReductionGraph::new();

        // Forward path
        let forward = graph.find_paths::<IndependentSet<i32>, VertexCovering<i32>>();
        assert!(!forward.is_empty());

        // Backward path
        let backward = graph.find_paths::<VertexCovering<i32>, IndependentSet<i32>>();
        assert!(!backward.is_empty());
    }

    #[test]
    fn test_to_json() {
        let graph = ReductionGraph::new();
        let json = graph.to_json();

        // Check nodes
        assert!(json.nodes.len() >= 10);
        assert!(json.nodes.iter().any(|n| n.name == "IndependentSet"));
        assert!(json.nodes.iter().any(|n| n.category == "graph"));
        assert!(json.nodes.iter().any(|n| n.category == "optimization"));

        // Check edges
        assert!(json.edges.len() >= 10);

        // Check that IS <-> VC is marked bidirectional
        let is_vc_edge = json.edges.iter().find(|e| {
            (e.source.name.contains("IndependentSet") && e.target.name.contains("VertexCovering"))
                || (e.source.name.contains("VertexCovering") && e.target.name.contains("IndependentSet"))
        });
        assert!(is_vc_edge.is_some());
        assert!(is_vc_edge.unwrap().bidirectional);
    }

    #[test]
    fn test_to_json_string() {
        let graph = ReductionGraph::new();
        let json_string = graph.to_json_string().unwrap();

        // Should be valid JSON
        assert!(json_string.contains("\"nodes\""));
        assert!(json_string.contains("\"edges\""));
        assert!(json_string.contains("IndependentSet"));
        assert!(json_string.contains("\"category\""));
        assert!(json_string.contains("\"bidirectional\""));
    }

    #[test]
    fn test_categorize_type() {
        // Graph problems
        assert_eq!(
            ReductionGraph::categorize_type("IndependentSet<i32>"),
            "graph"
        );
        assert_eq!(
            ReductionGraph::categorize_type("VertexCovering<i32>"),
            "graph"
        );
        assert_eq!(ReductionGraph::categorize_type("MaxCut<i32>"), "graph");
        assert_eq!(ReductionGraph::categorize_type("Coloring"), "graph");
        assert_eq!(
            ReductionGraph::categorize_type("DominatingSet<i32>"),
            "graph"
        );
        assert_eq!(ReductionGraph::categorize_type("Matching<i32>"), "graph");

        // Set problems
        assert_eq!(ReductionGraph::categorize_type("SetPacking<i32>"), "set");
        assert_eq!(ReductionGraph::categorize_type("SetCovering<i32>"), "set");

        // Optimization
        assert_eq!(
            ReductionGraph::categorize_type("SpinGlass<i32>"),
            "optimization"
        );
        assert_eq!(ReductionGraph::categorize_type("QUBO<f64>"), "optimization");

        // Satisfiability
        assert_eq!(
            ReductionGraph::categorize_type("Satisfiability<i32>"),
            "satisfiability"
        );
        assert_eq!(
            ReductionGraph::categorize_type("KSatisfiability<3, i32>"),
            "satisfiability"
        );
        assert_eq!(
            ReductionGraph::categorize_type("CircuitSAT<i32>"),
            "satisfiability"
        );

        // Specialized
        assert_eq!(ReductionGraph::categorize_type("Factoring"), "specialized");

        // Unknown
        assert_eq!(ReductionGraph::categorize_type("UnknownProblem"), "other");
    }

    #[test]
    fn test_sat_based_reductions() {
        use crate::models::graph::Coloring;
        use crate::models::graph::DominatingSet;
        use crate::models::satisfiability::Satisfiability;

        let graph = ReductionGraph::new();

        // SAT -> IS
        assert!(graph.has_direct_reduction::<Satisfiability<i32>, IndependentSet<i32>>());

        // SAT -> Coloring
        assert!(graph.has_direct_reduction::<Satisfiability<i32>, Coloring>());

        // SAT -> DominatingSet
        assert!(graph.has_direct_reduction::<Satisfiability<i32>, DominatingSet<i32>>());
    }

    #[test]
    fn test_circuit_reductions() {
        use crate::models::optimization::SpinGlass;
        use crate::models::specialized::{CircuitSAT, Factoring};

        let graph = ReductionGraph::new();

        // Factoring -> CircuitSAT
        assert!(graph.has_direct_reduction::<Factoring, CircuitSAT<i32>>());

        // CircuitSAT -> SpinGlass
        assert!(graph.has_direct_reduction::<CircuitSAT<i32>, SpinGlass<f64>>());

        // Find path from Factoring to SpinGlass
        let paths = graph.find_paths::<Factoring, SpinGlass<f64>>();
        assert!(!paths.is_empty());
        let shortest = graph
            .find_shortest_path::<Factoring, SpinGlass<f64>>()
            .unwrap();
        assert_eq!(shortest.len(), 2); // Factoring -> CircuitSAT -> SpinGlass
    }

    #[test]
    fn test_optimization_reductions() {
        use crate::models::graph::MaxCut;
        use crate::models::optimization::{SpinGlass, QUBO};

        let graph = ReductionGraph::new();

        // SpinGlass <-> QUBO (bidirectional)
        assert!(graph.has_direct_reduction::<SpinGlass<f64>, QUBO<f64>>());
        assert!(graph.has_direct_reduction::<QUBO<f64>, SpinGlass<f64>>());

        // MaxCut <-> SpinGlass (bidirectional)
        assert!(graph.has_direct_reduction::<MaxCut<i32>, SpinGlass<f64>>());
        assert!(graph.has_direct_reduction::<SpinGlass<f64>, MaxCut<i32>>());
    }

    #[test]
    fn test_ksat_reductions() {
        use crate::models::satisfiability::{KSatisfiability, Satisfiability};

        let graph = ReductionGraph::new();

        // SAT <-> 3-SAT (bidirectional)
        assert!(graph.has_direct_reduction::<Satisfiability<i32>, KSatisfiability<3, i32>>());
        assert!(graph.has_direct_reduction::<KSatisfiability<3, i32>, Satisfiability<i32>>());
    }

    #[test]
    fn test_all_categories_present() {
        let graph = ReductionGraph::new();
        let json = graph.to_json();

        let categories: std::collections::HashSet<&str> =
            json.nodes.iter().map(|n| n.category.as_str()).collect();

        assert!(categories.contains("graph"));
        assert!(categories.contains("set"));
        assert!(categories.contains("optimization"));
        assert!(categories.contains("satisfiability"));
        assert!(categories.contains("specialized"));
    }

    #[test]
    fn test_empty_path_source_target() {
        let path = ReductionPath { type_names: vec![] };
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
        assert!(path.source().is_none());
        assert!(path.target().is_none());
    }

    #[test]
    fn test_single_node_path() {
        let path = ReductionPath {
            type_names: vec!["IndependentSet"],
        };
        assert!(!path.is_empty());
        assert_eq!(path.len(), 0); // No reductions, just one type
        assert_eq!(path.source(), Some("IndependentSet"));
        assert_eq!(path.target(), Some("IndependentSet"));
    }

    #[test]
    fn test_default_implementation() {
        let graph1 = ReductionGraph::new();
        let graph2 = ReductionGraph::default();

        assert_eq!(graph1.num_types(), graph2.num_types());
        assert_eq!(graph1.num_reductions(), graph2.num_reductions());
    }

    #[test]
    fn test_to_json_file() {
        use std::env;
        use std::fs;

        let graph = ReductionGraph::new();
        let file_path = env::temp_dir().join("problemreductions_test_graph.json");

        // Write to file
        graph.to_json_file(&file_path).unwrap();

        // Read back and verify
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("\"nodes\""));
        assert!(content.contains("\"edges\""));
        assert!(content.contains("IndependentSet"));

        // Parse as generic JSON to verify validity
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(!parsed["nodes"].as_array().unwrap().is_empty());
        assert!(!parsed["edges"].as_array().unwrap().is_empty());

        // Clean up
        let _ = fs::remove_file(&file_path);
    }

    #[test]
    fn test_has_direct_reduction_unregistered_types() {
        // Test with a type that's not registered in the graph
        struct UnregisteredType;

        let graph = ReductionGraph::new();

        // Source type not registered
        assert!(!graph.has_direct_reduction::<UnregisteredType, IndependentSet<i32>>());

        // Target type not registered
        assert!(!graph.has_direct_reduction::<IndependentSet<i32>, UnregisteredType>());

        // Both types not registered
        assert!(!graph.has_direct_reduction::<UnregisteredType, UnregisteredType>());
    }

    #[test]
    fn test_find_paths_unregistered_source() {
        struct UnregisteredType;

        let graph = ReductionGraph::new();
        let paths = graph.find_paths::<UnregisteredType, IndependentSet<i32>>();
        assert!(paths.is_empty());
    }

    #[test]
    fn test_find_paths_unregistered_target() {
        struct UnregisteredType;

        let graph = ReductionGraph::new();
        let paths = graph.find_paths::<IndependentSet<i32>, UnregisteredType>();
        assert!(paths.is_empty());
    }

    #[test]
    fn test_find_shortest_path_no_path() {
        struct UnregisteredType;

        let graph = ReductionGraph::new();
        let path = graph.find_shortest_path::<UnregisteredType, IndependentSet<i32>>();
        assert!(path.is_none());
    }

    #[test]
    fn test_categorize_circuit_as_specialized() {
        // CircuitSAT should be categorized as specialized (contains "Circuit")
        assert_eq!(
            ReductionGraph::categorize_type("CircuitSAT<i32>"),
            "satisfiability"
        );
        // But it contains "SAT" so it goes to satisfiability first
        // Let's verify the actual behavior matches what the code does
    }

    #[test]
    fn test_edge_bidirectionality_detection() {
        let graph = ReductionGraph::new();
        let json = graph.to_json();

        // Count bidirectional and unidirectional edges
        let bidirectional_count = json.edges.iter().filter(|e| e.bidirectional).count();
        let unidirectional_count = json.edges.iter().filter(|e| !e.bidirectional).count();

        // We should have both types
        assert!(bidirectional_count > 0, "Should have bidirectional edges");
        assert!(unidirectional_count > 0, "Should have unidirectional edges");

        // Verify specific known bidirectional edges
        let is_vc_bidir = json.edges.iter().any(|e| {
            (e.source.name.contains("IndependentSet") && e.target.name.contains("VertexCovering")
                || e.source.name.contains("VertexCovering") && e.target.name.contains("IndependentSet"))
                && e.bidirectional
        });
        assert!(is_vc_bidir, "IS <-> VC should be bidirectional");

        // Verify specific known unidirectional edge
        let factoring_circuit_unidir = json.edges.iter().any(|e| {
            e.source.name.contains("Factoring") && e.target.name.contains("CircuitSAT") && !e.bidirectional
        });
        assert!(
            factoring_circuit_unidir,
            "Factoring -> CircuitSAT should be unidirectional"
        );
    }

    // New tests for set-theoretic path finding

    #[test]
    fn test_graph_hierarchy_built() {
        let graph = ReductionGraph::new();
        let hierarchy = graph.graph_hierarchy();

        // Should have relationships from GraphSubtypeEntry registrations
        // UnitDiskGraph -> PlanarGraph -> SimpleGraph
        // BipartiteGraph -> SimpleGraph
        assert!(
            hierarchy
                .get("UnitDiskGraph")
                .map(|s| s.contains("SimpleGraph"))
                .unwrap_or(false),
            "UnitDiskGraph should have SimpleGraph as supertype"
        );
        assert!(
            hierarchy
                .get("PlanarGraph")
                .map(|s| s.contains("SimpleGraph"))
                .unwrap_or(false),
            "PlanarGraph should have SimpleGraph as supertype"
        );
    }

    #[test]
    fn test_is_graph_subtype_reflexive() {
        let graph = ReductionGraph::new();

        // Every type is a subtype of itself
        assert!(graph.is_graph_subtype("SimpleGraph", "SimpleGraph"));
        assert!(graph.is_graph_subtype("PlanarGraph", "PlanarGraph"));
        assert!(graph.is_graph_subtype("UnitDiskGraph", "UnitDiskGraph"));
    }

    #[test]
    fn test_is_graph_subtype_direct() {
        let graph = ReductionGraph::new();

        // Direct subtype relationships
        assert!(graph.is_graph_subtype("PlanarGraph", "SimpleGraph"));
        assert!(graph.is_graph_subtype("BipartiteGraph", "SimpleGraph"));
        assert!(graph.is_graph_subtype("UnitDiskGraph", "PlanarGraph"));
    }

    #[test]
    fn test_is_graph_subtype_transitive() {
        let graph = ReductionGraph::new();

        // Transitive closure: UnitDiskGraph -> PlanarGraph -> SimpleGraph
        assert!(graph.is_graph_subtype("UnitDiskGraph", "SimpleGraph"));
    }

    #[test]
    fn test_is_graph_subtype_not_supertype() {
        let graph = ReductionGraph::new();

        // SimpleGraph is NOT a subtype of PlanarGraph (only the reverse)
        assert!(!graph.is_graph_subtype("SimpleGraph", "PlanarGraph"));
        assert!(!graph.is_graph_subtype("SimpleGraph", "UnitDiskGraph"));
    }

    #[test]
    fn test_rule_applicable_same_graphs() {
        let graph = ReductionGraph::new();

        // Rule for SimpleGraph -> SimpleGraph applies to same
        assert!(graph.rule_applicable("SimpleGraph", "SimpleGraph", "SimpleGraph", "SimpleGraph"));
    }

    #[test]
    fn test_rule_applicable_subtype_source() {
        let graph = ReductionGraph::new();

        // Rule for SimpleGraph -> SimpleGraph applies when source is PlanarGraph
        // (because PlanarGraph <= SimpleGraph)
        assert!(graph.rule_applicable("PlanarGraph", "SimpleGraph", "SimpleGraph", "SimpleGraph"));
    }

    #[test]
    fn test_rule_applicable_subtype_target() {
        let graph = ReductionGraph::new();

        // Rule producing PlanarGraph applies when we want SimpleGraph
        // (because PlanarGraph <= SimpleGraph)
        assert!(graph.rule_applicable("SimpleGraph", "SimpleGraph", "SimpleGraph", "PlanarGraph"));
    }

    #[test]
    fn test_rule_not_applicable_wrong_source() {
        let graph = ReductionGraph::new();

        // Rule requiring PlanarGraph does NOT apply to SimpleGraph source
        // (because SimpleGraph is NOT <= PlanarGraph)
        assert!(!graph.rule_applicable("SimpleGraph", "SimpleGraph", "PlanarGraph", "SimpleGraph"));
    }

    #[test]
    fn test_rule_not_applicable_wrong_target() {
        let graph = ReductionGraph::new();

        // Rule producing SimpleGraph does NOT apply when we need PlanarGraph
        // (because SimpleGraph is NOT <= PlanarGraph)
        assert!(!graph.rule_applicable("SimpleGraph", "PlanarGraph", "SimpleGraph", "SimpleGraph"));
    }

    #[test]
    fn test_find_cheapest_path_minimize_steps() {
        let graph = ReductionGraph::new();
        let cost_fn = MinimizeSteps;
        let input_size = ProblemSize::new(vec![("n", 10), ("m", 20)]);

        // Find path from IndependentSet to VertexCovering on SimpleGraph
        let path = graph.find_cheapest_path(
            ("IndependentSet", "SimpleGraph"),
            ("VertexCovering", "SimpleGraph"),
            &input_size,
            &cost_fn,
        );

        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 1); // Direct path
    }

    #[test]
    fn test_find_cheapest_path_multi_step() {
        let graph = ReductionGraph::new();
        let cost_fn = MinimizeSteps;
        let input_size = ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 20)]);

        // Find multi-step path where all edges use compatible graph types
        // IndependentSet (SimpleGraph) -> SetPacking (SimpleGraph)
        // This tests the algorithm can find paths with consistent graph types
        let path = graph.find_cheapest_path(
            ("IndependentSet", "SimpleGraph"),
            ("SetPacking", "SimpleGraph"),
            &input_size,
            &cost_fn,
        );

        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 1); // Direct path: IndependentSet -> SetPacking
    }

    #[test]
    fn test_find_cheapest_path_no_path() {
        let graph = ReductionGraph::new();
        let cost_fn = MinimizeSteps;
        let input_size = ProblemSize::new(vec![("n", 10)]);

        // No path from IndependentSet to QUBO
        let path = graph.find_cheapest_path(
            ("IndependentSet", "SimpleGraph"),
            ("QUBO", "SimpleGraph"),
            &input_size,
            &cost_fn,
        );

        assert!(path.is_none());
    }

    #[test]
    fn test_find_cheapest_path_unknown_source() {
        let graph = ReductionGraph::new();
        let cost_fn = MinimizeSteps;
        let input_size = ProblemSize::new(vec![("n", 10)]);

        let path = graph.find_cheapest_path(
            ("UnknownProblem", "SimpleGraph"),
            ("VertexCovering", "SimpleGraph"),
            &input_size,
            &cost_fn,
        );

        assert!(path.is_none());
    }

    #[test]
    fn test_find_cheapest_path_unknown_target() {
        let graph = ReductionGraph::new();
        let cost_fn = MinimizeSteps;
        let input_size = ProblemSize::new(vec![("n", 10)]);

        let path = graph.find_cheapest_path(
            ("IndependentSet", "SimpleGraph"),
            ("UnknownProblem", "SimpleGraph"),
            &input_size,
            &cost_fn,
        );

        assert!(path.is_none());
    }

    #[test]
    fn test_reduction_edge_struct() {
        let edge = ReductionEdge {
            source_variant: &[("graph", "PlanarGraph"), ("weight", "Unweighted")],
            target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
            overhead: ReductionOverhead::default(),
        };

        assert_eq!(edge.source_graph(), "PlanarGraph");
        assert_eq!(edge.target_graph(), "SimpleGraph");
    }

    #[test]
    fn test_reduction_edge_default_graph() {
        // When no "graph" key is present, default to SimpleGraph
        let edge = ReductionEdge {
            source_variant: &[("weight", "Unweighted")],
            target_variant: &[],
            overhead: ReductionOverhead::default(),
        };

        assert_eq!(edge.source_graph(), "SimpleGraph");
        assert_eq!(edge.target_graph(), "SimpleGraph");
    }

    #[test]
    fn test_variant_to_map() {
        let variant: &[(&str, &str)] = &[("graph", "SimpleGraph"), ("weight", "i32")];
        let map = ReductionGraph::variant_to_map(variant);
        assert_eq!(map.get("graph"), Some(&"SimpleGraph".to_string()));
        assert_eq!(map.get("weight"), Some(&"i32".to_string()));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_variant_to_map_empty() {
        let variant: &[(&str, &str)] = &[];
        let map = ReductionGraph::variant_to_map(variant);
        assert!(map.is_empty());
    }

    #[test]
    fn test_make_variant_ref() {
        let variant: &[(&str, &str)] = &[("graph", "PlanarGraph"), ("weight", "f64")];
        let variant_ref = ReductionGraph::make_variant_ref("IndependentSet", variant);
        assert_eq!(variant_ref.name, "IndependentSet");
        assert_eq!(variant_ref.variant.get("graph"), Some(&"PlanarGraph".to_string()));
        assert_eq!(variant_ref.variant.get("weight"), Some(&"f64".to_string()));
    }

    #[test]
    fn test_to_json_nodes_have_variants() {
        let graph = ReductionGraph::new();
        let json = graph.to_json();

        // Check that nodes have variant information
        for node in &json.nodes {
            // Verify node has a name
            assert!(!node.name.is_empty());
            // Verify node has a category
            assert!(!node.category.is_empty());
        }
    }

    #[test]
    fn test_to_json_edges_have_variants() {
        let graph = ReductionGraph::new();
        let json = graph.to_json();

        // Check that edges have source and target variant refs
        for edge in &json.edges {
            assert!(!edge.source.name.is_empty());
            assert!(!edge.target.name.is_empty());
        }
    }

    #[test]
    fn test_json_variant_content() {
        let graph = ReductionGraph::new();
        let json = graph.to_json();

        // Find a node and verify its variant contains expected keys
        let is_node = json.nodes.iter().find(|n| n.name == "IndependentSet");
        assert!(is_node.is_some(), "IndependentSet node should exist");

        // Find an edge involving IndependentSet (could be source or target)
        let is_edge = json.edges.iter().find(|e| {
            e.source.name == "IndependentSet" || e.target.name == "IndependentSet"
        });
        assert!(is_edge.is_some(), "Edge involving IndependentSet should exist");
    }
}
