//! Graph problems.
//!
//! Problems whose input is a graph (optionally weighted):
//! - [`AcyclicPartition`]: Partition a digraph into bounded-weight groups with an acyclic quotient graph
//! - [`MaximumIndependentSet`]: Maximum weight independent set
//! - [`MaximalIS`]: Maximal independent set
//! - [`MinimumVertexCover`]: Minimum weight vertex cover
//! - [`MinimumDominatingSet`]: Minimum dominating set
//! - [`MinimumFeedbackVertexSet`]: Minimum weight feedback vertex set in a directed graph
//! - [`MaximumClique`]: Maximum weight clique
//! - [`MaxCut`]: Maximum cut on weighted graphs
//! - [`GraphPartitioning`]: Minimum bisection (balanced graph partitioning)
//! - [`MinimumCutIntoBoundedSets`]: Minimum cut into bounded sets (Garey & Johnson ND17)
//! - [`HamiltonianCircuit`]: Hamiltonian circuit (decision problem)
//! - [`IsomorphicSpanningTree`]: Isomorphic spanning tree (satisfaction)
//! - [`KClique`]: Clique decision problem with threshold k
//! - [`KthBestSpanningTree`]: K distinct bounded spanning trees (satisfaction)
//! - [`KColoring`]: K-vertex coloring
//! - [`PartitionIntoTriangles`]: Partition vertices into triangles
//! - [`MaximumMatching`]: Maximum weight matching
//! - [`TravelingSalesman`]: Traveling Salesman (minimum weight Hamiltonian cycle)
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`MinimumMultiwayCut`]: Minimum weight multiway cut
//! - [`HamiltonianPath`]: Hamiltonian path (simple path visiting every vertex)
//! - [`ShortestWeightConstrainedPath`]: Bicriteria simple s-t path with length and weight bounds
//! - [`PartitionIntoPathsOfLength2`]: Partition vertices into triples with at least two edges each
//! - [`BicliqueCover`]: Biclique cover on bipartite graphs
//! - [`SteinerTreeInGraphs`]: Minimum weight Steiner tree connecting terminal vertices
//! - [`BalancedCompleteBipartiteSubgraph`]: Balanced biclique decision problem
//! - [`BiconnectivityAugmentation`]: Biconnectivity augmentation with weighted potential edges
//! - [`BoundedComponentSpanningForest`]: Partition vertices into bounded-weight connected components
//! - [`BottleneckTravelingSalesman`]: Hamiltonian cycle minimizing the maximum selected edge weight
//! - [`MultipleCopyFileAllocation`]: File-copy placement under storage and access costs
//! - [`OptimalLinearArrangement`]: Optimal linear arrangement (total edge length at most K)
//! - [`MinimumFeedbackArcSet`]: Minimum feedback arc set on directed graphs
//! - [`MinMaxMulticenter`]: Min-max multicenter (vertex p-center, satisfaction)
//! - [`MinimumSumMulticenter`]: Min-sum multicenter (p-median)
//! - [`MultipleChoiceBranching`]: Directed branching with partition constraints
//! - [`LengthBoundedDisjointPaths`]: Length-bounded internally disjoint s-t paths
//! - [`RuralPostman`]: Rural Postman (circuit covering required edges)
//! - [`MixedChinesePostman`]: Mixed-graph postman tour with bounded total length
//! - [`SteinerTree`]: Minimum-weight tree spanning all required terminals
//! - [`SubgraphIsomorphism`]: Subgraph isomorphism (decision problem)
//! - [`DirectedTwoCommodityIntegralFlow`]: Directed two-commodity integral flow (satisfaction)
//! - [`UndirectedFlowLowerBounds`]: Feasible s-t flow in an undirected graph with lower/upper bounds
//! - [`UndirectedTwoCommodityIntegralFlow`]: Two-commodity integral flow on undirected graphs
//! - [`StrongConnectivityAugmentation`]: Strong connectivity augmentation with weighted candidate arcs

