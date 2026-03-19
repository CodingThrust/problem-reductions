//! Conjunctive Query Foldability problem implementation.
//!
//! Given two conjunctive queries Q1 and Q2 over a finite domain with relations,
//! the problem asks whether there exists a substitution of undistinguished variables
//! that transforms Q1 into Q2. NP-complete (Chandra & Merlin, 1977).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConjunctiveQueryFoldability",
        display_name: "Conjunctive Query Foldability",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine if one conjunctive query can be folded into another by substituting undistinguished variables",
        fields: &[
            FieldInfo { name: "domain_size", type_name: "usize", description: "Size of the finite domain D" },
            FieldInfo { name: "num_distinguished", type_name: "usize", description: "Number of distinguished variables X" },
            FieldInfo { name: "num_undistinguished", type_name: "usize", description: "Number of undistinguished variables Y" },
            FieldInfo { name: "relation_arities", type_name: "Vec<usize>", description: "Arity of each relation" },
            FieldInfo { name: "query1_conjuncts", type_name: "Vec<(usize, Vec<Term>)>", description: "Atoms of query Q1" },
            FieldInfo { name: "query2_conjuncts", type_name: "Vec<(usize, Vec<Term>)>", description: "Atoms of query Q2" },
        ],
    }
}

/// A term in a conjunctive query atom.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "index")]
pub enum Term {
    /// A domain constant `D[i]`.
    Constant(usize),
    /// A distinguished variable `X[i]`.
    Distinguished(usize),
    /// An undistinguished variable `Y[i]`.
    Undistinguished(usize),
}

/// The Conjunctive Query Foldability problem.
///
/// Given a finite domain `D`, a set of relation symbols with fixed arities,
/// distinguished variables `X`, undistinguished variables `Y`, and two
/// conjunctive queries Q1 and Q2 (over `X ∪ Y ∪ D`), this problem asks:
/// does there exist a substitution `σ: Y → X ∪ Y ∪ D` that maps every atom
/// of Q1 to an atom of Q2?
///
/// This is equivalent to the *query containment* problem for conjunctive queries
/// and is NP-complete (Chandra & Merlin, 1977; Garey & Johnson A4 SR30).
///
/// # Representation
///
/// - Each undistinguished variable `Y[i]` is a configuration variable whose
///   value encodes its substitution target:
///   - `0..domain_size` → `Constant(v)`
///   - `domain_size..domain_size+num_distinguished` → `Distinguished(v - domain_size)`
///   - `domain_size+num_distinguished..` → `Undistinguished(v - domain_size - num_distinguished)`
/// - The problem is satisfiable iff applying `σ` to all atoms of Q1 yields
///   exactly the multiset (treated as a set) of atoms in Q2.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::{ConjunctiveQueryFoldability, Term};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Q1: R(x, u) ∧ R(u, u)    Q2: R(x, x)  (single atom; duplicates are irrelevant)
/// // σ: u → x (index = domain_size + 0 = 1) folds Q1 to Q2
/// let problem = ConjunctiveQueryFoldability::new(
///     0, 1, 1,
///     vec![2],
///     vec![
///         (0, vec![Term::Distinguished(0), Term::Undistinguished(0)]),
///         (0, vec![Term::Undistinguished(0), Term::Undistinguished(0)]),
///     ],
///     vec![
///         (0, vec![Term::Distinguished(0), Term::Distinguished(0)]),
///     ],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjunctiveQueryFoldability {
    /// Size of the finite domain D.
    domain_size: usize,
    /// Number of distinguished variables X.
    num_distinguished: usize,
    /// Number of undistinguished variables Y.
    num_undistinguished: usize,
    /// Arity of each relation symbol.
    relation_arities: Vec<usize>,
    /// Atoms of query Q1: each atom is `(relation_index, argument_list)`.
    query1_conjuncts: Vec<(usize, Vec<Term>)>,
    /// Atoms of query Q2: each atom is `(relation_index, argument_list)`.
    query2_conjuncts: Vec<(usize, Vec<Term>)>,
}

