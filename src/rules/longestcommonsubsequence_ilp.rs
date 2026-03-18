//! Reduction from LongestCommonSubsequence to ILP (Integer Linear Programming).
//!
//! The source problem is the decision version of LCS with a fixed witness
//! length `K`. The ILP builds a binary feasibility model:
//! - `x_(p,a)` selects symbol `a` at witness position `p`
//! - `y_(r,p,j)` selects the matching position `j` in source string `r`
//!
//! The constraints enforce exactly one symbol per witness position, exactly one
//! matched source position per `(r, p)`, character consistency, and strictly
//! increasing matched positions within each source string.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing LongestCommonSubsequence to ILP.
#[derive(Debug, Clone)]
pub struct ReductionLCSToILP {
    target: ILP<bool>,
    alphabet_size: usize,
    bound: usize,
}

impl ReductionLCSToILP {
    fn symbol_var(&self, position: usize, symbol: usize) -> usize {
        position * self.alphabet_size + symbol
    }
}

impl ReductionResult for ReductionLCSToILP {
    type Source = LongestCommonSubsequence;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut witness = Vec::with_capacity(self.bound);
        for position in 0..self.bound {
            let selected = (0..self.alphabet_size)
                .find(|&symbol| target_solution.get(self.symbol_var(position, symbol)) == Some(&1))
                .unwrap_or(0);
            witness.push(selected);
        }
        witness
    }
}

#[reduction(
    overhead = {
        num_vars = "bound * alphabet_size + bound * total_length",
        num_constraints = "bound + bound * num_strings + bound * total_length + num_transitions * sum_triangular_lengths",
    }
)]
impl ReduceTo<ILP<bool>> for LongestCommonSubsequence {
    type Result = ReductionLCSToILP;

    fn reduce_to(&self) -> Self::Result {
        let alphabet_size = self.alphabet_size();
        let bound = self.bound();
        let strings = self.strings();
        let total_length = self.total_length();

        let symbol_var_count = bound * alphabet_size;
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

        // Exactly one symbol per witness position.
        for position in 0..bound {
            let terms = (0..alphabet_size)
                .map(|symbol| (position * alphabet_size + symbol, 1.0))
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // For every string and witness position, choose exactly one matching source position.
        for (string_index, string) in strings.iter().enumerate() {
            for position in 0..bound {
                let terms = (0..string.len())
                    .map(|char_index| (match_var(string_index, position, char_index), 1.0))
                    .collect();
                constraints.push(LinearConstraint::eq(terms, 1.0));
            }
        }

        // A chosen source position can only realize the selected witness symbol.
        for (string_index, string) in strings.iter().enumerate() {
            for position in 0..bound {
                for (char_index, &symbol) in string.iter().enumerate() {
                    constraints.push(LinearConstraint::le(
                        vec![
                            (match_var(string_index, position, char_index), 1.0),
                            (position * alphabet_size + symbol, -1.0),
                        ],
                        0.0,
                    ));
                }
            }
        }

        // Consecutive witness positions must map to strictly increasing source positions.
        for (string_index, string) in strings.iter().enumerate() {
            for position in 0..bound.saturating_sub(1) {
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

        let num_vars = symbol_var_count + bound * total_length;
        let target = ILP::<bool>::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionLCSToILP {
            target,
            alphabet_size,
            bound,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcommonsubsequence_to_ilp",
        build: || {
            let source = LongestCommonSubsequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]], 2);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 2],
                    target_config: vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_ilp.rs"]
mod tests;
