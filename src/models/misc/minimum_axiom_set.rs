//! Minimum Axiom Set problem implementation.
//!
//! Given a finite set of sentences S, a subset T ⊆ S of true sentences, and a set
//! of implications (where each implication has a set of antecedent sentences and a
//! single consequent sentence), find a smallest subset S₀ ⊆ T such that the
//! deductive closure of S₀ under the implications equals T.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumAxiomSet",
        display_name: "Minimum Axiom Set",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find smallest axiom subset whose deductive closure equals the true sentences",
        fields: &[
            FieldInfo { name: "num_sentences", type_name: "usize", description: "Total number of sentences |S|" },
            FieldInfo { name: "true_sentences", type_name: "Vec<usize>", description: "Indices of true sentences T ⊆ S" },
            FieldInfo { name: "implications", type_name: "Vec<(Vec<usize>, usize)>", description: "Implication rules (antecedents, consequent)" },
        ],
    }
}

/// The Minimum Axiom Set problem.
///
/// Given a set of sentences `S = {0, ..., num_sentences - 1}`, a subset
/// `T ⊆ S` of true sentences, and a list of implications where each
/// implication `(A, c)` means "if all sentences in A hold, then c holds",
/// find a smallest subset `S₀ ⊆ T` whose deductive closure under the
/// implications equals `T`.
///
/// # Representation
///
/// Each true sentence has a binary variable: `config[i] = 1` if
/// `true_sentences[i]` is selected as an axiom, `0` otherwise.
/// The configuration space is `vec![2; |T|]`.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::MinimumAxiomSet;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 8 sentences, all true, with implications forming a cycle
/// let problem = MinimumAxiomSet::new(
///     8,
///     vec![0, 1, 2, 3, 4, 5, 6, 7],
///     vec![
///         (vec![0], 2), (vec![0], 3),
///         (vec![1], 4), (vec![1], 5),
///         (vec![2, 4], 6), (vec![3, 5], 7),
///         (vec![6, 7], 0), (vec![6, 7], 1),
///     ],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumAxiomSet {
    /// Total number of sentences |S|.
    num_sentences: usize,
    /// Indices of true sentences T ⊆ S.
    true_sentences: Vec<usize>,
    /// Implication rules: each (antecedents, consequent).
    implications: Vec<(Vec<usize>, usize)>,
}

impl MinimumAxiomSet {
    /// Create a new Minimum Axiom Set instance.
    ///
    /// # Panics
    ///
    /// Panics if any true sentence index is out of range,
    /// if true sentences contain duplicates,
    /// or if any implication references a sentence outside S.
    pub fn new(
        num_sentences: usize,
        true_sentences: Vec<usize>,
        implications: Vec<(Vec<usize>, usize)>,
    ) -> Self {
        // Validate true sentences
        for &s in &true_sentences {
            assert!(
                s < num_sentences,
                "True sentence index {s} out of range [0, {num_sentences})"
            );
        }
        // Check no duplicates
        let mut seen = vec![false; num_sentences];
        for &s in &true_sentences {
            assert!(!seen[s], "Duplicate true sentence index {s}");
            seen[s] = true;
        }
        // Validate implications
        for (antecedents, consequent) in &implications {
            for &a in antecedents {
                assert!(
                    a < num_sentences,
                    "Implication antecedent {a} out of range [0, {num_sentences})"
                );
            }
            assert!(
                *consequent < num_sentences,
                "Implication consequent {consequent} out of range [0, {num_sentences})"
            );
        }
        Self {
            num_sentences,
            true_sentences,
            implications,
        }
    }

    /// Returns the total number of sentences |S|.
    pub fn num_sentences(&self) -> usize {
        self.num_sentences
    }

    /// Returns the number of true sentences |T|.
    pub fn num_true_sentences(&self) -> usize {
        self.true_sentences.len()
    }

    /// Returns the number of implications.
    pub fn num_implications(&self) -> usize {
        self.implications.len()
    }

    /// Returns the true sentence indices.
    pub fn true_sentences(&self) -> &[usize] {
        &self.true_sentences
    }

    /// Returns the implications.
    pub fn implications(&self) -> &[(Vec<usize>, usize)] {
        &self.implications
    }
}

/// Compute the deductive closure of a set of sentences under the given implications.
///
/// Starting from `current`, repeatedly applies implications until a fixpoint.
fn deductive_closure(current: &mut [bool], implications: &[(Vec<usize>, usize)]) {
    loop {
        let mut changed = false;
        for (antecedents, consequent) in implications {
            if !current[*consequent] && antecedents.iter().all(|&a| current[a]) {
                current[*consequent] = true;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

impl Problem for MinimumAxiomSet {
    const NAME: &'static str = "MinimumAxiomSet";
    type Value = Min<usize>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_true_sentences()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<usize> {
        if config.len() != self.num_true_sentences() {
            return Min(None);
        }
        if config.iter().any(|&v| v >= 2) {
            return Min(None);
        }

        // Build the initial set of selected axioms
        let mut current = vec![false; self.num_sentences];
        let mut count = 0usize;
        for (i, &v) in config.iter().enumerate() {
            if v == 1 {
                current[self.true_sentences[i]] = true;
                count += 1;
            }
        }

        // Compute deductive closure
        deductive_closure(&mut current, &self.implications);

        // Check if closure equals T
        let closure_equals_t = self.true_sentences.iter().all(|&s| current[s]);

        if closure_equals_t {
            Min(Some(count))
        } else {
            Min(None)
        }
    }
}

crate::declare_variants! {
    default MinimumAxiomSet => "2^num_true_sentences",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 8 sentences, all true, with implications forming a cycle
    // Optimal: select {a, b} (indices 0, 1) → closure = all 8
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_axiom_set",
        instance: Box::new(MinimumAxiomSet::new(
            8,
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![
                (vec![0], 2),
                (vec![0], 3),
                (vec![1], 4),
                (vec![1], 5),
                (vec![2, 4], 6),
                (vec![3, 5], 7),
                (vec![6, 7], 0),
                (vec![6, 7], 1),
            ],
        )),
        optimal_config: vec![1, 1, 0, 0, 0, 0, 0, 0],
        optimal_value: serde_json::json!(2),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/minimum_axiom_set.rs"]
mod tests;
