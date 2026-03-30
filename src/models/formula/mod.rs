//! Logic and formula problems.
//!
//! Problems whose input is a boolean formula or circuit:
//! - [`Satisfiability`]: Boolean satisfiability (SAT) with CNF clauses
//! - [`NAESatisfiability`]: Not-All-Equal satisfiability with CNF clauses
//! - [`KSatisfiability`]: K-SAT where each clause has exactly K literals
//! - [`NonTautology`]: Find a falsifying assignment for a DNF formula
//! - [`OneInThreeSatisfiability`]: Exactly one literal true per clause (1-in-3 SAT)
//! - [`Planar3Satisfiability`]: 3-SAT restricted to planar variable-clause incidence graphs
//! - [`CircuitSAT`]: Boolean circuit satisfiability
//! - [`QuantifiedBooleanFormulas`]: Quantified Boolean Formulas (QBF) — PSPACE-complete

pub(crate) mod circuit;
pub(crate) mod ksat;
pub(crate) mod nae_satisfiability;
pub(crate) mod non_tautology;
pub(crate) mod one_in_three_satisfiability;
pub(crate) mod planar_3_satisfiability;
pub(crate) mod qbf;
pub(crate) mod sat;

pub use circuit::{Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use ksat::KSatisfiability;
pub use nae_satisfiability::NAESatisfiability;
pub use non_tautology::NonTautology;
pub use one_in_three_satisfiability::OneInThreeSatisfiability;
pub use planar_3_satisfiability::Planar3Satisfiability;
pub use qbf::{QuantifiedBooleanFormulas, Quantifier};
pub use sat::{CNFClause, Satisfiability};

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(sat::canonical_model_example_specs());
    specs.extend(nae_satisfiability::canonical_model_example_specs());
    specs.extend(ksat::canonical_model_example_specs());
    specs.extend(circuit::canonical_model_example_specs());
    specs.extend(non_tautology::canonical_model_example_specs());
    specs.extend(one_in_three_satisfiability::canonical_model_example_specs());
    specs.extend(planar_3_satisfiability::canonical_model_example_specs());
    specs.extend(qbf::canonical_model_example_specs());
    specs
}
