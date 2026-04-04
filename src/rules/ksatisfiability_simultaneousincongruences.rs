//! Reduction from 3-SAT to Simultaneous Incongruences.
//!
//! Uses distinct odd primes to encode variable assignments via residues
//! 1 (true) and 2 (false), then forbids each clause's unique falsifying
//! residue class via the Chinese Remainder Theorem.

use std::collections::BTreeMap;

use crate::models::algebraic::simultaneous_incongruences::MAX_LCM;
use crate::models::algebraic::SimultaneousIncongruences;
use crate::models::formula::{ksat::first_n_odd_primes, CNFClause, KSatisfiability};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;

#[derive(Debug, Clone)]
pub struct Reduction3SATToSimultaneousIncongruences {
    target: SimultaneousIncongruences,
    variable_primes: Vec<u64>,
}

impl ReductionResult for Reduction3SATToSimultaneousIncongruences {
    type Source = KSatisfiability<K3>;
    type Target = SimultaneousIncongruences;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let x = target_solution.first().copied().unwrap_or(0) as u64;
        self.variable_primes
            .iter()
            .map(|&prime| if x % prime == 1 { 1 } else { 0 })
            .collect()
    }
}

fn falsifying_residue(literal: i32) -> u64 {
    if literal > 0 {
        2
    } else {
        1
    }
}

fn modular_inverse(value: u64, modulus: u64) -> u64 {
    let mut t = 0i128;
    let mut new_t = 1i128;
    let mut r = modulus as i128;
    let mut new_r = value as i128;

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    assert_eq!(r, 1, "value and modulus must be coprime");
    if t < 0 {
        t += modulus as i128;
    }
    t as u64
}

fn crt_residue(congruences: &[(u64, u64)]) -> (u64, u64) {
    let modulus = congruences.iter().fold(1u64, |product, &(m, _)| {
        product
            .checked_mul(m)
            .expect("CRT modulus product overflow")
    });

    let residue = congruences
        .iter()
        .fold(0u128, |acc, &(modulus_i, residue_i)| {
            let partial = modulus / modulus_i;
            let inverse = modular_inverse(partial % modulus_i, modulus_i);
            acc + residue_i as u128 * partial as u128 * inverse as u128
        })
        % modulus as u128;

    (residue as u64, modulus)
}

fn clause_bad_residue(clause: &CNFClause, variable_primes: &[u64]) -> (u64, u64) {
    let mut residue_by_var = BTreeMap::new();
    let mut contradictory_var = None;

    for &literal in &clause.literals {
        let var_index = literal.unsigned_abs() as usize - 1;
        let residue = falsifying_residue(literal);

        match residue_by_var.insert(var_index, residue) {
            Some(existing) if existing != residue => {
                contradictory_var = Some(var_index);
                residue_by_var.insert(var_index, 0);
                break;
            }
            Some(existing) => {
                residue_by_var.insert(var_index, existing);
            }
            None => {}
        }
    }

    if let Some(var_index) = contradictory_var {
        for &literal in &clause.literals {
            let candidate = literal.unsigned_abs() as usize - 1;
            if candidate != var_index {
                residue_by_var
                    .entry(candidate)
                    .or_insert_with(|| falsifying_residue(literal));
            }
        }
    }

    let congruences = residue_by_var
        .into_iter()
        .map(|(var_index, residue)| {
            (
                *variable_primes
                    .get(var_index)
                    .expect("clause variable index must be within num_vars"),
                residue,
            )
        })
        .collect::<Vec<_>>();

    crt_residue(&congruences)
}

fn ensure_prime_product_within_lcm_cap(variable_primes: &[u64]) {
    let mut product = 1u128;
    for &prime in variable_primes {
        product = product.checked_mul(prime as u128).unwrap_or_else(|| {
            panic!(
                "3-SAT -> SimultaneousIncongruences requires the variable-prime product to fit within the target model's LCM cap ({MAX_LCM}); num_vars={} overflows while multiplying primes",
                variable_primes.len()
            )
        });
        if product > MAX_LCM {
            panic!(
                "3-SAT -> SimultaneousIncongruences requires the variable-prime product to fit within the target model's LCM cap ({MAX_LCM}); num_vars={} yields prime product {product}",
                variable_primes.len()
            );
        }
    }
}

#[reduction(overhead = {
    num_pairs = "simultaneous_incongruences_num_incongruences",
})]
impl ReduceTo<SimultaneousIncongruences> for KSatisfiability<K3> {
    type Result = Reduction3SATToSimultaneousIncongruences;

    fn reduce_to(&self) -> Self::Result {
        let variable_primes = first_n_odd_primes(self.num_vars());
        ensure_prime_product_within_lcm_cap(&variable_primes);

        let mut pairs = Vec::new();

        for &prime in &variable_primes {
            // Use (prime, prime) to forbid x ≡ 0 (mod prime), since the
            // model requires a ≥ 1. Note: prime % prime = 0, so this is
            // equivalent to forbidding residue 0.
            pairs.push((prime, prime));
            for residue in 3..prime {
                pairs.push((residue, prime));
            }
        }

        for clause in self.clauses() {
            let (bad_residue, clause_modulus) = clause_bad_residue(clause, &variable_primes);
            // The model requires a >= 1. Use modulus instead of 0 since
            // modulus % modulus = 0, achieving the same incongruence.
            let a = if bad_residue == 0 {
                clause_modulus
            } else {
                bad_residue
            };
            pairs.push((a, clause_modulus));
        }

        Reduction3SATToSimultaneousIncongruences {
            target: SimultaneousIncongruences::new(pairs)
                .expect("reduction produces valid incongruences"),
            variable_primes,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_simultaneous_incongruences",
        build: || {
            let source = KSatisfiability::<K3>::new(
                2,
                vec![
                    CNFClause::new(vec![1, 2, 2]),
                    CNFClause::new(vec![-1, 2, 2]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, SimultaneousIncongruences>(
                source,
                SolutionPair {
                    source_config: vec![1, 1],
                    target_config: vec![1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_simultaneousincongruences.rs"]
mod tests;
