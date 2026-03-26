//! Reduction from StackerCrane to ILP.
//!
//! One-hot position assignment for required arcs with McCormick products
//! for consecutive-pair costs. Uses precomputed shortest-path connector
//! distances.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::StackerCrane;
use crate::reduction;
use crate::rules::ilp_helpers::one_hot_decode;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing StackerCrane to ILP.
///
/// Variable layout (all binary):
/// - `x_{i,p}` at index `i*m + p` for i,p in 0..m
/// - `z_{i,j,p}` at index `m^2 + p*m^2 + i*m + j` for i,j,p in 0..m
///
/// Total: `m^2 + m^3` variables.
#[derive(Debug, Clone)]
pub struct ReductionSCToILP {
    target: ILP<bool>,
    num_arcs: usize,
}

impl ReductionResult for ReductionSCToILP {
    type Source = StackerCrane;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Decode the permutation: for each position p, find the arc a with x_{a,p} = 1
        one_hot_decode(target_solution, self.num_arcs, self.num_arcs, 0)
    }
}

#[reduction(
    overhead = {
        num_vars = "num_arcs * num_arcs + num_arcs * num_arcs * num_arcs",
        num_constraints = "num_arcs + num_arcs + 3 * num_arcs * num_arcs * num_arcs",
    }
)]
impl ReduceTo<ILP<bool>> for StackerCrane {
    type Result = ReductionSCToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_arcs();

        if m == 0 {
            return ReductionSCToILP {
                target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
                num_arcs: 0,
            };
        }

        let num_vars = m * m + m * m * m;
        let x_idx = |i: usize, p: usize| i * m + p;
        let z_idx = |i: usize, j: usize, p: usize| m * m + p * m * m + i * m + j;

        // Compute all-pairs shortest path distances in the mixed graph
        let n = self.num_vertices();
        let distances = all_pairs_shortest_paths(
            n,
            self.arcs(),
            self.arc_lengths(),
            self.edges(),
            self.edge_lengths(),
        );

        let mut constraints = Vec::new();

        // Each arc assigned to exactly one position: sum_p x_{i,p} = 1 for all i
        for i in 0..m {
            let terms: Vec<(usize, f64)> = (0..m).map(|p| (x_idx(i, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Each position assigned exactly one arc: sum_i x_{i,p} = 1 for all p
        for p in 0..m {
            let terms: Vec<(usize, f64)> = (0..m).map(|i| (x_idx(i, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // McCormick linearization for z_{i,j,p} = x_{i,p} * x_{j,(p+1) mod m}
        for p in 0..m {
            let next_p = (p + 1) % m;
            for i in 0..m {
                for j in 0..m {
                    let head_i = self.arcs()[i].1;
                    let tail_j = self.arcs()[j].0;

                    if distances[head_i][tail_j] == i64::MAX {
                        // Infeasible pair: z_{i,j,p} = 0
                        constraints.push(LinearConstraint::eq(vec![(z_idx(i, j, p), 1.0)], 0.0));
                    } else {
                        // z <= x_{i,p}
                        constraints.push(LinearConstraint::le(
                            vec![(z_idx(i, j, p), 1.0), (x_idx(i, p), -1.0)],
                            0.0,
                        ));
                        // z <= x_{j, next_p}
                        constraints.push(LinearConstraint::le(
                            vec![(z_idx(i, j, p), 1.0), (x_idx(j, next_p), -1.0)],
                            0.0,
                        ));
                        // z >= x_{i,p} + x_{j, next_p} - 1
                        constraints.push(LinearConstraint::le(
                            vec![
                                (x_idx(i, p), 1.0),
                                (x_idx(j, next_p), 1.0),
                                (z_idx(i, j, p), -1.0),
                            ],
                            1.0,
                        ));
                    }
                }
            }
        }

        // Objective: minimize total walk length = sum_i l_i + sum_p sum_i sum_j D[head_i, tail_j] * z_{i,j,p}
        // The constant sum_i l_i is ignored by the ILP solver (additive constant doesn't affect optimum).
        let mut objective = Vec::new();
        for p in 0..m {
            for i in 0..m {
                for j in 0..m {
                    let head_i = self.arcs()[i].1;
                    let tail_j = self.arcs()[j].0;
                    let dist = distances[head_i][tail_j];
                    if dist < i64::MAX {
                        objective.push((z_idx(i, j, p), dist as f64));
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionSCToILP {
            target,
            num_arcs: m,
        }
    }
}

/// All-pairs shortest paths via Floyd-Warshall on the mixed graph.
fn all_pairs_shortest_paths(
    n: usize,
    arcs: &[(usize, usize)],
    arc_lengths: &[i32],
    edges: &[(usize, usize)],
    edge_lengths: &[i32],
) -> Vec<Vec<i64>> {
    let mut dist = vec![vec![i64::MAX; n]; n];
    for (i, row) in dist.iter_mut().enumerate() {
        row[i] = 0;
    }

    // Directed arcs
    for (&(u, v), &length) in arcs.iter().zip(arc_lengths) {
        let cost = i64::from(length);
        if cost < dist[u][v] {
            dist[u][v] = cost;
        }
    }

    // Undirected edges (both directions)
    for (&(u, v), &length) in edges.iter().zip(edge_lengths) {
        let cost = i64::from(length);
        if cost < dist[u][v] {
            dist[u][v] = cost;
        }
        if cost < dist[v][u] {
            dist[v][u] = cost;
        }
    }

    // Floyd-Warshall
    for via in 0..n {
        for src in 0..n {
            if dist[src][via] == i64::MAX {
                continue;
            }
            for dst in 0..n {
                if dist[via][dst] == i64::MAX {
                    continue;
                }
                let through = dist[src][via] + dist[via][dst];
                if through < dist[src][dst] {
                    dist[src][dst] = through;
                }
            }
        }
    }

    dist
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::rules::ReduceTo as _;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "stackercrane_to_ilp",
        build: || {
            // Simple: 3 vertices, 2 arcs, 1 edge
            let source =
                StackerCrane::new(3, vec![(0, 1), (2, 0)], vec![(1, 2)], vec![1, 1], vec![1]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/stackercrane_ilp.rs"]
mod tests;
