//! Satisfiability problems.
//!
//! This module contains Boolean satisfiability problems:
//! - [`Satisfiability`]: Boolean satisfiability (SAT) with CNF clauses

mod sat;

pub use sat::{CNFClause, Satisfiability};

// Validation utilities
pub use sat::is_satisfying_assignment;
