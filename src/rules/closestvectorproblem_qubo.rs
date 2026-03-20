//! Reduction from ClosestVectorProblem to QUBO.
//!
//! Encodes each bounded CVP coefficient with an exact in-range binary basis and
//! expands the squared-distance objective into a QUBO over those bits.

#[cfg(feature = "example-db")]
use crate::export::SolutionPair;
use crate::models::algebraic::{ClosestVectorProblem, QUBO};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
struct EncodingSpan {
    start: usize,
    weights: Vec<usize>,
}

/// Result of reducing a bounded ClosestVectorProblem instance to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionCVPToQUBO {
    target: QUBO<f64>,
    encodings: Vec<EncodingSpan>,
}

impl ReductionResult for ReductionCVPToQUBO {
    type Source = ClosestVectorProblem<i32>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Reconstruct the source configuration offsets from the encoded QUBO bits.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.encodings
            .iter()
            .map(|encoding| {
                encoding
                    .weights
                    .iter()
                    .enumerate()
                    .map(|(offset, weight)| {
                        target_solution
                            .get(encoding.start + offset)
                            .copied()
                            .unwrap_or(0)
                            * weight
                    })
                    .sum()
            })
            .collect()
    }
}

#[cfg(feature = "example-db")]
fn canonical_cvp_instance() -> ClosestVectorProblem<i32> {
    ClosestVectorProblem::new(
        vec![vec![2, 0], vec![1, 2]],
        vec![2.8, 1.5],
        vec![
            crate::models::algebraic::VarBounds::bounded(-2, 4),
            crate::models::algebraic::VarBounds::bounded(-2, 4),
        ],
    )
}

fn encoding_spans(problem: &ClosestVectorProblem<i32>) -> Vec<EncodingSpan> {
    let mut start = 0usize;
    let mut spans = Vec::with_capacity(problem.num_basis_vectors());
    for bounds in problem.bounds() {
        let weights = bounds
            .exact_encoding_weights()
            .into_iter()
            .map(|weight| usize::try_from(weight).expect("encoding weights must be nonnegative"))
            .collect::<Vec<_>>();
        spans.push(EncodingSpan { start, weights });
        start += spans.last().expect("just pushed").weights.len();
    }
    spans
}

fn gram_matrix(problem: &ClosestVectorProblem<i32>) -> Vec<Vec<f64>> {
    let basis = problem.basis();
    let n = basis.len();
    let mut gram = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in i..n {
            let dot = basis[i]
                .iter()
                .zip(&basis[j])
                .map(|(&lhs, &rhs)| lhs as f64 * rhs as f64)
                .sum::<f64>();
            gram[i][j] = dot;
            gram[j][i] = dot;
        }
    }
    gram
}

fn at_times_target(problem: &ClosestVectorProblem<i32>) -> Vec<f64> {
    problem
        .basis()
        .iter()
        .map(|column| {
            column
                .iter()
                .zip(problem.target())
                .map(|(&entry, &target)| entry as f64 * target)
                .sum()
        })
        .collect()
}

#[reduction(overhead = { num_vars = "num_encoding_bits" })]
impl ReduceTo<QUBO<f64>> for ClosestVectorProblem<i32> {
    type Result = ReductionCVPToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let encodings = encoding_spans(self);
        let total_bits = encodings
            .last()
            .map(|encoding| encoding.start + encoding.weights.len())
            .unwrap_or(0);
        let mut matrix = vec![vec![0.0; total_bits]; total_bits];

        if total_bits == 0 {
            return ReductionCVPToQUBO {
                target: QUBO::from_matrix(matrix),
                encodings,
            };
        }

        let gram = gram_matrix(self);
        let h = at_times_target(self);
        let lowers = self
            .bounds()
            .iter()
            .map(|bounds| {
                bounds
                    .lower
                    .expect("CVP QUBO reduction requires finite lower bounds")
            })
            .map(|lower| lower as f64)
            .collect::<Vec<_>>();
        let g_lo_minus_h = (0..self.num_basis_vectors())
            .map(|i| {
                (0..self.num_basis_vectors())
                    .map(|j| gram[i][j] * lowers[j])
                    .sum::<f64>()
                    - h[i]
            })
            .collect::<Vec<_>>();

        let mut bit_terms = Vec::with_capacity(total_bits);
        for (var_index, encoding) in encodings.iter().enumerate() {
            for &weight in &encoding.weights {
                bit_terms.push((var_index, weight as f64));
            }
        }

        for u in 0..total_bits {
            let (var_u, weight_u) = bit_terms[u];
            matrix[u][u] =
                gram[var_u][var_u] * weight_u * weight_u + 2.0 * weight_u * g_lo_minus_h[var_u];

            for v in (u + 1)..total_bits {
                let (var_v, weight_v) = bit_terms[v];
                matrix[u][v] = 2.0 * gram[var_u][var_v] * weight_u * weight_v;
            }
        }

        ReductionCVPToQUBO {
            target: QUBO::from_matrix(matrix),
            encodings,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "closestvectorproblem_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<f64>>(
                canonical_cvp_instance(),
                SolutionPair {
                    source_config: vec![3, 3],
                    target_config: vec![0, 0, 1, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/closestvectorproblem_qubo.rs"]
mod tests;