pub(crate) mod acyclic_partition;
pub(crate) mod balanced_complete_bipartite_subgraph;
pub(crate) mod biclique_cover;
pub(crate) mod biconnectivity_augmentation;
pub(crate) mod bottleneck_traveling_salesman;
pub(crate) mod bounded_component_spanning_forest;
pub(crate) mod directed_two_commodity_integral_flow;
pub(crate) mod generalized_hex;
pub(crate) mod graph_partitioning;
pub(crate) mod hamiltonian_circuit;
pub(crate) mod hamiltonian_path;
pub(crate) mod isomorphic_spanning_tree;
pub(crate) mod kclique;
pub(crate) mod kcoloring;
pub(crate) mod kth_best_spanning_tree;
pub(crate) mod length_bounded_disjoint_paths;
pub(crate) mod longest_circuit;
pub(crate) mod max_cut;
pub(crate) mod maximal_is;
pub(crate) mod maximum_clique;
pub(crate) mod maximum_independent_set;
pub(crate) mod maximum_matching;
pub(crate) mod min_max_multicenter;
pub(crate) mod minimum_cut_into_bounded_sets;
pub(crate) mod minimum_dominating_set;
pub(crate) mod minimum_feedback_arc_set;
pub(crate) mod minimum_feedback_vertex_set;
pub(crate) mod minimum_multiway_cut;
pub(crate) mod minimum_sum_multicenter;
pub(crate) mod minimum_vertex_cover;
pub(crate) mod mixed_chinese_postman;
pub(crate) mod multiple_choice_branching;
pub(crate) mod multiple_copy_file_allocation;
pub(crate) mod optimal_linear_arrangement;
pub(crate) mod partition_into_paths_of_length_2;
pub(crate) mod partition_into_triangles;
pub(crate) mod rural_postman;
pub(crate) mod shortest_weight_constrained_path;
pub(crate) mod spin_glass;
pub(crate) mod steiner_tree;
pub(crate) mod steiner_tree_in_graphs;
pub(crate) mod strong_connectivity_augmentation;
pub(crate) mod subgraph_isomorphism;
pub(crate) mod traveling_salesman;
pub(crate) mod undirected_flow_lower_bounds;
pub(crate) mod undirected_two_commodity_integral_flow;

