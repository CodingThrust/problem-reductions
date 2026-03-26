//! Reduction from CapacityAssignment to ILP (Integer Linear Programming).
//!
//! The Capacity Assignment optimization problem can be formulated as a binary ILP:
//! - Variables: Binary x_{l,c} (link l gets capacity c), one-hot per link
//! - Constraints: Σ_c x_{l,c} = 1 for each link l (assignment);
//!   Σ_{l,c} delay[l][c]·x_{l,c} ≤ delay_budget
//! - Objective: Minimize Σ_{l,c} cost[l][c]·x_{l,c}
//! - Extraction: argmax_c x_{l,c} for each link l

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::CapacityAssignment;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing CapacityAssignment to ILP.
///
/// Variable layout: x_{l,c} at index l * num_capacities + c.
/// - l ∈ 0..num_links, c ∈ 0..num_capacities
///
/// Total: num_links * num_capacities variables.
#[derive(Debug, Clone)]
pub struct ReductionCAToILP {
    target: ILP<bool>,
    num_links: usize,
    num_capacities: usize,
}

impl ReductionResult for ReductionCAToILP {
    type Source = CapacityAssignment;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution: for each link l, find the unique capacity c where x_{l,c} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_capacities = self.num_capacities;
        (0..self.num_links)
            .map(|l| {
                (0..num_capacities)
                    .find(|&c| target_solution[l * num_capacities + c] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_links * num_capacities",
        num_constraints = "num_links + 1",
    }
)]
impl ReduceTo<ILP<bool>> for CapacityAssignment {
    type Result = ReductionCAToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_links = self.num_links();
        let num_capacities = self.num_capacities();
        let num_vars = num_links * num_capacities;

        let mut constraints = Vec::with_capacity(num_links + 1);

        // Assignment constraints: for each link l, Σ_c x_{l,c} = 1
        for l in 0..num_links {
            let terms: Vec<(usize, f64)> = (0..num_capacities)
                .map(|c| (l * num_capacities + c, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Delay budget constraint: Σ_{l,c} delay[l][c] * x_{l,c} ≤ delay_budget
        let delay_terms: Vec<(usize, f64)> = (0..num_links)
            .flat_map(|l| {
                (0..num_capacities)
                    .map(move |c| (l * num_capacities + c, self.delay()[l][c] as f64))
            })
            .collect();
        constraints.push(LinearConstraint::le(
            delay_terms,
            self.delay_budget() as f64,
        ));

        // Objective: minimize total cost
        let objective: Vec<(usize, f64)> = (0..num_links)
            .flat_map(|l| {
                (0..num_capacities).map(move |c| (l * num_capacities + c, self.cost()[l][c] as f64))
            })
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionCAToILP {
            target,
            num_links,
            num_capacities,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "capacityassignment_to_ilp",
        build: || {
            // 2 links, 2 capacity levels
            // cost: [[1,3],[2,4]], delay: [[8,4],[7,3]]
            // delay_budget=12
            // Minimize cost subject to total_delay ≤ 12.
            // link 0 → cap 0, link 1 → cap 0: cost=3, delay=15 > 12 — infeasible
            // link 0 → cap 1, link 1 → cap 0: cost=5, delay=11 ≤ 12 — feasible
            // link 0 → cap 0, link 1 → cap 1: cost=5, delay=11 ≤ 12 — feasible (tied)
            // link 0 → cap 1, link 1 → cap 1: cost=7, delay=7 ≤ 12 — feasible
            // Optimal: cost=5 at [1,0] or [0,1]
            let source = CapacityAssignment::new(
                vec![1, 2],
                vec![vec![1, 3], vec![2, 4]],
                vec![vec![8, 4], vec![7, 3]],
                12,
            );
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
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
#[path = "../unit_tests/rules/capacityassignment_ilp.rs"]
mod tests;
