//! Reduction from LongestCommonSubsequence to ILP (Integer Linear Programming).
//!
//! Uses the match-pair formulation (Blum et al., 2021; Althaus et al., 2006).
//! For 2 strings s1 (length n1) and s2 (length n2):
//!
//! ## Variables
//! For each pair (j1, j2) where s1[j1] == s2[j2], a binary variable m_{j1,j2}.
//!
//! ## Constraints
//! 1. Each position in s1 matched at most once
//! 2. Each position in s2 matched at most once
//! 3. Order preservation (no crossings): for (j1,j2),(j1',j2') with j1 < j1' and j2 > j2':
//!    m_{j1,j2} + m_{j1',j2'} <= 1
//!
//! ## Objective
//! Maximize sum of all match variables.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing LongestCommonSubsequence to ILP.
#[derive(Debug, Clone)]
pub struct ReductionLCSToILP {
    target: ILP<bool>,
    /// The match pairs: (j1, j2) for each variable index.
    match_pairs: Vec<(usize, usize)>,
    /// Number of characters in the first string.
    n1: usize,
    /// Number of characters in the second string.
    n2: usize,
}

impl ReductionResult for ReductionLCSToILP {
    type Source = LongestCommonSubsequence;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to LCS.
    ///
    /// The ILP solution has binary variables for match pairs. We extract the
    /// matched positions, build the LCS, and map back to a binary selection
    /// on the shortest source string.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // The source problem's dims() is based on the shortest string.
        // We build match pairs on s1=strings[0], s2=strings[1].
        // If shortest is s1, we use j1 positions; if s2, we use j2 positions.
        let shortest_len = std::cmp::min(self.n1, self.n2);
        let shortest_is_first = self.n1 <= self.n2;

        let matched_positions: Vec<usize> = self
            .match_pairs
            .iter()
            .enumerate()
            .filter(|(i, _)| target_solution.get(*i).copied().unwrap_or(0) == 1)
            .map(|(_, &(j1, j2))| if shortest_is_first { j1 } else { j2 })
            .collect();

        let mut config = vec![0usize; shortest_len];
        for pos in matched_positions {
            if pos < config.len() {
                config[pos] = 1;
            }
        }
        config
    }
}

#[reduction(
    overhead = {
    num_vars = "num_chars_first * num_chars_second",
    num_constraints = "num_chars_first + num_chars_second + (num_chars_first * num_chars_second) ^ 2",
    }
)]
impl ReduceTo<ILP<bool>> for LongestCommonSubsequence {
    type Result = ReductionLCSToILP;

    fn reduce_to(&self) -> Self::Result {
        let strings = self.strings();
        assert!(
            strings.len() == 2,
            "LCS to ILP reduction is defined for exactly 2 strings, got {}",
            strings.len()
        );

        let s1 = &strings[0];
        let s2 = &strings[1];
        let n1 = s1.len();
        let n2 = s2.len();

        // Build match pairs: (j1, j2) where s1[j1] == s2[j2]
        let mut match_pairs: Vec<(usize, usize)> = Vec::new();
        for (j1, &c1) in s1.iter().enumerate() {
            for (j2, &c2) in s2.iter().enumerate() {
                if c1 == c2 {
                    match_pairs.push((j1, j2));
                }
            }
        }

        let num_vars = match_pairs.len();
        let mut constraints = Vec::new();

        // Constraint 1: Each position in s1 matched at most once
        for j1 in 0..n1 {
            let terms: Vec<(usize, f64)> = match_pairs
                .iter()
                .enumerate()
                .filter(|(_, &(a, _))| a == j1)
                .map(|(idx, _)| (idx, 1.0))
                .collect();
            if !terms.is_empty() {
                constraints.push(LinearConstraint::le(terms, 1.0));
            }
        }

        // Constraint 2: Each position in s2 matched at most once
        for j2 in 0..n2 {
            let terms: Vec<(usize, f64)> = match_pairs
                .iter()
                .enumerate()
                .filter(|(_, &(_, b))| b == j2)
                .map(|(idx, _)| (idx, 1.0))
                .collect();
            if !terms.is_empty() {
                constraints.push(LinearConstraint::le(terms, 1.0));
            }
        }

        // Constraint 3: Order preservation (no crossings)
        // For all pairs (j1, j2) and (j1', j2') with j1 < j1' and j2 > j2':
        //   m_{j1,j2} + m_{j1',j2'} <= 1
        for (i, &(j1, j2)) in match_pairs.iter().enumerate() {
            for (k, &(j1p, j2p)) in match_pairs.iter().enumerate() {
                if i < k && j1 < j1p && j2 > j2p {
                    constraints.push(LinearConstraint::le(vec![(i, 1.0), (k, 1.0)], 1.0));
                }
                // Also check the reverse: j1 > j1p and j2 < j2p
                if i < k && j1 > j1p && j2 < j2p {
                    constraints.push(LinearConstraint::le(vec![(i, 1.0), (k, 1.0)], 1.0));
                }
            }
        }

        // Objective: maximize sum of all match variables
        let objective: Vec<(usize, f64)> = (0..num_vars).map(|i| (i, 1.0)).collect();

        let ilp = ILP::<bool>::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionLCSToILP {
            target: ilp,
            match_pairs,
            n1,
            n2,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcommonsubsequence_to_ilp",
        build: || {
            let source = LongestCommonSubsequence::new(vec![
                vec![b'A', b'B', b'A', b'C'],
                vec![b'B', b'A', b'C', b'A'],
            ]);
            crate::example_db::specs::direct_ilp_example::<_, bool, _>(source, |_, _| true)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_ilp.rs"]
mod tests;
