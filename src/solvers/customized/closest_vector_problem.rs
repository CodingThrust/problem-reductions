//! Textbook floating-point sphere enumeration for CVP.

use crate::models::algebraic::{ClosestVectorProblem, ClosestVectorTarget};
use crate::solvers::SolveError;
use num_traits::ToPrimitive;

type GramSchmidtData = (Vec<Vec<f64>>, Vec<f64>, Vec<f64>);

pub(crate) fn solve<T: ClosestVectorTarget>(
    problem: &ClosestVectorProblem<T>,
) -> Result<Vec<i64>, SolveError> {
    let n = problem.num_basis_vectors();
    if n == 0 {
        return Ok(Vec::new());
    }

    let basis = problem
        .basis()
        .iter()
        .map(|column| {
            column
                .iter()
                .map(|&entry| crate::types::i64_to_exact_f64(entry).map_err(SolveError::from))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let target = problem
        .target()
        .iter()
        .map(|coordinate| coordinate.to_f64().map_err(SolveError::Evaluation))
        .collect::<Result<Vec<_>, _>>()?;

    let (mu, norms, alpha) = gram_schmidt(&basis, &target)?;
    let mut best_squared = 0.0;
    for i in 0..n {
        best_squared = finite(
            best_squared + norms[i] * alpha[i] * alpha[i],
            "computing the initial CVP sphere radius",
        )?;
    }

    let mut coefficients = vec![0_i64; n];
    let mut best = coefficients.clone();
    enumerate(
        n - 1,
        0.0,
        &mu,
        &norms,
        &alpha,
        &mut coefficients,
        &mut best,
        &mut best_squared,
    )?;
    Ok(best)
}

fn gram_schmidt(basis: &[Vec<f64>], target: &[f64]) -> Result<GramSchmidtData, SolveError> {
    let n = basis.len();
    let mut orthogonal = basis.to_vec();
    let mut mu = vec![vec![0.0; n]; n];
    let mut norms = vec![0.0; n];

    for i in 0..n {
        for j in 0..i {
            let dot = basis[i]
                .iter()
                .zip(&orthogonal[j])
                .try_fold(0.0, |total, (&left, &right)| {
                    finite(total + left * right, "computing a CVP projection")
                })?;
            mu[i][j] = finite(dot / norms[j], "computing a CVP projection")?;
            for row in 0..orthogonal[i].len() {
                orthogonal[i][row] = finite(
                    orthogonal[i][row] - mu[i][j] * orthogonal[j][row],
                    "orthogonalizing a CVP basis",
                )?;
            }
        }
        norms[i] = orthogonal[i].iter().try_fold(0.0, |total, &value| {
            finite(total + value * value, "computing a CVP Gram--Schmidt norm")
        })?;
        if norms[i] <= 0.0 {
            return Err(SolveError::NonFiniteResult(
                "the integer basis is numerically rank deficient".into(),
            ));
        }
    }

    let alpha = orthogonal
        .iter()
        .zip(&norms)
        .map(|(column, &norm)| {
            let dot = target
                .iter()
                .zip(column)
                .try_fold(0.0, |total, (&left, &right)| {
                    finite(total + left * right, "projecting the CVP target")
                })?;
            finite(dot / norm, "projecting the CVP target")
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((mu, norms, alpha))
}

#[allow(clippy::too_many_arguments)]
fn enumerate(
    level: usize,
    partial_squared: f64,
    mu: &[Vec<f64>],
    norms: &[f64],
    alpha: &[f64],
    coefficients: &mut [i64],
    best: &mut Vec<i64>,
    best_squared: &mut f64,
) -> Result<(), SolveError> {
    let remaining = *best_squared - partial_squared;
    if remaining < 0.0 {
        return Ok(());
    }

    let mut center = alpha[level];
    for later in (level + 1)..coefficients.len() {
        let coefficient = crate::types::i64_to_exact_f64(coefficients[later])?;
        center = finite(
            center - mu[later][level] * coefficient,
            "computing a CVP enumeration center",
        )?;
    }
    let radius = finite(
        (remaining / norms[level]).sqrt(),
        "computing a CVP enumeration radius",
    )?;
    let lower = (center - radius).ceil().to_i64().ok_or_else(|| {
        SolveError::IntegerOverflow("converting a CVP coefficient interval endpoint".into())
    })?;
    let upper = (center + radius).floor().to_i64().ok_or_else(|| {
        SolveError::IntegerOverflow("converting a CVP coefficient interval endpoint".into())
    })?;

    crate::types::i64_to_exact_f64(lower)?;
    crate::types::i64_to_exact_f64(upper)?;
    for candidate in lower..=upper {
        coefficients[level] = candidate;
        let candidate = crate::types::i64_to_exact_f64(candidate)?;
        let delta = candidate - center;
        let next_squared = finite(
            partial_squared + norms[level] * delta * delta,
            "computing a CVP partial distance",
        )?;
        if level == 0 {
            if next_squared < *best_squared {
                *best_squared = next_squared;
                best.clone_from_slice(coefficients);
            }
        } else {
            enumerate(
                level - 1,
                next_squared,
                mu,
                norms,
                alpha,
                coefficients,
                best,
                best_squared,
            )?;
        }
    }
    Ok(())
}

fn finite(value: f64, operation: &str) -> Result<f64, SolveError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SolveError::NonFiniteResult(operation.into()))
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/closest_vector_problem.rs"]
mod tests;
