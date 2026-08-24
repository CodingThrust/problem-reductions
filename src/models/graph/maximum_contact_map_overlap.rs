//! Maximum Contact Map Overlap problem implementation.
//!
//! Given two finite ordered contact maps `G_1 = (V_1, E_1)` and
//! `G_2 = (V_2, E_2)` where each `V_r` is the ordered vertex set
//! `{0, 1, ..., n_r - 1}` and each `E_r` is a simple undirected contact set,
//! find an order-preserving partial injective alignment
//! `f: V_1 -> V_2 union {bot}` that maximizes the number of preserved contacts
//!
//! `|{{i, k} in E_1 : i, k both matched and {f(i), f(k)} in E_2}|`.
//!
//! The configuration vector has length `|V_1|`. For each source vertex `i`, the
//! value `config[i] in {0, 1, ..., |V_2|}` records the alignment: `0` denotes
//! `bot` (unmatched), and `j + 1` denotes "matched to vertex `j` of `G_2`".
//! Feasibility requires that the non-zero entries are pairwise distinct
//! (injectivity) and strictly increasing in source order (order-preserving).

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Max;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumContactMapOverlap",
        display_name: "Maximum Contact Map Overlap",
        aliases: &["CMO", "MaxCMO"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Maximize the number of preserved contacts under an order-preserving partial injective alignment from G_1 into G_2",
        fields: &[
            FieldInfo {
                name: "num_vertices_1",
                type_name: "usize",
                description: "Number of ordered residues/vertices in the first contact map G_1",
            },
            FieldInfo {
                name: "contacts_1",
                type_name: "Vec<(usize,usize)>",
                description: "Simple undirected contacts of G_1 as canonicalized (u,v) pairs with u < v",
            },
            FieldInfo {
                name: "num_vertices_2",
                type_name: "usize",
                description: "Number of ordered residues/vertices in the second contact map G_2",
            },
            FieldInfo {
                name: "contacts_2",
                type_name: "Vec<(usize,usize)>",
                description: "Simple undirected contacts of G_2 as canonicalized (u,v) pairs with u < v",
            },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MaximumContactMapOverlap",
        fields: &["num_vertices_1", "num_vertices_2", "num_contacts_1", "num_contacts_2"],
    }
}

/// The Maximum Contact Map Overlap problem.
///
/// Given two finite ordered contact maps `G_1 = (V_1, E_1)` and
/// `G_2 = (V_2, E_2)`, find an order-preserving partial injective alignment
/// `f: V_1 -> V_2 union {bot}` that maximizes the number of preserved contacts
///
/// `|{{i, k} in E_1 : i, k both matched and {f(i), f(k)} in E_2}|`.
///
/// # Configuration encoding
///
/// `dims()` returns `vec![|V_2| + 1; |V_1|]`. For each source vertex `i`,
/// `config[i] = 0` denotes `bot` (unmatched) and `config[i] = j + 1` denotes
/// "matched to vertex `j in V_2`". Feasibility requires that the nonzero
/// entries are pairwise distinct (injectivity) and strictly increasing along
/// the index order of `V_1` (order-preserving).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaximumContactMapOverlap {
    num_vertices_1: usize,
    contacts_1: Vec<(usize, usize)>,
    num_vertices_2: usize,
    contacts_2: Vec<(usize, usize)>,
}

/// Canonicalize a contact set: each pair is normalized to `(min, max)`, no
/// self-loops are allowed, all endpoints must be in range, and duplicates
/// (after normalization) cause a panic.
fn canonicalize_contacts(
    raw: Vec<(usize, usize)>,
    num_vertices: usize,
    side: &str,
) -> Vec<(usize, usize)> {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut out = Vec::with_capacity(raw.len());
    for (u, v) in raw {
        assert!(
            u < num_vertices && v < num_vertices,
            "{side} contact endpoint out of range for num_vertices = {num_vertices}: ({u}, {v})"
        );
        assert!(u != v, "{side} contact has self-loop: ({u}, {v})");
        let (a, b) = if u < v { (u, v) } else { (v, u) };
        assert!(
            seen.insert((a, b)),
            "{side} has duplicate contact after normalization: ({a}, {b})"
        );
        out.push((a, b));
    }
    out
}

impl MaximumContactMapOverlap {
    /// Construct a new instance from two ordered contact maps.
    ///
    /// Contacts are canonicalized to `(min, max)` pairs. Self-loops, duplicate
    /// contacts (after normalization), and out-of-range endpoints panic.
    pub fn new(
        num_vertices_1: usize,
        contacts_1: Vec<(usize, usize)>,
        num_vertices_2: usize,
        contacts_2: Vec<(usize, usize)>,
    ) -> Self {
        let contacts_1 = canonicalize_contacts(contacts_1, num_vertices_1, "G_1");
        let contacts_2 = canonicalize_contacts(contacts_2, num_vertices_2, "G_2");
        Self {
            num_vertices_1,
            contacts_1,
            num_vertices_2,
            contacts_2,
        }
    }

