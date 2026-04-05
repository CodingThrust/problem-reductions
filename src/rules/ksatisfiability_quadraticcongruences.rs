//! Reduction from KSatisfiability (3-SAT) to Quadratic Congruences.
//!
//! This follows the Manders-Adleman construction in its doubled-coefficient
//! form, matching the verified reference vectors for issue #553.

use std::collections::{BTreeMap, BTreeSet};

use crate::models::algebraic::QuadraticCongruences;
use crate::models::formula::KSatisfiability;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};

#[derive(Debug, Clone)]
pub struct Reduction3SATToQuadraticCongruences {
    target: QuadraticCongruences,
    source_num_vars: usize,
    active_to_source: Vec<usize>,
    standard_clause_count: usize,
    h: BigUint,
    prime_powers: Vec<BigUint>,
}

impl ReductionResult for Reduction3SATToQuadraticCongruences {
    type Source = KSatisfiability<K3>;
    type Target = QuadraticCongruences;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut source_assignment = vec![0; self.source_num_vars];
        let Some(x) = self.target.decode_witness(target_solution) else {
            return source_assignment;
        };
        if x > self.h {
            return source_assignment;
        }

        let h_minus_x = &self.h - &x;
        let h_plus_x = &self.h + &x;
        let mut alpha = vec![0i8; self.prime_powers.len()];

        for (j, prime_power) in self.prime_powers.iter().enumerate() {
            if (&h_minus_x % prime_power).is_zero() {
                alpha[j] = 1;
            } else if (&h_plus_x % prime_power).is_zero() {
                alpha[j] = -1;
            }
        }

        for (active_index, &source_index) in self.active_to_source.iter().enumerate() {
            let alpha_index = 2 * self.standard_clause_count + active_index + 1;
            source_assignment[source_index] = if alpha.get(alpha_index) == Some(&-1) {
                1
            } else {
                0
            };
        }

