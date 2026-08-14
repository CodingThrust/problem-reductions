//! Runtime reduction graph for discovering and executing reduction paths.
//!
//! The graph uses variant-level nodes: each node is a unique `(problem_name, variant)` pair.
//! Nodes come from `VariantEntry` inventory, and `ReductionEntry` inventory supplies edges.
//!
//! Edges come exclusively from `#[reduction]` registrations via `inventory::iter::<ReductionEntry>`.
//!
//! This module implements:
//! - Variant-level graph construction from `VariantEntry` and `ReductionEntry` inventory
//! - Symbolic path composition and concrete path execution
//! - JSON export for documentation and visualization

use crate::rules::registry::{
    AggregateReduceFn, EdgeCapabilities, ReduceFn, ReductionEntry, ReductionSizeContract,
    SizeContractError,
};
use crate::rules::traits::{DynAggregateReductionResult, DynReductionResult};
use crate::types::ProblemSize;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;
use std::any::Any;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

/// A source/target pair from the reduction graph, returned by
/// [`ReductionGraph::outgoing_reductions`] and [`ReductionGraph::incoming_reductions`].
#[derive(Debug, Clone)]
pub struct ReductionEdgeInfo {
    pub source_name: &'static str,
    pub source_variant: BTreeMap<String, String>,
    pub target_name: &'static str,
    pub target_variant: BTreeMap<String, String>,
    pub size_contract: Result<ReductionSizeContract, SizeContractError>,
    pub capabilities: EdgeCapabilities,
}

/// Internal edge data combining explicit size contracts and executable reduction functions.
#[derive(Clone)]
pub(crate) struct ReductionEdgeData {
    pub size_contract: Result<ReductionSizeContract, SizeContractError>,
    pub reduce_fn: Option<ReduceFn>,
    pub reduce_aggregate_fn: Option<AggregateReduceFn>,
    pub turing: bool,
}

impl ReductionEdgeData {
    fn capabilities(&self) -> EdgeCapabilities {
        EdgeCapabilities::from_executors(self.reduce_fn, self.reduce_aggregate_fn, self.turing)
    }
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
    /// Structural category declared by the problem schema.
    pub(crate) category: crate::registry::ProblemCategory,
    /// Relative rustdoc path (e.g., "models/graph/maximum_independent_set").
    pub(crate) doc_path: String,
    /// Worst-case time complexity expression (empty if not declared).
    pub(crate) complexity: String,
}

/// Internal reference to a problem variant, used as HashMap key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VariantRef {
    name: String,
    variant: BTreeMap<String, String>,
}

/// One explicitly classified target size field in graph export.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct SizeFieldJson {
    pub(crate) field: String,
    pub(crate) contract: &'static str,
    pub(crate) formula: Option<String>,
    pub(crate) reason: Option<String>,
}

/// An edge in the reduction graph JSON.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct EdgeJson {
    /// Index into the `nodes` array for the source problem variant.
    pub(crate) source: usize,
    /// Index into the `nodes` array for the target problem variant.
    pub(crate) target: usize,
    /// Symbolic or unavailable target-size fields.
    pub(crate) size_fields: Vec<SizeFieldJson>,
    pub(crate) size_contract_error: Option<String>,
    /// Relative rustdoc path for the reduction module.
    pub(crate) doc_path: String,
    /// Whether the edge supports witness/config workflows.
    pub(crate) witness: bool,
    /// Whether the edge supports aggregate/value workflows.
    pub(crate) aggregate: bool,
    /// Whether the edge is a Turing (multi-query) reduction.
    pub(crate) turing: bool,
}

/// A path through the variant-level reduction graph.
#[derive(Debug, Clone)]
pub struct ReductionPath {
    /// Variant-level steps in the path.
    pub steps: Vec<ReductionStep>,
}

/// A selected concrete path batch could not be executed.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ExecutePathsError {
    #[error("concrete path {path_index} is empty")]
    EmptyPath { path_index: usize },
    #[error("concrete path {path_index} contains no reduction edge")]
    NoEdges { path_index: usize },
    #[error("concrete path {path_index} starts at a different source node")]
    DifferentSource { path_index: usize },
    #[error("concrete path {path_index} references unknown node {problem} {variant:?}")]
    UnknownNode {
        path_index: usize,
        problem: String,
        variant: BTreeMap<String, String>,
    },
    #[error("concrete path {path_index} has no registered edge from {source_problem} to {target_problem}")]
    MissingEdge {
        path_index: usize,
        source_problem: String,
        target_problem: String,
    },
    #[error("concrete path {path_index} edge {source_problem} -> {target_problem} is not witness-executable")]
    NotWitnessExecutable {
        path_index: usize,
        source_problem: String,
        target_problem: String,
    },
}

