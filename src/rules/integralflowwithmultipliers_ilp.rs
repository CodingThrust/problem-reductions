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
    target: ILP<i32>,
}

impl ReductionResult for ReductionIFWMToILP {
    type Source = IntegralFlowWithMultipliers;
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
        num_constraints = "num_arcs + num_vertices - 1",
    }
)]
impl ReduceTo<ILP<i32>> for IntegralFlowWithMultipliers {
    type Result = ReductionIFWMToILP;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self.graph().arcs();
        let num_arcs = self.num_arcs();
        let num_vertices = self.num_vertices();
        let mut constraints = Vec::new();

        // Capacity: f_a <= c_a for each arc
        for (arc_idx, &capacity) in self.capacities().iter().enumerate() {
            constraints.push(LinearConstraint::le(vec![(arc_idx, 1.0)], capacity as f64));
        }

        // Multiplier-scaled conservation:
        // sum_{a in delta^+(v)} f_a = h(v) * sum_{a in delta^-(v)} f_a
        // for all v in V \ {s, t}
        // Rewrite: sum_{a in delta^+(v)} f_a - h(v) * sum_{a in delta^-(v)} f_a = 0
        for vertex in 0..num_vertices {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }
            let multiplier = self.multipliers()[vertex] as f64;
            let mut terms = Vec::new();
            for (arc_idx, &(u, v)) in arcs.iter().enumerate() {
                if u == vertex {
                    terms.push((arc_idx, 1.0)); // outgoing
                }
                if v == vertex {
                    terms.push((arc_idx, -multiplier)); // incoming scaled by -h(v)
                }
            }
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // Sink inflow requirement: sum_{a in delta^-(t)} f_a - sum_{a in delta^+(t)} f_a >= R
        let mut sink_terms = Vec::new();
        for (arc_idx, &(u, v)) in arcs.iter().enumerate() {
            if v == self.sink() {
                sink_terms.push((arc_idx, 1.0)); // incoming
            }
            if u == self.sink() {
                sink_terms.push((arc_idx, -1.0)); // outgoing
            }
        }
        constraints.push(LinearConstraint::ge(sink_terms, self.requirement() as f64));

        ReductionIFWMToILP {
            target: ILP::new(num_arcs, constraints, vec![], ObjectiveSense::Minimize),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
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
            let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
                source,
                SolutionPair {
                    source_config,
                    target_config: ilp_solution,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integralflowwithmultipliers_ilp.rs"]
mod tests;
