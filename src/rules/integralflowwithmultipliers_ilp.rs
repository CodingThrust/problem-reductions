//! Reduction from IntegralFlowWithMultipliers to ILP.
//!
//! One integer flow variable per arc. Capacity bounds, multiplier-scaled
//! conservation at non-terminals, and sink inflow requirement.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::IntegralFlowWithMultipliers;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing IntegralFlowWithMultipliers to ILP.
#[derive(Debug, Clone)]
pub struct ReductionIFWMToILP {
    target: ILP<i64>,
}

impl ReductionResult for ReductionIFWMToILP {
    type Source = IntegralFlowWithMultipliers;
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
    transform = exact {
        num_vars = "num_arcs",
        num_constraints = "num_arcs + num_vertices - 1",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<i64>> for IntegralFlowWithMultipliers {
    type Result = ReductionIFWMToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let arcs = self.graph().arcs();
        let num_arcs = self.num_arcs();
        let num_vertices = self.num_vertices();
        let mut constraints = Vec::new();

        // Capacity: f_a <= c_a for each arc
        for (arc_idx, &capacity) in self.capacities().iter().enumerate() {
            constraints.push(LinearConstraint::le(vec![(arc_idx, 1)], capacity));
        }

        // Multiplier-scaled conservation:
        // sum_{a in delta^+(v)} f_a = h(v) * sum_{a in delta^-(v)} f_a
        // for all v in V \ {s, t}
        // Rewrite: sum_{a in delta^+(v)} f_a - h(v) * sum_{a in delta^-(v)} f_a = 0
        for vertex in 0..num_vertices {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }
            let multiplier = self.multipliers()[vertex];
            let mut terms = Vec::new();
            for (arc_idx, &(u, v)) in arcs.iter().enumerate() {
                if u == vertex {
                    terms.push((arc_idx, 1)); // outgoing
                }
                if v == vertex {
                    terms.push((arc_idx, -multiplier)); // incoming scaled by -h(v)
                }
            }
            constraints.push(LinearConstraint::eq(terms, 0));
        }

        // Sink inflow requirement: sum_{a in delta^-(t)} f_a - sum_{a in delta^+(t)} f_a >= R
        let mut sink_terms = Vec::new();
        for (arc_idx, &(u, v)) in arcs.iter().enumerate() {
            if v == self.sink() {
                sink_terms.push((arc_idx, 1)); // incoming
            }
            if u == self.sink() {
                sink_terms.push((arc_idx, -1)); // outgoing
            }
        }
        constraints.push(LinearConstraint::ge(sink_terms, self.requirement()));

        Ok(ReductionIFWMToILP {
            target: ILP::new(num_arcs, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "integralflowwithmultipliers_to_ilp",
        build: || {
            // Simple diamond: s=0, t=3, intermediate vertices 1,2 with multiplier 1
            let source = IntegralFlowWithMultipliers::new(
                DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
                0,
                3,
                vec![1, 1, 1, 1], // source/sink entries ignored
                vec![2, 2, 2, 2],
                2,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integralflowwithmultipliers_ilp.rs"]
mod tests;
