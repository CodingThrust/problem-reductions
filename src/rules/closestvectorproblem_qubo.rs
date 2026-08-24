//! Reduction from ClosestVectorProblem to QUBO.
//!
//! Encodes each bounded CVP coefficient with an exact in-range binary basis and
//! expands the squared-distance objective into a QUBO over those bits.

#[cfg(feature = "example-db")]
use crate::export::SolutionPair;
use crate::models::algebraic::{ClosestVectorProblem, QUBO};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

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
    type Source = ClosestVectorProblem<i64>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Reconstruct the source configuration offsets from the encoded QUBO bits.
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            self.encodings
                .iter()
                .map(|encoding| {
                    encoding
                        .weights
                        .iter()
                        .enumerate()
                        .map(|(offset, weight)| target_solution[encoding.start + offset] * weight)
                        .sum()
                })
                .collect()
        })
    }
}

#[cfg(feature = "example-db")]
fn canonical_cvp_instance() -> ClosestVectorProblem<i64> {
    ClosestVectorProblem::new(
        vec![vec![2, 0], vec![1, 2]],
        vec![2.8, 1.5],
        vec![
            crate::models::algebraic::VarBounds::bounded(-2, 4),
            crate::models::algebraic::VarBounds::bounded(-2, 4),
        ],
    )
    .expect("canonical closest-vector instance must be valid")
}

fn encoding_spans(
    problem: &ClosestVectorProblem<i64>,
) -> Result<Vec<EncodingSpan>, crate::rules::ReductionError> {
    let mut start = 0usize;
    let mut spans = Vec::with_capacity(problem.num_basis_vectors());
    for bounds in problem.bounds() {
        let weights = bounds
            .exact_encoding_weights()
            .map_err(
                crate::rules::ReductionError::construction::<ClosestVectorProblem<i64>, QUBO<f64>>,
            )?
            .into_iter()
            .map(|weight| {
                usize::try_from(weight).map_err(|_| {
                    crate::rules::ReductionError::integer_overflow::<
                        ClosestVectorProblem<i64>,
                        QUBO<f64>,
                    >("converting a CVP encoding weight to usize")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let next_start = start.checked_add(weights.len()).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<ClosestVectorProblem<i64>, QUBO<f64>>(
                "computing CVP encoding offsets",
            )
        })?;
        spans.push(EncodingSpan { start, weights });
        start = next_start;
    }
    Ok(spans)
}

fn gram_matrix(
    problem: &ClosestVectorProblem<i64>,
) -> Result<Vec<Vec<f64>>, crate::rules::ReductionError> {
    let basis = problem.basis();
    let n = basis.len();
    let mut gram = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in i..n {
            let dot = basis[i]
                .iter()
                .zip(&basis[j])
                .try_fold(0_i64, |total, (&lhs, &rhs)| {
                    let product = lhs.checked_mul(rhs).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<
                            ClosestVectorProblem<i64>,
                            QUBO<f64>,
                        >("multiplying closest-vector basis entries")
                    })?;
                    total.checked_add(product).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<
                            ClosestVectorProblem<i64>,
                            QUBO<f64>,
                        >("summing a closest-vector Gram entry")
                    })
                })?;
            let dot = i64_to_exact_f64(dot).map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    ClosestVectorProblem<i64>,
                    QUBO<f64>,
                >(error)
            })?;
            gram[i][j] = dot;
            gram[j][i] = dot;
        }
    }
    Ok(gram)
}

fn at_times_target(
    problem: &ClosestVectorProblem<i64>,
) -> Result<Vec<f64>, crate::rules::ReductionError> {
    problem
        .basis()
        .iter()
        .map(|column| {
            column
                .iter()
                .zip(problem.target())
                .try_fold(0.0, |total, (&entry, &target)| {
                    let entry = i64_to_exact_f64(entry).map_err(|error| {
                        crate::rules::ReductionError::inexact_float_conversion::<
                            ClosestVectorProblem<i64>,
                            QUBO<f64>,
                        >(error)
                    })?;
                    let result = total + entry * target;
                    if result.is_finite() {
                        Ok(result)
                    } else {
                        Err(crate::rules::ReductionError::non_finite_result::<
                            ClosestVectorProblem<i64>,
                            QUBO<f64>,
                        >(
                            "computing A^T times the target produced a non-finite value",
                        ))
                    }
                })
        })
        .collect()
}

