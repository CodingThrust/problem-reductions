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
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

type Source = ClosestVectorProblem;
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
    let size = matrix.len();
    if size == 0 {
        return Ok(1);
    }
    // Pivoted Bareiss elimination uses exact division and cubic arithmetic work.
    // BigInt preserves cancellation when intermediate products exceed i64.
    let mut matrix: Vec<Vec<BigInt>> = matrix
        .iter()
        .map(|row| row.iter().copied().map(BigInt::from).collect())
        .collect();
    let mut previous_pivot = BigInt::from(1);
    let mut negative = false;
    for column in 0..size - 1 {
        let Some(pivot_row) = (column..size).find(|&row| !matrix[row][column].is_zero()) else {
            return Ok(0);
        };
        if pivot_row != column {
            matrix.swap(column, pivot_row);
            negative = !negative;
        }
        let pivot = matrix[column][column].clone();
        for row in column + 1..size {
            for next_column in column + 1..size {
                matrix[row][next_column] = (&matrix[row][next_column] * &pivot
                    - &matrix[row][column] * &matrix[column][next_column])
                    / &previous_pivot;
            }
            matrix[row][column] = BigInt::zero();
        }
        previous_pivot = pivot;
    }
    let value = matrix[size - 1][size - 1].clone();
    i64::try_from(if negative { -value } else { value })
        .map_err(|_| overflow("computing a closest-vector determinant"))
}

fn coefficient_bounds(problem: &Source) -> Result<Vec<(i64, i64)>, crate::rules::ReductionError> {
    let rows = problem
        .independent_rows()
        .map_err(crate::rules::ReductionError::construction::<Source, Target>)?;
    let size = problem.num_basis_vectors();
    if size == 0 {
        return Ok(Vec::new());
    }
    let matrix: Vec<Vec<_>> = rows
        .iter()
        .map(|&row| problem.basis().iter().map(|column| column[row]).collect())
        .collect();
    let determinant = determinant(&matrix)?;
    if determinant == 0 {
        return Err(
            crate::rules::ReductionError::invalid_target::<Source, Target>(
                "selected closest-vector rows are not independent",
            ),
        );
    }
    let denominator = BigInt::from(determinant).abs();
    let mut adjugate = Vec::new();
    for coefficient in 0..size {
        let mut row = Vec::new();
        for selected_row in 0..size {
            let minor: Vec<Vec<_>> = (0..size)
                .filter(|&row| row != selected_row)
                .map(|row| {
                    (0..size)
                        .filter(|&column| column != coefficient)
                        .map(|column| matrix[row][column])
                        .collect()
                })
                .collect();
            let mut entry = BigInt::from(self::determinant(&minor)?);
            if (coefficient + selected_row) % 2 == 1 {
                entry = -entry;
            }
            if determinant < 0 {
                entry = -entry;
            }
            row.push(entry);
        }
        adjugate.push(row);
    }
    // Cramer's rule gives a rational center. A rounded center supplies a
    // feasible lattice point, so its squared residual bounds the optimum.
    let centers: Vec<BigInt> = adjugate
        .iter()
        .map(|row| {
            row.iter()
                .zip(&rows)
                .map(|(entry, &coordinate)| entry * problem.target()[coordinate])
                .sum()
        })
        .collect();
    let candidate: Vec<BigInt> = centers
        .iter()
        .map(|center| {
            let mut quotient = center / &denominator;
            let remainder = center % &denominator;
            if remainder.abs() * 2 >= denominator {
                quotient += remainder.signum();
            }
            quotient
        })
        .collect();
    let mut radius_squared = BigInt::zero();
    let mut zero_distance = BigInt::zero();
    for coordinate in 0..problem.ambient_dimension() {
        let target = BigInt::from(problem.target()[coordinate]);
        let lattice: BigInt = problem
            .basis()
            .iter()
            .zip(&candidate)
            .map(|(column, coefficient)| coefficient * column[coordinate])
            .sum();
        let residual = lattice - &target;
        radius_squared += &residual * &residual;
        zero_distance += &target * &target;
    }
    radius_squared = radius_squared.min(zero_distance);
    let floor = |numerator: &BigInt| {
        let quotient = numerator / &denominator;
        if numerator.is_negative() && !(numerator % &denominator).is_zero() {
            quotient - 1
        } else {
            quotient
        }
    };
    centers
        .iter()
        .zip(&adjugate)
        .map(|(center, row)| {
            // Cauchy-Schwarz: |det(A) z_i - center_i|² <= ||adj_i||² R².
            // The left side is integer, so the integer square root is exact here.
            let norm: BigInt = row.iter().map(|entry| entry * entry).sum();
            let radius = BigInt::from(
                (norm * &radius_squared)
                    .to_biguint()
                    .expect("squared radius is nonnegative")
                    .sqrt(),
            );
            let lower = -floor(&(&radius - center));
            let upper = floor(&(center + radius));
            Ok((
                i64::try_from(lower)
                    .map_err(|_| overflow("bounding closest-vector coefficients"))?,
                i64::try_from(upper)
                    .map_err(|_| overflow("bounding closest-vector coefficients"))?,
            ))
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

fn encoding_spans(
    bounds: &[(i64, i64)],
) -> Result<Vec<EncodingSpan>, crate::rules::ReductionError> {
    let mut start = 0usize;
    bounds
        .iter()
        .map(|&(lower, upper)| {
            let maximum = upper
                .checked_sub(lower)
                .ok_or_else(|| overflow("computing a closest-vector encoding range"))?;
            let weights = exact_range_weights(maximum)?;
            let span = EncodingSpan {
                start,
                weights,
                lower,
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

// Cofactor bounds give at most r^2 + d + r*h + 3 bits per coefficient,
// where r is the rank, d the ambient dimension, and h the input magnitude bits.
#[reduction(transform = upper_bound {
    num_vars = "num_basis_vectors * (num_basis_vectors^2 + ambient_dimension + num_basis_vectors * max_numeric_magnitude_bits + 3)",
    num_quadratic_terms = "(num_basis_vectors * (num_basis_vectors^2 + ambient_dimension + num_basis_vectors * max_numeric_magnitude_bits + 3))^2",
})]
impl ReduceTo<QUBO<i64>> for ClosestVectorProblem {
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
    ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3_i64, 1])
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
                    target_config: serde_json::json!(vec![true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/closestvectorproblem_qubo.rs"]
mod tests;
