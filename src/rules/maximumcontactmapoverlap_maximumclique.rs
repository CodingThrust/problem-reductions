//! Reduction from MaximumContactMapOverlap to MaximumClique (unit weights) via an
//! order-preserving association graph.
//!
//! One target vertex is created for every pair (contact of `G_1`, contact of `G_2`);
//! it proposes the two residue assignments `u -> a` and `v -> b`. Two vertices are
//! adjacent when every pair of proposed assignments compares identically on both
//! sides (`<`, `=`, `>`), so a clique is a set of mutually consistent assignments:
//! a partial injective order-preserving alignment. A maximum clique has as many
//! vertices as the optimal alignment has preserved contacts.
//!
//! Background: Strickland, Barnes and Sokol, "Optimal Protein Structure Alignment
//! Using Maximum Cliques", Operations Research 53(3), 2005, formulate contact-map
//! overlap through maximum cliques. The construction here was derived for this
//! library's definitions and is not taken from that article.

use crate::models::graph::{MaximumClique, MaximumContactMapOverlap};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::One;

/// Result of reducing MaximumContactMapOverlap to MaximumClique.
///
/// Target vertex `i * num_contacts_2 + j` stands for the pair (`i`-th contact of
/// `G_1`, `j`-th contact of `G_2`), in the order the contacts are stored.
#[derive(Debug, Clone)]
pub struct ReductionCMOToClique {
    target: MaximumClique<SimpleGraph, One>,
    num_vertices_1: usize,
    /// Per target vertex, its two assignments `(source residue, target residue)`.
    assignments: Vec<[(usize, usize); 2]>,
}

impl ReductionResult for ReductionCMOToClique {
    type Source = MaximumContactMapOverlap;
    type Target = MaximumClique<SimpleGraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Write `t + 1` at source position `s` for every assignment `(s, t)` of a selected
    /// vertex; every other position stays unmatched.
    ///
    /// The result of a maximum clique is an optimal alignment. A clique that is not
    /// maximum may extract to an alignment preserving more contacts than the clique has
    /// vertices, because compatible assignments can imply further preserved contacts.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0.is_some(),
            "selected vertices are not a clique",
        )?;

        let mut alignment = vec![0; self.num_vertices_1];
        for (pair, _) in self
            .assignments
            .iter()
            .zip(target_solution)
            .filter(|&(_, &selected)| selected)
        {
            for &(source, image) in pair {
                alignment[source] = image + 1;
            }
        }
        Ok(alignment)
    }
}

/// Two assignment pairs are compatible when all cross comparisons agree on both sides.
fn compatible(p: &[(usize, usize); 2], q: &[(usize, usize); 2]) -> bool {
    p.iter()
        .all(|&(s, t)| q.iter().all(|&(s2, t2)| s.cmp(&s2) == t.cmp(&t2)))
}

/// Number of target vertices, an overflow error when it does not fit in `usize`.
fn pair_count(
    num_contacts_1: usize,
    num_contacts_2: usize,
) -> Result<usize, crate::rules::ReductionError> {
    num_contacts_1.checked_mul(num_contacts_2).ok_or_else(|| {
        crate::rules::ReductionError::integer_overflow::<
            MaximumContactMapOverlap,
            MaximumClique<SimpleGraph, One>,
        >("counting contact pairs")
    })
}

#[reduction(transform = {
    exact {
        num_vertices = "num_contacts_1 * num_contacts_2",
    },
    upper_bound {
        num_edges = "num_contacts_1 * (num_contacts_1 - 1) * num_contacts_2 * (num_contacts_2 - 1) / 2",
    },
})]
impl ReduceTo<MaximumClique<SimpleGraph, One>> for MaximumContactMapOverlap {
    type Result = ReductionCMOToClique;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let contacts_1 = self.contacts_1();
        let contacts_2 = self.contacts_2();
        let num_vertices = pair_count(contacts_1.len(), contacts_2.len())?;

        let mut assignments = Vec::new();
        for &(u, v) in contacts_1 {
            for &(a, b) in contacts_2 {
                assignments.push([(u, a), (v, b)]);
            }
        }
        debug_assert_eq!(assignments.len(), num_vertices);

        let mut edges = Vec::new();
        for (p, left) in assignments.iter().enumerate() {
            for (offset, right) in assignments[p + 1..].iter().enumerate() {
                if compatible(left, right) {
                    edges.push((p, p + 1 + offset));
                }
            }
        }

        let graph = SimpleGraph::try_new(num_vertices, edges)
            .map_err(<Self as ReduceTo<MaximumClique<SimpleGraph, One>>>::target_construction)?;
        Ok(ReductionCMOToClique {
            target: MaximumClique::new(graph, vec![One; num_vertices]),
            num_vertices_1: self.num_vertices_1(),
            assignments,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumcontactmapoverlap_to_maximumclique",
        build: || {
            let source = MaximumContactMapOverlap::new(
                4,
                vec![(0, 2), (1, 3), (0, 3)],
                4,
                vec![(0, 1), (2, 3), (0, 3)],
            );
            let mut clique = vec![false; 9];
            clique[0] = true;
            clique[8] = true;
            crate::example_db::specs::rule_example_with_witness::<_, MaximumClique<SimpleGraph, One>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![1, 0, 2, 4]),
                    target_config: serde_json::json!(clique),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumcontactmapoverlap_maximumclique.rs"]
mod tests;
