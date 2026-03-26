//! Reduction from SequencingToMinimizeWeightedTardiness to ILP<i32>.
//!
//! Pairwise order variables y_{i,j}, integer completion times C_j,
//! and nonnegative tardiness variables T_j. Big-M disjunctive constraints
//! force a single-machine order; the weighted tardiness sum is bounded by K.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingToMinimizeWeightedTardiness;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SequencingToMinimizeWeightedTardiness to ILP<i32>.
///
/// Variable layout:
/// - `y_{i,j}` for i < j: pairwise order bits (n*(n-1)/2 vars)
/// - `C_j` for j in 0..n: completion times (n vars)
/// - `T_j` for j in 0..n: tardiness (n vars)
///
/// Total: n*(n-1)/2 + 2*n variables.
#[derive(Debug, Clone)]
pub struct ReductionSTMWTToILP {
    target: ILP<i32>,
    num_tasks: usize,
    num_order_vars: usize,
}

impl ReductionSTMWTToILP {
    fn encode_schedule_as_lehmer(schedule: &[usize]) -> Vec<usize> {
        let mut available: Vec<usize> = (0..schedule.len()).collect();
        let mut config = Vec::with_capacity(schedule.len());
        for &task in schedule {
            let digit = available
                .iter()
                .position(|&c| c == task)
                .expect("schedule must be a permutation");
            config.push(digit);
            available.remove(digit);
        }
        config
    }
}

impl ReductionResult for ReductionSTMWTToILP {
    type Source = SequencingToMinimizeWeightedTardiness;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract: sort jobs by completion time C_j, convert to Lehmer code.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_tasks;
        let c_offset = self.num_order_vars;
        let mut jobs: Vec<usize> = (0..n).collect();
        jobs.sort_by_key(|&j| (target_solution.get(c_offset + j).copied().unwrap_or(0), j));
        Self::encode_schedule_as_lehmer(&jobs)
    }
}

#[reduction(overhead = {
    num_vars = "num_tasks * (num_tasks - 1) / 2 + 2 * num_tasks",
    num_constraints = "num_tasks * (num_tasks - 1) / 2 + num_tasks + num_tasks * (num_tasks - 1) + 2 * num_tasks + 1",
})]
impl ReduceTo<ILP<i32>> for SequencingToMinimizeWeightedTardiness {
    type Result = ReductionSTMWTToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();
        let num_order_vars = n * n.saturating_sub(1) / 2;
        let num_vars = num_order_vars + 2 * n;

        let order_var = |i: usize, j: usize| -> usize {
            debug_assert!(i < j);
            i * (2 * n - i - 1) / 2 + (j - i - 1)
        };
        let c_var = |j: usize| -> usize { num_order_vars + j };
        let t_var = |j: usize| -> usize { num_order_vars + n + j };

        let lengths = self.lengths();
        let deadlines = self.deadlines();
        let weights = self.weights();
        let bound = self.bound();

        // M = sum of all lengths (valid schedule-horizon bound)
        let big_m: f64 = lengths.iter().sum::<u64>() as f64;

        let mut constraints = Vec::new();

        // 1. y_{i,j} in {0,1}: 0 <= y_{i,j} <= 1
        for i in 0..n {
            for j in (i + 1)..n {
                constraints.push(LinearConstraint::le(vec![(order_var(i, j), 1.0)], 1.0));
                constraints.push(LinearConstraint::ge(vec![(order_var(i, j), 1.0)], 0.0));
            }
        }

        // 2. C_j >= l_j for all j
        for (j, &l_j) in lengths.iter().enumerate() {
            constraints.push(LinearConstraint::ge(vec![(c_var(j), 1.0)], l_j as f64));
        }

        // 3. Disjunctive: C_j >= C_i + l_j - M*(1 - y_{i,j}) for i != j
        for i in 0..n {
            for (j, &l_j) in lengths.iter().enumerate() {
                if i == j {
                    continue;
                }
                if i < j {
                    // y_{i,j} is the stored variable.
                    // C_j >= C_i + l_j - M*(1 - y_{i,j})
                    // => C_j - C_i - M*y_{i,j} >= l_j - M
                    constraints.push(LinearConstraint::ge(
                        vec![(c_var(j), 1.0), (c_var(i), -1.0), (order_var(i, j), -big_m)],
                        l_j as f64 - big_m,
                    ));
                } else {
                    // i > j: y_{j,i} is stored, y_{i,j} = 1 - y_{j,i}
                    // C_j >= C_i + l_j - M*y_{j,i}
                    // C_j - C_i + M*y_{j,i} >= l_j
                    constraints.push(LinearConstraint::ge(
                        vec![(c_var(j), 1.0), (c_var(i), -1.0), (order_var(j, i), big_m)],
                        l_j as f64,
                    ));
                }
            }
        }

        // 4. T_j >= C_j - d_j for all j
        for (j, &d_j) in deadlines.iter().enumerate() {
            constraints.push(LinearConstraint::ge(
                vec![(t_var(j), 1.0), (c_var(j), -1.0)],
                -(d_j as f64),
            ));
        }

        // 5. T_j >= 0 for all j
        for j in 0..n {
            constraints.push(LinearConstraint::ge(vec![(t_var(j), 1.0)], 0.0));
        }

        // 6. Σ_j w_j * T_j <= K
        let terms: Vec<(usize, f64)> = (0..n).map(|j| (t_var(j), weights[j] as f64)).collect();
        constraints.push(LinearConstraint::le(terms, bound as f64));

        ReductionSTMWTToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_tasks: n,
            num_order_vars,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
        vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingtominimizeweightedtardiness_to_ilp",
        build: || {
            let source = SequencingToMinimizeWeightedTardiness::new(
                vec![3, 4, 2],
                vec![2, 3, 1],
                vec![5, 8, 4],
                10,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingtominimizeweightedtardiness_ilp.rs"]
mod tests;
