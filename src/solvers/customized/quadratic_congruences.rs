//! Exact bounded square roots with budgeted CRT preprocessing.
//! CRT proof: https://kconrad.math.uconn.edu/blurbs/ugradnumthy/crt.pdf
//! Hensel lifting: https://www.math-cs.gordon.edu/~kcrisman/mat338/section-74.html

use crate::models::algebraic::{QuadraticCongruences, QuadraticDiophantineEquations};
use crate::solvers::SolveError;
use num_bigint::BigUint;
use num_traits::{One, Pow, Zero};

struct WorkBudget(BigUint);

struct BudgetExceeded;

impl WorkBudget {
    fn take(&mut self, amount: BigUint) -> Result<(), BudgetExceeded> {
        if amount > self.0 {
            return Err(BudgetExceeded);
        }
        self.0 -= amount;
        Ok(())
    }
}

fn prime_powers(
    modulus: &BigUint,
    budget: &mut WorkBudget,
) -> Result<Vec<(BigUint, usize)>, BudgetExceeded> {
    let mut remaining = modulus.clone();
    let mut prime = BigUint::from(2u8);
    let mut factors = Vec::new();
    while &prime * &prime <= remaining {
        budget.take(BigUint::one())?;
        let mut exponent = 0;
        while (&remaining % &prime).is_zero() {
            budget.take(BigUint::one())?;
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
    Ok(factors)
}

/// Return the complete root classes, allowing a smaller period for nonunits.
fn local_roots(
    a: &BigUint,
    prime: &BigUint,
    exponent: usize,
    budget: &mut WorkBudget,
) -> Result<Option<(BigUint, Vec<BigUint>)>, BudgetExceeded> {
    let full = Pow::pow(prime.clone(), exponent);
    let mut unit = a % &full;
    if unit.is_zero() {
        return Ok(Some((
            Pow::pow(prime.clone(), exponent.div_ceil(2)),
            vec![BigUint::zero()],
        )));
    }
    let mut valuation = 0;
    while (&unit % prime).is_zero() {
        unit /= prime;
        valuation += 1;
    }
    if valuation % 2 != 0 {
        return Ok(None);
    }
    budget.take(prime.clone())?;
    let mut roots = Vec::new();
    let mut candidate = BigUint::zero();
    while &candidate < prime {
        if (&candidate * &candidate) % prime == &unit % prime {
            roots.push(candidate.clone());
        }
        candidate += 1u8;
    }
    if roots.is_empty() {
        return Ok(None);
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
            return Ok(None);
        }
        roots = lifted;
        period = next;
    }
    let scale: BigUint = Pow::pow(prime.clone(), valuation / 2);
    Ok(Some((
        &period * &scale,
        roots.into_iter().map(|root| root * &scale).collect(),
    )))
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

fn enumerate_root(
    a: &BigUint,
    b: &BigUint,
    mut candidate: BigUint,
    limit: &BigUint,
) -> Option<BigUint> {
    let residue = a % b;
    while &candidate <= limit {
        if (&candidate * &candidate) % b == residue {
            return Some(candidate);
        }
        candidate += 1u8;
    }
    None
}

fn bounded_root(a: &BigUint, b: &BigUint, c: &BigUint) -> Result<Option<BigUint>, SolveError> {
    if c <= &BigUint::one() {
        return Ok(None);
    }
    // A prefix finds cheap witnesses regardless of the size of c. One complete
    // positive residue period suffices when c exceeds b.
    let limit = (c - BigUint::one()).min(b.clone());
    let prefix = limit.clone().min(BigUint::from(4096u32));
    if let Some(root) = enumerate_root(a, b, BigUint::one(), &prefix) {
        return Ok(Some(root));
    }
    if prefix == limit {
        return Ok(None);
    }
    let fallback = || enumerate_root(a, b, &prefix + BigUint::one(), &limit);
    // Never spend more trials on factorization and prime-root scans than
    // there are remaining witnesses. Exhaustion requests enumeration, not NO.
    let mut budget = WorkBudget(&limit - &prefix);
    let classes = (|| {
        prime_powers(b, &mut budget)?
            .into_iter()
            .map(|(prime, exponent)| local_roots(a, &prime, exponent, &mut budget))
            .collect::<Result<Option<Vec<_>>, BudgetExceeded>>()
    })();
    let classes = match classes {
        Ok(Some(classes)) => classes,
        Ok(None) => return Ok(None),
        Err(BudgetExceeded) => return Ok(fallback()),
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
    let combinations: BigUint = [&choices[..split], &choices[split..]]
        .into_iter()
        .map(|half| {
            half.iter()
                .map(|roots| BigUint::from(roots.len()))
                .product::<BigUint>()
        })
        .sum();
    if budget.take(combinations).is_err() {
        return Ok(fallback());
    }
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
