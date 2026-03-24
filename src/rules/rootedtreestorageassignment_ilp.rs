//! Reduction from RootedTreeStorageAssignment to ILP (Integer Linear Programming).
//!
//! Uses parent indicators p_{v,u}, depth variables d_v, ancestor indicators
//! a_{u,v}, transitive-closure helpers h_{u,v,w}, and per-subset gadgets
//! (top/bottom selectors, pair selectors, endpoint depths, extension costs).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::RootedTreeStorageAssignment;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

// Index helpers

fn idx_p(n: usize, v: usize, u: usize) -> usize {
    v * n + u
}

fn idx_d(n: usize, v: usize) -> usize {
    n * n + v
}

fn idx_a(n: usize, u: usize, v: usize) -> usize {
    n * n + n + u * n + v
}

fn idx_h(n: usize, u: usize, v: usize, w: usize) -> usize {
    2 * n * n + n + (u * n + v) * n + w
}

fn idx_t(n: usize, r: usize, s: usize, u: usize) -> usize {
    let _ = r;
    n * n * n + 2 * n * n + n + s * n + u
}

fn idx_b(n: usize, r: usize, s: usize, v: usize) -> usize {
    n * n * n + 2 * n * n + n + r * n + s * n + v
}

fn idx_m(n: usize, r: usize, s: usize, u: usize, v: usize) -> usize {
    n * n * n + 2 * n * n + n + 2 * r * n + s * n * n + u * n + v
}

fn idx_big_t(n: usize, r: usize, s: usize) -> usize {
    n * n * n + 2 * n * n + n + 2 * r * n + r * n * n + s
}

fn idx_big_b(n: usize, r: usize, s: usize) -> usize {
    n * n * n + 2 * n * n + n + 2 * r * n + r * n * n + r + s
}

fn idx_c(n: usize, r: usize, s: usize) -> usize {
    n * n * n + 2 * n * n + n + 2 * r * n + r * n * n + 2 * r + s
}

fn total_vars(n: usize, r: usize) -> usize {
    n * n * n + 2 * n * n + n + r * (n * n + 2 * n + 3)
}

#[derive(Debug, Clone)]
pub struct ReductionRTSAToILP {
    target: ILP<i32>,
    n: usize,
}

impl ReductionResult for ReductionRTSAToILP {
    type Source = RootedTreeStorageAssignment;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Decode parent array from one-hot parent indicators p_{v,u}.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        (0..n)
            .map(|v| {
                (0..n)
                    .find(|&u| target_solution[idx_p(n, v, u)] == 1)
                    .unwrap_or(v)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "universe_size * universe_size * universe_size + 2 * universe_size * universe_size + universe_size + num_subsets * (universe_size * universe_size + 2 * universe_size + 3)",
        num_constraints = "universe_size * universe_size * universe_size + universe_size * universe_size + universe_size * universe_size + num_subsets * universe_size * universe_size",
    }
)]
impl ReduceTo<ILP<i32>> for RootedTreeStorageAssignment {
    type Result = ReductionRTSAToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.universe_size();
        let subsets = self.subsets();
        let bound = self.bound();

        // Nontrivial subsets (size >= 2)
        let nontrivial: Vec<usize> = (0..subsets.len())
            .filter(|&k| subsets[k].len() >= 2)
            .collect();
        let r = nontrivial.len();

        if n == 0 {
            return ReductionRTSAToILP {
                target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
                n,
            };
        }

        let nv = total_vars(n, r);
        let big_m = n as f64;
        let big_m_depth = (n - 1) as f64;

        let mut constraints = Vec::new();

        // === Rooted-tree constraints ===

