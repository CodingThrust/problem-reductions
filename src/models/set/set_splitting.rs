//! Set Splitting problem implementation.
//!
//! Set Splitting asks whether a universe can be 2-colored so that every
//! specified subset is non-monochromatic (contains both colors).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SetSplitting",
        display_name: "Set Splitting",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Partition a universe into two parts so that every subset is non-monochromatic",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "universe_size" },
            FieldInfo { name: "subsets", type_name: "Vec<Vec<usize>>", description: "Subsets that must each contain elements from both parts" },
        ],
    }
}

/// The Set Splitting problem.
///
/// Given a finite universe $U = \{0, \ldots, n-1\}$ and a collection
/// $\mathcal{C}$ of subsets of $U$, decide whether there exists a
/// 2-coloring (partition into $S_1$ and $S_2$) of $U$ such that every
/// subset in $\mathcal{C}$ is non-monochromatic, i.e., contains at
/// least one element from each part.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::SetSplitting;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Universe {0,1,2,3,4,5}, subsets that all must be split
/// let problem = SetSplitting::new(6, vec![
///     vec![0, 1, 2],
///     vec![2, 3, 4],
///     vec![0, 4, 5],
///     vec![1, 3, 5],
/// ]);
///
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem);
/// assert!(witness.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "SetSplittingDef")]
pub struct SetSplitting {
    /// Size of the universe.
    universe_size: usize,
    /// Subsets that must each contain elements from both parts.
    subsets: Vec<Vec<usize>>,
}

impl SetSplitting {
    /// Create a new Set Splitting problem.
    ///
    /// # Panics
    ///
    /// Panics if any subset is empty, has fewer than 2 elements, or contains an
    /// element outside the universe.
    pub fn new(universe_size: usize, subsets: Vec<Vec<usize>>) -> Self {
        Self::try_new(universe_size, subsets).unwrap_or_else(|err| panic!("{err}"))
    }

    /// Create a new Set Splitting problem, returning an error instead of panicking.
    pub fn try_new(universe_size: usize, subsets: Vec<Vec<usize>>) -> Result<Self, String> {
        for (i, subset) in subsets.iter().enumerate() {
            if subset.len() < 2 {
                return Err(format!(
                    "Subset {} has {} element(s), expected at least 2",
                    i,
                    subset.len()
                ));
            }
            for &elem in subset {
                if elem >= universe_size {
                    return Err(format!(
                        "Subset {} contains element {} which is outside universe of size {}",
                        i, elem, universe_size
                    ));
                }
            }
        }
        Ok(Self {
            universe_size,
            subsets,
        })
    }

    /// Get the size of the universe.
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    /// Get the number of subsets.
    pub fn num_subsets(&self) -> usize {
        self.subsets.len()
    }

    /// Get the subsets.
    pub fn subsets(&self) -> &[Vec<usize>] {
        &self.subsets
    }

    /// Check if a coloring (config) splits all subsets.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config).0
    }
}

impl Problem for SetSplitting {
    const NAME: &'static str = "SetSplitting";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.universe_size]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or(self.subsets.iter().all(|subset| {
            let has_zero = subset.iter().any(|&e| config[e] == 0);
            let has_one = subset.iter().any(|&e| config[e] == 1);
            has_zero && has_one
        }))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default SetSplitting => "2^universe_size",
}

#[derive(Debug, Clone, Deserialize)]
struct SetSplittingDef {
    universe_size: usize,
    subsets: Vec<Vec<usize>>,
}

impl TryFrom<SetSplittingDef> for SetSplitting {
    type Error = String;

    fn try_from(value: SetSplittingDef) -> Result<Self, Self::Error> {
        Self::try_new(value.universe_size, value.subsets)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "set_splitting",
        instance: Box::new(SetSplitting::new(
            6,
            vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4, 5], vec![1, 3, 5]],
        )),
        // config[i]=0 means element i in S1, config[i]=1 means element i in S2
        // S1={1,3,4}, S2={0,2,5} → config [1,0,1,0,0,1]
        optimal_config: vec![1, 0, 1, 0, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/set_splitting.rs"]
mod tests;
