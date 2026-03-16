//! Set Basis problem implementation.
//!
//! Given a collection of sets over a finite universe and an integer `k`,
//! determine whether there exist `k` basis sets such that every target set
//! can be reconstructed as a union of some subcollection of the basis.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SetBasis",
        display_name: "Set Basis",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a collection of sets admits a basis of size k under union",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Size of the ground set S" },
            FieldInfo { name: "collection", type_name: "Vec<Vec<usize>>", description: "Collection C of target subsets of S" },
            FieldInfo { name: "k", type_name: "usize", description: "Required number of basis sets" },
        ],
    }
}

/// The Set Basis decision problem.
///
/// Given a collection `C` of subsets of a finite set `S` and an integer `k`,
/// determine whether there exists a collection `B` of exactly `k` subsets of
/// `S` such that every set in `C` can be expressed as the union of some
/// subcollection of `B`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBasis {
    /// Size of the universe (elements are `0..universe_size`).
    universe_size: usize,
    /// Collection of target sets, each sorted and deduplicated.
    collection: Vec<Vec<usize>>,
    /// Number of basis sets to encode in a configuration.
    k: usize,
}

impl SetBasis {
    /// Create a new Set Basis instance.
    ///
    /// # Panics
    ///
    /// Panics if any element in `collection` lies outside the universe.
    pub fn new(universe_size: usize, collection: Vec<Vec<usize>>, k: usize) -> Self {
        let mut collection = collection;
        for (set_index, set) in collection.iter_mut().enumerate() {
            set.sort_unstable();
            set.dedup();
            for &element in set.iter() {
                assert!(
                    element < universe_size,
                    "Set {} contains element {} which is outside universe of size {}",
                    set_index,
                    element,
                    universe_size
                );
            }
        }

        Self {
            universe_size,
            collection,
            k,
        }
    }

    /// Return the universe size.
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    /// Return the number of target sets.
    pub fn num_sets(&self) -> usize {
        self.collection.len()
    }

    /// Return the required basis size.
    pub fn basis_size(&self) -> usize {
        self.k
    }

    /// Return the target collection.
    pub fn collection(&self) -> &[Vec<usize>] {
        &self.collection
    }

    /// Return a single target set.
    pub fn get_set(&self, index: usize) -> Option<&Vec<usize>> {
        self.collection.get(index)
    }

    /// Check whether the configuration is a satisfying Set Basis solution.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }

    fn decode_basis(&self, config: &[usize]) -> Option<Vec<Vec<usize>>> {
        let expected_len = self.k * self.universe_size;
        if config.len() != expected_len || config.iter().any(|&value| value > 1) {
            return None;
        }

        let mut basis = Vec::with_capacity(self.k);
        for row in 0..self.k {
            let mut subset = Vec::new();
            let start = row * self.universe_size;
            for element in 0..self.universe_size {
                if config[start + element] == 1 {
                    subset.push(element);
                }
            }
            basis.push(subset);
        }
        Some(basis)
    }

    fn is_subset(candidate: &[usize], target: &[usize]) -> bool {
        candidate
            .iter()
            .all(|element| target.binary_search(element).is_ok())
    }

    fn can_represent_target(basis: &[Vec<usize>], target: &[usize], universe_size: usize) -> bool {
        let mut covered = vec![false; universe_size];
        for subset in basis {
            if Self::is_subset(subset, target) {
                for &element in subset {
                    covered[element] = true;
                }
            }
        }

        target.iter().all(|&element| covered[element])
    }
}

impl Problem for SetBasis {
    const NAME: &'static str = "SetBasis";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.k * self.universe_size]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let Some(basis) = self.decode_basis(config) else {
            return false;
        };

        self.collection
            .iter()
            .all(|target| Self::can_represent_target(&basis, target, self.universe_size))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for SetBasis {}

crate::declare_variants! {
    default sat SetBasis => "2^(basis_size * universe_size)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "set_basis",
        build: || {
            let problem = SetBasis::new(
                4,
                vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![0, 1, 2]],
                3,
            );
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0]],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/set_basis.rs"]
mod tests;
