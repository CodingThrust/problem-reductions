//! Minimum Decision Tree problem implementation.
//!
//! Given a set of objects distinguished by binary tests, find a decision tree
//! that identifies each object with minimum total external path length
//! (sum of depths of all leaves).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDecisionTree",
        display_name: "Minimum Decision Tree",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find decision tree identifying objects with minimum total path length",
        fields: &[
            FieldInfo { name: "test_matrix", type_name: "Vec<Vec<bool>>", description: "Binary matrix: test_matrix[j][i] = object i passes test j" },
            FieldInfo { name: "num_objects", type_name: "usize", description: "Number of objects to identify" },
            FieldInfo { name: "num_tests", type_name: "usize", description: "Number of available binary tests" },
        ],
    }
}

/// Minimum Decision Tree problem.
///
/// Given objects distinguished by binary tests, find a decision tree
/// minimizing the total external path length (sum of leaf depths).
///
/// The configuration encodes a flattened complete binary tree of depth
/// `num_objects - 1`. Each internal node stores either a test index
/// (0..num_tests-1) or a sentinel value `num_tests` meaning "leaf".
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumDecisionTree;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let problem = MinimumDecisionTree::new(
///     vec![
///         vec![true, true, false, false],   // T0
///         vec![true, false, false, false],   // T1
///         vec![false, true, false, true],    // T2
///     ],
///     4,
///     3,
/// );
/// let solver = BruteForce::new();
/// let value = solver.solve(&problem);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumDecisionTree {
    /// Binary matrix: test_matrix[j][i] = true iff object i passes test j.
    test_matrix: Vec<Vec<bool>>,
    /// Number of objects.
    num_objects: usize,
    /// Number of tests.
    num_tests: usize,
}

impl MinimumDecisionTree {
    /// Create a new MinimumDecisionTree problem.
    ///
    /// # Panics
    /// - If num_objects < 2 or num_tests < 1
    /// - If test_matrix dimensions don't match
    /// - If tests don't distinguish all object pairs
    pub fn new(test_matrix: Vec<Vec<bool>>, num_objects: usize, num_tests: usize) -> Self {
        assert!(num_objects >= 2, "Need at least 2 objects");
        assert!(num_tests >= 1, "Need at least 1 test");
        assert_eq!(
            test_matrix.len(),
            num_tests,
            "test_matrix must have num_tests rows"
        );
        for (j, row) in test_matrix.iter().enumerate() {
            assert_eq!(
                row.len(),
                num_objects,
                "test_matrix[{j}] must have num_objects columns"
            );
        }
        // Check that every pair of objects is distinguished by at least one test
        for a in 0..num_objects {
            for b in (a + 1)..num_objects {
                let distinguished = (0..num_tests).any(|j| test_matrix[j][a] != test_matrix[j][b]);
                assert!(
                    distinguished,
                    "Objects {a} and {b} are not distinguished by any test"
                );
            }
        }
        Self {
            test_matrix,
            num_objects,
            num_tests,
        }
    }

    /// Get the number of objects.
    pub fn num_objects(&self) -> usize {
        self.num_objects
    }

    /// Get the number of tests.
    pub fn num_tests(&self) -> usize {
        self.num_tests
    }

    /// Get the test matrix.
    pub fn test_matrix(&self) -> &[Vec<bool>] {
        &self.test_matrix
    }

    /// Number of internal node slots in the flattened complete binary tree.
    fn num_tree_slots(&self) -> usize {
        (1usize << (self.num_objects - 1)) - 1
    }

    /// Sentinel value meaning "this node is a leaf".
    fn leaf_sentinel(&self) -> usize {
        self.num_tests
    }

    /// Simulate the decision tree for all objects and return total external path length,
    /// or None if the tree is invalid (doesn't identify all objects uniquely).
    fn simulate(&self, config: &[usize]) -> Option<usize> {
        let sentinel = self.leaf_sentinel();
        let max_slots = self.num_tree_slots();
        let mut leaf_assignments: Vec<Option<usize>> = vec![None; max_slots + self.num_objects];
        // Track (node_index, depth) where each object lands
        let mut object_depths = vec![0usize; self.num_objects];
        let mut object_placed = vec![false; self.num_objects];

        for obj in 0..self.num_objects {
            let mut node = 0usize;
            let mut depth = 0usize;

            loop {
                if node >= max_slots || config[node] == sentinel {
                    // This is a leaf — place object here
                    // Use node index as leaf identifier
                    let leaf_id = node;
                    if leaf_id < leaf_assignments.len() {
                        if let Some(existing) = leaf_assignments[leaf_id] {
                            if existing != obj {
                                // Two objects at same leaf — invalid
                                return None;
                            }
                        }
                        leaf_assignments[leaf_id] = Some(obj);
                    }
                    object_depths[obj] = depth;
                    object_placed[obj] = true;
                    break;
                }

                let test_idx = config[node];
                if test_idx >= self.num_tests {
                    return None; // Invalid test index (but not sentinel — shouldn't happen)
                }

                // Apply test: false (0) → left child, true (1) → right child
                let result = self.test_matrix[test_idx][obj];
                let left = 2 * node + 1;
                let right = 2 * node + 2;
                node = if result { right } else { left };
                depth += 1;

                // Safety: prevent infinite loops on malformed configs
                if depth > self.num_objects {
                    return None;
                }
            }
        }

        // Check all objects were placed
        if !object_placed.iter().all(|&p| p) {
            return None;
        }

        // Check all objects reached unique leaves
        let mut seen_leaves = std::collections::HashSet::new();
        for obj in 0..self.num_objects {
            // Re-traverse to get leaf node
            let mut node = 0;
            loop {
                if node >= max_slots || config[node] == sentinel {
                    if !seen_leaves.insert(node) {
                        return None; // Duplicate leaf
                    }
                    break;
                }
                let result = self.test_matrix[config[node]][obj];
                node = if result { 2 * node + 2 } else { 2 * node + 1 };
            }
        }

        Some(object_depths.iter().sum())
    }
}

impl Problem for MinimumDecisionTree {
    const NAME: &'static str = "MinimumDecisionTree";
    type Value = Min<usize>;

    fn dims(&self) -> Vec<usize> {
        // Each internal node can hold test 0..num_tests-1 or sentinel (leaf)
        vec![self.num_tests + 1; self.num_tree_slots()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != self.num_tree_slots() {
            return Min(None);
        }
        Min(self.simulate(config))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default MinimumDecisionTree => "num_tests^num_objects",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_decision_tree",
        instance: Box::new(MinimumDecisionTree::new(
            vec![
                vec![true, true, false, false],
                vec![true, false, false, false],
                vec![false, true, false, true],
            ],
            4,
            3,
        )),
        // T0 at root, T2 left, T1 right, rest are leaves (sentinel=3)
        optimal_config: vec![0, 2, 1, 3, 3, 3, 3],
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_decision_tree.rs"]
mod tests;