impl ConjunctiveQueryFoldability {
    /// Create a new `ConjunctiveQueryFoldability` instance.
    ///
    /// # Arguments
    ///
    /// * `domain_size` – Number of domain constants `|D|`.
    /// * `num_distinguished` – Number of distinguished variables `|X|`.
    /// * `num_undistinguished` – Number of undistinguished variables `|Y|`.
    /// * `relation_arities` – Arity of each relation symbol.
    /// * `query1_conjuncts` – Atoms of Q1 as `(relation_index, args)` pairs.
    /// * `query2_conjuncts` – Atoms of Q2 as `(relation_index, args)` pairs.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Any atom references a relation index out of range.
    /// - Any atom has the wrong number of arguments for its relation's arity.
    /// - Any `Constant(i)` has `i >= domain_size`.
    /// - Any `Distinguished(i)` has `i >= num_distinguished`.
    /// - Any `Undistinguished(i)` has `i >= num_undistinguished`.
    pub fn new(
        domain_size: usize,
        num_distinguished: usize,
        num_undistinguished: usize,
        relation_arities: Vec<usize>,
        query1_conjuncts: Vec<(usize, Vec<Term>)>,
        query2_conjuncts: Vec<(usize, Vec<Term>)>,
    ) -> Self {
        let instance = Self {
            domain_size,
            num_distinguished,
            num_undistinguished,
            relation_arities,
            query1_conjuncts,
            query2_conjuncts,
        };
        instance.validate();
        instance
    }

    /// Validate the instance, panicking on any inconsistency.
    fn validate(&self) {
        for (query_name, conjuncts) in [
            ("Q1", &self.query1_conjuncts),
            ("Q2", &self.query2_conjuncts),
        ] {
            for (atom_idx, (rel_idx, args)) in conjuncts.iter().enumerate() {
                assert!(
                    *rel_idx < self.relation_arities.len(),
                    "Atom {atom_idx} of {query_name}: relation index {rel_idx} out of range \
                     (num_relations = {})",
                    self.relation_arities.len()
                );
                let arity = self.relation_arities[*rel_idx];
                assert_eq!(
                    args.len(),
                    arity,
                    "Atom {atom_idx} of {query_name}: relation {rel_idx} has arity {arity} \
                     but got {} arguments",
                    args.len()
                );
                for term in args {
                    match term {
                        Term::Constant(i) => assert!(
                            *i < self.domain_size,
                            "Atom {atom_idx} of {query_name}: Constant({i}) out of range \
                             (domain_size = {})",
                            self.domain_size
                        ),
                        Term::Distinguished(i) => assert!(
                            *i < self.num_distinguished,
                            "Atom {atom_idx} of {query_name}: Distinguished({i}) out of range \
                             (num_distinguished = {})",
                            self.num_distinguished
                        ),
                        Term::Undistinguished(i) => assert!(
                            *i < self.num_undistinguished,
                            "Atom {atom_idx} of {query_name}: Undistinguished({i}) out of range \
                             (num_undistinguished = {})",
                            self.num_undistinguished
                        ),
                    }
                }
            }
        }
    }

    /// Returns the size of the finite domain D.
    pub fn domain_size(&self) -> usize {
        self.domain_size
    }

    /// Returns the number of distinguished variables X.
    pub fn num_distinguished(&self) -> usize {
        self.num_distinguished
    }

    /// Returns the number of undistinguished variables Y.
    pub fn num_undistinguished(&self) -> usize {
        self.num_undistinguished
    }

    /// Returns the number of conjuncts (atoms) in Q1.
    pub fn num_conjuncts_q1(&self) -> usize {
        self.query1_conjuncts.len()
    }

    /// Returns the number of conjuncts (atoms) in Q2.
    pub fn num_conjuncts_q2(&self) -> usize {
        self.query2_conjuncts.len()
    }

    /// Returns the number of relation symbols.
    pub fn num_relations(&self) -> usize {
        self.relation_arities.len()
    }

    /// Returns the arities of the relation symbols.
    pub fn relation_arities(&self) -> &[usize] {
        &self.relation_arities
    }

    /// Returns the atoms (conjuncts) of query Q1.
    pub fn query1_conjuncts(&self) -> &[(usize, Vec<Term>)] {
        &self.query1_conjuncts
    }

    /// Returns the atoms (conjuncts) of query Q2.
    pub fn query2_conjuncts(&self) -> &[(usize, Vec<Term>)] {
        &self.query2_conjuncts
    }

