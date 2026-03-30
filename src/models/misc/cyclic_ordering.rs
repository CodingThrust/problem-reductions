//! Cyclic Ordering problem implementation.
//!
//! Given a finite set A and a collection C of ordered triples (a, b, c),
//! determine whether there exists a permutation f: A → {0, ..., |A|-1}
//! such that for each (a, b, c) ∈ C, the values f(a), f(b), f(c) appear
//! in cyclic order — i.e., (f(a) < f(b) < f(c)) ∨ (f(b) < f(c) < f(a))
//! ∨ (f(c) < f(a) < f(b)).

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "CyclicOrdering",
        display_name: "Cyclic Ordering",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a permutation satisfying cyclic ordering constraints on triples",
        fields: &[
            FieldInfo { name: "num_elements", type_name: "usize", description: "Number of elements in the set A" },
            FieldInfo { name: "triples", type_name: "Vec<(usize, usize, usize)>", description: "Collection of ordered triples (a, b, c) requiring cyclic order" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "CyclicOrdering",
        fields: &["num_elements", "num_triples"],
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CyclicOrdering {
    num_elements: usize,
    triples: Vec<(usize, usize, usize)>,
}

impl CyclicOrdering {
    fn validate_inputs(
        num_elements: usize,
        triples: &[(usize, usize, usize)],
    ) -> Result<(), String> {
        if num_elements == 0 {
            return Err("CyclicOrdering requires at least one element".to_string());
        }
        for (i, &(a, b, c)) in triples.iter().enumerate() {
            if a >= num_elements || b >= num_elements || c >= num_elements {
                return Err(format!(
                    "Triple {} has element(s) out of range 0..{}",
                    i, num_elements
                ));
            }
            if a == b || b == c || a == c {
                return Err(format!(
                    "Triple {} has duplicate elements ({}, {}, {})",
                    i, a, b, c
                ));
            }
        }
        Ok(())
    }

    pub fn try_new(
        num_elements: usize,
        triples: Vec<(usize, usize, usize)>,
    ) -> Result<Self, String> {
        Self::validate_inputs(num_elements, &triples)?;
        Ok(Self {
            num_elements,
            triples,
        })
    }

    /// Create a new CyclicOrdering instance.
    ///
    /// # Panics
    ///
    /// Panics if any triple element is out of range or if any triple has duplicate elements.
    pub fn new(num_elements: usize, triples: Vec<(usize, usize, usize)>) -> Self {
        Self::try_new(num_elements, triples).unwrap_or_else(|message| panic!("{message}"))
    }

    /// Number of elements in the set A.
    pub fn num_elements(&self) -> usize {
        self.num_elements
    }

    /// Number of cyclic ordering triples.
    pub fn num_triples(&self) -> usize {
        self.triples.len()
    }

    /// The collection of ordered triples.
    pub fn triples(&self) -> &[(usize, usize, usize)] {
        &self.triples
    }

    /// Check whether a configuration represents a valid permutation and
    /// satisfies all cyclic ordering constraints.
    fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.num_elements {
            return false;
        }

        // Check that config is a valid permutation of 0..n
        let n = self.num_elements;
        let mut seen = vec![false; n];
        for &pos in config {
            if pos >= n || seen[pos] {
                return false;
            }
            seen[pos] = true;
        }

        // Check cyclic ordering constraints: for each (a, b, c),
        // (fa < fb < fc) OR (fb < fc < fa) OR (fc < fa < fb)
        for &(a, b, c) in &self.triples {
            if !is_cyclic_order(config[a], config[b], config[c]) {
                return false;
            }
        }

        true
    }
}

/// Check whether three distinct values appear in cyclic order:
/// (a < b < c) OR (b < c < a) OR (c < a < b).
#[allow(clippy::nonminimal_bool)]
fn is_cyclic_order(a: usize, b: usize, c: usize) -> bool {
    (a < b && b < c) || (b < c && c < a) || (c < a && a < b)
}

#[derive(Deserialize)]
struct CyclicOrderingData {
    num_elements: usize,
    triples: Vec<(usize, usize, usize)>,
}

impl<'de> Deserialize<'de> for CyclicOrdering {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = CyclicOrderingData::deserialize(deserializer)?;
        Self::try_new(data.num_elements, data.triples).map_err(D::Error::custom)
    }
}

impl Problem for CyclicOrdering {
    const NAME: &'static str = "CyclicOrdering";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_elements; self.num_elements]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or(self.is_valid_solution(config))
    }
}

crate::declare_variants! {
    default CyclicOrdering => "factorial(num_elements)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "cyclic_ordering",
        instance: Box::new(CyclicOrdering::new(
            5,
            vec![(0, 1, 2), (2, 3, 0), (1, 3, 4)],
        )),
        optimal_config: vec![1, 3, 4, 0, 2],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/cyclic_ordering.rs"]
mod tests;
