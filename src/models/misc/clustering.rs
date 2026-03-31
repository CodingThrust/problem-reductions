//! Clustering problem implementation.
//!
//! Given a distance matrix over n elements, a cluster count bound K,
//! and a diameter bound B, determine whether the elements can be partitioned
//! into at most K non-empty clusters such that all intra-cluster pairwise
//! distances are at most B.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "Clustering",
        display_name: "Clustering",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Partition elements into at most K clusters where all intra-cluster distances are at most B",
        fields: &[
            FieldInfo { name: "distances", type_name: "Vec<Vec<u64>>", description: "Symmetric distance matrix with zero diagonal" },
            FieldInfo { name: "num_clusters", type_name: "usize", description: "Maximum number of clusters K" },
            FieldInfo { name: "diameter_bound", type_name: "u64", description: "Maximum allowed intra-cluster pairwise distance B" },
        ],
    }
}

/// The Clustering problem.
///
/// Given a set of `n` elements with pairwise distances, a cluster count
/// bound `K`, and a diameter bound `B`, determine whether there exists
/// a partition of the elements into at most `K` non-empty clusters such
/// that for every cluster, all pairwise distances within that cluster
/// are at most `B`.
///
/// # Representation
///
/// Each element `i` is assigned a cluster index `config[i] ∈ {0, ..., K-1}`.
/// The problem is satisfiable iff every non-empty cluster has all pairwise
/// distances ≤ B.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::Clustering;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 elements, 2 clusters, diameter bound 1
/// let distances = vec![
///     vec![0, 1, 3, 3],
///     vec![1, 0, 3, 3],
///     vec![3, 3, 0, 1],
///     vec![3, 3, 1, 0],
/// ];
/// let problem = Clustering::new(distances, 2, 1);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clustering {
    /// Symmetric distance matrix with zero diagonal.
    distances: Vec<Vec<u64>>,
    /// Maximum number of clusters K.
    num_clusters: usize,
    /// Maximum allowed intra-cluster pairwise distance B.
    diameter_bound: u64,
}

impl Clustering {
    /// Create a new Clustering instance.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `distances` is empty
    /// - `distances` is not square
    /// - `distances` is not symmetric
    /// - diagonal entries are not zero
    /// - `num_clusters` is zero
    pub fn new(distances: Vec<Vec<u64>>, num_clusters: usize, diameter_bound: u64) -> Self {
        let n = distances.len();
        assert!(n > 0, "Clustering requires at least one element");
        assert!(num_clusters > 0, "num_clusters must be at least 1");
        for (i, row) in distances.iter().enumerate() {
            assert_eq!(
                row.len(),
                n,
                "Distance matrix must be square: row {i} has {} columns, expected {n}",
                row.len()
            );
            assert_eq!(
                distances[i][i], 0,
                "Diagonal entry distances[{i}][{i}] must be 0"
            );
        }
        for i in 0..n {
            for j in (i + 1)..n {
                assert_eq!(
                    distances[i][j], distances[j][i],
                    "Distance matrix must be symmetric: distances[{i}][{j}] = {} != distances[{j}][{i}] = {}",
                    distances[i][j], distances[j][i]
                );
            }
        }
        Self {
            distances,
            num_clusters,
            diameter_bound,
        }
    }

    /// Returns the distance matrix.
    pub fn distances(&self) -> &Vec<Vec<u64>> {
        &self.distances
    }

    /// Returns the number of elements.
    pub fn num_elements(&self) -> usize {
        self.distances.len()
    }

    /// Returns the maximum number of clusters K.
    pub fn num_clusters(&self) -> usize {
        self.num_clusters
    }

    /// Returns the diameter bound B.
    pub fn diameter_bound(&self) -> u64 {
        self.diameter_bound
    }
}

impl Problem for Clustering {
    const NAME: &'static str = "Clustering";
    type Value = crate::types::Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_clusters; self.num_elements()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            let n = self.num_elements();
            if config.len() != n {
                return crate::types::Or(false);
            }
            let k = self.num_clusters;
            // Check all assignments are valid cluster indices
            if config.iter().any(|&c| c >= k) {
                return crate::types::Or(false);
            }
            // For each non-empty cluster, check all pairwise distances ≤ B
            for c in 0..k {
                let members: Vec<usize> = config
                    .iter()
                    .enumerate()
                    .filter(|(_, &cl)| cl == c)
                    .map(|(i, _)| i)
                    .collect();
                for a in 0..members.len() {
                    for b in (a + 1)..members.len() {
                        if self.distances[members[a]][members[b]] > self.diameter_bound {
                            return crate::types::Or(false);
                        }
                    }
                }
            }
            true
        })
    }
}

crate::declare_variants! {
    default Clustering => "num_clusters^num_elements",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 6 elements in two tight groups {0,1,2} and {3,4,5}
    // Intra-group distance = 1, inter-group distance = 3
    // K=2, B=1
    let distances = vec![
        vec![0, 1, 1, 3, 3, 3],
        vec![1, 0, 1, 3, 3, 3],
        vec![1, 1, 0, 3, 3, 3],
        vec![3, 3, 3, 0, 1, 1],
        vec![3, 3, 3, 1, 0, 1],
        vec![3, 3, 3, 1, 1, 0],
    ];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "clustering",
        instance: Box::new(Clustering::new(distances, 2, 1)),
        optimal_config: vec![0, 0, 0, 1, 1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/clustering.rs"]
mod tests;
