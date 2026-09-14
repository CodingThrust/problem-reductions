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
use crate::solvers::ProblemOutcome;
use crate::solvers::SolveOutcome;
use crate::topology::SimpleGraph;
use std::collections::HashMap;

/// Result of reducing TravelingSalesman to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionTravelingSalesmanToQUBO {
    target: QUBO<i64>,
    num_vertices: usize,
    num_edges: usize,
    edge_index: HashMap<(usize, usize), usize>,
    objective_offset: i128,
    feasible_energy_upper: i128,
    small_optimum: Option<(Vec<bool>, i64)>,
}

impl ReductionResult for ReductionTravelingSalesmanToQUBO {
    type Source = TravelingSalesman<SimpleGraph, i64>;
    type Target = QUBO<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Decode an optimum whose value relation establishes source feasibility.
    /// The energy gap guarantees a permutation using existing source edges.
    fn recover_result(
        &self,
        source: &Self::Source,
        target: ProblemOutcome<Self::Target>,
    ) -> crate::rules::ExtractionResult<ProblemOutcome<Self::Source>> {
        match target {
            SolveOutcome::Infeasible => Ok(SolveOutcome::Infeasible),
            SolveOutcome::Optimal {
                solution,
                evaluation,
            } => {
                if !self.map_value(evaluation).is_valid() {
                    return Ok(SolveOutcome::Infeasible);
                }
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::optimal(source, solution)?)
            }
            SolveOutcome::Feasible {
                solution,
                evaluation,
            } => {
                if !self.map_value(evaluation).is_valid() {
                    return Err(crate::rules::ExtractionError::InsufficientSolutionQuality);
                }
                let solution = self.map_solution(&solution)?;
                Ok(SolveOutcome::feasible(source, solution)?)
            }
        }
    }
}

impl ReductionTravelingSalesmanToQUBO {
    fn map_solution(
        &self,
        target_solution: &<<Self as ReductionResult>::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<
        <<Self as ReductionResult>::Source as crate::traits::Problem>::Solution,
    > {
        if self.num_vertices < 3 {
            return Ok(self.small_optimum.as_ref().unwrap().0.clone());
        }
        let n = self.num_vertices;
        let tour: Vec<usize> = (0..n)
            .map(|position| {
                (0..n)
                    .find(|&vertex| target_solution[vertex * n + position])
                    .unwrap()
            })
            .collect();
        let mut config = vec![false; self.num_edges];
        for p in 0..n {
            let (u, v) = (tour[p], tour[(p + 1) % n]);
            config[self.edge_index[&(u.min(v), u.max(v))]] = true;
        }
        Ok(config)
    }
}

impl ReductionTravelingSalesmanToQUBO {
    fn map_value(&self, value: crate::types::Min<i64>) -> crate::types::Min<i64> {
        if self.num_vertices < 3 {
            return crate::types::Min(
                value
                    .0
                    .and(self.small_optimum.as_ref().map(|(_, cost)| *cost)),
            );
        }
        crate::types::Min(
            value.0.filter(|&energy| i128::from(energy) < self.feasible_energy_upper)
            // Construction bounds source tour costs by a representable sum of
            // absolute edge weights; the offset is calculated in i128.
            .map(|energy| i64::try_from(i128::from(energy) + self.objective_offset).unwrap()),
        )
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
                target: QUBO::from_sparse(sprs::CsMat::zero((dim, dim)))
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
            .checked_add(1)
            .ok_or_else(|| overflow("computing the tour penalty"))?;
        let omitted_constant = 2 * n as i128 * i128::from(a);
        let objective_offset = omitted_constant + n as i128 * i128::from(shift);
        let feasible_energy_upper = i128::from(a) - omitted_constant;

        // Build n^2 x n^2 upper-triangular QUBO matrix
        let mut matrix = vec![std::collections::BTreeMap::new(); dim];

        // Helper: add value to upper-triangular position
        let mut add_upper = |i: usize, j: usize, val: i64| {
            let (lo, hi) = if i <= j { (i, j) } else { (j, i) };
            let coefficient = matrix[lo].entry(hi).or_insert(0i64);
            *coefficient = coefficient
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

        let target =
            QUBO::from_rows(matrix).map_err(<Self as ReduceTo<QUBO<i64>>>::target_construction)?;

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
