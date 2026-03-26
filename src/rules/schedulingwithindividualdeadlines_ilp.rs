//! Reduction from SchedulingWithIndividualDeadlines to ILP<bool>.
//!
//! Uses a time-indexed binary formulation with per-task deadline windows:
//! - Variables: Binary x_{j,t} where x_{j,t} = 1 iff task j is scheduled at time slot t,
//!   for t in 0..max_deadline (slots beyond each task's deadline are zero-fixed).
//! - Variable index: j * max_deadline + t  for j in 0..num_tasks, t in 0..max_deadline
//! - Constraints:
//!   1. One-hot: Σ_{t<d_j} x_{j,t} = 1 for each task j (using only valid slots)
//!   2. Zero beyond deadline: x_{j,t} = 0 for t >= d_j
//!   3. Capacity: Σ_j x_{j,t} ≤ m for each time slot t
//!   4. Precedence: Σ_t t·x_{j,t} ≥ Σ_t t·x_{i,t} + 1 for each (i,j)
//! - Objective: Minimize 0 (feasibility)

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SchedulingWithIndividualDeadlines;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SchedulingWithIndividualDeadlines to ILP<bool>.
///
/// Variable layout: x_{j,t} at index j * max_deadline + t
/// for j in 0..num_tasks, t in 0..max_deadline.
#[derive(Debug, Clone)]
pub struct ReductionSWIDToILP {
    target: ILP<bool>,
    num_tasks: usize,
    max_deadline: usize,
}

impl ReductionResult for ReductionSWIDToILP {
    type Source = SchedulingWithIndividualDeadlines;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract schedule from ILP solution.
    ///
    /// For each task j, find the time slot t where x_{j,t} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let d = self.max_deadline;
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
        num_vars = "num_tasks * max_deadline",
        num_constraints = "num_tasks + max_deadline + num_precedences",
    }
)]
impl ReduceTo<ILP<bool>> for SchedulingWithIndividualDeadlines {
    type Result = ReductionSWIDToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let m = self.num_processors();
        let max_d = self.max_deadline();
        let num_vars = n * max_d;

        let var = |j: usize, t: usize| j * max_d + t;

        let mut constraints = Vec::new();

        // 1. One-hot: for each task j, sum over valid slots 0..d_j equals 1
        for j in 0..n {
            let dj = self.deadlines()[j];
            let terms: Vec<(usize, f64)> = (0..dj).map(|t| (var(j, t), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Capacity: Σ_j x_{j,t} ≤ m for each time slot t
        for t in 0..max_d {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (var(j, t), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, m as f64));
        }

        // 3. Precedence: Σ_t t·x_{j,t} - Σ_t t·x_{i,t} ≥ 1 for each (i,j)
        for &(i, j) in self.precedences() {
            let di = self.deadlines()[i];
            let dj = self.deadlines()[j];
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for t in 0..dj {
                terms.push((var(j, t), t as f64));
            }
            for t in 0..di {
                terms.push((var(i, t), -(t as f64)));
            }
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        ReductionSWIDToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_tasks: n,
            max_deadline: max_d,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
        vec![crate::example_db::specs::RuleExampleSpec {
        id: "schedulingwithindividualdeadlines_to_ilp",
        build: || {
            // 3 tasks, 2 processors, deadlines [2, 2, 3], precedence (0, 2)
            let source = SchedulingWithIndividualDeadlines::new(3, 2, vec![2, 2, 3], vec![(0, 2)]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/schedulingwithindividualdeadlines_ilp.rs"]
mod tests;
