//! Reduction from OpenShopScheduling to ILP<i32>.
//!
//! Disjunctive formulation with binary ordering variables and integer start times:
//!
//! **Variables:**
//! - `x_{j,k,i}` for j < k, all machines i: binary, 1 if job j precedes job k on machine i.
//!   Index: pair index * m + i, where pair index = `j*(2n-j-1)/2 + (k-j-1)`.
//!   Count: n*(n-1)/2 * m variables.
//! - `s_{j,i}` for all (j, i): integer start time of job j on machine i.
//!   Index: num_order_vars + j * m + i.
//!   Count: n * m variables.
//! - `C` (makespan): integer, index num_order_vars + n * m.
//!
//! **Constraints:**
//! 1. Binary bounds: 0 ≤ x_{j,k,i} ≤ 1 for all j < k, i.
//! 2. Machine non-overlap for each pair (j, k) and machine i:
//!    - s_{k,i} ≥ s_{j,i} + p_{j,i} - M*(1 - x_{j,k,i})  →  s_{k,i} - s_{j,i} + M*x_{j,k,i} ≥ p_{j,i}
//!    - s_{j,i} ≥ s_{k,i} + p_{k,i} - M*x_{j,k,i}         →  s_{j,i} - s_{k,i} - M*x_{j,k,i} ≥ p_{k,i} - M
//! 3. Job non-overlap for each job j and each pair of machines (i, i'):
//!    Uses separate binary variable y_{j,i,i'} for i < i' to decide which task runs first.
//!    Variables y_{j,i,i'}: appended after s variables.
//!    - s_{j,i'} ≥ s_{j,i} + p_{j,i} - M*(1 - y_{j,i,i'})
//!    - s_{j,i} ≥ s_{j,i'} + p_{j,i'} - M*y_{j,i,i'}
//! 4. Makespan: C ≥ s_{j,i} + p_{j,i} for all (j, i).
//! 5. Non-negativity of start times: s_{j,i} ≥ 0 (implied by ILP non-negativity).
//!
//! **Objective:** Minimize C.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::OpenShopScheduling;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing OpenShopScheduling to ILP<i32>.
///
/// Variable layout:
/// - `x_{j,k,i}` at index `pair_idx(j,k) * m + i`    (num_pairs * m vars)
/// - `s_{j,i}`   at index `num_order_vars + j * m + i`  (n * m vars)
/// - `y_{j,i,i'}` for i < i': at `num_order_vars + n*m + j * num_machine_pairs + machine_pair_idx(i,i')`
///   (n * m*(m-1)/2 vars)
/// - `C`: at index `num_order_vars + n * m + n * m*(m-1)/2` (1 var)
#[derive(Debug, Clone)]
pub struct ReductionOSSToILP {
    target: ILP<i32>,
    num_jobs: usize,
    num_machines: usize,
    /// n*(n-1)/2 * m — start index of s_{j,i} variables
    num_order_vars: usize,
}

impl ReductionOSSToILP {
    fn pair_idx(&self, j: usize, k: usize) -> usize {
        debug_assert!(j < k);
        let n = self.num_jobs;
        j * (2 * n - j - 1) / 2 + (k - j - 1)
    }

    fn x_var(&self, j: usize, k: usize, i: usize) -> usize {
        self.pair_idx(j, k) * self.num_machines + i
    }

    fn s_var(&self, j: usize, i: usize) -> usize {
        self.num_order_vars + j * self.num_machines + i
    }

    fn machine_pair_idx(&self, i: usize, ip: usize) -> usize {
        debug_assert!(i < ip);
        let m = self.num_machines;
        i * (2 * m - i - 1) / 2 + (ip - i - 1)
    }

    fn y_var(&self, j: usize, i: usize, ip: usize) -> usize {
        let num_machine_pairs = self.num_machines * self.num_machines.saturating_sub(1) / 2;
        self.num_order_vars
            + self.num_jobs * self.num_machines
            + j * num_machine_pairs
            + self.machine_pair_idx(i, ip)
    }
}

