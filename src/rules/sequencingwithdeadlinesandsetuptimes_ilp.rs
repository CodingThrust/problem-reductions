//! Reduction from SequencingWithDeadlinesAndSetUpTimes to ILP<bool>.
//!
//! Position-assignment ILP with compiler-switch detection.
//!
//! Variables:
//! - `x_{j,p}` binary: task j occupies position p  (n*n variables)
//! - `sw_p`    binary: a compiler switch occurs before position p (n-1 variables, p >= 1)
//! - `a_{j,p}` binary: x_{j,p} = 1 AND sw_p = 1   (n*(n-1) variables, p >= 1)
//!
//! The completion time of task j at position p equals the sum of all task
//! lengths up to and including position p, plus the setup times for switches
//! at each position 1..=p. Using the `a_{j,p}` linearisation, the setup
//! contribution at position p is `sum_j s[k(j)] * a_{j,p}`.
//!
//! Deadline enforcement uses the standard big-M trick: for each (j, p),
//! if `x_{j,p}=1` then the completion time at p must not exceed `d[j]`.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::SequencingWithDeadlinesAndSetUpTimes;
use crate::reduction;
use crate::rules::ilp_helpers::one_hot_decode;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SequencingWithDeadlinesAndSetUpTimes to ILP<bool>.
#[derive(Debug, Clone)]
pub struct ReductionSWDSTToILP {
    target: ILP<bool>,
    num_tasks: usize,
}

impl ReductionResult for ReductionSWDSTToILP {
    type Source = SequencingWithDeadlinesAndSetUpTimes;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_tasks;
        // x_{j,p} occupies the first n*n variables: decode the permutation.
        one_hot_decode(target_solution, n, n, 0)
    }
}

#[reduction(overhead = {
    num_vars = "num_tasks * num_tasks + (num_tasks - 1) + num_tasks * (num_tasks - 1)",
    num_constraints = "2 * num_tasks + num_tasks^2 * (num_tasks - 1) + 3 * num_tasks * (num_tasks - 1) + num_tasks * num_tasks",
})]
impl ReduceTo<ILP<bool>> for SequencingWithDeadlinesAndSetUpTimes {
    type Result = ReductionSWDSTToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_tasks();

        // Handle empty case.
        if n == 0 {
            return ReductionSWDSTToILP {
                target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
                num_tasks: 0,
            };
        }

        // Variable layout:
        //   x_{j,p}   = j*n + p          for j,p in 0..n        → indices 0..n*n
        //   sw_p       = n*n + (p-1)      for p in 1..n          → indices n*n .. n*n+(n-1)
        //   a_{j,p}    = n*n+(n-1)+j*(n-1)+(p-1)  for j in 0..n, p in 1..n
        //                                           → indices n*n+(n-1) .. n*n+(n-1)+n*(n-1)
        let num_x = n * n;
        let sw_offset = num_x;
        let a_offset = sw_offset + (n - 1);
        let num_vars = a_offset + n * (n - 1);

        let x_var = |j: usize, p: usize| -> usize { j * n + p };
        let sw_var = |p: usize| -> usize { sw_offset + (p - 1) }; // p >= 1
        let a_var = |j: usize, p: usize| -> usize { a_offset + j * (n - 1) + (p - 1) }; // p >= 1

        let lengths = self.lengths();
        let deadlines = self.deadlines();
        let compilers = self.compilers();
        let setup_times = self.setup_times();

        // Big-M: total processing time + worst-case total setup overhead.
        let total_length: u64 = lengths.iter().copied().sum();
        let max_setup: u64 = setup_times.iter().copied().max().unwrap_or(0);
        let big_m = total_length as f64 + max_setup as f64 * (n as f64 - 1.0);

        let mut constraints = Vec::new();

