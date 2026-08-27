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
    target: ILP<i64>,
}

impl ReductionResult for ReductionIFBToILP {
    type Source = IntegralFlowBundles;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::decode_usize_values(target_solution)
    }
}

#[reduction(
    size = exact {
        num_vars = "num_arcs",
        num_constraints = "num_bundles + num_vertices - 1",
    },
)]
impl ReduceTo<ILP<i64>> for IntegralFlowBundles {
    type Result = ReductionIFBToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let arcs = self.graph().arcs();
        let mut constraints = Vec::with_capacity(self.num_bundles() + self.num_vertices() - 1);

        for (bundle, &capacity) in self.bundles().iter().zip(self.bundle_capacities()) {
            let terms = bundle.iter().map(|&arc_index| (arc_index, 1)).collect();
            constraints.push(LinearConstraint::le(terms, capacity));
        }

        for vertex in 0..self.num_vertices() {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }

            let mut terms = Vec::new();
            for (arc_index, (u, v)) in arcs.iter().copied().enumerate() {
                if vertex == u {
                    terms.push((arc_index, -1));
                }
                if vertex == v {
                    terms.push((arc_index, 1));
                }
            }
            constraints.push(LinearConstraint::eq(terms, 0));
        }

        let mut sink_terms = Vec::new();
        for (arc_index, (u, v)) in arcs.iter().copied().enumerate() {
            if self.sink() == u {
                sink_terms.push((arc_index, -1));
            }
            if self.sink() == v {
                sink_terms.push((arc_index, 1));
            }
        }
        constraints.push(LinearConstraint::ge(sink_terms, self.requirement()));

        Ok(ReductionIFBToILP {
            target: ILP::new(
                self.num_arcs(),
                constraints,
                vec![],
                ObjectiveSense::Minimize,
            )
            .map_err(Self::target_construction)?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "integralflowbundles_to_ilp",
        build: || {
            let source = IntegralFlowBundles::new(
                DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2), (2, 1)]),
                0,
                3,
                vec![vec![0, 1], vec![2, 5], vec![3, 4]],
                vec![1, 1, 1],
                1,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integralflowbundles_ilp.rs"]
mod tests;
