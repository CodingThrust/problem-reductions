//! Rooted Tree Arrangement problem implementation.
//!
//! The Rooted Tree Arrangement problem asks whether a graph can be embedded
//! into the nodes of a rooted tree so that every graph edge lies on a single
//! root-to-leaf path and the total tree stretch is bounded.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::VariantParam;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "RootedTreeArrangement",
        display_name: "Rooted Tree Arrangement",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a rooted-tree embedding of a graph with bounded total edge stretch",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
            FieldInfo { name: "bound", type_name: "i64", description: "Upper bound K on total tree stretch" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct RootedTreeArrangement<G> {
    graph: G,
    bound: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct RootedTreeArrangementRandomSpec {
    /// Number of graph vertices.
    num_vertices: usize,
    /// Independent edge probability (default: 0.5).
    edge_prob: Option<f64>,
    /// Seed for reproducible generation.
    seed: Option<i64>,
    /// Maximum total edge stretch (defaults to a graph-size upper bound).
    bound: Option<i64>,
}

#[derive(Debug, Clone)]
struct TreeInfo {
    depth: Vec<usize>,
}

impl<G: Graph> RootedTreeArrangement<G> {
    pub fn new(graph: G, bound: i64) -> Self {
        Self { graph, bound }
    }

    pub fn graph(&self) -> &G {
        &self.graph
    }

    pub fn bound(&self) -> i64 {
        self.bound
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(matches!(self.total_edge_stretch(config)?, Some(stretch) if stretch <= self.bound))
    }

    pub fn total_edge_stretch(
        &self,
        config: &[usize],
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        let n = self.graph.num_vertices();
        if n == 0 {
            return Ok(config.is_empty().then_some(0));
        }

        let Some((parent, mapping)) = self.split_config(config) else {
            return Ok(None);
        };
        let Some(tree) = analyze_parent_array(parent) else {
            return Ok(None);
        };
        if !is_valid_permutation(mapping) {
            return Ok(None);
        }

        let mut total = 0_i64;
        for (u, v) in self.graph.edges() {
            let tree_u = mapping[u];
            let tree_v = mapping[v];
            if !are_ancestor_comparable(parent, tree_u, tree_v) {
                return Ok(None);
            }
            let stretch =
                i64::try_from(tree.depth[tree_u].abs_diff(tree.depth[tree_v])).map_err(|_| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "converting a rooted-tree edge stretch to i64".to_string(),
                    )
                })?;
            total = total.checked_add(stretch).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing rooted-tree arrangement edge stretches".to_string(),
                )
            })?;
        }

        Ok(Some(total))
    }

    fn split_config<'a>(&self, config: &'a [usize]) -> Option<(&'a [usize], &'a [usize])> {
        let n = self.graph.num_vertices();
        (config.len() == 2 * n).then(|| config.split_at(n))
    }
}

impl<G> Problem for RootedTreeArrangement<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "RootedTreeArrangement";
    type Solution = Vec<usize>;
    type Value = crate::types::Or;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        let n = self.graph.num_vertices();
        if config.len() != 2 * n {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "tree-arrangement representation length does not match the graph".into(),
            ));
        }
        Ok(crate::types::Or(self.is_valid_solution(config)?))
    }
}

impl<G> crate::solvers::BruteForceProblem for RootedTreeArrangement<G>
where
    G: Graph + VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        let n = self.graph.num_vertices();
        vec![n; 2 * n]
    }
}

fn analyze_parent_array(parent: &[usize]) -> Option<TreeInfo> {
    let n = parent.len();
    if n == 0 {
        return Some(TreeInfo { depth: vec![] });
    }

    if parent.iter().any(|&p| p >= n) {
        return None;
    }

    let roots = parent
        .iter()
        .enumerate()
        .filter_map(|(node, &p)| (node == p).then_some(node))
        .collect::<Vec<_>>();
    if roots.len() != 1 {
        return None;
    }
    let root = roots[0];

    let mut state = vec![0u8; n];
    let mut depth = vec![0usize; n];

    fn visit(
        node: usize,
        root: usize,
        parent: &[usize],
        state: &mut [u8],
        depth: &mut [usize],
    ) -> Option<usize> {
        match state[node] {
            1 => return None,
            2 => return Some(depth[node]),
            _ => {}
        }

        state[node] = 1;
        let d = if node == root {
            0
        } else {
            let next = parent[node];
            if next == node {
                return None;
            }
            visit(next, root, parent, state, depth)? + 1
        };
        depth[node] = d;
        state[node] = 2;
        Some(d)
    }

    for node in 0..n {
        visit(node, root, parent, &mut state, &mut depth)?;
    }

    Some(TreeInfo { depth })
}

fn is_valid_permutation(mapping: &[usize]) -> bool {
    let n = mapping.len();
    let mut seen = vec![false; n];
    for &image in mapping {
        if image >= n || seen[image] {
            return false;
        }
        seen[image] = true;
    }
    true
}

fn is_ancestor(parent: &[usize], ancestor: usize, descendant: usize) -> bool {
    let mut current = descendant;
    loop {
        if current == ancestor {
            return true;
        }
        let next = parent[current];
        if next == current {
            return false;
        }
        current = next;
    }
}

fn are_ancestor_comparable(parent: &[usize], u: usize, v: usize) -> bool {
    is_ancestor(parent, u, v) || is_ancestor(parent, v, u)
}

crate::impl_random_generate!(
    RootedTreeArrangement<SimpleGraph>,
    RootedTreeArrangementRandomSpec,
    |spec| {
        let graph = crate::random::SimpleGraphRandomSpec {
            num_vertices: spec.num_vertices,
            edge_prob: spec.edge_prob,
            seed: spec.seed,
        }
        .graph()?;
        let bound = match spec.bound {
            Some(bound) => bound,
            None => {
                let max_depth = if spec.num_vertices == 0 {
                    0
                } else {
                    spec.num_vertices - 1
                };
                let max_stretch = max_depth.checked_mul(graph.num_edges()).ok_or_else(|| {
                    crate::registry::ConstructionError::IntegerOverflow(
                        "default rooted-tree arrangement bound overflows usize".into(),
                    )
                })?;
                i64::try_from(max_stretch).map_err(|_| {
                    crate::registry::ConstructionError::IntegerOverflow(
                        "default rooted-tree arrangement bound does not fit i64".into(),
                    )
                })?
            }
        };
        Ok(RootedTreeArrangement::new(graph, bound))
    }
);

crate::declare_variants! {
    default RootedTreeArrangement<SimpleGraph> => "2^num_vertices" random,
}

crate::register_brute_force! {
    RootedTreeArrangement<SimpleGraph>,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "rooted_tree_arrangement_simplegraph",
        instance: Box::new(RootedTreeArrangement::new(
            SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]),
            5,
        )),
        optimal_config: serde_json::json!(vec![0, 0, 1, 2, 0, 1, 2, 3]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/rooted_tree_arrangement.rs"]
mod tests;
