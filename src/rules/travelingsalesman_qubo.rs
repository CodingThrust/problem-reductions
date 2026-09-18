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
use crate::topology::SimpleGraph;
use std::collections::HashMap;

/// Result of reducing TravelingSalesman to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionTravelingSalesmanToQUBO {
    target: QUBO<i64>,
    num_vertices: usize,
    num_edges: usize,
    edge_index: HashMap<(usize, usize), usize>,
    objective_offset: i64,
    feasible_energy_upper: i128,
    small_optimum: Option<(Vec<bool>, i64)>,
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
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if crate::rules::AggregateReductionResult::extract_value(self, value)
            .0
            .is_none()
        {
            return Err(crate::rules::ExtractionError::invalid(
                "target energy does not encode a feasible tour",
            ));
        }
        if self.num_vertices < 3 {
            return Ok(self
                .small_optimum
                .as_ref()
                .expect("value mapping established a small tour")
                .0
                .clone());
        }

        Ok({
            let n = self.num_vertices;

            let tour: Vec<usize> = (0..n)
                .map(|position| {
                    let mut selected =
                        (0..n).filter(|&vertex| target_solution[vertex * n + position]);
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

impl crate::rules::AggregateReductionResult for ReductionTravelingSalesmanToQUBO {
    type Source = TravelingSalesman<SimpleGraph, i64>;
    type Target = QUBO<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: crate::types::Min<i64>) -> crate::types::Min<i64> {
        if self.num_vertices < 3 {
            return crate::types::Min(
                value
                    .0
                    .and(self.small_optimum.as_ref().map(|(_, cost)| *cost)),
            );
        }
        // The offset is nonnegative; below the feasibility bound, the sum is
        // less than A + n * shift <= A, so addition cannot overflow in either direction.
        crate::types::Min(
            value
                .0
                .filter(|&energy| i128::from(energy) < self.feasible_energy_upper)
                .map(|energy| energy + self.objective_offset),
        )
    }
}

#[reduction(
    aggregate = custom,
    transform = exact {
        num_vars = "num_vertices^2",
    }
)]
impl ReduceTo<QUBO<i64>> for TravelingSalesman<SimpleGraph, i64> {
    type Result = ReductionTravelingSalesmanToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let edges = self.edges();

        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, QUBO<i64>>(operation)
        };
        let num_edges = edges.len();
        let dim = n
            .checked_mul(n)
            .ok_or_else(|| overflow("computing the number of QUBO variables"))?;

        // The source represents a connected degree-two edge set. With fewer
        // than three vertices this means one loop or two parallel edges.
        if n < 3 {
            let mut candidates: Vec<usize> = edges
                .iter()
                .enumerate()
                .filter(|&(_, &(u, v, _))| (n == 1 && u == v) || (n == 2 && u != v))
                .map(|(index, _)| index)
                .collect();

            let small_optimum = if n > 0 && candidates.len() >= n {
                candidates.select_nth_unstable_by_key(n - 1, |&index| (edges[index].2, index));
                let mut solution = vec![false; num_edges];
                let mut cost = 0i64;
                for &index in &candidates[..n] {
                    solution[index] = true;
                    cost = cost
                        .checked_add(edges[index].2)
                        .ok_or_else(|| overflow("summing a small tour cost"))?;
                }
                Some((solution, cost))
            } else {
                None
            };
            return Ok(ReductionTravelingSalesmanToQUBO {
                target: QUBO::from_matrix(vec![vec![0; dim]; dim])
                    .map_err(<Self as ReduceTo<QUBO<i64>>>::target_construction)?,
                num_vertices: n,
                num_edges,
                edge_index: HashMap::new(),
                objective_offset: 0,
                feasible_energy_upper: 0,
                small_optimum,
            });
        }

        // A tour on at least three vertices uses no loops and at most one
        // edge per endpoint pair. Retain the cheapest parallel edge.
        let mut edge_index: HashMap<(usize, usize), usize> = HashMap::new();
        for (index, &(u, v, weight)) in edges.iter().enumerate() {
            if u == v {
                continue;
            }
            let key = (u.min(v), u.max(v));
            edge_index
                .entry(key)
                .and_modify(|previous| {
                    if weight < edges[*previous].2 {
                        *previous = index;
                    }
                })
                .or_insert(index);
        }
        let shift = edge_index
            .values()
            .map(|&index| edges[index].2)
            .fold(0, i64::min);
        let mut shifted_sum = 0i64;
        let mut absolute_sum = 0i64;
        for &index in edge_index.values() {
            let weight = edges[index].2;
            absolute_sum = absolute_sum
                .checked_add(
                    weight
                        .checked_abs()
                        .ok_or_else(|| overflow("taking the absolute value of a tour weight"))?,
                )
                .ok_or_else(|| overflow("summing absolute tour weights"))?;
            let shifted = weight
                .checked_sub(shift)
                .ok_or_else(|| overflow("shifting a tour weight"))?;
            shifted_sum = shifted_sum
                .checked_add(shifted)
                .ok_or_else(|| overflow("summing shifted tour weights"))?;
        }
        // Every permutation tour uses n edges. Shifting each cost therefore
        // adds a constant. All costs are now nonnegative even off-premise.
        let a = shifted_sum
            .max(absolute_sum)
            .checked_add(1)
            .ok_or_else(|| overflow("computing the tour penalty"))?;
        let omitted_constant = 2 * n as i128 * i128::from(a);
        // A >= |shift| makes this offset positive. Check its transport once.
        let objective_offset = i64::try_from(omitted_constant + n as i128 * i128::from(shift))
            .map_err(|_| overflow("computing the tour objective offset"))?;
        let feasible_energy_upper = i128::from(a) - omitted_constant;

        // Build n^2 x n^2 upper-triangular QUBO matrix
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
                let cost = edge_index.get(&(u, v)).map_or(a, |&index| {
                    // The bound calculation already checked this subtraction.
                    edges[index].2 - shift
                });
                for p in 0..n {
                    let p_next = (p + 1) % n;
                    // x_{u,p} * x_{v,p_next}
                    add_upper(u * n + p, v * n + p_next, cost)?;
                    // x_{v,p} * x_{u,p_next}
                    add_upper(v * n + p, u * n + p_next, cost)?;
                }
            }
        }

        let target = QUBO::from_matrix(matrix)
            .map_err(<Self as ReduceTo<QUBO<i64>>>::target_construction)?;

        Ok(ReductionTravelingSalesmanToQUBO {
            target,
            num_vertices: n,
            num_edges,
            edge_index,
            objective_offset,
            feasible_energy_upper,
            small_optimum: None,
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
