//! Runtime reduction graph for discovering and executing reduction paths.

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
    /// Unique identifier for the node.
    pub id: String,
    /// Display label for the node.
    pub label: String,
    /// Category of the problem (e.g., "graph", "set", "optimization", "satisfiability", "specialized").
    pub category: String,
    /// X position for layout (computed automatically).
    pub x: f64,
    /// Y position for layout (computed automatically).
    pub y: f64,
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
    /// Human-readable type names in the path.
    pub type_names: Vec<&'static str>,
    /// Type IDs for each step (reserved for future use).
    #[allow(dead_code)]
    type_ids: Vec<TypeId>,
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
pub struct ReductionGraph {
    graph: DiGraph<TypeId, ()>,
    type_names: HashMap<TypeId, &'static str>,
    node_indices: HashMap<TypeId, NodeIndex>,
}

impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions.
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut type_names = HashMap::new();
        let mut node_indices = HashMap::new();

        // Register all problem types
        Self::register_types(&mut graph, &mut type_names, &mut node_indices);

        // Register all reductions as edges
        Self::register_reductions(&mut graph, &node_indices);

        Self {
            graph,
            type_names,
            node_indices,
        }
    }

    fn register_types(
        graph: &mut DiGraph<TypeId, ()>,
        type_names: &mut HashMap<TypeId, &'static str>,
        node_indices: &mut HashMap<TypeId, NodeIndex>,
    ) {
        // Add all problem types
        macro_rules! register {
            ($($ty:ty => $name:expr),* $(,)?) => {
                $(
                    let id = TypeId::of::<$ty>();
                    let idx = graph.add_node(id);
                    type_names.insert(id, $name);
                    node_indices.insert(id, idx);
                )*
            };
        }

        use crate::models::graph::*;
        use crate::models::optimization::*;
        use crate::models::satisfiability::*;
        use crate::models::set::*;
        use crate::models::specialized::*;

        register! {
            IndependentSet<i32> => "IndependentSet<i32>",
            VertexCovering<i32> => "VertexCovering<i32>",
            SetPacking<i32> => "SetPacking<i32>",
            SetCovering<i32> => "SetCovering<i32>",
            Matching<i32> => "Matching<i32>",
            DominatingSet<i32> => "DominatingSet<i32>",
            Coloring => "Coloring",
            MaxCut<i32> => "MaxCut<i32>",
            SpinGlass<i32> => "SpinGlass<i32>",
            SpinGlass<f64> => "SpinGlass<f64>",
            QUBO<f64> => "QUBO<f64>",
            Satisfiability<i32> => "Satisfiability<i32>",
            KSatisfiability<3, i32> => "KSatisfiability<3, i32>",
            CircuitSAT<i32> => "CircuitSAT<i32>",
            Factoring => "Factoring",
        }
    }

    fn register_reductions(
        graph: &mut DiGraph<TypeId, ()>,
        node_indices: &HashMap<TypeId, NodeIndex>,
    ) {
        use crate::models::graph::*;
        use crate::models::optimization::*;
        use crate::models::satisfiability::*;
        use crate::models::set::*;
        use crate::models::specialized::*;

        macro_rules! add_edge {
            ($src:ty => $dst:ty) => {
                if let (Some(&src), Some(&dst)) = (
                    node_indices.get(&TypeId::of::<$src>()),
                    node_indices.get(&TypeId::of::<$dst>()),
                ) {
                    graph.add_edge(src, dst, ());
                }
            };
        }

        // Register all implemented reductions

        // Graph problem reductions
        add_edge!(IndependentSet<i32> => VertexCovering<i32>);
        add_edge!(VertexCovering<i32> => IndependentSet<i32>);
        add_edge!(IndependentSet<i32> => SetPacking<i32>);
        add_edge!(SetPacking<i32> => IndependentSet<i32>);
        add_edge!(VertexCovering<i32> => SetCovering<i32>);
        add_edge!(Matching<i32> => SetPacking<i32>);

        // Optimization reductions
        add_edge!(SpinGlass<f64> => QUBO<f64>);
        add_edge!(QUBO<f64> => SpinGlass<f64>);
        add_edge!(MaxCut<i32> => SpinGlass<i32>);
        add_edge!(SpinGlass<i32> => MaxCut<i32>);

        // SAT-based reductions
        add_edge!(Satisfiability<i32> => KSatisfiability<3, i32>);
        add_edge!(KSatisfiability<3, i32> => Satisfiability<i32>);
        add_edge!(Satisfiability<i32> => IndependentSet<i32>);
        add_edge!(Satisfiability<i32> => Coloring);
        add_edge!(Satisfiability<i32> => DominatingSet<i32>);

        // Circuit reductions
        add_edge!(CircuitSAT<i32> => SpinGlass<i32>);
        add_edge!(Factoring => CircuitSAT<i32>);
    }

    /// Find all paths from source to target type.
    pub fn find_paths<S: 'static, T: 'static>(&self) -> Vec<ReductionPath> {
        let src_id = TypeId::of::<S>();
        let dst_id = TypeId::of::<T>();

        let src_idx = match self.node_indices.get(&src_id) {
            Some(&idx) => idx,
            None => return vec![],
        };
        let dst_idx = match self.node_indices.get(&dst_id) {
            Some(&idx) => idx,
            None => return vec![],
        };

        let paths: Vec<Vec<NodeIndex>> =
            all_simple_paths(&self.graph, src_idx, dst_idx, 0, None).collect();

        paths
            .into_iter()
            .map(|path| {
                let type_ids: Vec<TypeId> = path.iter().map(|&idx| self.graph[idx]).collect();
                let type_names: Vec<&'static str> = type_ids
                    .iter()
                    .filter_map(|id| self.type_names.get(id).copied())
                    .collect();
                ReductionPath {
                    type_names,
                    type_ids,
                }
            })
            .collect()
    }

    /// Find the shortest path from source to target type.
    pub fn find_shortest_path<S: 'static, T: 'static>(&self) -> Option<ReductionPath> {
        let paths = self.find_paths::<S, T>();
        paths.into_iter().min_by_key(|p| p.len())
    }

    /// Check if a direct reduction exists from S to T.
    pub fn has_direct_reduction<S: 'static, T: 'static>(&self) -> bool {
        let src_id = TypeId::of::<S>();
        let dst_id = TypeId::of::<T>();

        if let (Some(&src_idx), Some(&dst_idx)) = (
            self.node_indices.get(&src_id),
            self.node_indices.get(&dst_id),
        ) {
            self.graph.find_edge(src_idx, dst_idx).is_some()
        } else {
            false
        }
    }

    /// Get all registered problem type names.
    pub fn problem_types(&self) -> Vec<&'static str> {
        self.type_names.values().copied().collect()
    }

    /// Get the number of registered problem types.
    pub fn num_types(&self) -> usize {
        self.type_names.len()
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
        let mut edge_set: HashMap<(String, String), bool> = HashMap::new();

        for edge in self.graph.edge_indices() {
            if let Some((src_idx, dst_idx)) = self.graph.edge_endpoints(edge) {
                let src_id = self.graph[src_idx];
                let dst_id = self.graph[dst_idx];

                if let (Some(&src_name), Some(&dst_name)) =
                    (self.type_names.get(&src_id), self.type_names.get(&dst_id))
                {
                    let src = src_name.to_string();
                    let dst = dst_name.to_string();

                    // Check if reverse edge exists
                    let reverse_key = (dst.clone(), src.clone());
                    if edge_set.contains_key(&reverse_key) {
                        // Mark the existing edge as bidirectional
                        edge_set.insert(reverse_key, true);
                    } else {
                        edge_set.insert((src, dst), false);
                    }
                }
            }
        }

        // Compute layered layout positions
        let positions = self.compute_layered_layout();

        // Build nodes with categories and positions
        let nodes: Vec<NodeJson> = self
            .type_names
            .values()
            .map(|&name| {
                let category = Self::categorize_type(name);
                let (x, y) = positions.get(name).copied().unwrap_or((0.0, 0.0));
                NodeJson {
                    id: name.to_string(),
                    label: Self::simplify_type_name(name),
                    category: category.to_string(),
                    x,
                    y,
                }
            })
            .collect();

        // Build edges (only include one direction for bidirectional edges)
        let edges: Vec<EdgeJson> = edge_set
            .into_iter()
            .map(|((src, dst), bidirectional)| EdgeJson {
                source: src,
                target: dst,
                bidirectional,
            })
            .collect();

        ReductionGraphJson { nodes, edges }
    }

    /// Compute layered layout positions using a simplified Sugiyama algorithm.
    fn compute_layered_layout(&self) -> HashMap<&'static str, (f64, f64)> {
        use std::collections::VecDeque;

        let mut positions: HashMap<&'static str, (f64, f64)> = HashMap::new();

        // Step 1: Assign layers based on category hierarchy
        // This is simpler and avoids cycles in bidirectional edges
        let mut layers: HashMap<&'static str, usize> = HashMap::new();

        for &name in self.type_names.values() {
            let layer = match Self::categorize_type(name) {
                "specialized" => {
                    if name.contains("Factoring") {
                        0
                    } else {
                        1
                    }
                }
                "satisfiability" => {
                    if name.contains("Circuit") {
                        1
                    } else {
                        2
                    }
                }
                "graph" => 3,
                "set" => 4,
                "optimization" => 2,
                _ => 3,
            };
            layers.insert(name, layer);
        }

        // Step 2: Refine layers using BFS from sources (with visit tracking)
        let mut visited: std::collections::HashSet<NodeIndex> = std::collections::HashSet::new();
        let mut queue: VecDeque<(NodeIndex, usize)> = VecDeque::new();

        // Start from Factoring (if exists) or any source
        for (type_id, &idx) in &self.node_indices {
            if let Some(&name) = self.type_names.get(type_id) {
                if name.contains("Factoring") {
                    queue.push_back((idx, 0));
                    visited.insert(idx);
                    layers.insert(name, 0);
                    break;
                }
            }
        }

        // BFS with limited iterations to prevent infinite loops
        let max_iterations = self.graph.node_count() * 10;
        let mut iterations = 0;

        while let Some((idx, depth)) = queue.pop_front() {
            iterations += 1;
            if iterations > max_iterations {
                break;
            }

            for neighbor in self.graph.neighbors(idx) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    if let Some(&neighbor_name) = self.type_names.get(&self.graph[neighbor]) {
                        let current_layer = layers.get(neighbor_name).copied().unwrap_or(0);
                        let new_layer = depth + 1;
                        if new_layer > current_layer {
                            layers.insert(neighbor_name, new_layer);
                        }
                        queue.push_back((neighbor, new_layer));
                    }
                }
            }
        }

        // Step 3: Group nodes by layer
        let mut layer_groups: HashMap<usize, Vec<&'static str>> = HashMap::new();
        for (&name, &layer) in &layers {
            layer_groups.entry(layer).or_default().push(name);
        }

        // Step 4: Sort nodes within each layer by category for visual grouping
        for nodes in layer_groups.values_mut() {
            nodes.sort_by_key(|&name| match Self::categorize_type(name) {
                "specialized" => 0,
                "satisfiability" => 1,
                "graph" => 2,
                "set" => 3,
                "optimization" => 4,
                _ => 5,
            });
        }

        // Step 5: Assign positions (compact layout)
        let y_spacing = 1.0;
        let x_spacing = 1.2;

        for (&layer, nodes) in &layer_groups {
            let y = layer as f64 * y_spacing;
            let total_width = (nodes.len() as f64 - 1.0) * x_spacing;
            let start_x = -total_width / 2.0;

            for (i, &name) in nodes.iter().enumerate() {
                let x = start_x + i as f64 * x_spacing;
                positions.insert(name, (x, y));
            }
        }

        positions
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

    /// Simplify a type name for display (remove generic parameters).
    fn simplify_type_name(name: &str) -> String {
        // Remove <i32>, <f64>, etc. for cleaner display
        if let Some(idx) = name.find('<') {
            name[..idx].to_string()
        } else {
            name.to_string()
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
        // No path between IS<i32> and QUBO<f64> (different weight types)
        let paths =
            graph.find_paths::<IndependentSet<i32>, crate::models::optimization::QUBO<f64>>();
        assert!(paths.is_empty());
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
    fn test_simplify_type_name() {
        assert_eq!(
            ReductionGraph::simplify_type_name("IndependentSet<i32>"),
            "IndependentSet"
        );
        assert_eq!(
            ReductionGraph::simplify_type_name("SpinGlass<f64>"),
            "SpinGlass"
        );
        assert_eq!(ReductionGraph::simplify_type_name("Coloring"), "Coloring");
        assert_eq!(
            ReductionGraph::simplify_type_name("KSatisfiability<3, i32>"),
            "KSatisfiability"
        );
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
        assert!(graph.has_direct_reduction::<CircuitSAT<i32>, SpinGlass<i32>>());

        // Find path from Factoring to SpinGlass
        let paths = graph.find_paths::<Factoring, SpinGlass<i32>>();
        assert!(!paths.is_empty());
        let shortest = graph
            .find_shortest_path::<Factoring, SpinGlass<i32>>()
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
        assert!(graph.has_direct_reduction::<MaxCut<i32>, SpinGlass<i32>>());
        assert!(graph.has_direct_reduction::<SpinGlass<i32>, MaxCut<i32>>());
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
    fn test_json_layout_positions() {
        let graph = ReductionGraph::new();
        let json = graph.to_json();

        // Check that all nodes have positions
        for node in &json.nodes {
            // x and y should be finite numbers
            assert!(node.x.is_finite());
            assert!(node.y.is_finite());
        }

        // Check that Factoring is at or near the top (y=0)
        let factoring = json.nodes.iter().find(|n| n.label == "Factoring");
        assert!(factoring.is_some());
        assert!(factoring.unwrap().y <= 0.1);
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
            type_ids: vec![],
        };
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
        assert!(path.source().is_none());
        assert!(path.target().is_none());
    }

    #[test]
    fn test_single_node_path() {
        let path = ReductionPath {
            type_names: vec!["IndependentSet<i32>"],
            type_ids: vec![std::any::TypeId::of::<IndependentSet<i32>>()],
        };
        assert!(!path.is_empty());
        assert_eq!(path.len(), 0); // No reductions, just one type
        assert_eq!(path.source(), Some("IndependentSet<i32>"));
        assert_eq!(path.target(), Some("IndependentSet<i32>"));
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
        assert!(parsed["nodes"].as_array().unwrap().len() > 0);
        assert!(parsed["edges"].as_array().unwrap().len() > 0);

        // Clean up
        let _ = fs::remove_file(&file_path);
    }
}
