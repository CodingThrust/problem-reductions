//! Reduction from BottleneckTravelingSalesman to ILP (Integer Linear Programming).
//!
//! Cyclic position-assignment formulation with bottleneck variable:
//! - Binary x_{v,p}: vertex v at position p (cyclic tour)
//! - Binary z_{e,p,dir}: linearized consecutive-pair products
//! - Integer bottleneck variable b >= w_e * z_{e,p,dir}
//! - Objective: minimize b

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::BottleneckTravelingSalesman;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;

/// Result of reducing BottleneckTravelingSalesman to ILP.
///
/// Variable layout (ILP<i32>, all non-negative):
/// - `x_{v,p}` at index `v * n + p`, bounded to {0,1}
/// - `z_{e,p,dir}` at index `n^2 + 2*(e*n + p) + dir`, bounded to {0,1}
/// - `b` (bottleneck) at index `n^2 + 2*m*n`
#[derive(Debug, Clone)]
pub struct ReductionBTSPToILP {
    target: ILP<i32>,
    num_vertices: usize,
    source_edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionBTSPToILP {
    type Source = BottleneckTravelingSalesman;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract: decode tour from x variables, then mark selected edges.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;

        // Decode tour: for each position p, find vertex v with x_{v,p} = 1
        let mut tour = vec![0usize; n];
        for p in 0..n {
            for v in 0..n {
                if target_solution[v * n + p] == 1 {
                    tour[p] = v;
                    break;
                }
            }
        }

        // Map tour to edge selection
        let mut edge_selection = vec![0usize; self.source_edges.len()];
        for p in 0..n {
            let u = tour[p];
            let v = tour[(p + 1) % n];
            for (idx, &(a, b)) in self.source_edges.iter().enumerate() {
                if (a == u && b == v) || (a == v && b == u) {
                    edge_selection[idx] = 1;
                    break;
                }
            }
        }

        edge_selection
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices^2 + 2 * num_edges * num_vertices + 1",
        num_constraints = "2 * num_vertices + num_vertices^2 + 2 * num_edges * num_vertices + 6 * num_edges * num_vertices + num_vertices + 2 * num_edges * num_vertices",
    }
)]
impl ReduceTo<ILP<i32>> for BottleneckTravelingSalesman {
    type Result = ReductionBTSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let graph = self.graph();
        let edges = graph.edges();
        let m = edges.len();
        let weights = self.weights();

        let num_x = n * n;
        let num_z = 2 * m * n;
        let b_idx = num_x + num_z;
        let num_vars = num_x + num_z + 1;

        let x_idx = |v: usize, p: usize| -> usize { v * n + p };
        let z_fwd_idx = |e: usize, p: usize| -> usize { num_x + 2 * (e * n + p) };
        let z_rev_idx = |e: usize, p: usize| -> usize { num_x + 2 * (e * n + p) + 1 };

        let mut constraints = Vec::new();

        // Assignment: each vertex in exactly one position
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|p| (x_idx(v, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Assignment: each position has exactly one vertex
        for p in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|v| (x_idx(v, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Binary bounds for x variables (ILP<i32> is non-negative integer)
        for idx in 0..num_x {
            constraints.push(LinearConstraint::le(vec![(idx, 1.0)], 1.0));
        }

        // Binary bounds for z variables
        for idx in 0..num_z {
            constraints.push(LinearConstraint::le(vec![(num_x + idx, 1.0)], 1.0));
        }

        // McCormick linearization for z variables (cyclic: position (p+1) mod n)
        for (e, &(u, v)) in edges.iter().enumerate() {
            for p in 0..n {
                let p_next = (p + 1) % n;
                // Forward: z_fwd = x_{u,p} * x_{v,p_next}
                constraints.extend(mccormick_product(
                    z_fwd_idx(e, p),
                    x_idx(u, p),
                    x_idx(v, p_next),
                ));
                // Reverse: z_rev = x_{v,p} * x_{u,p_next}
                constraints.extend(mccormick_product(
                    z_rev_idx(e, p),
                    x_idx(v, p),
                    x_idx(u, p_next),
                ));
            }
        }

        // Adjacency: for each position p, exactly one edge in either direction
        for p in 0..n {
            let mut terms = Vec::new();
            for e in 0..m {
                terms.push((z_fwd_idx(e, p), 1.0));
                terms.push((z_rev_idx(e, p), 1.0));
            }
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Bottleneck: b >= w_e * z_{e,p,dir} for all e, p, dir
        for (e, &w) in weights.iter().enumerate() {
            let w_f64 = w as f64;
            for p in 0..n {
                constraints.push(LinearConstraint::ge(
                    vec![(b_idx, 1.0), (z_fwd_idx(e, p), -w_f64)],
                    0.0,
                ));
                constraints.push(LinearConstraint::ge(
                    vec![(b_idx, 1.0), (z_rev_idx(e, p), -w_f64)],
                    0.0,
                ));
            }
        }

        // Objective: minimize b
        let objective = vec![(b_idx, 1.0)];

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionBTSPToILP {
            target,
            num_vertices: n,
            source_edges: edges,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "bottlenecktravelingsalesman_to_ilp",
        build: || {
            // C4 with varying weights
            let source = BottleneckTravelingSalesman::new(
                crate::topology::SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]),
                vec![1, 2, 3, 4],
            );
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
#[path = "../unit_tests/rules/bottlenecktravelingsalesman_ilp.rs"]
mod tests;
