//! Reduction from PartitionIntoCliques to MinimumCoveringByCliques.
//!
//! This implements Orlin's classical construction for turning a partition of
//! the source vertices into an edge-clique cover. The target graph contains
//! left/right copies of the source vertices, one directed gadget per source
//! edge, and two side-clique anchors.

use crate::models::graph::{MinimumCoveringByCliques, PartitionIntoCliques};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct OrlinLayout {
    num_source_vertices: usize,
    directed_pairs: Vec<(usize, usize)>,
}

impl OrlinLayout {
    fn new(graph: &SimpleGraph) -> Self {
        let n = graph.num_vertices();
        let directed_pairs = (0..n)
            .flat_map(|i| {
                (0..n).filter_map(move |j| (i != j && graph.has_edge(i, j)).then_some((i, j)))
            })
            .collect();
        Self {
            num_source_vertices: n,
            directed_pairs,
        }
    }

    fn num_directed_pairs(&self) -> usize {
        self.directed_pairs.len()
    }

    fn x(&self, i: usize) -> usize {
        i
    }

    fn y(&self, i: usize) -> usize {
        self.num_source_vertices + i
    }

    fn a(&self, directed_pair_index: usize) -> usize {
        2 * self.num_source_vertices + directed_pair_index
    }

    fn b(&self, directed_pair_index: usize) -> usize {
        2 * self.num_source_vertices + self.num_directed_pairs() + directed_pair_index
    }

    fn z_left(&self) -> usize {
        2 * self.num_source_vertices + 2 * self.num_directed_pairs()
    }

    fn z_right(&self) -> usize {
        self.z_left() + 1
    }

    fn left_vertices(&self) -> Vec<usize> {
        let mut vertices = (0..self.num_source_vertices)
            .map(|i| self.x(i))
            .collect::<Vec<_>>();
        vertices.extend((0..self.num_directed_pairs()).map(|idx| self.a(idx)));
        vertices
    }

    fn right_vertices(&self) -> Vec<usize> {
        let mut vertices = (0..self.num_source_vertices)
            .map(|i| self.y(i))
            .collect::<Vec<_>>();
        vertices.extend((0..self.num_directed_pairs()).map(|idx| self.b(idx)));
        vertices
    }

    fn total_vertices(&self) -> usize {
        self.z_right() + 1
    }
}

fn add_clique_edges(vertices: &[usize], edges: &mut Vec<(usize, usize)>) {
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            edges.push((vertices[i], vertices[j]));
        }
    }
}

fn invalid_source_solution(num_source_vertices: usize, num_source_cliques: usize) -> Vec<usize> {
    vec![num_source_cliques; num_source_vertices]
}

/// Result of reducing PartitionIntoCliques to MinimumCoveringByCliques.
#[derive(Debug, Clone)]
pub struct ReductionPartitionIntoCliquesToMinimumCoveringByCliques {
    target: MinimumCoveringByCliques<SimpleGraph>,
    source_graph: SimpleGraph,
    source_num_cliques: usize,
}

impl ReductionResult for ReductionPartitionIntoCliquesToMinimumCoveringByCliques {
    type Source = PartitionIntoCliques<SimpleGraph>;
    type Target = MinimumCoveringByCliques<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.source_graph.num_vertices();
        let target_edges = self.target.graph().edges();
        if target_solution.len() != target_edges.len() {
            return invalid_source_solution(n, self.source_num_cliques);
        }

        let mut matching_labels = vec![None; n];
        for ((u, v), &label) in target_edges.iter().zip(target_solution.iter()) {
            let matching_index = if *u < n && *v == n + *u {
                Some(*u)
            } else if *v < n && *u == n + *v {
                Some(*v)
            } else {
                None
            };

            if let Some(i) = matching_index {
                matching_labels[i] = Some(label);
            }
        }

        if matching_labels.iter().any(Option::is_none) {
            return invalid_source_solution(n, self.source_num_cliques);
        }

        let mut label_map = BTreeMap::new();
        let extracted = matching_labels
            .into_iter()
            .map(|label| {
                let label = label.expect("checked above");
                let next = label_map.len();
                *label_map.entry(label).or_insert(next)
            })
            .collect::<Vec<_>>();

