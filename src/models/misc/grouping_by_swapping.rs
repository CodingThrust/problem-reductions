//! Grouping by Swapping problem implementation.
//!
//! Given a string over a finite alphabet and a swap budget `K`, determine
//! whether at most `K` adjacent swaps can transform the string so that every
//! symbol appears in a single contiguous block.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "GroupingBySwapping",
        display_name: "Grouping by Swapping",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Group equal symbols into contiguous blocks using at most K adjacent swaps",
        fields: &[
            FieldInfo { name: "alphabet_size", type_name: "usize", description: "Size of the alphabet" },
            FieldInfo { name: "string", type_name: "Vec<usize>", description: "Input string over {0, ..., alphabet_size-1}" },
            FieldInfo { name: "budget", type_name: "usize", description: "Maximum number of adjacent swaps allowed" },
        ],
    }
}

/// Grouping by Swapping.
///
/// A configuration is a length-`budget` swap program. Each entry is either an
/// adjacent swap position `i` (swap positions `i` and `i + 1`) or the special
/// no-op value `string_len - 1`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupingBySwapping {
    alphabet_size: usize,
    string: Vec<usize>,
    budget: usize,
}

impl GroupingBySwapping {
    /// Create a new GroupingBySwapping instance.
    ///
    /// # Panics
    ///
    /// Panics if the string contains a symbol outside the declared alphabet,
    /// or if the string is empty while the budget is positive.
    pub fn new(alphabet_size: usize, string: Vec<usize>, budget: usize) -> Self {
        assert!(
            alphabet_size > 0 || string.is_empty(),
            "alphabet_size must be > 0 when string is non-empty"
        );
        assert!(
            string.iter().all(|&symbol| symbol < alphabet_size),
            "input symbols must be less than alphabet_size"
        );
        assert!(
            !string.is_empty() || budget == 0,
            "budget must be 0 when string is empty"
        );
        Self {
            alphabet_size,
            string,
            budget,
        }
    }

    /// Returns the alphabet size.
    pub fn alphabet_size(&self) -> usize {
        self.alphabet_size
    }

    /// Returns the input string.
    pub fn string(&self) -> &[usize] {
        &self.string
    }

    /// Returns the swap budget.
    pub fn budget(&self) -> usize {
        self.budget
    }

    /// Returns the input string length.
    pub fn string_len(&self) -> usize {
        self.string.len()
    }

    /// Applies a swap program to the input string.
    ///
    /// Returns `None` if the configuration has the wrong length or contains an
    /// out-of-range swap slot.
    pub fn apply_swap_program(&self, config: &[usize]) -> Option<Vec<usize>> {
        if config.len() != self.budget {
            return None;
        }
        if self.string.is_empty() {
            return if config.is_empty() {
                Some(Vec::new())
            } else {
                None
            };
        }

        let no_op = self.string.len() - 1;
        let mut current = self.string.clone();
        for &slot in config {
            if slot >= self.string.len() {
                return None;
            }
            if slot != no_op {
                current.swap(slot, slot + 1);
            }
        }
        Some(current)
    }

    /// Returns whether every symbol in `candidate` appears in a single block.
    pub fn is_grouped(&self, candidate: &[usize]) -> bool {
        if candidate.iter().any(|&symbol| symbol >= self.alphabet_size) {
            return false;
        }
        if candidate.is_empty() {
            return true;
        }

        let mut closed = vec![false; self.alphabet_size];
        let mut current_symbol = candidate[0];

        for &symbol in candidate.iter().skip(1) {
            if symbol == current_symbol {
                continue;
            }
            closed[current_symbol] = true;
            if closed[symbol] {
                return false;
            }
            current_symbol = symbol;
        }

        true
    }
}

impl Problem for GroupingBySwapping {
    const NAME: &'static str = "GroupingBySwapping";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![self.string_len(); self.budget]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            self.apply_swap_program(config)
                .is_some_and(|candidate| self.is_grouped(&candidate))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default GroupingBySwapping => "string_len ^ budget",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "grouping_by_swapping",
        instance: Box::new(GroupingBySwapping::new(3, vec![0, 1, 2, 0, 1, 2], 5)),
        optimal_config: vec![2, 1, 3, 5, 5],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/grouping_by_swapping.rs"]
mod tests;
