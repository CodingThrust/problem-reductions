//! Reduction from NAE-Satisfiability to Set Splitting.
//!
//! Given an NAE-SAT instance with n variables and m clauses, we construct a
//! Set Splitting instance on universe {0, ..., 2n-1} where elements 2i and
//! 2i+1 represent variable x_{i+1} and its negation respectively.
//!
//! Subsets consist of:
//! - Complementarity subsets {2i, 2i+1} for each variable i, ensuring that
//!   a variable and its negation receive opposite colors.
//! - Clause subsets mapping each literal to its element, ensuring the NAE
//!   condition (each clause has both true and false literals).

use crate::models::formula::NAESatisfiability;
use crate::models::set::SetSplitting;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing NAE-Satisfiability to Set Splitting.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToSS {
    /// Number of original variables in the source problem.
    source_num_vars: usize,
    /// The target Set Splitting problem.
    target: SetSplitting,
}

/// Map a 1-indexed signed literal to a 0-indexed universe element.
///
/// Positive literal l maps to 2*(l-1), negative literal -l maps to 2*(l-1)+1.
fn literal_to_element(lit: i32) -> usize {
    let var_idx = lit.unsigned_abs() as usize - 1;
    if lit > 0 {
        2 * var_idx
    } else {
        2 * var_idx + 1
    }
}

impl ReductionResult for ReductionNAESATToSS {
    type Source = NAESatisfiability;
    type Target = SetSplitting;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.source_num_vars;
        // Variable i is true iff element 2i is in part 0 (config[2i] == 0).
        (0..n).map(|i| 1 - target_solution[2 * i]).collect()
    }
}

#[reduction(overhead = {
    universe_size = "2 * num_vars",
    num_subsets = "num_clauses + num_vars",
})]
impl ReduceTo<SetSplitting> for NAESatisfiability {
    type Result = ReductionNAESATToSS;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let universe_size = 2 * n;

        let mut subsets = Vec::with_capacity(n + self.num_clauses());

        // Complementarity subsets: {2i, 2i+1} for i = 0..n-1
        for i in 0..n {
            subsets.push(vec![2 * i, 2 * i + 1]);
        }

        // Clause subsets: map each literal to its element
        for clause in self.clauses() {
            let subset: Vec<usize> = clause.literals.iter().map(|&lit| literal_to_element(lit)).collect();
            subsets.push(subset);
        }

        let target = SetSplitting::new(universe_size, subsets);

        ReductionNAESATToSS {
            source_num_vars: n,
            target,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_setsplitting.rs"]
mod tests;