        source_assignment
    }
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct MandersAdlemanConstruction {
    target: QuadraticCongruences,
    source_num_vars: usize,
    active_to_source: Vec<usize>,
    remapped_clause_set: BTreeSet<Vec<i32>>,
    standard_clauses: Vec<Vec<i32>>,
    standard_clause_count: usize,
    active_var_count: usize,
    #[cfg_attr(not(test), allow(dead_code))]
    doubled_coefficients: Vec<BigInt>,
    #[cfg_attr(not(test), allow(dead_code))]
    tau_2: BigInt,
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
    if candidate % 2 == 0 {
        return false;
    }

    let mut divisor = 3u64;
    while divisor * divisor <= candidate {
        if candidate % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

fn admissible_primes(count: usize) -> Vec<u64> {
    let mut primes = Vec::with_capacity(count);
    let mut candidate = 13u64;
    while primes.len() < count {
        if is_prime(candidate) {
            primes.push(candidate);
        }
        candidate += 1;
    }
    primes
}

fn pow_biguint_u64(base: u64, exp: usize) -> BigUint {
    let mut result = BigUint::one();
    let factor = BigUint::from(base);
    for _ in 0..exp {
        result *= &factor;
    }
    result
}

fn bigint_mod_to_biguint(value: &BigInt, modulus: &BigUint) -> BigUint {
    let modulus_bigint = BigInt::from(modulus.clone());
    let reduced = ((value % &modulus_bigint) + &modulus_bigint) % &modulus_bigint;
    reduced
        .to_biguint()
        .expect("nonnegative reduced residue must fit BigUint")
}

fn modular_inverse(value: &BigUint, modulus: &BigUint) -> BigUint {
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = BigInt::from(modulus.clone());
    let mut new_r = BigInt::from(value.clone() % modulus);

    while !new_r.is_zero() {
        let quotient = &r / &new_r;
        let next_t = &t - &quotient * &new_t;
        let next_r = &r - &quotient * &new_r;
        t = new_t;
        new_t = next_t;
        r = new_r;
        new_r = next_r;
    }

    assert_eq!(r, BigInt::one(), "value and modulus must be coprime");
    if t.is_negative() {
        t += BigInt::from(modulus.clone());
    }
    t.to_biguint().expect("inverse must be nonnegative")
}

fn normalize_clause(clause: &[i32]) -> Option<Vec<i32>> {
    let mut lits = BTreeSet::new();
    for &lit in clause {
        if lits.contains(&-lit) {
            return None;
        }
        lits.insert(lit);
    }
    Some(lits.into_iter().collect())
}

fn preprocess_formula(source: &KSatisfiability<K3>) -> (Vec<Vec<i32>>, Vec<usize>) {
    let mut seen = BTreeSet::new();
    let mut normalized_clauses = Vec::new();
    let mut active_vars = BTreeSet::new();

    for clause in source.clauses() {
        let Some(normalized) = normalize_clause(&clause.literals) else {
            continue;
        };
        if normalized.len() != 3 {
            panic!(
                "3-SAT -> QuadraticCongruences requires each non-tautological clause to use three distinct literals; got {:?}",
                clause.literals
            );
        }

        let distinct_vars: BTreeSet<_> = normalized
            .iter()
            .map(|lit| lit.unsigned_abs() as usize)
            .collect();
        if distinct_vars.len() != 3 {
            panic!(
                "3-SAT -> QuadraticCongruences requires each non-tautological clause to use three distinct variables; got {:?}",
                clause.literals
            );
        }

        if seen.insert(normalized.clone()) {
            for &lit in &normalized {
                active_vars.insert(lit.unsigned_abs() as usize);
            }
            normalized_clauses.push(normalized);
        }
    }

    (normalized_clauses, active_vars.into_iter().collect())
}

fn build_standard_clauses(num_active_vars: usize) -> (Vec<Vec<i32>>, BTreeMap<Vec<i32>, usize>) {
    let mut clauses = Vec::new();
    let mut index = BTreeMap::new();

    if num_active_vars < 3 {
        return (clauses, index);
    }

    for i in 1..=num_active_vars - 2 {
        for j in i + 1..=num_active_vars - 1 {
            for k in j + 1..=num_active_vars {
                for s1 in [1i32, -1] {
                    for s2 in [1i32, -1] {
                        for s3 in [1i32, -1] {
                            let mut clause = vec![s1 * i as i32, s2 * j as i32, s3 * k as i32];
                            clause.sort_unstable();
                            index.insert(clause.clone(), clauses.len() + 1);
                            clauses.push(clause);
                        }
                    }
                }
            }
        }
    }

    (clauses, index)
}

fn pow8_table(max_power: usize) -> Vec<BigUint> {
    let mut table = Vec::with_capacity(max_power + 1);
    table.push(BigUint::one());
    for _ in 0..max_power {
        let next = table.last().expect("pow8 table is nonempty") * BigUint::from(8u32);
        table.push(next);
    }
    table
}

fn build_construction(source: &KSatisfiability<K3>) -> MandersAdlemanConstruction {
    let (normalized_clauses, active_vars) = preprocess_formula(source);
    let active_var_count = active_vars.len();
    let var_map: BTreeMap<usize, usize> = active_vars
        .iter()
        .enumerate()
        .map(|(new_index, &old_index)| (old_index, new_index + 1))
        .collect();

    let remapped_clauses = normalized_clauses
        .iter()
        .map(|clause| {
            let mut remapped = clause
                .iter()
                .map(|&lit| {
                    let var = lit.unsigned_abs() as usize;
                    let new_var = *var_map
                        .get(&var)
                        .expect("active variable must be present in the remapping");
                    if lit > 0 {
                        new_var as i32
                    } else {
                        -(new_var as i32)
                    }
                })
                .collect::<Vec<_>>();
            remapped.sort_unstable();
            remapped
        })
        .collect::<Vec<_>>();
    let remapped_clause_set = remapped_clauses.iter().cloned().collect::<BTreeSet<_>>();

    let (standard_clauses, standard_index) = build_standard_clauses(active_var_count);
    let standard_clause_count = standard_clauses.len();
    let aux_dimension = 2 * standard_clause_count + active_var_count;
    let pow8 = pow8_table(standard_clause_count + 1);

    let mut tau_phi = BigInt::zero();
    for clause in &remapped_clauses {
        if let Some(&j) = standard_index.get(clause) {
            tau_phi -= BigInt::from(pow8[j].clone());
        }
    }

    let mut positive_occurrences = vec![BigInt::zero(); active_var_count + 1];
    let mut negative_occurrences = vec![BigInt::zero(); active_var_count + 1];
    for (j, clause) in standard_clauses.iter().enumerate() {
        let weight = BigInt::from(pow8[j + 1].clone());
        for &lit in clause {
            let var = lit.unsigned_abs() as usize;
            if lit > 0 {
                positive_occurrences[var] += &weight;
            } else {
                negative_occurrences[var] += &weight;
            }
        }
    }

    let mut doubled_coefficients = vec![BigInt::zero(); aux_dimension + 1];
    doubled_coefficients[0] = BigInt::from(2u32);
    for k in 1..=standard_clause_count {
        let weight = BigInt::from(pow8[k].clone());
        doubled_coefficients[2 * k - 1] = -weight.clone();
        doubled_coefficients[2 * k] = -(weight * BigInt::from(2u32));
    }
    for i in 1..=active_var_count {
        doubled_coefficients[2 * standard_clause_count + i] =
            &positive_occurrences[i] - &negative_occurrences[i];
    }

    let sum_coefficients = doubled_coefficients
        .iter()
        .cloned()
        .fold(BigInt::zero(), |acc, value| acc + value);
    let sum_negative_occurrences = negative_occurrences
        .iter()
        .skip(1)
        .cloned()
        .fold(BigInt::zero(), |acc, value| acc + value);
    let tau_2 = BigInt::from(2u32) * tau_phi
        + sum_coefficients
        + BigInt::from(2u32) * sum_negative_occurrences;
    let mod_val = BigUint::from(2u32) * pow8[standard_clause_count + 1].clone();

    let primes = admissible_primes(aux_dimension + 1);
    let prime_powers = primes
        .iter()
        .map(|&prime| pow_biguint_u64(prime, aux_dimension + 1))
        .collect::<Vec<_>>();
    let k = prime_powers
        .iter()
        .cloned()
        .fold(BigUint::one(), |acc, value| acc * value);

    let mut thetas = Vec::with_capacity(aux_dimension + 1);
    for j in 0..=aux_dimension {
        let other = &k / &prime_powers[j];
        let lcm = &other * &mod_val;
        let residue = bigint_mod_to_biguint(&doubled_coefficients[j], &mod_val);
        let inverse = modular_inverse(&(other.clone() % &mod_val), &mod_val);
        let mut theta = (&other * ((&residue * inverse) % &mod_val)) % &lcm;
        if theta.is_zero() {
            theta = lcm.clone();
        }
        let prime = BigUint::from(primes[j]);
        while (&theta % &prime).is_zero() {
            theta += &lcm;
        }
        thetas.push(theta);
    }

    let h = thetas
        .iter()
        .cloned()
        .fold(BigUint::zero(), |acc, value| acc + value);
    let beta = &mod_val * &k;
    let inverse_factor = &mod_val + &k;
    let inverse = modular_inverse(&(inverse_factor % &beta), &beta);
    let tau_2_squared = (&tau_2 * &tau_2)
        .to_biguint()
        .expect("squared doubled target must be nonnegative");
    let alpha = (&inverse * ((&k * tau_2_squared) + (&mod_val * (&h * &h)))) % &beta;
    let target = QuadraticCongruences::new(alpha, beta, &h + BigUint::one());

    MandersAdlemanConstruction {
        target,
        source_num_vars: source.num_vars(),
        active_to_source: active_vars.into_iter().map(|var| var - 1).collect(),
        remapped_clause_set,
        standard_clauses,
        standard_clause_count,
        active_var_count,
        doubled_coefficients,
        tau_2,
        thetas,
        h,
        prime_powers,
    }
}

#[cfg(any(test, feature = "example-db"))]
fn build_alphas(
    construction: &MandersAdlemanConstruction,
    assignment: &[usize],
) -> Option<Vec<i8>> {
    if assignment.len() != construction.source_num_vars {
        return None;
    }

    let mut active_assignment = vec![0i8; construction.active_var_count + 1];
    for (active_index, &source_index) in construction.active_to_source.iter().enumerate() {
        active_assignment[active_index + 1] = if assignment[source_index] == 0 { 0 } else { 1 };
    }

    let mut alphas =
        vec![0i8; 2 * construction.standard_clause_count + construction.active_var_count + 1];
    alphas[0] = 1;

    for i in 1..=construction.active_var_count {
        alphas[2 * construction.standard_clause_count + i] = 1 - 2 * active_assignment[i];
    }

    for k in 1..=construction.standard_clause_count {
        let clause = &construction.standard_clauses[k - 1];
        let mut y = 0i32;
        for &lit in clause {
            let var = lit.unsigned_abs() as usize;
            if lit > 0 {
                y += i32::from(active_assignment[var]);
            } else {
                y += 1 - i32::from(active_assignment[var]);
            }
        }
        if construction.remapped_clause_set.contains(clause) {
            y -= 1;
        }

        match 3 - 2 * y {
            3 => {
                alphas[2 * k - 1] = 1;
                alphas[2 * k] = 1;
            }
            1 => {
                alphas[2 * k - 1] = -1;
                alphas[2 * k] = 1;
            }
            -1 => {
                alphas[2 * k - 1] = 1;
                alphas[2 * k] = -1;
            }
            -3 => {
                alphas[2 * k - 1] = -1;
                alphas[2 * k] = -1;
            }
            _ => return None,
        }
    }

    Some(alphas)
}

#[cfg(any(test, feature = "example-db"))]
fn witness_value_from_alphas(alphas: &[i8], thetas: &[BigUint]) -> BigUint {
    let signed_sum = alphas
        .iter()
        .zip(thetas)
        .fold(BigInt::zero(), |acc, (&alpha, theta)| {
            if alpha == 1 {
                acc + BigInt::from(theta.clone())
            } else {
                acc - BigInt::from(theta.clone())
            }
        });
    signed_sum
        .abs()
        .to_biguint()
        .expect("absolute witness must be nonnegative")
}

#[cfg(any(test, feature = "example-db"))]
fn witness_config_for_assignment(
    source: &KSatisfiability<K3>,
    assignment: &[usize],
) -> Option<Vec<usize>> {
    let construction = build_construction(source);
    let alphas = build_alphas(&construction, assignment)?;
    let witness = witness_value_from_alphas(&alphas, &construction.thetas);
    construction.target.encode_witness(&witness)
}

#[cfg(test)]
fn exhaustive_alpha_solution(source: &KSatisfiability<K3>) -> Option<Vec<i8>> {
    let construction = build_construction(source);
    let n = construction.doubled_coefficients.len();
    if n >= usize::BITS as usize {
        return None;
    }

    for bits in 0usize..(1usize << n) {
        let alphas = (0..n)
            .map(|j| if (bits >> j) & 1 == 1 { 1i8 } else { -1i8 })
            .collect::<Vec<_>>();
        let sum = construction
            .doubled_coefficients
            .iter()
            .zip(&alphas)
            .fold(BigInt::zero(), |acc, (coefficient, &alpha)| {
                acc + coefficient * BigInt::from(alpha)
            });
        if sum == construction.tau_2 {
            return Some(alphas);
        }
    }

    None
}

#[reduction(overhead = {
    bit_length_a = "(num_vars + num_clauses)^2 * log(num_vars + num_clauses + 1)",
    bit_length_b = "(num_vars + num_clauses)^2 * log(num_vars + num_clauses + 1)",
    bit_length_c = "(num_vars + num_clauses)^2 * log(num_vars + num_clauses + 1)",
})]
impl ReduceTo<QuadraticCongruences> for KSatisfiability<K3> {
    type Result = Reduction3SATToQuadraticCongruences;

    fn reduce_to(&self) -> Self::Result {
        let construction = build_construction(self);
        Reduction3SATToQuadraticCongruences {
            target: construction.target,
            source_num_vars: construction.source_num_vars,
            active_to_source: construction.active_to_source,
            standard_clause_count: construction.standard_clause_count,
            h: construction.h,
            prime_powers: construction.prime_powers,
        }
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
            let target_config = witness_config_for_assignment(&source, &[1, 0, 0])
                .expect("canonical satisfying assignment should lift to a QC witness");
            crate::example_db::specs::rule_example_with_witness::<_, QuadraticCongruences>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 0],
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_quadraticcongruences.rs"]
mod tests;
