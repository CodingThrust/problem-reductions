//! Conjunctive Boolean Query problem implementation.
//!
//! Given a finite domain `D = {0, ..., domain_size-1}`, a collection of
//! relations `R`, and a conjunctive Boolean query
//! `Q = (exists y_1, ..., y_l)(A_1 /\ ... /\ A_r)`, determine whether `Q` is
//! true over `R` and `D`.
//!
//! Each conjunct `A_i` applies a relation to a tuple of arguments, where each
//! argument is either an existentially quantified variable or a constant from
//! the domain. The query is satisfiable iff there exists an assignment to the
//! variables such that every conjunct's resolved tuple belongs to its relation.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConjunctiveBooleanQuery",
        display_name: "Conjunctive Boolean Query",
        aliases: &["CBQ"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Evaluate a conjunctive Boolean query over a relational database",
        fields: &[
            FieldInfo { name: "domain_size", type_name: "usize", description: "Size of the finite domain D" },
            FieldInfo { name: "relations", type_name: "Vec<Relation>", description: "Collection of relations R" },
            FieldInfo { name: "num_variables", type_name: "usize", description: "Number of existentially quantified variables" },
            FieldInfo { name: "conjuncts", type_name: "Vec<(usize, Vec<QueryArg>)>", description: "Query conjuncts: (relation_index, arguments)" },
        ],
    }
}

/// A relation with fixed arity and a set of tuples over a finite domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    /// The arity (number of columns) of this relation.
    pub arity: usize,
    /// The set of tuples; each tuple has length == arity, entries in `0..domain_size`.
    pub tuples: Vec<Vec<usize>>,
}

/// An argument in a conjunctive query atom.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryArg {
    /// A reference to existential variable `y_i` (0-indexed).
    Variable(usize),
    /// A constant value from the domain `D`.
    Constant(usize),
}

/// The Conjunctive Boolean Query problem.
///
/// Given a finite domain `D = {0, ..., domain_size-1}`, a collection of
/// relations `R`, and a conjunctive Boolean query
/// `Q = (exists y_1, ..., y_l)(A_1 /\ ... /\ A_r)`, determine whether `Q` is
/// true over `R` and `D`.
///
/// # Representation
///
/// The configuration is a vector of length `num_variables`, where each entry is
/// a value in `{0, ..., domain_size-1}` representing an assignment to the
/// existentially quantified variables.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::{ConjunctiveBooleanQuery, CbqRelation, QueryArg};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// let relations = vec![
///     CbqRelation { arity: 2, tuples: vec![vec![0, 3], vec![1, 3]] },
/// ];
/// let conjuncts = vec![
///     (0, vec![QueryArg::Variable(0), QueryArg::Constant(3)]),
/// ];
/// let problem = ConjunctiveBooleanQuery::new(6, relations, 1, conjuncts);
/// let solver = BruteForce::new();
/// let solution = solver.find_satisfying(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConjunctiveBooleanQuery {
    domain_size: usize,
    relations: Vec<Relation>,
    num_variables: usize,
    conjuncts: Vec<(usize, Vec<QueryArg>)>,
}

