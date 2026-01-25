//! Runtime reduction graph for discovering and executing reduction paths.

use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use std::any::TypeId;
use std::collections::HashMap;

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
        let paths = graph.find_paths::<IndependentSet<i32>, crate::models::optimization::QUBO<f64>>();
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
        let path = graph.find_shortest_path::<IndependentSet<i32>, VertexCovering<i32>>().unwrap();

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
}
