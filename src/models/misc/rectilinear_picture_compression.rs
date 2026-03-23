//! Rectilinear Picture Compression problem implementation.
//!
//! Given an m x n binary matrix M and a nonnegative integer K, determine whether
//! there exists a collection of at most K axis-aligned all-1 rectangles that
//! covers precisely the 1-entries of M. Each rectangle (r1, c1, r2, c2) with
//! r1 <= r2, c1 <= c2 covers entries M[i][j] for r1 <= i <= r2, c1 <= j <= c2,
//! and every covered entry must be 1.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

type Rectangle = (usize, usize, usize, usize);

inventory::submit! {
    ProblemSchemaEntry {
        name: "RectilinearPictureCompression",
        display_name: "Rectilinear Picture Compression",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Cover all 1-entries of a binary matrix with at most K axis-aligned all-1 rectangles",
        fields: &[
            FieldInfo { name: "matrix", type_name: "Vec<Vec<bool>>", description: "m x n binary matrix" },
            FieldInfo { name: "bound", type_name: "i64", description: "Maximum number of rectangles allowed" },
        ],
    }
}

/// The Rectilinear Picture Compression problem.
///
/// Given an m x n binary matrix M and a nonnegative integer K, determine whether
/// there exists a collection of at most K axis-aligned all-1 rectangles that
/// covers precisely the 1-entries of M.
///
/// # Representation
///
/// The configuration space consists of the maximal all-1 rectangles in the
/// matrix. Each variable is binary: 1 if the rectangle is selected, 0 otherwise.
/// The problem is satisfiable iff the selected rectangles number at most K and
/// their union covers all 1-entries.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::RectilinearPictureCompression;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let matrix = vec![
///     vec![true, true, false, false],
///     vec![true, true, false, false],
///     vec![false, false, true, true],
///     vec![false, false, true, true],
/// ];
/// let problem = RectilinearPictureCompression::new(matrix, 2);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RectilinearPictureCompression {
    matrix: Vec<Vec<bool>>,
    bound: i64,
    #[serde(skip)]
    maximal_rects: Vec<Rectangle>,
}

impl<'de> Deserialize<'de> for RectilinearPictureCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            matrix: Vec<Vec<bool>>,
            bound: i64,
        }
        let inner = Inner::deserialize(deserializer)?;
        Ok(Self::new(inner.matrix, inner.bound))
    }
}

impl RectilinearPictureCompression {
    /// Create a new RectilinearPictureCompression instance.
    ///
    /// # Panics
    ///
    /// Panics if `matrix` is empty or has inconsistent row lengths.
    pub fn new(matrix: Vec<Vec<bool>>, bound: i64) -> Self {
        assert!(!matrix.is_empty(), "Matrix must not be empty");
        let cols = matrix[0].len();
        assert!(cols > 0, "Matrix must have at least one column");
        assert!(
            matrix.iter().all(|row| row.len() == cols),
            "All rows must have the same length"
        );
        let mut instance = Self {
            matrix,
            bound,
            maximal_rects: Vec::new(),
        };
        instance.maximal_rects = instance.compute_maximal_rectangles();
        instance
    }

    /// Returns the number of rows in the matrix.
    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    /// Returns the number of columns in the matrix.
    pub fn num_cols(&self) -> usize {
        self.matrix[0].len()
    }

    /// Returns the bound K.
    pub fn bound(&self) -> i64 {
        self.bound
    }

    /// Returns a reference to the binary matrix.
    pub fn matrix(&self) -> &[Vec<bool>] {
        &self.matrix
    }

    /// Returns the precomputed maximal all-1 sub-rectangles.
    ///
    /// Each rectangle is `(r1, c1, r2, c2)` covering rows `r1..=r2` and
    /// columns `c1..=c2`.
    pub fn maximal_rectangles(&self) -> &[Rectangle] {
        &self.maximal_rects
    }

    fn build_prefix_sum(&self) -> Vec<Vec<usize>> {
        let m = self.num_rows();
        let n = self.num_cols();
        let mut prefix_sum = vec![vec![0; n + 1]; m + 1];

        for r in 0..m {
            let mut row_sum = 0;
            for c in 0..n {
                row_sum += usize::from(self.matrix[r][c]);
                prefix_sum[r + 1][c + 1] = prefix_sum[r][c + 1] + row_sum;
            }
        }

        prefix_sum
    }

    fn range_is_all_ones(
        prefix_sum: &[Vec<usize>],
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize,
    ) -> bool {
        let area = (r2 - r1 + 1) * (c2 - c1 + 1);
        let sum = prefix_sum[r2 + 1][c2 + 1] + prefix_sum[r1][c1]
            - prefix_sum[r1][c2 + 1]
            - prefix_sum[r2 + 1][c1];
        sum == area
    }

