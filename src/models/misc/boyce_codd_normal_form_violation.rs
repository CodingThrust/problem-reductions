//! Boyce-Codd Normal Form Violation problem implementation.
//!
//! Given a set of attributes `A`, a collection of functional dependencies over `A`,
//! and a target subset `A' ⊆ A`, determine whether there exists a non-trivial subset
//! `X ⊆ A'` such that the closure of `X` under the functional dependencies contains
//! some but not all attributes of `A' \ X` — i.e., a witness to a BCNF violation.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BoyceCoddNormalFormViolation",
        display_name: "Boyce-Codd Normal Form Violation",
        aliases: &["BCNFViolation", "BCNF"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Test whether a subset of attributes violates Boyce-Codd normal form",
        fields: &[
            FieldInfo { name: "num_attributes", type_name: "usize", description: "Total number of attributes in A" },
            FieldInfo { name: "functional_deps", type_name: "Vec<(Vec<usize>, Vec<usize>)>", description: "Functional dependencies (lhs_attributes, rhs_attributes)" },
            FieldInfo { name: "target_subset", type_name: "Vec<usize>", description: "Subset A' of attributes to test for BCNF violation" },
        ],
    }
}

/// The Boyce-Codd Normal Form Violation decision problem.
///
/// Given a set of attributes `A = {0, ..., num_attributes - 1}`, a collection of
/// functional dependencies `F` over `A`, and a target subset `A' ⊆ A`, determine
/// whether there exists a subset `X ⊆ A'` such that the closure `X⁺` under `F`
/// contains some element of `A' \ X` but not all — witnessing a BCNF violation.
///
/// # Representation
///
/// A configuration is a binary vector of length `|A'|`, where bit `i = 1` means
/// attribute `target_subset[i]` is included in the candidate set `X`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::BoyceCoddNormalFormViolation;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 6 attributes, FDs: {0,1}→{2}, {2}→{3}, {3,4}→{5}
/// let problem = BoyceCoddNormalFormViolation::new(
///     6,
///     vec![
///         (vec![0, 1], vec![2]),
///         (vec![2], vec![3]),
///         (vec![3, 4], vec![5]),
///     ],
///     vec![0, 1, 2, 3, 4, 5],
/// );
/// let solver = BruteForce::new();
/// // X = {2}: closure = {2, 3}, y=3 ∈ closure, z=0 ∉ closure → BCNF violation
/// assert!(problem.evaluate(&[0, 0, 1, 0, 0, 0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoyceCoddNormalFormViolation {
    /// Total number of attributes (elements are `0..num_attributes`).
    num_attributes: usize,
    /// Functional dependencies as (lhs_attributes, rhs_attributes) pairs.
    functional_deps: Vec<(Vec<usize>, Vec<usize>)>,
    /// Target subset `A'` of attributes to test for BCNF violation.
    target_subset: Vec<usize>,
}

impl BoyceCoddNormalFormViolation {
    /// Create a new Boyce-Codd Normal Form Violation instance.
    ///
    /// # Panics
    ///
    /// Panics if any attribute index in `functional_deps` or `target_subset` is
    /// out of range (≥ `num_attributes`), if `target_subset` is empty, or if any
    /// functional dependency has an empty LHS.
    ///
    /// The constructor also normalizes the instance by sorting and deduplicating
    /// every functional dependency LHS/RHS and the `target_subset`. As a result,
    /// the configuration bit positions correspond to the normalized
    /// `target_subset()` order rather than the caller's original input order.
    pub fn new(
        num_attributes: usize,
        functional_deps: Vec<(Vec<usize>, Vec<usize>)>,
        target_subset: Vec<usize>,
    ) -> Self {
        assert!(!target_subset.is_empty(), "target_subset must be non-empty");

        let mut functional_deps = functional_deps;
        for (fd_index, (lhs, rhs)) in functional_deps.iter_mut().enumerate() {
            assert!(
                !lhs.is_empty(),
                "Functional dependency {} has an empty LHS",
                fd_index
            );
            lhs.sort_unstable();
            lhs.dedup();
            rhs.sort_unstable();
            rhs.dedup();
            for &attr in lhs.iter().chain(rhs.iter()) {
                assert!(
                    attr < num_attributes,
                    "Functional dependency {} contains attribute {} which is out of range (num_attributes = {})",
                    fd_index,
                    attr,
                    num_attributes
                );
            }
        }

        let mut target_subset = target_subset;
        target_subset.sort_unstable();
        target_subset.dedup();
        for &attr in &target_subset {
            assert!(
                attr < num_attributes,
                "target_subset contains attribute {} which is out of range (num_attributes = {})",
                attr,
                num_attributes
            );
        }

        Self {
            num_attributes,
            functional_deps,
            target_subset,
        }
    }

    /// Return the total number of attributes.
    pub fn num_attributes(&self) -> usize {
        self.num_attributes
    }

    /// Return the number of functional dependencies.
    pub fn num_functional_deps(&self) -> usize {
        self.functional_deps.len()
    }

    /// Return the number of attributes in the target subset.
    pub fn num_target_attributes(&self) -> usize {
        self.target_subset.len()
    }

    /// Return the functional dependencies.
    pub fn functional_deps(&self) -> &[(Vec<usize>, Vec<usize>)] {
        &self.functional_deps
    }

    /// Return the target subset `A'`.
    pub fn target_subset(&self) -> &[usize] {
        &self.target_subset
    }

    /// Compute the closure of a set of attributes under a collection of functional dependencies.
    fn compute_closure(x: &HashSet<usize>, fds: &[(Vec<usize>, Vec<usize>)]) -> HashSet<usize> {
        let mut closure = x.clone();
        let mut changed = true;
        while changed {
            changed = false;
            for (lhs, rhs) in fds {
                if lhs.iter().all(|a| closure.contains(a)) {
                    for &a in rhs {
                        if closure.insert(a) {
                            changed = true;
                        }
                    }
                }
            }
        }
        closure
    }
}

impl Problem for BoyceCoddNormalFormViolation {
    const NAME: &'static str = "BoyceCoddNormalFormViolation";
    type Value = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.target_subset.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.target_subset.len() || config.iter().any(|&v| v > 1) {
            return false;
        }
        let x: HashSet<usize> = config
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| self.target_subset[i])
            .collect();
        let closure = Self::compute_closure(&x, &self.functional_deps);
        // Check: ∃ y, z ∈ A' \ X s.t. y ∈ closure ∧ z ∉ closure
        let mut has_in_closure = false;
        let mut has_not_in_closure = false;
        for &a in &self.target_subset {
            if !x.contains(&a) {
                if closure.contains(&a) {
                    has_in_closure = true;
                } else {
                    has_not_in_closure = true;
                }
            }
        }
        has_in_closure && has_not_in_closure
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for BoyceCoddNormalFormViolation {}

crate::declare_variants! {
    default sat BoyceCoddNormalFormViolation => "2^num_target_attributes * num_target_attributes^2 * num_functional_deps",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "boyce_codd_normal_form_violation",
        instance: Box::new(BoyceCoddNormalFormViolation::new(
            6,
            vec![
                (vec![0, 1], vec![2]),
                (vec![2], vec![3]),
                (vec![3, 4], vec![5]),
            ],
            vec![0, 1, 2, 3, 4, 5],
        )),
        // X={2}: closure={2,3}, y=3 in closure, z=0 not in closure -> violation
        optimal_config: vec![0, 0, 1, 0, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/boyce_codd_normal_form_violation.rs"]
mod tests;
