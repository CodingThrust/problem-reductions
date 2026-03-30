//! Reduction from LongestCommonSubsequence to MaximumIndependentSet.
//!
//! Constructs a conflict graph where vertices are match-node k-tuples
//! (positions in each string that share the same character) and edges
//! connect conflicting tuples that cannot both appear in a valid common
//! subsequence. A maximum independent set in this graph corresponds to
//! a longest common subsequence.
//!
//! Reference: Santini, Blum, Djukanovic et al. (2021),
//! "Solving Longest Common Subsequence Problems via a Transformation
//! to the Maximum Clique Problem," Computers & Operations Research.

use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::LongestCommonSubsequence;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::One;

/// Result of reducing LongestCommonSubsequence to MaximumIndependentSet.
///
/// Each vertex in the target graph corresponds to a match-node k-tuple
/// `(p_1, ..., p_k)` where all strings have the same character at their
/// respective positions.
#[derive(Debug, Clone)]
pub struct ReductionLCSToIS {
    /// The target MaximumIndependentSet problem.
    target: MaximumIndependentSet<SimpleGraph, One>,
    /// Match-node k-tuples: `match_nodes[v]` gives the position tuple for vertex v.
    match_nodes: Vec<Vec<usize>>,
    /// Character for each match node.
    match_chars: Vec<usize>,
    /// Maximum possible subsequence length in the source problem.
    max_length: usize,
    /// Alphabet size of the source problem (used as the padding symbol).
    alphabet_size: usize,
}

impl ReductionResult for ReductionLCSToIS {
    type Source = LongestCommonSubsequence;
    type Target = MaximumIndependentSet<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract an LCS solution from a MaximumIndependentSet solution.
    ///
    /// Selected vertices correspond to match nodes. Sort by position in
    /// the first string to get the subsequence order, then pad to `max_length`.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Collect selected match nodes with their characters
        let mut selected: Vec<(usize, usize)> = target_solution
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| (self.match_nodes[i][0], self.match_chars[i]))
            .collect();
        // Sort by position in the first string
        selected.sort_by_key(|&(pos, _)| pos);

        // Build config: characters followed by padding
        let mut config = Vec::with_capacity(self.max_length);
        for &(_, ch) in &selected {
            config.push(ch);
        }
        // Pad with alphabet_size (the padding symbol)
        while config.len() < self.max_length {
            config.push(self.alphabet_size);
        }
        config
    }
}

#[reduction(
    overhead = {
        num_vertices = "cross_frequency_product",
        num_edges = "cross_frequency_product^2",
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, One>> for LongestCommonSubsequence {
    type Result = ReductionLCSToIS;

    fn reduce_to(&self) -> Self::Result {
        let strings = self.strings();
        let k = self.num_strings();

        // Step 1: Build match nodes.
        // For each character c, find all k-tuples of positions where every
        // string has character c at its respective position.
        let mut match_nodes: Vec<Vec<usize>> = Vec::new();
        let mut match_chars: Vec<usize> = Vec::new();

        for c in 0..self.alphabet_size() {
            // For each string, collect positions where character c appears
            let positions_per_string: Vec<Vec<usize>> = strings
                .iter()
                .map(|s| {
                    s.iter()
                        .enumerate()
                        .filter(|(_, &sym)| sym == c)
                        .map(|(i, _)| i)
                        .collect()
                })
                .collect();

            // Generate all k-tuples (Cartesian product of position lists)
            let tuples = cartesian_product(&positions_per_string);
            for tuple in tuples {
                match_nodes.push(tuple);
                match_chars.push(c);
            }
        }

        let num_vertices = match_nodes.len();

        // Step 2: Build conflict edges.
        // Two nodes u = (a_1, ..., a_k) and v = (b_1, ..., b_k) conflict when
        // they cannot both appear in a valid common subsequence: NOT(all a_i < b_i)
        // AND NOT(all a_i > b_i).
        let mut edges: Vec<(usize, usize)> = Vec::new();

        for i in 0..num_vertices {
            for j in (i + 1)..num_vertices {
                if nodes_conflict(&match_nodes[i], &match_nodes[j], k) {
                    edges.push((i, j));
                }
            }
        }

        let target = MaximumIndependentSet::new(
            SimpleGraph::new(num_vertices, edges),
            vec![One; num_vertices],
        );

        ReductionLCSToIS {
            target,
            match_nodes,
            match_chars,
            max_length: self.max_length(),
            alphabet_size: self.alphabet_size(),
        }
    }
}

/// Check whether two match nodes conflict (cannot both be in a common subsequence).
///
/// Two nodes `u = (a_1, ..., a_k)` and `v = (b_1, ..., b_k)` conflict when
/// NOT (all a_i < b_i) AND NOT (all a_i > b_i).
fn nodes_conflict(u: &[usize], v: &[usize], k: usize) -> bool {
    let mut all_less = true;
    let mut all_greater = true;
    for i in 0..k {
        if u[i] >= v[i] {
            all_less = false;
        }
        if u[i] <= v[i] {
            all_greater = false;
        }
    }
    !all_less && !all_greater
}

/// Compute the Cartesian product of a list of position vectors.
fn cartesian_product(lists: &[Vec<usize>]) -> Vec<Vec<usize>> {
    if lists.is_empty() {
        return vec![vec![]];
    }

    let mut result = vec![vec![]];
    for list in lists {
        let mut new_result = Vec::new();
        for prefix in &result {
            for &item in list {
                let mut new_tuple = prefix.clone();
                new_tuple.push(item);
                new_result.push(new_tuple);
            }
        }
        result = new_result;
    }
    result
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    /// Build the example from the issue: k=2, s1="ABAC", s2="BACA", alphabet={A,B,C}.
    fn lcs_abac_baca() -> LongestCommonSubsequence {
        // A=0, B=1, C=2
        LongestCommonSubsequence::new(
            3,
            vec![
                vec![0, 1, 0, 2], // ABAC
                vec![1, 0, 2, 0], // BACA
            ],
        )
    }

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcommonsubsequence_to_maximumindependentset",
        build: || {
            // Issue example: MIS solution {v2, v4, v5} gives LCS "BAC" (length 3).
            // Match nodes (ordered by character):
            //   c=A(0): v0=(0,1), v1=(0,3), v2=(2,1), v3=(2,3)
            //   c=B(1): v4=(1,0)
            //   c=C(2): v5=(3,2)
            // MIS {v2, v4, v5} => positions B@(1,0), A@(2,1), C@(3,2)
            // source_config = [1, 0, 2, 3] (B, A, C, padding)
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MaximumIndependentSet<SimpleGraph, One>,
            >(
                lcs_abac_baca(),
                SolutionPair {
                    source_config: vec![1, 0, 2, 3],
                    target_config: vec![0, 0, 1, 0, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs"]
mod tests;
