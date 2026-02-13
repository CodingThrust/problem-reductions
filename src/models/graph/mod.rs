//! Graph-based optimization problems.
//!
//! This module contains NP-hard problems defined on graphs:
//! - [`MaximumIndependentSet`]: Maximum weight independent set
//! - [`MaximalIS`]: Maximal independent set
//! - [`MinimumVertexCover`]: Minimum weight vertex cover
//! - [`MinimumDominatingSet`]: Minimum dominating set
//! - [`MaximumClique`]: Maximum weight clique
//! - [`MaxCut`]: Maximum cut on weighted graphs
//! - [`KColoring`]: K-vertex coloring
//! - [`MaximumMatching`]: Maximum weight matching
//! - [`TravelingSalesman`]: Traveling Salesman (minimum weight Hamiltonian cycle)

mod traveling_salesman;
mod kcoloring;
mod max_cut;
mod maximal_is;
mod maximum_clique;
mod maximum_independent_set;
mod maximum_matching;
mod minimum_dominating_set;
mod minimum_vertex_cover;

pub use traveling_salesman::{is_hamiltonian_cycle, TravelingSalesman};
pub use kcoloring::{is_valid_coloring, KColoring};
pub use max_cut::{cut_size, MaxCut};
pub use maximal_is::{is_maximal_independent_set, MaximalIS};
pub use maximum_clique::{is_clique, MaximumClique};
pub use maximum_independent_set::{is_independent_set, MaximumIndependentSet};
pub use maximum_matching::{is_matching, MaximumMatching};
pub use minimum_dominating_set::{is_dominating_set, MinimumDominatingSet};
pub use minimum_vertex_cover::{is_vertex_cover, MinimumVertexCover};
