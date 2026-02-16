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

pub(crate) mod kcoloring;
pub(crate) mod max_cut;
pub(crate) mod maximal_is;
pub(crate) mod maximum_clique;
pub(crate) mod maximum_independent_set;
pub(crate) mod maximum_matching;
pub(crate) mod minimum_dominating_set;
pub(crate) mod minimum_vertex_cover;
pub(crate) mod traveling_salesman;

pub use kcoloring::KColoring;
pub use max_cut::MaxCut;
pub use maximal_is::MaximalIS;
pub use maximum_clique::MaximumClique;
pub use maximum_independent_set::MaximumIndependentSet;
pub use maximum_matching::MaximumMatching;
pub use minimum_dominating_set::MinimumDominatingSet;
pub use minimum_vertex_cover::MinimumVertexCover;
pub use traveling_salesman::TravelingSalesman;
