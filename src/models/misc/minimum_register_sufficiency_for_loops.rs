//! Minimum Register Sufficiency for Loops problem implementation.
//!
//! Given a loop of length N and a set of variables, each active during a
//! contiguous circular arc of timesteps, assign registers to variables
//! minimizing the number of distinct registers used, such that no two
//! conflicting (overlapping) variables share the same register.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumRegisterSufficiencyForLoops",
        display_name: "Minimum Register Sufficiency for Loops",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Assign registers to loop variables minimizing register count, no two conflicting variables share a register",
        fields: &[
            FieldInfo { name: "loop_length", type_name: "usize", description: "Loop length N (number of timesteps)" },
            FieldInfo { name: "variables", type_name: "Vec<(usize, usize)>", description: "Variables as (start_time, duration) circular arcs" },
        ],
    }
}

/// The Minimum Register Sufficiency for Loops problem.
///
/// Given a loop of length N (representing N timesteps arranged in a circle)
/// and a set of variables, each active during a contiguous circular arc of
/// timesteps specified by (start_time, duration), assign a register index
/// to each variable such that:
/// - No two variables with overlapping circular arcs share the same register
/// - The number of distinct registers used is minimized
///
/// This is equivalent to the circular arc graph coloring problem, where each
/// variable corresponds to a circular arc and registers correspond to colors.
///
/// # Representation
///
/// Each variable is assigned a register index from `{0, ..., n-1}` where n is
/// the number of variables. The configuration `config[i]` gives the register
/// assigned to variable i.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumRegisterSufficiencyForLoops;
/// use problemreductions::{Problem, Solver, BruteForce, Min};
///
/// // 3 variables on a loop of length 6, all pairs conflict
/// let problem = MinimumRegisterSufficiencyForLoops::new(
///     6,
///     vec![(0, 3), (2, 3), (4, 3)],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// let val = problem.evaluate(&solution.unwrap());
/// assert_eq!(val, Min(Some(3)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumRegisterSufficiencyForLoops {
    /// Loop length N (number of timesteps in the circular loop).
    loop_length: usize,
    /// Variables as (start_time, duration) pairs representing circular arcs.
    variables: Vec<(usize, usize)>,
}

impl MinimumRegisterSufficiencyForLoops {
    /// Create a new Minimum Register Sufficiency for Loops instance.
    ///
    /// # Panics
    ///
    /// Panics if `loop_length` is zero, if any duration is zero or exceeds
    /// `loop_length`, or if any `start_time >= loop_length`.
    pub fn new(loop_length: usize, variables: Vec<(usize, usize)>) -> Self {
        assert!(loop_length > 0, "loop_length must be positive");
        for (i, &(start, dur)) in variables.iter().enumerate() {
            assert!(
                start < loop_length,
                "Variable {} start_time {} >= loop_length {}",
                i,
                start,
                loop_length
            );
            assert!(
                dur > 0 && dur <= loop_length,
                "Variable {} duration {} must be in [1, {}]",
                i,
                dur,
                loop_length
            );
        }
        Self {
            loop_length,
            variables,
        }
    }

    /// Get the loop length N.
    pub fn loop_length(&self) -> usize {
        self.loop_length
    }

    /// Get the number of variables.
    pub fn num_variables(&self) -> usize {
        self.variables.len()
    }

    /// Get the variables as (start_time, duration) pairs.
    pub fn variables(&self) -> &[(usize, usize)] {
        &self.variables
    }

    /// Check if two circular arcs overlap.
    ///
    /// Arc [s, s+l) mod N covers timesteps {s, s+1, ..., s+l-1} mod N.
    /// Two arcs overlap iff their covered timestep sets intersect.
    fn arcs_overlap(s1: usize, l1: usize, s2: usize, l2: usize, n: usize) -> bool {
        // Use the modular distance check:
        // Timestep t is in arc [s, s+l) mod N iff (t - s) mod N < l
        // Two arcs are disjoint iff arc2 fits entirely in the gap of arc1
        // or arc1 fits entirely in the gap of arc2.
        // Gap of arc [s, s+l) is [(s+l) mod N, s) with length N-l.

        // If either arc covers the entire loop, they always overlap (if both non-empty)
        if l1 == n || l2 == n {
            return true;
        }

        // Check if arc2 is entirely in the gap of arc1.
        // Gap of arc1 starts at (s1+l1) % n and has length n-l1.
        // Arc2 fits in this gap if the "gap distance" of s2 from gap_start
        // plus l2 <= n - l1.
        let gap1_start = (s1 + l1) % n;
        let dist_s2_in_gap1 = (s2 + n - gap1_start) % n;
        if dist_s2_in_gap1 + l2 <= n - l1 {
            return false;
        }

        // Check if arc1 is entirely in the gap of arc2.
        let gap2_start = (s2 + l2) % n;
        let dist_s1_in_gap2 = (s1 + n - gap2_start) % n;
        if dist_s1_in_gap2 + l1 <= n - l2 {
            return false;
        }

        true
    }
}

impl Problem for MinimumRegisterSufficiencyForLoops {
    const NAME: &'static str = "MinimumRegisterSufficiencyForLoops";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let n = self.variables.len();
        vec![n; n]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        let n = self.variables.len();
        if config.len() != n {
            return Min(None);
        }
        // Check all register indices are in valid range
        if config.iter().any(|&r| r >= n) {
            return Min(None);
        }

        // Check for conflicts: no two overlapping variables share a register
        for i in 0..n {
            for j in (i + 1)..n {
                if config[i] == config[j] {
                    let (s1, l1) = self.variables[i];
                    let (s2, l2) = self.variables[j];
                    if Self::arcs_overlap(s1, l1, s2, l2, self.loop_length) {
                        return Min(None);
                    }
                }
            }
        }

        // Count distinct registers used
        let mut used = vec![false; n];
        for &r in config {
            used[r] = true;
        }
        let count = used.iter().filter(|&&u| u).count();
        Min(Some(count))
    }
}

crate::declare_variants! {
    default MinimumRegisterSufficiencyForLoops => "num_variables ^ num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_register_sufficiency_for_loops",
        // 3 variables on a loop of length 6, all pairs conflict (K3)
        // Optimal: 3 registers (chromatic number of K3)
        instance: Box::new(MinimumRegisterSufficiencyForLoops::new(
            6,
            vec![(0, 3), (2, 3), (4, 3)],
        )),
        optimal_config: vec![0, 1, 2],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_register_sufficiency_for_loops.rs"]
mod tests;
