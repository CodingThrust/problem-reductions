//! A permutation matrix with direct consecutive-position adjacency constraints.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::HamiltonianPath;
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_assignment_constraints, one_hot_decode};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Binary `x[v,p]` at index `v * n + p` selects vertex v at path position p.
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianPathToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianPathToILP {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
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

        one_hot_decode(target_solution, self.num_vertices, self.num_vertices, 0)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionHamiltonianPathToILP {}

#[reduction(transform = {
    exact {
        max_constraint_magnitude_bits = "1",
        num_vars = "num_vertices^2",
        num_constraints = "2 * num_vertices + num_vertices * num_consecutive_positions",
    },
    upper_bound {
        num_nonzeros = "2 * num_vertices^2 + num_consecutive_positions * (num_vertices + 2 * num_edges)",
    },
})]
impl ReduceTo<ILP<bool>> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHamiltonianPathToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let num_vars = n.checked_mul(n).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(
                "counting Hamiltonian permutation variables",
            )
        })?;
        <Self as ReduceTo<ILP<bool>>>::exact_i64(n, "bounding adjacency row sums")?;
        let mut neighbors = vec![Vec::new(); n];
        for (u, v) in self.graph().edges() {
            if u != v {
                neighbors[u].push(v);
                neighbors[v].push(u);
            }
        }
        let mut constraints = one_hot_assignment_constraints(n, n, 0);
        for (v, adjacent) in neighbors.iter_mut().enumerate() {
            adjacent.sort_unstable();
            adjacent.dedup();
            for p in 0..self.num_consecutive_positions() {
                let mut terms = vec![(v * n + p, 1)];
                terms.extend(adjacent.iter().map(|&w| (w * n + p + 1, -1)));
                constraints.push(LinearConstraint::le(terms, 0));
            }
        }

        // Feasibility: no objective
        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;

        Ok(ReductionHamiltonianPathToILP {
            target,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_ilp",
        build: || {
            // Path graph: 0-1-2-3 (has Hamiltonian path)
            let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_ilp.rs"]
mod tests;
