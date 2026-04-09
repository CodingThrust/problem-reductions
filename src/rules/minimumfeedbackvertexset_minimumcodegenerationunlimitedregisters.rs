//! Reduction from MinimumFeedbackVertexSet to MinimumCodeGenerationUnlimitedRegisters.
//!
//! The Aho-Johnson-Ullman chain gadget construction: for each source vertex x
//! with outgoing edges (x,y₁),...,(x,y_d), create a chain of internal nodes
//! x¹,...,x^d where xⁱ has left child x^{i-1} and right child y_i⁰ (the leaf).
//! Copies in an optimal program correspond exactly to a minimum feedback vertex set.

use crate::models::graph::MinimumFeedbackVertexSet;
use crate::models::misc::MinimumCodeGenerationUnlimitedRegisters;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumFeedbackVertexSet to MinimumCodeGenerationUnlimitedRegisters.
#[derive(Debug, Clone)]
pub struct ReductionFVSToCodeGen {
    target: MinimumCodeGenerationUnlimitedRegisters,
    /// Number of source vertices (= number of leaves in the target DAG).
    num_source_vertices: usize,
    /// For each source vertex x, the target index of x¹ (first chain node).
    /// `None` if vertex x has out-degree 0 (no chain, leaf only).
    chain_start: Vec<Option<usize>>,
    /// For each leaf x⁰ (source vertex x), the list of internal node target
    /// indices that use x⁰ as a right child.
    right_child_users: Vec<Vec<usize>>,
}

impl ReductionResult for ReductionFVSToCodeGen {
    type Source = MinimumFeedbackVertexSet<i32>;
    type Target = MinimumCodeGenerationUnlimitedRegisters;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a feedback vertex set from a target evaluation-order solution.
    ///
    /// A leaf register R_x is destroyed when x¹ executes (left operand).
    /// If any right-child user of x⁰ is evaluated after x¹, a LOAD was needed,
    /// meaning x is in the feedback vertex set.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_source_vertices;
        let mut source_config = vec![0usize; n];

        // target_solution[i] = evaluation position for the i-th internal node
        // Internal nodes are indices n, n+1, ..., n+m-1 (sorted), so
        // target_solution[j] = position for internal node (n + j).

        // Build: target_node_index -> evaluation position
        let num_internal = target_solution.len();
        let mut eval_pos = vec![0usize; num_internal];
        for j in 0..num_internal {
            // internal node (n + j) has evaluation position target_solution[j]
            eval_pos[j] = target_solution[j];
        }

        for x in 0..n {
            if let Some(chain_start_idx) = self.chain_start[x] {
                // chain_start_idx is the target node index of x¹
                // Its position in the internal array = chain_start_idx - n
                let start_j = chain_start_idx - n;
                let start_pos = eval_pos[start_j];

                // Check if any right-child user of leaf x⁰ is evaluated after x¹
                for &user_idx in &self.right_child_users[x] {
                    let user_j = user_idx - n;
                    if eval_pos[user_j] > start_pos {
                        source_config[x] = 1;
                        break;
                    }
                }
            }
        }

        source_config
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices + num_arcs",
    }
)]
impl ReduceTo<MinimumCodeGenerationUnlimitedRegisters> for MinimumFeedbackVertexSet<i32> {
    type Result = ReductionFVSToCodeGen;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let m = self.graph().num_arcs();

        // Build outgoing adjacency list
        let mut out_neighbors: Vec<Vec<usize>> = vec![vec![]; n];
        for (u, v) in self.graph().arcs() {
            out_neighbors[u].push(v);
        }

        // Assign internal node indices: leaves are 0..n, chain nodes are n..n+m
        let mut left_arcs = Vec::with_capacity(m);
        let mut right_arcs = Vec::with_capacity(m);
        let mut chain_start = vec![None; n];
        let mut right_child_users: Vec<Vec<usize>> = vec![vec![]; n];

        let mut next_internal = n; // first internal node index
        for x in 0..n {
            let neighbors = &out_neighbors[x];
            let d = neighbors.len();
            if d == 0 {
                continue;
            }

            chain_start[x] = Some(next_internal);

            for i in 0..d {
                let node_idx = next_internal + i;
                // Left child: predecessor in chain
                let left_child = if i == 0 {
                    x // leaf x⁰
                } else {
                    next_internal + i - 1 // previous chain node
                };
                // Right child: leaf y_i⁰
                let right_child = neighbors[i]; // leaf index = source vertex index

                left_arcs.push((node_idx, left_child));
                right_arcs.push((node_idx, right_child));
                right_child_users[right_child].push(node_idx);
            }

            next_internal += d;
        }

        debug_assert_eq!(next_internal, n + m);

        let target =
            MinimumCodeGenerationUnlimitedRegisters::new(n + m, left_arcs, right_arcs);

        ReductionFVSToCodeGen {
            target,
            num_source_vertices: n,
            chain_start,
            right_child_users,
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
fn issue_example_source() -> MinimumFeedbackVertexSet<i32> {
    use crate::topology::DirectedGraph;
    // 3-cycle: a→b→c→a (vertices 0,1,2)
    MinimumFeedbackVertexSet::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1i32; 3],
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
            let reduction =
                ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&source);

            // Find a target witness whose extracted source solution matches an optimal FVS
            let solver = BruteForce::new();
            let source_witnesses = solver.find_all_witnesses(&source);
            let target_witnesses = solver.find_all_witnesses(reduction.target_problem());

            let (source_config, target_config) = target_witnesses
                .iter()
                .find_map(|tw| {
                    let extracted = reduction.extract_solution(tw);
                    if source_witnesses.contains(&extracted) {
                        Some((extracted, tw.clone()))
                    } else {
                        None
                    }
                })
                .expect("canonical FVS -> CodeGen example must have matching witness");

            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumfeedbackvertexset_minimumcodegenerationunlimitedregisters.rs"]
mod tests;
