//! Reduction from PrecedenceConstrainedScheduling to ILP<bool>.
//!
//! Uses a time-indexed binary formulation:
//! - Variables: Binary x_{j,t} where x_{j,t} = 1 iff task j is scheduled at time slot t.
//! - Variable index: j * deadline + t  for j in 0..num_tasks, t in 0..deadline
//! - Constraints:
//!   1. One-hot: Σ_t x_{j,t} = 1 for each task j
//!   2. Capacity: Σ_j x_{j,t} ≤ m for each time slot t
//!   3. Precedence: Σ_t t·x_{j,t} ≥ Σ_t t·x_{i,t} + 1 for each (i,j) in precedences
//! - Objective: Minimize 0 (feasibility)
//! - Extraction: For each task j, find argmax_t x_{j,t}

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::PrecedenceConstrainedScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing PrecedenceConstrainedScheduling to ILP<bool>.
///
/// Variable layout: x_{j,t} at index j * deadline + t
/// for j in 0..num_tasks, t in 0..deadline.
#[derive(Debug, Clone)]
pub struct ReductionPCSToILP {
    target: ILP<bool>,
    num_tasks: usize,
    deadline: usize,
}

impl ReductionResult for ReductionPCSToILP {
    type Source = PrecedenceConstrainedScheduling;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract schedule from ILP solution.
    ///
    /// For each task j, find the time slot t where x_{j,t} = 1.
    /// Returns the time slot for each task (matching the `dims()` encoding of PCS).
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

#[reduction(
    overhead = {
        num_vars = "num_tasks * deadline",
        num_constraints = "num_tasks + deadline + num_tasks^2",
    }
)]
impl ReduceTo<ILP<bool>> for PrecedenceConstrainedScheduling {
    type Result = ReductionPCSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let m = self.num_processors();
        let d = self.deadline();
        let num_vars = n * d;

        // x_{j,t} variable index
        let var = |j: usize, t: usize| j * d + t;

        let mut constraints = Vec::new();

        // 1. One-hot: Σ_t x_{j,t} = 1 for each task j
        for j in 0..n {
            let terms: Vec<(usize, f64)> = (0..d).map(|t| (var(j, t), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Capacity: Σ_j x_{j,t} ≤ m for each time slot t
        for t in 0..d {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (var(j, t), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, m as f64));
        }

        // 3. Precedence: Σ_t t·x_{j,t} ≥ Σ_t t·x_{i,t} + 1 for each (i,j)
        // Rearranged: Σ_t t·x_{j,t} - Σ_t t·x_{i,t} ≥ 1
        for &(i, j) in self.precedences() {
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for t in 0..d {
                terms.push((var(j, t), t as f64));
                terms.push((var(i, t), -(t as f64)));
            }
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        ReductionPCSToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_tasks: n,
            deadline: d,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "precedenceconstrainedscheduling_to_ilp",
        build: || {
            // 3 tasks, 2 processors, deadline 2, with task 0 < task 2
            // Schedule: task 0 and 1 at slot 0, task 2 at slot 1
            // Variables: x_{0,0}=1, x_{0,1}=0, x_{1,0}=1, x_{1,1}=0, x_{2,0}=0, x_{2,1}=1
            let source = PrecedenceConstrainedScheduling::new(3, 2, 2, vec![(0, 2)]);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 1],
                    target_config: vec![1, 0, 1, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/precedenceconstrainedscheduling_ilp.rs"]
mod tests;
