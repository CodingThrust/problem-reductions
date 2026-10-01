//! Exact bounded square roots: prime-power lifting, CRT, and meet in the middle.
//! CRT proof: https://kconrad.math.uconn.edu/blurbs/ugradnumthy/crt.pdf
//! Hensel lifting: https://www.math-cs.gordon.edu/~kcrisman/mat338/section-74.html

use crate::models::algebraic::{QuadraticCongruences, QuadraticDiophantineEquations};
use crate::solvers::SolveError;
use num_bigint::BigUint;
use num_traits::{One, Pow, Zero};

fn prime_powers(modulus: &BigUint) -> Vec<(BigUint, usize)> {
    let mut remaining = modulus.clone();
    let mut prime = BigUint::from(2u8);
    let mut factors = Vec::new();
    // ponytail: exact trial division; use certified faster factoring if large
    // prime factors, rather than the number of CRT choices, become the bottleneck.
    while &prime * &prime <= remaining {
        let mut exponent = 0;
        while (&remaining % &prime).is_zero() {
            remaining /= &prime;
            exponent += 1;
        }
        if exponent > 0 {
            factors.push((prime.clone(), exponent));
        }
        prime += if prime == BigUint::from(2u8) {
            1u8
        } else {
            2u8
        };
    }
    if remaining > BigUint::one() {
        factors.push((remaining, 1));
    }
    factors
}

/// Return the complete root classes, allowing a smaller period for nonunits.
fn local_roots(a: &BigUint, prime: &BigUint, exponent: usize) -> Option<(BigUint, Vec<BigUint>)> {
    let full = Pow::pow(prime.clone(), exponent);
    let mut unit = a % &full;
    if unit.is_zero() {
        return Some((
            Pow::pow(prime.clone(), exponent.div_ceil(2)),
            vec![BigUint::zero()],
        ));
    }
    let mut valuation = 0;
    while (&unit % prime).is_zero() {
        unit /= prime;
        valuation += 1;
    }
    if valuation % 2 != 0 {
        return None;
    }
    let mut roots = Vec::new();
    let mut candidate = BigUint::zero();
    while &candidate < prime {
        if (&candidate * &candidate) % prime == &unit % prime {
            roots.push(candidate.clone());
        }
        candidate += 1u8;
    }
    if roots.is_empty() {
        return None;
    }
    let mut period = prime.clone();
    for _ in 1..exponent - valuation {
        let next = &period * prime;
        let mut lifted = Vec::new();
        for root in roots {
            if prime == &BigUint::from(2u8) {
                for digit in 0u8..2 {
                    let value = &root + &period * digit;
                    if (&value * &value) % &next == &unit % &next {
                        lifted.push(value);
                    }
                }
            } else {
                let difference = (&unit % &next + &next - (&root * &root) % &next) % &next;
                let inverse = (&root * 2u8)
                    .modinv(prime)
                    .expect("odd-prime unit derivative is invertible");
                let digit = (difference / &period * inverse) % prime;
                lifted.push(root + &period * digit);
            }
        }
        if lifted.is_empty() {
            return None;
        }
        roots = lifted;
        period = next;
    }
    let scale: BigUint = Pow::pow(prime.clone(), valuation / 2);
    Some((
        &period * &scale,
        roots.into_iter().map(|root| root * &scale).collect(),
    ))
}

fn half_sums(choices: &[Vec<BigUint>], modulus: &BigUint) -> Result<Vec<BigUint>, SolveError> {
    let mut sums = vec![BigUint::zero()];
    for choices in choices {
        let mut next = Vec::new();
        for choice in choices {
            next.try_reserve(sums.len())?;
            next.extend(sums.iter().map(|sum| (sum + choice) % modulus));
        }
        sums = next;
    }
    sums.sort_unstable();
    sums.dedup();
    Ok(sums)
}

fn bounded_root(a: &BigUint, b: &BigUint, c: &BigUint) -> Result<Option<BigUint>, SolveError> {
    if c <= &BigUint::one() {
        return Ok(None);
    }
    let Some(classes) = prime_powers(b)
        .into_iter()
        .map(|(prime, exponent)| local_roots(a, &prime, exponent))
        .collect::<Option<Vec<_>>>()
    else {
        return Ok(None);
    };
    let modulus: BigUint = classes.iter().map(|(period, _)| period).product();
    let choices: Vec<Vec<BigUint>> = classes
        .into_iter()
        .map(|(period, roots)| {
            let other = &modulus / &period;
            let weight = &other
                * other
                    .modinv(&period)
                    .expect("prime-power periods are coprime");
            roots
                .into_iter()
                .map(|root| root * &weight % &modulus)
                .collect()
        })
        .collect();
    if &modulus < c {
        // Every root class has a positive representative in 1..=M.
        let residue = choices.iter().map(|roots| &roots[0]).sum::<BigUint>() % &modulus;
        return Ok(Some(if residue.is_zero() { modulus } else { residue }));
    }
    let split = choices.len() / 2;
    let left = half_sums(&choices[..split], &modulus)?;
    let right = half_sums(&choices[split..], &modulus)?;
    // Since c <= M, zero residues are excluded, including after wraparound.
    let wrapped_start = &modulus + BigUint::one();
    for a in left {
        for start in [&BigUint::one(), &wrapped_start] {
            let lower = if start > &a {
                start - &a
            } else {
                BigUint::zero()
            };
            let index = right.partition_point(|b| b < &lower);
            if let Some(b) = right.get(index) {
                let residue = (&a + b) % &modulus;
                if !residue.is_zero() && &residue < c {
                    return Ok(Some(residue));
                }
            }
        }
    }
    Ok(None)
}

pub(crate) fn solve(problem: &QuadraticCongruences) -> Result<Option<BigUint>, SolveError> {
    bounded_root(problem.a(), problem.b(), problem.c())
}

pub(crate) fn solve_diophantine(
    problem: &QuadraticDiophantineEquations,
) -> Result<Option<BigUint>, SolveError> {
    if problem.c() < &(problem.a() + problem.b()) {
        return Ok(None);
    }
    let (mut gcd, mut remainder) = (problem.a().clone(), problem.b().clone());
    while !remainder.is_zero() {
        (gcd, remainder) = (remainder.clone(), gcd % remainder);
    }
    if !(problem.c() % &gcd).is_zero() {
        return Ok(None);
    }
    let a = problem.a() / &gcd;
    let b = problem.b() / &gcd;
    let residue = if b.is_one() {
        BigUint::zero()
    } else {
        problem.c() / &gcd * a.modinv(&b).expect("dividing by gcd makes a invertible") % &b
    };
    // y >= 1 is equivalent to ax² <= c-b, including equality.
    let bound = ((problem.c() - problem.b()) / problem.a()).sqrt() + BigUint::one();
    bounded_root(&residue, &b, &bound)
}
