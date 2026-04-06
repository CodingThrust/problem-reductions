//! Minimum Disjunctive Normal Form (DNF) problem implementation.
//!
//! Given a Boolean function specified by its truth table, find a DNF formula
//! with the minimum number of terms (prime implicants) equivalent to the function.
//! NP-hard (Masek 1979, via reduction from Minimum Cover).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDisjunctiveNormalForm",
        display_name: "Minimum Disjunctive Normal Form",
        aliases: &["MinDNF"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find minimum-term DNF formula equivalent to a Boolean function",
        fields: &[
            FieldInfo { name: "num_variables", type_name: "usize", description: "Number of Boolean variables" },
            FieldInfo { name: "truth_table", type_name: "Vec<bool>", description: "Truth table of length 2^n" },
        ],
    }
}

/// A prime implicant, represented as a pattern over n variables.
/// Each entry is `Some(true)` (positive literal), `Some(false)` (negative literal),
/// or `None` (don't care).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrimeImplicant {
    /// Pattern: one entry per variable.
    pub pattern: Vec<Option<bool>>,
}

impl PrimeImplicant {
    /// Check if this prime implicant covers a given minterm (as a bit pattern).
    pub fn covers(&self, minterm: usize) -> bool {
        for (i, &p) in self.pattern.iter().enumerate() {
            if let Some(val) = p {
                let bit = ((minterm >> (self.pattern.len() - 1 - i)) & 1) == 1;
                if bit != val {
                    return false;
                }
            }
        }
        true
    }
}

/// Minimum Disjunctive Normal Form problem.
///
/// Given a Boolean function by its truth table, find the minimum number of
/// prime implicants whose disjunction (OR) is equivalent to the function.
///
/// The constructor computes all prime implicants via Quine-McCluskey.
/// The configuration is a binary selection over prime implicants.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumDisjunctiveNormalForm;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // f(x1,x2,x3) = 1 when exactly 1 or 2 variables are true
/// let truth_table = vec![false, true, true, true, true, true, true, false];
/// let problem = MinimumDisjunctiveNormalForm::new(3, truth_table);
/// let solver = BruteForce::new();
/// let value = solver.solve(&problem);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumDisjunctiveNormalForm {
    /// Number of Boolean variables.
    num_variables: usize,
    /// Truth table of length 2^n.
    truth_table: Vec<bool>,
    /// Precomputed prime implicants.
    prime_implicants: Vec<PrimeImplicant>,
    /// Minterms (indices where truth table is true).
    minterms: Vec<usize>,
}

impl MinimumDisjunctiveNormalForm {
    /// Create a new MinimumDisjunctiveNormalForm problem.
    ///
    /// # Panics
    /// - If truth_table length != 2^num_variables
    /// - If the function is identically false (no minterms)
    pub fn new(num_variables: usize, truth_table: Vec<bool>) -> Self {
        assert!(num_variables >= 1, "Need at least 1 variable");
        assert_eq!(
            truth_table.len(),
            1 << num_variables,
            "Truth table must have 2^n entries"
        );

        let minterms: Vec<usize> = truth_table
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .collect();
        assert!(
            !minterms.is_empty(),
            "Function must have at least one minterm"
        );

        let prime_implicants = compute_prime_implicants(num_variables, &minterms);

        Self {
            num_variables,
            truth_table,
            prime_implicants,
            minterms,
        }
    }

    /// Get the number of variables.
    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    /// Get the truth table.
    pub fn truth_table(&self) -> &[bool] {
        &self.truth_table
    }

    /// Get the prime implicants.
    pub fn prime_implicants(&self) -> &[PrimeImplicant] {
        &self.prime_implicants
    }

    /// Get the number of prime implicants.
    pub fn num_prime_implicants(&self) -> usize {
        self.prime_implicants.len()
    }

    /// Get the minterms.
    pub fn minterms(&self) -> &[usize] {
        &self.minterms
    }
}

impl Problem for MinimumDisjunctiveNormalForm {
    const NAME: &'static str = "MinimumDisjunctiveNormalForm";
    type Value = Min<usize>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.prime_implicants.len()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != self.prime_implicants.len() {
            return Min(None);
        }

        // Collect selected prime implicants
        let selected: Vec<usize> = config
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v == 1 { Some(i) } else { None })
            .collect();

        if selected.is_empty() {
            return Min(None);
        }

        // Check that all minterms are covered
        for &mt in &self.minterms {
            let covered = selected
                .iter()
                .any(|&pi_idx| self.prime_implicants[pi_idx].covers(mt));
            if !covered {
                return Min(None);
            }
        }

        Min(Some(selected.len()))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default MinimumDisjunctiveNormalForm => "2^(3^num_variables)",
}

/// Compute all prime implicants of a Boolean function using Quine-McCluskey.
///
/// Each implicant is represented as a Vec<Option<bool>> of length num_variables.
fn compute_prime_implicants(num_vars: usize, minterms: &[usize]) -> Vec<PrimeImplicant> {
    use std::collections::HashSet;

    if minterms.is_empty() {
        return vec![];
    }

    type Pattern = Vec<Option<bool>>;

    let mut current: Vec<Pattern> = minterms
        .iter()
        .map(|&mt| {
            (0..num_vars)
                .map(|i| Some(((mt >> (num_vars - 1 - i)) & 1) == 1))
                .collect()
        })
        .collect();

    let mut all_prime: HashSet<Pattern> = HashSet::new();

    loop {
        let mut next_set: HashSet<Pattern> = HashSet::new();
        let mut used = vec![false; current.len()];

        for i in 0..current.len() {
            for j in (i + 1)..current.len() {
                if let Some(merged) = try_merge(&current[i], &current[j]) {
                    next_set.insert(merged);
                    used[i] = true;
                    used[j] = true;
                }
            }
        }

        for (i, &was_used) in used.iter().enumerate() {
            if !was_used {
                all_prime.insert(current[i].clone());
            }
        }

        if next_set.is_empty() {
            break;
        }
        current = next_set.into_iter().collect();
    }

    let mut result: Vec<PrimeImplicant> = all_prime
        .into_iter()
        .map(|pattern| PrimeImplicant { pattern })
        .collect();
    // Sort for deterministic output (HashSet iteration order is non-deterministic)
    result.sort_by(|a, b| a.pattern.cmp(&b.pattern));
    result
}

/// Try to merge two implicant patterns that differ in exactly one position.
/// Returns the merged pattern (with that position set to None) or None if they can't merge.
fn try_merge(a: &[Option<bool>], b: &[Option<bool>]) -> Option<Vec<Option<bool>>> {
    if a.len() != b.len() {
        return None;
    }

    let mut diff_count = 0;
    let mut diff_pos = 0;

    for (i, (va, vb)) in a.iter().zip(b.iter()).enumerate() {
        if va != vb {
            diff_count += 1;
            diff_pos = i;
            if diff_count > 1 {
                return None;
            }
        }
    }

    if diff_count == 1 {
        let mut merged = a.to_vec();
        merged[diff_pos] = None;
        Some(merged)
    } else {
        None
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_disjunctive_normal_form",
        instance: Box::new(MinimumDisjunctiveNormalForm::new(
            3,
            vec![false, true, true, true, true, true, true, false],
        )),
        // Select prime implicants: p1(¬x1∧x2), p4(x1∧¬x3), p5(¬x2∧x3)
        // The order of PIs depends on the QMC algorithm output.
        // We'll verify this in tests.
        optimal_config: vec![1, 0, 0, 1, 1, 0],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_disjunctive_normal_form.rs"]
mod tests;
