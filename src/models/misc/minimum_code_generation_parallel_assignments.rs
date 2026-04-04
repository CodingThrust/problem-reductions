//! Minimum Code Generation for Parallel Assignments problem implementation.
//!
//! Given a set of simultaneous variable assignments, find an execution ordering
//! (permutation) that minimizes the number of backward dependencies -- cases where
//! a variable is overwritten before a later assignment reads its old value.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCodeGenerationParallelAssignments",
        display_name: "Minimum Code Generation (Parallel Assignments)",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find an ordering of parallel assignments minimizing backward dependencies",
        fields: &[
            FieldInfo { name: "num_variables", type_name: "usize", description: "Number of variables" },
            FieldInfo { name: "assignments", type_name: "Vec<(usize, Vec<usize>)>", description: "Each assignment (target_var, read_vars)" },
        ],
    }
}

/// The Minimum Code Generation for Parallel Assignments problem.
///
/// Given a set V of variables and a collection of assignments A_i: "v_i <- op(B_i)"
/// where v_i is the target variable and B_i is the set of variables read,
/// find a permutation of the assignments that minimizes the number of backward
/// dependencies. A backward dependency occurs when assignment pi(i) writes
/// variable v and assignment pi(j) (j > i) reads v.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumCodeGenerationParallelAssignments;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 4 variables, 4 assignments:
/// // A_0: a <- op(b, c)   -> (0, [1, 2])
/// // A_1: b <- op(a)      -> (1, [0])
/// // A_2: c <- op(d)      -> (2, [3])
/// // A_3: d <- op(b, c)   -> (3, [1, 2])
/// let assignments = vec![
///     (0, vec![1, 2]),
///     (1, vec![0]),
///     (2, vec![3]),
///     (3, vec![1, 2]),
/// ];
/// let problem = MinimumCodeGenerationParallelAssignments::new(4, assignments);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCodeGenerationParallelAssignments {
    num_variables: usize,
    assignments: Vec<(usize, Vec<usize>)>,
}

impl MinimumCodeGenerationParallelAssignments {
    /// Create a new MinimumCodeGenerationParallelAssignments instance.
    ///
    /// # Panics
    /// Panics if any target variable or read variable index is >= num_variables.
    pub fn new(num_variables: usize, assignments: Vec<(usize, Vec<usize>)>) -> Self {
        for (i, (target, reads)) in assignments.iter().enumerate() {
            assert!(
                *target < num_variables,
                "assignment {i}: target variable {target} >= num_variables {num_variables}"
            );
            for &r in reads {
                assert!(
                    r < num_variables,
                    "assignment {i}: read variable {r} >= num_variables {num_variables}"
                );
            }
        }
        Self {
            num_variables,
            assignments,
        }
    }

    /// Returns the number of variables.
    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    /// Returns the number of assignments.
    pub fn num_assignments(&self) -> usize {
        self.assignments.len()
    }

    /// Returns the assignments.
    pub fn assignments(&self) -> &[(usize, Vec<usize>)] {
        &self.assignments
    }
}

impl Problem for MinimumCodeGenerationParallelAssignments {
    const NAME: &'static str = "MinimumCodeGenerationParallelAssignments";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let m = self.num_assignments();
        vec![m; m]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let m = self.num_assignments();

        // Validate config length
        if config.len() != m {
            return Min(None);
        }

        // Validate permutation: all values must be distinct and in 0..m
        let mut seen = vec![false; m];
        for &pos in config {
            if pos >= m || seen[pos] {
                return Min(None);
            }
            seen[pos] = true;
        }

        // config[i] = position of assignment i in execution order
        // Build execution order: order[pos] = assignment index
        let mut order = vec![0usize; m];
        for (assignment_idx, &pos) in config.iter().enumerate() {
            order[pos] = assignment_idx;
        }

        // Count backward dependencies: for each pair (i, j) where i < j
        // (i executes before j), check if the target variable of order[i]
        // is in the read set of order[j]
        let mut count = 0usize;
        for (i, &earlier) in order.iter().enumerate() {
            let (target_var, _) = &self.assignments[earlier];
            for &later in &order[(i + 1)..] {
                let (_, read_vars) = &self.assignments[later];
                if read_vars.contains(target_var) {
                    count += 1;
                }
            }
        }

        Min(Some(count))
    }
}

crate::declare_variants! {
    default MinimumCodeGenerationParallelAssignments => "2^num_assignments",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 4 variables, 4 assignments:
    // A_0: a <- op(b, c) -> (0, [1, 2])
    // A_1: b <- op(a)    -> (1, [0])
    // A_2: c <- op(d)    -> (2, [3])
    // A_3: d <- op(b, c) -> (3, [1, 2])
    //
    // Optimal ordering: config [0, 3, 1, 2] means
    // A_0 at position 0, A_1 at position 3, A_2 at position 1, A_3 at position 2
    // Order: (A_0, A_2, A_3, A_1)
    // Backward deps: A_0 writes a, A_1 reads a (later) -> 1
    //                A_2 writes c, A_3 reads c (later) -> 1
    //                Total: 2
    let assignments = vec![(0, vec![1, 2]), (1, vec![0]), (2, vec![3]), (3, vec![1, 2])];
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_code_generation_parallel_assignments",
        instance: Box::new(MinimumCodeGenerationParallelAssignments::new(
            4,
            assignments,
        )),
        optimal_config: vec![0, 3, 1, 2],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_code_generation_parallel_assignments.rs"]
mod tests;
