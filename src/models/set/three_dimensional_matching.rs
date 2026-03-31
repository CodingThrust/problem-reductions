//! Three-Dimensional Matching (3DM) problem implementation.
//!
//! Given disjoint sets W, X, Y each with q elements and a set M of triples
//! (w, x, y) with w in W, x in X, y in Y, determine if there exists a
//! matching M' of size q where no two triples agree in any coordinate.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ThreeDimensionalMatching",
        display_name: "Three-Dimensional Matching",
        aliases: &["3DM"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a perfect matching in a tripartite hypergraph",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Size of each set W, X, Y (q)" },
            FieldInfo { name: "triples", type_name: "Vec<(usize, usize, usize)>", description: "Set M of triples (w, x, y)" },
        ],
    }
}

/// Three-Dimensional Matching (3DM) problem.
///
/// Given disjoint sets W = {0, ..., q-1}, X = {0, ..., q-1}, Y = {0, ..., q-1}
/// and a set M of triples (w, x, y) where w is in W, x is in X, y is in Y,
/// determine if there exists a subset M' of M with |M'| = q such that no two
/// triples in M' agree in any coordinate.
///
/// This is a classical NP-complete problem (Karp, 1972), closely related to
/// Exact Cover by 3-Sets.
///
/// # Example
///
/// ```
/// use problemreductions::models::set::ThreeDimensionalMatching;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // W = X = Y = {0, 1, 2} (q = 3)
/// // Triples: (0,1,2), (1,0,1), (2,2,0), (0,0,0), (1,2,2)
/// let problem = ThreeDimensionalMatching::new(
///     3,
///     vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
/// );
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem);
///
/// // First three triples form a valid matching
/// assert!(!solutions.is_empty());
/// assert!(problem.evaluate(&solutions[0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDimensionalMatching {
    /// Size of each set W, X, Y (elements are 0..universe_size).
    universe_size: usize,
    /// Set M of triples (w, x, y) where w, x, y are in 0..universe_size.
    triples: Vec<(usize, usize, usize)>,
}

impl ThreeDimensionalMatching {
    /// Create a new 3DM problem.
    ///
    /// # Panics
    ///
    /// Panics if any triple contains an element outside 0..universe_size.
    pub fn new(universe_size: usize, triples: Vec<(usize, usize, usize)>) -> Self {
        for (i, &(w, x, y)) in triples.iter().enumerate() {
            assert!(
                w < universe_size,
                "Triple {} has w-coordinate {} which is outside 0..{}",
                i,
                w,
                universe_size
            );
            assert!(
                x < universe_size,
                "Triple {} has x-coordinate {} which is outside 0..{}",
                i,
                x,
                universe_size
            );
            assert!(
                y < universe_size,
                "Triple {} has y-coordinate {} which is outside 0..{}",
                i,
                y,
                universe_size
            );
        }
        Self {
            universe_size,
            triples,
        }
    }

    /// Get the universe size (q).
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    /// Get the number of triples in M.
    pub fn num_triples(&self) -> usize {
        self.triples.len()
    }

    /// Get the triples.
    pub fn triples(&self) -> &[(usize, usize, usize)] {
        &self.triples
    }

    /// Get a specific triple.
    pub fn get_triple(&self, index: usize) -> Option<&(usize, usize, usize)> {
        self.triples.get(index)
    }
}

impl Problem for ThreeDimensionalMatching {
    const NAME: &'static str = "ThreeDimensionalMatching";
    type Value = crate::types::Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.triples.len()]
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Or {
        crate::types::Or({
            if config.len() != self.triples.len() || config.iter().any(|&value| value > 1) {
                return crate::types::Or(false);
            }

            // Count selected triples
            let selected_count: usize = config.iter().filter(|&&v| v == 1).sum();
            if selected_count != self.universe_size {
                return crate::types::Or(false);
            }

            // Check that selected triples have all distinct coordinates
            let mut used_w = HashSet::with_capacity(self.universe_size);
            let mut used_x = HashSet::with_capacity(self.universe_size);
            let mut used_y = HashSet::with_capacity(self.universe_size);

            for (i, &selected) in config.iter().enumerate() {
                if selected == 1 {
                    let (w, x, y) = self.triples[i];
                    if !used_w.insert(w) {
                        return crate::types::Or(false);
                    }
                    if !used_x.insert(x) {
                        return crate::types::Or(false);
                    }
                    if !used_y.insert(y) {
                        return crate::types::Or(false);
                    }
                }
            }

            true
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default ThreeDimensionalMatching => "2^num_triples",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "three_dimensional_matching",
        instance: Box::new(ThreeDimensionalMatching::new(
            3,
            vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
        )),
        optimal_config: vec![1, 1, 1, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/three_dimensional_matching.rs"]
mod tests;
