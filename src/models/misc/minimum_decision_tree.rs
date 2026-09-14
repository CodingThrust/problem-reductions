//! Minimum Decision Tree problem implementation.
//!
//! Given a set of objects distinguished by binary tests, find a decision tree
//! that identifies each object with minimum total external path length
//! (sum of depths of all leaves).

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumDecisionTree",
        display_name: "Minimum Decision Tree",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Find decision tree identifying objects with minimum total path length",
        fields: MinimumDecisionTreeCreateSpec::FIELDS,
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
/// use problemreductions::{Problem, BruteForce};
///
/// let problem = MinimumDecisionTree::new(
///     vec![
///         vec![true, true, false, false],   // T0
///         vec![true, false, false, false],   // T1
///         vec![false, true, false, true],    // T2
///     ],
///     4,
///     3,
/// ).unwrap();
/// let solver = BruteForce::new();
/// let value = solver.solve(&problem).unwrap();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MinimumDecisionTreeCreateSpec")]
pub struct MinimumDecisionTree {
    /// Binary matrix: test_matrix[j][i] = true iff object i passes test j.
    test_matrix: Vec<Vec<bool>>,
    /// Number of objects.
    num_objects: usize,
    /// Number of tests.
    num_tests: usize,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MinimumDecisionTreeCreateSpec {
    /// Binary test matrix as JSON.
    #[create(codec = "json")]
    test_matrix: Vec<Vec<bool>>,
    /// Number of objects.
    num_objects: usize,
    /// Number of tests.
    num_tests: usize,
}

impl TryFrom<MinimumDecisionTreeCreateSpec> for MinimumDecisionTree {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MinimumDecisionTreeCreateSpec) -> Result<Self, Self::Error> {
        Self::new(spec.test_matrix, spec.num_objects, spec.num_tests)
    }
}

impl MinimumDecisionTree {
    /// Create a new MinimumDecisionTree problem.
    ///
    /// # Errors
    /// - If num_objects < 2 or num_tests < 1
    /// - If test_matrix dimensions don't match
    /// - If tests don't distinguish all object pairs
    pub fn new(
        test_matrix: Vec<Vec<bool>>,
        num_objects: usize,
        num_tests: usize,
    ) -> Result<Self, crate::registry::ConstructionError> {
        if !(num_objects >= 2) {
            return Err("Need at least 2 objects".into());
        }
        if num_tests == 0 {
            return Err("Need at least 1 test".into());
        }
        if test_matrix.len() != num_tests {
            return Err("test_matrix must have num_tests rows".into());
        }
        for (j, row) in test_matrix.iter().enumerate() {
            if row.len() != num_objects {
                return Err(format!("test_matrix[{j}] must have num_objects columns").into());
            }
        }
        // Check that every pair of objects is distinguished by at least one test
        for a in 0..num_objects {
            for b in (a + 1)..num_objects {
                let distinguished = (0..num_tests).any(|j| test_matrix[j][a] != test_matrix[j][b]);
                if !(distinguished) {
                    return Err(
                        format!("Objects {a} and {b} are not distinguished by any test").into(),
                    );
                }
            }
        }
        Ok(Self {
            test_matrix,
            num_objects,
            num_tests,
        })
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
    fn num_tree_slots(&self) -> Result<usize, crate::traits::EvaluationError> {
        self.num_objects
            .checked_sub(1)
            .and_then(|depth| u32::try_from(depth).ok())
            .and_then(|depth| 1usize.checked_shl(depth))
            .map(|leaves| leaves - 1)
            .ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "representing the decision-tree witness slots".into(),
                )
            })
    }

    /// Sentinel value meaning "this node is a leaf".
    fn leaf_sentinel(&self) -> usize {
        self.num_tests
    }

    /// Simulate the decision tree for all objects and return total external path length,
    /// or None if the tree is invalid (doesn't identify all objects uniquely).
    fn simulate(&self, config: &[usize]) -> Result<Option<i64>, crate::traits::EvaluationError> {
        let sentinel = self.leaf_sentinel();
        let max_slots = self.num_tree_slots()?;
        let mut seen_leaves = std::collections::HashSet::new();
        let mut total_depth = 0_i64;

        for obj in 0..self.num_objects {
            let mut node = 0usize;
            let mut depth = 0usize;

            loop {
                if node >= max_slots || config[node] == sentinel {
                    // Two objects at same leaf — invalid
                    if !seen_leaves.insert(node) {
                        return Ok(None);
                    }
                    let depth = i64::try_from(depth).map_err(|_| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "converting decision-tree depth to i64".to_string(),
                        )
                    })?;
                    total_depth = total_depth.checked_add(depth).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing decision-tree external path length".to_string(),
                        )
                    })?;
                    break;
                }

                let test_idx = config[node];
                debug_assert!(test_idx < self.num_tests);

                let result = self.test_matrix[test_idx][obj];
                node = if result { 2 * node + 2 } else { 2 * node + 1 };
                depth += 1;

                if depth > self.num_objects {
                    return Ok(None);
                }
            }
        }

        Ok(Some(total_depth))
    }
}

impl Problem for MinimumDecisionTree {
    const NAME: &'static str = "MinimumDecisionTree";
    type Solution = Vec<usize>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_objects", num_objects), ("num_tests", num_tests),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if config.len() != self.num_tree_slots()? {
                return Err(crate::traits::EvaluationError::InvalidConfiguration(
                    "decision-tree encoding length does not match the instance".into(),
                ));
            }
            Min(self.simulate(config)?)
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for MinimumDecisionTree {
    fn num_variables(&self) -> Result<usize, crate::solvers::SolveError> {
        Ok(self.num_tree_slots()?)
    }

    fn dimension(&self, _variable: usize) -> Result<usize, crate::solvers::SolveError> {
        (self.num_tests).checked_add(1usize).ok_or_else(|| {
            crate::solvers::SolveError::IntegerOverflow("computing a coordinate cardinality".into())
        })
    }
}

crate::declare_variants! {
    default MinimumDecisionTree => "num_tests^num_objects" create MinimumDecisionTreeCreateSpec,
}

crate::register_brute_force! {
    MinimumDecisionTree,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_decision_tree",
        instance: Box::new(
            MinimumDecisionTree::new(
                vec![
                    vec![true, true, false, false],
                    vec![true, false, false, false],
                    vec![false, true, false, true],
                ],
                4,
                3,
            )
            .unwrap(),
        ),
        // T0 at root, T2 left, T1 right, rest are leaves (sentinel=3)
        optimal_config: serde_json::json!(vec![0, 2, 1, 3, 3, 3, 3]),
        optimal_value: serde_json::json!(8),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_decision_tree.rs"]
mod tests;
