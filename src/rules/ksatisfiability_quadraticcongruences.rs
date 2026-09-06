//! Reduction from 3-SAT to bounded quadratic congruences.
//!
//! Manders and Adleman (ERL-M615, 1976), Section 2: an odd signed-knapsack
//! target and prime-power CRT encode the clauses. Only the actual normalized
//! clauses need base-eight positions; widths zero through three use the same
//! slack equation. Squaring uses twice the linear modulus, and extraction
//! orients every sign by the distinguished odd coordinate.

use std::collections::{BTreeMap, BTreeSet};

use crate::models::algebraic::QuadraticCongruences;
use crate::models::formula::KSatisfiability;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;
use num_bigint::{BigInt, BigUint};
#[cfg(any(test, feature = "example-db"))]
use num_traits::Signed;
use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct Reduction3SATToQuadraticCongruences {
    target: QuadraticCongruences,
    source_num_vars: usize,
    active_to_source: Vec<usize>,
    clause_count: usize,
    h: BigUint,
    prime_powers: Vec<BigUint>,
}

impl ReductionResult for Reduction3SATToQuadraticCongruences {
    type Source = KSatisfiability<K3>;
    type Target = QuadraticCongruences;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target integer does not satisfy the bounded quadratic congruence",
            ));
        }
        // Validation gives 0 < x <= H. Each prime power divides exactly one
        // of H-x and H+x. The coordinate zero sign chooses x or -x so that
        // the odd linear target, rather than its negative, is recovered.
        let h_minus_x = &self.h - target_solution;
        let positive_orientation = (&h_minus_x % &self.prime_powers[0]).is_zero();
        let mut assignment = vec![false; self.source_num_vars];
        for (active, &original) in self.active_to_source.iter().enumerate() {
            let coordinate = 2 * self.clause_count + active + 1;
            let positive = (&h_minus_x % &self.prime_powers[coordinate]).is_zero();
            assignment[original] = positive != positive_orientation;
        }
        Ok(assignment)
    }
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct MandersAdlemanConstruction {
    target: QuadraticCongruences,
    source_num_vars: usize,
    active_to_source: Vec<usize>,
    clauses: Vec<Vec<i64>>,
    #[cfg_attr(not(test), allow(dead_code))]
    coefficients: Vec<BigInt>,
    #[cfg_attr(not(test), allow(dead_code))]
    tau: BigInt,
    thetas: Vec<BigUint>,
    h: BigUint,
    prime_powers: Vec<BigUint>,
}

fn is_prime(candidate: u64) -> bool {
    if candidate < 2 {
        return false;
    }
    if candidate == 2 {
        return true;
    }
    if candidate.is_multiple_of(2) {
        return false;
    }
    let mut divisor = 3u64;
    while divisor <= candidate / divisor {
        if candidate.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }
    true
}

fn admissible_primes(count: usize) -> Result<Vec<u64>, crate::rules::ReductionError> {
    let mut primes = Vec::with_capacity(count);
    let mut candidate = 13u64;
    while primes.len() < count {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate = candidate.checked_add(1).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                KSatisfiability<K3>,
                QuadraticCongruences,
            >("enumerating CRT primes")
        })?;
    }
    Ok(primes)
}

fn bigint_mod_to_biguint(value: &BigInt, modulus: &BigUint) -> BigUint {
    let modulus_bigint = BigInt::from(modulus.clone());
    let reduced = ((value % &modulus_bigint) + &modulus_bigint) % &modulus_bigint;
    reduced
        .to_biguint()
        .expect("Euclidean residue is nonnegative")
}

fn normalize_clause(clause: &[i64]) -> Option<Vec<i64>> {
    let mut literals = BTreeSet::new();
    for &literal in clause {
        if literals.contains(&-literal) {
            return None;
        }
        literals.insert(literal);
    }
    Some(literals.into_iter().collect())
}

