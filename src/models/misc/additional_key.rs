//! Additional Key problem implementation.
//!
//! Given a relational schema (R, F) and a set K of known candidate keys,
//! determine whether there exists a candidate key of R (under the functional
//! dependencies F) that is not in K. A candidate key is a minimal set of
//! attributes whose closure under F covers all of R.
//!
//! The problem is NP-complete (Garey & Johnson, SR7).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "AdditionalKey",
        display_name: "Additional Key",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a relational schema has a candidate key not in a given set",
        fields: &[
            FieldInfo { name: "num_attributes", type_name: "usize", description: "Number of attributes in A" },
            FieldInfo { name: "dependencies", type_name: "Vec<(Vec<usize>, Vec<usize>)>", description: "Functional dependencies F; each (lhs, rhs)" },
            FieldInfo { name: "relation_attrs", type_name: "Vec<usize>", description: "Relation scheme attributes R ⊆ A" },
            FieldInfo { name: "known_keys", type_name: "Vec<Vec<usize>>", description: "Known candidate keys K" },
        ],
    }
}

/// The Additional Key problem.
///
/// Given a set `A` of attributes, a set of functional dependencies `F` over `A`,
/// a relation scheme `R ⊆ A`, and a set `K` of known candidate keys of `R`
/// under `F`, determine whether `R` has a candidate key not in `K`.
///
/// A **candidate key** is a minimal subset `X ⊆ R` such that the closure of `X`
/// under `F` contains all attributes of `R`.
///
/// # Representation
///
/// Each attribute in `R` has a binary variable: `x_i = 1` if the attribute is
/// selected, `0` otherwise. A configuration is satisfying iff the selected
/// attributes form a candidate key of `R` that is not in `K`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::AdditionalKey;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = AdditionalKey::new(
///     3,
///     vec![(vec![0], vec![1, 2])],
///     vec![0, 1, 2],
///     vec![],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalKey {
    num_attributes: usize,
    dependencies: Vec<(Vec<usize>, Vec<usize>)>,
    relation_attrs: Vec<usize>,
    known_keys: Vec<Vec<usize>>,
}

impl AdditionalKey {
    /// Create a new AdditionalKey instance.
    ///
    /// # Panics
    ///
    /// Panics if any attribute index is >= `num_attributes`, or if
    /// `relation_attrs` contains duplicates.
    pub fn new(
        num_attributes: usize,
        dependencies: Vec<(Vec<usize>, Vec<usize>)>,
        relation_attrs: Vec<usize>,
        known_keys: Vec<Vec<usize>>,
    ) -> Self {
        // Validate all attribute indices
        for &a in &relation_attrs {
            assert!(
                a < num_attributes,
                "relation_attrs element {a} >= num_attributes {num_attributes}"
            );
        }
        // Validate relation_attrs uniqueness
        let mut sorted_ra = relation_attrs.clone();
        sorted_ra.sort_unstable();
        sorted_ra.dedup();
        assert_eq!(
            sorted_ra.len(),
            relation_attrs.len(),
            "relation_attrs contains duplicates"
        );
        for (lhs, rhs) in &dependencies {
            for &a in lhs {
                assert!(
                    a < num_attributes,
                    "dependency lhs attribute {a} >= num_attributes {num_attributes}"
                );
            }
            for &a in rhs {
                assert!(
                    a < num_attributes,
                    "dependency rhs attribute {a} >= num_attributes {num_attributes}"
                );
            }
        }
        for key in &known_keys {
            for &a in key {
                assert!(
                    a < num_attributes,
                    "known_keys attribute {a} >= num_attributes {num_attributes}"
                );
            }
        }
        // Sort known_keys entries internally for consistent comparison
        let known_keys: Vec<Vec<usize>> = known_keys
            .into_iter()
            .map(|mut k| {
                k.sort_unstable();
                k
            })
            .collect();
        Self {
            num_attributes,
            dependencies,
            relation_attrs,
            known_keys,
        }
    }

    /// Returns the number of attributes in the universal set A.
    pub fn num_attributes(&self) -> usize {
        self.num_attributes
    }

    /// Returns the number of functional dependencies.
    pub fn num_dependencies(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns the number of attributes in the relation scheme R.
    pub fn num_relation_attrs(&self) -> usize {
        self.relation_attrs.len()
    }

    /// Returns the number of known candidate keys.
    pub fn num_known_keys(&self) -> usize {
        self.known_keys.len()
    }

    /// Returns the functional dependencies.
    pub fn dependencies(&self) -> &[(Vec<usize>, Vec<usize>)] {
        &self.dependencies
    }

    /// Returns the relation scheme attributes.
    pub fn relation_attrs(&self) -> &[usize] {
        &self.relation_attrs
    }

    /// Returns the known candidate keys.
    pub fn known_keys(&self) -> &[Vec<usize>] {
        &self.known_keys
    }

    /// Compute the closure of a set of attributes under the functional dependencies.
    fn compute_closure(&self, attrs: &[bool]) -> Vec<bool> {
        let mut closure = attrs.to_vec();
        let mut changed = true;
        while changed {
            changed = false;
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
        }
        closure
    }
}

impl Problem for AdditionalKey {
    const NAME: &'static str = "AdditionalKey";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.relation_attrs.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        // Check config length
        if config.len() != self.relation_attrs.len() {
            return false;
        }
        // Check all values are 0 or 1
        if config.iter().any(|&v| v >= 2) {
            return false;
        }

        // Build selected attribute set
        let selected: Vec<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| self.relation_attrs[i])
            .collect();

        // Empty selection is not a key
        if selected.is_empty() {
            return false;
        }

        // Compute closure of selected attributes
        let mut attr_set = vec![false; self.num_attributes];
        for &a in &selected {
            attr_set[a] = true;
        }
        let closure = self.compute_closure(&attr_set);

        // Check closure covers all relation_attrs
        if !self.relation_attrs.iter().all(|&a| closure[a]) {
            return false;
        }

        // Check minimality: removing any single selected attribute should break coverage
        for &a in &selected {
            let mut reduced = attr_set.clone();
            reduced[a] = false;
            let reduced_closure = self.compute_closure(&reduced);
            if self.relation_attrs.iter().all(|&ra| reduced_closure[ra]) {
                return false; // Not minimal
            }
        }

        // Build sorted selected vec and check it's not in known_keys
        let mut sorted_selected = selected;
        sorted_selected.sort_unstable();
        !self.known_keys.contains(&sorted_selected)
    }
}

impl SatisfactionProblem for AdditionalKey {}

crate::declare_variants! {
    default sat AdditionalKey => "2^num_relation_attrs * num_dependencies * num_attributes",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "additional_key",
        instance: Box::new(AdditionalKey::new(
            6,
            vec![
                (vec![0, 1], vec![2, 3]),
                (vec![2, 3], vec![4, 5]),
                (vec![4, 5], vec![0, 1]),
                (vec![0, 2], vec![3]),
                (vec![3, 5], vec![1]),
            ],
            vec![0, 1, 2, 3, 4, 5],
            vec![vec![0, 1], vec![2, 3], vec![4, 5]],
        )),
        optimal_config: vec![1, 0, 1, 0, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/additional_key.rs"]
mod tests;
