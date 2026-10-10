//! Capped match-tuple graph with a polynomial anchored fallback.
//!
//! Redundant strings are removed first. Tuple vertices preserve the LCS value;
//! for k retained strings, anchored graphs have L * k plus the LCS value.
//! The cap is L * (k + 2) + F,
//! where F counts equal-symbol position pairs against a shortest input string.

use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::{is_subsequence, LongestCommonSubsequence};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::One;
use std::collections::{BTreeMap, HashSet};
use std::ops::Range;

#[derive(Debug, Clone)]
enum Encoding {
    Tuples(Vec<(usize, Vec<usize>)>),
    Anchored {
        anchor: Vec<usize>,
        embeddings: Vec<Vec<Range<usize>>>,
    },
}

/// Target graph and the selected mathematical witness mapping.
#[derive(Debug, Clone)]
pub struct ReductionLCSToIS {
    target: MaximumIndependentSet<SimpleGraph, One>,
    encoding: Encoding,
    max_length: usize,
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
        let mut decoded = Vec::with_capacity(self.max_length);
        match &self.encoding {
            Encoding::Tuples(nodes) => {
                let mut selected = nodes
                    .iter()
                    .zip(solution)
                    .filter(|(_, bit)| **bit)
                    .map(|((symbol, positions), _)| (positions[0], *symbol))
                    .collect::<Vec<_>>();
                selected.sort_unstable_by_key(|&(position, _)| position);
                decoded.extend(selected.into_iter().map(|(_, symbol)| Some(symbol)));
            }
            Encoding::Anchored { anchor, embeddings } => {
                for (p, &symbol) in anchor.iter().enumerate() {
                    if solution[3 * p] != solution[3 * p + 1] {
                        return Err(crate::rules::ExtractionError::invalid(
                            "active anchor twins are incomplete",
                        ));
                    }
                    if solution[3 * p]
                        && embeddings.iter().all(|groups| {
                            let group = &groups[p];
                            solution[group.start..group.end - 1].iter().any(|&bit| bit)
                        })
                    {
                        decoded.push(Some(symbol));
                    }
                }
            }
        }
        while decoded.len() < self.max_length {
            decoded.push(None);
        }
        Ok(decoded)
    }
}

#[reduction(transform = upper_bound {
    num_vertices = "max_length * (num_strings + 2) + anchor_matching_pairs",
    num_edges = "(max_length * (num_strings + 2) + anchor_matching_pairs)^2",
})]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, One>> for LongestCommonSubsequence {
    type Result = ReductionLCSToIS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        if self.max_length() == 0 {
            return Ok(ReductionLCSToIS {
                target: MaximumIndependentSet::new(SimpleGraph::new(0, vec![]), vec![]),
                encoding: Encoding::Tuples(vec![]),
                max_length: 0,
            });
        }
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<
                Self,
                MaximumIndependentSet<SimpleGraph, One>,
            >("counting anchored vertices")
        };
        let anchor = self
            .strings()
            .iter()
            .min_by_key(|s| s.len())
            .map(Vec::as_slice)
            .unwrap_or_default();
        let mut seen = HashSet::new();
        let retained = std::iter::once(anchor).chain(
            self.strings()
                .iter()
                .map(Vec::as_slice)
                .filter(|string| !is_subsequence(anchor, string) && seen.insert(*string)),
        );
        let positions = retained
            .map(|string| {
                let mut map = BTreeMap::<usize, Vec<usize>>::new();
                for (j, &symbol) in string.iter().enumerate() {
                    map.entry(symbol).or_default().push(j);
                }
                map
            })
            .collect::<Vec<_>>();

        // Only nonempty position lists enter the product; a later absent symbol
        // must contribute zero before any multiplication can exceed the cap.
        let common = positions[0]
            .keys()
            .filter_map(|&symbol| {
                let lists = positions
                    .iter()
                    .map(|map| map.get(&symbol).map(Vec::as_slice))
                    .collect::<Option<Vec<_>>>()?;
                Some((symbol, lists))
            })
            .collect::<Vec<_>>();
        let mut cap = anchor.len().checked_mul(3).ok_or_else(overflow)?;
        for map in &positions[1..] {
            for symbol in anchor {
                cap = cap
                    .checked_add(map.get(symbol).map_or(0, Vec::len))
                    .and_then(|count| count.checked_add(1))
                    .ok_or_else(overflow)?;
            }
        }
        let tuple_count = common.iter().try_fold(0usize, |total, (_, lists)| {
            let product = lists.iter().try_fold(1usize, |product, list| {
                product
                    .checked_mul(list.len())
                    .filter(|&count| count <= cap)
            })?;
            total.checked_add(product).filter(|&count| count <= cap)
        });
        let mut num_vertices = tuple_count.unwrap_or(cap);
        <Self as ReduceTo<MaximumIndependentSet<SimpleGraph, One>>>::exact_i64(
            num_vertices,
            "representing the independent-set objective",
        )?;
        let mut edges = Vec::new();
        let mut encoding = if tuple_count.is_some() {
            let mut nodes = Vec::with_capacity(num_vertices);
            for (symbol, lists) in common {
                let mut tuples = vec![Vec::new()];
                for list in lists {
                    tuples = tuples
                        .into_iter()
                        .flat_map(|prefix| {
                            list.iter().map(move |&position| {
                                let mut tuple = prefix.clone();
                                tuple.push(position);
                                tuple
                            })
                        })
                        .collect();
                }
                nodes.extend(tuples.into_iter().map(|tuple| (symbol, tuple)));
            }
            for (i, (_, left)) in nodes.iter().enumerate() {
                for (j, (_, right)) in nodes.iter().enumerate().skip(i + 1) {
                    if !left.iter().zip(right).all(|(a, b)| a < b)
                        && !left.iter().zip(right).all(|(a, b)| a > b)
                    {
                        edges.push((i, j));
                    }
                }
            }
            Encoding::Tuples(nodes)
        } else {
            let (anchor_edges, anchor_encoding) = anchored_graph(anchor, &positions, cap)?;
            edges = anchor_edges;
            anchor_encoding
        };
        // A tuple graph costing at most the anchor's vertices alone cannot lose.
        if tuple_count.is_some() && edges.len() > cap - num_vertices {
            let (anchor_edges, anchor_encoding) = anchored_graph(anchor, &positions, cap)?;
            if edges.len() > anchor_edges.len()
                && edges.len() - anchor_edges.len() > cap - num_vertices
            {
                num_vertices = cap;
                edges = anchor_edges;
                encoding = anchor_encoding;
            }
        }
        let target = MaximumIndependentSet::new(
            SimpleGraph::new(num_vertices, edges),
            vec![One; num_vertices],
        );
        Ok(ReductionLCSToIS {
            target,
            encoding,
            max_length: self.max_length(),
        })
    }
}

