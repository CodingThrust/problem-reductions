//! Reduction from Longest Common Subsequence (LCS) to MaximumIndependentSet.
//!
//! For k strings, we create a vertex for each k-tuple of positions where all
//! strings share the same character. Two vertices are connected by an edge if
//! their position tuples conflict — i.e., they cannot both appear in a common
//! subsequence because the relative ordering is inconsistent across strings.
//!
//! The maximum independent set in this conflict graph corresponds to the longest
//! common subsequence.

use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::One;

/// Check whether two position tuples conflict.
///
/// Two tuples `a` and `b` conflict if they cannot both belong to a common
/// subsequence. This happens when it is NOT the case that all components of `a`
/// are strictly less than the corresponding components of `b`, AND it is NOT the
/// case that all components of `b` are strictly less than the corresponding
/// components of `a`.
pub(crate) fn tuples_conflict(a: &[usize], b: &[usize]) -> bool {
    let all_a_lt_b = a.iter().zip(b.iter()).all(|(&ai, &bi)| ai < bi);
    let all_b_lt_a = b.iter().zip(a.iter()).all(|(&bi, &ai)| bi < ai);
    !all_a_lt_b && !all_b_lt_a
}

/// Result of reducing LCS to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionLCSToIS {
    /// The target MaximumIndependentSet problem.
    target: MaximumIndependentSet<SimpleGraph, One>,
    /// Position tuples for each vertex in the IS graph.
    /// `nodes[v]` is a k-tuple of positions, one per input string.
    nodes: Vec<Vec<usize>>,
    /// Length of the shortest input string (= number of source variables).
    num_source_variables: usize,
    /// Index of the shortest input string.
    shortest_index: usize,
}

impl ReductionResult for ReductionLCSToIS {
    type Source = LongestCommonSubsequence;
    type Target = MaximumIndependentSet<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract an LCS solution from an IS solution.
    ///
    /// For each selected vertex, look up its position in the shortest string
    /// and set that bit to 1 in the output configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut config = vec![0usize; self.num_source_variables];
        for (vertex_idx, &selected) in target_solution.iter().enumerate() {
            if selected == 1 {
                let pos_in_shortest = self.nodes[vertex_idx][self.shortest_index];
                config[pos_in_shortest] = 1;
            }
        }
        config
    }
}

impl ReductionLCSToIS {
    /// Get a reference to the position-tuple nodes.
    pub fn nodes(&self) -> &[Vec<usize>] {
        &self.nodes
    }
}

#[reduction(
    overhead = {
        num_vertices = "total_length^num_strings",
        num_edges = "total_length^(2 * num_strings)",
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, One>> for LongestCommonSubsequence {
    type Result = ReductionLCSToIS;

    fn reduce_to(&self) -> Self::Result {
        let strings = self.strings();
        let k = strings.len();

        // Find the shortest string index
        let shortest_index = strings
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| s.len())
            .map(|(i, _)| i)
            .unwrap_or(0);
        let num_source_variables = strings[shortest_index].len();

        // Collect character positions per string: char -> list of positions
        let mut char_positions: Vec<std::collections::HashMap<u8, Vec<usize>>> =
            Vec::with_capacity(k);
        for s in strings {
            let mut map = std::collections::HashMap::new();
            for (pos, &ch) in s.iter().enumerate() {
                map.entry(ch).or_insert_with(Vec::new).push(pos);
            }
            char_positions.push(map);
        }

        // Find characters common to all strings
        let common_chars: Vec<u8> = char_positions[0]
            .keys()
            .copied()
            .filter(|ch| char_positions.iter().all(|cp| cp.contains_key(ch)))
            .collect();

        // Generate match nodes: for each common character, take the Cartesian
        // product of positions across all strings
        let mut nodes: Vec<Vec<usize>> = Vec::new();
        for ch in &common_chars {
            let position_lists: Vec<&Vec<usize>> =
                char_positions.iter().map(|cp| &cp[ch]).collect();
            // Cartesian product of position_lists
            let mut tuples: Vec<Vec<usize>> = vec![vec![]];
            for positions in &position_lists {
                let mut new_tuples = Vec::new();
                for tuple in &tuples {
                    for &pos in *positions {
                        let mut t = tuple.clone();
                        t.push(pos);
                        new_tuples.push(t);
                    }
                }
                tuples = new_tuples;
            }
            nodes.extend(tuples);
        }

        let n = nodes.len();

        // Build conflict edges
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                if tuples_conflict(&nodes[i], &nodes[j]) {
                    edges.push((i, j));
                }
            }
        }

        let target = MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![One; n]);

        ReductionLCSToIS {
            target,
            nodes,
            num_source_variables,
            shortest_index,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs"]
mod tests;
