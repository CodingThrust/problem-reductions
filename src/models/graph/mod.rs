//! Graph-based optimization problems.
//!
//! This module contains NP-hard problems defined on graphs:
//! - [`IndependentSet`]: Maximum weight independent set
//! - [`MaximalIS`]: Maximal independent set
//! - [`VertexCovering`]: Minimum weight vertex cover
//! - [`DominatingSet`]: Minimum dominating set
//! - [`Clique`]: Maximum weight clique
//! - [`MaxCut`]: Maximum cut on weighted graphs
//! - [`KColoring`]: K-vertex coloring
//! - [`Matching`]: Maximum weight matching

mod clique;
mod dominating_set;
mod independent_set;
mod kcoloring;
mod matching;
mod max_cut;
mod maximal_is;
mod vertex_covering;

pub use clique::{is_clique, Clique};
pub use dominating_set::{is_dominating_set, DominatingSet};
pub use independent_set::{is_independent_set, IndependentSet};
pub use kcoloring::{is_valid_coloring, KColoring};
pub use matching::{is_matching, Matching};
pub use max_cut::{cut_size, MaxCut};
pub use maximal_is::{is_maximal_independent_set, MaximalIS};
pub use vertex_covering::{is_vertex_cover, VertexCovering};
