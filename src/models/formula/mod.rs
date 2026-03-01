//! Logic and formula problems.
//!
//! Problems whose input is a boolean formula or circuit:
//! - [`Satisfiability`]: Boolean satisfiability (SAT) with CNF clauses
//! - [`KSatisfiability`]: K-SAT where each clause has exactly K literals
//! - [`CircuitSAT`]: Boolean circuit satisfiability

pub(crate) mod circuit;
mod ksat;
mod sat;

pub use circuit::{Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use ksat::KSatisfiability;
pub use sat::{CNFClause, Satisfiability};
