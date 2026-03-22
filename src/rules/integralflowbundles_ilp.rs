//! Reduction from Integral Flow with Bundles to ILP.
//!
//! Each directed arc gets one non-negative integer ILP variable. The ILP keeps
//! the bundle-capacity inequalities, flow-conservation equalities at
//! nonterminals, and the sink inflow lower bound from the source problem.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::IntegralFlowBundles;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing IntegralFlowBundles to ILP.
#[derive(Debug, Clone)]
pub struct ReductionIFBToILP {
    target: ILP<i32>,
}

impl ReductionResult for ReductionIFBToILP {
    type Source = IntegralFlowBundles;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_arcs",
        num_constraints = "num_bundles + num_vertices - 1",
    }
)]
impl ReduceTo<ILP<i32>> for IntegralFlowBundles {
    type Result = ReductionIFBToILP;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self.graph().arcs();
        let mut constraints = Vec::with_capacity(self.num_bundles() + self.num_vertices() - 1);

        for (bundle, &capacity) in self.bundles().iter().zip(self.bundle_capacities()) {
            let terms = bundle.iter().map(|&arc_index| (arc_index, 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, capacity as f64));
        }

        for vertex in 0..self.num_vertices() {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }

            let mut terms = Vec::new();
            for (arc_index, (u, v)) in arcs.iter().copied().enumerate() {
                if vertex == u {
                    terms.push((arc_index, -1.0));
                }
                if vertex == v {
                    terms.push((arc_index, 1.0));
                }
            }
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        let mut sink_terms = Vec::new();
        for (arc_index, (u, v)) in arcs.iter().copied().enumerate() {
            if self.sink() == u {
                sink_terms.push((arc_index, -1.0));
            }
            if self.sink() == v {
                sink_terms.push((arc_index, 1.0));
            }
        }
        constraints.push(LinearConstraint::ge(sink_terms, self.requirement() as f64));

        ReductionIFBToILP {
            target: ILP::new(
                self.num_arcs(),
                constraints,
                vec![],
                ObjectiveSense::Minimize,
            ),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "integralflowbundles_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                IntegralFlowBundles::new(
                    DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (2, 1)]),
                    0,
                    3,
                    vec![vec![0, 1], vec![2, 5], vec![3, 4]],
                    vec![1, 1, 1],
                    1,
                ),
                SolutionPair {
                    source_config: vec![1, 0, 1, 0, 0, 0],
                    target_config: vec![1, 0, 1, 0, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integralflowbundles_ilp.rs"]
mod tests;
