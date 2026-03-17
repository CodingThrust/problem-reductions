//! Reduction from KSatisfiability (3-SAT) to SubsetSum.
//!
//! Classical Karp reduction using base-10 digit encoding. Each integer has
//! (n + m) digits, where n is the number of variables and m is the number
//! of clauses. Variable digits ensure exactly one of y_i/z_i per variable;
//! clause digits count satisfied literals, padded to 4 by slack integers.
//!
//! No carries occur because the maximum digit sum is at most 3 + 2 = 5 < 10.
//!
//! Uses `SubsetSum` with arbitrary-precision integers so the encoding does not
//! overflow on large instances.
//!
//! Reference: Karp 1972; Sipser Theorem 7.56; CLRS §34.5.5

use crate::models::formula::KSatisfiability;
use crate::models::misc::SubsetSum;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;
use num_bigint::BigUint;
use num_traits::Zero;

/// Result of reducing KSatisfiability<K3> to SubsetSum.
#[derive(Debug, Clone)]
pub struct Reduction3SATToSubsetSum {
    target: SubsetSum,
    source_num_vars: usize,
}

impl ReductionResult for Reduction3SATToSubsetSum {
    type Source = KSatisfiability<K3>;
    type Target = SubsetSum;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Variable integers are the first 2n elements in 0-based indexing:
        // for variable i (0 <= i < n), y_i is stored at index 2*i and z_i at index 2*i + 1.
        // If y_i is selected (target_solution[2*i] == 1), set x_i = 1; otherwise x_i = 0.
        (0..self.source_num_vars)
            .map(|i| {
                let y_selected = target_solution[2 * i] == 1;
                if y_selected {
                    1
                } else {
                    0
                }
            })
            .collect()
    }
}

/// Build a base-10 integer from a digit array (most significant first).
///
/// digits[0] is the most significant digit, digits[num_digits-1] is the least.
fn digits_to_integer(digits: &[u8]) -> BigUint {
    let mut value = BigUint::zero();
    let ten = BigUint::from(10u8);
    for &d in digits {
        value = value * &ten + BigUint::from(d);
    }
    value
}

#[reduction(
    overhead = { num_elements = "2 * num_vars + 2 * num_clauses" }
)]
impl ReduceTo<SubsetSum> for KSatisfiability<K3> {
    type Result = Reduction3SATToSubsetSum;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let m = self.num_clauses();
        let num_digits = n + m;

        let mut sizes = Vec::with_capacity(2 * n + 2 * m);

        // Step 1: Variable integers (2n integers)
        for i in 0..n {
            // y_i: d_i = 1, d_{n+j} = 1 if x_{i+1} appears positive in C_j
            let mut y_digits = vec![0u8; num_digits];
            y_digits[i] = 1;
            for (j, clause) in self.clauses().iter().enumerate() {
                for &lit in &clause.literals {
                    let var_idx = (lit.unsigned_abs() as usize) - 1;
                    if var_idx == i && lit > 0 {
                        y_digits[n + j] = 1;
                    }
                }
            }
            sizes.push(digits_to_integer(&y_digits));

            // z_i: d_i = 1, d_{n+j} = 1 if ¬x_{i+1} appears in C_j
            let mut z_digits = vec![0u8; num_digits];
            z_digits[i] = 1;
            for (j, clause) in self.clauses().iter().enumerate() {
                for &lit in &clause.literals {
                    let var_idx = (lit.unsigned_abs() as usize) - 1;
                    if var_idx == i && lit < 0 {
                        z_digits[n + j] = 1;
                    }
                }
            }
            sizes.push(digits_to_integer(&z_digits));
        }

        // Step 2: Slack integers (2m integers)
        for j in 0..m {
            // g_j: d_{n+j} = 1
            let mut g_digits = vec![0u8; num_digits];
            g_digits[n + j] = 1;
            sizes.push(digits_to_integer(&g_digits));

            // h_j: d_{n+j} = 2
            let mut h_digits = vec![0u8; num_digits];
            h_digits[n + j] = 2;
            sizes.push(digits_to_integer(&h_digits));
        }

        // Step 3: Target
        let mut target_digits = vec![0u8; num_digits];
        for d in target_digits.iter_mut().take(n) {
            *d = 1;
        }
        for d in target_digits.iter_mut().skip(n).take(m) {
            *d = 4;
        }
        let target = digits_to_integer(&target_digits);

        Reduction3SATToSubsetSum {
            target: SubsetSum::new_unchecked(sizes, target),
            source_num_vars: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::models::formula::CNFClause;
    use crate::models::misc::SubsetSum;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_subsetsum",
        build: || {
            let source = KSatisfiability::<K3>::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, -2, 3]),
                ],
            );
            crate::example_db::specs::direct_satisfying_example::<_, SubsetSum, _>(
                source,
                |_, _| true,
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_subsetsum.rs"]
mod tests;
