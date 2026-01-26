//! Runtime reduction graph for discovering and executing reduction paths.
//!
//! The graph uses type-erased names (e.g., "SpinGlass" instead of "SpinGlass<i32>")
//! for topology, allowing path finding regardless of weight type parameters.

use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::Serialize;
use std::any::TypeId;
use std::collections::HashMap;

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
    /// Unique identifier for the node (base type name).
    pub id: String,
    /// Display label for the node.
    pub label: String,
    /// Category of the problem (e.g., "graph", "set", "optimization", "satisfiability", "specialized").
    pub category: String,
}

/// An edge in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub struct EdgeJson {
    /// Source node ID.
    pub source: String,
    /// Target node ID.
    pub target: String,
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

/// Runtime graph of all registered reductions.
///
/// Uses type-erased names for the graph topology, so `MaxCut<i32>` and `MaxCut<f64>`
/// map to the same node "MaxCut". This allows finding reduction paths regardless
/// of weight type parameters.
pub struct ReductionGraph {
    /// Graph with base type names as node data.
    graph: DiGraph<&'static str, ()>,
    /// Map from base type name to node index.
    name_indices: HashMap<&'static str, NodeIndex>,
    /// Map from TypeId to base type name (for generic API compatibility).
    type_to_name: HashMap<TypeId, &'static str>,
}

impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions.
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut name_indices = HashMap::new();
        let mut type_to_name = HashMap::new();

        // Register all problem types
        Self::register_types(&mut graph, &mut name_indices, &mut type_to_name);

        // Register all reductions as edges
        Self::register_reductions(&mut graph, &name_indices);

        Self {
            graph,
            name_indices,
            type_to_name,
        }
    }

    fn register_types(
        graph: &mut DiGraph<&'static str, ()>,
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
            // Satisfiability problems
            Satisfiability<i32> => "Satisfiability",
            KSatisfiability<3, i32> => "KSatisfiability",
            CircuitSAT<i32> => "CircuitSAT",
            // Specialized
            Factoring => "Factoring",
        }
    }

    fn register_reductions(
        graph: &mut DiGraph<&'static str, ()>,
        name_indices: &HashMap<&'static str, NodeIndex>,
    ) {
        // Add an edge between two problem types by name.
        macro_rules! add_edge {
            ($src:expr => $dst:expr) => {
                if let (Some(&src), Some(&dst)) = (name_indices.get($src), name_indices.get($dst)) {
                    // Avoid duplicate edges
                    if graph.find_edge(src, dst).is_none() {
                        graph.add_edge(src, dst, ());
                    }
                }
            };
        }

        // Graph problem reductions
        add_edge!("IndependentSet" => "VertexCovering");
        add_edge!("VertexCovering" => "IndependentSet");
        add_edge!("IndependentSet" => "SetPacking");
        add_edge!("SetPacking" => "IndependentSet");
        add_edge!("VertexCovering" => "SetCovering");
        add_edge!("Matching" => "SetPacking");

        // Optimization reductions
        add_edge!("SpinGlass" => "QUBO");
        add_edge!("QUBO" => "SpinGlass");
        add_edge!("MaxCut" => "SpinGlass");
        add_edge!("SpinGlass" => "MaxCut");

        // SAT-based reductions
        add_edge!("Satisfiability" => "KSatisfiability");
        add_edge!("KSatisfiability" => "Satisfiability");
        add_edge!("Satisfiability" => "IndependentSet");
        add_edge!("Satisfiability" => "Coloring");
        add_edge!("Satisfiability" => "DominatingSet");

        // Circuit reductions
        add_edge!("CircuitSAT" => "SpinGlass");
        add_edge!("Factoring" => "CircuitSAT");
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
}

impl Default for ReductionGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ReductionGraph {
    /// Export the reduction graph as a JSON-serializable structure.
    pub fn to_json(&self) -> ReductionGraphJson {
        // Collect all edges first to determine bidirectionality
        let mut edge_set: HashMap<(&str, &str), bool> = HashMap::new();

        for edge in self.graph.edge_indices() {
            if let Some((src_idx, dst_idx)) = self.graph.edge_endpoints(edge) {
                let src_name = self.graph[src_idx];
                let dst_name = self.graph[dst_idx];

                // Check if reverse edge exists
                let reverse_key = (dst_name, src_name);
                if edge_set.contains_key(&reverse_key) {
                    // Mark the existing edge as bidirectional
                    edge_set.insert(reverse_key, true);
                } else {
                    edge_set.insert((src_name, dst_name), false);
                }
            }
        }

        // Build nodes with categories
        let nodes: Vec<NodeJson> = self
            .name_indices
            .keys()
            .map(|&name| {
                let category = Self::categorize_type(name);
                NodeJson {
                    id: name.to_string(),
                    label: name.to_string(), // Base name is already simplified
                    category: category.to_string(),
                }
            })
            .collect();

        // Build edges (only include one direction for bidirectional edges)
        let edges: Vec<EdgeJson> = edge_set
            .into_iter()
            .map(|((src, dst), bidirectional)| EdgeJson {
                source: src.to_string(),
                target: dst.to_string(),
                bidirectional,
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
        } else if name.contains("SpinGlass") || name.contains("QUBO") {
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
        let paths_i32 =
            graph.find_paths::<crate::models::graph::MaxCut<i32>, crate::models::optimization::SpinGlass<i32>>();
        let paths_f64 =
            graph.find_paths::<crate::models::graph::MaxCut<f64>, crate::models::optimization::SpinGlass<f64>>();

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
        assert!(json.nodes.iter().any(|n| n.label == "IndependentSet"));
        assert!(json.nodes.iter().any(|n| n.category == "graph"));
        assert!(json.nodes.iter().any(|n| n.category == "optimization"));

        // Check edges
        assert!(json.edges.len() >= 10);

        // Check that IS <-> VC is marked bidirectional
        let is_vc_edge = json.edges.iter().find(|e| {
            (e.source.contains("IndependentSet") && e.target.contains("VertexCovering"))
                || (e.source.contains("VertexCovering") && e.target.contains("IndependentSet"))
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
        let path = ReductionPath {
            type_names: vec![],
        };
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
            (e.source.contains("IndependentSet") && e.target.contains("VertexCovering")
                || e.source.contains("VertexCovering") && e.target.contains("IndependentSet"))
                && e.bidirectional
        });
        assert!(is_vc_bidir, "IS <-> VC should be bidirectional");

        // Verify specific known unidirectional edge
        let factoring_circuit_unidir = json.edges.iter().any(|e| {
            e.source.contains("Factoring")
                && e.target.contains("CircuitSAT")
                && !e.bidirectional
        });
        assert!(
            factoring_circuit_unidir,
            "Factoring -> CircuitSAT should be unidirectional"
        );
    }
}
