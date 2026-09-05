//! Reduction from integer-target CVP to QUBO.
//!
//! The reduction derives a finite coefficient box from the lattice basis and
//! target, then expands the squared Euclidean distance over exact-range binary
//! encodings.

#[cfg(feature = "example-db")]
use crate::export::SolutionPair;
use crate::models::algebraic::{ClosestVectorProblem, QUBO};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

type Source = ClosestVectorProblem<i64>;
type Target = QUBO<i64>;

#[derive(Debug, Clone)]
struct EncodingSpan {
    start: usize,
    weights: Vec<i64>,
    lower: i64,
}

/// Result of reducing an integer-target CVP instance to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionCVPToQUBO {
    target: Target,
    encodings: Vec<EncodingSpan>,
}

impl ReductionResult for ReductionCVPToQUBO {
    type Source = Source;
    type Target = Target;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        self.encodings
            .iter()
            .map(|encoding| {
                let offset = encoding.weights.iter().enumerate().try_fold(
                    0_i64,
                    |offset, (index, &weight)| {
                        if target_solution[encoding.start + index] {
                            offset.checked_add(weight)
                        } else {
                            Some(offset)
                        }
                    },
                );
                offset
                    .and_then(|offset| encoding.lower.checked_add(offset))
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid(
                            "decoded closest-vector coefficient overflows i64",
                        )
                    })
            })
            .collect()
    }
}

fn overflow(operation: &str) -> crate::rules::ReductionError {
    crate::rules::ReductionError::integer_overflow::<Source, Target>(operation)
}

