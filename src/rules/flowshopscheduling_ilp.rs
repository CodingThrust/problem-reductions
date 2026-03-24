//! Reduction from FlowShopScheduling to ILP<i32>.
//!
//! Binary order variables y_{i,j} with y_{i,j}=1 iff job i precedes job j,
//! integer completion-time variables C_{j,q} for each job j and machine q.
//! Machine-chain and big-M disjunctive constraints enforce a valid flow-shop
//! schedule; the deadline becomes a makespan bound.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::FlowShopScheduling;
use crate::reduction;
use crate::rules::ilp_helpers::permutation_to_lehmer;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing FlowShopScheduling to ILP<i32>.
///
/// Variable layout:
/// - `y_{i,j}` for each ordered pair (i,j) with i<j: index `i*n + j - (i+1)*(i+2)/2`
///   (upper triangle, n*(n-1)/2 variables)
/// - `C_{j,q}` for j in 0..n, q in 0..m: index `num_order_vars + j*m + q`
///
/// Total: n*(n-1)/2 + n*m variables.
#[derive(Debug, Clone)]
pub struct ReductionFSSToILP {
    target: ILP<i32>,
    num_jobs: usize,
    num_machines: usize,
    num_order_vars: usize,
}

impl ReductionFSSToILP {
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

impl ReductionResult for ReductionFSSToILP {
    type Source = FlowShopScheduling;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract solution: sort jobs by final-machine completion time C_{j,m-1},
    /// then convert permutation to Lehmer code.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_jobs;
        let m = self.num_machines;
        let c_offset = self.num_order_vars;
        let mut jobs: Vec<usize> = (0..n).collect();
        jobs.sort_by_key(|&j| {
            let idx = c_offset + j * m + (m - 1);
            (target_solution.get(idx).copied().unwrap_or(0), j)
        });
        let perm = permutation_to_lehmer(&jobs);
        Self::encode_schedule_as_lehmer(&jobs)
            .into_iter()
            .zip(perm)
            .map(|(lehmer, _)| lehmer)
            .collect()
    }
}

#[reduction(overhead = {
    num_vars = "num_jobs * (num_jobs - 1) / 2 + num_jobs * num_processors",
    num_constraints = "num_jobs * (num_jobs - 1) / 2 + num_jobs + num_jobs * (num_processors - 1) + num_jobs * (num_jobs - 1) * num_processors + num_jobs",
})]
impl ReduceTo<ILP<i32>> for FlowShopScheduling {
    type Result = ReductionFSSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_jobs();
        let m = self.num_processors();

        let num_order_vars = n * n.saturating_sub(1) / 2;
        let num_completion_vars = n * m;
        let num_vars = num_order_vars + num_completion_vars;

        // Order variable index for pair (i, j) with i < j
        let order_var = |i: usize, j: usize| -> usize {
            debug_assert!(i < j);
            i * (2 * n - i - 1) / 2 + (j - i - 1)
        };
        // Completion time variable index for job j, machine q
        let c_var = |j: usize, q: usize| -> usize { num_order_vars + j * m + q };

        let p = self.task_lengths();
        let d = self.deadline();

        // Big-M: D + max processing time
        let max_p = p
            .iter()
            .flat_map(|row| row.iter())
            .copied()
            .max()
            .unwrap_or(0);
        let big_m = (d + max_p) as f64;

        let mut constraints = Vec::new();

        // 1. Symmetry: y_{i,j} + y_{j,i} = 1 for all i != j
        // Since we only store y_{i,j} for i < j, we enforce y_{i,j} in {0,1}
        // via 0 <= y_{i,j} <= 1.
        for i in 0..n {
            for j in (i + 1)..n {
                constraints.push(LinearConstraint::le(vec![(order_var(i, j), 1.0)], 1.0));
                constraints.push(LinearConstraint::ge(vec![(order_var(i, j), 1.0)], 0.0));
            }
        }

        // 2. C_{j,0} >= p_{j,0} for all j
        for (j, p_j) in p.iter().enumerate() {
            constraints.push(LinearConstraint::ge(
                vec![(c_var(j, 0), 1.0)],
                p_j[0] as f64,
            ));
        }

