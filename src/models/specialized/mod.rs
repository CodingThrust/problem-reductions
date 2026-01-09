//! Specialized NP-hard problems.
//!
//! This module contains problems that don't fit neatly into other categories:
//! - [`CircuitSAT`]: Boolean circuit satisfiability
//! - [`Factoring`]: Integer factorization
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`BicliqueCover`]: Biclique cover on bipartite graphs
//! - [`BMF`]: Boolean matrix factorization

mod biclique_cover;
mod bmf;
mod circuit;
mod factoring;
mod paintshop;

pub use biclique_cover::{is_biclique_cover, BicliqueCover};
pub use bmf::{boolean_matrix_product, matrix_hamming_distance, BMF};
pub use circuit::{is_circuit_satisfying, Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use factoring::{is_factoring, Factoring};
pub use paintshop::{count_paint_switches, PaintShop};
