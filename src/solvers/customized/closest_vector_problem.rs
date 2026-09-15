//! Integer and numerical CVP sphere enumeration in nearest-first (Schnorr--Euchner) order.

use crate::models::algebraic::ClosestVectorProblem;
use crate::solvers::SolveError;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};

type GramSchmidtData = (Vec<Vec<BigRational>>, Vec<BigRational>, Vec<BigRational>);

pub(crate) fn solve(problem: &ClosestVectorProblem<i64>) -> Result<Vec<i64>, SolveError> {
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
                .map(|&entry| BigRational::from_integer(entry.into()))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let target = problem
        .target()
        .iter()
        .map(|&v| BigRational::from_integer(v.into()))
        .collect::<Vec<_>>();

    let (mu, norms, alpha) = gram_schmidt(&basis, &target);
    let mut best_squared = (0..n).map(|i| &norms[i] * &alpha[i] * &alpha[i]).sum();

    let mut coefficients = vec![BigInt::zero(); n];
    let mut best = coefficients.clone();
    enumerate(
        n - 1,
        BigRational::zero(),
        &mu,
        &norms,
        &alpha,
        &mut coefficients,
        &mut best,
        &mut best_squared,
    );
    best.into_iter()
        .map(|v| {
            v.to_i64()
                .ok_or_else(|| SolveError::IntegerOverflow("returning a CVP coefficient".into()))
        })
        .collect()
}

fn gram_schmidt(basis: &[Vec<BigRational>], target: &[BigRational]) -> GramSchmidtData {
    let n = basis.len();
    let mut orthogonal = basis.to_vec();
    let mut mu = vec![vec![BigRational::zero(); n]; n];
    let mut norms = vec![BigRational::zero(); n];

    for i in 0..n {
        for j in 0..i {
            let dot: BigRational = basis[i]
                .iter()
                .zip(&orthogonal[j])
                .map(|(left, right)| left * right)
                .sum();
            mu[i][j] = dot / &norms[j];
            for row in 0..orthogonal[i].len() {
                let projection = &mu[i][j] * &orthogonal[j][row];
                orthogonal[i][row] -= projection;
            }
        }
        // Construction guarantees independent columns, so exact norms are positive.
        norms[i] = orthogonal[i].iter().map(|value| value * value).sum();
    }

    let alpha = orthogonal
        .iter()
        .zip(&norms)
        .map(|(column, norm)| {
            target
                .iter()
                .zip(column)
                .map(|(left, right)| left * right)
                .sum::<BigRational>()
                / norm
        })
        .collect();
    (mu, norms, alpha)
}

#[allow(clippy::too_many_arguments)]
fn enumerate(
    level: usize,
    partial_squared: BigRational,
    mu: &[Vec<BigRational>],
    norms: &[BigRational],
    alpha: &[BigRational],
    coefficients: &mut [BigInt],
    best: &mut [BigInt],
    best_squared: &mut BigRational,
) {
    if partial_squared >= *best_squared {
        return;
    }

    let mut center = alpha[level].clone();
    for later in (level + 1)..coefficients.len() {
        center -= &mu[later][level] * BigRational::from_integer(coefficients[later].clone());
    }
    let mut candidate = center.round().to_integer();
    let nearest = BigRational::from_integer(candidate.clone());
    let mut step = BigInt::from(if center > nearest { 1 } else { -1 });

    // Visit the nearest integer, then alternate sides in increasing distance.
    // The first descent tries the nearest-plane candidate; every subsequent
    // branch uses the improved incumbent rather than a fixed initial interval.
    loop {
        coefficients[level] = candidate.clone();
        let delta = BigRational::from_integer(candidate.clone()) - &center;
        let next_squared = &partial_squared + &norms[level] * &delta * &delta;
        if next_squared >= *best_squared {
            break;
        }
        if level == 0 {
            *best_squared = next_squared;
            best.clone_from_slice(coefficients);
            break;
        }
        enumerate(
            level - 1,
            next_squared,
            mu,
            norms,
            alpha,
            coefficients,
            best,
            best_squared,
        );
        if partial_squared >= *best_squared {
            break;
        }
        // Differences +1,-2,+3,... (or -1,+2,-3,...) alternate around the center.
        candidate += &step;
        step = -&step - step.signum();
    }
}

type FloatGramSchmidtData = (Vec<Vec<f64>>, Vec<f64>, Vec<f64>);

pub(crate) fn solve_float(problem: &ClosestVectorProblem<f64>) -> Result<Vec<i64>, SolveError> {
    let n = problem.num_basis_vectors();
    if n == 0 {
        return Ok(Vec::new());
    }

    let (mu, norms, alpha) = float_gram_schmidt(problem.basis(), problem.target())?;
    let mut best_squared = 0.0;
    for i in 0..n {
        best_squared = finite(
            best_squared + norms[i] * alpha[i] * alpha[i],
            "computing the initial CVP sphere radius",
        )?;
    }

    let mut coefficients = vec![0_i64; n];
    let mut best = coefficients.clone();
    enumerate_float(
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

fn float_gram_schmidt(
    basis: &[Vec<f64>],
    target: &[f64],
) -> Result<FloatGramSchmidtData, SolveError> {
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
                "the basis is numerically rank deficient".into(),
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
fn enumerate_float(
    level: usize,
    partial_squared: f64,
    mu: &[Vec<f64>],
    norms: &[f64],
    alpha: &[f64],
    coefficients: &mut [i64],
    best: &mut [i64],
    best_squared: &mut f64,
) -> Result<(), SolveError> {
    if partial_squared >= *best_squared {
        return Ok(());
    }

    let mut center = alpha[level];
    for later in (level + 1)..coefficients.len() {
        let coefficient = coefficients[later] as f64;
        center = finite(
            center - mu[later][level] * coefficient,
            "computing a CVP enumeration center",
        )?;
    }
    let mut candidate = center
        .round()
        .to_i64()
        .ok_or_else(|| SolveError::IntegerOverflow("rounding a CVP enumeration center".into()))?;
    let nearest = candidate as f64;
    let mut step = if center > nearest { 1_i64 } else { -1 };

    // Visit the nearest integer, then alternate sides in increasing distance.
    // The first descent tries the nearest-plane candidate; every subsequent
    // branch uses the improved incumbent rather than a fixed initial interval.
    loop {
        coefficients[level] = candidate;
        let delta = candidate as f64 - center;
        let next_squared = finite(
            partial_squared + norms[level] * delta * delta,
            "computing a CVP partial distance",
        )?;
        if next_squared >= *best_squared {
            break;
        }
        if level == 0 {
            *best_squared = next_squared;
            best.clone_from_slice(coefficients);
            break;
        }
        enumerate_float(
            level - 1,
            next_squared,
            mu,
            norms,
            alpha,
            coefficients,
            best,
            best_squared,
        )?;
        if partial_squared >= *best_squared {
            break;
        }
        // Differences +1,-2,+3,... (or -1,+2,-3,...) alternate around the center.
        candidate = candidate.checked_add(step).ok_or_else(|| {
            SolveError::IntegerOverflow("advancing a numerical CVP coefficient".into())
        })?;
        step = step
            .checked_neg()
            .and_then(|v| v.checked_sub(step.signum()))
            .ok_or_else(|| SolveError::IntegerOverflow("advancing a numerical CVP step".into()))?;
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
