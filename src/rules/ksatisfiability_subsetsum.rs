//! Reduction from KSatisfiability (3-SAT) to SubsetSum.
//!
//! Classical Karp reduction using base-10 digit encoding. Each integer has
//! (n + m) digits, where n is the number of variables and m is the number
//! of clauses. Variable digits ensure exactly one of y_i/z_i per variable;
//! clause digits count satisfied literals, padded to 4 by slack integers.
//!
//! No carries occur because the maximum digit sum is at most 3 + 2 = 5 < 10.
//!
//! Reference: Karp 1972; Sipser Theorem 7.56; CLRS §34.5.5

use crate::models::formula::KSatisfiability;
use crate::models::misc::SubsetSum;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;

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
        // Variable integers are the first 2n elements: y_1, z_1, y_2, z_2, ...
        // If y_i is selected (index 2*(i-1) = 1), set x_i = 1
        // If z_i is selected (index 2*(i-1)+1 = 1), set x_i = 0
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
fn digits_to_integer(digits: &[i64]) -> i64 {
    let mut value: i64 = 0;
    for &d in digits {
        value = value * 10 + d;
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
        assert!(
            num_digits <= 18,
            "3-SAT to SubsetSum reduction requires n + m <= 18 for i64 encoding \
             (got n={n}, m={m}, n+m={num_digits})"
        );

        let mut sizes = Vec::with_capacity(2 * n + 2 * m);

        // Step 1: Variable integers (2n integers)
        for i in 0..n {
            // y_i: d_i = 1, d_{n+j} = 1 if x_{i+1} appears positive in C_j
            let mut y_digits = vec![0i64; num_digits];
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
            let mut z_digits = vec![0i64; num_digits];
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
            let mut g_digits = vec![0i64; num_digits];
            g_digits[n + j] = 1;
            sizes.push(digits_to_integer(&g_digits));

            // h_j: d_{n+j} = 2
            let mut h_digits = vec![0i64; num_digits];
            h_digits[n + j] = 2;
            sizes.push(digits_to_integer(&h_digits));
        }

        // Step 3: Target
        let mut target_digits = vec![0i64; num_digits];
        for d in target_digits.iter_mut().take(n) {
            *d = 1;
        }
        for d in target_digits.iter_mut().skip(n).take(m) {
            *d = 4;
        }
        let target = digits_to_integer(&target_digits);

        Reduction3SATToSubsetSum {
            target: SubsetSum::new(sizes, target),
            source_num_vars: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_subsetsum.rs"]
mod tests;
