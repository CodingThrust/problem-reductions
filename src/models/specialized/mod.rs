//! Specialized NP-hard problems.
//!
//! This module contains problems that don't fit neatly into other categories:
//! - [`CircuitSAT`]: Boolean circuit satisfiability
//! - [`Factoring`]: Integer factorization
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`BicliqueCover`]: Biclique cover on bipartite graphs
//! - [`BMF`]: Boolean matrix factorization

pub(crate) mod biclique_cover;
pub(crate) mod bmf;
pub(crate) mod circuit;
pub(crate) mod factoring;
pub(crate) mod paintshop;

pub use biclique_cover::BicliqueCover;
pub use bmf::BMF;
pub use circuit::{Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use factoring::Factoring;
pub use paintshop::PaintShop;
