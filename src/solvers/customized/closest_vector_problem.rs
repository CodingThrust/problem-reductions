//! Exact-rational CVP sphere enumeration in nearest-first (Schnorr--Euchner) order.

use crate::models::algebraic::{ClosestVectorProblem, ClosestVectorTarget};
use crate::solvers::SolveError;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};

type GramSchmidtData = (Vec<Vec<BigRational>>, Vec<BigRational>, Vec<BigRational>);

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
                .map(|&entry| {
                    crate::types::i64_to_exact_f64(entry)?;
                    Ok(BigRational::from_integer(entry.into()))
                })
                .collect::<Result<Vec<_>, SolveError>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let target = problem
        .target()
        .iter()
        .map(|coordinate| {
            let value = coordinate.to_f64().map_err(SolveError::Evaluation)?;
            BigRational::from_float(value).ok_or_else(|| {
                SolveError::NonFiniteResult("converting a CVP target to an exact rational".into())
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (mu, norms, alpha) = gram_schmidt(&basis, &target);
    let mut best_squared = (0..n).map(|i| &norms[i] * &alpha[i] * &alpha[i]).sum();

    let mut coefficients = vec![0_i64; n];
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
    )?;
    Ok(best)
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
    coefficients: &mut [i64],
    best: &mut Vec<i64>,
    best_squared: &mut BigRational,
) -> Result<(), SolveError> {
    if partial_squared >= *best_squared {
        return Ok(());
    }

    let mut center = alpha[level].clone();
    for later in (level + 1)..coefficients.len() {
        center -= &mu[later][level] * BigRational::from_integer(coefficients[later].into());
    }
    let mut candidate =
        center.round().to_integer().to_i64().ok_or_else(|| {
            SolveError::IntegerOverflow("rounding a CVP enumeration center".into())
        })?;
    crate::types::i64_to_exact_f64(candidate)?;
    let nearest = BigRational::from_integer(candidate.into());
    let mut step = if center > nearest { 1_i64 } else { -1 };

    // Visit the nearest integer, then alternate sides in increasing distance.
    // The first descent tries the nearest-plane candidate; every subsequent
    // branch uses the improved incumbent rather than a fixed initial interval.
    loop {
        coefficients[level] = candidate;
        crate::types::i64_to_exact_f64(candidate)?;
        let delta = BigRational::from_integer(candidate.into()) - &center;
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
        )?;
        if partial_squared >= *best_squared {
            break;
        }
        // Differences +1,-2,+3,... (or -1,+2,-3,...) alternate around the center.
        // Exact f64 coefficient transport keeps these i64 updates below 2^55.
        candidate += step;
        step = -step - step.signum();
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/closest_vector_problem.rs"]
mod tests;
