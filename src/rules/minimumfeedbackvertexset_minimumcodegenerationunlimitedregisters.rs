//! Unit-weight feedback vertex set to unlimited-register code generation.
//!
//! The Aho–Johnson–Ullman reduction encodes cardinality, not arbitrary vertex
//! weights. Each source vertex has a start operation followed by its outgoing
//! arc operations. The separate start makes each right-use strictly later than
//! the originating start, including self-loops. An additional right-only leaf
//! makes every start binary without introducing another possible copy.

use crate::models::graph::MinimumFeedbackVertexSet;
use crate::models::misc::MinimumCodeGenerationUnlimitedRegisters;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::One;

/// Result of the unit-weight FVS to code-generation reduction.
#[derive(Debug, Clone)]
pub struct ReductionFVSToCodeGen {
    target: MinimumCodeGenerationUnlimitedRegisters,
    /// Configuration index of each source vertex's start operation.
    chain_start: Vec<usize>,
    /// Configuration indices of operations that right-use each original leaf.
    right_child_users: Vec<Vec<usize>>,
}

impl ReductionResult for ReductionFVSToCodeGen {
    type Source = MinimumFeedbackVertexSet<One>;
    type Target = MinimumCodeGenerationUnlimitedRegisters;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if value.0.is_none() {
            return Err(crate::rules::ExtractionError::invalid(
                "target order must be a permutation respecting expression dependencies",
            ));
        }
        Ok(self
            .chain_start
            .iter()
            .zip(&self.right_child_users)
            .map(|(&start, users)| {
                users
                    .iter()
                    .any(|&user| target_solution[user] > target_solution[start])
            })
            .collect())
    }
}

/// Count source leaves, the dummy leaf, and all start/arc operations before allocation.
fn code_generation_vertex_count(n: usize, m: usize) -> Result<usize, crate::rules::ReductionError> {
    n.checked_mul(2)
        .and_then(|count| count.checked_add(m))
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                MinimumFeedbackVertexSet<One>,
                MinimumCodeGenerationUnlimitedRegisters,
            >("counting code-generation start and arc nodes")
        })
}

#[reduction(
    transform = exact {
        num_vertices = "2 * num_vertices + num_arcs + 1",
    }
)]
impl ReduceTo<MinimumCodeGenerationUnlimitedRegisters> for MinimumFeedbackVertexSet<One> {
    type Result = ReductionFVSToCodeGen;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let m = self.graph().num_arcs();
        let num_vertices = code_generation_vertex_count(n, m)?;
        // All subsequent offsets are bounded by the checked total above.
        let first_internal = n + 1;
        let num_internal = n + m;
        let mut out_neighbors = vec![vec![]; n];
        for (u, v) in self.graph().arcs() {
            out_neighbors[u].push(v);
        }
        let mut left_arcs = Vec::with_capacity(num_internal);
        let mut right_arcs = Vec::with_capacity(num_internal);
        let mut chain_start = Vec::with_capacity(n);
        let mut right_child_users = vec![vec![]; n];
        let mut next_internal = first_internal;
        for (x, neighbors) in out_neighbors.iter().enumerate() {
            chain_start.push(next_internal - first_internal);
            left_arcs.push((next_internal, x));
            right_arcs.push((next_internal, n)); // dummy leaf is never overwritten
            next_internal += 1;
            for &neighbor in neighbors {
                left_arcs.push((next_internal, next_internal - 1));
                right_arcs.push((next_internal, neighbor));
                right_child_users[neighbor].push(next_internal - first_internal);
                next_internal += 1;
            }
        }
        debug_assert_eq!(next_internal, num_vertices);
        let target =
            MinimumCodeGenerationUnlimitedRegisters::new(num_vertices, left_arcs, right_arcs);
        Ok(ReductionFVSToCodeGen {
            target,
            chain_start,
            right_child_users,
        })
    }
}

#[cfg(any(test, feature = "example-db"))]
fn issue_example_source() -> MinimumFeedbackVertexSet<One> {
    use crate::topology::DirectedGraph;
    MinimumFeedbackVertexSet::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![One; 3],
    )
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumfeedbackvertexset_to_minimumcodegenerationunlimitedregisters",
        build: || {
            let source = issue_example_source();
            let reduction = ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&source)
                .expect("reduction should succeed");
            let target_config = BruteForce::new()
                .solve(reduction.target_problem())
                .expect("canonical target evaluation must succeed")
                .expect("canonical DAG has an evaluation order");
            let source_config = reduction.extract_solution(&target_config).unwrap();
            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::to_value(source_config)
                        .expect("solution serialization must succeed"),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfeedbackvertexset_minimumcodegenerationunlimitedregisters.rs"]
mod tests;
