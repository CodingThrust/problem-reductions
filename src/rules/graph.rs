//! Runtime reduction graph for discovering and executing reduction paths.
//!
//! The graph uses variant-level nodes: each node is a unique `(problem_name, variant)` pair.
//! Nodes come from `VariantEntry` inventory, and `ReductionEntry` inventory supplies edges.
//!
//! Edges come exclusively from `#[reduction]` registrations via `inventory::iter::<ReductionEntry>`.
//!
//! This module implements:
//! - Variant-level graph construction from `VariantEntry` and `ReductionEntry` inventory
//! - Exact and bounded-approximate Pareto path search with custom cost functions
//! - JSON export for documentation and visualization

use crate::rules::cost::PathCostFn;
use crate::rules::pareto::{CostLabel, GrowthLabel, MeasuredLabel, PathLabel, ReductionEdge};
use crate::rules::registry::{
    AggregateReduceFn, EdgeCapabilities, ReduceFn, ReductionEntry, ReductionOverhead,
};
use crate::rules::search::SearchTracker;
use crate::rules::traits::{DynAggregateReductionResult, DynReductionResult};
use crate::rules::{LimitReached, SearchMode, SearchOutcome};
use crate::types::ProblemSize;
use ordered_float::OrderedFloat;
use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;
use std::any::Any;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

/// A source/target pair from the reduction graph, returned by
/// [`ReductionGraph::outgoing_reductions`] and [`ReductionGraph::incoming_reductions`].
#[derive(Debug, Clone)]
pub struct ReductionEdgeInfo {
    pub source_name: &'static str,
    pub source_variant: BTreeMap<String, String>,
    pub target_name: &'static str,
    pub target_variant: BTreeMap<String, String>,
    pub overhead: ReductionOverhead,
    pub capabilities: EdgeCapabilities,
}

/// Internal edge data combining overhead and executable reduce function.
#[derive(Clone)]
pub(crate) struct ReductionEdgeData {
    pub overhead: ReductionOverhead,
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
    /// Category of the problem (e.g., "graph", "set", "optimization", "satisfiability", "specialized").
    pub(crate) category: String,
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
    /// Reduction overhead: output size as expressions of input size.
    pub(crate) overhead: Vec<OverheadFieldJson>,
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
/// - Exact and bounded-approximate Pareto search with custom cost functions
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

struct ExactParetoDfs<'a, 'b, L> {
    graph: &'a ReductionGraph,
    dst: NodeIndex,
    adjacency: &'a [Vec<(NodeIndex, EdgeIndex)>],
    front: &'b mut Vec<(ReductionPath, L)>,
    tracker: &'b mut SearchTracker,
}

impl<L: PathLabel> ExactParetoDfs<'_, '_, L> {
    fn visit(
        &mut self,
        node: NodeIndex,
        label: L,
        path: &mut Vec<NodeIndex>,
        visited: &mut [bool],
    ) {
        if node == self.dst {
            let candidate = (self.graph.node_path_to_reduction_path(path), label);
            self.graph
                .insert_terminal_candidate(self.front, candidate, self.tracker);
            return;
        }

        let edge_count = self.adjacency[node.index()].len();
        if edge_count == 0 {
            return;
        }
        self.tracker.record_expanded();

        for edge_pos in 0..edge_count {
            let (target, edge_idx) = self.adjacency[node.index()][edge_pos];
            if visited[target.index()] {
                continue;
            }
            let weight = &self.graph.graph[edge_idx];
            let target_node = &self.graph.nodes[self.graph.graph[target]];
            let edge = ReductionEdge {
                overhead: &weight.overhead,
                reduce_fn: weight.reduce_fn,
                target_name: target_node.name,
                target_variant: &target_node.variant,
            };
            let Some(next_label) = label.extend(&edge) else {
                self.tracker.record_infeasible();
                continue;
            };
            self.tracker.record_generated();
            visited[target.index()] = true;
            path.push(target);
            self.visit(target, next_label, path, visited);
            path.pop();
            visited[target.index()] = false;
        }
    }
}

impl ReductionGraph {
    fn measured_path_from_label(
        path: ReductionPath,
        label: MeasuredLabel<'_>,
    ) -> Option<MeasuredPath> {
        let steps = label.chain();
        if steps.is_empty() {
            return None;
        }
        Some(MeasuredPath {
            path,
            size: label.measured_size().clone(),
            steps,
        })
    }

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