impl ReductionResult for ReductionOSSToILP {
    type Source = OpenShopScheduling;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract per-machine job orderings from the ILP start times, then
    /// convert to the config format (direct permutation indices per machine).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_jobs;
        let m = self.num_machines;

        // Read start times s_{j,i} for each (j, i)
        let start = |j: usize, i: usize| -> usize {
            let idx = self.num_order_vars + j * m + i;
            target_solution.get(idx).copied().unwrap_or(0)
        };

        // For each machine, sort jobs by their start time on that machine
        let mut config = Vec::with_capacity(n * m);
        for i in 0..m {
            let mut jobs: Vec<usize> = (0..n).collect();
            jobs.sort_by_key(|&j| (start(j, i), j));
            config.extend(jobs);
        }
        config
    }
}

#[reduction(overhead = {
    num_vars = "num_jobs * (num_jobs - 1) / 2 * num_machines + num_jobs * num_machines + num_jobs * num_machines * (num_machines - 1) / 2 + 1",
    num_constraints = "num_jobs * (num_jobs - 1) / 2 * num_machines + num_jobs * num_machines + 1 + 2 * num_jobs * (num_jobs - 1) / 2 * num_machines + num_jobs * num_machines * (num_machines - 1) / 2 + 2 * num_jobs * num_machines * (num_machines - 1) / 2 + num_jobs * num_machines",
})]
impl ReduceTo<ILP<i32>> for OpenShopScheduling {
    type Result = ReductionOSSToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_jobs();
        let m = self.num_machines();
        let p = self.processing_times();

        let num_pairs = n * n.saturating_sub(1) / 2;
        let num_machine_pairs = m * m.saturating_sub(1) / 2;

        // Variable counts
        let num_order_vars = num_pairs * m; // x_{j,k,i}: binary
        let num_start_vars = n * m; // s_{j,i}: integer
        let num_job_pair_vars = n * num_machine_pairs; // y_{j,i,i'}: binary
        let num_vars = num_order_vars + num_start_vars + num_job_pair_vars + 1; // +1 for C

        let result = ReductionOSSToILP {
            target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
            num_jobs: n,
            num_machines: m,
            num_order_vars,
        };

        // Big-M: sum of all processing times (loose upper bound on makespan)
        let total_p: usize = p.iter().flat_map(|row| row.iter()).sum();
        let big_m = total_p as f64;

        let c_var = num_order_vars + num_start_vars + num_job_pair_vars;

        let mut constraints = Vec::new();

        // 1. Binary bounds on x_{j,k,i}: 0 ≤ x ≤ 1
        for j in 0..n {
            for k in (j + 1)..n {
                for i in 0..m {
                    let x = result.x_var(j, k, i);
                    constraints.push(LinearConstraint::le(vec![(x, 1.0)], 1.0));
                }
            }
        }

        // Upper bounds on start time variables: s_{j,i} ≤ total_p
        // (no task can start after all tasks have finished)
        for j in 0..n {
            for i in 0..m {
                let sji = result.s_var(j, i);
                constraints.push(LinearConstraint::le(vec![(sji, 1.0)], big_m));
            }
        }

        // Upper bound on makespan C ≤ total_p
        constraints.push(LinearConstraint::le(vec![(c_var, 1.0)], big_m));