    /// Number of ordered residues/vertices in `G_1`.
    pub fn num_vertices_1(&self) -> usize {
        self.num_vertices_1
    }

    /// Number of ordered residues/vertices in `G_2`.
    pub fn num_vertices_2(&self) -> usize {
        self.num_vertices_2
    }

    /// Number of contacts in `G_1`.
    pub fn num_contacts_1(&self) -> usize {
        self.contacts_1.len()
    }

    /// Number of contacts in `G_2`.
    pub fn num_contacts_2(&self) -> usize {
        self.contacts_2.len()
    }

    /// Contacts of `G_1` as canonicalized `(u, v)` pairs with `u < v`.
    pub fn contacts_1(&self) -> &[(usize, usize)] {
        &self.contacts_1
    }

    /// Contacts of `G_2` as canonicalized `(u, v)` pairs with `u < v`.
    pub fn contacts_2(&self) -> &[(usize, usize)] {
        &self.contacts_2
    }

    /// Check that `config` describes an order-preserving partial injective
    /// alignment.
    ///
    /// Validity requires: `config.len() == |V_1|`, every entry lies in
    /// `0..=|V_2|` (with `0` denoting `bot`), all nonzero entries are
    /// pairwise distinct (injectivity), and the nonzero entries are strictly
    /// increasing in source order.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.num_vertices_1 {
            return false;
        }
        let max_value = self.num_vertices_2; // valid range is 0..=num_vertices_2
        let mut previous_nonzero: Option<usize> = None;
        let mut used: HashSet<usize> = HashSet::new();
        for &value in config {
            if value > max_value {
                return false;
            }
            if value == 0 {
                continue;
            }
            if !used.insert(value) {
                return false;
            }
            if let Some(prev) = previous_nonzero {
                if value <= prev {
                    return false;
                }
            }
            previous_nonzero = Some(value);
        }
        true
    }

    /// Count contacts of `G_1` preserved by the alignment `config`. Returns
    /// `None` if `config` is infeasible.
    pub fn preserved_contact_count(
        &self,
        config: &[usize],
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        if !self.is_valid_solution(config) {
            return Ok(None);
        }
        let contacts_2_set: HashSet<(usize, usize)> = self.contacts_2.iter().copied().collect();
        let mut count = 0usize;
        for &(i, k) in &self.contacts_1 {
            let fi = config[i];
            let fk = config[k];
            if fi == 0 || fk == 0 {
                continue;
            }
            // Encoding: nonzero value v means vertex v - 1 of G_2.
            let a = fi - 1;
            let b = fk - 1;
            let pair = if a < b { (a, b) } else { (b, a) };
            if contacts_2_set.contains(&pair) {
                count += 1;
            }
        }
        Ok(Some(i64::try_from(count).map_err(|_| {
            crate::traits::EvaluationError::IntegerOverflow(
                "converting preserved-contact count to i64".into(),
            )
        })?))
    }
}

impl Problem for MaximumContactMapOverlap {
    const NAME: &'static str = "MaximumContactMapOverlap";
    type Value = Max<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_vertices_2 + 1; self.num_vertices_1]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Max<i64>, crate::traits::EvaluationError> {
        Ok({
            match self.preserved_contact_count(config)? {
                Some(count) => Max(Some(count)),
                None => Max(None),
            }
        })
    }
}

crate::declare_variants! {
    default MaximumContactMapOverlap => "(num_vertices_2 + 1)^num_vertices_1",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Canonical example from the issue:
    //   G_1: n_1 = 4, E_1 = {{0,2}, {1,3}}
    //   G_2: n_2 = 5, E_2 = {{0,3}, {1,4}, {0,2}}
    // Optimal alignment: 0->0, 1->1, 2->3, 3->4 (encoded as [1, 2, 4, 5]).
    //   - order-preserving: 1 < 2 < 4 < 5
    //   - injectivity: all values distinct
    //   - contact {0,2}: mapped (0, 3); sorted (0, 3) in E_2
    //   - contact {1,3}: mapped (1, 4); sorted (1, 4) in E_2
    //   - value = 2 contacts preserved.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "maximum_contact_map_overlap",
        instance: Box::new(MaximumContactMapOverlap::new(
            4,
            vec![(0, 2), (1, 3)],
            5,
            vec![(0, 3), (1, 4), (0, 2)],
        )),
        optimal_config: vec![1, 2, 4, 5],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/maximum_contact_map_overlap.rs"]
mod tests;
