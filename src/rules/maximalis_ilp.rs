//! Reduction from MaximalIS to ILP (Integer Linear Programming).
//!
//! Binary variable x_v per vertex. Independence: ∀ edge (u,v): x_u + x_v ≤ 1.
//! Maximality: ∀ v: x_v + Σ_{u∈N(v)} x_u ≥ 1. Maximize Σ w_v·x_v.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximalIS;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

#[derive(Debug, Clone)]
pub struct ReductionMxISToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionMxISToILP {
    type Source = MaximalIS<SimpleGraph, i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices",
        num_constraints = "num_edges + num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for MaximalIS<SimpleGraph, i32> {
    type Result = ReductionMxISToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let mut constraints = Vec::new();

        // Independence: ∀ edge (u,v): x_u + x_v ≤ 1
        for u in 0..n {
            for v in (u + 1)..n {
                if self.graph().has_edge(u, v) {
                    constraints.push(LinearConstraint::le(vec![(u, 1.0), (v, 1.0)], 1.0));
                }
            }
        }

        // Maximality: ∀ v: x_v + Σ_{u∈N(v)} x_u ≥ 1
        for v in 0..n {
            let mut terms = vec![(v, 1.0)];
            for u in self.graph().neighbors(v) {
                terms.push((u, 1.0));
            }
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // Objective: Maximize Σ w_v·x_v
        let weights = self.weights();
        let objective: Vec<(usize, f64)> =
            weights.iter().enumerate().map(|(i, w)| (i, *w as f64)).collect();

        let target = ILP::new(n, constraints, objective, ObjectiveSense::Maximize);
        ReductionMxISToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximalis_to_ilp",
        build: || {
            // Path P3: 0-1-2
            let source = MaximalIS::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![1, 1, 1],
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 1],
                    target_config: vec![1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximalis_ilp.rs"]
mod tests;
