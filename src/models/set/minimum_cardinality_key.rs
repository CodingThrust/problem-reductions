//! Minimum Cardinality Key problem implementation.
//!
//! Given a set of attribute names and functional dependencies,
//! find a candidate key of minimum cardinality.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCardinalityKey",
        display_name: "Minimum Cardinality Key",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a candidate key of minimum cardinality in a relational system",
        fields: &[
            FieldInfo { name: "num_attributes", type_name: "usize", description: "Number of attributes in the relation" },
            FieldInfo { name: "dependencies", type_name: "Vec<(Vec<usize>, Vec<usize>)>", description: "Functional dependencies as (lhs, rhs) pairs" },
        ],
    }
}

/// The Minimum Cardinality Key optimization problem.
///
/// Given a set of attributes `A = {0, ..., n-1}` and a set of functional
/// dependencies `F` (each a pair `(X, Y)` where `X, Y` are subsets of `A`),
/// find a subset `K ⊆ A` of minimum cardinality such that the closure of `K`
/// under `F` equals `A` (i.e., `K` is a key).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCardinalityKey {
    /// Number of attributes (elements are `0..num_attributes`).
    num_attributes: usize,
    /// Functional dependencies as `(lhs, rhs)` pairs.
    dependencies: Vec<(Vec<usize>, Vec<usize>)>,
}

impl MinimumCardinalityKey {
    /// Create a new Minimum Cardinality Key instance.
    ///
    /// # Panics
    ///
    /// Panics if any attribute index in a dependency lies outside the attribute set.
    pub fn new(
        num_attributes: usize,
        dependencies: Vec<(Vec<usize>, Vec<usize>)>,
    ) -> Self {
        let mut dependencies = dependencies;
        for (dep_index, (lhs, rhs)) in dependencies.iter_mut().enumerate() {
            lhs.sort_unstable();
            lhs.dedup();
            rhs.sort_unstable();
            rhs.dedup();
            for &attr in lhs.iter().chain(rhs.iter()) {
                assert!(
                    attr < num_attributes,
                    "Dependency {} contains attribute {} which is outside attribute set of size {}",
                    dep_index,
                    attr,
                    num_attributes
                );
            }
        }

        Self {
            num_attributes,
            dependencies,
        }
    }

    /// Return the number of attributes.
    pub fn num_attributes(&self) -> usize {
        self.num_attributes
    }

    /// Return the number of functional dependencies.
    pub fn num_dependencies(&self) -> usize {
        self.dependencies.len()
    }

    /// Return the functional dependencies.
    pub fn dependencies(&self) -> &[(Vec<usize>, Vec<usize>)] {
        &self.dependencies
    }

    /// Compute the attribute closure of the selected attributes under the
    /// functional dependencies. Starts with the selected set and repeatedly
    /// applies each FD: if all lhs attributes are in the closure, add all rhs
    /// attributes. Repeats until no change.
    fn compute_closure(&self, selected: &[bool]) -> Vec<bool> {
        let mut closure = selected.to_vec();
        loop {
            let mut changed = false;
            for (lhs, rhs) in &self.dependencies {
                if lhs.iter().all(|&a| closure[a]) {
                    for &a in rhs {
                        if !closure[a] {
                            closure[a] = true;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        closure
    }

    /// Check whether the selected attributes form a key (their closure equals
    /// the full attribute set).
    fn is_key(&self, selected: &[bool]) -> bool {
        let closure = self.compute_closure(selected);
        closure.iter().all(|&v| v)
    }

}

impl Problem for MinimumCardinalityKey {
    const NAME: &'static str = "MinimumCardinalityKey";
    type Value = Min<i64>;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_attributes]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i64> {
        if config.len() != self.num_attributes || config.iter().any(|&v| v > 1) {
            return Min(None);
        }

        let selected: Vec<bool> = config.iter().map(|&v| v == 1).collect();

        if self.is_key(&selected) {
            let count = selected.iter().filter(|&&v| v).count();
            Min(Some(count as i64))
        } else {
            Min(None)
        }
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default MinimumCardinalityKey => "2^num_attributes",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_cardinality_key",
        instance: Box::new(MinimumCardinalityKey::new(
            6,
            vec![
                (vec![0, 1], vec![2]),
                (vec![0, 2], vec![3]),
                (vec![1, 3], vec![4]),
                (vec![2, 4], vec![5]),
            ],
        )),
        optimal_config: vec![1, 1, 0, 0, 0, 0],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/minimum_cardinality_key.rs"]
mod tests;
