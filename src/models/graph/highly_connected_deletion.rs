//! Highly Connected Deletion problem implementation.
//!
//! Given a simple undirected graph `G = (V, E)`, find a minimum-cardinality
//! edge set `F ⊆ E` such that every connected component of `G - F` is either:
//!
//! - an isolated vertex (singleton component), or
//! - a highly connected graph on at least `3` vertices, i.e. with edge
//!   connectivity `λ(H) > |V(H)| / 2` (strict inequality).
//!
//! Components of size `2` (isolated edges) are explicitly *not* valid clusters.
//!
//! Reference:
//! - Hüffner, Komusiewicz, Liebtrau, Niedermeier, "Partitioning Biological
//!   Networks into Highly Connected Clusters with Maximum Edge Coverage",
//!   IEEE/ACM TCBB 11(3):455–467, 2014.
//! - Hartuv, Shamir, "A clustering algorithm based on graph connectivity",
//!   Information Processing Letters 76(4–6):175–181, 2000.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

inventory::submit! {
    ProblemSchemaEntry {
        name: "HighlyConnectedDeletion",
        display_name: "Highly Connected Deletion",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Minimum number of edge deletions so every component is an isolated vertex or a highly connected graph on >=3 vertices",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "HighlyConnectedDeletion",
        fields: &["num_vertices", "num_edges"],
    }
}

/// The Highly Connected Deletion problem.
///
/// Given a simple undirected graph `G = (V, E)`, find a minimum-cardinality
/// edge set `F ⊆ E` such that every connected component of `G - F` is either
/// an isolated vertex or a highly connected graph on at least `3` vertices.
///
/// A graph `H` is *highly connected* if its edge connectivity `λ(H)` is
/// strictly greater than `|V(H)| / 2`. Components of size `2` (isolated edges)
/// are never valid clusters.
///
/// # Type Parameters
///
/// * `G` - Graph type (currently only `SimpleGraph`).
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::HighlyConnectedDeletion;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{BruteForce, Problem, Solver};
/// use problemreductions::types::Min;
///
/// // Triangle on {0,1,2} with leaf vertex 3 attached to 2.
/// let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]);
/// let problem = HighlyConnectedDeletion::new(graph);
///
/// // Optimal: delete only the leaf edge (2,3) → K3 + isolated {3}.
/// assert_eq!(BruteForce::new().solve(&problem).unwrap(), Min(Some(1)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct HighlyConnectedDeletion<G> {
    /// The underlying graph.
    graph: G,
}

impl<G: Graph> HighlyConnectedDeletion<G> {
    /// Create a new Highly Connected Deletion instance from a graph.
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check whether a deletion configuration leaves every component as either
    /// an isolated vertex or a highly connected graph on at least `3` vertices.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_feasible_deletion(&self.graph, config)
    }
}

impl<G> Problem for HighlyConnectedDeletion<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "HighlyConnectedDeletion";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if !is_feasible_deletion(&self.graph, config) {
                return Ok(Min(None));
            }
            let deleted =
                i64::try_from(config.iter().filter(|&&x| x == 1).count()).map_err(|_| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "converting deleted-edge count to i64".into(),
                    )
                })?;
            Min(Some(deleted))
        })
    }
}

/// Decide feasibility of a deletion configuration.
///
/// `config[e] = 1` means edge `e` (in `graph.edges()` order) is deleted.
/// The remaining graph `G - F` must have every connected component be either
/// a singleton or a highly connected graph on at least `3` vertices.
fn is_feasible_deletion<G: Graph>(graph: &G, config: &[usize]) -> bool {
    let n = graph.num_vertices();
    let edges = graph.edges();
    if config.len() != edges.len() {
        return false;
    }

    // Build adjacency from the surviving edges only.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        if config.get(i).copied().unwrap_or(0) == 0 {
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    // Find connected components by BFS.
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }
        let mut component: Vec<usize> = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);
        visited[start] = true;
        while let Some(u) = queue.pop_front() {
            component.push(u);
            for &w in &adj[u] {
                if !visited[w] {
                    visited[w] = true;
                    queue.push_back(w);
                }
            }
        }
        let size = component.len();
        if size == 1 {
            continue; // isolated vertex: allowed
        }
        if size == 2 {
            return false; // 2-vertex component never valid
        }
        // size >= 3: must be highly connected.
        let lambda = edge_connectivity(&component, &adj);
        // Strict inequality: λ > size/2 (avoid float by using 2*λ > size).
        if 2 * lambda <= size {
            return false;
        }
    }
    true
}

