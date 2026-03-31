//! Three-Matroid Intersection problem implementation.
//!
//! Given three partition matroids on a common ground set E and a positive integer K,
//! determine whether there exists a common independent set of size K.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ThreeMatroidIntersection",
        display_name: "Three-Matroid Intersection",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a common independent set of size K in three partition matroids",
        fields: &[
            FieldInfo { name: "ground_set_size", type_name: "usize", description: "Number of elements in the ground set E" },
            FieldInfo { name: "partitions", type_name: "Vec<Vec<Vec<usize>>>", description: "Three partition matroids, each as a list of groups" },
            FieldInfo { name: "bound", type_name: "usize", description: "Required size K of the common independent set" },
        ],
    }
}

/// Three-Matroid Intersection problem.
///
/// Given three partition matroids on a common ground set E = {0, ..., n-1} and a
/// positive integer K ≤ |E|, determine whether there exists a subset E' ⊆ E such
/// that |E'| = K and E' is independent in all three matroids.
///
/// A partition matroid is defined by a partition of E into groups. A set S is
/// independent if |S ∩ G| ≤ 1 for every group G.
///
/// While 2-matroid intersection is solvable in polynomial time (Edmonds, 1970),
/// the jump to three matroids captures NP-hardness.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::ThreeMatroidIntersection;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Ground set E = {0, 1, 2, 3, 4, 5}, K = 2
/// let problem = ThreeMatroidIntersection::new(
///     6,
///     vec![
///         vec![vec![0, 1, 2], vec![3, 4, 5]],       // M1
///         vec![vec![0, 3], vec![1, 4], vec![2, 5]],  // M2
///         vec![vec![0, 4], vec![1, 5], vec![2, 3]],  // M3
///     ],
///     2,
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem);
/// assert!(!solutions.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeMatroidIntersection {
    /// Number of elements in the ground set E (elements are 0..ground_set_size).
    ground_set_size: usize,
    /// Three partition matroids. Each matroid is a list of groups, where each
    /// group is a list of element indices. A set is independent in a partition
    /// matroid if it contains at most one element from each group.
    partitions: Vec<Vec<Vec<usize>>>,
    /// Required size K of the common independent set.
    bound: usize,
}

impl ThreeMatroidIntersection {
    /// Create a new Three-Matroid Intersection problem.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `partitions` does not contain exactly 3 matroids
    /// - Any element index is outside `0..ground_set_size`
    /// - `bound` exceeds `ground_set_size`
    pub fn new(ground_set_size: usize, partitions: Vec<Vec<Vec<usize>>>, bound: usize) -> Self {
        assert_eq!(
            partitions.len(),
            3,
            "Expected exactly 3 partition matroids, got {}",
            partitions.len()
        );
        assert!(
            bound <= ground_set_size,
            "Bound {} exceeds ground set size {}",
            bound,
            ground_set_size
        );
        for (m_idx, matroid) in partitions.iter().enumerate() {
            for (g_idx, group) in matroid.iter().enumerate() {
                for &elem in group {
                    assert!(
                        elem < ground_set_size,
                        "Matroid {} group {} contains element {} which is outside 0..{}",
                        m_idx,
                        g_idx,
                        elem,
                        ground_set_size
                    );
                }
            }
        }
        Self {
            ground_set_size,
            partitions,
            bound,
        }
    }

    /// Get the ground set size.
    pub fn ground_set_size(&self) -> usize {
        self.ground_set_size
    }

    /// Get the three partition matroids.
    pub fn partitions(&self) -> &[Vec<Vec<usize>>] {
        &self.partitions
    }

    /// Get the bound K.
    pub fn bound(&self) -> usize {
        self.bound
    }

    /// Get the total number of groups across all three matroids.
    pub fn num_groups(&self) -> usize {
        self.partitions.iter().map(|m| m.len()).sum()
    }
}

impl Problem for ThreeMatroidIntersection {
    const NAME: &'static str = "ThreeMatroidIntersection";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.ground_set_size]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            if config.len() != self.ground_set_size || config.iter().any(|&v| v > 1) {
                return crate::types::Or(false);
            }

            // Check selected set has exactly K elements
            let selected_count: usize = config.iter().filter(|&&v| v == 1).sum();
            if selected_count != self.bound {
                return crate::types::Or(false);
            }

            // Check independence in each of the three partition matroids
            for matroid in &self.partitions {
                for group in matroid {
                    let count = group.iter().filter(|&&e| config[e] == 1).count();
                    if count > 1 {
                        return crate::types::Or(false);
                    }
                }
            }

            true
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default ThreeMatroidIntersection => "2^ground_set_size",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "three_matroid_intersection",
        instance: Box::new(ThreeMatroidIntersection::new(
            6,
            vec![
                vec![vec![0, 1, 2], vec![3, 4, 5]],
                vec![vec![0, 3], vec![1, 4], vec![2, 5]],
                vec![vec![0, 4], vec![1, 5], vec![2, 3]],
            ],
            2,
        )),
        optimal_config: vec![1, 0, 0, 0, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/three_matroid_intersection.rs"]
mod tests;
