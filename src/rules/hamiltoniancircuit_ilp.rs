//! Degree two and a single bounded flow enforce a spanning cycle.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::HamiltonianCircuit;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

type Target = ILP<i64, i64, Bounded>;

#[derive(Debug, Clone)]
pub struct ReductionHamiltonianCircuitToILP {
    target: Target,
    graph: SimpleGraph,
}

impl ReductionResult for ReductionHamiltonianCircuitToILP {
    type Source = HamiltonianCircuit<SimpleGraph>;
    type Target = Target;

    fn target_problem(&self) -> &Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        let selected: Vec<_> = solution[..self.graph.num_edges()]
            .iter()
            .map(|&value| value == 1)
            .collect();
        crate::rules::graph_helpers::edges_to_cycle_order(&self.graph, &selected)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionHamiltonianCircuitToILP {}

#[crate::reduction(transform = upper_bound {
    num_vars = "3 * num_edges",
    num_constraints = "2 * num_vertices + 2 * num_edges + 1",
    num_nonzeros = "10 * num_edges",
    max_constraint_magnitude_bits = "num_vertices + 2",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for HamiltonianCircuit<SimpleGraph> {
    type Result = ReductionHamiltonianCircuitToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let mut edges: Vec<_> = self
            .graph()
            .edges()
            .into_iter()
            .filter(|(u, v)| u != v)
            .map(|(u, v)| (u.min(v), u.max(v)))
            .collect();
        edges.sort_unstable();
        edges.dedup();
        let m = edges.len();
        let count =
            m.checked_mul(3).ok_or(
                crate::rules::ReductionError::integer_overflow::<Self, Target>(
                    "counting circuit flow variables",
                ),
            )?;
        let capacity = <Self as ReduceTo<Target>>::exact_i64(
            n.saturating_sub(1),
            "bounding spanning-cycle flow",
        )?;
        let mut variables = vec![IntegerVariable::binary(); m];
        variables.resize(
            count,
            IntegerVariable::new(Some(0), Some(capacity))
                .map_err(<Self as ReduceTo<Target>>::target_construction)?,
        );
        let mut degree = vec![Vec::new(); n];
        let mut balance = vec![Vec::new(); n];
        let mut constraints = Vec::new();
        for (edge, &(u, v)) in edges.iter().enumerate() {
            degree[u].push((edge, 1));
            degree[v].push((edge, 1));
            for (direction, (from, to)) in [(u, v), (v, u)].into_iter().enumerate() {
                let flow = m + 2 * edge + direction;
                balance[from].push((flow, 1));
                balance[to].push((flow, -1));
                constraints.push(LinearConstraint::le(vec![(flow, 1), (edge, -capacity)], 0));
            }
        }
        for (v, (degree, balance)) in degree.into_iter().zip(balance).enumerate() {
            constraints.push(LinearConstraint::eq(degree, 2));
            constraints.push(LinearConstraint::eq(
                balance,
                if v == 0 { capacity } else { -1 },
            ));
        }
        if n < 3 {
            constraints.push(LinearConstraint::eq(vec![], 1));
        }
        let target =
            Target::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(<Self as ReduceTo<Target>>::target_construction)?;
        Ok(ReductionHamiltonianCircuitToILP {
            target,
            graph: SimpleGraph::new(n, edges),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltoniancircuit_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(HamiltonianCircuit::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
            ))
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltoniancircuit_ilp.rs"]
mod tests;
