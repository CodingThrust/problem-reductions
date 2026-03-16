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
//! - [`IsomorphicSpanningTree`]: Isomorphic spanning tree (satisfaction)
//! - [`KColoring`]: K-vertex coloring
//! - [`PartitionIntoTriangles`]: Partition vertices into triangles
//! - [`MaximumMatching`]: Maximum weight matching
//! - [`TravelingSalesman`]: Traveling Salesman (minimum weight Hamiltonian cycle)
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`HamiltonianPath`]: Hamiltonian path (simple path visiting every vertex)
//! - [`BicliqueCover`]: Biclique cover on bipartite graphs
//! - [`OptimalLinearArrangement`]: Optimal linear arrangement (total edge length at most K)
//! - [`MinimumFeedbackArcSet`]: Minimum feedback arc set on directed graphs
//! - [`MinimumSumMulticenter`]: Min-sum multicenter (p-median)
//! - [`RuralPostman`]: Rural Postman (circuit covering required edges)
//! - [`SubgraphIsomorphism`]: Subgraph isomorphism (decision problem)
//! - [`DirectedTwoCommodityIntegralFlow`]: Directed two-commodity integral flow (satisfaction)

pub(crate) mod biclique_cover;
pub(crate) mod directed_two_commodity_integral_flow;
pub(crate) mod graph_partitioning;
pub(crate) mod hamiltonian_path;
pub(crate) mod isomorphic_spanning_tree;
pub(crate) mod kcoloring;
pub(crate) mod max_cut;
pub(crate) mod maximal_is;
pub(crate) mod maximum_clique;
pub(crate) mod maximum_independent_set;
pub(crate) mod maximum_matching;
pub(crate) mod minimum_dominating_set;
pub(crate) mod minimum_feedback_arc_set;
pub(crate) mod minimum_feedback_vertex_set;
pub(crate) mod minimum_sum_multicenter;
pub(crate) mod minimum_vertex_cover;
pub(crate) mod optimal_linear_arrangement;
pub(crate) mod partition_into_triangles;
pub(crate) mod rural_postman;
pub(crate) mod spin_glass;
pub(crate) mod steiner_tree;
pub(crate) mod subgraph_isomorphism;
pub(crate) mod traveling_salesman;

pub use biclique_cover::BicliqueCover;
pub use directed_two_commodity_integral_flow::DirectedTwoCommodityIntegralFlow;
pub use graph_partitioning::GraphPartitioning;
pub use hamiltonian_path::HamiltonianPath;
pub use isomorphic_spanning_tree::IsomorphicSpanningTree;
pub use kcoloring::KColoring;
pub use max_cut::MaxCut;
pub use maximal_is::MaximalIS;
pub use maximum_clique::MaximumClique;
pub use maximum_independent_set::MaximumIndependentSet;
pub use maximum_matching::MaximumMatching;
pub use minimum_dominating_set::MinimumDominatingSet;
pub use minimum_feedback_arc_set::MinimumFeedbackArcSet;
pub use minimum_feedback_vertex_set::MinimumFeedbackVertexSet;
pub use minimum_sum_multicenter::MinimumSumMulticenter;
pub use minimum_vertex_cover::MinimumVertexCover;
pub use optimal_linear_arrangement::OptimalLinearArrangement;
pub use partition_into_triangles::PartitionIntoTriangles;
pub use rural_postman::RuralPostman;
pub use spin_glass::SpinGlass;
pub use steiner_tree::SteinerTree;
pub use subgraph_isomorphism::SubgraphIsomorphism;
pub use traveling_salesman::TravelingSalesman;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(maximum_independent_set::canonical_model_example_specs());
    specs.extend(minimum_vertex_cover::canonical_model_example_specs());
    specs.extend(max_cut::canonical_model_example_specs());
    specs.extend(hamiltonian_path::canonical_model_example_specs());
    specs.extend(isomorphic_spanning_tree::canonical_model_example_specs());
    specs.extend(kcoloring::canonical_model_example_specs());
    specs.extend(minimum_dominating_set::canonical_model_example_specs());
    specs.extend(maximum_matching::canonical_model_example_specs());
    specs.extend(traveling_salesman::canonical_model_example_specs());
    specs.extend(maximum_clique::canonical_model_example_specs());
    specs.extend(maximal_is::canonical_model_example_specs());
    specs.extend(minimum_feedback_vertex_set::canonical_model_example_specs());
    specs.extend(minimum_sum_multicenter::canonical_model_example_specs());
    specs.extend(spin_glass::canonical_model_example_specs());
    specs.extend(biclique_cover::canonical_model_example_specs());
    specs.extend(partition_into_triangles::canonical_model_example_specs());
    specs.extend(steiner_tree::canonical_model_example_specs());
    specs.extend(directed_two_commodity_integral_flow::canonical_model_example_specs());
    specs
}
