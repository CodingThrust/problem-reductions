//! Reduction from MinimumEdgeCostFlow to `ILP<i64, i64, Bounded>`.
//!
//! Variables (2m total):
//!   f_a  (a = 0..m-1)  — integer flow on arc a, domain {0, ..., c(a)}
//!   y_a  (a = m..2m-1) — binary indicator: y_a = 1 iff f_a > 0
//!
//! Constraints:
//!   f_a ≤ c(a) · y_a    — linking: forces y_a = 1 when f_a > 0 (m constraints)
//!   y_a ≤ 1             — binary bound on indicators (m constraints)
//!   conservation at non-isolated non-terminal vertices (at most |V|-2 rows)
//!   net flow into sink ≥ R (1 constraint)
//!
//! Total: at most 2m + |V| - 1 constraints. Capacity is also a variable bound.
//!
//! Objective: minimize Σ p(a) · y_a.
//! Extraction: first m variables are the flow values.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumEdgeCostFlow;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumEdgeCostFlow to `ILP<i64, i64, Bounded>`.
///
/// Variable layout:
/// - `f_a` at index a for a in 0..num_edges (flow on arc a)
/// - `y_a` at index num_edges + a for a in 0..num_edges (binary indicator)
#[derive(Debug, Clone)]
pub struct ReductionMECFToILP {
    target: ILP<i64, i64, Bounded>,
    num_edges: usize,
}

impl ReductionResult for ReductionMECFToILP {
    type Source = MinimumEdgeCostFlow;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    /// Extract flow solution: first m variables are the flow values.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        crate::rules::ilp_helpers::decode_usize_values(&target_solution[..self.num_edges])
    }
}

// Linking and indicator rows contribute at most 3m nonzeros. Conservation
// and the sink row together use each arc at most once per endpoint: at most 2m.
#[reduction(transform = {
    exact {
        num_vars = "2 * num_edges",
    },
    upper_bound {
        max_constraint_magnitude_bits = "max_capacity * (num_edges + 1) + 2",
        num_constraints = "2 * num_edges + num_vertices - 1",
        num_nonzeros = "5 * num_edges",
    },
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for MinimumEdgeCostFlow {
    type Result = ReductionMECFToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let arcs = self.graph().arcs();
        let m = arcs.len();
        let n = self.num_vertices();
        let num_vars = 2 * m;

        let f = |a: usize| a; // flow variable index
        let y = |a: usize| m + a; // indicator variable index

        let mut constraints = Vec::new();

        // 1. Linking: f_a - c(a) * y_a ≤ 0  (forces y_a = 1 when f_a > 0)
        for a in 0..m {
            constraints.push(LinearConstraint::le(
                vec![(f(a), 1), (y(a), -self.capacities()[a])],
                0,
            ));
        }

        // 2. Binary bound: y_a ≤ 1
        for a in 0..m {
            constraints.push(LinearConstraint::le(vec![(y(a), 1)], 1));
        }

        // 3. Flow conservation at non-terminal vertices
        for vertex in 0..n {
            if vertex == self.source() || vertex == self.sink() {
                continue;
            }

            let mut terms: Vec<(usize, i64)> = Vec::new();
            for (a, &(u, v)) in arcs.iter().enumerate() {
                if vertex == u {
                    terms.push((f(a), -1)); // outgoing
                } else if vertex == v {
                    terms.push((f(a), 1)); // incoming
                }
            }

            if !terms.is_empty() {
                constraints.push(LinearConstraint::eq(terms, 0));
            }
        }

        // 4. Flow requirement: net flow into sink ≥ R
        let sink = self.sink();
        let mut sink_terms: Vec<(usize, i64)> = Vec::new();
        for (a, &(u, v)) in arcs.iter().enumerate() {
            if v == sink {
                sink_terms.push((f(a), 1));
            } else if u == sink {
                sink_terms.push((f(a), -1));
            }
        }
        constraints.push(LinearConstraint::ge(
            sink_terms,
            crate::rules::ilp_helpers::bounded_flow_requirement(
                self.required_flow(),
                self.capacities().iter().copied(),
            ),
        ));

        // Objective: minimize Σ p(a) · y_a
        let objective: Vec<(usize, i64)> = (0..m).map(|a| (y(a), self.prices()[a])).collect();

        let mut variables = self
            .capacities()
            .iter()
            .map(|&capacity| IntegerVariable::new(Some(0), Some(capacity)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::target_construction)?;
        variables.resize(num_vars, IntegerVariable::binary());

        Ok(ReductionMECFToILP {
            target: ILP::with_variables(
                variables,
                constraints,
                objective,
                ObjectiveSense::Minimize,
            )
            .map_err(Self::target_construction)?,
            num_edges: m,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumedgecostflow_to_ilp",
        build: || {
            let source = MinimumEdgeCostFlow::new(
                DirectedGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4)]),
                vec![3, 1, 2, 0, 0, 0],
                vec![2, 2, 2, 2, 2, 2],
                0,
                4,
                3,
            );
            crate::example_db::specs::rule_example_via_bounded_ilp::<_>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumedgecostflow_ilp.rs"]
mod tests;
