//! Reduction from PartitionIntoCliques to MinimumCoveringByCliques.
//!
//! This implements Orlin's classical construction for turning a partition of
//! the source vertices into an edge-clique cover. The target graph contains
//! left/right copies of the source vertices, one directed gadget per source
//! edge, and two side-clique anchors.
//! The source is satisfiable iff the target optimum is at most min(K,n) + q + 2,
//! where q counts distinct directed non-loop adjacencies. Each side includes
//! a private vertex, so its forced clique exists even for an empty source.

use crate::models::graph::{MinimumCoveringByCliques, PartitionIntoCliques};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{Min, OptimizationValue, Or};
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
        vertices.push(self.z_right() + 1);
        vertices
    }

    fn right_vertices(&self) -> Vec<usize> {
        let mut vertices = (0..self.num_source_vertices)
            .map(|i| self.y(i))
            .collect::<Vec<_>>();
        vertices.extend((0..self.num_directed_pairs()).map(|idx| self.b(idx)));
        vertices.push(self.z_right() + 2);
        vertices
    }

    fn dimensions(&self) -> Result<(usize, usize), crate::rules::ReductionError> {
        let n = self.num_source_vertices;
        let q = self.num_directed_pairs();
        let overflow = |operation: &str| {
            crate::rules::ReductionError::integer_overflow::<
                PartitionIntoCliques<SimpleGraph>,
                MinimumCoveringByCliques<SimpleGraph>,
            >(operation)
        };
        // The two sides each have n+q+1 vertices, including their private
        // vertex. These checked counts bound every subsequent layout index.
        let side = n
            .checked_add(q)
            .and_then(|s| s.checked_add(1))
            .ok_or_else(|| overflow("counting side vertices"))?;
        let target_vertices = side
            .checked_mul(2)
            .and_then(|s| s.checked_add(2))
            .ok_or_else(|| overflow("counting target vertices"))?;
        let target_edges = side
            .checked_mul(side)
            .and_then(|s| s.checked_add(side))
            .and_then(|s| s.checked_add(n))
            .and_then(|s| q.checked_mul(4).and_then(|cross| s.checked_add(cross)))
            .ok_or_else(|| overflow("counting target edges"))?;
        <PartitionIntoCliques<SimpleGraph> as ReduceTo<MinimumCoveringByCliques<SimpleGraph>>>::exact_i64(
            target_edges,
            "representing every target cover value",
        )?;
        Ok((target_vertices, target_edges))
    }
}

fn add_clique_edges(vertices: &[usize], edges: &mut Vec<(usize, usize)>) {
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            edges.push((vertices[i], vertices[j]));
        }
    }
}

fn target_clique_bound(
    num_cliques: i64,
    num_directed_pairs: i64,
) -> Result<i64, crate::rules::ReductionError> {
    num_directed_pairs
        .checked_add(2)
        .and_then(|offset| num_cliques.checked_add(offset))
        .ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                PartitionIntoCliques<SimpleGraph>,
                MinimumCoveringByCliques<SimpleGraph>,
            >("computing target clique bound")
        })
}

/// Result of reducing PartitionIntoCliques to MinimumCoveringByCliques.
#[derive(Debug, Clone)]
pub struct ReductionPartitionIntoCliquesToMinimumCoveringByCliques {
    target: MinimumCoveringByCliques<SimpleGraph>,
    num_source_vertices: usize,
    source_num_cliques: usize,
    target_bound: i64,
}

impl ReductionResult for ReductionPartitionIntoCliquesToMinimumCoveringByCliques {
    type Source = PartitionIntoCliques<SimpleGraph>;
    type Target = MinimumCoveringByCliques<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !Min::meets_bound(&value, &self.target_bound) {
            return Err(crate::rules::ExtractionError::invalid(
                "target cover does not certify the source clique bound",
            ));
        }

        Ok({
            let n = self.num_source_vertices;
            let target_edges = self.target.graph().edges();
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

            let mut label_map = BTreeMap::new();
            let extracted = matching_labels
                .into_iter()
                .map(|label| {
                    let label = label.ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(
                            "target cover does not label every matching gadget edge",
                        )
                    })?;
                    let next = label_map.len();
                    Ok(*label_map.entry(label).or_insert(next))
                })
                .collect::<crate::rules::ExtractionResult<Vec<_>>>()?;

            if label_map.len() > self.source_num_cliques {
                return Err(crate::rules::ExtractionError::invalid(format!(
                    "target cover uses {} cliques, exceeding source bound {}",
                    label_map.len(),
                    self.source_num_cliques
                )));
            }

            // Equal matching-edge labels imply pairwise source adjacency.
            // The target certificate leaves at most K labels for these edges.
            extracted
        })
    }
}

impl crate::rules::AggregateReductionResult
    for ReductionPartitionIntoCliquesToMinimumCoveringByCliques
{
    type Source = PartitionIntoCliques<SimpleGraph>;
    type Target = MinimumCoveringByCliques<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Min<i64>) -> Or {
        Or(Min::meets_bound(&target_value, &self.target_bound))
    }
}

#[reduction(
    aggregate = custom,
    transform = upper_bound {
        num_vertices = "2 * num_vertices + 4 * num_edges + 4",
        num_edges = "(num_vertices + 2 * num_edges)^2 + 4 * num_vertices + 14 * num_edges + 2",
    }
)]
impl ReduceTo<MinimumCoveringByCliques<SimpleGraph>> for PartitionIntoCliques<SimpleGraph> {
    type Result = ReductionPartitionIntoCliquesToMinimumCoveringByCliques;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let layout = OrlinLayout::new(self.graph());
        let q = layout.num_directed_pairs();
        let (target_vertices, target_edges) = layout.dimensions()?;
        let source_bound = <Self as ReduceTo<MinimumCoveringByCliques<SimpleGraph>>>::exact_i64(
            self.num_cliques().min(n),
            "converting effective clique bound",
        )?;
        let directed_pairs = <Self as ReduceTo<MinimumCoveringByCliques<SimpleGraph>>>::exact_i64(
            q,
            "converting gadget count",
        )?;
        let target_bound = target_clique_bound(source_bound, directed_pairs)?;
        let left_vertices = layout.left_vertices();
        let right_vertices = layout.right_vertices();
        let mut edges = Vec::with_capacity(target_edges);

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

        let target_graph = SimpleGraph::new(target_vertices, edges);
        let target = MinimumCoveringByCliques::new(target_graph);

        Ok(ReductionPartitionIntoCliquesToMinimumCoveringByCliques {
            target,
            num_source_vertices: n,
            source_num_cliques: self.num_cliques(),
            target_bound,
        })
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
            let reduction = ReduceTo::<MinimumCoveringByCliques<SimpleGraph>>::reduce_to(&source)
                .expect("reduction should succeed");
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
                    source_config: serde_json::json!(vec![0, 0, 1]),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partitionintocliques_minimumcoveringbycliques.rs"]
mod tests;
