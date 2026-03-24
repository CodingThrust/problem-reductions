//! Reduction from IsomorphicSpanningTree to ILP (Integer Linear Programming).
//!
//! Binary variable x_{u,v} with x_{u,v} = 1 iff tree vertex u maps to graph
//! vertex v. Bijection constraints plus non-edge exclusion for every tree edge.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::IsomorphicSpanningTree;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;

#[derive(Debug, Clone)]
pub struct ReductionISTToILP {
    target: ILP<bool>,
    n: usize,
}

impl ReductionResult for ReductionISTToILP {
    type Source = IsomorphicSpanningTree;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// For each tree vertex u, output the unique graph vertex v with x_{u,v} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        (0..n)
            .map(|u| {
                (0..n)
                    .find(|&v| target_solution[u * n + v] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices * num_vertices",
        num_constraints = "2 * num_vertices + 2 * num_tree_edges * num_vertices * num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for IsomorphicSpanningTree {
    type Result = ReductionISTToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let num_vars = n * n;

        let mut constraints = Vec::new();

        // Each tree vertex u maps to exactly one graph vertex:
        // Σ_v x_{u,v} = 1  ∀ u
        for u in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|v| (u * n + v, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Each graph vertex v is mapped to by exactly one tree vertex:
        // Σ_u x_{u,v} = 1  ∀ v
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|u| (u * n + v, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // For each tree edge {u, w} and each pair (v, z) that is NOT a graph edge:
        // x_{u,v} + x_{w,z} <= 1
        // x_{u,z} + x_{w,v} <= 1
        for (u, w) in self.tree().edges() {
            for v in 0..n {
                for z in 0..n {
                    if v != z && !self.graph().has_edge(v, z) {
                        constraints.push(LinearConstraint::le(
                            vec![(u * n + v, 1.0), (w * n + z, 1.0)],
                            1.0,
                        ));
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionISTToILP { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::SimpleGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "isomorphicspanningtree_to_ilp",
        build: || {
            // K4 graph, star tree
            let source = IsomorphicSpanningTree::new(
                SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
                SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
            );
            // Identity bijection works
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 2, 3],
                    // x_{0,0}=1, x_{1,1}=1, x_{2,2}=1, x_{3,3}=1
                    target_config: vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/isomorphicspanningtree_ilp.rs"]
mod tests;