fn anchored_graph(
    anchor: &[usize],
    positions: &[BTreeMap<usize, Vec<usize>>],
    cap: usize,
) -> Result<(Vec<(usize, usize)>, Encoding), crate::rules::ReductionError> {
    <LongestCommonSubsequence as ReduceTo<MaximumIndependentSet<SimpleGraph, One>>>::exact_i64(
        cap,
        "representing the anchored independent-set objective",
    )?;
    let mut edges = Vec::new();
    let mut cursor = anchor.len() * 3;
    let mut embeddings = Vec::with_capacity(positions.len() - 1);
    for p in 0..anchor.len() {
        edges.extend([(3 * p, 3 * p + 2), (3 * p + 1, 3 * p + 2)]);
    }
    for map in &positions[1..] {
        let mut groups = Vec::<(&[usize], Range<usize>)>::with_capacity(anchor.len());
        for (p, symbol) in anchor.iter().enumerate() {
            let matches = map.get(symbol).map(Vec::as_slice).unwrap_or_default();
            let group = cursor..cursor + matches.len() + 1;
            cursor = group.end;
            for a in group.clone() {
                for b in a + 1..group.end {
                    edges.push((a, b));
                }
            }
            let skip = group.end - 1;
            edges.extend([(3 * p, skip), (3 * p + 1, skip)]);
            edges.extend((group.start..skip).map(|j| (3 * p + 2, j)));
            // Only this string's earlier slots can cross. Sorted matches let
            // us emit exactly the conflicting prefix of each group.
            for (earlier, previous) in &groups {
                for (i, &position) in earlier.iter().enumerate() {
                    let conflicts = matches.partition_point(|&j| j <= position);
                    edges.extend((0..conflicts).map(|j| (previous.start + i, group.start + j)));
                }
            }
            groups.push((matches, group));
        }
        embeddings.push(groups.into_iter().map(|(_, range)| range).collect());
    }
    debug_assert_eq!(cursor, cap);
    Ok((
        edges,
        Encoding::Anchored {
            anchor: anchor.to_vec(),
            embeddings,
        },
    ))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestcommonsubsequence_to_maximumindependentset",
        build: || {
            let source = LongestCommonSubsequence::new(3, vec![vec![0, 1, 0, 2], vec![1, 0, 2, 0]]);
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MaximumIndependentSet<SimpleGraph, One>,
            >(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![Some(1), Some(0), Some(2), None]),
                    target_config: serde_json::json!(vec![false, false, true, false, true, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestcommonsubsequence_maximumindependentset.rs"]
mod tests;
