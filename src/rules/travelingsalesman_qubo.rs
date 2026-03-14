//! Reduction from TravelingSalesman to QUBO.
//!
//! Uses the standard position-based QUBO encoding for TSP:
//! - Binary variables x_{v,p} = 1 iff vertex v is at position p in the tour
//! - H_A: each vertex appears exactly once (row constraint)
//! - H_B: each position has exactly one vertex (column constraint)
//! - H_C: objective encoding edge costs between consecutive positions

use crate::models::algebraic::QUBO;
use crate::models::graph::TravelingSalesman;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use std::collections::HashMap;

/// Result of reducing TravelingSalesman to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionTravelingSalesmanToQUBO {
    target: QUBO<f64>,
    num_vertices: usize,
    num_edges: usize,
    edge_index: HashMap<(usize, usize), usize>,
}

impl ReductionResult for ReductionTravelingSalesmanToQUBO {
    type Source = TravelingSalesman<SimpleGraph, i32>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Decode position encoding back to edge-based configuration.
    ///
    /// The QUBO solution uses n^2 binary variables x_{v,p} (vertex v at position p).
    /// We extract the tour order, then map consecutive pairs to edge indices.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;

        // For each position p, find the vertex v where x_{v,p} == 1
        let mut tour = vec![0usize; n];
        for p in 0..n {
            for v in 0..n {
                if target_solution[v * n + p] == 1 {
                    tour[p] = v;
                    break;
                }
            }
        }

        // Build edge-based config: for each consecutive pair in the tour, mark the edge
        let mut config = vec![0usize; self.num_edges];
        for p in 0..n {
            let u = tour[p];
            let v = tour[(p + 1) % n];
            let key = (u.min(v), u.max(v));
            if let Some(&idx) = self.edge_index.get(&key) {
                config[idx] = 1;
            }
        }

        config
    }
}

#[reduction(
    id = "travelingsalesman_to_qubo_simplegraph_i32_f64",
    overhead = {
        num_vars = "num_vertices^2",
    }
)]
impl ReduceTo<QUBO<f64>> for TravelingSalesman<SimpleGraph, i32> {
    type Result = ReductionTravelingSalesmanToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();

        // Build edge weight map (both directions for undirected lookup)
        let mut edge_weight_map: HashMap<(usize, usize), f64> = HashMap::new();
        let mut weight_sum: f64 = 0.0;
        for &(u, v, w) in &edges {
            let wf = w as f64;
            edge_weight_map.insert((u, v), wf);
            edge_weight_map.insert((v, u), wf);
            weight_sum += wf.abs();
        }

        // Build edge index map: canonical (min, max) → edge index
        let graph_edges = self.graph().edges();
        let num_edges = graph_edges.len();
        let mut edge_index: HashMap<(usize, usize), usize> = HashMap::new();
        for (idx, &(u, v)) in graph_edges.iter().enumerate() {
            edge_index.insert((u.min(v), u.max(v)), idx);
        }

        // Penalty weight: must exceed any possible tour cost
        let a = 1.0 + weight_sum;

        // Build n^2 x n^2 upper-triangular QUBO matrix
        let dim = n * n;
        let mut matrix = vec![vec![0.0f64; dim]; dim];

        // Helper: add value to upper-triangular position
        let mut add_upper = |i: usize, j: usize, val: f64| {
            let (lo, hi) = if i <= j { (i, j) } else { (j, i) };
            matrix[lo][hi] += val;
        };

        // H_A: each vertex visited exactly once (row constraint)
        // For each vertex v: (sum_p x_{v,p} - 1)^2
        // = sum_p x_{v,p}^2 - 2*sum_p x_{v,p} + 1
        // = -sum_p x_{v,p} + 2*sum_{p1<p2} x_{v,p1}*x_{v,p2} + const
        for v in 0..n {
            for p in 0..n {
                // Diagonal: -A (from expanding (sum - 1)^2, the -2*x + x^2 = -x for binary)
                add_upper(v * n + p, v * n + p, -a);
            }
            for p1 in 0..n {
                for p2 in (p1 + 1)..n {
                    // Cross terms: 2*A * x_{v,p1} * x_{v,p2}
                    add_upper(v * n + p1, v * n + p2, 2.0 * a);
                }
            }
        }

        // H_B: each position has exactly one vertex (column constraint)
        // For each position p: (sum_v x_{v,p} - 1)^2
        for p in 0..n {
            for v in 0..n {
                add_upper(v * n + p, v * n + p, -a);
            }
            for v1 in 0..n {
                for v2 in (v1 + 1)..n {
                    add_upper(v1 * n + p, v2 * n + p, 2.0 * a);
                }
            }
        }

        // H_C: distance objective
        // For each pair (u, v), add cost for x_{u,p} * x_{v,p_next} and x_{v,p} * x_{u,p_next}
        for u in 0..n {
            for v in (u + 1)..n {
                let cost = edge_weight_map.get(&(u, v)).copied().unwrap_or(a);
                for p in 0..n {
                    let p_next = (p + 1) % n;
                    // x_{u,p} * x_{v,p_next}
                    add_upper(u * n + p, v * n + p_next, cost);
                    // x_{v,p} * x_{u,p_next}
                    add_upper(v * n + p, u * n + p_next, cost);
                }
            }
        }

        let target = QUBO::from_matrix(matrix);

        ReductionTravelingSalesmanToQUBO {
            target,
            num_vertices: n,
            num_edges,
            edge_index,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::models::algebraic::QUBO;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "travelingsalesman_to_qubo",
        build: || {
            let source = TravelingSalesman::new(
                SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]),
                vec![1, 2, 3],
            );
            crate::example_db::specs::direct_best_example::<_, QUBO<f64>, _>(
                source,
                |_, _| true,
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/travelingsalesman_qubo.rs"]
mod tests;
