//! Reduction from MaximumContactMapOverlap to ILP (Integer Linear Programming).
//!
//! Binary mapping variables `x_(i,j)` indicate that residue `i in V_1` is
//! aligned to residue `j in V_2`. Row and column inequalities encode a partial
//! injective alignment. Order-preservation is enforced by forbidding crossings
//! and equal-image matches: for every `i < k in V_1` and every `j >= l in V_2`,
//! we add `x_(i,j) + x_(k,l) <= 1`. For every pair of contacts
//! `({i,k} in E_1, {j,l} in E_2)` with `i < k` and `j < l` we introduce a
//! binary `y_(i,k,j,l)` linked by `y <= x_(i,j)` and `y <= x_(k,l)`. The ILP
//! objective is `max sum y_(i,k,j,l)`, which equals the number of preserved
//! contacts under the alignment.
//!
//! This is a direct ILP rendering of the polyhedral formulation studied by
//! Andonov, Malod-Dognin, and Yanev (J. Comput. Biol., 2011) and by
//! Xie and Sahinidis (J. Comput. Biol., 2007).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximumContactMapOverlap;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MaximumContactMapOverlap to ILP.
///
/// Variable layout (all binary):
/// - `x_(i,j)` at index `i * n2 + j` for `i in V_1`, `j in V_2`
/// - `y_(i,k,j,l)` for each contact pair from `E_1 x E_2` (with `i < k` and
///   `j < l` enforced by the contact canonicalization), indexed sequentially
///   after the `x` block in the order they are enumerated by the constructor.
#[derive(Debug, Clone)]
pub struct ReductionCMOToILP {
    target: ILP<bool>,
    num_vertices_1: usize,
    num_vertices_2: usize,
}

impl ReductionResult for ReductionCMOToILP {
    type Source = MaximumContactMapOverlap;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract the CMO configuration from the ILP assignment.
    ///
    /// For each source residue `i in V_1`, find the unique `j` with
    /// `x_(i,j) = 1` and encode it as `j + 1` (CMO's `bot` is `0`); if no
    /// `x_(i,*)` is selected, the residue is left unmatched (`0`).
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let n2 = self.num_vertices_2;
        (0..self.num_vertices_1)
            .map(|residue| {
                let mut selected =
                    (0..n2).filter(|&mapped| target_solution[residue * n2 + mapped] == 1);
                match (selected.next(), selected.next()) {
                    (Some(mapped), None) => Ok(mapped + 1),
                    (None, _) => Ok(0),
                    (Some(_), Some(_)) => Err(crate::rules::ExtractionError::invalid(format!(
                        "source residue {residue} maps to multiple target residues"
                    ))),
                }
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices_1 * num_vertices_2 + num_contacts_1 * num_contacts_2",
        num_constraints = "num_vertices_1 + num_vertices_2 + num_vertices_1 * (num_vertices_1 - 1) / 2 * num_vertices_2 * (num_vertices_2 + 1) / 2 + 2 * num_contacts_1 * num_contacts_2",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumContactMapOverlap {
    type Result = ReductionCMOToILP;

    fn reduce_to(&self) -> Self::Result {
        let n1 = self.num_vertices_1();
        let n2 = self.num_vertices_2();
        let contacts_1 = self.contacts_1();
        let contacts_2 = self.contacts_2();

        let num_x = n1 * n2;
        let x_idx = |i: usize, j: usize| -> usize { i * n2 + j };

        // y-variables: one per (contact in E_1) x (contact in E_2). The
        // canonicalized contacts already satisfy i < k and j < l.
        let num_y = contacts_1.len() * contacts_2.len();
        let num_vars = num_x + num_y;
        let y_idx = |seq: usize| -> usize { num_x + seq };

        let mut constraints: Vec<LinearConstraint> = Vec::new();

        // Row constraints: each residue of G_1 maps to at most one residue of G_2.
        for i in 0..n1 {
            let terms: Vec<(usize, f64)> = (0..n2).map(|j| (x_idx(i, j), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        // Column constraints: each residue of G_2 receives at most one residue of G_1.
        for j in 0..n2 {
            let terms: Vec<(usize, f64)> = (0..n1).map(|i| (x_idx(i, j), 1.0)).collect();
            constraints.push(LinearConstraint::le(terms, 1.0));
        }

        // Order-preservation: for i < k in V_1 and j >= l in V_2,
        // forbid x_(i,j) + x_(k,l) <= 1. This rules out crossings (j > l)
        // as well as equal-image matches (j == l).
        for i in 0..n1 {
            for k in (i + 1)..n1 {
                for j in 0..n2 {
                    for l in 0..=j {
                        constraints.push(LinearConstraint::le(
                            vec![(x_idx(i, j), 1.0), (x_idx(k, l), 1.0)],
                            1.0,
                        ));
                    }
                }
            }
        }

        // Linking constraints for every contact pair: y_(i,k,j,l) <= x_(i,j) and
        // y_(i,k,j,l) <= x_(k,l). Because each y has positive objective
        // coefficient and there is no negative coupling, an optimum sets y to 1
        // exactly when both endpoint-match variables are selected.
        let mut seq = 0usize;
        for &(i, k) in contacts_1 {
            for &(j, l) in contacts_2 {
                let yv = y_idx(seq);
                constraints.push(LinearConstraint::le(
                    vec![(yv, 1.0), (x_idx(i, j), -1.0)],
                    0.0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(yv, 1.0), (x_idx(k, l), -1.0)],
                    0.0,
                ));
                seq += 1;
            }
        }

        // Objective: maximize the number of preserved contacts.
        let objective: Vec<(usize, f64)> = (0..num_y).map(|s| (y_idx(s), 1.0)).collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionCMOToILP {
            target,
            num_vertices_1: n1,
            num_vertices_2: n2,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumcontactmapoverlap_to_ilp",
        build: || {
            // Canonical instance from issue #1043:
            //   G_1: n_1 = 4, E_1 = {{0,2}, {1,3}}
            //   G_2: n_2 = 5, E_2 = {{0,3}, {1,4}, {0,2}}
            // Optimal CMO alignment 0->0, 1->1, 2->3, 3->4 preserves 2 contacts.
            let source = MaximumContactMapOverlap::new(
                4,
                vec![(0, 2), (1, 3)],
                5,
                vec![(0, 3), (1, 4), (0, 2)],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumcontactmapoverlap_ilp.rs"]
mod tests;