        // 2. Machine non-overlap: for each pair (j,k) with j<k, each machine i
        //    x_{j,k,i}=1 means j precedes k on machine i:
        //      s_{k,i} ≥ s_{j,i} + p_{j,i}  →  s_{k,i} - s_{j,i} + M*x_{j,k,i} ≥ p_{j,i} (active when x=0)
        //    Actually: s_{k,i} ≥ s_{j,i} + p_{j,i} - M*(1 - x_{j,k,i})
        //              ⟺ s_{k,i} - s_{j,i} - M*x_{j,k,i} ≥ p_{j,i} - M
        //    And:    s_{j,i} ≥ s_{k,i} + p_{k,i} - M*x_{j,k,i}
        //              ⟺ s_{j,i} - s_{k,i} + M*x_{j,k,i} ≥ p_{k,i}  (active when x=1, i.e. k before j)
        //    Wait, let's be careful. x=1 means j before k.
        //      (a) if j before k: s_k ≥ s_j + p_{j,i}  →  when x=1 this is active, when x=0 inactive
        //      (b) if k before j (x=0): s_j ≥ s_k + p_{k,i}
        //
        //    Linearization:
        //      (a) s_{k,i} - s_{j,i} + M*(1-x) ≥ p_{j,i}
        //          s_{k,i} - s_{j,i} - M*x ≥ p_{j,i} - M
        //      (b) s_{j,i} - s_{k,i} + M*x ≥ p_{k,i}
        for j in 0..n {
            for k in (j + 1)..n {
                for i in 0..m {
                    let x = result.x_var(j, k, i);
                    let sj = result.s_var(j, i);
                    let sk = result.s_var(k, i);
                    let pji = p[j][i] as f64;
                    let pki = p[k][i] as f64;

                    // (a) s_{k,i} - s_{j,i} - M*x_{j,k,i} >= p_{j,i} - M
                    constraints.push(LinearConstraint::ge(
                        vec![(sk, 1.0), (sj, -1.0), (x, -big_m)],
                        pji - big_m,
                    ));

                    // (b) s_{j,i} - s_{k,i} + M*x_{j,k,i} >= p_{k,i}
                    constraints.push(LinearConstraint::ge(
                        vec![(sj, 1.0), (sk, -1.0), (x, big_m)],
                        pki,
                    ));
                }
            }
        }

        // 3. Binary bounds on y_{j,i,i'}: 0 ≤ y ≤ 1
        for j in 0..n {
            for i in 0..m {
                for ip in (i + 1)..m {
                    let y = result.y_var(j, i, ip);
                    constraints.push(LinearConstraint::le(vec![(y, 1.0)], 1.0));
                }
            }
        }

        // 4. Job non-overlap: for each job j and each pair (i, i') with i < i'
        //    y_{j,i,i'}=1 means machine i is scheduled before machine i' for job j:
        //      (a) s_{j,i'} ≥ s_{j,i} + p_{j,i} - M*(1-y)
        //          s_{j,i'} - s_{j,i} - M*y ≥ p_{j,i} - M
        //      (b) s_{j,i} ≥ s_{j,i'} + p_{j,i'} - M*y
        //          s_{j,i} - s_{j,i'} + M*y ≥ p_{j,i'}
        for (j, pj) in p.iter().enumerate() {
            for i in 0..m {
                for ip in (i + 1)..m {
                    let y = result.y_var(j, i, ip);
                    let sji = result.s_var(j, i);
                    let sjip = result.s_var(j, ip);
                    let pji = pj[i] as f64;
                    let pjip = pj[ip] as f64;

                    // (a) s_{j,i'} - s_{j,i} - M*y >= p_{j,i} - M
                    constraints.push(LinearConstraint::ge(
                        vec![(sjip, 1.0), (sji, -1.0), (y, -big_m)],
                        pji - big_m,
                    ));

                    // (b) s_{j,i} - s_{j,i'} + M*y >= p_{j,i'}
                    constraints.push(LinearConstraint::ge(
                        vec![(sji, 1.0), (sjip, -1.0), (y, big_m)],
                        pjip,
                    ));
                }
            }
        }

        // 5. Makespan: C ≥ s_{j,i} + p_{j,i}  ⟺  C - s_{j,i} ≥ p_{j,i}
        for (j, pj) in p.iter().enumerate() {
            for (i, &pji) in pj.iter().enumerate() {
                let sji = result.s_var(j, i);
                constraints.push(LinearConstraint::ge(
                    vec![(c_var, 1.0), (sji, -1.0)],
                    pji as f64,
                ));
            }
        }

        // Objective: minimize C
        let objective = vec![(c_var, 1.0)];

        ReductionOSSToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_jobs: n,
            num_machines: m,
            num_order_vars,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "openshopscheduling_to_ilp",
        build: || {
            // Small 2x2 instance for canonical example
            let source = OpenShopScheduling::new(2, vec![vec![1, 2], vec![2, 1]]);
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/openshopscheduling_ilp.rs"]
mod tests;
