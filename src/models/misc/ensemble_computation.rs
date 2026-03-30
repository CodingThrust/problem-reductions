//! Ensemble Computation problem implementation.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "EnsembleComputation",
        display_name: "Ensemble Computation",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find the minimum-length sequence of disjoint unions that builds all required subsets",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Number of elements in the universe A" },
            FieldInfo { name: "subsets", type_name: "Vec<Vec<usize>>", description: "Required subsets that must appear among the computed z_i values" },
            FieldInfo { name: "budget", type_name: "usize", description: "Maximum number of union operations J" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "EnsembleComputationDef")]
pub struct EnsembleComputation {
    universe_size: usize,
    subsets: Vec<Vec<usize>>,
    budget: usize,
}

impl EnsembleComputation {
    pub fn new(universe_size: usize, subsets: Vec<Vec<usize>>, budget: usize) -> Self {
        Self::try_new(universe_size, subsets, budget).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Create with an automatically derived search-space bound.
    ///
    /// The default budget is the sum of all subset sizes (worst-case without
    /// intermediate-set reuse). This is always sufficient for the optimal
    /// solution to fit within the search space.
    pub fn with_default_budget(universe_size: usize, subsets: Vec<Vec<usize>>) -> Self {
        let budget = Self::default_budget(&subsets);
        Self::new(universe_size, subsets, budget)
    }

    /// Compute a default search-space bound from the subsets.
    ///
    /// Returns the sum of all subset sizes, clamped to at least 1.
    pub fn default_budget(subsets: &[Vec<usize>]) -> usize {
        subsets.iter().map(|s| s.len()).sum::<usize>().max(1)
    }

    pub fn try_new(
        universe_size: usize,
        subsets: Vec<Vec<usize>>,
        budget: usize,
    ) -> Result<Self, String> {
        if budget == 0 {
            return Err("budget must be positive".to_string());
        }
        let subsets = subsets
            .into_iter()
            .enumerate()
            .map(|(subset_index, subset)| {
                Self::normalize_subset(universe_size, subset).ok_or_else(|| {
                    format!(
                        "subset {subset_index} contains element outside universe of size {universe_size}"
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            universe_size,
            subsets,
            budget,
        })
    }

    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    pub fn subsets(&self) -> &[Vec<usize>] {
        &self.subsets
    }

    pub fn num_subsets(&self) -> usize {
        self.subsets.len()
    }

    pub fn budget(&self) -> usize {
        self.budget
    }

    fn normalize_subset(universe_size: usize, mut subset: Vec<usize>) -> Option<Vec<usize>> {
        if subset.iter().any(|&element| element >= universe_size) {
            return None;
        }
        subset.sort_unstable();
        subset.dedup();
        Some(subset)
    }

    fn decode_operand(&self, operand: usize, computed: &[Vec<usize>]) -> Option<Vec<usize>> {
        if operand < self.universe_size {
            return Some(vec![operand]);
        }
        computed.get(operand - self.universe_size).cloned()
    }

    fn are_disjoint(left: &[usize], right: &[usize]) -> bool {
        let mut i = 0;
        let mut j = 0;

        while i < left.len() && j < right.len() {
            match left[i].cmp(&right[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => return false,
            }
        }

        true
    }

    fn union_disjoint(left: &[usize], right: &[usize]) -> Vec<usize> {
        let mut union = Vec::with_capacity(left.len() + right.len());
        let mut i = 0;
        let mut j = 0;

        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                union.push(left[i]);
                i += 1;
            } else {
                union.push(right[j]);
                j += 1;
            }
        }

        union.extend_from_slice(&left[i..]);
        union.extend_from_slice(&right[j..]);
        union
    }

    fn required_subsets(&self) -> Option<Vec<Vec<usize>>> {
        self.subsets
            .iter()
            .cloned()
            .map(|subset| Self::normalize_subset(self.universe_size, subset))
            .collect()
    }

    fn all_required_subsets_present(
        required_subsets: &[Vec<usize>],
        computed: &[Vec<usize>],
    ) -> bool {
        required_subsets
            .iter()
            .all(|subset| computed.iter().any(|candidate| candidate == subset))
    }
}

impl Problem for EnsembleComputation {
    const NAME: &'static str = "EnsembleComputation";
    type Value = Min<usize>;

    fn dims(&self) -> Vec<usize> {
        vec![self.universe_size + self.budget; 2 * self.budget]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != 2 * self.budget {
            return Min(None);
        }

        let Some(required_subsets) = self.required_subsets() else {
            return Min(None);
        };
        if required_subsets.is_empty() {
            return Min(Some(0));
        }

        let mut computed = Vec::with_capacity(self.budget);
        for step in 0..self.budget {
            let left_operand = config[2 * step];
            let right_operand = config[2 * step + 1];

            let Some(left) = self.decode_operand(left_operand, &computed) else {
                return Min(None);
            };
            let Some(right) = self.decode_operand(right_operand, &computed) else {
                return Min(None);
            };

            if !Self::are_disjoint(&left, &right) {
                return Min(None);
            }

            computed.push(Self::union_disjoint(&left, &right));
            if Self::all_required_subsets_present(&required_subsets, &computed) {
                return Min(Some(step + 1));
            }
        }

        Min(None)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default EnsembleComputation => "(universe_size + budget)^(2 * budget)",
}

#[derive(Debug, Clone, Deserialize)]
struct EnsembleComputationDef {
    universe_size: usize,
    subsets: Vec<Vec<usize>>,
    budget: usize,
}

impl TryFrom<EnsembleComputationDef> for EnsembleComputation {
    type Error = String;

    fn try_from(value: EnsembleComputationDef) -> Result<Self, Self::Error> {
        Self::try_new(value.universe_size, value.subsets, value.budget)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Keep the canonical example small enough for the example-db optimality check to solve
    // it via brute force, while still demonstrating reuse of a previously computed set.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "ensemble_computation",
        instance: Box::new(EnsembleComputation::new(
            3,
            vec![vec![0, 1], vec![0, 1, 2]],
            2,
        )),
        optimal_config: vec![0, 1, 3, 2],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/ensemble_computation.rs"]
mod tests;