    /// Enumerate all maximal all-1 sub-rectangles in the matrix.
    ///
    /// A rectangle is maximal if it cannot be extended one step left, right,
    /// up, or down while remaining all-1. The result is sorted lexicographically.
    fn compute_maximal_rectangles(&self) -> Vec<Rectangle> {
        let m = self.num_rows();
        let n = self.num_cols();

        // Step 1: Enumerate right-maximal candidate rectangles by fixing
        // (r1, c1, r2) and taking the widest all-1 prefix common to rows r1..=r2.
        let mut candidates = Vec::new();
        for r1 in 0..m {
            for c1 in 0..n {
                if !self.matrix[r1][c1] {
                    continue;
                }
                // Find the rightmost column from c1 that is all-1 in row r1.
                let mut c_max = n;
                for c in c1..n {
                    if !self.matrix[r1][c] {
                        c_max = c;
                        break;
                    }
                }
                // Extend downward row by row, narrowing column range.
                let mut c_end = c_max; // exclusive upper bound on columns
                for r2 in r1..m {
                    // Narrow c_end based on row r2.
                    let mut new_c_end = c1;
                    for c in c1..c_end {
                        if self.matrix[r2][c] {
                            new_c_end = c + 1;
                        } else {
                            break;
                        }
                    }
                    if new_c_end <= c1 {
                        break;
                    }
                    c_end = new_c_end;
                    candidates.push((r1, c1, r2, c_end - 1));
                }
            }
        }

        // Step 2: Remove duplicates.
        candidates.sort();
        candidates.dedup();

        // Step 3: Filter to keep only rectangles that cannot be extended in
        // any cardinal direction. A 2D prefix sum makes each extension check O(1).
        let prefix_sum = self.build_prefix_sum();
        candidates
            .into_iter()
            .filter(|&(r1, c1, r2, c2)| {
                let can_extend_left =
                    c1 > 0 && Self::range_is_all_ones(&prefix_sum, r1, c1 - 1, r2, c1 - 1);
                let can_extend_right =
                    c2 + 1 < n && Self::range_is_all_ones(&prefix_sum, r1, c2 + 1, r2, c2 + 1);
                let can_extend_up =
                    r1 > 0 && Self::range_is_all_ones(&prefix_sum, r1 - 1, c1, r1 - 1, c2);
                let can_extend_down =
                    r2 + 1 < m && Self::range_is_all_ones(&prefix_sum, r2 + 1, c1, r2 + 1, c2);

                !(can_extend_left || can_extend_right || can_extend_up || can_extend_down)
            })
            .collect()
    }
}

impl Problem for RectilinearPictureCompression {
    const NAME: &'static str = "RectilinearPictureCompression";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.maximal_rects.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        let rects = &self.maximal_rects;
        if config.len() != rects.len() {
            return false;
        }
        if config.iter().any(|&v| v >= 2) {
            return false;
        }

        // Count selected rectangles.
        let selected_count: usize = config.iter().sum();
        if (selected_count as i64) > self.bound {
            return false;
        }

        // Check that all 1-entries are covered.
        let m = self.num_rows();
        let n = self.num_cols();
        let mut covered = vec![vec![false; n]; m];
        for (i, &x) in config.iter().enumerate() {
            if x == 1 {
                let (r1, c1, r2, c2) = rects[i];
                for row in &mut covered[r1..=r2] {
                    for cell in &mut row[c1..=c2] {
                        *cell = true;
                    }
                }
            }
        }

        for (row_m, row_c) in self.matrix.iter().zip(covered.iter()) {
            for (&entry, &cov) in row_m.iter().zip(row_c.iter()) {
                if entry && !cov {
                    return false;
                }
            }
        }

        true
    }
}

impl SatisfactionProblem for RectilinearPictureCompression {}

crate::declare_variants! {
    default sat RectilinearPictureCompression => "2^(num_rows * num_cols)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "rectilinear_picture_compression",
        // Config: select both maximal rectangles (the two 2x2 blocks).
        // The maximal rectangles for this matrix are exactly:
        // (0,0,1,1) and (2,2,3,3), so config [1,1] selects both.
        instance: Box::new(RectilinearPictureCompression::new(
            vec![
                vec![true, true, false, false],
                vec![true, true, false, false],
                vec![false, false, true, true],
                vec![false, false, true, true],
            ],
            2,
        )),
        optimal_config: vec![1, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/rectilinear_picture_compression.rs"]
mod tests;
