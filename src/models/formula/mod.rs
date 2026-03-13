//! Logic and formula problems.
//!
//! Problems whose input is a boolean formula or circuit:
//! - [`Satisfiability`]: Boolean satisfiability (SAT) with CNF clauses
//! - [`KSatisfiability`]: K-SAT where each clause has exactly K literals
//! - [`CircuitSAT`]: Boolean circuit satisfiability
//! - [`QuantifiedBooleanFormulas`]: Quantified Boolean Formulas (QBF) — PSPACE-complete

pub(crate) mod circuit;
mod ksat;
pub(crate) mod qbf;
mod sat;

pub use circuit::{Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use ksat::KSatisfiability;
pub use qbf::{QuantifiedBooleanFormulas, Quantifier};
pub use sat::{CNFClause, Satisfiability};