/// Compute the edge connectivity `λ(H)` of the induced subgraph on
/// `vertices` using the surviving-edge adjacency list `adj`.
///
/// `λ(H) = min over distinct s, t in V(H) of max-flow(s -> t)` with unit edge
/// capacities. We fix one source `s` (the first vertex in the component) and
/// iterate over every other vertex `t`; by symmetry this suffices because the
/// minimum cut separating *any* pair must also separate `s` from one of the
/// resulting sides.
///
/// For each `(s, t)` pair we run Edmonds–Karp on the directed expansion of the
/// induced subgraph (each undirected edge becomes two arcs of capacity 1).
/// Components are small in tests so this runs well under the per-test budget.
fn edge_connectivity(vertices: &[usize], adj: &[Vec<usize>]) -> usize {
    let size = vertices.len();
    if size <= 1 {
        return 0;
    }
    // Index vertices locally 0..size for compact tables.
    let mut local: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::with_capacity(size);
    for (i, &v) in vertices.iter().enumerate() {
        local.insert(v, i);
    }

    // Build directed-arc lists with residual capacities for Edmonds–Karp.
    // Arc layout: arcs[2k] is forward, arcs[2k+1] is reverse for the k-th
    // undirected edge. `head[a]` is the arc's destination; `cap[a]` is its
    // current residual capacity.
    let in_component: HashSet<usize> = vertices.iter().copied().collect();
    let mut head: Vec<usize> = Vec::new();
    let mut cap: Vec<u8> = Vec::new();
    let mut out: Vec<Vec<usize>> = vec![Vec::new(); size];

    let mut seen_edges: HashSet<(usize, usize)> = HashSet::new();
    for &u in vertices {
        let lu = local[&u];
        for &v in &adj[u] {
            if !in_component.contains(&v) {
                continue;
            }
            let key = if u < v { (u, v) } else { (v, u) };
            if !seen_edges.insert(key) {
                continue;
            }
            let lv = local[&v];
            // Forward arc u -> v.
            let a = head.len();
            head.push(lv);
            cap.push(1);
            // Reverse arc v -> u.
            head.push(lu);
            cap.push(1);
            out[lu].push(a);
            out[lv].push(a + 1);
        }
    }

    let mut best = usize::MAX;
    let s = 0;
    for t in 1..size {
        // Reset residual capacities for each (s, t) pair.
        for c in cap.iter_mut() {
            *c = 1;
        }
        let mut flow = 0usize;
        loop {
            // BFS to find an augmenting path with positive residual capacity.
            let mut parent_arc: Vec<Option<usize>> = vec![None; size];
            let mut visited = vec![false; size];
            visited[s] = true;
            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(s);
            while let Some(u) = queue.pop_front() {
                if u == t {
                    break;
                }
                for &a in &out[u] {
                    let v = head[a];
                    if !visited[v] && cap[a] > 0 {
                        visited[v] = true;
                        parent_arc[v] = Some(a);
                        queue.push_back(v);
                    }
                }
            }
            if !visited[t] {
                break;
            }
            // Augment by 1 (unit capacities).
            let mut cur = t;
            while cur != s {
                let a = parent_arc[cur].expect("visited vertex has a BFS parent arc");
                cap[a] -= 1;
                cap[a ^ 1] += 1;
                // The originating endpoint is the head of the reverse arc.
                cur = head[a ^ 1];
            }
            flow += 1;
        }
        if flow < best {
            best = flow;
            if best == 0 {
                return 0;
            }
        }
    }
    if best == usize::MAX {
        0
    } else {
        best
    }
}

crate::declare_variants! {
    default HighlyConnectedDeletion<SimpleGraph> => "2^num_edges",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "highly_connected_deletion_simplegraph",
        instance: Box::new(HighlyConnectedDeletion::new(SimpleGraph::new(
            4,
            vec![(0, 1), (0, 2), (1, 2), (2, 3)],
        ))),
        // Edges in input order; deleting only edge index 3 = (2,3) leaves K3 + {3}.
        optimal_config: vec![0, 0, 0, 1],
        optimal_value: serde_json::json!(1),
    }]
}

/// Check whether a vertex subset `S` is a *feasible cluster* of `graph`.
///
/// A feasible cluster is either a singleton (`|S| = 1`) or a set of at least
/// `3` vertices whose induced subgraph `G[S]` is connected and *highly
/// connected* (edge connectivity strictly greater than `|S| / 2`).
///
/// This is the cluster-feasibility predicate used by the set-partitioning ILP
/// reduction: `x_S` is allowed exactly when `is_feasible_cluster(graph, S)`.
pub(crate) fn is_feasible_cluster<G: Graph>(graph: &G, vertices: &[usize]) -> bool {
    let size = vertices.len();
    if size == 0 {
        return false;
    }
    if size == 1 {
        return true;
    }
    if size == 2 {
        return false;
    }

    // Build induced-subgraph adjacency restricted to `vertices`.
    let n = graph.num_vertices();
    let in_subset: HashSet<usize> = vertices.iter().copied().collect();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in graph.edges() {
        if in_subset.contains(&u) && in_subset.contains(&v) {
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    // The induced subgraph must itself be connected (a single component).
    let mut visited: HashSet<usize> = HashSet::new();
    let start = vertices[0];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);
    while let Some(u) = queue.pop_front() {
        for &w in &adj[u] {
            if !visited.contains(&w) {
                visited.insert(w);
                queue.push_back(w);
            }
        }
    }
    if visited.len() != size {
        return false;
    }

    // Strict inequality: λ(G[S]) > |S| / 2, equivalently 2 * λ > |S|.
    let lambda = edge_connectivity(vertices, &adj);
    2 * lambda > size
}

/// Count the number of induced edges of `graph` whose endpoints both lie
/// inside `vertices`.
pub(crate) fn induced_edge_count<G: Graph>(graph: &G, vertices: &[usize]) -> usize {
    let in_subset: HashSet<usize> = vertices.iter().copied().collect();
    graph
        .edges()
        .into_iter()
        .filter(|(u, v)| in_subset.contains(u) && in_subset.contains(v))
        .count()
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/highly_connected_deletion.rs"]
mod tests;

#[cfg(test)]
pub(crate) fn edge_connectivity_for_tests(vertices: &[usize], adj: &[Vec<usize>]) -> usize {
    edge_connectivity(vertices, adj)
}