fn build_construction(
    source: &KSatisfiability<K3>,
) -> Result<MandersAdlemanConstruction, crate::rules::ReductionError> {
    let clauses: BTreeSet<_> = source
        .clauses()
        .iter()
        .filter_map(|clause| normalize_clause(&clause.literals))
        .collect();
    let active_vars: Vec<_> = clauses
        .iter()
        .flatten()
        .map(|literal| {
            usize::try_from(literal.unsigned_abs()).expect("native SAT indices fit usize")
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let var_map: BTreeMap<_, _> = active_vars
        .iter()
        .enumerate()
        .map(|(compact, &original)| (original, compact + 1))
        .collect();
    let clauses: Vec<Vec<i64>> = clauses
        .into_iter()
        .map(|clause| {
            clause
                .into_iter()
                .map(|literal| {
                    let original = usize::try_from(literal.unsigned_abs())
                        .expect("native SAT indices fit usize");
                    let variable =
                        i64::try_from(var_map[&original]).expect("compact SAT indices fit i64");
                    if literal > 0 {
                        variable
                    } else {
                        -variable
                    }
                })
                .collect()
        })
        .collect();
    let m = clauses.len();
    let coordinate_count = m
        .checked_mul(2)
        .and_then(|aux| aux.checked_add(active_vars.len()))
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                KSatisfiability<K3>,
                QuadraticCongruences,
            >("counting signed-knapsack coordinates")
        })?;
    let mut coefficients = vec![BigInt::zero(); coordinate_count];
    coefficients[0] = BigInt::one();
    let mut tau = BigInt::one();
    let mut weight = BigUint::one();
    for (j, clause) in clauses.iter().enumerate() {
        weight *= 8u32;
        let half_weight = BigInt::from(&weight / 2u32);
        coefficients[2 * j + 1] = -&half_weight;
        coefficients[2 * j + 2] = -BigInt::from(weight.clone());
        let width =
            i64::try_from(clause.len()).expect("native K3 clauses have at most three literals");
        tau += (width - 5) * &half_weight;
        for &literal in clause {
            let variable =
                usize::try_from(literal.unsigned_abs()).expect("compact SAT indices fit usize");
            if literal > 0 {
                coefficients[2 * m + variable] += &half_weight;
            } else {
                coefficients[2 * m + variable] -= &half_weight;
            }
        }
    }
    // c.alpha - tau = alpha_0 - 1 + sum_j (y_j - true_literals_j + 1) 8^j.
    // tau is odd; coefficient doubling would invalidate the square-root lemma.
    let linear_modulus = weight * 8u32;
    let primes = admissible_primes(coordinate_count)?;
    let prime_powers: Vec<_> = primes
        .iter()
        .map(|&prime| num_traits::Pow::pow(BigUint::from(prime), coordinate_count))
        .collect();
    let k: BigUint = prime_powers.iter().product();
    let mut thetas = Vec::with_capacity(coordinate_count);
    for ((coefficient, prime_power), prime) in coefficients.iter().zip(&prime_powers).zip(&primes) {
        let other = &k / prime_power;
        let step = &other * &linear_modulus;
        let residue = bigint_mod_to_biguint(coefficient, &linear_modulus);
        let inverse = other
            .modinv(&linear_modulus)
            .expect("an odd prime-power product is coprime to a power of two");
        let mut theta = &other * ((residue * inverse) % &linear_modulus);
        if theta.is_zero() {
            theta += &step;
        }
        if (&theta % BigUint::from(*prime)).is_zero() {
            theta += &step;
        }
        thetas.push(theta);
    }
    let h: BigUint = thetas.iter().sum();
    // With N=2m+l+1, 13^N > 4N*8^(m+1) for m>=1, hence 2H<K.
    // For the empty formula N=1, theta_0=1 and K=13 give the same strict bound.
    let square_modulus = &linear_modulus * 2u32;
    let b = &square_modulus * &k;
    let inverse = (&square_modulus + &k)
        .modinv(&b)
        .expect("the two CRT moduli are coprime");
    let tau_squared = (&tau * &tau).to_biguint().expect("a square is nonnegative");
    let a = (inverse * (&k * tau_squared + &square_modulus * &h * &h)) % &b;
    let target = QuadraticCongruences::try_new(a, b, &h + BigUint::one()).map_err(
        crate::rules::ReductionError::construction::<KSatisfiability<K3>, QuadraticCongruences>,
    )?;
    Ok(MandersAdlemanConstruction {
        target,
        source_num_vars: source.num_vars(),
        active_to_source: active_vars
            .into_iter()
            .map(|variable| variable - 1)
            .collect(),
        clauses,
        coefficients,
        tau,
        thetas,
        h,
        prime_powers,
    })
}