            let overhead = entry.overhead();
            if graph.find_edge(src_idx, dst_idx).is_none() {
                graph.add_edge(
                    src_idx,
                    dst_idx,
                    ReductionEdgeData {
                        overhead,
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

    /// Find the cheapest path between two specific problem variants.
    ///
    /// Searches the variant-level graph from the exact source variant node to the exact
    /// target variant node under the caller's explicit completeness policy. `Exact`
    /// covers every elementary path permitted by the formula label semantics;
    /// `Approximate` returns a valid best-so-far path and records every reached limit.
    /// Formula-search exactness does not imply that a predicted size equals a later
    /// constructed instance size.
    #[allow(clippy::too_many_arguments)]
    pub fn find_cheapest_path<C: PathCostFn>(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        input_size: &ProblemSize,
        cost_fn: &C,
        search_mode: SearchMode,
    ) -> SearchOutcome<Option<ReductionPath>> {
        self.find_cheapest_path_mode(
            source,
            source_variant,
            target,
            target_variant,
            ReductionMode::Witness,
            input_size,
            cost_fn,
            search_mode,
        )
    }

    /// Find the cheapest path between two specific problem variants while
    /// requiring a specific edge capability.
    ///
    /// Runs the generic [multi-label elementary-path search](Self::pareto_search) with a
    /// [`CostLabel`] domain. Returns the front's best element under the deterministic
    /// tie-break (smallest cost, then fewest hops, then lexicographic node names).
    /// `Exact` covers the full elementary-path space for those formula semantics;
    /// `Approximate` may return a best-so-far result with structured limit reasons.
    #[allow(clippy::too_many_arguments)]
    pub fn find_cheapest_path_mode<C: PathCostFn>(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        mode: ReductionMode,
        input_size: &ProblemSize,
        cost_fn: &C,
        search_mode: SearchMode,
    ) -> SearchOutcome<Option<ReductionPath>> {
        let mut tracker = SearchTracker::new(&search_mode);
        let (Some(src), Some(dst)) = (
            self.lookup_node(source, source_variant),
            self.lookup_node(target, target_variant),
        ) else {
            return tracker.finish(None);
        };
        let initial = CostLabel::new(input_size.clone(), cost_fn);
        let mut front = self.pareto_search(src, dst, mode, initial, &mut tracker);
        tracker.finish(self.pick_best_front(&mut front).map(|(path, _)| path))
    }

    /// Generic multi-label elementary-path search from `src` to `dst`.
    ///
    /// Intermediate pruning and coalescing are forbidden because arbitrary reduction
    /// overheads are not guaranteed to be isotone and labels do not identify complete
    /// constructed problems. Pareto dominance is applied only to completed destination
    /// labels. Exact search has no configurable truncation; approximate limits are
    /// explicit and reported.
    ///
    /// Returns the Pareto front at `dst`: `(path, label)` pairs, deterministically
    /// ordered by (cost, hops, node-name path).
    pub(crate) fn pareto_search<L: PathLabel>(
        &self,
        src: NodeIndex,
        dst: NodeIndex,
        mode: ReductionMode,
        initial: L,
        tracker: &mut SearchTracker,
    ) -> Vec<(ReductionPath, L)> {
        if tracker.is_exact_mode() {
            return self.pareto_search_exact(src, dst, mode, initial, tracker);
        }

        struct Entry<L> {
            node: NodeIndex,
            label: Option<L>,
            pred: Option<usize>,
            hops: usize,
            visited: Vec<bool>,
        }

        let mut arena: Vec<Entry<L>> = Vec::new();
        let mut bags: HashMap<NodeIndex, Vec<usize>> = HashMap::new();
        let mut frontier: BinaryHeap<Reverse<(OrderedFloat<f64>, usize)>> = BinaryHeap::new();
        let mut adjacency: HashMap<NodeIndex, Vec<(NodeIndex, EdgeIndex)>> = HashMap::new();

        tracker.record_generated();
        if tracker.label_limit() == Some(0) {
            tracker.reach(LimitReached::LabelsPerNodeLimit);
            return Vec::new();
        }

        let mut initial_visited = vec![false; self.graph.node_count()];
        initial_visited[src.index()] = true;
        arena.push(Entry {
            node: src,
            label: Some(initial.clone()),
            pred: None,
            hops: 0,
            visited: initial_visited,
        });
        bags.entry(src).or_default().push(0);
        tracker.observe_bag(1);
        frontier.push(Reverse((OrderedFloat(initial.cost()), 0)));

        let node_path = |arena: &Vec<Entry<L>>, idx: usize| -> Vec<NodeIndex> {
            let mut nodes = Vec::new();
            let mut cur = Some(idx);
            while let Some(i) = cur {
                nodes.push(arena[i].node);
                cur = arena[i].pred;
            }
            nodes.reverse();
            nodes
        };
        while let Some(Reverse((_cost, idx))) = frontier.pop() {
            let node = arena[idx].node;
            if arena[idx].label.is_none() {
                continue;
            }
            if node == dst {
                continue;
            }

            let edges = adjacency
                .entry(node)
                .or_insert_with(|| self.ordered_outgoing_edges(node, mode));
            if edges.is_empty() {
                continue;
            }
            if tracker.timed_out() || tracker.expansion_limited() {
                break;
            }
            if tracker.hop_limited(arena[idx].hops) {
                continue;
            }
            tracker.record_expanded();

            let Some(cur_label) = arena[idx].label.clone() else {
                continue;
            };
            let cur_visited = arena[idx].visited.clone();

            let hops = arena[idx].hops;
            for &(target, edge_idx) in edges.iter() {
                if cur_visited[target.index()] {
                    continue;
                }
                let weight = &self.graph[edge_idx];
                let target_node = &self.nodes[self.graph[target]];
                let redge = ReductionEdge {
                    overhead: &weight.overhead,
                    reduce_fn: weight.reduce_fn,
                    target_name: target_node.name,
                    target_variant: &target_node.variant,
                };
                let Some(new_label) = cur_label.extend(&redge) else {
                    tracker.record_infeasible();
                    continue;
                };
                tracker.record_generated();
                let new_cost = new_label.cost();
                let mut new_visited = cur_visited.clone();
                new_visited[target.index()] = true;

                let nidx = arena.len();
                arena.push(Entry {
                    node: target,
                    label: Some(new_label),
                    pred: Some(idx),
                    hops: hops + 1,
                    visited: new_visited,
                });
                bags.entry(target).or_default().push(nidx);
                frontier.push(Reverse((OrderedFloat(new_cost), nidx)));
                tracker.observe_bag(bags[&target].len());

                if let Some(limit) = tracker.label_limit() {
                    if bags[&target].len() <= limit {
                        continue;
                    }
                    tracker.reach(LimitReached::LabelsPerNodeLimit);
                    let mut entries = bags[&target].clone();
                    let entry_cost = |i: usize| {
                        arena[i]
                            .label
                            .as_ref()
                            .map(|l| l.cost())
                            .unwrap_or(f64::INFINITY)
                    };
                    entries.sort_by(|&a, &b| {
                        entry_cost(a)
                            .partial_cmp(&entry_cost(b))
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then_with(|| arena[a].hops.cmp(&arena[b].hops))
                            .then_with(|| {
                                self.path_order_key(&node_path(&arena, a))
                                    .cmp(&self.path_order_key(&node_path(&arena, b)))
                            })
                    });
                    for &j in &entries[limit..] {
                        arena[j].label = None;
                    }
                    entries.truncate(limit);
                    bags.insert(target, entries);
                }
            }
        }

        // Collect every retained destination label. Strict dominance is safe here because
        // completed labels have no future extension whose non-monotonicity could reverse
        // the order.
        let mut completed: Vec<(ReductionPath, L)> = bags
            .get(&dst)
            .map(|b| b.as_slice())
            .unwrap_or(&[])
            .iter()
            .map(|&idx| {
                let node_path = node_path(&arena, idx);
                (
                    self.node_path_to_reduction_path(&node_path),
                    // Live dst bag members are always `Some` (bag-member invariant).
                    arena[idx]
                        .label
                        .clone()
                        .expect("live dst bag member has a label"),
                )
            })
            .collect();

        completed.sort_by(Self::compare_front_entries);

        let mut front = Vec::new();
        for candidate in completed {
            self.insert_terminal_candidate(&mut front, candidate, tracker);
        }
        front.sort_by(Self::compare_front_entries);
        front
    }

    /// Exact elementary-path traversal with working memory proportional to path depth.
    ///
    /// No intermediate state is compared with another. A single visited set and path are
    /// mutated during deterministic DFS backtracking; only terminal Pareto labels remain
    /// live after their branch returns.
    fn pareto_search_exact<L: PathLabel>(
        &self,
        src: NodeIndex,
        dst: NodeIndex,
        mode: ReductionMode,
        initial: L,
        tracker: &mut SearchTracker,
    ) -> Vec<(ReductionPath, L)> {
        let mut adjacency = vec![Vec::new(); self.graph.node_count()];
        for node in self.graph.node_indices() {
            adjacency[node.index()] = self.ordered_outgoing_edges(node, mode);
        }

        tracker.record_generated();
        tracker.observe_bag(1);
        let mut path = vec![src];
        let mut visited = vec![false; self.graph.node_count()];
        visited[src.index()] = true;
        let mut front = Vec::new();
        ExactParetoDfs {
            graph: self,
            dst,
            adjacency: &adjacency,
            front: &mut front,
            tracker,
        }
        .visit(src, initial, &mut path, &mut visited);
        front.sort_by(Self::compare_front_entries);
        front
    }

    fn compare_front_entries<L: PathLabel>(
        a: &(ReductionPath, L),
        b: &(ReductionPath, L),
    ) -> std::cmp::Ordering {
        a.1.cost()
            .partial_cmp(&b.1.cost())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.len().cmp(&b.0.len()))
            .then_with(|| a.0.type_names().cmp(&b.0.type_names()))
    }

    fn insert_terminal_candidate<L: PathLabel>(
        &self,
        front: &mut Vec<(ReductionPath, L)>,
        candidate: (ReductionPath, L),
        tracker: &mut SearchTracker,
    ) {
        let precedes = |a: &(ReductionPath, L), b: &(ReductionPath, L)| {
            a.1.final_dominates(&b.1)
                && (!b.1.final_dominates(&a.1)
                    || Self::compare_front_entries(a, b) != std::cmp::Ordering::Greater)
        };
        if front.iter().any(|existing| precedes(existing, &candidate)) {
            tracker.record_dominated(1);
            return;
        }

        let before = front.len();
        front.retain(|existing| !precedes(&candidate, existing));
        tracker.record_dominated(before - front.len());
        front.push(candidate);
    }

    /// Name-keyed entry to [`pareto_search`](Self::pareto_search): resolves the source
    /// and target variant nodes, then runs the generic search. Returns an empty vector
    /// if either endpoint is not registered. Test-only: drives the generic kernel with a
    /// custom label on a hand-built graph.
    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn pareto_search_by_name<L: PathLabel>(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        mode: ReductionMode,
        initial: L,
        search_mode: SearchMode,
    ) -> SearchOutcome<Vec<(ReductionPath, L)>> {
        let mut tracker = SearchTracker::new(&search_mode);
        let (Some(src), Some(dst)) = (
            self.lookup_node(source, source_variant),
            self.lookup_node(target, target_variant),
        ) else {
            return tracker.finish(vec![]);
        };
        let front = self.pareto_search(src, dst, mode, initial, &mut tracker);
        tracker.finish(front)
    }

    /// Pick the best element of a Pareto front under the deterministic tie-break
    /// (smallest cost, then fewest hops, then lexicographic node names). The front is
    /// already sorted by [`pareto_search`](Self::pareto_search), so this returns the
    /// first element.
    fn pick_best_front<L: PathLabel>(
        &self,
        front: &mut Vec<(ReductionPath, L)>,
    ) -> Option<(ReductionPath, L)> {
        if front.is_empty() {
            None
        } else {
            Some(front.remove(0))
        }
    }

    /// Deterministic total-order key for a node-index path.
    ///
    /// Reproduces the `Name/val1/val2` slash signature the CLI historically used
    /// as an ordering tiebreak, but computed purely from library node data so the
    /// ordering lives in exactly one place. Within a fixed path length the length
    /// contributes nothing, so sorting a same-length level by this key yields a
    /// reproducible, build-independent order (BTreeMap variant iteration is
    /// deterministic). Distinct simple paths produce distinct keys because each
    /// node is a unique `(name, variant)` pair.
    fn path_order_key(&self, node_path: &[NodeIndex]) -> String {
        let mut key = String::new();
        for (i, &idx) in node_path.iter().enumerate() {
            if i > 0 {
                key.push('>');
            }
            let node = &self.nodes[self.graph[idx]];
            key.push_str(node.name);
            for v in node.variant.values() {
                key.push('/');
                key.push_str(v);
            }
        }
        key
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

    /// Enumerate witness-capable simple paths from `src` to any target, executing each
    /// reduction as it is reached and retaining the measured-smallest completed target.
    ///
    /// This is deliberately separate from [`pareto_search`](Self::pareto_search): no
    /// dominance relation, hop cap, bag cap, or scalar branch-and-bound is valid for a
    /// structure-dependent concrete instance. Repeated nodes are excluded because this
    /// API searches graph paths (not unbounded walks); that is the sole structural
    /// termination condition.
    fn measured_best_simple_path<'a>(
        &self,
        src: NodeIndex,
        targets: &HashSet<NodeIndex>,
        mode: ReductionMode,
        initial: MeasuredLabel<'a>,
        tracker: &mut SearchTracker,
    ) -> Option<(ReductionPath, MeasuredLabel<'a>)> {
        tracker.record_generated();
        if tracker.label_limit() == Some(0) {
            tracker.reach(LimitReached::LabelsPerNodeLimit);
            return None;
        }
        let mut stack = vec![(src, vec![src], initial)];
        let mut retained_per_node: HashMap<NodeIndex, usize> = HashMap::new();
        retained_per_node.insert(src, 1);
        tracker.observe_bag(1);
        let mut adjacency: HashMap<NodeIndex, Vec<(NodeIndex, EdgeIndex)>> = HashMap::new();
        let mut best: Option<(Vec<NodeIndex>, MeasuredLabel<'a>)> = None;

        while let Some((node, node_path, label)) = stack.pop() {
            if let Some(retained) = retained_per_node.get_mut(&node) {
                *retained -= 1;
            }
            if targets.contains(&node) {
                let candidate_key = (
                    label.measured_size().total(),
                    node_path.len(),
                    self.path_order_key(&node_path),
                );
                let is_better = best.as_ref().is_none_or(|(best_path, best_label)| {
                    let best_key = (
                        best_label.measured_size().total(),
                        best_path.len(),
                        self.path_order_key(best_path),
                    );
                    candidate_key < best_key
                });
                if is_better {
                    best = Some((node_path, label));
                }
                continue;
            }

            let edges = adjacency
                .entry(node)
                .or_insert_with(|| self.ordered_outgoing_edges(node, mode));
            if edges.is_empty() {
                continue;
            }
            if tracker.timed_out() || tracker.expansion_limited() {
                break;
            }
            if tracker.hop_limited(node_path.len() - 1) {
                continue;
            }
            tracker.record_expanded();

            // Reverse push order so DFS visits the deterministic ascending edge order.
            for &(target, edge_idx) in edges.iter().rev() {
                if node_path.contains(&target) {
                    continue;
                }
                let weight = &self.graph[edge_idx];
                let target_node = &self.nodes[self.graph[target]];
                let edge = ReductionEdge {
                    overhead: &weight.overhead,
                    reduce_fn: weight.reduce_fn,
                    target_name: target_node.name,
                    target_variant: &target_node.variant,
                };
                let Some(next_label) = label.extend(&edge) else {
                    tracker.record_infeasible();
                    continue;
                };
                tracker.record_generated();
                if tracker.label_limit().is_some_and(|limit| {
                    retained_per_node.get(&target).copied().unwrap_or(0) >= limit
                }) {
                    tracker.reach(LimitReached::LabelsPerNodeLimit);
                    continue;
                }
                let mut next_path = node_path.clone();
                next_path.push(target);
                stack.push((target, next_path, next_label));
                let retained = retained_per_node.entry(target).or_default();
                *retained += 1;
                tracker.observe_bag(*retained);
            }
        }

        best.map(|(path, label)| (self.node_path_to_reduction_path(&path), label))
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

        // Enumerate every simple path in a single DFS pass and keep only the `limit`
        // that sort smallest under the deterministic total order: fewest nodes first
        // (shortest routes), then by `path_order_key`. Taking `limit` in petgraph's raw
        // DFS discovery order (the previous approach) could drop a short route
        // discovered late while returning a long route discovered early. A single
        // bounded max-heap keyed by `(node count, order key)` retains exactly those
        // `limit` paths — push each candidate, and once over capacity pop the current
        // largest — so ordering and the truncated subset are reproducible and
        // build-independent with O(limit) memory, however many paths the graph holds.
        // (`limit == 0` falls out naturally: every push is immediately popped.)
        let max_intermediate =
            max_intermediate_nodes.unwrap_or_else(|| self.graph.node_count().saturating_sub(2));

        let mut heap: BinaryHeap<(usize, String, Vec<NodeIndex>)> = BinaryHeap::new();
        for p in all_simple_paths::<Vec<NodeIndex>, _, std::hash::RandomState>(
            &self.graph,
            src,
            dst,
            0,
            Some(max_intermediate),
        ) {
            if !self.node_path_supports_mode(&p, mode) {
                continue;
            }
            let key = self.path_order_key(&p);
            heap.push((p.len(), key, p));
            if heap.len() > limit {
                heap.pop();
            }
        }

        // `into_sorted_vec` yields ascending `(node count, order key)` order.
        heap.into_sorted_vec()
            .into_iter()
            .map(|(_, _, p)| self.node_path_to_reduction_path(&p))
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

    /// Get the per-edge overhead expressions along a reduction path.
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
    /// Returns a single `ReductionOverhead` whose expressions map from the
    /// source problem's size variables directly to the final target's size variables.
    pub fn compose_path_overhead(&self, path: &ReductionPath) -> ReductionOverhead {
        self.path_overheads(path)
            .into_iter()
            .reduce(|acc, oh| acc.compose(&oh))
            .unwrap_or_default()
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
                    overhead: self.graph[e.id()].overhead.clone(),
                    capabilities: self.graph[e.id()].capabilities(),
                }
            })
            .collect()
    }

    /// Get the problem size field names for a problem type.
    ///
    /// Derives size fields from the overhead expressions of reduction entries
    /// where this problem appears as source or target. When the problem is a
    /// source, its size fields are the input variables referenced in the overhead
    /// expressions. When it's a target, its size fields are the output field names.
    pub fn size_field_names(&self, name: &str) -> Vec<&'static str> {
        let mut fields: std::collections::HashSet<&'static str> =
            crate::registry::declared_size_fields(name)
                .into_iter()
                .collect();
        for entry in inventory::iter::<ReductionEntry> {
            if entry.source_name == name {
                // Source's size fields are the input variables of the overhead.
                fields.extend(entry.overhead().input_variable_names());
            }
            if entry.target_name == name {
                // Target's size fields are the output field names.
                let overhead = entry.overhead();
                fields.extend(overhead.output_size.iter().map(|(name, _)| *name));
            }
        }
        let mut result: Vec<&'static str> = fields.into_iter().collect();
        result.sort_unstable();
        result
    }