impl ConjunctiveBooleanQuery {
    /// Create a new ConjunctiveBooleanQuery instance.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - Any relation's tuples have incorrect arity
    /// - Any tuple entry is >= domain_size
    /// - Any conjunct references a non-existent relation
    /// - Any `Variable(i)` has `i >= num_variables`
    /// - Any `Constant(c)` has `c >= domain_size`
    /// - Any conjunct's argument count does not match the referenced relation's arity
    pub fn new(
        domain_size: usize,
        relations: Vec<Relation>,
        num_variables: usize,
        conjuncts: Vec<(usize, Vec<QueryArg>)>,
    ) -> Self {
        for (i, rel) in relations.iter().enumerate() {
            for (j, tuple) in rel.tuples.iter().enumerate() {
                assert!(
                    tuple.len() == rel.arity,
                    "Relation {i}: tuple {j} has length {}, expected arity {}",
                    tuple.len(),
                    rel.arity
                );
                for (k, &val) in tuple.iter().enumerate() {
                    assert!(
                        val < domain_size,
                        "Relation {i}: tuple {j}, entry {k} is {val}, must be < {domain_size}"
                    );
                }
            }
        }
        for (i, (rel_idx, args)) in conjuncts.iter().enumerate() {
            assert!(
                *rel_idx < relations.len(),
                "Conjunct {i}: relation index {rel_idx} out of range (have {} relations)",
                relations.len()
            );
            assert!(
                args.len() == relations[*rel_idx].arity,
                "Conjunct {i}: has {} args, expected arity {}",
                args.len(),
                relations[*rel_idx].arity
            );
            for (k, arg) in args.iter().enumerate() {
                match arg {
                    QueryArg::Variable(v) => {
                        assert!(
                            *v < num_variables,
                            "Conjunct {i}, arg {k}: Variable({v}) >= num_variables ({num_variables})"
                        );
                    }
                    QueryArg::Constant(c) => {
                        assert!(
                            *c < domain_size,
                            "Conjunct {i}, arg {k}: Constant({c}) >= domain_size ({domain_size})"
                        );
                    }
                }
            }
        }
        Self {
            domain_size,
            relations,
            num_variables,
            conjuncts,
        }
    }

    /// Returns the size of the finite domain.
    pub fn domain_size(&self) -> usize {
        self.domain_size
    }

    /// Returns the number of relations.
    pub fn num_relations(&self) -> usize {
        self.relations.len()
    }

    /// Returns the number of existentially quantified variables.
    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    /// Returns the number of conjuncts in the query.
    pub fn num_conjuncts(&self) -> usize {
        self.conjuncts.len()
    }

    /// Returns the relations.
    pub fn relations(&self) -> &[Relation] {
        &self.relations
    }

    /// Returns the conjuncts.
    pub fn conjuncts(&self) -> &[(usize, Vec<QueryArg>)] {
        &self.conjuncts
    }
}

impl Problem for ConjunctiveBooleanQuery {
    const NAME: &'static str = "ConjunctiveBooleanQuery";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.domain_size; self.num_variables]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_variables {
            return false;
        }
        if config.iter().any(|&v| v >= self.domain_size) {
            return false;
        }
        self.conjuncts.iter().all(|(rel_idx, args)| {
            let tuple: Vec<usize> = args
                .iter()
                .map(|arg| match arg {
                    QueryArg::Variable(i) => config[*i],
                    QueryArg::Constant(c) => *c,
                })
                .collect();
            self.relations[*rel_idx].tuples.contains(&tuple)
        })
    }
}

impl SatisfactionProblem for ConjunctiveBooleanQuery {}

crate::declare_variants! {
    default sat ConjunctiveBooleanQuery => "domain_size ^ num_variables",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "conjunctive_boolean_query",
        // D={0..5}, 2 relations (binary R0, ternary R1), 3 atoms, 2 variables.
        // Satisfying assignment: y0=0, y1=1.
        instance: Box::new(ConjunctiveBooleanQuery::new(
            6,
            vec![
                Relation {
                    arity: 2,
                    tuples: vec![vec![0, 3], vec![1, 3], vec![2, 4], vec![3, 4], vec![4, 5]],
                },
                Relation {
                    arity: 3,
                    tuples: vec![vec![0, 1, 5], vec![1, 2, 5], vec![2, 3, 4], vec![0, 4, 3]],
                },
            ],
            2,
            vec![
                (0, vec![QueryArg::Variable(0), QueryArg::Constant(3)]),
                (0, vec![QueryArg::Variable(1), QueryArg::Constant(3)]),
                (
                    1,
                    vec![
                        QueryArg::Variable(0),
                        QueryArg::Variable(1),
                        QueryArg::Constant(5),
                    ],
                ),
            ],
        )),
        optimal_config: vec![0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/conjunctive_boolean_query.rs"]
mod tests;
