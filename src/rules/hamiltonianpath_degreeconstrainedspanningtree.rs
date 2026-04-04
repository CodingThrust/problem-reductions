//! Reduction from HamiltonianPath to DegreeConstrainedSpanningTree.
//!
//! A spanning tree with maximum degree 2 is exactly a Hamiltonian path.

use crate::models::graph::{DegreeConstrainedSpanningTree, HamiltonianPath};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianPath to DegreeConstrainedSpanningTree.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianPathToDegreeConstrainedSpanningTree {
    target: DegreeConstrainedSpanningTree<SimpleGraph>,
}

impl ReductionResult for ReductionHamiltonianPathToDegreeConstrainedSpanningTree {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = DegreeConstrainedSpanningTree<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        extract_hamiltonian_order(self.target.graph(), target_solution)
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<DegreeConstrainedSpanningTree<SimpleGraph>> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHamiltonianPathToDegreeConstrainedSpanningTree;

    fn reduce_to(&self) -> Self::Result {
        let target = DegreeConstrainedSpanningTree::new(
            SimpleGraph::new(self.graph().num_vertices(), self.graph().edges()),
            2,
        );
        ReductionHamiltonianPathToDegreeConstrainedSpanningTree { target }
    }
}

fn extract_hamiltonian_order(graph: &SimpleGraph, target_solution: &[usize]) -> Vec<usize> {
    let num_vertices = graph.num_vertices();
    if num_vertices == 0 {
        return vec![];
    }
    if num_vertices == 1 {
        return vec![0];
    }

    let edges = graph.edges();
    if target_solution.len() != edges.len() {
        return vec![];
    }

    let mut adjacency = vec![Vec::new(); num_vertices];
    for ((u, v), &selected) in edges.iter().copied().zip(target_solution.iter()) {
        if selected != 1 {
            continue;
        }
        adjacency[u].push(v);
        adjacency[v].push(u);
    }

    let mut endpoints: Vec<usize> = adjacency
        .iter()
        .enumerate()
        .filter_map(|(vertex, neighbors)| (neighbors.len() == 1).then_some(vertex))
        .collect();
    endpoints.sort_unstable();
    if endpoints.len() != 2 {
        return vec![];
    }

    let mut order = Vec::with_capacity(num_vertices);
    let mut visited = vec![false; num_vertices];
    let mut previous = None;
    let mut current = endpoints[0];

    loop {
        if visited[current] {
            return vec![];
        }
        visited[current] = true;
        order.push(current);

        let next = adjacency[current]
            .iter()
            .copied()
            .find(|&neighbor| Some(neighbor) != previous && !visited[neighbor]);
        match next {
            Some(next_vertex) => {
                previous = Some(current);
                current = next_vertex;
            }
            None => break,
        }
    }

    if order.len() == num_vertices {
        order
    } else {
        vec![]
    }
}

#[cfg(feature = "example-db")]
fn edge_config_for_path(graph: &SimpleGraph, path: &[usize]) -> Vec<usize> {
    let selected_edges: Vec<(usize, usize)> = path
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect();
    graph
        .edges()
        .into_iter()
        .map(|(u, v)| {
            usize::from(
                selected_edges
                    .iter()
                    .any(|&(a, b)| (a == u && b == v) || (a == v && b == u)),
            )
        })
        .collect()
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    fn source_example() -> HamiltonianPath<SimpleGraph> {
        HamiltonianPath::new(SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (3, 4),
                (3, 5),
                (4, 2),
                (5, 1),
            ],
        ))
    }

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_degreeconstrainedspanningtree",
        build: || {
            let source_config = vec![0, 2, 4, 3, 1, 5];
            let source = source_example();
            let reduction =
                ReduceTo::<DegreeConstrainedSpanningTree<SimpleGraph>>::reduce_to(&source);
            let target_config =
                edge_config_for_path(reduction.target_problem().graph(), &source_config);
            crate::example_db::specs::rule_example_with_witness::<
                _,
                DegreeConstrainedSpanningTree<SimpleGraph>,
            >(
                source,
                crate::export::SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_degreeconstrainedspanningtree.rs"]
mod tests;