/// Why symbolic size propagation could not be completed for a path.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PathSizeError {
    #[error("cannot compose an empty reduction path")]
    EmptyPath,
    #[error("reduction path references unknown node {problem} {variant:?}")]
    UnknownNode {
        problem: String,
        variant: BTreeMap<String, String>,
    },
    #[error(
        "reduction path contains no registered edge from {source_problem} to {target_problem}"
    )]
    MissingEdge {
        source_problem: String,
        target_problem: String,
    },
    #[error("reduction step {step} ({source_problem} -> {target_problem}) is a multi-query reduction without a query-cost model")]
    TuringEdge {
        step: usize,
        source_problem: String,
        target_problem: String,
    },
    #[error("reduction step {step} ({source_problem} -> {target_problem}) has an invalid size contract: {error}")]
    InvalidContract {
        step: usize,
        source_problem: String,
        target_problem: String,
        #[source]
        error: Box<SizeContractError>,
    },
    #[error("reduction step {step} ({source_problem} -> {target_problem}) has no symbolic size transform")]
    Unavailable {
        step: usize,
        source_problem: String,
        target_problem: String,
    },
    #[error(
        "cannot compose reduction step {step} ({source_problem} -> {target_problem}): {error}"
    )]
    Step {
        step: usize,
        source_problem: String,
        target_problem: String,
        #[source]
        error: Box<crate::size::SizeTransformError>,
    },
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
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize)]
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

/// Internal node data for the variant-level graph.
#[derive(Debug, Clone)]
struct VariantNode {
    name: &'static str,
    variant: BTreeMap<String, String>,
    complexity: &'static str,
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

/// Traversal mode for graph exploration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalFlow {
    /// Follow outgoing edges (what can this reduce to?).
    Outgoing,
    /// Follow incoming edges (what can reduce to this?).
    Incoming,
    /// Follow edges in both directions.
    Both,
}

/// Required capability for reduction path search.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionMode {
    Witness,
    Aggregate,
    /// Multi-query (Turing) reductions: solving the source requires multiple
    /// adaptive queries to the target (e.g., binary search over a bound).
    Turing,
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

