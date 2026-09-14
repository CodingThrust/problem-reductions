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
        category: crate::registry::ProblemCategory::Set,
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
#[serde(try_from = "MinimumCardinalityKeyData")]
pub struct MinimumCardinalityKey {
    /// Number of attributes (elements are `0..num_attributes`).
    num_attributes: usize,
    /// Functional dependencies as `(lhs, rhs)` pairs.
    dependencies: Vec<(Vec<usize>, Vec<usize>)>,
}

#[derive(Deserialize)]
struct MinimumCardinalityKeyData {
    num_attributes: usize,
    dependencies: Vec<(Vec<usize>, Vec<usize>)>,
}

impl TryFrom<MinimumCardinalityKeyData> for MinimumCardinalityKey {
    type Error = crate::registry::ConstructionError;

    fn try_from(data: MinimumCardinalityKeyData) -> Result<Self, Self::Error> {
        Self::new(data.num_attributes, data.dependencies)
    }
}

impl MinimumCardinalityKey {
    /// Create a new Minimum Cardinality Key instance.
    ///
    /// # Errors
    ///
    /// Returns an error when the instance violates its documented input conditions.
    pub fn new(
        num_attributes: usize,
        mut dependencies: Vec<(Vec<usize>, Vec<usize>)>,
    ) -> Result<Self, crate::registry::ConstructionError> {
        for (index, (lhs, rhs)) in dependencies.iter_mut().enumerate() {
            lhs.sort_unstable();
            lhs.dedup();
            rhs.sort_unstable();
            rhs.dedup();
            if let Some(attribute) = lhs
                .iter()
                .chain(rhs.iter())
                .find(|&&attribute| attribute >= num_attributes)
            {
                return Err(format!("dependency {index} contains attribute {attribute} outside attribute set of size {num_attributes}").into());
            }
        }
        Ok(Self {
            num_attributes,
            dependencies,
        })
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
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_parameters![
        ("num_attributes", num_attributes),
        ("num_dependencies", num_dependencies),
    ];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.num_attributes {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "attribute-selection length does not match the relation".into(),
                ));
            }

            if self.is_key(config) {
                let count = config.iter().filter(|&&v| v).count();
                Min(Some(i64::try_from(count).map_err(|_| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "converting selected-attribute count to i64".into(),
                    )
                })?))
            } else {
                Min(None)
            }
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for MinimumCardinalityKey {
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(self.num_attributes)
    }

    fn dimension(&self, _variable: usize) -> Result<usize, crate::solvers::SolveError> {
        Ok(2usize)
    }
}

crate::declare_variants! {
    default MinimumCardinalityKey => "2^num_attributes",
}

crate::register_brute_force! {
    MinimumCardinalityKey decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_cardinality_key",
        instance: Box::new(
            MinimumCardinalityKey::new(
                6,
                vec![
                    (vec![0, 1], vec![2]),
                    (vec![0, 2], vec![3]),
                    (vec![1, 3], vec![4]),
                    (vec![2, 4], vec![5]),
                ],
            )
            .unwrap(),
        ),
        optimal_config: serde_json::json!(vec![true, true, false, false, false, false]),
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/minimum_cardinality_key.rs"]
mod tests;
