//! Exact CVP sphere enumeration in nearest-first (Schnorr--Euchner) order.

use crate::models::algebraic::ClosestVectorProblem;
use crate::solvers::SolveError;
use crate::traits::Problem;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};

type GramSchmidtData = (Vec<Vec<BigRational>>, Vec<BigRational>, Vec<BigRational>);

pub(crate) fn solve(problem: &ClosestVectorProblem) -> Result<Vec<i64>, SolveError> {
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
    let mut best_representable = problem.evaluate(&vec![0; n]).is_ok();
    enumerate(
        problem,
        &mut best_representable,
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
    problem: &ClosestVectorProblem,
    best_representable: &mut bool,
    level: usize,
    partial_squared: BigRational,
    mu: &[Vec<BigRational>],
    norms: &[BigRational],
    alpha: &[BigRational],
    coefficients: &mut [BigInt],
    best: &mut [BigInt],
    best_squared: &mut BigRational,
) {
    if partial_squared > *best_squared || (partial_squared == *best_squared && *best_representable)
    {
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
        if next_squared > *best_squared || (next_squared == *best_squared && *best_representable) {
            break;
        }
        if level == 0 {
            *best_squared = next_squared;
            best.clone_from_slice(coefficients);
            // Equal optima may differ in whether checked evaluation can represent
            // their lattice coordinates. Keep searching ties until one fits.
            *best_representable = coefficients
                .iter()
                .map(ToPrimitive::to_i64)
                .collect::<Option<Vec<_>>>()
                .is_some_and(|solution| problem.evaluate(&solution).is_ok());
            if *best_representable {
                break;
            }
        } else {
            enumerate(
                problem,
                best_representable,
                level - 1,
                next_squared,
                mu,
                norms,
                alpha,
                coefficients,
                best,
                best_squared,
            );
        }
        if partial_squared > *best_squared
            || (partial_squared == *best_squared && *best_representable)
        {
            break;
        }
        // Differences +1,-2,+3,... (or -1,+2,-3,...) alternate around the center.
        candidate += &step;
        step = -&step - step.signum();
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/closest_vector_problem.rs"]
mod tests;