#[cfg(any(test, feature = "example-db"))]
fn build_alphas(construction: &MandersAdlemanConstruction, assignment: &[bool]) -> Option<Vec<i8>> {
    if assignment.len() != construction.source_num_vars {
        return None;
    }
    let m = construction.clauses.len();
    let mut alphas = vec![1; construction.thetas.len()];
    for (i, &original) in construction.active_to_source.iter().enumerate() {
        alphas[2 * m + i + 1] = 1 - 2 * i8::from(assignment[original]);
    }
    for (j, clause) in construction.clauses.iter().enumerate() {
        let mut y = -1i8;
        for &literal in clause {
            let variable =
                usize::try_from(literal.unsigned_abs()).expect("compact SAT indices fit usize") - 1;
            let truth = assignment[construction.active_to_source[variable]];
            y += i8::from(truth == (literal > 0));
        }
        let (first, second) = match y {
            0 => (1, 1),
            1 => (-1, 1),
            2 => (1, -1),
            _ => return None,
        };
        alphas[2 * j + 1] = first;
        alphas[2 * j + 2] = second;
    }
    Some(alphas)
}

#[cfg(any(test, feature = "example-db"))]
fn witness_value_from_alphas(alphas: &[i8], thetas: &[BigUint]) -> BigUint {
    alphas
        .iter()
        .zip(thetas)
        .map(|(&alpha, theta)| BigInt::from(alpha) * BigInt::from(theta.clone()))
        .sum::<BigInt>()
        .abs()
        .to_biguint()
        .expect("absolute witness is nonnegative")
}

#[cfg(any(test, feature = "example-db"))]
fn witness_config_for_assignment(
    source: &KSatisfiability<K3>,
    assignment: &[bool],
) -> Option<BigUint> {
    let construction = build_construction(source).expect("example or test source must reduce");
    let alphas = build_alphas(&construction, assignment)?;
    Some(witness_value_from_alphas(&alphas, &construction.thetas))
}

#[reduction(
    transform = upper_bound {
        bit_length_a = "64 * (2 * num_clauses + num_vars + 1)^2 + 3 * num_clauses + 4",
        bit_length_b = "64 * (2 * num_clauses + num_vars + 1)^2 + 3 * num_clauses + 4",
        bit_length_c = "64 * (2 * num_clauses + num_vars + 1)^2 + 3 * num_clauses + 4",
    }
)]
impl ReduceTo<QuadraticCongruences> for KSatisfiability<K3> {
    type Result = Reduction3SATToQuadraticCongruences;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let construction = build_construction(self)?;
        Ok(Reduction3SATToQuadraticCongruences {
            target: construction.target,
            source_num_vars: construction.source_num_vars,
            active_to_source: construction.active_to_source,
            clause_count: construction.clauses.len(),
            h: construction.h,
            prime_powers: construction.prime_powers,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_quadraticcongruences",
        build: || {
            let source = KSatisfiability::<K3>::new(
                3,
                vec![crate::models::formula::CNFClause::new(vec![1, 2, 3])],
            );
            let target_config = witness_config_for_assignment(&source, &[true, false, false])
                .expect("canonical satisfying assignment should lift to a QC witness");
            crate::example_db::specs::rule_example_with_witness::<_, QuadraticCongruences>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false]),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_quadraticcongruences.rs"]
mod tests;
