//! Set Basis problem implementation.
//!
//! Given a collection of sets over a finite universe and an integer `k`,
//! determine whether there exist `k` basis sets such that every target set
//! can be reconstructed as a union of some subcollection of the basis.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "SetBasis",
        display_name: "Set Basis",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Set,
        module_path: module_path!(),
        description: "Determine whether a collection of sets admits a basis of size k under union",
        fields: SetBasisCreateSpec::FIELDS,
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
    /// Collection of target sets.
    collection: Vec<Vec<usize>>,
    /// Number of basis sets to encode in a configuration.
    k: usize,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct SetBasisCreateSpec {
    /// Size of the ground set S.
    universe_size: usize,
    /// Collection C of target subsets of S.
    subsets: Vec<Vec<usize>>,
    /// Required number of basis sets.
    k: usize,
}

impl TryFrom<SetBasisCreateSpec> for SetBasis {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: SetBasisCreateSpec) -> Result<Self, Self::Error> {
        for (set_index, set) in spec.subsets.iter().enumerate() {
            if let Some(&element) = set.iter().find(|&&element| element >= spec.universe_size) {
                return Err(format!(
                    "subsets[{set_index}] contains element {element} outside universe of size {}",
                    spec.universe_size
                )
                .into());
            }
        }
        Ok(Self::new(spec.universe_size, spec.subsets, spec.k))
    }
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
    pub fn is_valid_solution(
        &self,
        solution: &[Vec<bool>],
    ) -> Result<bool, crate::traits::EvaluationError> {
        if solution.len() != self.k
            || solution
                .iter()
                .any(|subset| subset.len() != self.universe_size)
        {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "set-basis dimensions do not match the instance".into(),
            ));
        }
        let basis = Self::decode_basis(solution);
        Ok(self
            .collection
            .iter()
            .all(|target| Self::can_represent_target(&basis, target, self.universe_size)))
    }

    fn decode_basis(solution: &[Vec<bool>]) -> Vec<Vec<usize>> {
        solution
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(element, &selected)| selected.then_some(element))
                    .collect()
            })
            .collect()
    }

    fn is_subset(candidate: &[usize], target_membership: &[bool]) -> bool {
        candidate.iter().all(|&element| target_membership[element])
    }

    fn can_represent_target(basis: &[Vec<usize>], target: &[usize], universe_size: usize) -> bool {
        let mut target_membership = vec![false; universe_size];
        for &element in target {
            if element >= universe_size {
                return false;
            }
            target_membership[element] = true;
        }

        let mut covered = vec![false; universe_size];
        for subset in basis {
            if Self::is_subset(subset, &target_membership) {
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
    type Solution = Vec<Vec<bool>>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("universe_size", universe_size),
        ("num_sets", num_sets),
        ("basis_size", basis_size),
    ];

    fn evaluate(
        &self,
        solution: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok(crate::types::Or(self.is_valid_solution(solution)?))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for SetBasis {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.k * self.universe_size]
    }
}

crate::declare_variants! {
    default SetBasis => "2^(basis_size * universe_size)" create SetBasisCreateSpec,
}

crate::register_brute_force! {
    SetBasis decode |problem: &SetBasis, indices: Vec<usize>| if problem.universe_size() == 0 { vec![Vec::new(); problem.basis_size()] } else { indices.chunks(problem.universe_size()).map(crate::config::config_to_bits).collect() },
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "set_basis",
        instance: Box::new(SetBasis::new(
            4,
            vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![0, 1, 2]],
            3,
        )),
        optimal_config: serde_json::json!(vec![
            vec![false, false, true, false],
            vec![false, true, false, false],
            vec![true, false, false, false]
        ]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/set_basis.rs"]
mod tests;
