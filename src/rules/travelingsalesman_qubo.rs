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
    target: QUBO<i64>,
    num_vertices: usize,
    num_edges: usize,
    edge_index: HashMap<(usize, usize), usize>,
}

impl ReductionResult for ReductionTravelingSalesmanToQUBO {
    type Source = TravelingSalesman<SimpleGraph, i64>;
    type Target = QUBO<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Decode position encoding back to edge-based configuration.
    ///
    /// The QUBO solution uses n^2 binary variables x_{v,p} (vertex v at position p).
    /// We extract the tour order, then map consecutive pairs to edge indices.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let n = self.num_vertices;

            let tour: Vec<usize> = (0..n)
                .map(|position| {
                    let mut selected =
                        (0..n).filter(|&vertex| target_solution[position * n + vertex]);
                    match (selected.next(), selected.next()) {
                        (Some(vertex), None) => Ok(vertex),
                        _ => Err(crate::rules::ExtractionError::invalid(format!(
                            "tour position {position} does not select exactly one vertex"
                        ))),
                    }
                })
                .collect::<crate::rules::ExtractionResult<_>>()?;

            // Build edge-based config: for each consecutive pair in the tour, mark the edge
            let mut config = vec![false; self.num_edges];
            for p in 0..n {
                let u = tour[p];
                let v = tour[(p + 1) % n];
                let key = (u.min(v), u.max(v));
                let &edge = self.edge_index.get(&key).ok_or_else(|| {
                    crate::rules::ExtractionError::invalid(format!(
                        "target tour uses absent source edge ({u}, {v})"
                    ))
                })?;
                config[edge] = true;
            }

            config
        })
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices^2",
    }
)]
impl ReduceTo<QUBO<i64>> for TravelingSalesman<SimpleGraph, i64> {
    type Result = ReductionTravelingSalesmanToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let edges = self.edges();

        // Build edge weight map (both directions for undirected lookup)
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<
                TravelingSalesman<SimpleGraph, i64>,
                QUBO<i64>,
            >(operation)
        };
        let mut edge_weight_map: HashMap<(usize, usize), i64> = HashMap::new();
        let mut weight_sum = 0i64;
        for &(u, v, w) in &edges {
            edge_weight_map.insert((u, v), w);
            edge_weight_map.insert((v, u), w);
            let magnitude = w
                .checked_abs()
                .ok_or_else(|| overflow("taking the absolute value of a tour weight"))?;
            weight_sum = weight_sum
                .checked_add(magnitude)
                .ok_or_else(|| overflow("summing absolute tour weights"))?;
        }

        // Build edge index map: canonical (min, max) → edge index
        let graph_edges = self.graph().edges();
        let num_edges = graph_edges.len();
        let mut edge_index: HashMap<(usize, usize), usize> = HashMap::new();
        for (idx, &(u, v)) in graph_edges.iter().enumerate() {
            edge_index.insert((u.min(v), u.max(v)), idx);
        }

        // Penalty weight: must exceed any possible tour cost
        let a = weight_sum
            .checked_add(1)
            .ok_or_else(|| overflow("computing the tour penalty"))?;

        // Build n^2 x n^2 upper-triangular QUBO matrix
        let dim = n
            .checked_mul(n)
            .ok_or_else(|| overflow("computing the number of QUBO variables"))?;
        let mut matrix = vec![vec![0i64; dim]; dim];

        // Helper: add value to upper-triangular position
        let mut add_upper = |i: usize, j: usize, val: i64| {
            let (lo, hi) = if i <= j { (i, j) } else { (j, i) };
            matrix[lo][hi] = matrix[lo][hi]
                .checked_add(val)
                .ok_or_else(|| overflow("adding a tour QUBO coefficient"))?;
            Ok::<(), crate::rules::ReductionError>(())
        };

        // H_A: each vertex visited exactly once (row constraint)
        // For each vertex v: (sum_p x_{v,p} - 1)^2
        // = sum_p x_{v,p}^2 - 2*sum_p x_{v,p} + 1
        // = -sum_p x_{v,p} + 2*sum_{p1<p2} x_{v,p1}*x_{v,p2} + const
        for v in 0..n {
            for p in 0..n {
                // Diagonal: -A (from expanding (sum - 1)^2, the -2*x + x^2 = -x for binary)
                add_upper(
                    v * n + p,
                    v * n + p,
                    a.checked_neg()
                        .ok_or_else(|| overflow("negating the tour penalty"))?,
                )?;
            }
            for p1 in 0..n {
                for p2 in (p1 + 1)..n {
                    // Cross terms: 2*A * x_{v,p1} * x_{v,p2}
                    add_upper(
                        v * n + p1,
                        v * n + p2,
                        a.checked_mul(2)
                            .ok_or_else(|| overflow("doubling the tour penalty"))?,
                    )?;
                }
            }
        }

        // H_B: each position has exactly one vertex (column constraint)
        // For each position p: (sum_v x_{v,p} - 1)^2
        for p in 0..n {
            for v in 0..n {
                add_upper(
                    v * n + p,
                    v * n + p,
                    a.checked_neg()
                        .ok_or_else(|| overflow("negating the tour penalty"))?,
                )?;
            }
            for v1 in 0..n {
                for v2 in (v1 + 1)..n {
                    add_upper(
                        v1 * n + p,
                        v2 * n + p,
                        a.checked_mul(2)
                            .ok_or_else(|| overflow("doubling the tour penalty"))?,
                    )?;
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
                    add_upper(u * n + p, v * n + p_next, cost)?;
                    // x_{v,p} * x_{u,p_next}
                    add_upper(v * n + p, u * n + p_next, cost)?;
                }
            }
        }

        let target = QUBO::from_matrix(matrix).map_err(|message| {
            crate::rules::ReductionError::construction::<
                TravelingSalesman<SimpleGraph, i64>,
                QUBO<i64>,
            >(message)
        })?;

        Ok(ReductionTravelingSalesmanToQUBO {
            target,
            num_vertices: n,
            num_edges,
            edge_index,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::algebraic::QUBO;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "travelingsalesman_to_qubo",
        build: || {
            let source = TravelingSalesman::new(
                SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]),
                vec![1, 2, 3],
            );
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, true, true]),
                    target_config: serde_json::json!(vec![
                        false, false, true, true, false, false, false, true, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/travelingsalesman_qubo.rs"]
mod tests;
