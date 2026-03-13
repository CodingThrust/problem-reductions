//! Graph problems.
//!
//! Problems whose input is a graph (optionally weighted):
//! - [`MaximumIndependentSet`]: Maximum weight independent set
//! - [`MaximalIS`]: Maximal independent set
//! - [`MinimumVertexCover`]: Minimum weight vertex cover
//! - [`MinimumDominatingSet`]: Minimum dominating set
//! - [`MinimumFeedbackVertexSet`]: Minimum weight feedback vertex set in a directed graph
//! - [`MaximumClique`]: Maximum weight clique
//! - [`MaxCut`]: Maximum cut on weighted graphs
//! - [`GraphPartitioning`]: Minimum bisection (balanced graph partitioning)
//! - [`KColoring`]: K-vertex coloring
//! - [`PartitionIntoTriangles`]: Partition vertices into triangles
//! - [`MaximumMatching`]: Maximum weight matching
//! - [`TravelingSalesman`]: Traveling Salesman (minimum weight Hamiltonian cycle)
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`BicliqueCover`]: Biclique cover on bipartite graphs
//! - [`MinSumMulticenter`]: Min-sum multicenter (p-median)
//! - [`RuralPostman`]: Rural Postman (circuit covering required edges)
//! - [`SubgraphIsomorphism`]: Subgraph isomorphism (decision problem)

pub(crate) mod biclique_cover;
pub(crate) mod graph_partitioning;
pub(crate) mod kcoloring;
pub(crate) mod max_cut;
pub(crate) mod maximal_is;
pub(crate) mod maximum_clique;
pub(crate) mod maximum_independent_set;
pub(crate) mod maximum_matching;
pub(crate) mod min_sum_multicenter;
pub(crate) mod minimum_dominating_set;
pub(crate) mod minimum_feedback_vertex_set;
pub(crate) mod minimum_vertex_cover;
pub(crate) mod partition_into_triangles;
pub(crate) mod rural_postman;
pub(crate) mod spin_glass;
pub(crate) mod subgraph_isomorphism;
pub(crate) mod traveling_salesman;

pub use biclique_cover::BicliqueCover;
pub use graph_partitioning::GraphPartitioning;
pub use kcoloring::KColoring;
pub use max_cut::MaxCut;
pub use maximal_is::MaximalIS;
pub use maximum_clique::MaximumClique;
pub use maximum_independent_set::MaximumIndependentSet;
pub use maximum_matching::MaximumMatching;
pub use min_sum_multicenter::MinSumMulticenter;
pub use minimum_dominating_set::MinimumDominatingSet;
pub use minimum_feedback_vertex_set::MinimumFeedbackVertexSet;
pub use minimum_vertex_cover::MinimumVertexCover;
pub use partition_into_triangles::PartitionIntoTriangles;
pub use rural_postman::RuralPostman;
pub use spin_glass::SpinGlass;
pub use subgraph_isomorphism::SubgraphIsomorphism;
pub use traveling_salesman::TravelingSalesman;
