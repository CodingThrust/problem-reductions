//! Reduction from SchedulingToMinimizeWeightedCompletionTime to ILP.
//!
//! The reduction uses binary assignment variables `x_{t,p}` (task t on
//! processor p), integer completion-time variables `C_t`, and binary
//! ordering variables `y_{i,j}` for each task pair. Big-M constraints
//! enforce that tasks sharing a processor do not overlap.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SchedulingToMinimizeWeightedCompletionTime;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SchedulingToMinimizeWeightedCompletionTime to ILP.
///
/// Variable layout:
/// - `x_{t,p}` at index `t * m + p` for t in 0..n, p in 0..m
/// - `C_t` at index `n * m + t` for t in 0..n (completion times)
/// - `y_{i,j}` at index `n * m + n + pair_index(i,j)` for i < j
///   (1 if task i is before task j on their shared processor)
///
/// Total variables: n*m + n + n*(n-1)/2
#[derive(Debug, Clone)]
pub struct ReductionSMWCTToILP {
    target: ILP<i32>,
    num_tasks: usize,
    num_processors: usize,
}

impl ReductionSMWCTToILP {
    fn x_var(&self, task: usize, processor: usize) -> usize {
        task * self.num_processors + processor
    }

    fn c_var(&self, task: usize) -> usize {
        self.num_tasks * self.num_processors + task
    }

    fn y_var(&self, i: usize, j: usize) -> usize {
        debug_assert!(i < j);
        let base = self.num_tasks * self.num_processors + self.num_tasks;
        base + i * (2 * self.num_tasks - i - 1) / 2 + (j - i - 1)
    }
}

impl ReductionResult for ReductionSMWCTToILP {
    type Source = SchedulingToMinimizeWeightedCompletionTime;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract solution: for each task, find the processor with x_{t,p} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_tasks)
            .map(|t| {
                (0..self.num_processors)
                    .find(|&p| target_solution[self.x_var(t, p)] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_tasks * num_processors + num_tasks + num_tasks * (num_tasks - 1) / 2",
        num_constraints = "num_tasks + num_tasks * num_processors + 2 * num_tasks + 2 * num_tasks * (num_tasks - 1) / 2 * num_processors + num_tasks * (num_tasks - 1) / 2",
    }
)]
impl ReduceTo<ILP<i32>> for SchedulingToMinimizeWeightedCompletionTime {
    type Result = ReductionSMWCTToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let m = self.num_processors();

        let total_processing_time: u64 = self.lengths().iter().sum();
        let big_m = total_processing_time as f64;

        let num_pairs = n * n.saturating_sub(1) / 2;
        let num_vars = n * m + n + num_pairs;

        let result = ReductionSMWCTToILP {
            target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
            num_tasks: n,
            num_processors: m,
        };

        let mut constraints = Vec::new();

        // 1. Assignment constraints: each task assigned to exactly one processor
        // sum_p x_{t,p} = 1 for each t
        for t in 0..n {
            let terms: Vec<(usize, f64)> = (0..m).map(|p| (result.x_var(t, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Binary bounds on x_{t,p}: 0 <= x_{t,p} <= 1
        for t in 0..n {
            for p in 0..m {
                constraints.push(LinearConstraint::le(vec![(result.x_var(t, p), 1.0)], 1.0));
            }
        }

        // 3. Completion time bounds: l_t <= C_t <= M
        for t in 0..n {
            constraints.push(LinearConstraint::ge(
                vec![(result.c_var(t), 1.0)],
                self.lengths()[t] as f64,
            ));
            constraints.push(LinearConstraint::le(vec![(result.c_var(t), 1.0)], big_m));
        }

        // 4. Disjunctive constraints: for each pair (i,j) with i < j, on each processor p:
        //    If both tasks are on processor p and y_{i,j}=1 (i before j):
        //      C_j >= C_i + l_j - M*(2 - x_{i,p} - x_{j,p}) - M*(1 - y_{i,j})
        //    If both on p and y_{i,j}=0 (j before i):
        //      C_i >= C_j + l_i - M*(2 - x_{i,p} - x_{j,p}) - M*y_{i,j}
        //
        // Rearranged:
        //   C_j - C_i + M*x_{i,p} + M*x_{j,p} + M*y_{i,j} >= l_j - 3M + M
        //     => C_j - C_i + M*x_{i,p} + M*x_{j,p} + M*y_{i,j} >= l_j - 2M
        //   C_i - C_j + M*x_{i,p} + M*x_{j,p} - M*y_{i,j} >= l_i - 2M - M + M
        //     => C_i - C_j + M*x_{i,p} + M*x_{j,p} - M*y_{i,j} >= l_i - 3M

        for i in 0..n {
            for j in (i + 1)..n {
                let y = result.y_var(i, j);
                let ci = result.c_var(i);
                let cj = result.c_var(j);
                let li = self.lengths()[i] as f64;
                let lj = self.lengths()[j] as f64;

                for p in 0..m {
                    let xip = result.x_var(i, p);
                    let xjp = result.x_var(j, p);

                    // If i before j on processor p: C_j >= C_i + l_j
                    // C_j - C_i + M*(1-y) + M*(1-x_{i,p}) + M*(1-x_{j,p}) >= l_j
                    // C_j - C_i - M*y - M*x_{i,p} - M*x_{j,p} >= l_j - 3M
                    constraints.push(LinearConstraint::ge(
                        vec![
                            (cj, 1.0),
                            (ci, -1.0),
                            (y, -big_m),
                            (xip, -big_m),
                            (xjp, -big_m),
                        ],
                        lj - 3.0 * big_m,
                    ));

                    // If j before i on processor p: C_i >= C_j + l_i
                    // C_i - C_j + M*y + M*(1-x_{i,p}) + M*(1-x_{j,p}) >= l_i
                    // C_i - C_j + M*y - M*x_{i,p} - M*x_{j,p} >= l_i - 2M
                    constraints.push(LinearConstraint::ge(
                        vec![
                            (ci, 1.0),
                            (cj, -1.0),
                            (y, big_m),
                            (xip, -big_m),
                            (xjp, -big_m),
                        ],
                        li - 2.0 * big_m,
                    ));
                }

                // Binary bound on y_{i,j}: 0 <= y <= 1
                constraints.push(LinearConstraint::le(vec![(y, 1.0)], 1.0));
            }
        }

        // Objective: minimize sum_t w_t * C_t
        let objective: Vec<(usize, f64)> = (0..n)
            .map(|t| (result.c_var(t), self.weights()[t] as f64))
            .collect();

        ReductionSMWCTToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_tasks: n,
            num_processors: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "schedulingtominimizeweightedcompletiontime_to_ilp",
        build: || {
            // 3 tasks, 2 processors: simple instance for canonical example
            let source =
                SchedulingToMinimizeWeightedCompletionTime::new(vec![1, 2, 3], vec![4, 2, 1], 2);
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/schedulingtominimizeweightedcompletiontime_ilp.rs"]
mod tests;
