//! Reduction from CapacityAssignment to ILP (Integer Linear Programming).
//!
//! The Capacity Assignment feasibility problem can be formulated as a binary ILP:
//! - Variables: Binary x_{l,c} (link l gets capacity c), one-hot per link
//! - Constraints: Σ_c x_{l,c} = 1 for each link l (assignment);
//!   Σ_{l,c} cost[l][c]·x_{l,c} ≤ cost_budget; Σ_{l,c} delay[l][c]·x_{l,c} ≤ delay_budget
//! - Objective: Minimize 0 (feasibility)
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
        num_constraints = "num_links + 2",
    }
)]
impl ReduceTo<ILP<bool>> for CapacityAssignment {
    type Result = ReductionCAToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_links = self.num_links();
        let num_capacities = self.num_capacities();
        let num_vars = num_links * num_capacities;

        let mut constraints = Vec::with_capacity(num_links + 2);

        // Assignment constraints: for each link l, Σ_c x_{l,c} = 1
        for l in 0..num_links {
            let terms: Vec<(usize, f64)> = (0..num_capacities)
                .map(|c| (l * num_capacities + c, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Cost budget constraint: Σ_{l,c} cost[l][c] * x_{l,c} ≤ cost_budget
        let cost_terms: Vec<(usize, f64)> = (0..num_links)
            .flat_map(|l| {
                (0..num_capacities).map(move |c| (l * num_capacities + c, self.cost()[l][c] as f64))
            })
            .collect();
        constraints.push(LinearConstraint::le(cost_terms, self.cost_budget() as f64));

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

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

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
            // budgets: cost=5, delay=12
            // Solution: link 0 → cap 0 (cost=1, delay=8), link 1 → cap 0 (cost=2, delay=7)
            // total cost=3 ≤ 5, total delay=15 > 12 -- try cap 1 for both
            // link 0 → cap 1 (cost=3, delay=4), link 1 → cap 1 (cost=4, delay=3)
            // total cost=7 > 5 -- try mixed: link 0 → cap 0, link 1 → cap 0: cost=3≤5, delay=15>12
            // link 0 → cap 1 (cost=3, delay=4), link 1 → cap 0 (cost=2, delay=7): cost=5≤5, delay=11≤12
            let source = CapacityAssignment::new(
                vec![1, 2],
                vec![vec![1, 3], vec![2, 4]],
                vec![vec![8, 4], vec![7, 3]],
                5,
                12,
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    // link 0 → cap 1, link 1 → cap 0
                    source_config: vec![1, 0],
                    // x_{0,0}=0, x_{0,1}=1, x_{1,0}=1, x_{1,1}=0
                    target_config: vec![0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/capacityassignment_ilp.rs"]
mod tests;