        // 1. Each task assigned to exactly one position: sum_p x_{j,p} = 1 for all j.
        for j in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|p| (x_var(j, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // 2. Each position has exactly one task: sum_j x_{j,p} = 1 for all p.
        for p in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (x_var(j, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // For each position p >= 1:
        for p in 1..n {
            // 3. Switch detection: sw_p >= x_{j,p} + x_{j',p-1} - 1
            //    whenever k(j) != k(j').
            //    This forces sw_p = 1 whenever the tasks at p-1 and p differ.
            for j in 0..n {
                for j_prev in 0..n {
                    if compilers[j] != compilers[j_prev] {
                        // sw_p - x_{j,p} - x_{j',p-1} >= -1
                        // i.e., x_{j,p} + x_{j',p-1} - sw_p <= 1
                        constraints.push(LinearConstraint::le(
                            vec![
                                (x_var(j, p), 1.0),
                                (x_var(j_prev, p - 1), 1.0),
                                (sw_var(p), -1.0),
                            ],
                            1.0,
                        ));
                    }
                }
            }

            // 4. Linearisation of a_{j,p} = x_{j,p} * sw_p for each j:
            //    a_{j,p} <= x_{j,p}
            //    a_{j,p} <= sw_p
            //    a_{j,p} >= x_{j,p} + sw_p - 1
            for j in 0..n {
                // a_{j,p} <= x_{j,p}
                constraints.push(LinearConstraint::le(
                    vec![(a_var(j, p), 1.0), (x_var(j, p), -1.0)],
                    0.0,
                ));
                // a_{j,p} <= sw_p
                constraints.push(LinearConstraint::le(
                    vec![(a_var(j, p), 1.0), (sw_var(p), -1.0)],
                    0.0,
                ));
                // a_{j,p} >= x_{j,p} + sw_p - 1
                // i.e. x_{j,p} + sw_p - a_{j,p} <= 1
                constraints.push(LinearConstraint::le(
                    vec![(x_var(j, p), 1.0), (sw_var(p), 1.0), (a_var(j, p), -1.0)],
                    1.0,
                ));
            }
        }

        // 5. Deadline constraints: for each (j, p), if x_{j,p}=1, then
        //    the completion time at position p must be <= d[j].
        //
        //    Completion time at position p =
        //      sum_{p'<=p} sum_{j''} l_{j''} * x_{j'',p'}
        //      + sum_{p'=1..=p} sum_{j''} s[k(j'')] * a_{j'',p'}
        //
        //    Big-M form (only active when x_{j,p}=1):
        //    M * x_{j,p}
        //      + sum_{p'<p} sum_{j''} l_{j''} * x_{j'',p'}
        //      + sum_{p'=1..=p} sum_{j''} s[k(j'')] * a_{j'',p'}
        //      - M * x_{j,p}   (cancels the activation term)
        //      <= d[j] - l[j] + M
        //
        //    Simplifying (M * x_{j,p} - M * x_{j,p} vanishes):
        //      sum_{p'<p} sum_{j''} l_{j''} * x_{j'',p'}
        //      + sum_{p'=1..=p} sum_{j''} s[k(j'')] * a_{j'',p'}
        //      + M * x_{j,p}
        //      - M
        //      <= d[j] - l[j]
        //
        //    i.e.:
        //      M * x_{j,p}
        //      + sum_{p'<p} sum_{j''} l_{j''} * x_{j'',p'}
        //      + sum_{p'=1..=p} sum_{j''} s[k(j'')] * a_{j'',p'}
        //      <= d[j] - l[j] + M
        for j in 0..n {
            for p in 0..n {
                let mut terms: Vec<(usize, f64)> = Vec::new();
                // Big-M activation term
                terms.push((x_var(j, p), big_m));
                // Processing time for positions 0..p (not including p itself)
                for pp in 0..p {
                    for (jj, &len) in lengths.iter().enumerate() {
                        terms.push((x_var(jj, pp), len as f64));
                    }
                }
                // Setup time for positions 1..=p
                for pp in 1..=p {
                    for jj in 0..n {
                        let s = setup_times[compilers[jj]] as f64;
                        if s > 0.0 {
                            terms.push((a_var(jj, pp), s));
                        }
                    }
                }
                let rhs = deadlines[j] as f64 - lengths[j] as f64 + big_m;
                constraints.push(LinearConstraint::le(terms, rhs));
            }
        }

        ReductionSWDSTToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize),
            num_tasks: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "sequencingwithdeadlinesandsetuptimes_to_ilp",
        build: || {
            let source = SequencingWithDeadlinesAndSetUpTimes::new(
                vec![2, 3, 1, 2, 2],
                vec![4, 11, 3, 16, 7],
                vec![0, 1, 0, 1, 0],
                vec![1, 2],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sequencingwithdeadlinesandsetuptimes_ilp.rs"]
mod tests;
