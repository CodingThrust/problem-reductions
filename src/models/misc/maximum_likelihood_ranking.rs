//! Maximum Likelihood Ranking problem implementation.
//!
//! Given an n x n antisymmetric comparison matrix A where a_ij + a_ji = c
//! (constant) for every pair and a_ii = 0, find a permutation pi minimizing
//! the total disagreement cost: sum over all position pairs (i > j) of
//! a_{pi(i), pi(j)}.  Entries may be negative (e.g. c = 0 gives a
//! skew-symmetric matrix).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumLikelihoodRanking",
        display_name: "Maximum Likelihood Ranking",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a ranking minimizing total pairwise disagreement cost",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<i32>>", description: "Antisymmetric comparison matrix A (a_ij + a_ji = c, a_ii = 0)" },
        ],
    }
}

/// The Maximum Likelihood Ranking problem.
///
/// Given an n x n antisymmetric comparison matrix A where a_ij + a_ji = c
/// (constant) for every pair and a_ii = 0, find a permutation pi that
/// minimizes the total disagreement cost: sum_{i > j} a_{pi(i), pi(j)}.
/// Entries may be negative (e.g. c = 0 gives a skew-symmetric matrix).
///
/// Each item is assigned a rank position (0-indexed). The configuration
/// maps item -> rank: `config[item] = rank`. The permutation pi maps
/// rank -> item (the inverse of config).
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MaximumLikelihoodRanking;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let matrix = vec![
///     vec![0, 4, 3, 5],
///     vec![1, 0, 4, 3],
///     vec![2, 1, 0, 4],
///     vec![0, 2, 1, 0],
/// ];
/// let problem = MaximumLikelihoodRanking::new(matrix);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumLikelihoodRanking {
    matrix: Vec<Vec<i32>>,
}

impl MaximumLikelihoodRanking {
    /// Create a new MaximumLikelihoodRanking instance.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if any diagonal element is nonzero,
    /// or if the pairwise sums `a_ij + a_ji` are not the same constant for
    /// all `i != j`.
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        for (i, row) in matrix.iter().enumerate() {
            assert_eq!(
                row.len(),
                n,
                "matrix must be square: row {i} has length {} but expected {n}",
                row.len()
            );
            assert_eq!(
                row[i], 0,
                "diagonal entries must be zero: matrix[{i}][{i}] = {}",
                row[i]
            );
        }

        let mut comparison_count = None;
        for (i, row) in matrix.iter().enumerate() {
            for (j, &entry) in row.iter().enumerate().skip(i + 1) {
                let pair_sum = entry + matrix[j][i];
                match comparison_count {
                    None => comparison_count = Some(pair_sum),
                    Some(expected) => assert_eq!(
                        pair_sum,
                        expected,
                        "all off-diagonal pairs must have the same comparison count: matrix[{i}][{j}] + matrix[{j}][{i}] = {pair_sum}, expected {expected}"
                    ),
                }
            }
        }

        Self { matrix }
    }

    /// Returns the comparison matrix.
    pub fn matrix(&self) -> &Vec<Vec<i32>> {
        &self.matrix
    }

    /// Returns the number of items to rank.
    pub fn num_items(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the constant pairwise comparison count `c`.
    pub fn comparison_count(&self) -> i32 {
        if self.matrix.len() < 2 {
            0
        } else {
            self.matrix[0][1] + self.matrix[1][0]
        }
    }
}

impl Problem for MaximumLikelihoodRanking {
    const NAME: &'static str = "MaximumLikelihoodRanking";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.num_items();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i64> {
        let n = self.num_items();

        // Validate config length
        if config.len() != n {
            return Min(None);
        }

        // Validate permutation: all values must be distinct and in 0..n
        let mut seen = vec![false; n];
        for &rank in config {
            if rank >= n || seen[rank] {
                return Min(None);
            }
            seen[rank] = true;
        }

        // config[item] = rank position of item
        // Disagreement cost: for all pairs of items (a, b) where a is
        // ranked AFTER b (config[a] > config[b]), add matrix[a][b].
        let mut cost: i64 = 0;
        for a in 0..n {
            for b in 0..n {
                if a != b && config[a] > config[b] {
                    cost += self.matrix[a][b] as i64;
                }
            }
        }

        Min(Some(cost))
    }
}

crate::declare_variants! {
    default MaximumLikelihoodRanking => "num_items * num_items * 2^num_items",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 4 items with comparison matrix.
    // Optimal ranking: [0, 1, 2, 3] (identity) gives cost 7.
    // Let's verify: items ranked in order 0,1,2,3.
    // Disagreement = sum over (a,b) where config[a] > config[b] of matrix[a][b]
    // = matrix[1][0] + matrix[2][0] + matrix[2][1] + matrix[3][0] + matrix[3][1] + matrix[3][2]
    // = 1 + 2 + 1 + 0 + 2 + 1 = 7
    let matrix = vec![
        vec![0, 4, 3, 5],
        vec![1, 0, 4, 3],
        vec![2, 1, 0, 4],
        vec![0, 2, 1, 0],
    ];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_likelihood_ranking",
        instance: Box::new(MaximumLikelihoodRanking::new(matrix)),
        optimal_config: vec![0, 1, 2, 3],
        optimal_value: serde_json::json!(7),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/maximum_likelihood_ranking.rs"]
mod tests;
