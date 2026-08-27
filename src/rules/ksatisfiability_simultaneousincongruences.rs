//! Reduction from 3-SAT to Simultaneous Incongruences.
//!
//! Uses distinct odd primes to encode variable assignments via residues
//! 1 (true) and 2 (false), then forbids each clause's unique falsifying
//! residue class via the Chinese Remainder Theorem.

use std::collections::BTreeMap;

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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let x = u64::try_from(*target_solution).map_err(|_| {
                crate::rules::ExtractionError::invalid(
                    "target value cannot be represented in the CRT implementation domain",
                )
            })?;
            self.variable_primes
                .iter()
                .map(|&prime| x % prime == 1)
                .collect()
        })
    }
}

fn falsifying_residue(literal: i64) -> u64 {
    if literal > 0 {
        2
    } else {
        1
    }
}

fn modular_inverse(value: u64, modulus: u64) -> Option<u64> {
    let mut t = 0i128;
    let mut new_t = 1i128;
    let mut r = modulus as i128;
    let mut new_r = value as i128;

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    if r != 1 {
        return None;
    }
    if t < 0 {
        t += modulus as i128;
    }
    Some(t as u64)
}

fn crt_residue(congruences: &[(u64, u64)]) -> Result<(u64, u64), &'static str> {
    let modulus = congruences
        .iter()
        .try_fold(1u64, |product, &(m, _)| product.checked_mul(m))
        .ok_or("CRT modulus product overflow")?;

    let residue = congruences
        .iter()
        .try_fold(0u128, |acc, &(modulus_i, residue_i)| {
            let partial = modulus / modulus_i;
            let inverse = modular_inverse(partial % modulus_i, modulus_i)
                .ok_or("CRT moduli must be pairwise coprime")?;
            let term = u128::from(residue_i)
                .checked_mul(u128::from(partial))
                .and_then(|value| value.checked_mul(u128::from(inverse)))
                .ok_or("CRT residue term overflow")?;
            acc.checked_add(term).ok_or("CRT residue sum overflow")
        })?
        % modulus as u128;

    Ok((residue as u64, modulus))
}

fn clause_bad_residue(
    clause: &CNFClause,
    variable_primes: &[u64],
) -> Result<(u64, u64), &'static str> {
    let mut residue_by_var = BTreeMap::new();
    let mut contradictory_var = None;

    for &literal in &clause.literals {
        let var_index =
            usize::try_from(literal.unsigned_abs()).map_err(|_| "literal index exceeds usize")? - 1;
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
            let candidate = usize::try_from(literal.unsigned_abs())
                .map_err(|_| "literal index exceeds usize")?
                - 1;
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
            variable_primes
                .get(var_index)
                .copied()
                .map(|prime| (prime, residue))
                .ok_or("clause variable index exceeds num_vars")
        })
        .collect::<Result<Vec<_>, _>>()?;

    crt_residue(&congruences)
}

fn ensure_prime_product_fits_target(
    variable_primes: &[u64],
) -> Result<(), crate::registry::ConstructionError> {
    let mut product = 1u128;
    for &prime in variable_primes {
        product = product.checked_mul(prime as u128).ok_or_else(|| {
            format!(
                "variable-prime product overflows for {} variables",
                variable_primes.len()
            )
        })?;
        if product > i64::MAX as u128 {
            return Err(format!(
                "variable-prime product {product} for {} variables exceeds the target i64 domain",
                variable_primes.len()
            )
            .into());
        }
    }
    Ok(())
}

#[reduction(
    size = unavailable {
        num_pairs = "the number of residue pairs depends on the first num_vars odd primes and is not expressible in the size-expression language",
    }
)]
impl ReduceTo<SimultaneousIncongruences> for KSatisfiability<K3> {
    type Result = Reduction3SATToSimultaneousIncongruences;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let variable_primes = first_n_odd_primes(self.num_vars());
        ensure_prime_product_fits_target(&variable_primes).map_err(|message| {
            crate::rules::ReductionError::invalid_target::<
                KSatisfiability<K3>,
                SimultaneousIncongruences,
            >(message.to_string())
        })?;

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
            let (bad_residue, clause_modulus) = clause_bad_residue(clause, &variable_primes)
                .map_err(|message| {
                    crate::rules::ReductionError::invalid_target::<
                        KSatisfiability<K3>,
                        SimultaneousIncongruences,
                    >(message)
                })?;
            // The model requires a >= 1. Use modulus instead of 0 since
            // modulus % modulus = 0, achieving the same incongruence.
            let a = if bad_residue == 0 {
                clause_modulus
            } else {
                bad_residue
            };
            pairs.push((a, clause_modulus));
        }

        let pairs = pairs
            .into_iter()
            .map(|(residue, modulus)| {
                Ok((
                    i64::try_from(residue).map_err(|_| "residue exceeds i64")?,
                    i64::try_from(modulus).map_err(|_| "modulus exceeds i64")?,
                ))
            })
            .collect::<Result<Vec<_>, &str>>()
            .map_err(|message| {
                crate::rules::ReductionError::invalid_target::<
                    KSatisfiability<K3>,
                    SimultaneousIncongruences,
                >(message)
            })?;
        let target = SimultaneousIncongruences::new(pairs).map_err(|message| {
            crate::rules::ReductionError::construction::<
                KSatisfiability<K3>,
                SimultaneousIncongruences,
            >(message)
        })?;
        Ok(Reduction3SATToSimultaneousIncongruences {
            target,
            variable_primes,
        })
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
                    source_config: serde_json::json!(vec![true, true]),
                    target_config: serde_json::json!(1),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_simultaneousincongruences.rs"]
mod tests;