    /// Evaluate the cumulative output size along a reduction path.
    ///
    /// Walks the path from start to end, applying each edge's overhead
    /// expressions to transform the problem size at each step.
    /// Returns `None` if any edge in the path cannot be found.
    pub fn evaluate_path_overhead(
        &self,
        path: &ReductionPath,
        input_size: &ProblemSize,
    ) -> Option<ProblemSize> {
        let mut current_size = input_size.clone();
        for pair in path.steps.windows(2) {
            let src = self.lookup_node(&pair[0].name, &pair[0].variant)?;
            let dst = self.lookup_node(&pair[1].name, &pair[1].variant)?;
            let edge_idx = self.graph.find_edge(src, dst)?;
            let edge = &self.graph[edge_idx];
            current_size = edge.overhead.evaluate_output_size(&current_size);
        }
        Some(current_size)
    }

    /// Compute the source problem's size from a type-erased instance.
    ///
    /// Iterates over all registered reduction entries with an exact source name and
    /// variant match, then merges their `source_size_fn` results to capture all size fields.
    /// Different entries may reference different getter methods (e.g., one uses
    /// `num_vertices` while another also uses `num_edges`).
    pub fn compute_source_size(
        name: &str,
        variant: &BTreeMap<String, String>,
        instance: &dyn Any,
    ) -> ProblemSize {
        let mut merged: Vec<(String, usize)> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        for entry in inventory::iter::<ReductionEntry> {
            if entry.source_name != name {
                continue;
            }
            let entry_variant = entry.source_variant();
            let variant_matches = entry_variant.len() == variant.len()
                && entry_variant.iter().all(|(key, value)| {
                    let value = if *key == "graph" && value.is_empty() {
                        "SimpleGraph"
                    } else {
                        value
                    };
                    variant.get(*key).is_some_and(|expected| expected == value)
                });
            if variant_matches {
                for (k, v) in (entry.source_size_fn)(instance).components {
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
                    overhead: self.graph[e.id()].overhead.clone(),
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
            let overhead = &edge_ref.weight().overhead;
            let capabilities = edge_ref.weight().capabilities();

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

    /// Find the matching `ReductionEntry` for a (source_name, target_name) pair
    /// given exact source and target variants.
    ///
    /// Returns `Some(MatchedEntry)` only when both the source and target variants
    /// match exactly. No fallback is attempted — callers that need fuzzy matching
    /// should resolve variants before calling this method.
    pub fn find_best_entry(
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
                    overhead: entry.overhead(),
                });
            }
        }

        None
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

/// A concrete reduction path selected by the measured Pareto search.
///
/// Holds the winning [`ReductionPath`], its **measured** final target
/// [`ProblemSize`], and the already-constructed reduction chain so downstream
/// solve/witness extraction reuses it without re-executing the reductions.
pub struct MeasuredPath {
    /// The variant-level path.
    pub path: ReductionPath,
    /// Measured size of the final target problem.
    pub size: ProblemSize,
    /// The executed reduction steps (one per hop), shared via `Rc`.
    steps: Vec<Rc<dyn DynReductionResult>>,
}

impl MeasuredPath {
    /// Get the final target problem as a type-erased reference.
    pub fn target_problem_any(&self) -> &dyn Any {
        self.steps
            .last()
            .expect("MeasuredPath has no steps")
            .target_problem_any()
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
    /// Find the reduction path with the smallest **measured** final target size.
    ///
    /// Unlike [`find_cheapest_path_mode`](Self::find_cheapest_path_mode), which ranks
    /// paths by overhead *formulas* (scaling upper bounds that can be arbitrarily loose
    /// on structure-dependent constructions), this runs the [`MeasuredLabel`] domain:
    /// it *actually executes* each reduction on `source_instance` and measures the real
    /// constructed target size. Asymptotic overhead formulas are not treated as concrete
    /// bounds and do not prune candidates. See design doc M3/F3b.
    ///
    /// `budget` is the hard total-size limit (sum of `ProblemSize` components); use
    /// [`DEFAULT_SIZE_BUDGET`](crate::rules::DEFAULT_SIZE_BUDGET) for the default.
    /// Exact search enumerates witness-capable simple paths without dominance pruning or
    /// branch-and-bound. Approximate search applies only the limits explicitly carried by
    /// `search_mode`. Neither size vectors nor serialized state equality discard a route.
    /// The post-construction measured-budget guard still applies.
    /// Because the target must be built before it can be measured, the budget is not an
    /// anti-OOM guarantee.
    ///
    /// Returns `None` if no in-budget witness-capable path exists (or `source == target`).
    #[allow(clippy::too_many_arguments)]
    pub fn find_measured_best_path(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        mode: ReductionMode,
        source_instance: &dyn Any,
        budget: usize,
        search_mode: SearchMode,
    ) -> SearchOutcome<Option<MeasuredPath>> {
        let mut tracker = SearchTracker::new(&search_mode);
        let (Some(src), Some(dst)) = (
            self.lookup_node(source, source_variant),
            self.lookup_node(target, target_variant),
        ) else {
            return tracker.finish(None);
        };
        if src == dst {
            return tracker.finish(None);
        }
        let source_size = Self::compute_source_size(source, source_variant, source_instance);
        let initial = MeasuredLabel::new(source_instance, source_size, budget);
        let targets = HashSet::from([dst]);
        let result = self
            .measured_best_simple_path(src, &targets, mode, initial, &mut tracker)
            .and_then(|(path, label)| Self::measured_path_from_label(path, label));
        tracker.finish(result)
    }

    /// Compute the **asymptotic Pareto front** of reduction paths from `source` to
    /// `target` — the instance-free path search (design doc M3/F3a).
    ///
    /// Runs the generic [multi-label elementary-path search](Self::pareto_search) with the
    /// [`GrowthLabel`] domain: no concrete instance is needed, and each returned path
    /// carries its composed Big-O per target size field (in the source problem's size
    /// variables), read off the returned label. Because asymptotic growth over several
    /// size variables is a *partial* order, the answer is a front: possibly several
    /// mutually incomparable optimal paths (one better in one size field, another in a
    /// different one). Paths whose composed growth is [`Growth::Unknown`] (nonlinear
    /// exponent, factorial) are still returned, with those fields marked `Unknown` —
    /// never a fabricated bound.
    ///
    /// The terminal front reports **one representative path per distinct growth vector**:
    /// the asymptotic front is a Pareto set over *growth vectors*, not routes. Many
    /// syntactically different reduction chains compose to the exact same Big-O per size
    /// field (e.g. dozens of `MinimumVertexCover → … → ILP` routes all yield
    /// `num_constraints = O(num_edges), num_vars = O(num_vertices)`); reporting each
    /// route would drown the genuinely distinct trade-offs the user cares about.
    /// So terminal equality filtering keeps the deterministic best per group: fewest
    /// hops, then lexicographic node-name path. Equality is purely by the growth vector,
    /// so two paths that
    /// reach *different* target variants (e.g. `ILP/bool` vs `ILP/i32`) with the same
    /// composed Big-O collapse to a single representative — the endpoint variant is not
    /// part of the asymptotic identity.
    ///
    /// The front is ordered deterministically by (hops, lexicographic node names), so
    /// the output is byte-identical across runs and platforms. Returns an empty vector
    /// if either endpoint is unregistered or no path exists. `Exact` covers every
    /// elementary path under the symbolic growth domain; `Approximate` may return a
    /// best-so-far front and reports any reached limits. Symbolic exactness is not a
    /// statement about concrete constructed target sizes.
    pub fn asymptotic_front(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        target_variant: &BTreeMap<String, String>,
        mode: ReductionMode,
        search_mode: SearchMode,
    ) -> SearchOutcome<Vec<(ReductionPath, GrowthLabel)>> {
        let mut tracker = SearchTracker::new(&search_mode);
        let (Some(src), Some(dst)) = (
            self.lookup_node(source, source_variant),
            self.lookup_node(target, target_variant),
        ) else {
            return tracker.finish(vec![]);
        };
        let source_fields = self.size_field_names(source);
        let initial = GrowthLabel::source(&source_fields);
        let mut front = self.pareto_search(src, dst, mode, initial, &mut tracker);
        // Order per the public contract: (hops, lexicographic node names). The kernel's
        // own ordering leads with `cost()`, which is only an agenda heuristic.
        front.sort_by(|a, b| {
            a.0.len()
                .cmp(&b.0.len())
                .then_with(|| a.0.type_names().cmp(&b.0.type_names()))
        });
        tracker.finish(front)
    }

    /// Find the measured-smallest path from `source` to **any** variant of the target
    /// problem name `target`.
    ///
    /// Performs one traversal whose terminal set contains every target variant, so limits,
    /// statistics, and constructed prefixes are shared across the whole request. Returns
    /// the overall measured-smallest result with a deterministic tie-break by measured
    /// total size, hops, and node-name path. Exactness is relative to in-budget elementary
    /// paths: the concrete budget is checked after each intermediate is constructed and
    /// is not an allocation-safety guarantee.
    #[allow(clippy::too_many_arguments)]
    pub fn find_measured_best_path_to_name(
        &self,
        source: &str,
        source_variant: &BTreeMap<String, String>,
        target: &str,
        mode: ReductionMode,
        source_instance: &dyn Any,
        budget: usize,
        search_mode: SearchMode,
    ) -> SearchOutcome<Option<MeasuredPath>> {
        let mut tracker = SearchTracker::new(&search_mode);
        let Some(src) = self.lookup_node(source, source_variant) else {
            return tracker.finish(None);
        };
        let targets: HashSet<NodeIndex> = self
            .variants_for(target)
            .into_iter()
            .filter_map(|variant| self.lookup_node(target, &variant))
            .filter(|target_node| *target_node != src)
            .collect();
        if targets.is_empty() {
            return tracker.finish(None);
        }

        let source_size = Self::compute_source_size(source, source_variant, source_instance);
        let initial = MeasuredLabel::new(source_instance, source_size, budget);
        let result = self
            .measured_best_simple_path(src, &targets, mode, initial, &mut tracker)
            .and_then(|(path, label)| Self::measured_path_from_label(path, label));
        tracker.finish(result)
    }
}

#[cfg(test)]
impl ReductionGraph {
    /// Build a bare reduction graph from an explicit node/edge list (test-only).
    ///
    /// Nodes carry the empty variant and empty complexity; each edge carries a
    /// [`ReductionEdgeData`]. This lets tests exercise the generic Pareto search on a
    /// hand-built topology (e.g. the negative-control diamond) without depending on the
    /// registered inventory.
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
#[path = "../unit_tests/rules/pareto.rs"]
mod pareto_tests;

#[cfg(test)]
#[path = "../unit_tests/rules/reduction_path_parity.rs"]
mod reduction_path_parity_tests;

#[cfg(all(test, feature = "ilp-solver"))]
#[path = "../unit_tests/rules/maximumindependentset_ilp.rs"]
mod maximumindependentset_ilp_path_tests;

#[cfg(all(test, feature = "ilp-solver"))]
#[path = "../unit_tests/rules/minimumvertexcover_ilp.rs"]
mod minimumvertexcover_ilp_path_tests;

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_qubo.rs"]
mod maximumindependentset_qubo_path_tests;

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_qubo.rs"]
mod minimumvertexcover_qubo_path_tests;
