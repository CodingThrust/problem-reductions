//! Polynomial-size LCS reduction using symbol and embedding choice groups.
//!
//! Each choice is an independent cluster: two vertices for a position or padding,
//! three for an active symbol. Conflicts enforce one choice per group, matching
//! characters, increasing positions, and a padding suffix. Every optimum fills
//! all groups; its size is 2 * max_length * (num_strings + 1) plus the LCS length.

use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::One;
use std::ops::Range;

#[derive(Debug, Clone)]
struct Choice {
    group: usize,
    slot: usize,
    string: Option<usize>,
    value: Option<usize>,
    vertices: Range<usize>,
}

/// Stores the choice clusters needed to decode a complete independent set.
#[derive(Debug, Clone)]
pub struct ReductionLCSToIS {
    target: MaximumIndependentSet<SimpleGraph, One>,
    choices: Vec<Choice>,
    num_groups: usize,
    max_length: usize,
}

impl ReductionResult for ReductionLCSToIS {
    type Source = LongestCommonSubsequence;
    type Target = MaximumIndependentSet<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.0.is_some(),
            "selected vertices do not form an independent set",
        )?;
        let mut selected_groups = vec![false; self.num_groups];
        let mut decoded = vec![None; self.max_length];
        for choice in &self.choices {
            let selected = solution[choice.vertices.clone()]
                .iter()
                .filter(|&&bit| bit)
                .count();
            if selected == 0 {
                continue;
            }
            if selected != choice.vertices.len() {
                return Err(crate::rules::ExtractionError::invalid(
                    "selected choice cluster is incomplete",
                ));
            }
            selected_groups[choice.group] = true;
            if choice.group < self.max_length {
                decoded[choice.group] = choice.value;
            }
        }
        if let Some(group) = selected_groups.iter().position(|&selected| !selected) {
            return Err(crate::rules::ExtractionError::invalid(format!(
                "choice group {group} has no selected choice"
            )));
        }
        Ok(decoded)
    }
}

#[reduction(
    transform = {
        exact {
            num_vertices = "max_length * (3 * num_distinct_symbols + 2 + 2 * (total_length + num_strings))",
        },
        upper_bound {
            num_edges = "(max_length * (3 * num_distinct_symbols + 2 + 2 * (total_length + num_strings)))^2",
        },
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, One>> for LongestCommonSubsequence {
    type Result = ReductionLCSToIS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<
                Self,
                MaximumIndependentSet<SimpleGraph, One>,
            >("counting choice vertices")
        };
        let total_length = self
            .strings()
            .iter()
            .try_fold(0usize, |total, string| total.checked_add(string.len()))
            .ok_or_else(overflow)?;
        let symbols = self
            .strings()
            .iter()
            .flatten()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let per_slot = symbols
            .len()
            .checked_mul(3)
            .and_then(|value| value.checked_add(2))
            .and_then(|value| {
                total_length
                    .checked_add(self.num_strings())?
                    .checked_mul(2)?
                    .checked_add(value)
            })
            .ok_or_else(overflow)?;
        let num_vertices = self
            .max_length()
            .checked_mul(per_slot)
            .ok_or_else(overflow)?;
        <Self as ReduceTo<MaximumIndependentSet<SimpleGraph, One>>>::exact_i64(
            num_vertices,
            "representing the independent-set objective",
        )?;
        let mut choices = Vec::new();
        let mut cursor = 0usize;
        let mut num_groups = 0;
        let mut add_group = |slot, string: Option<usize>, values: &[usize]| {
            for value in values
                .iter()
                .copied()
                .map(Some)
                .chain(std::iter::once(None))
            {
                let count = if string.is_none() && value.is_some() {
                    3
                } else {
                    2
                };
                let end = cursor.checked_add(count).ok_or_else(overflow)?;
                choices.push(Choice {
                    group: num_groups,
                    slot,
                    string,
                    value,
                    vertices: cursor..end,
                });
                cursor = end;
            }
            num_groups += 1;
            Ok::<_, crate::rules::ReductionError>(())
        };
        for slot in 0..self.max_length() {
            add_group(slot, None, &symbols)?;
        }
        for (string, input) in self.strings().iter().enumerate() {
            let positions = (0..input.len()).collect::<Vec<_>>();
            for slot in 0..self.max_length() {
                add_group(slot, Some(string), &positions)?;
            }
        }
        let mut edges = Vec::new();
        for (i, left) in choices.iter().enumerate() {
            for right in &choices[i + 1..] {
                let conflict = if left.group == right.group {
                    true
                } else {
                    // Groups are generated in slot order, with symbols before embeddings.
                    match (left.string, right.string) {
                        (None, None) => left.value.is_none() && right.value.is_some(),
                        (Some(a), Some(b)) if a == b => right
                            .value
                            .is_some_and(|j| left.value.is_none_or(|i| i >= j)),
                        (None, Some(string)) if left.slot == right.slot => {
                            left.value != right.value.map(|j| self.strings()[string][j])
                        }
                        _ => false,
                    }
                };
                if conflict {
                    for a in left.vertices.clone() {
                        for b in right.vertices.clone() {
                            edges.push((a, b));
                        }
                    }
                }
            }
        }
        let target = MaximumIndependentSet::new(
            SimpleGraph::new(num_vertices, edges),
            vec![One; num_vertices],
        );
        Ok(ReductionLCSToIS {
            target,
            choices,
            num_groups,
            max_length: self.max_length(),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcommonsubsequence_to_maximumindependentset",
        build: || {
            let source = LongestCommonSubsequence::new(3, vec![vec![0, 1, 0, 2], vec![1, 0, 2, 0]]);
            let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source)
                .expect("reduction should succeed");
            // BAC, then padding; its positions are (1,2,3) and (0,1,2).
            let assignments = [
                Some(1),
                Some(0),
                Some(2),
                None,
                Some(1),
                Some(2),
                Some(3),
                None,
                Some(0),
                Some(1),
                Some(2),
                None,
            ];
            let mut target_config = vec![false; reduction.target.num_vertices()];
            for choice in &reduction.choices {
                if assignments[choice.group] == choice.value {
                    target_config[choice.vertices.clone()].fill(true);
                }
            }
            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::json!(&assignments[..source.max_length()]),
                    target_config: serde_json::json!(target_config),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs"]
mod tests;