        // 3. Machine chain: C_{j,q+1} >= C_{j,q} + p_{j,q+1} for all j, q in 0..m-1
        for (j, p_j) in p.iter().enumerate() {
            for q in 0..(m.saturating_sub(1)) {
                // C_{j,q+1} - C_{j,q} >= p_{j,q+1}
                constraints.push(LinearConstraint::ge(
                    vec![(c_var(j, q + 1), 1.0), (c_var(j, q), -1.0)],
                    p_j[q + 1] as f64,
                ));
            }
        }

        // 4. Disjunctive: C_{j,q} >= C_{i,q} + p_{j,q} - M*(1 - y_{i,j}) for i != j, all q
        // For i < j: y_{i,j} is the variable.
        //   C_{j,q} - C_{i,q} + M*y_{i,j} >= p_{j,q} + M  ... wrong
        //   Actually: C_{j,q} >= C_{i,q} + p_{j,q} - M*(1 - y_{i,j})
        //   => C_{j,q} - C_{i,q} + M*y_{i,j} >= p_{j,q}   ... when y_{i,j}=0 (i NOT before j): inactive
        //                                                        when y_{i,j}=1 (i before j): C_{j,q} >= C_{i,q} + p_{j,q}
        //   Wait, this needs reconsideration. The paper says:
        //   C_{j,q} >= C_{i,q} + p_{j,q} - M*(1 - y_{i,j})
        //   => C_{j,q} - C_{i,q} - M*y_{i,j} >= p_{j,q} - M
        //   No let me expand directly:
        //   C_{j,q} - C_{i,q} + M*y_{i,j} >= p_{j,q} + M*(0)... hmm
        //
        // Let me re-derive: C_{j,q} >= C_{i,q} + p_{j,q} - M*(1 - y_{i,j})
        //   = C_{j,q} - C_{i,q} + M*(1 - y_{i,j}) >= p_{j,q}
        //   = C_{j,q} - C_{i,q} + M - M*y_{i,j} >= p_{j,q}
        //   = C_{j,q} - C_{i,q} - M*y_{i,j} >= p_{j,q} - M
        for i in 0..n {
            for (j, p_j) in p.iter().enumerate() {
                if i == j {
                    continue;
                }
                for (q, &p_jq) in p_j.iter().enumerate() {
                    if i < j {
                        // y_{i,j} is the variable. When y_{i,j} = 1, i precedes j,
                        // so C_{j,q} >= C_{i,q} + p_{j,q}.
                        // C_{j,q} - C_{i,q} - M*y_{i,j} >= p_{j,q} - M
                        constraints.push(LinearConstraint::ge(
                            vec![
                                (c_var(j, q), 1.0),
                                (c_var(i, q), -1.0),
                                (order_var(i, j), -big_m),
                            ],
                            p_jq as f64 - big_m,
                        ));
                    } else {
                        // i > j: y_{j,i} is stored. y_{i,j} = 1 - y_{j,i}.
                        // C_{j,q} >= C_{i,q} + p_{j,q} - M*(1 - (1 - y_{j,i}))
                        // C_{j,q} >= C_{i,q} + p_{j,q} - M*y_{j,i}
                        // C_{j,q} - C_{i,q} + M*y_{j,i} >= p_{j,q}
                        constraints.push(LinearConstraint::ge(
                            vec![
                                (c_var(j, q), 1.0),
                                (c_var(i, q), -1.0),
                                (order_var(j, i), big_m),
                            ],
                            p_jq as f64,
                        ));
                    }
                }
            }
        }

        // 5. Deadline: C_{j,m-1} <= D for all j
        if m > 0 {
            for j in 0..n {
                constraints.push(LinearConstraint::le(vec![(c_var(j, m - 1), 1.0)], d as f64));
            }
        }

        ReductionFSSToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_jobs: n,
            num_machines: m,
            num_order_vars,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "flowshopscheduling_to_ilp",
        build: || {
            // 2 machines, 3 jobs, deadline 10
            let source = FlowShopScheduling::new(2, vec![vec![2, 3], vec![3, 2], vec![1, 4]], 10);
            let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
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
#[path = "../unit_tests/rules/flowshopscheduling_ilp.rs"]
mod tests;
