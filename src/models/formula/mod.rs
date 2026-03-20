//! Logic and formula problems.
//!
//! Problems whose input is a boolean formula or circuit:
//! - [`Satisfiability`]: Boolean satisfiability (SAT) with CNF clauses
//! - [`KSatisfiability`]: K-SAT where each clause has exactly K literals
//! - [`CircuitSAT`]: Boolean circuit satisfiability
//! - [`QuantifiedBooleanFormulas`]: Quantified Boolean Formulas (QBF) — PSPACE-complete

pub(crate) mod circuit;
pub(crate) mod ksat;
pub(crate) mod qbf;
pub(crate) mod sat;

pub use circuit::{Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use ksat::KSatisfiability;
pub use qbf::{QuantifiedBooleanFormulas, Quantifier};
pub use sat::{CNFClause, Satisfiability};

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(sat::canonical_model_example_specs());
    specs.extend(ksat::canonical_model_example_specs());
    specs.extend(circuit::canonical_model_example_specs());
    specs
}
