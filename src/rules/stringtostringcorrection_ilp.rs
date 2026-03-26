//! Reduction from StringToStringCorrection to ILP (Integer Linear Programming).
//!
//! A time-expanded ILP with state variables z_{t,p,i} tracking token positions,
//! emptiness bits e_{t,p}, and operation selectors (delete, swap, no-op) at
//! each of K stages.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::StringToStringCorrection;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing StringToStringCorrection to ILP.
#[derive(Debug, Clone)]
pub struct ReductionSTSCToILP {
    target: ILP<bool>,
    n: usize,
    bound: usize,
}

// Index helper functions (free functions to avoid `Self::` ambiguity in trait impls).
fn idx_z(n: usize, t: usize, p: usize, i: usize) -> usize {
    t * n * n + p * n + i
}

fn idx_e(n: usize, k: usize, t: usize, p: usize) -> usize {
    (k + 1) * n * n + t * n + p
}

fn idx_d(n: usize, k: usize, t: usize, j: usize) -> usize {
    (k + 1) * (n * n + n) + (t - 1) * n + j
}

fn idx_s(n: usize, k: usize, t: usize, j: usize) -> usize {
    let nm1 = n.saturating_sub(1);
    (k + 1) * (n * n + n) + k * n + (t - 1) * nm1 + j
}

fn idx_nu(n: usize, k: usize, t: usize) -> usize {
    let nm1 = n.saturating_sub(1);
    (k + 1) * (n * n + n) + k * n + k * nm1 + (t - 1)
}

fn total_vars(n: usize, k: usize) -> usize {
    let nm1 = n.saturating_sub(1);
    (k + 1) * n * n + (k + 1) * n + k * n + k * nm1 + k
}

impl ReductionResult for ReductionSTSCToILP {
    type Source = StringToStringCorrection;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract operation sequence from ILP solution.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        let k = self.bound;
        let noop_code = 2 * n;

        if n == 0 {
            return vec![noop_code; k];
        }

        let nm1 = n.saturating_sub(1);
        let mut ops = Vec::with_capacity(k);

        for t in 1..=k {
            // current length at step t-1
            let current_len = (0..n)
                .filter(|&p| target_solution[idx_e(n, k, t - 1, p)] == 0)
                .count();

            if target_solution[idx_nu(n, k, t)] == 1 {
                ops.push(noop_code);
            } else {
                let mut found = false;
                for j in 0..n {
                    if target_solution[idx_d(n, k, t, j)] == 1 {
                        ops.push(j);
                        found = true;
                        break;
                    }
                }
                if !found {
                    for j in 0..nm1 {
                        if target_solution[idx_s(n, k, t, j)] == 1 {
                            ops.push(current_len + j);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        ops.push(noop_code);
                    }
                }
            }
        }
        ops
    }
}

#[reduction(
    overhead = {
        num_vars = "(bound + 1) * source_length * source_length + (bound + 1) * source_length + 2 * bound * source_length",
        num_constraints = "(bound + 1) * source_length * source_length",
    }
)]
impl ReduceTo<ILP<bool>> for StringToStringCorrection {
    type Result = ReductionSTSCToILP;

