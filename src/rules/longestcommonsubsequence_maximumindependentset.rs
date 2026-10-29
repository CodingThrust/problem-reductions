//! Polynomial reduction from LCS to unweighted independent set.
//!
//! Use match tuples when they dominate the normalized reward-anchored graph.
//! The target optimum is LCS for tuples, or h * m + LCS for h slots and m strings.

use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::{is_subsequence, LongestCommonSubsequence};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;
use std::collections::{BTreeSet, HashSet};
use std::ops::Range;

/// Both encodings decode the same way: vertex `i < symbols.len()` selects
/// `symbols[i]` when every embedding group `i` is nonempty. Tuple vertices,
/// ordered by anchor slot, have no embedding groups.
#[derive(Debug, Clone)]
struct Encoding {
    symbols: Vec<usize>,
    // Matching vertices per string and slot; the skip vertex is range.end.
    embeddings: Vec<Vec<Range<usize>>>,
}

/// Target graph and witness mapping for the selected encoding.
#[derive(Debug, Clone)]
pub struct ReductionLCSToIS {
    target: MaximumIndependentSet<SimpleGraph, One>,
    max_length: usize,
    encoding: Encoding,
}

impl ReductionResult for ReductionLCSToIS {
    type Source = LongestCommonSubsequence;
    type Target = MaximumIndependentSet<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.0.is_some(),
            "selected vertices do not form an independent set",
        )?;
        let Encoding {
            symbols,
            embeddings,
        } = &self.encoding;
        let mut decoded = symbols
            .iter()
            .enumerate()
            .filter(|&(i, _)| {
                solution[i]
                    && embeddings
                        .iter()
                        .all(|groups| solution[groups[i].clone()].iter().any(|&bit| bit))
            })
            .map(|(_, &symbol)| Some(symbol))
            .collect::<Vec<_>>();
        decoded.resize(self.max_length, None);
        Ok(decoded)
    }
}

fn tuple_graph(
    anchor: &[usize],
    matches: &[Vec<Vec<usize>>],
    vertex_cap: usize,
    edge_cap: usize,
) -> Option<(SimpleGraph, Encoding)> {
    let count = (0..anchor.len()).try_fold(0usize, |total, p| {
        let product = matches.iter().try_fold(1usize, |product, slots| {
            product
                .checked_mul(slots[p].len())
                .filter(|&n| n <= vertex_cap)
        })?;
        total.checked_add(product).filter(|&n| n <= vertex_cap)
    })?;
    let mut tuples = Vec::with_capacity(count);
    for p in 0..anchor.len() {
        let mut product = vec![vec![p]];
        for slots in matches {
            product = product
                .iter()
                .flat_map(|prefix| {
                    slots[p].iter().map(|&j| {
                        let mut tuple = prefix.clone();
                        tuple.push(j);
                        tuple
                    })
                })
                .collect();
        }
        tuples.extend(product);
    }
    let mut edges = Vec::new();
    for (i, left) in tuples.iter().enumerate() {
        for (j, right) in tuples.iter().enumerate().skip(i + 1) {
            let before = left[0] < right[0];
            if left
                .iter()
                .zip(right)
                .any(|(a, b)| a == b || (a < b) != before)
            {
                if edges.len() == edge_cap {
                    return None;
                }
                edges.push((i, j));
            }
        }
    }
    let encoding = Encoding {
        symbols: tuples.iter().map(|tuple| anchor[tuple[0]]).collect(),
        embeddings: vec![],
    };
    Some((SimpleGraph::new(count, edges), encoding))
}

fn anchored_graph(anchor: Vec<usize>, matches: &[Vec<Vec<usize>>]) -> (SimpleGraph, Encoding) {
    let mut edges = Vec::new();
    let mut cursor = anchor.len();
    let mut embeddings = Vec::new();
    for slots in matches {
        let mut groups = Vec::<Range<usize>>::new();
        for (p, positions) in slots.iter().enumerate() {
            let group = cursor..cursor + positions.len();
            let skip = group.end;
            cursor = skip + 1;
            for a in group.start..=skip {
                edges.extend((a + 1..=skip).map(|b| (a, b)));
            }
            edges.push((p, skip));
            for (earlier, previous) in slots.iter().zip(&groups) {
                for (i, &position) in earlier.iter().enumerate() {
                    let conflicts = positions.partition_point(|&j| j <= position);
                    edges.extend((0..conflicts).map(|j| (previous.start + i, group.start + j)));
                }
            }
            groups.push(group);
        }
        embeddings.push(groups);
    }
    (
        SimpleGraph::new(cursor, edges),
        Encoding {
            symbols: anchor,
            embeddings,
        },
    )
}

#[reduction(transform = upper_bound {
    num_vertices = "max_length * (num_strings + total_length - max_length)",
    num_edges = "max_length * (max_length + 1) / 2 * (sum_triangular_lengths - max_length * (max_length + 1) / 2) + max_length * (num_strings - 1)",
})]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, One>> for LongestCommonSubsequence {
    type Result = ReductionLCSToIS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let anchor = self
            .strings()
            .iter()
            .min_by_key(|s| s.len())
            .into_iter()
            .flatten()
            .copied()
            .filter(|symbol| self.strings().iter().all(|s| s.contains(symbol)))
            .collect::<Vec<_>>();
        let symbols = anchor.iter().copied().collect::<HashSet<_>>();
        let strings = self
            .strings()
            .iter()
            .map(|s| {
                s.iter()
                    .copied()
                    .filter(|c| symbols.contains(c))
                    .collect::<Vec<_>>()
            })
            .filter(|string| !is_subsequence(&anchor, string))
            .collect::<BTreeSet<_>>();
        let matches = strings
            .iter()
            .map(|s| {
                anchor
                    .iter()
                    .map(|symbol| {
                        (0..s.len())
                            .filter(|&j| s[j] == *symbol)
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let anchored = anchored_graph(anchor.clone(), &matches);
        <Self as ReduceTo<MaximumIndependentSet<SimpleGraph, One>>>::exact_i64(
            anchored.0.num_vertices(),
            "representing the independent-set objective",
        )?;
        let (graph, encoding) = tuple_graph(
            &anchor,
            &matches,
            anchored.0.num_vertices(),
            anchored.0.num_edges(),
        )
        .unwrap_or(anchored);
        let weights = vec![One; graph.num_vertices()];
        let target = MaximumIndependentSet::new(graph, weights);
        Ok(ReductionLCSToIS {
            target,
            max_length: self.max_length(),
            encoding,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcommonsubsequence_to_maximumindependentset",
        build: || {
            let source = LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![1, 0]]);
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MaximumIndependentSet<SimpleGraph, One>,
            >(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![Some(0), None]),
                    target_config: serde_json::json!(vec![true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs"]
mod tests;
