//! Reduction from graph K-Coloring to Boolean Satisfiability.

use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::graph::KColoring;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::KN;

/// Result of reducing K-Coloring to Satisfiability.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToSatisfiability {
    target: Satisfiability,
    num_vertices: usize,
    num_colors: usize,
}

impl ReductionResult for ReductionKColoringToSatisfiability {
    type Source = KColoring<KN, SimpleGraph>;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_vertices)
            .map(|vertex| {
                let start = vertex * self.num_colors;
                target_solution[start..start + self.num_colors]
                    .iter()
                    .position(|&value| value == 1)
                    .expect("satisfying assignment must select one color per vertex")
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices * num_colors",
        num_clauses = "num_vertices + num_vertices * num_colors * (num_colors - 1) / 2 + num_edges * num_colors",
        num_literals = "num_vertices * num_colors + num_vertices * num_colors * (num_colors - 1) + 2 * num_edges * num_colors",
    }
)]
impl ReduceTo<Satisfiability> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToSatisfiability;

    fn reduce_to(&self) -> Self::Result {
        let num_vertices = self.graph().num_vertices();
        let num_colors = self.num_colors();
        let variable = |vertex: usize, color: usize| (vertex * num_colors + color + 1) as i32;
        let mut clauses = Vec::new();

        for vertex in 0..num_vertices {
            clauses.push(CNFClause::new(
                (0..num_colors)
                    .map(|color| variable(vertex, color))
                    .collect(),
            ));
        }

        for vertex in 0..num_vertices {
            for first_color in 0..num_colors {
                for second_color in first_color + 1..num_colors {
                    clauses.push(CNFClause::new(vec![
                        -variable(vertex, first_color),
                        -variable(vertex, second_color),
                    ]));
                }
            }
        }

        for (first_vertex, second_vertex) in self.graph().edges() {
            for color in 0..num_colors {
                clauses.push(CNFClause::new(vec![
                    -variable(first_vertex, color),
                    -variable(second_vertex, color),
                ]));
            }
        }

        ReductionKColoringToSatisfiability {
            target: Satisfiability::new(num_vertices * num_colors, clauses),
            num_vertices,
            num_colors,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_satisfiability",
        build: || {
            let source = KColoring::<KN, _>::with_k(SimpleGraph::cycle(5), 3);
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0, 1, 2],
                    target_config: vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_satisfiability.rs"]
mod tests;