    #[allow(clippy::needless_range_loop)]
    fn reduce_to(&self) -> Self::Result {
        let n = self.source_length();
        let m = self.target_length();
        let k = self.bound();
        let source = self.source();
        let target = self.target();

        // If infeasible by length check, return trivially infeasible ILP
        if m > n || m < n.saturating_sub(k) {
            return ReductionSTSCToILP {
                target: ILP::new(
                    0,
                    vec![LinearConstraint::le(vec![], -1.0)],
                    vec![],
                    ObjectiveSense::Minimize,
                ),
                n,
                bound: k,
            };
        }

        // n == 0 edge case: source and target both empty, all no-ops
        if n == 0 {
            let nv = k;
            let mut constraints = Vec::new();
            for t in 1..=k {
                constraints.push(LinearConstraint::eq(vec![(t - 1, 1.0)], 1.0));
            }
            return ReductionSTSCToILP {
                target: ILP::new(nv, constraints, vec![], ObjectiveSense::Minimize),
                n,
                bound: k,
            };
        }

        let nm1 = n.saturating_sub(1);
        let nv = total_vars(n, k);

        let mut constraints = Vec::new();

        // === State validity ===

        // e_{t,p} + Σ_i z_{t,p,i} = 1  ∀ t,p
        for t in 0..=k {
            for p in 0..n {
                let mut terms = vec![(idx_e(n, k, t, p), 1.0)];
                for i in 0..n {
                    terms.push((idx_z(n, t, p, i), 1.0));
                }
                constraints.push(LinearConstraint::eq(terms, 1.0));
            }
        }

        // Σ_p z_{t,p,i} <= 1  ∀ t,i
        for t in 0..=k {
            for i in 0..n {
                let terms: Vec<(usize, f64)> = (0..n).map(|p| (idx_z(n, t, p, i), 1.0)).collect();
                constraints.push(LinearConstraint::le(terms, 1.0));
            }
        }

        // e_{t,p} <= e_{t,p+1}  ∀ t, p < n-1
        for t in 0..=k {
            for p in 0..nm1 {
                constraints.push(LinearConstraint::le(
                    vec![(idx_e(n, k, t, p), 1.0), (idx_e(n, k, t, p + 1), -1.0)],
                    0.0,
                ));
            }
        }

        // === Initial state ===
        for p in 0..n {
            constraints.push(LinearConstraint::eq(vec![(idx_z(n, 0, p, p), 1.0)], 1.0));
            for i in 0..n {
                if i != p {
                    constraints.push(LinearConstraint::eq(vec![(idx_z(n, 0, p, i), 1.0)], 0.0));
                }
            }
            constraints.push(LinearConstraint::eq(vec![(idx_e(n, k, 0, p), 1.0)], 0.0));
        }

        // === Operation choice ===
        for t in 1..=k {
            let mut terms = Vec::new();
            for j in 0..n {
                terms.push((idx_d(n, k, t, j), 1.0));
            }
            for j in 0..nm1 {
                terms.push((idx_s(n, k, t, j), 1.0));
            }
            terms.push((idx_nu(n, k, t), 1.0));
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Legality
        for t in 1..=k {
            for j in 0..n {
                constraints.push(LinearConstraint::le(
                    vec![(idx_d(n, k, t, j), 1.0), (idx_e(n, k, t - 1, j), 1.0)],
                    1.0,
                ));
            }
            for j in 0..nm1 {
                constraints.push(LinearConstraint::le(
                    vec![(idx_s(n, k, t, j), 1.0), (idx_e(n, k, t - 1, j), 1.0)],
                    1.0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(idx_s(n, k, t, j), 1.0), (idx_e(n, k, t - 1, j + 1), 1.0)],
                    1.0,
                ));
            }
        }

        // === State-update (M=1 big-M) ===
        for t in 1..=k {
            for p in 0..n {
                for i in 0..n {
                    // No-op
                    constraints.push(LinearConstraint::le(
                        vec![
                            (idx_z(n, t, p, i), 1.0),
                            (idx_z(n, t - 1, p, i), -1.0),
                            (idx_nu(n, k, t), 1.0),
                        ],
                        1.0,
                    ));
                    constraints.push(LinearConstraint::le(
                        vec![
                            (idx_z(n, t - 1, p, i), 1.0),
                            (idx_z(n, t, p, i), -1.0),
                            (idx_nu(n, k, t), 1.0),
                        ],
                        1.0,
                    ));

                    // Delete at position j
                    for j in 0..n {
                        if p < j {
                            // Before deleted position: unchanged
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t, p, i), 1.0),
                                    (idx_z(n, t - 1, p, i), -1.0),
                                    (idx_d(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t - 1, p, i), 1.0),
                                    (idx_z(n, t, p, i), -1.0),
                                    (idx_d(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                        } else if p + 1 < n {
                            // j <= p < n-1: shift from p+1
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t, p, i), 1.0),
                                    (idx_z(n, t - 1, p + 1, i), -1.0),
                                    (idx_d(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t - 1, p + 1, i), 1.0),
                                    (idx_z(n, t, p, i), -1.0),
                                    (idx_d(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                        } else {
                            // p == n-1: last slot must be empty
                            constraints.push(LinearConstraint::le(
                                vec![(idx_z(n, t, n - 1, i), 1.0), (idx_d(n, k, t, j), 1.0)],
                                1.0,
                            ));
                        }
                    }

                    // Swap at position j
                    for j in 0..nm1 {
                        if p != j && p != j + 1 {
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t, p, i), 1.0),
                                    (idx_z(n, t - 1, p, i), -1.0),
                                    (idx_s(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t - 1, p, i), 1.0),
                                    (idx_z(n, t, p, i), -1.0),
                                    (idx_s(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                        } else if p == j {
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t, j, i), 1.0),
                                    (idx_z(n, t - 1, j + 1, i), -1.0),
                                    (idx_s(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t - 1, j + 1, i), 1.0),
                                    (idx_z(n, t, j, i), -1.0),
                                    (idx_s(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                        } else {
                            // p == j+1
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t, j + 1, i), 1.0),
                                    (idx_z(n, t - 1, j, i), -1.0),
                                    (idx_s(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                            constraints.push(LinearConstraint::le(
                                vec![
                                    (idx_z(n, t - 1, j, i), 1.0),
                                    (idx_z(n, t, j + 1, i), -1.0),
                                    (idx_s(n, k, t, j), 1.0),
                                ],
                                1.0,
                            ));
                        }
                    }
                }
            }
        }

        // === Final state equals target ===
        for p in 0..m {
            let terms: Vec<(usize, f64)> = (0..n)
                .filter(|&i| source[i] == target[p])
                .map(|i| (idx_z(n, k, p, i), 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }
        for p in m..n {
            constraints.push(LinearConstraint::eq(vec![(idx_e(n, k, k, p), 1.0)], 1.0));
        }

        let target_ilp = ILP::new(nv, constraints, vec![], ObjectiveSense::Minimize);
        ReductionSTSCToILP {
            target: target_ilp,
            n,
            bound: k,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "stringtostringcorrection_to_ilp",
        build: || {
            // source=[0,1,0], target=[1,0], bound=1 (delete position 0)
            let source = StringToStringCorrection::new(2, vec![0, 1, 0], vec![1, 0], 1);
            let reduction: ReductionSTSCToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let target_config = {
                let ilp_solver = crate::solvers::ILPSolver::new();
                ilp_solver
                    .solve(reduction.target_problem())
                    .expect("ILP should be solvable")
            };
            let source_config = reduction.extract_solution(&target_config);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/stringtostringcorrection_ilp.rs"]
mod tests;