/// Runtime graph of all registered reductions.
///
/// Uses variant-level nodes: each node is a unique `(problem_name, variant)` pair.
/// All edges come from `inventory::iter::<ReductionEntry>` registrations.
///
/// The graph supports:
/// - Auto-discovery of reductions from `inventory::iter::<ReductionEntry>`
/// - Path finding by problem type or by name
pub struct ReductionGraph {
    /// Graph with node indices as node data, edge weights as ReductionEdgeData.
    graph: DiGraph<usize, ReductionEdgeData>,
    /// All variant nodes, indexed by position.
    nodes: Vec<VariantNode>,
    /// Map from base type name to all NodeIndex values for that name.
    name_to_nodes: HashMap<&'static str, Vec<NodeIndex>>,
    /// Declared default variant for each problem name.
    default_variants: HashMap<String, BTreeMap<String, String>>,
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
                           complexity: &'static str,
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
                nodes.push(VariantNode {
                    name,
                    variant,
                    complexity,
                });
                let idx = graph.add_node(node_id);
                node_index.insert(vref, idx);
                name_to_nodes.entry(name).or_default().push(idx);
                idx
            }
        };

        // Collect declared default variants from VariantEntry inventory
        let mut default_variants: HashMap<String, BTreeMap<String, String>> = HashMap::new();

        // Phase 1: Build nodes from VariantEntry inventory
        for entry in inventory::iter::<crate::registry::VariantEntry> {
            let variant = Self::variant_to_map(&entry.variant());
            ensure_node(
                entry.name,
                variant.clone(),
                entry.complexity,
                &mut nodes,
                &mut graph,
                &mut node_index,
                &mut name_to_nodes,
            );
            if entry.is_default {
                default_variants.insert(entry.name.to_string(), variant);
            }
        }

        // Phase 2: Build edges from ReductionEntry inventory
        for entry in inventory::iter::<ReductionEntry> {
            let source_variant = Self::variant_to_map(&entry.source_variant());
            let target_variant = Self::variant_to_map(&entry.target_variant());

            let src_idx = node_index[&VariantRef {
                name: entry.source_name.to_string(),
                variant: source_variant,
            }];
            let dst_idx = node_index[&VariantRef {
                name: entry.target_name.to_string(),
                variant: target_variant,
            }];

            let size_contract = entry.size_contract();
            if graph.find_edge(src_idx, dst_idx).is_none() {
                graph.add_edge(
                    src_idx,
                    dst_idx,
                    ReductionEdgeData {
                        size_contract,
                        reduce_fn: entry.reduce_fn,
                        reduce_aggregate_fn: entry.reduce_aggregate_fn,
                        turing: entry.turing,
                    },
                );
            }
        }

        Self {
            graph,
            nodes,
            name_to_nodes,
            default_variants,
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

    fn edge_supports_mode(edge: &ReductionEdgeData, mode: ReductionMode) -> bool {
        match mode {
            ReductionMode::Witness => edge.reduce_fn.is_some(),
            ReductionMode::Aggregate => edge.reduce_aggregate_fn.is_some(),
            ReductionMode::Turing => edge.turing,
        }
    }

    fn ordered_outgoing_edges(
        &self,
        node: NodeIndex,
        mode: ReductionMode,
    ) -> Vec<(NodeIndex, EdgeIndex)> {
        let mut edges: Vec<_> = self
            .graph
            .edges(node)
            .filter(|edge| Self::edge_supports_mode(edge.weight(), mode))
            .map(|edge| (edge.target(), edge.id()))
            .collect();
        edges.sort_by(|a, b| {
            let a = &self.nodes[self.graph[a.0]];
            let b = &self.nodes[self.graph[b.0]];
            (a.name, &a.variant).cmp(&(b.name, &b.variant))
        });
        edges
    }

    fn node_path_supports_mode(&self, node_path: &[NodeIndex], mode: ReductionMode) -> bool {
        node_path.windows(2).all(|pair| {
            self.graph
                .find_edge(pair[0], pair[1])
                .is_some_and(|edge_idx| Self::edge_supports_mode(&self.graph[edge_idx], mode))
        })
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
        self.find_all_paths_mode(
            source,
            source_variant,
            target,
            target_variant,
            ReductionMode::Witness,
        )
    }

    /// Find all simple paths between two specific problem variants while
    /// requiring a specific edge capability.
    pub fn find_all_paths_mode(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        mode: ReductionMode,
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
            .filter(|p| self.node_path_supports_mode(p, mode))
            .map(|p| self.node_path_to_reduction_path(p))
            .collect()
    }

    /// Find up to `limit` simple paths between two specific problem variants.
    ///
    /// Like [`find_all_paths`](Self::find_all_paths) but stops enumeration after
    /// collecting `limit` paths. This avoids combinatorial explosion on dense graphs.
    pub fn find_paths_up_to(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        limit: usize,
    ) -> Vec<ReductionPath> {
        self.find_paths_up_to_mode_bounded(
            source,
            source_variant,
            target,
            target_variant,
            ReductionMode::Witness,
            limit,
            None,
        )
    }

    /// Like [`find_all_paths_mode`](Self::find_all_paths_mode) but stops
    /// enumeration after collecting `limit` paths.
    pub fn find_paths_up_to_mode(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        mode: ReductionMode,
        limit: usize,
    ) -> Vec<ReductionPath> {
        self.find_paths_up_to_mode_bounded(
            source,
            source_variant,
            target,
            target_variant,
            mode,
            limit,
            None,
        )
    }

    /// Like [`find_paths_up_to_mode`](Self::find_paths_up_to_mode) but also
    /// bounds the number of intermediate nodes in each enumerated path.
    #[allow(clippy::too_many_arguments)]
    pub fn find_paths_up_to_mode_bounded(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        mode: ReductionMode,
        limit: usize,
        max_intermediate_nodes: Option<usize>,
    ) -> Vec<ReductionPath> {
        let src = match self.lookup_node(source, source_variant) {
            Some(idx) => idx,
            None => return vec![],
        };
        let dst = match self.lookup_node(target, target_variant) {
            Some(idx) => idx,
            None => return vec![],
        };

        if limit == 0 {
            return Vec::new();
        }

        // Enumerate simple paths breadth-first. Each level is already lexicographic
        // because both the preceding level and every outgoing edge list are ordered
        // by canonical node identity. Completed paths therefore arrive in the exact
        // public order: fewest nodes first, then canonical node identity. Stop immediately
        // after `limit` results instead of traversing every simple path.
        let max_intermediate =
            max_intermediate_nodes.unwrap_or_else(|| self.graph.node_count().saturating_sub(2));
        let max_nodes = max_intermediate.saturating_add(2);
        let mut frontier = vec![vec![src]];
        let mut paths = Vec::with_capacity(limit);

        while !frontier.is_empty() && frontier[0].len() < max_nodes {
            let mut next_frontier = Vec::new();
            for path in frontier {
                let current = path[path.len() - 1];
                for (next, _) in self.ordered_outgoing_edges(current, mode) {
                    if path.contains(&next) {
                        continue;
                    }
                    let mut extended = path.clone();
                    extended.push(next);
                    if next == dst {
                        paths.push(self.node_path_to_reduction_path(&extended));
                        if paths.len() == limit {
                            return paths;
                        }
                    } else {
                        next_frontier.push(extended);
                    }
                }
            }
            frontier = next_frontier;
        }
        paths
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

    /// Check if a direct reduction exists by name in a specific mode.
    pub fn has_direct_reduction_by_name_mode(
        &self,
        src: &str,
        dst: &str,
        mode: ReductionMode,
    ) -> bool {
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
                if dst_set.contains(&edge_ref.target())
                    && Self::edge_supports_mode(edge_ref.weight(), mode)
                {
                    return true;
                }
            }
        }

        false
    }

    /// Check if a direct reduction exists from S to T in a specific mode.
    pub fn has_direct_reduction_mode<S: crate::traits::Problem, T: crate::traits::Problem>(
        &self,
        mode: ReductionMode,
    ) -> bool {
        self.has_direct_reduction_by_name_mode(S::NAME, T::NAME, mode)
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

    /// Return the symbolic size transform for every edge of a path.
    pub fn path_size_transforms(
        &self,
        path: &ReductionPath,
    ) -> Result<Vec<crate::size::SizeTransform>, PathSizeError> {
        if path.steps.len() <= 1 {
            return Ok(vec![]);
        }

        let node_indices: Vec<NodeIndex> = path
            .steps
            .iter()
            .map(|step| {
                self.lookup_node(&step.name, &step.variant).ok_or_else(|| {
                    PathSizeError::UnknownNode {
                        problem: step.name.clone(),
                        variant: step.variant.clone(),
                    }
                })
            })
            .collect::<Result<_, _>>()?;

        node_indices
            .windows(2)
            .enumerate()
            .map(|(index, pair)| {
                let edge_idx = self.graph.find_edge(pair[0], pair[1]).ok_or_else(|| {
                    PathSizeError::MissingEdge {
                        source_problem: path.steps[index].name.clone(),
                        target_problem: path.steps[index + 1].name.clone(),
                    }
                })?;
                if self.graph[edge_idx].turing {
                    return Err(PathSizeError::TuringEdge {
                        step: index + 1,
                        source_problem: path.steps[index].name.clone(),
                        target_problem: path.steps[index + 1].name.clone(),
                    });
                }
                let contract = self.graph[edge_idx]
                    .size_contract
                    .as_ref()
                    .map_err(|error| PathSizeError::InvalidContract {
                        step: index + 1,
                        source_problem: path.steps[index].name.clone(),
                        target_problem: path.steps[index + 1].name.clone(),
                        error: Box::new(error.clone()),
                    })?;
                contract
                    .transform()
                    .cloned()
                    .ok_or_else(|| PathSizeError::Unavailable {
                        step: index + 1,
                        source_problem: path.steps[index].name.clone(),
                        target_problem: path.steps[index + 1].name.clone(),
                    })
            })
            .collect()
    }

    /// Compose symbolic size transforms along a path.
    pub fn compose_path_size_transform(
        &self,
        path: &ReductionPath,
    ) -> Result<Option<crate::size::SizeTransform>, PathSizeError> {
        if path.steps.is_empty() {
            return Err(PathSizeError::EmptyPath);
        }
        if path.steps.len() == 1 {
            return Ok(None);
        }

        let mut transforms = self.path_size_transforms(path)?.into_iter();
        let Some(mut composed) = transforms.next() else {
            return Ok(None);
        };
        for (offset, transform) in transforms.enumerate() {
            let edge_index = offset + 1;
            composed = composed
                .compose(
                    &transform,
                    format!(
                        "{} -> {}",
                        path.steps[0].name,
                        path.steps[edge_index + 1].name
                    ),
                )
                .map_err(|error| PathSizeError::Step {
                    step: edge_index + 1,
                    source_problem: path.steps[edge_index].name.clone(),
                    target_problem: path.steps[edge_index + 1].name.clone(),
                    error: Box::new(error),
                })?;
        }
        Ok(Some(composed))
    }

    /// Get all variant maps registered for a problem name.
    ///
    /// Returns variants sorted deterministically: the "default" variant
    /// (SimpleGraph, i32, etc.) comes first, then remaining variants
    /// in lexicographic order.
    pub fn variants_for(&self, name: &str) -> Vec<BTreeMap<String, String>> {
        let mut variants: Vec<BTreeMap<String, String>> = self
            .name_to_nodes
            .get(name)
            .map(|indices| {
                indices
                    .iter()
                    .map(|&idx| self.nodes[self.graph[idx]].variant.clone())
                    .collect()
            })
            .unwrap_or_default();
        // Sort deterministically: default variant values (SimpleGraph, One, KN)
        // sort first so callers can rely on variants[0] being the "base" variant.
        variants.sort_by(|a, b| {
            fn default_rank(v: &BTreeMap<String, String>) -> usize {
                v.values()
                    .filter(|val| !["SimpleGraph", "One", "KN"].contains(&val.as_str()))
                    .count()
            }
            default_rank(a).cmp(&default_rank(b)).then_with(|| a.cmp(b))
        });
        variants
    }

    /// Get the declared default variant for a problem type.
    ///
    /// Returns the variant that was marked `default` in `declare_variants!`.
    /// If no entry was explicitly marked `default`, the first registered variant
    /// for the problem is used as the implicit default.
    /// Returns `None` if the problem type is not registered.
    pub fn default_variant_for(&self, name: &str) -> Option<BTreeMap<String, String>> {
        self.default_variants.get(name).cloned()
    }

    /// Get the complexity expression for a specific variant.
    pub fn variant_complexity(
        &self,
        name: &str,
        variant: &BTreeMap<String, String>,
    ) -> Option<&'static str> {
        let idx = self.lookup_node(name, variant)?;
        let node = &self.nodes[self.graph[idx]];
        if node.complexity.is_empty() {
            None
        } else {
            Some(node.complexity)
        }
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
                    size_contract: self.graph[e.id()].size_contract.clone(),
                    capabilities: self.graph[e.id()].capabilities(),
                }
            })
            .collect()
    }

    /// Get executable outgoing reductions from one exact problem variant.
    ///
    /// # Panics
    ///
    /// Panics if `name` and `variant` do not identify an exactly registered problem variant.
    pub fn outgoing_reductions_from(
        &self,
        name: &str,
        variant: &BTreeMap<String, String>,
        mode: ReductionMode,
    ) -> Vec<ReductionEdgeInfo> {
        let source = self
            .lookup_node(name, variant)
            .unwrap_or_else(|| panic!("registered problem variant not found: {name} {variant:?}"));

        self.ordered_outgoing_edges(source, mode)
            .into_iter()
            .map(|(target, edge)| {
                let src = &self.nodes[self.graph[source]];
                let dst = &self.nodes[self.graph[target]];
                ReductionEdgeInfo {
                    source_name: src.name,
                    source_variant: src.variant.clone(),
                    target_name: dst.name,
                    target_variant: dst.variant.clone(),
                    size_contract: self.graph[edge].size_contract.clone(),
                    capabilities: self.graph[edge].capabilities(),
                }
            })
            .collect()
    }

    /// Get the problem size field names for a problem type.
    ///
    /// Derives size fields from the explicit size contracts of reduction entries
    /// where this problem appears as source or target. When the problem is a
    /// source, its size fields are the input variables referenced in size
    /// expressions. When it's a target, its size fields are the output field names.
    pub fn size_field_names(&self, name: &str) -> Vec<String> {
        let mut fields: std::collections::HashSet<String> =
            crate::registry::declared_size_fields(name)
                .into_iter()
                .map(str::to_string)
                .collect();
        for entry in inventory::iter::<ReductionEntry> {
            let declarations = (entry.size_declarations_fn)();
            if entry.source_name == name {
                fields.extend(
                    declarations
                        .fields
                        .iter()
                        .flat_map(|(_, expression)| expression.variables())
                        .map(str::to_string),
                );
            }
            if entry.target_name == name {
                fields.extend(
                    declarations
                        .fields
                        .iter()
                        .map(|(field, _)| (*field).to_string()),
                );
                fields.extend(
                    declarations
                        .unavailable
                        .iter()
                        .map(|field| field.field.to_string()),
                );
            }
        }
        let mut result: Vec<String> = fields.into_iter().collect();
        result.sort_unstable();
        result
    }

    /// Evaluate every symbolic size transform along a reduction path.
    pub fn evaluate_path_size(
        &self,
        path: &ReductionPath,
        input_size: &ProblemSize,
    ) -> Result<crate::size::EvaluatedSize, PathSizeError> {
        let mut current = crate::size::EvaluatedSize::from_problem_size(input_size);
        for (index, transform) in self.path_size_transforms(path)?.iter().enumerate() {
            current = transform
                .evaluate(&current)
                .map_err(|error| PathSizeError::Step {
                    step: index + 1,
                    source_problem: path.steps[index].name.clone(),
                    target_problem: path.steps[index + 1].name.clone(),
                    error: Box::new(error),
                })?;
        }
        Ok(current)
    }

    /// Measure every size field used by a rule at this exact problem variant.
    ///
    /// Both sides of each rule contribute getters. In particular, a sink variant is
    /// measured from incoming rules instead of incorrectly producing an empty size.
    pub fn compute_problem_size(
        name: &str,
        variant: &BTreeMap<String, String>,
        instance: &dyn Any,
    ) -> ProblemSize {
        let mut merged: Vec<(String, usize)> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        let variant_matches = |entry_variant: Vec<(&str, &str)>| {
            let variant_matches = entry_variant.len() == variant.len()
                && entry_variant.iter().all(|(key, value)| {
                    let value = if *key == "graph" && value.is_empty() {
                        "SimpleGraph"
                    } else {
                        value
                    };
                    variant.get(*key).is_some_and(|expected| expected == value)
                });
            variant_matches
        };

        for entry in inventory::iter::<ReductionEntry> {
            let measured = if entry.source_name == name && variant_matches(entry.source_variant()) {
                Some((entry.source_size_measure_fn)(instance))
            } else if entry.target_name == name && variant_matches(entry.target_variant()) {
                Some((entry.target_size_measure_fn)(instance))
            } else {
                None
            };
            if let Some(measured) = measured {
                for (k, v) in measured.components {
                    if seen.insert(k.clone()) {
                        merged.push((k, v));
                    }
                }
            }
        }
        ProblemSize { components: merged }
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
                    size_contract: self.graph[e.id()].size_contract.clone(),
                    capabilities: self.graph[e.id()].capabilities(),
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
        direction: TraversalFlow,
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

            let directions = match direction {
                TraversalFlow::Outgoing => vec![petgraph::Outgoing],
                TraversalFlow::Incoming => vec![petgraph::Incoming],
                TraversalFlow::Both => {
                    vec![petgraph::Outgoing, petgraph::Incoming]
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
        direction: TraversalFlow,
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

            let directions = match direction {
                TraversalFlow::Outgoing => vec![petgraph::Outgoing],
                TraversalFlow::Incoming => vec![petgraph::Incoming],
                TraversalFlow::Both => {
                    vec![petgraph::Outgoing, petgraph::Incoming]
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

        // Build the model-owned metadata lookup from ProblemSchemaEntry inventory.
        let schema_metadata: HashMap<&str, (&str, crate::registry::ProblemCategory)> =
            inventory::iter::<ProblemSchemaEntry>
                .into_iter()
                .map(|entry| (entry.name, (entry.module_path, entry.category)))
                .collect();

        // Build sorted node list from the internal nodes
        let mut json_nodes: Vec<(usize, NodeJson)> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let &(module_path, category) =
                    schema_metadata.get(node.name).unwrap_or_else(|| {
                        panic!(
                            "missing problem schema for registered variant `{}`",
                            node.name
                        )
                    });
                (
                    i,
                    NodeJson {
                        name: node.name.to_string(),
                        variant: node.variant.clone(),
                        category,
                        doc_path: Self::doc_path_from_module_path(module_path, node.name),
                        complexity: node.complexity.to_string(),
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
            let contract = &edge_ref.weight().size_contract;
            let capabilities = edge_ref.weight().capabilities();

            let mut size_fields = Vec::new();
            if let Ok(contract) = contract {
                if let Some(transform) = contract.transform() {
                    let relation = match transform.relation() {
                        crate::size::SizeRelation::Exact => "exact",
                        crate::size::SizeRelation::UpperBound => "upper_bound",
                    };
                    size_fields.extend(transform.expressions().map(|(field, expression)| {
                        SizeFieldJson {
                            field: field.to_string(),
                            contract: relation,
                            formula: Some(expression.to_string()),
                            reason: None,
                        }
                    }));
                }
                size_fields.extend(contract.unavailable().iter().map(|unavailable| {
                    SizeFieldJson {
                        field: unavailable.field.to_string(),
                        contract: "unavailable",
                        formula: None,
                        reason: Some(unavailable.reason.to_string()),
                    }
                }));
            }
            let size_contract_error = contract.as_ref().err().map(ToString::to_string);

            // Find the doc_path from the matching ReductionEntry
            let src_name = self.nodes[src_node_id].name;
            let dst_name = self.nodes[dst_node_id].name;
            let src_variant = &self.nodes[src_node_id].variant;
            let dst_variant = &self.nodes[dst_node_id].variant;

            let doc_path = self.find_entry_doc_path(src_name, dst_name, src_variant, dst_variant);

            edges.push(EdgeJson {
                source: old_to_new[&src_node_id],
                target: old_to_new[&dst_node_id],
                size_fields,
                size_contract_error,
                doc_path,
                witness: capabilities.witness,
                aggregate: capabilities.aggregate,
                turing: capabilities.turing,
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

    /// Find the matching `ReductionEntry` for a (source_name, target_name) pair
    /// given exact source and target variants.
    ///
    /// Returns `Some(MatchedEntry)` only when both the source and target variants
    /// match exactly. No fallback is attempted — callers that need fuzzy matching
    /// should resolve variants before calling this method.
    pub fn find_entry(
        &self,
        source_name: &str,
        source_variant: &BTreeMap<String, String>,
        target_name: &str,
        target_variant: &BTreeMap<String, String>,
    ) -> Option<MatchedEntry> {
        for entry in inventory::iter::<ReductionEntry> {
            if entry.source_name != source_name || entry.target_name != target_name {
                continue;
            }

            let entry_source = Self::variant_to_map(&entry.source_variant());
            let entry_target = Self::variant_to_map(&entry.target_variant());

            // Exact match on both source and target variant
            if source_variant == &entry_source && target_variant == &entry_target {
                return Some(MatchedEntry {
                    source_variant: entry_source,
                    target_variant: entry_target,
                    size_contract: entry.size_contract(),
                });
            }
        }

        None
    }
}

/// A matched reduction entry returned by [`ReductionGraph::find_entry`].
pub struct MatchedEntry {
    /// The entry's source variant.
    pub source_variant: BTreeMap<String, String>,
    /// The entry's target variant.
    pub target_variant: BTreeMap<String, String>,
    /// The reduction's explicit size contract.
    pub size_contract: Result<ReductionSizeContract, SizeContractError>,
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
    pub fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        let mut solution = target_solution.to_vec();
        for step in self.steps.iter().rev() {
            solution = step.extract_solution_dyn(&solution)?;
        }
        Ok(solution)
    }
}

/// A composed aggregate reduction chain produced by
/// [`ReductionGraph::reduce_aggregate_along_path`].
pub struct AggregateReductionChain {
    steps: Vec<Box<dyn DynAggregateReductionResult>>,
}

impl AggregateReductionChain {
    /// Get the final target problem as a type-erased reference.
    pub fn target_problem_any(&self) -> &dyn Any {
        self.steps
            .last()
            .expect("AggregateReductionChain has no steps")
            .target_problem_any()
    }

    /// Get a typed reference to the final target problem.
    ///
    /// Panics if the actual target type does not match `T`.
    pub fn target_problem<T: 'static>(&self) -> &T {
        self.target_problem_any()
            .downcast_ref::<T>()
            .expect("AggregateReductionChain target type mismatch")
    }

    /// Extract an aggregate value from target space back to source space.
    pub fn extract_value_dyn(&self, target_value: serde_json::Value) -> serde_json::Value {
        self.steps
            .iter()
            .rev()
            .fold(target_value, |value, step| step.extract_value_dyn(value))
    }
}

impl ReductionGraph {
    fn execute_aggregate_edge(
        &self,
        edge_idx: EdgeIndex,
        input: &dyn Any,
    ) -> Option<Box<dyn DynAggregateReductionResult>> {
        let edge = &self.graph[edge_idx];
        if !Self::edge_supports_mode(edge, ReductionMode::Aggregate) {
            return None;
        }

        Some(edge.reduce_aggregate_fn?(input))
    }

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
            if !Self::edge_supports_mode(&self.graph[edge_idx], ReductionMode::Witness) {
                return None;
            }
            edge_fns.push(self.graph[edge_idx].reduce_fn?);
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

    /// Execute an aggregate-value reduction path on a source problem instance.
    pub fn reduce_aggregate_along_path(
        &self,
        path: &ReductionPath,
        source: &dyn Any,
    ) -> Option<AggregateReductionChain> {
        if path.steps.len() < 2 {
            return None;
        }

        let mut edge_indices = Vec::new();
        for window in path.steps.windows(2) {
            let src = self.lookup_node(&window[0].name, &window[0].variant)?;
            let dst = self.lookup_node(&window[1].name, &window[1].variant)?;
            let edge_idx = self.graph.find_edge(src, dst)?;
            edge_indices.push(edge_idx);
        }

        let mut steps: Vec<Box<dyn DynAggregateReductionResult>> = Vec::new();
        let step = self.execute_aggregate_edge(edge_indices[0], source)?;
        steps.push(step);
        for &edge_idx in &edge_indices[1..] {
            let step = {
                let prev_target = steps.last().unwrap().target_problem_any();
                self.execute_aggregate_edge(edge_idx, prev_target)?
            };
            steps.push(step);
        }
        Some(AggregateReductionChain { steps })
    }
}

/// A concrete reduction path whose reductions have already been executed.
///
/// The constructed chain is retained so callers can inspect target sizes and
/// extract solutions without re-executing any reduction.
pub struct ExecutedPath {
    /// The variant-level path.
    pub path: ReductionPath,
    /// The executed reduction steps (one per hop), shared via `Rc`.
    steps: Vec<Rc<dyn DynReductionResult>>,
}

impl ExecutedPath {
    /// Get the final target problem as a type-erased reference.
    pub fn target_problem_any(&self) -> &dyn Any {
        self.steps
            .last()
            .expect("ExecutedPath has no steps")
            .target_problem_any()
    }

    /// Return the size of every concrete intermediate target in this path.
    pub fn target_sizes(&self) -> Vec<ProblemSize> {
        self.steps
            .iter()
            .zip(self.path.steps.iter().skip(1))
            .map(|(result, target)| {
                ReductionGraph::compute_problem_size(
                    &target.name,
                    &target.variant,
                    result.target_problem_any(),
                )
            })
            .collect()
    }

    /// Extract a solution from target space back to source space.
    pub fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        let mut solution = target_solution.to_vec();
        for step in self.steps.iter().rev() {
            solution = step.extract_solution_dyn(&solution)?;
        }
        Ok(solution)
    }
}

impl ReductionGraph {
    /// Execute a selected batch of witness paths while sharing every common prefix.
    pub fn execute_paths(
        &self,
        paths: &[ReductionPath],
        source_instance: &dyn Any,
    ) -> Result<Vec<ExecutedPath>, ExecutePathsError> {
        let mut prefixes: HashMap<Vec<ReductionStep>, Vec<Rc<dyn DynReductionResult>>> =
            HashMap::new();
        let mut executed = Vec::with_capacity(paths.len());
        let mut batch_source: Option<&ReductionStep> = None;
        for (path_index, path) in paths.iter().enumerate() {
            let source = path
                .steps
                .first()
                .ok_or(ExecutePathsError::EmptyPath { path_index })?;
            if path.steps.len() < 2 {
                return Err(ExecutePathsError::NoEdges { path_index });
            }
            if let Some(expected) = batch_source {
                if source != expected {
                    return Err(ExecutePathsError::DifferentSource { path_index });
                }
            } else {
                batch_source = Some(source);
            }
            let source_prefix = vec![source.clone()];
            let mut chain = prefixes.get(&source_prefix).cloned().unwrap_or_default();
            prefixes.entry(source_prefix.clone()).or_default();
            let mut prefix = source_prefix;
            for pair in path.steps.windows(2) {
                prefix.push(pair[1].clone());
                if let Some(cached) = prefixes.get(&prefix) {
                    chain = cached.clone();
                    continue;
                }
                let source_node = self
                    .lookup_node(&pair[0].name, &pair[0].variant)
                    .ok_or_else(|| ExecutePathsError::UnknownNode {
                        path_index,
                        problem: pair[0].name.clone(),
                        variant: pair[0].variant.clone(),
                    })?;
                let target_node_index = self
                    .lookup_node(&pair[1].name, &pair[1].variant)
                    .ok_or_else(|| ExecutePathsError::UnknownNode {
                        path_index,
                        problem: pair[1].name.clone(),
                        variant: pair[1].variant.clone(),
                    })?;
                let edge_index = self
                    .graph
                    .find_edge(source_node, target_node_index)
                    .ok_or_else(|| ExecutePathsError::MissingEdge {
                        path_index,
                        source_problem: pair[0].name.clone(),
                        target_problem: pair[1].name.clone(),
                    })?;
                let edge_data = &self.graph[edge_index];
                let Some(reduce_fn) = edge_data.reduce_fn else {
                    return Err(ExecutePathsError::NotWitnessExecutable {
                        path_index,
                        source_problem: pair[0].name.clone(),
                        target_problem: pair[1].name.clone(),
                    });
                };
                let current = chain
                    .last()
                    .map(|step| step.target_problem_any())
                    .unwrap_or(source_instance);
                chain.push(Rc::from(reduce_fn(current)));
                prefixes.insert(prefix.clone(), chain.clone());
            }
            executed.push(ExecutedPath {
                path: path.clone(),
                steps: chain,
            });
        }
        Ok(executed)
    }
}

#[cfg(test)]
impl ReductionGraph {
    /// Build a bare reduction graph from an explicit node/edge list (test-only).
    ///
    /// Nodes carry the empty variant and empty complexity; each edge carries a
    /// [`ReductionEdgeData`] without depending on registered inventory.
    pub(crate) fn from_test_edges(
        node_names: &[&'static str],
        edges: &[(&'static str, &'static str, ReductionEdgeData)],
    ) -> Self {
        Self::from_test_variant_edges(
            &node_names
                .iter()
                .map(|&name| (name, BTreeMap::new()))
                .collect::<Vec<_>>(),
            edges,
        )
    }

    pub(crate) fn from_test_variant_edges(
        test_nodes: &[(&'static str, BTreeMap<String, String>)],
        edges: &[(&'static str, &'static str, ReductionEdgeData)],
    ) -> Self {
        let mut graph: DiGraph<usize, ReductionEdgeData> = DiGraph::new();
        let mut nodes: Vec<VariantNode> = Vec::new();
        let mut name_to_nodes: HashMap<&'static str, Vec<NodeIndex>> = HashMap::new();
        let mut index_of: HashMap<&'static str, NodeIndex> = HashMap::new();

        for (name, variant) in test_nodes {
            let node_id = nodes.len();
            nodes.push(VariantNode {
                name,
                variant: variant.clone(),
                complexity: "",
            });
            let idx = graph.add_node(node_id);
            index_of.insert(name, idx);
            name_to_nodes.entry(name).or_default().push(idx);
        }

        for (src, dst, data) in edges {
            let s = index_of[src];
            let d = index_of[dst];
            graph.add_edge(s, d, data.clone());
        }

        Self {
            graph,
            nodes,
            name_to_nodes,
            default_variants: HashMap::new(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/graph.rs"]
mod tests;

#[cfg(test)]
#[path = "../unit_tests/rules/reduction_path_parity.rs"]
mod reduction_path_parity_tests;

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_ilp.rs"]
mod maximumindependentset_ilp_path_tests;

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_ilp.rs"]
mod minimumvertexcover_ilp_path_tests;

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_qubo.rs"]
mod maximumindependentset_qubo_path_tests;

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_qubo.rs"]
mod minimumvertexcover_qubo_path_tests;
