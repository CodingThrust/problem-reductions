//! Satisfiability problems.
//!
//! This module contains Boolean satisfiability problems:
//! - [`Satisfiability`]: Boolean satisfiability (SAT) with CNF clauses
//! - [`KSatisfiability`]: K-SAT where each clause has exactly K literals

mod ksat;
mod sat;

pub use ksat::KSatisfiability;
pub use sat::{CNFClause, Satisfiability};

// Validation utilities
pub use sat::is_satisfying_assignment;