    /// Decode a config index into the [`Term`] it represents under σ.
    ///
    /// The mapping is:
    /// - `0..domain_size` → `Constant(v)`
    /// - `domain_size..domain_size+num_distinguished` → `Distinguished(v - domain_size)`
    /// - `domain_size+num_distinguished..` → `Undistinguished(v - domain_size - num_distinguished)`
    fn decode_substitution(&self, v: usize) -> Term {
        if v < self.domain_size {
            Term::Constant(v)
        } else if v < self.domain_size + self.num_distinguished {
            Term::Distinguished(v - self.domain_size)
        } else {
            Term::Undistinguished(v - self.domain_size - self.num_distinguished)
        }
    }

    /// Apply substitution `σ` (given as a slice of config values) to a single term.
    ///
    /// Distinguished variables and constants are left unchanged; undistinguished
    /// variable `Y[i]` is replaced by `decode_substitution(config[i])`.
    fn apply_substitution(&self, term: &Term, config: &[usize]) -> Term {
        match term {
            Term::Undistinguished(i) => self.decode_substitution(config[*i]),
            other => other.clone(),
        }
    }
}

impl Problem for ConjunctiveQueryFoldability {
    const NAME: &'static str = "ConjunctiveQueryFoldability";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    /// Returns the configuration space dimensions.
    ///
    /// Each of the `num_undistinguished` variables can map to any element of
    /// `D ∪ X ∪ Y`, giving `domain_size + num_distinguished + num_undistinguished`
    /// choices per variable.  When `num_undistinguished == 0` the vector is empty
    /// (Q1 contains no variables to substitute; the problem is trivially decided
    /// by checking set equality of Q1 and Q2 at evaluation time).
    fn dims(&self) -> Vec<usize> {
        let range = self.domain_size + self.num_distinguished + self.num_undistinguished;
        vec![range; self.num_undistinguished]
    }

    /// Evaluate whether configuration `config` represents a folding of Q1 into Q2.
    ///
    /// Returns `true` iff applying the substitution encoded by `config` to every
    /// atom of Q1 produces exactly the set of atoms in Q2.
    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_undistinguished {
            return false;
        }
        let range = self.domain_size + self.num_distinguished + self.num_undistinguished;
        if config.iter().any(|&v| v >= range) {
            return false;
        }

        // Apply σ to every atom of Q1.
        let substituted: HashSet<(usize, Vec<Term>)> = self
            .query1_conjuncts
            .iter()
            .map(|(rel_idx, args)| {
                let new_args = args
                    .iter()
                    .map(|term| self.apply_substitution(term, config))
                    .collect();
                (*rel_idx, new_args)
            })
            .collect();

        // Collect Q2 as a set.
        let q2_set: HashSet<(usize, Vec<Term>)> = self.query2_conjuncts.iter().cloned().collect();

        substituted == q2_set
    }
}

impl SatisfactionProblem for ConjunctiveQueryFoldability {}

crate::declare_variants! {
    default sat ConjunctiveQueryFoldability => "(num_distinguished + num_undistinguished + domain_size)^num_undistinguished * num_conjuncts_q1",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // YES instance: triangle + self-loop folds to lollipop.
    //
    // Q1: R(x, u) ∧ R(u, v) ∧ R(v, x) ∧ R(u, u)
    // Q2: R(x, a) ∧ R(a, a) ∧ R(a, x)
    //
    // The substitution σ: U(0) → U(2), U(1) → U(2), U(2) → U(2)
    // maps Q1 → Q2 (as a set). Config = [3, 3, 3].
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "conjunctive_query_foldability",
        instance: Box::new(ConjunctiveQueryFoldability::new(
            0,       // domain_size
            1,       // num_distinguished (x)
            3,       // num_undistinguished (u, v, a)
            vec![2], // one binary relation R
            vec![
                (0, vec![Term::Distinguished(0), Term::Undistinguished(0)]), // R(x, u)
                (0, vec![Term::Undistinguished(0), Term::Undistinguished(1)]), // R(u, v)
                (0, vec![Term::Undistinguished(1), Term::Distinguished(0)]), // R(v, x)
                (0, vec![Term::Undistinguished(0), Term::Undistinguished(0)]), // R(u, u)
            ],
            vec![
                (0, vec![Term::Distinguished(0), Term::Undistinguished(2)]), // R(x, a)
                (0, vec![Term::Undistinguished(2), Term::Undistinguished(2)]), // R(a, a)
                (0, vec![Term::Undistinguished(2), Term::Distinguished(0)]), // R(a, x)
            ],
        )),
        optimal_config: vec![3, 3, 3],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/conjunctive_query_foldability.rs"]
mod tests;
