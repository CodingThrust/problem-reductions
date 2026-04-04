//! Reduction from ResourceConstrainedScheduling to ILP<bool>.
//!
//! Time-indexed binary formulation: x_{j,t} = 1 iff task j runs in slot t.
//! Each task in exactly one slot; processor capacity and resource bounds
//! enforced per time slot.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ResourceConstrainedScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ResourceConstrainedScheduling to ILP<bool>.
///
/// Variable layout: x_{j,t} at index `j * D + t`
/// for j in 0..n, t in 0..D.
#[derive(Debug, Clone)]
pub struct ReductionRCSToILP {
    target: ILP<bool>,
    num_tasks: usize,
    deadline: usize,
}

impl ReductionResult for ReductionRCSToILP {
    type Source = ResourceConstrainedScheduling;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract: for each task j, find the unique slot t with x_{j,t} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let d = self.deadline;
        (0..self.num_tasks)
            .map(|j| {
                (0..d)
                    .find(|&t| target_solution.get(j * d + t).copied().unwrap_or(0) == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_vars = "num_tasks * deadline",
    num_constraints = "num_tasks + deadline + num_resources * deadline",
})]
impl ReduceTo<ILP<bool>> for ResourceConstrainedScheduling {
    type Result = ReductionRCSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let d = self.deadline() as usize;
        let r = self.num_resources();
        let m = self.num_processors();
        let num_vars = n * d;

        let var = |j: usize, t: usize| -> usize { j * d + t };

        let mut constraints = Vec::new();

        // 1. Each task in exactly one slot: Σ_t x_{j,t} = 1 for all j
        for j in 0..n {
            let terms: Vec<(usize, f64)> = (0..d).map(|t| (var(j, t), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Processor capacity: Σ_j x_{j,t} <= m for each time slot t
        for t in 0..d {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (var(j, t), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, m as f64));
        }

        // 3. Resource bounds: Σ_j r_{j,q} * x_{j,t} <= B_q for all q, t
        for q in 0..r {
            for t in 0..d {
                let terms: Vec<(usize, f64)> = (0..n)
                    .map(|j| (var(j, t), self.resource_requirements()[j][q] as f64))
                    .collect();
                constraints.push(LinearConstraint::le(
                    terms,
                    self.resource_bounds()[q] as f64,
                ));
            }
        }

        ReductionRCSToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_tasks: n,
            deadline: d,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "resourceconstrainedscheduling_to_ilp",
        build: || {
            // 6 tasks, 3 processors, 1 resource with bound 20, deadline 2
            let source = ResourceConstrainedScheduling::new(
                3,
                vec![20],
                vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
                2,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/resourceconstrainedscheduling_ilp.rs"]
mod tests;