        // Σ_u p_{v,u} = 1  ∀ v
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|u| (idx_p(n, v, u), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Σ_v p_{v,v} = 1 (exactly one root)
        let root_terms: Vec<(usize, f64)> = (0..n).map(|v| (idx_p(n, v, v), 1.0)).collect();
        constraints.push(LinearConstraint::eq(root_terms, 1.0));

        // p_{v,u} binary: upper bound p_{v,u} <= 1
        for v in 0..n {
            for u in 0..n {
                constraints.push(LinearConstraint::le(vec![(idx_p(n, v, u), 1.0)], 1.0));
            }
        }

        // d_v <= (n-1)(1 - p_{v,v})  ∀ v  (root has depth 0)
        for v in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(idx_d(n, v), 1.0), (idx_p(n, v, v), big_m_depth)],
                big_m_depth,
            ));
        }

        // d_v >= 0  ∀ v
        for v in 0..n {
            constraints.push(LinearConstraint::ge(vec![(idx_d(n, v), 1.0)], 0.0));
        }

        // d_v <= n-1  ∀ v
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(idx_d(n, v), 1.0)], big_m_depth));
        }

        // For u != v: d_v - d_u >= 1 - n(1 - p_{v,u})
        //             d_v - d_u <= 1 + n(1 - p_{v,u})
        for v in 0..n {
            for u in 0..n {
                if u != v {
                    // d_v - d_u + n*p_{v,u} >= 1 - n + n = 1
                    // => d_v - d_u + n*p_{v,u} >= 1 - n*(1 - p_{v,u})
                    // Rewrite: d_v - d_u + n*p_{v,u} >= 1 - n + n*p_{v,u} ... no.
                    // Original: d_v - d_u >= 1 - n(1 - p_{v,u})
                    // => d_v - d_u + n - n*p_{v,u} >= 1
                    // => d_v - d_u - n*p_{v,u} >= 1 - n
                    constraints.push(LinearConstraint::ge(
                        vec![
                            (idx_d(n, v), 1.0),
                            (idx_d(n, u), -1.0),
                            (idx_p(n, v, u), -big_m),
                        ],
                        1.0 - big_m,
                    ));

                    // d_v - d_u <= 1 + n(1 - p_{v,u})
                    // => d_v - d_u - n + n*p_{v,u} <= 1
                    // => d_v - d_u + n*p_{v,u} <= 1 + n
                    constraints.push(LinearConstraint::le(
                        vec![
                            (idx_d(n, v), 1.0),
                            (idx_d(n, u), -1.0),
                            (idx_p(n, v, u), big_m),
                        ],
                        1.0 + big_m,
                    ));
                }
            }
        }

        // === Ancestor relation ===

        // a_{v,v} = 1  ∀ v
        for v in 0..n {
            constraints.push(LinearConstraint::eq(vec![(idx_a(n, v, v), 1.0)], 1.0));
        }

        // h_{u,v,v} = 0  ∀ u,v
        for u in 0..n {
            for v in 0..n {
                constraints.push(LinearConstraint::eq(vec![(idx_h(n, u, v, v), 1.0)], 0.0));
            }
        }

        // For u != v: a_{u,v} = Σ_w h_{u,v,w}
        for u in 0..n {
            for v in 0..n {
                if u != v {
                    let mut terms = vec![(idx_a(n, u, v), -1.0)];
                    for w in 0..n {
                        terms.push((idx_h(n, u, v, w), 1.0));
                    }
                    constraints.push(LinearConstraint::eq(terms, 0.0));
                }
            }
        }

        // h_{u,v,w} <= p_{v,w}  ∀ u,v,w with w != v
        // h_{u,v,w} <= a_{u,w}  ∀ u,v,w with w != v
        // h_{u,v,w} >= p_{v,w} + a_{u,w} - 1  ∀ u,v,w with w != v
        for u in 0..n {
            for v in 0..n {
                for w in 0..n {
                    if w != v {
                        constraints.push(LinearConstraint::le(
                            vec![(idx_h(n, u, v, w), 1.0), (idx_p(n, v, w), -1.0)],
                            0.0,
                        ));
                        constraints.push(LinearConstraint::le(
                            vec![(idx_h(n, u, v, w), 1.0), (idx_a(n, u, w), -1.0)],
                            0.0,
                        ));
                        constraints.push(LinearConstraint::ge(
                            vec![
                                (idx_h(n, u, v, w), 1.0),
                                (idx_p(n, v, w), -1.0),
                                (idx_a(n, u, w), -1.0),
                            ],
                            -1.0,
                        ));
                    }
                }
            }
        }

        // Binary bounds for a, h
        for u in 0..n {
            for v in 0..n {
                constraints.push(LinearConstraint::le(vec![(idx_a(n, u, v), 1.0)], 1.0));
                for w in 0..n {
                    constraints.push(LinearConstraint::le(vec![(idx_h(n, u, v, w), 1.0)], 1.0));
                }
            }
        }

        // === Subset gadgets ===
        for (s, &orig_k) in nontrivial.iter().enumerate() {
            let subset = &subsets[orig_k];
            let subset_size = subset.len();

            // Top selectors: Σ_{u ∈ S} t_{s,u} = 1, t_{s,u} = 0 for u ∉ S
            let top_terms: Vec<(usize, f64)> =
                subset.iter().map(|&u| (idx_t(n, r, s, u), 1.0)).collect();
            constraints.push(LinearConstraint::eq(top_terms, 1.0));
            for u in 0..n {
                if !subset.contains(&u) {
                    constraints.push(LinearConstraint::eq(vec![(idx_t(n, r, s, u), 1.0)], 0.0));
                }
                // Binary bound
                constraints.push(LinearConstraint::le(vec![(idx_t(n, r, s, u), 1.0)], 1.0));
            }

            // Bottom selectors: Σ_{v ∈ S} b_{s,v} = 1, b_{s,v} = 0 for v ∉ S
            let bot_terms: Vec<(usize, f64)> =
                subset.iter().map(|&v| (idx_b(n, r, s, v), 1.0)).collect();
            constraints.push(LinearConstraint::eq(bot_terms, 1.0));
            for v in 0..n {
                if !subset.contains(&v) {
                    constraints.push(LinearConstraint::eq(vec![(idx_b(n, r, s, v), 1.0)], 0.0));
                }
                constraints.push(LinearConstraint::le(vec![(idx_b(n, r, s, v), 1.0)], 1.0));
            }

            // Pair selectors (McCormick): m_{s,u,v} = t_{s,u} * b_{s,v}
            for u in 0..n {
                for v in 0..n {
                    constraints.push(LinearConstraint::le(
                        vec![(idx_m(n, r, s, u, v), 1.0), (idx_t(n, r, s, u), -1.0)],
                        0.0,
                    ));
                    constraints.push(LinearConstraint::le(
                        vec![(idx_m(n, r, s, u, v), 1.0), (idx_b(n, r, s, v), -1.0)],
                        0.0,
                    ));
                    constraints.push(LinearConstraint::ge(
                        vec![
                            (idx_m(n, r, s, u, v), 1.0),
                            (idx_t(n, r, s, u), -1.0),
                            (idx_b(n, r, s, v), -1.0),
                        ],
                        -1.0,
                    ));
                    constraints.push(LinearConstraint::le(vec![(idx_m(n, r, s, u, v), 1.0)], 1.0));
                }
            }

            // Path condition: m_{s,u,v} <= a_{u,v} (top is ancestor of bottom)
            for u in 0..n {
                for v in 0..n {
                    constraints.push(LinearConstraint::le(
                        vec![(idx_m(n, r, s, u, v), 1.0), (idx_a(n, u, v), -1.0)],
                        0.0,
                    ));
                }
            }

            // Every subset element w lies on the chain:
            // m_{s,u,v} <= a_{u,w} and m_{s,u,v} <= a_{w,v}  ∀ w ∈ S, u, v
            for &w in subset {
                for u in 0..n {
                    for v in 0..n {
                        constraints.push(LinearConstraint::le(
                            vec![(idx_m(n, r, s, u, v), 1.0), (idx_a(n, u, w), -1.0)],
                            0.0,
                        ));
                        constraints.push(LinearConstraint::le(
                            vec![(idx_m(n, r, s, u, v), 1.0), (idx_a(n, w, v), -1.0)],
                            0.0,
                        ));
                    }
                }
            }

            // Endpoint depths: T_s, B_s
            // T_s - d_u <= (n-1)(1 - t_{s,u})  and  d_u - T_s <= (n-1)(1 - t_{s,u})
            for &u in subset {
                constraints.push(LinearConstraint::le(
                    vec![
                        (idx_big_t(n, r, s), 1.0),
                        (idx_d(n, u), -1.0),
                        (idx_t(n, r, s, u), big_m_depth),
                    ],
                    big_m_depth,
                ));
                constraints.push(LinearConstraint::le(
                    vec![
                        (idx_d(n, u), 1.0),
                        (idx_big_t(n, r, s), -1.0),
                        (idx_t(n, r, s, u), big_m_depth),
                    ],
                    big_m_depth,
                ));
            }
            // B_s - d_v <= (n-1)(1 - b_{s,v})  and  d_v - B_s <= (n-1)(1 - b_{s,v})
            for &v in subset {
                constraints.push(LinearConstraint::le(
                    vec![
                        (idx_big_b(n, r, s), 1.0),
                        (idx_d(n, v), -1.0),
                        (idx_b(n, r, s, v), big_m_depth),
                    ],
                    big_m_depth,
                ));
                constraints.push(LinearConstraint::le(
                    vec![
                        (idx_d(n, v), 1.0),
                        (idx_big_b(n, r, s), -1.0),
                        (idx_b(n, r, s, v), big_m_depth),
                    ],
                    big_m_depth,
                ));
            }

            // Depth bounds for T_s, B_s
            constraints.push(LinearConstraint::ge(vec![(idx_big_t(n, r, s), 1.0)], 0.0));
            constraints.push(LinearConstraint::le(
                vec![(idx_big_t(n, r, s), 1.0)],
                big_m_depth,
            ));
            constraints.push(LinearConstraint::ge(vec![(idx_big_b(n, r, s), 1.0)], 0.0));
            constraints.push(LinearConstraint::le(
                vec![(idx_big_b(n, r, s), 1.0)],
                big_m_depth,
            ));

            // Extension cost: c_s = B_s - T_s + 1 - |S|
            // => c_s - B_s + T_s = 1 - |S|
            constraints.push(LinearConstraint::eq(
                vec![
                    (idx_c(n, r, s), 1.0),
                    (idx_big_b(n, r, s), -1.0),
                    (idx_big_t(n, r, s), 1.0),
                ],
                1.0 - subset_size as f64,
            ));

            // c_s >= 0
            constraints.push(LinearConstraint::ge(vec![(idx_c(n, r, s), 1.0)], 0.0));
        }

        // Total cost bound: Σ c_s <= K
        if r > 0 {
            let cost_terms: Vec<(usize, f64)> = (0..r).map(|s| (idx_c(n, r, s), 1.0)).collect();
            constraints.push(LinearConstraint::le(cost_terms, bound as f64));
        }

        let target = ILP::new(nv, constraints, vec![], ObjectiveSense::Minimize);
        ReductionRTSAToILP { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "rootedtreestorageassignment_to_ilp",
        build: || {
            let source = RootedTreeStorageAssignment::new(3, vec![vec![0, 1], vec![1, 2]], 1);
            let reduction: ReductionRTSAToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
            let target_config = {
                let ilp_solver = crate::solvers::ILPSolver::new();
                ilp_solver
                    .solve(reduction.target_problem())
                    .expect("ILP should be solvable")
            };
            let source_config = reduction.extract_solution(&target_config);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
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
#[path = "../unit_tests/rules/rootedtreestorageassignment_ilp.rs"]
mod tests;