fn determinant(matrix: &[Vec<i64>]) -> Result<i64, crate::rules::ReductionError> {
    match matrix.len() {
        0 => Ok(1),
        1 => Ok(matrix[0][0]),
        size => {
            let mut result = 0_i64;
            for column in 0..size {
                let minor = (1..size)
                    .map(|row| {
                        (0..size)
                            .filter(|&next_column| next_column != column)
                            .map(|next_column| matrix[row][next_column])
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                let term = matrix[0][column]
                    .checked_mul(determinant(&minor)?)
                    .ok_or_else(|| overflow("computing a closest-vector determinant"))?;
                result = if column % 2 == 0 {
                    result.checked_add(term)
                } else {
                    result.checked_sub(term)
                }
                .ok_or_else(|| overflow("computing a closest-vector determinant"))?;
            }
            Ok(result)
        }
    }
}

fn coefficient_bounds(problem: &Source) -> Result<Vec<i64>, crate::rules::ReductionError> {
    let rows = problem
        .independent_rows()
        .map_err(crate::rules::ReductionError::construction::<Source, Target>)?;
    let size = problem.num_basis_vectors();
    if size == 0 {
        return Ok(Vec::new());
    }

    let matrix = rows
        .iter()
        .map(|&row| {
            problem
                .basis()
                .iter()
                .map(|column| column[row])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if determinant(&matrix)? == 0 {
        return Err(
            crate::rules::ReductionError::invalid_target::<Source, Target>(
                "selected closest-vector rows are not independent",
            ),
        );
    }

    let target_norm = problem.target().iter().try_fold(0_i64, |total, &value| {
        total
            .checked_add(
                value
                    .checked_abs()
                    .ok_or_else(|| overflow("taking a closest-vector target absolute value"))?,
            )
            .ok_or_else(|| overflow("computing the closest-vector target one-norm"))
    })?;
    let row_bounds = rows
        .iter()
        .map(|&row| {
            problem.target()[row]
                .checked_abs()
                .and_then(|value| value.checked_add(target_norm))
                .ok_or_else(|| overflow("computing a closest-vector selected-row bound"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    (0..size)
        .map(|coefficient| {
            (0..size).try_fold(0_i64, |bound, selected_row| {
                let minor = (0..size)
                    .filter(|&row| row != selected_row)
                    .map(|row| {
                        (0..size)
                            .filter(|&column| column != coefficient)
                            .map(|column| matrix[row][column])
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                let adjugate_magnitude = determinant(&minor)?
                    .checked_abs()
                    .ok_or_else(|| overflow("taking a closest-vector cofactor absolute value"))?;
                let term = adjugate_magnitude
                    .checked_mul(row_bounds[selected_row])
                    .ok_or_else(|| overflow("computing a closest-vector coefficient bound"))?;
                bound
                    .checked_add(term)
                    .ok_or_else(|| overflow("computing a closest-vector coefficient bound"))
            })
        })
        .collect()
}

fn exact_range_weights(maximum: i64) -> Result<Vec<i64>, crate::rules::ReductionError> {
    let mut weights = Vec::new();
    let mut remaining = maximum;
    let mut power = 1_i64;
    while remaining > 0 {
        let weight = power.min(remaining);
        weights.push(weight);
        remaining -= weight;
        if remaining > 0 {
            power = power
                .checked_mul(2)
                .ok_or_else(|| overflow("computing closest-vector encoding weights"))?;
        }
    }
    Ok(weights)
}

fn encoding_spans(bounds: &[i64]) -> Result<Vec<EncodingSpan>, crate::rules::ReductionError> {
    let mut start = 0usize;
    bounds
        .iter()
        .map(|&bound| {
            let maximum = bound
                .checked_mul(2)
                .ok_or_else(|| overflow("computing a closest-vector encoding range"))?;
            let weights = exact_range_weights(maximum)?;
            let span = EncodingSpan {
                start,
                weights,
                lower: -bound,
            };
            start = start
                .checked_add(span.weights.len())
                .ok_or_else(|| overflow("computing closest-vector encoding offsets"))?;
            Ok(span)
        })
        .collect()
}

fn dot(left: &[i64], right: &[i64], operation: &str) -> Result<i64, crate::rules::ReductionError> {
    left.iter()
        .zip(right)
        .try_fold(0_i64, |total, (&left, &right)| {
            let product = left.checked_mul(right).ok_or_else(|| overflow(operation))?;
            total
                .checked_add(product)
                .ok_or_else(|| overflow(operation))
        })
}

#[reduction(transform = unavailable {
    num_vars = "the exact encoding size depends on the concrete basis and target values",
})]
impl ReduceTo<QUBO<i64>> for ClosestVectorProblem<i64> {
    type Result = ReductionCVPToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let bounds = coefficient_bounds(self)?;
        let encodings = encoding_spans(&bounds)?;
        let total_bits = encodings
            .last()
            .map(|encoding| encoding.start + encoding.weights.len())
            .unwrap_or(0);

        let size = self.num_basis_vectors();
        let mut gram = vec![vec![0_i64; size]; size];
        for (i, row) in gram.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = dot(
                    &self.basis()[i],
                    &self.basis()[j],
                    "computing a closest-vector Gram entry",
                )?;
            }
        }
        let h = self
            .basis()
            .iter()
            .map(|column| {
                dot(
                    column,
                    self.target(),
                    "computing a closest-vector target projection",
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let linear = (0..size)
            .map(|i| {
                let product = (0..size).try_fold(0_i64, |total, j| {
                    let term = gram[i][j]
                        .checked_mul(encodings[j].lower)
                        .ok_or_else(|| overflow("computing a closest-vector linear term"))?;
                    total
                        .checked_add(term)
                        .ok_or_else(|| overflow("computing a closest-vector linear term"))
                })?;
                product
                    .checked_sub(h[i])
                    .ok_or_else(|| overflow("computing a closest-vector linear term"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let bit_terms = encodings
            .iter()
            .enumerate()
            .flat_map(|(coefficient, encoding)| {
                encoding
                    .weights
                    .iter()
                    .map(move |&weight| (coefficient, weight))
            })
            .collect::<Vec<_>>();
        let mut integer_matrix = vec![vec![0_i64; total_bits]; total_bits];
        for u in 0..total_bits {
            let (coefficient_u, weight_u) = bit_terms[u];
            let quadratic = gram[coefficient_u][coefficient_u]
                .checked_mul(weight_u)
                .and_then(|value| value.checked_mul(weight_u))
                .ok_or_else(|| overflow("computing a closest-vector QUBO diagonal"))?;
            let linear_term = linear[coefficient_u]
                .checked_mul(weight_u)
                .and_then(|value| value.checked_mul(2))
                .ok_or_else(|| overflow("computing a closest-vector QUBO diagonal"))?;
            integer_matrix[u][u] = quadratic
                .checked_add(linear_term)
                .ok_or_else(|| overflow("computing a closest-vector QUBO diagonal"))?;

            for v in (u + 1)..total_bits {
                let (coefficient_v, weight_v) = bit_terms[v];
                integer_matrix[u][v] = gram[coefficient_u][coefficient_v]
                    .checked_mul(weight_u)
                    .and_then(|value| value.checked_mul(weight_v))
                    .and_then(|value| value.checked_mul(2))
                    .ok_or_else(|| overflow("computing a closest-vector QUBO interaction"))?;
            }
        }

        Ok(ReductionCVPToQUBO {
            target: QUBO::from_matrix(integer_matrix)
                .map_err(crate::rules::ReductionError::construction::<Source, Target>)?,
            encodings,
        })
    }
}

#[cfg(feature = "example-db")]
fn canonical_cvp_instance() -> Source {
    ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 2])
        .expect("canonical closest-vector instance must be valid")
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "closestvectorproblem_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
                canonical_cvp_instance(),
                SolutionPair {
                    source_config: serde_json::json!(vec![1, 1]),
                    target_config: serde_json::json!(vec![
                        false, false, false, true, true, false, false, true, false, false, true,
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/closestvectorproblem_qubo.rs"]
mod tests;
