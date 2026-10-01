//! Reduction from IntegralFlowHomologousArcs to ILP.
//!
//! One integer flow variable per arc. Capacity bounds, conservation at
//! non-terminals, homologous-pair equality, and sink inflow requirement.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::IntegralFlowHomologousArcs;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing IntegralFlowHomologousArcs to ILP.
#[derive(Debug, Clone)]
pub struct ReductionIFHAToILP {
    target: ILP<i64, i64, Bounded>,
}

impl ReductionResult for ReductionIFHAToILP {
    type Source = IntegralFlowHomologousArcs;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;

        crate::rules::ilp_helpers::decode_usize_values(target_solution)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionIFHAToILP {}

// Capacity rows contribute m terms; conservation plus sink balance use at
// most 2m; each declared homologous pair contributes at most two. Duplicate
// pairs retain separate rows, while self-loops and (a,a) pairs cancel terms.
// The clamped requirement has magnitude at most sum(capacities)+1, whose
// bits are at most h+bit_length(m) <= h+ceil(m/2+1); m=0 is also covered.
#[reduction(
    transform = upper_bound {
        max_constraint_magnitude_bits = "max_capacity_bits + num_arcs / 2 + 1",
        num_vars = "num_arcs",
        num_constraints = "num_arcs + num_vertices + num_homologous_pairs",
        num_nonzeros = "3 * num_arcs + 2 * num_homologous_pairs",
    },
)]
impl ReduceTo<ILP<i64, i64, Bounded>> for IntegralFlowHomologousArcs {
    type Result = ReductionIFHAToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let arcs = self.graph().arcs();
        let num_vertices = self.num_vertices();
        let mut constraints = Vec::new();

        // Capacity: f_a <= c_a for each arc
        for (arc_idx, &capacity) in self.capacities().iter().enumerate() {
            constraints.push(LinearConstraint::le(vec![(arc_idx, 1)], capacity));
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
                    terms.push((arc_idx, 1)); // incoming
                }
                if u == vertex {
                    terms.push((arc_idx, -1)); // outgoing
                }
            }
            constraints.push(LinearConstraint::eq(terms, 0));
        }

        // Homologous equality: f_a = f_b for each pair (a, b)
        for &(a, b) in self.homologous_pairs() {
            constraints.push(LinearConstraint::eq(vec![(a, 1), (b, -1)], 0));
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
        constraints.push(LinearConstraint::ge(
            sink_terms,
            crate::rules::ilp_helpers::bounded_flow_requirement(
                self.requirement(),
                self.capacities().iter().copied(),
            ),
        ));

        let variables = self
            .capacities()
            .iter()
            .copied()
            .map(|capacity| IntegerVariable::new(Some(0), Some(capacity)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::target_construction)?;

        Ok(ReductionIFHAToILP {
            target: ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
        })
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
            crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integralflowhomologousarcs_ilp.rs"]
mod tests;