pub use acyclic_partition::AcyclicPartition;
pub use balanced_complete_bipartite_subgraph::BalancedCompleteBipartiteSubgraph;
pub use biclique_cover::BicliqueCover;
pub use biconnectivity_augmentation::BiconnectivityAugmentation;
pub use bottleneck_traveling_salesman::BottleneckTravelingSalesman;
pub use bounded_component_spanning_forest::BoundedComponentSpanningForest;
pub use directed_two_commodity_integral_flow::DirectedTwoCommodityIntegralFlow;
pub use generalized_hex::GeneralizedHex;
pub use graph_partitioning::GraphPartitioning;
pub use hamiltonian_circuit::HamiltonianCircuit;
pub use hamiltonian_path::HamiltonianPath;
pub use isomorphic_spanning_tree::IsomorphicSpanningTree;
pub use kclique::KClique;
pub use kcoloring::KColoring;
pub use kth_best_spanning_tree::KthBestSpanningTree;
pub use length_bounded_disjoint_paths::LengthBoundedDisjointPaths;
pub use longest_circuit::LongestCircuit;
pub use max_cut::MaxCut;
pub use maximal_is::MaximalIS;
pub use maximum_clique::MaximumClique;
pub use maximum_independent_set::MaximumIndependentSet;
pub use maximum_matching::MaximumMatching;
pub use min_max_multicenter::MinMaxMulticenter;
pub use minimum_cut_into_bounded_sets::MinimumCutIntoBoundedSets;
pub use minimum_dominating_set::MinimumDominatingSet;
pub use minimum_feedback_arc_set::MinimumFeedbackArcSet;
pub use minimum_feedback_vertex_set::MinimumFeedbackVertexSet;
pub use minimum_multiway_cut::MinimumMultiwayCut;
pub use minimum_sum_multicenter::MinimumSumMulticenter;
pub use minimum_vertex_cover::MinimumVertexCover;
pub use mixed_chinese_postman::MixedChinesePostman;
pub use multiple_choice_branching::MultipleChoiceBranching;
pub use multiple_copy_file_allocation::MultipleCopyFileAllocation;
pub use optimal_linear_arrangement::OptimalLinearArrangement;
pub use partition_into_paths_of_length_2::PartitionIntoPathsOfLength2;
pub use partition_into_triangles::PartitionIntoTriangles;
pub use rural_postman::RuralPostman;
pub use shortest_weight_constrained_path::ShortestWeightConstrainedPath;
pub use spin_glass::SpinGlass;
pub use steiner_tree::SteinerTree;
pub use steiner_tree_in_graphs::SteinerTreeInGraphs;
pub use strong_connectivity_augmentation::StrongConnectivityAugmentation;
pub use subgraph_isomorphism::SubgraphIsomorphism;
pub use traveling_salesman::TravelingSalesman;
pub use undirected_flow_lower_bounds::UndirectedFlowLowerBounds;
pub use undirected_two_commodity_integral_flow::UndirectedTwoCommodityIntegralFlow;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(acyclic_partition::canonical_model_example_specs());
    specs.extend(maximum_independent_set::canonical_model_example_specs());
    specs.extend(minimum_vertex_cover::canonical_model_example_specs());
    specs.extend(max_cut::canonical_model_example_specs());
    specs.extend(generalized_hex::canonical_model_example_specs());
    specs.extend(hamiltonian_circuit::canonical_model_example_specs());
    specs.extend(hamiltonian_path::canonical_model_example_specs());
    specs.extend(isomorphic_spanning_tree::canonical_model_example_specs());
    specs.extend(kclique::canonical_model_example_specs());
    specs.extend(kcoloring::canonical_model_example_specs());
    specs.extend(kth_best_spanning_tree::canonical_model_example_specs());
    specs.extend(length_bounded_disjoint_paths::canonical_model_example_specs());
    specs.extend(longest_circuit::canonical_model_example_specs());
    specs.extend(minimum_dominating_set::canonical_model_example_specs());
    specs.extend(maximum_matching::canonical_model_example_specs());
    specs.extend(traveling_salesman::canonical_model_example_specs());
    specs.extend(maximum_clique::canonical_model_example_specs());
    specs.extend(maximal_is::canonical_model_example_specs());
    specs.extend(minimum_cut_into_bounded_sets::canonical_model_example_specs());
    specs.extend(multiple_copy_file_allocation::canonical_model_example_specs());
    specs.extend(minimum_feedback_vertex_set::canonical_model_example_specs());
    specs.extend(min_max_multicenter::canonical_model_example_specs());
    specs.extend(minimum_multiway_cut::canonical_model_example_specs());
    specs.extend(minimum_sum_multicenter::canonical_model_example_specs());
    specs.extend(shortest_weight_constrained_path::canonical_model_example_specs());
    specs.extend(multiple_choice_branching::canonical_model_example_specs());
    specs.extend(spin_glass::canonical_model_example_specs());
    specs.extend(biclique_cover::canonical_model_example_specs());
    specs.extend(balanced_complete_bipartite_subgraph::canonical_model_example_specs());
    specs.extend(biconnectivity_augmentation::canonical_model_example_specs());
    specs.extend(bottleneck_traveling_salesman::canonical_model_example_specs());
    specs.extend(bounded_component_spanning_forest::canonical_model_example_specs());
    specs.extend(partition_into_triangles::canonical_model_example_specs());
    specs.extend(partition_into_paths_of_length_2::canonical_model_example_specs());
    specs.extend(steiner_tree::canonical_model_example_specs());
    specs.extend(steiner_tree_in_graphs::canonical_model_example_specs());
    specs.extend(directed_two_commodity_integral_flow::canonical_model_example_specs());
    specs.extend(undirected_flow_lower_bounds::canonical_model_example_specs());
    specs.extend(undirected_two_commodity_integral_flow::canonical_model_example_specs());
    specs.extend(strong_connectivity_augmentation::canonical_model_example_specs());
    specs.extend(rural_postman::canonical_model_example_specs());
    specs.extend(graph_partitioning::canonical_model_example_specs());
    specs.extend(minimum_feedback_arc_set::canonical_model_example_specs());
    specs.extend(optimal_linear_arrangement::canonical_model_example_specs());
    specs.extend(mixed_chinese_postman::canonical_model_example_specs());
    specs.extend(subgraph_isomorphism::canonical_model_example_specs());
    specs
}
