//! Reduction from MinimumMultiwayCut to QUBO.
//!
//! Variable mapping: k*n binary variables x_{u,t} for each vertex u and
//! terminal position t. x_{u,t} = 1 means vertex u is assigned to terminal t's
//! component. Variable index: u * k + t.
//!
//! QUBO Hamiltonian: H = H_A + H_B
//!
//! H_A enforces valid partition (one-hot per vertex) and terminal pinning.
//! H_B encodes the cut cost objective.
//!
//! Reference: Heidari, Dinneen & Delmas (2022).

use crate::models::algebraic::QUBO;
use crate::models::graph::MinimumMultiwayCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumMultiwayCut to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionMinimumMultiwayCutToQUBO {
    target: QUBO<i64>,
    num_vertices: usize,
    num_terminals: usize,
    edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionMinimumMultiwayCutToQUBO {
    type Source = MinimumMultiwayCut<SimpleGraph, i64>;
    type Target = QUBO<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Decode one-hot assignment: for each vertex find its terminal, then
    /// for each edge check if endpoints are in different terminals.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let k = self.num_terminals;
            let n = self.num_vertices;

            // For each vertex, find which terminal position it is assigned to
            let assignments: Vec<usize> = (0..n)
                .map(|vertex| {
                    let mut selected =
                        (0..k).filter(|&terminal| target_solution[vertex * k + terminal]);
                    match (selected.next(), selected.next()) {
                        (Some(terminal), None) => Ok(terminal),
                        _ => Err(crate::rules::ExtractionError::invalid(format!(
                            "vertex {vertex} does not have exactly one terminal assignment"
                        ))),
                    }
                })
                .collect::<crate::rules::ExtractionResult<_>>()?;

            // For each edge, output 1 (cut) if endpoints differ, 0 (keep) otherwise
            self.edges
                .iter()
                .map(|&(u, v)| assignments[u] != assignments[v])
                .collect()
        })
    }
}

#[reduction(transform = exact {
    num_vars = "num_terminals * num_vertices",
})]
impl ReduceTo<QUBO<i64>> for MinimumMultiwayCut<SimpleGraph, i64> {
    type Result = ReductionMinimumMultiwayCutToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let k = self.num_terminals();
        let edges = self.graph().edges();
        let edge_weights = self.edge_weights();
        let terminals = self.terminals();
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<
                MinimumMultiwayCut<SimpleGraph, i64>,
                QUBO<i64>,
            >(operation)
        };
        let nq = n
            .checked_mul(k)
            .ok_or_else(|| overflow("computing the number of QUBO variables"))?;

        // Penalty: sum of all edge weights + 1
        let alpha = edge_weights.iter().try_fold(0i64, |total, &weight| {
            total
                .checked_add(
                    weight
                        .checked_abs()
                        .ok_or_else(|| overflow("taking the absolute value of a cut weight"))?,
                )
                .ok_or_else(|| overflow("summing absolute cut weights"))
        })?;
        let alpha = alpha
            .checked_add(1)
            .ok_or_else(|| overflow("computing the partition penalty"))?;

        let mut matrix = vec![vec![0i64; nq]; nq];

        // Helper: add value to upper-triangular position
        let mut add_upper = |i: usize, j: usize, val: i64| {
            let (lo, hi) = if i <= j { (i, j) } else { (j, i) };
            matrix[lo][hi] = matrix[lo][hi]
                .checked_add(val)
                .ok_or_else(|| overflow("adding a multiway-cut QUBO coefficient"))?;
            Ok::<(), crate::rules::ReductionError>(())
        };

        // H_A: one-hot constraint per vertex
        // (1 - sum_t x_{u,t})^2 = 1 - sum_t x_{u,t} + 2 * sum_{s<t} x_{u,s} * x_{u,t}
        // (using x^2 = x for binary variables)
        for u in 0..n {
            // Diagonal: -alpha for each terminal position
            for s in 0..k {
                add_upper(
                    u * k + s,
                    u * k + s,
                    alpha
                        .checked_neg()
                        .ok_or_else(|| overflow("negating the partition penalty"))?,
                )?;
            }
            // Off-diagonal within same vertex: +2*alpha for each pair
            for s in 0..k {
                for t in (s + 1)..k {
                    add_upper(
                        u * k + s,
                        u * k + t,
                        alpha
                            .checked_mul(2)
                            .ok_or_else(|| overflow("doubling the partition penalty"))?,
                    )?;
                }
            }
        }

        // H_A: terminal pinning
        // For each terminal vertex, penalize assignment to wrong position
        for (t_pos, &t_vertex) in terminals.iter().enumerate() {
            for s in 0..k {
                if s != t_pos {
                    add_upper(t_vertex * k + s, t_vertex * k + s, alpha)?;
                }
            }
        }

        // H_B: cut cost
        // For each edge (u,v) with weight w, for each pair of distinct
        // terminal positions s != t: add w to Q[u*k+s, v*k+t]
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            let w = edge_weights[edge_idx];
            for s in 0..k {
                for t in 0..k {
                    if s != t {
                        add_upper(u * k + s, v * k + t, w)?;
                    }
                }
            }
        }

        Ok(ReductionMinimumMultiwayCutToQUBO {
            target: QUBO::from_matrix(matrix).map_err(|message| {
                crate::rules::ReductionError::construction::<
                    MinimumMultiwayCut<SimpleGraph, i64>,
                    QUBO<i64>,
                >(message)
            })?,
            num_vertices: n,
            num_terminals: k,
            edges,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimummultiwaycut_to_qubo",
        build: || {
            use crate::models::algebraic::QUBO;
            use crate::models::graph::MinimumMultiwayCut;
            use crate::topology::SimpleGraph;
            let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
            let source = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, true, true, false]),
                    target_config: serde_json::json!(vec![
                        true, false, false, false, true, false, false, true, false, false, true,
                        false, false, false, true
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummultiwaycut_qubo.rs"]
mod tests;
