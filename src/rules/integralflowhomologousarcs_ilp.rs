//! Reduction from IntegralFlowHomologousArcs to ILP.
//!
//! One integer flow variable per arc. Capacity bounds, conservation at
//! non-terminals, homologous-pair equality, and sink inflow requirement.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::IntegralFlowHomologousArcs;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing IntegralFlowHomologousArcs to ILP.
#[derive(Debug, Clone)]
pub struct ReductionIFHAToILP {
    target: ILP<i32>,
}

impl ReductionResult for ReductionIFHAToILP {
    type Source = IntegralFlowHomologousArcs;
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
        num_constraints = "num_arcs + num_vertices - 2 + 1",
    }
)]
impl ReduceTo<ILP<i32>> for IntegralFlowHomologousArcs {
    type Result = ReductionIFHAToILP;

    fn reduce_to(&self) -> Self::Result {
        let arcs = self.graph().arcs();
        let num_arcs = self.num_arcs();
        let num_vertices = self.num_vertices();
        let mut constraints = Vec::new();

        // Capacity: f_a <= c_a for each arc
        for (arc_idx, &capacity) in self.capacities().iter().enumerate() {
            constraints.push(LinearConstraint::le(vec![(arc_idx, 1.0)], capacity as f64));
        }

        // Conservation: sum_{a in delta^-(v)} f_a = sum_{a in delta^+(v)} f_a
        // for all v in V \ {s, t}
        for vertex in 0..num_vertices {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }
            let mut terms = Vec::new();
            for (arc_idx, &(u, v)) in arcs.iter().enumerate() {
                if v == vertex {
                    terms.push((arc_idx, 1.0)); // incoming
                }
                if u == vertex {
                    terms.push((arc_idx, -1.0)); // outgoing
                }
            }
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // Homologous equality: f_a = f_b for each pair (a, b)
        for &(a, b) in self.homologous_pairs() {
            constraints.push(LinearConstraint::eq(vec![(a, 1.0), (b, -1.0)], 0.0));
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

        ReductionIFHAToILP {
            target: ILP::new(num_arcs, constraints, vec![], ObjectiveSense::Minimize),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "integralflowhomologousarcs_to_ilp",
        build: || {
            let source = IntegralFlowHomologousArcs::new(
                DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
                vec![2, 2, 2, 2],
                0,
                3,
                2,
                vec![(0, 1)],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integralflowhomologousarcs_ilp.rs"]
mod tests;