#[reduction(size = exact {
    num_vars = "num_encoding_bits",
})]
impl ReduceTo<QUBO<f64>> for ClosestVectorProblem<i64> {
    type Result = ReductionCVPToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let encodings = encoding_spans(self)?;
        let total_bits = encodings
            .last()
            .map(|encoding| encoding.start + encoding.weights.len())
            .unwrap_or(0);
        let mut matrix = vec![vec![0.0; total_bits]; total_bits];

        if total_bits == 0 {
            return Ok(ReductionCVPToQUBO {
                target:
                    QUBO::from_matrix(matrix).map_err(|message| {
                        crate::rules::ReductionError::construction::<
                            ClosestVectorProblem<i64>,
                            QUBO<f64>,
                        >(message)
                    })?,
                encodings,
            });
        }

        let gram = gram_matrix(self)?;
        let h = at_times_target(self)?;
        let lowers = self
            .bounds()
            .iter()
            .map(|bounds| {
                bounds.lower.ok_or_else(|| {
                    crate::rules::ReductionError::invalid_target::<
                        ClosestVectorProblem<i64>,
                        QUBO<f64>,
                    >("QUBO encoding requires finite lower bounds")
                })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|lower| {
                i64_to_exact_f64(lower).map_err(|error| {
                    crate::rules::ReductionError::inexact_float_conversion::<
                        ClosestVectorProblem<i64>,
                        QUBO<f64>,
                    >(error)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let g_lo_minus_h = (0..self.num_basis_vectors())
            .map(|i| {
                let product_sum = (0..self.num_basis_vectors())
                    .map(|j| gram[i][j] * lowers[j])
                    .try_fold(0.0, |total, value| {
                        let result = total + value;
                        result.is_finite().then_some(result).ok_or_else(|| {
                            crate::rules::ReductionError::non_finite_result::<
                                ClosestVectorProblem<i64>,
                                QUBO<f64>,
                            >("computing a closest-vector linear term")
                        })
                    })?;
                let result = product_sum - h[i];
                result.is_finite().then_some(result).ok_or_else(|| {
                    crate::rules::ReductionError::non_finite_result::<
                        ClosestVectorProblem<i64>,
                        QUBO<f64>,
                    >("computing a closest-vector linear term")
                })
            })
            .collect::<Result<Vec<_>, crate::rules::ReductionError>>()?;

        let mut bit_terms = Vec::with_capacity(total_bits);
        for (var_index, encoding) in encodings.iter().enumerate() {
            for &weight in &encoding.weights {
                let weight = i64::try_from(weight).map_err(|_| {
                    crate::rules::ReductionError::integer_overflow::<
                        ClosestVectorProblem<i64>,
                        QUBO<f64>,
                    >("converting a closest-vector encoding weight to i64")
                })?;
                let weight = i64_to_exact_f64(weight).map_err(|error| {
                    crate::rules::ReductionError::inexact_float_conversion::<
                        ClosestVectorProblem<i64>,
                        QUBO<f64>,
                    >(error)
                })?;
                bit_terms.push((var_index, weight));
            }
        }

        for u in 0..total_bits {
            let (var_u, weight_u) = bit_terms[u];
            matrix[u][u] =
                gram[var_u][var_u] * weight_u * weight_u + 2.0 * weight_u * g_lo_minus_h[var_u];
            if !matrix[u][u].is_finite() {
                return Err(crate::rules::ReductionError::non_finite_result::<
                    ClosestVectorProblem<i64>,
                    QUBO<f64>,
                >(
                    "computing a closest-vector QUBO diagonal produced a non-finite value",
                ));
            }

            for (v, &(var_v, weight_v)) in bit_terms.iter().enumerate().skip(u + 1) {
                matrix[u][v] = 2.0 * gram[var_u][var_v] * weight_u * weight_v;
                if !matrix[u][v].is_finite() {
                    return Err(crate::rules::ReductionError::non_finite_result::<
                        ClosestVectorProblem<i64>,
                        QUBO<f64>,
                    >(
                        "computing a closest-vector QUBO interaction produced a non-finite value",
                    ));
                }
            }
        }

        Ok(ReductionCVPToQUBO {
            target: QUBO::from_matrix(matrix).map_err(|message| {
                crate::rules::ReductionError::construction::<ClosestVectorProblem<i64>, QUBO<f64>>(
                    message,
                )
            })?,
            encodings,
        })
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
