//! Reduction from LongestCommonSubsequence to ILP (Integer Linear Programming).
//!
//! The source problem is the optimization version of LCS. The ILP builds a
//! binary model that maximizes the number of active (non-padding) positions:
//! - `x_(p,a)` selects symbol `a` at witness position `p` (including padding)
//! - `y_(r,p,j)` selects the matching position `j` in source string `r`
//!
//! The constraints enforce exactly one symbol per position (including the
//! padding symbol), contiguity of padding, conditional matching for active
//! positions, and character consistency. The objective maximizes the number of
//! non-padding positions.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing LongestCommonSubsequence to ILP.
#[derive(Debug, Clone)]
pub struct ReductionLCSToILP {
    target: ILP<bool>,
    alphabet_size: usize,
    max_length: usize,
}

impl ReductionResult for ReductionLCSToILP {
    type Source = LongestCommonSubsequence;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let num_symbols = self.alphabet_size + 1;
        let mut witness = Vec::with_capacity(self.max_length);
        for position in 0..self.max_length {
            let selected = (0..num_symbols)
                .find(|&symbol| target_solution.get(position * num_symbols + symbol) == Some(&1))
                .unwrap_or(self.alphabet_size);
            witness.push(selected);
        }
        witness
    }
}

#[reduction(
    overhead = {
        num_vars = "max_length * (alphabet_size + 1) + max_length * total_length",
        num_constraints = "max_length + num_transitions + max_length * num_strings + max_length * total_length + num_transitions * sum_triangular_lengths",
    }
)]
impl ReduceTo<ILP<bool>> for LongestCommonSubsequence {
    type Result = ReductionLCSToILP;

    fn reduce_to(&self) -> Self::Result {
        let alphabet_size = self.alphabet_size();
        let max_length = self.max_length();
        let strings = self.strings();
        let total_length = self.total_length();
        let padding = alphabet_size; // padding symbol index
        let num_symbols = alphabet_size + 1; // includes padding

        let symbol_var_count = max_length * num_symbols;
        let mut string_offsets = Vec::with_capacity(strings.len());
        let mut running_offset = 0usize;
        for string in strings {
            string_offsets.push(running_offset);
            running_offset += string.len();
        }

        let match_var = |string_index: usize, position: usize, char_index: usize| -> usize {
            symbol_var_count + position * total_length + string_offsets[string_index] + char_index
        };

        let mut constraints = Vec::new();

        // (1) Exactly one symbol (including padding) per witness position.
        for position in 0..max_length {
            let terms = (0..num_symbols)
                .map(|symbol| (position * num_symbols + symbol, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // (2) Contiguity: once padding starts, it stays padding.
        // x_(p+1, padding) >= x_(p, padding)
        for position in 0..max_length.saturating_sub(1) {
            constraints.push(LinearConstraint::ge(
                vec![
                    (position * num_symbols + padding, -1.0),
                    ((position + 1) * num_symbols + padding, 1.0),
                ],
                0.0,
            ));
        }

        // (3) For every string and witness position, the sum of match variables
        // equals 1 when active and 0 when padding:
        //   sum_j y_(r,p,j) + x_(p, padding) = 1
        for (string_index, string) in strings.iter().enumerate() {
            for position in 0..max_length {
                let mut terms: Vec<(usize, f64)> = (0..string.len())
                    .map(|char_index| (match_var(string_index, position, char_index), 1.0))
                    .collect();
                terms.push((position * num_symbols + padding, 1.0));
                constraints.push(LinearConstraint::eq(terms, 1.0));
            }
        }

        // (4) A chosen source position can only realize the selected witness symbol.
        // y_(r, p, j) <= x_(p, string[j])
        for (string_index, string) in strings.iter().enumerate() {
            for position in 0..max_length {
                for (char_index, &symbol) in string.iter().enumerate() {
                    constraints.push(LinearConstraint::le(
                        vec![
                            (match_var(string_index, position, char_index), 1.0),
                            (position * num_symbols + symbol, -1.0),
                        ],
                        0.0,
                    ));
                }
            }
        }

        // (5) Consecutive active witness positions must map to strictly increasing
        // source positions.
        for (string_index, string) in strings.iter().enumerate() {
            for position in 0..max_length.saturating_sub(1) {
                for previous in 0..string.len() {
                    for next in 0..=previous {
                        constraints.push(LinearConstraint::le(
                            vec![
                                (match_var(string_index, position, previous), 1.0),
                                (match_var(string_index, position + 1, next), 1.0),
                            ],
                            1.0,
                        ));
                    }
                }
            }
        }

        let num_vars = symbol_var_count + max_length * total_length;

        // Objective: maximize number of non-padding positions.
        // maximize sum_p sum_{a != padding} x_(p,a)
        let objective: Vec<(usize, f64)> = (0..max_length)
            .flat_map(|p| (0..alphabet_size).map(move |a| (p * num_symbols + a, 1.0)))
            .collect();

        let target = ILP::<bool>::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionLCSToILP {
            target,
            alphabet_size,
            max_length,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcommonsubsequence_to_ilp",
        build: || {
            // Source: alphabet {0,1,2}, strings [0,1,2] and [1,0,2], max_length = 3
            // Optimal LCS: [0,2] (length 2) or [1,2] (length 2)
            // Config with padding: e.g. [0, 2, 3] (symbol 3 = padding)
            let source = LongestCommonSubsequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]]);
            // num_symbols = 4, max_length = 3
            // symbol_var_count = 3 * 4 = 12
            // total_length = 6, match vars = 3 * 6 = 18, total vars = 30
            //
            // Using witness [0, 2, padding]:
            // Symbol vars: x_(0,0)=1 → var 0, x_(1,2)=1 → var 6, x_(2,3)=1 → var 11
            // For string 0 = [0,1,2]:
            //   pos 0 active → match j=0 (str[0]=0): match_var(0,0,0) = 12
            //   pos 1 active → match j=2 (str[2]=2): match_var(0,1,2) = 12+6+2 = 20
            //   pos 2 padding → all match vars = 0 (sum + x_pad = 1, x_pad=1)
            // For string 1 = [1,0,2]:
            //   pos 0 active → match j=1 (str[1]=0): match_var(1,0,1) = 12+3+1 = 16
            //   pos 1 active → match j=2 (str[2]=2): match_var(1,1,2) = 12+6+3+2 = 23
            //   pos 2 padding → all match vars = 0
            let mut target_config = vec![0usize; 30];
            // symbol vars
            target_config[0] = 1; // x_(0,0) = 1
            target_config[6] = 1; // x_(1,2) = 1
            target_config[11] = 1; // x_(2,padding=3) = 1
                                   // match vars for string 0
            target_config[12] = 1; // y_(0,0,0) = 1
            target_config[20] = 1; // y_(0,1,2) = 1
                                   // match vars for string 1
            target_config[16] = 1; // y_(1,0,1) = 1
            target_config[23] = 1; // y_(1,1,2) = 1
                                   // pos 2 match vars: all zero (padding)

            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 2, 3],
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_ilp.rs"]
mod tests;
