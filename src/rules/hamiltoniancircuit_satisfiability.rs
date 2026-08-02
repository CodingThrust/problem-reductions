//! Reduction from HamiltonianCircuit to Satisfiability.
//!
//! The construction uses one Boolean variable for each `(vertex, position)` pair.
//! Exactly-one constraints make satisfying assignments permutation matrices, and
//! forbidden-successor clauses require consecutive vertices, including the last
//! and first positions, to be adjacent in the source graph.

use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::graph::HamiltonianCircuit;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianCircuit to Satisfiability.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToSatisfiability {
    target: Satisfiability,
    num_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianCircuitToSatisfiability {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;
        (0..n)
            .map(|position| {
                (0..n)
                    .find(|&vertex| target_solution[vertex * n + position] == 1)
                    .expect("satisfying assignment has one vertex at every position")
            })
            .collect()
    }
}

fn variable(vertex: usize, position: usize, n: usize) -> i32 {
    (vertex * n + position + 1) as i32
}

#[reduction(overhead = {
    num_vars = "num_vertices * num_vertices + 1",
    num_clauses = "2 * num_vertices + num_vertices * num_vertices * (num_vertices - 1) + num_vertices^3 + 2",
    num_literals = "4 * num_vertices^3 + 2",
})]
impl ReduceTo<Satisfiability> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToSatisfiability;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        if n < 3 {
            return ReductionHamiltonianCircuitToSatisfiability {
                target: Satisfiability::new(
                    1,
                    vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
                ),
                num_vertices: n,
            };
        }

        let mut clauses = Vec::new();

        // Every position contains exactly one vertex.
        for position in 0..n {
            clauses.push(CNFClause::new(
                (0..n).map(|vertex| variable(vertex, position, n)).collect(),
            ));
            for first_vertex in 0..n {
                for second_vertex in first_vertex + 1..n {
                    clauses.push(CNFClause::new(vec![
                        -variable(first_vertex, position, n),
                        -variable(second_vertex, position, n),
                    ]));
                }
            }
        }

        // Every vertex occurs at exactly one position.
        for vertex in 0..n {
            clauses.push(CNFClause::new(
                (0..n)
                    .map(|position| variable(vertex, position, n))
                    .collect(),
            ));
            for first_position in 0..n {
                for second_position in first_position + 1..n {
                    clauses.push(CNFClause::new(vec![
                        -variable(vertex, first_position, n),
                        -variable(vertex, second_position, n),
                    ]));
                }
            }
        }

        // Consecutive positions contain distinct adjacent vertices. The successor
        // position is cyclic, so this also enforces the closing edge.
        for position in 0..n {
            let successor = (position + 1) % n;
            for vertex in 0..n {
                for next_vertex in 0..n {
                    if vertex == next_vertex || !self.graph().has_edge(vertex, next_vertex) {
                        clauses.push(CNFClause::new(vec![
                            -variable(vertex, position, n),
                            -variable(next_vertex, successor, n),
                        ]));
                    }
                }
            }
        }

        ReductionHamiltonianCircuitToSatisfiability {
            target: Satisfiability::new(n * n, clauses),
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_satisfiability",
        build: || {
            let source = HamiltonianCircuit::new(SimpleGraph::new(
                5,
                vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (0, 2)],
            ));
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3, 4],
                    target_config: vec![
                        1, 0, 0, 0, 0, // vertex 0 is at position 0
                        0, 1, 0, 0, 0, // vertex 1 is at position 1
                        0, 0, 1, 0, 0, // vertex 2 is at position 2
                        0, 0, 0, 1, 0, // vertex 3 is at position 3
                        0, 0, 0, 0, 1, // vertex 4 is at position 4
                    ],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_satisfiability.rs"]
mod tests;