        if label_map.len() > self.source_num_cliques {
            return invalid_source_solution(n, self.source_num_cliques);
        }

        let source_problem =
            PartitionIntoCliques::new(self.source_graph.clone(), self.source_num_cliques);
        if <PartitionIntoCliques<SimpleGraph> as crate::traits::Problem>::evaluate(
            &source_problem,
            &extracted,
        )
        .0
        {
            extracted
        } else {
            invalid_source_solution(n, self.source_num_cliques)
        }
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vertices + 4 * num_edges + 2",
        num_edges = "(num_vertices + 2 * num_edges)^2 + 2 * num_vertices + 10 * num_edges",
    }
)]
impl ReduceTo<MinimumCoveringByCliques<SimpleGraph>> for PartitionIntoCliques<SimpleGraph> {
    type Result = ReductionPartitionIntoCliquesToMinimumCoveringByCliques;

    fn reduce_to(&self) -> Self::Result {
        let layout = OrlinLayout::new(self.graph());
        let left_vertices = layout.left_vertices();
        let right_vertices = layout.right_vertices();
        let mut edges = Vec::new();

        // Step 1-2: L and R are cliques.
        add_clique_edges(&left_vertices, &mut edges);
        add_clique_edges(&right_vertices, &mut edges);

        // Step 3-4: connect z_L and z_R to their respective sides.
        for &u in &left_vertices {
            edges.push((layout.z_left(), u));
        }
        for &u in &right_vertices {
            edges.push((layout.z_right(), u));
        }

        // Step 5: matching edges x_i y_i.
        for i in 0..self.num_vertices() {
            edges.push((layout.x(i), layout.y(i)));
        }

        // Step 6: one 4-vertex gadget for each directed source edge.
        for (idx, &(i, j)) in layout.directed_pairs.iter().enumerate() {
            edges.push((layout.x(i), layout.y(j)));
            edges.push((layout.x(i), layout.b(idx)));
            edges.push((layout.a(idx), layout.y(j)));
            edges.push((layout.a(idx), layout.b(idx)));
        }

        let target_graph = SimpleGraph::new(layout.total_vertices(), edges);
        let target = MinimumCoveringByCliques::new(target_graph);

        ReductionPartitionIntoCliquesToMinimumCoveringByCliques {
            target,
            source_graph: self.graph().clone(),
            source_num_cliques: self.num_cliques(),
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
fn edge_labels_from_clique_cover(graph: &SimpleGraph, cliques: &[Vec<usize>]) -> Vec<usize> {
    let clique_sets = cliques
        .iter()
        .map(|clique| {
            clique
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
        })
        .collect::<Vec<_>>();

    graph
        .edges()
        .into_iter()
        .map(|(u, v)| {
            clique_sets
                .iter()
                .position(|clique| clique.contains(&u) && clique.contains(&v))
                .expect("canonical cover should cover every target edge")
        })
        .collect()
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partitionintocliques_to_minimumcoveringbycliques",
        build: || {
            let source = PartitionIntoCliques::new(SimpleGraph::new(3, vec![(0, 1)]), 2);
            let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source);
            let layout = OrlinLayout::new(source.graph());

            let target_config = edge_labels_from_clique_cover(
                reduction.target_problem().graph(),
                &[
                    vec![layout.x(0), layout.x(1), layout.y(0), layout.y(1)],
                    vec![layout.x(2), layout.y(2)],
                    vec![layout.x(0), layout.a(0), layout.b(0), layout.y(1)],
                    vec![layout.x(1), layout.a(1), layout.b(1), layout.y(0)],
                    {
                        let mut clique = layout.left_vertices();
                        clique.push(layout.z_left());
                        clique
                    },
                    {
                        let mut clique = layout.right_vertices();
                        clique.push(layout.z_right());
                        clique
                    },
                ],
            );

            crate::example_db::specs::rule_example_with_witness::<
                _,
                MinimumCoveringByCliques<SimpleGraph>,
            >(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1],
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partitionintocliques_minimumcoveringbycliques.rs"]
mod tests;
