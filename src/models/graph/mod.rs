//! Graph problems.
//!
//! Problems whose input is a graph (optionally weighted):
//! - [`AcyclicPartition`]: Partition a digraph into bounded-weight groups with an acyclic quotient graph
//! - [`BoundedDiameterSpanningTree`]: Spanning tree with bounded weight and diameter
//! - [`DegreeConstrainedSpanningTree`]: Spanning tree with maximum vertex degree at most K
//! - [`DirectedHamiltonianPath`]: Directed Hamiltonian path (decision problem)
//! - [`MaximumIndependentSet`]: Maximum weight independent set
//! - [`MaximumLeafSpanningTree`]: Spanning tree maximizing number of leaves
//! - [`MaximalIS`]: Maximal independent set
//! - [`MinimumVertexCover`]: Minimum weight vertex cover
//! - [`MinimumCoveringByCliques`]: Minimum number of cliques covering all edges
//! - [`MonochromaticTriangle`]: 2-color edges so that no triangle is monochromatic
//! - [`MinimumIntersectionGraphBasis`]: Minimum universe size for intersection graph representation
//! - [`MinimumCapacitatedSpanningTree`]: Minimum weight spanning tree with subtree capacity constraints
//! - [`MinimumDominatingSet`]: Minimum dominating set
//! - [`MinimumMetricDimension`]: Minimum resolving set (metric dimension)
//! - [`MinimumEdgeCostFlow`]: Minimum edge-cost integral flow
//! - [`MinimumGeometricConnectedDominatingSet`]: Minimum connected dominating set in a geometric point set
//! - [`MinimumFeedbackVertexSet`]: Minimum weight feedback vertex set in a directed graph
//! - [`MaximumClique`]: Maximum weight clique
//! - [`MaximumAchromaticNumber`]: Maximum number of colors in a complete proper coloring
//! - [`MaximumDomaticNumber`]: Maximum partition into disjoint dominating sets
//! - [`MaxCut`]: Maximum cut on weighted graphs
//! - [`MinimumCutIntoBoundedSets`]: Minimum cut into bounded sets (Garey & Johnson ND17)
//! - [`MinimumDummyActivitiesPert`]: Minimum dummy activities in activity-on-arc PERT networks
//! - [`HamiltonianCircuit`]: Hamiltonian circuit (decision problem)
//! - [`IsomorphicSpanningTree`]: Isomorphic spanning tree (satisfaction)
//! - [`Kernel`]: Kernel of a directed graph (independent and absorbing vertex subset)
//! - [`KClique`]: Clique decision problem with threshold k
//! - [`KthBestSpanningTree`]: K distinct bounded spanning trees (satisfaction)
//! - [`KColoring`]: K-vertex coloring
//! - [`PartitionIntoTriangles`]: Partition vertices into triangles
//! - [`MaximumMatching`]: Maximum weight matching
//! - [`MinimumMaximalMatching`]: Minimum-size maximal matching
//! - [`TravelingSalesman`]: Traveling Salesman (minimum weight Hamiltonian cycle)
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`MinimumMultiwayCut`]: Minimum weight multiway cut
//! - [`HamiltonianPath`]: Hamiltonian path (simple path visiting every vertex)
//! - [`HamiltonianPathBetweenTwoVertices`]: Hamiltonian path between two specified vertices (decision problem)
//! - [`LongestPath`]: Maximum-length simple s-t path
//! - [`ShortestWeightConstrainedPath`]: Bicriteria simple s-t path with length and weight bounds
//! - [`PartitionIntoCliques`]: Partition vertices into K groups each inducing a clique
//! - [`PartitionIntoForests`]: Partition vertices into K classes each inducing an acyclic subgraph
//! - [`PartitionIntoPerfectMatchings`]: Partition vertices into K groups each inducing a perfect matching
//! - [`PartitionIntoPathsOfLength2`]: Partition vertices into triples with at least two edges each
//! - [`BicliqueCover`]: Biclique cover on bipartite graphs
//! - [`SteinerTreeInGraphs`]: Minimum weight Steiner tree connecting terminal vertices
//! - [`BalancedCompleteBipartiteSubgraph`]: Balanced biclique decision problem
//! - [`BiconnectivityAugmentation`]: Biconnectivity augmentation with weighted potential edges
//! - [`BoundedComponentSpanningForest`]: Partition vertices into bounded-weight connected components
//! - [`BottleneckTravelingSalesman`]: Hamiltonian cycle minimizing the maximum selected edge weight
//! - [`MultipleCopyFileAllocation`]: File-copy placement under storage and access costs
//! - [`OptimalLinearArrangement`]: Optimal linear arrangement (total edge length at most K)
//! - [`PartialFeedbackEdgeSet`]: Remove at most K edges to hit every short cycle
//! - [`RootedTreeArrangement`]: Rooted-tree embedding with bounded total edge stretch
//! - [`MinimumFeedbackArcSet`]: Minimum feedback arc set on directed graphs
//! - [`MinMaxMulticenter`]: Min-max multicenter (vertex p-center, satisfaction)
//! - [`MinimumSumMulticenter`]: Min-sum multicenter (p-median)
//! - [`MultipleChoiceBranching`]: Directed branching with partition constraints
//! - [`LengthBoundedDisjointPaths`]: Length-bounded internally disjoint s-t paths
//! - [`PathConstrainedNetworkFlow`]: Integral flow on a prescribed collection of directed s-t paths
//! - [`RuralPostman`]: Rural Postman (circuit covering required edges)
//! - [`MixedChinesePostman`]: Mixed-graph postman tour with bounded total length
//! - [`SteinerTree`]: Minimum-weight tree spanning all required terminals
//! - [`SubgraphIsomorphism`]: Subgraph isomorphism (decision problem)
//! - [`DirectedTwoCommodityIntegralFlow`]: Directed two-commodity integral flow (satisfaction)
//! - [`IntegralFlowBundles`]: Integral flow feasibility with overlapping bundle capacities
//! - [`IntegralFlowHomologousArcs`]: Integral flow with arc-pair equality constraints
//! - [`IntegralFlowWithMultipliers`]: Integral flow with vertex multipliers on a directed graph
//! - [`UndirectedFlowLowerBounds`]: Feasible s-t flow in an undirected graph with lower/upper bounds
//! - [`UndirectedTwoCommodityIntegralFlow`]: Two-commodity integral flow on undirected graphs
//! - [`StrongConnectivityAugmentation`]: Strong connectivity augmentation with weighted candidate arcs
//! - [`DisjointConnectingPaths`]: Vertex-disjoint paths connecting prescribed terminal pairs
//! - [`MinimumGraphBandwidth`]: Minimum graph bandwidth (minimize maximum edge stretch)

pub(crate) mod acyclic_partition;
pub(crate) mod balanced_complete_bipartite_subgraph;
pub(crate) mod biclique_cover;
pub(crate) mod biconnectivity_augmentation;
pub(crate) mod bottleneck_traveling_salesman;
pub(crate) mod bounded_component_spanning_forest;
pub(crate) mod bounded_diameter_spanning_tree;
pub(crate) mod degree_constrained_spanning_tree;
pub(crate) mod directed_hamiltonian_path;
pub(crate) mod directed_two_commodity_integral_flow;
pub(crate) mod disjoint_connecting_paths;
pub(crate) mod generalized_hex;
pub(crate) mod graph_partitioning;
pub(crate) mod hamiltonian_circuit;
pub(crate) mod hamiltonian_path;
pub(crate) mod hamiltonian_path_between_two_vertices;
pub(crate) mod integral_flow_bundles;
pub(crate) mod integral_flow_homologous_arcs;
pub(crate) mod integral_flow_with_multipliers;
pub(crate) mod isomorphic_spanning_tree;
pub(crate) mod kclique;
pub(crate) mod kcoloring;
pub(crate) mod kernel;
pub(crate) mod kth_best_spanning_tree;
pub(crate) mod length_bounded_disjoint_paths;
pub(crate) mod longest_circuit;
pub(crate) mod longest_path;
pub(crate) mod max_cut;
pub(crate) mod maximal_is;
pub(crate) mod maximum_achromatic_number;
pub(crate) mod maximum_clique;
pub(crate) mod maximum_domatic_number;
pub(crate) mod maximum_independent_set;
pub(crate) mod maximum_leaf_spanning_tree;
pub(crate) mod maximum_matching;
pub(crate) mod min_max_multicenter;
pub(crate) mod minimum_capacitated_spanning_tree;
pub(crate) mod minimum_covering_by_cliques;
pub(crate) mod minimum_cut_into_bounded_sets;
pub(crate) mod minimum_dominating_set;
pub(crate) mod minimum_dummy_activities_pert;
pub(crate) mod minimum_edge_cost_flow;
pub(crate) mod minimum_feedback_arc_set;
pub(crate) mod minimum_feedback_vertex_set;
pub(crate) mod minimum_geometric_connected_dominating_set;
pub(crate) mod minimum_graph_bandwidth;
pub(crate) mod minimum_intersection_graph_basis;
pub(crate) mod minimum_maximal_matching;
pub(crate) mod minimum_metric_dimension;
pub(crate) mod minimum_multiway_cut;
pub(crate) mod minimum_sum_multicenter;
pub(crate) mod minimum_vertex_cover;
pub(crate) mod mixed_chinese_postman;
pub(crate) mod monochromatic_triangle;
pub(crate) mod multiple_choice_branching;
pub(crate) mod multiple_copy_file_allocation;
pub(crate) mod optimal_linear_arrangement;
pub(crate) mod partial_feedback_edge_set;
pub(crate) mod partition_into_cliques;
pub(crate) mod partition_into_forests;
pub(crate) mod partition_into_paths_of_length_2;
pub(crate) mod partition_into_perfect_matchings;
pub(crate) mod partition_into_triangles;
pub(crate) mod path_constrained_network_flow;
pub(crate) mod rooted_tree_arrangement;
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
pub use bounded_diameter_spanning_tree::BoundedDiameterSpanningTree;
pub use degree_constrained_spanning_tree::DegreeConstrainedSpanningTree;
pub use directed_hamiltonian_path::DirectedHamiltonianPath;
pub use directed_two_commodity_integral_flow::DirectedTwoCommodityIntegralFlow;
pub use disjoint_connecting_paths::DisjointConnectingPaths;
pub use generalized_hex::GeneralizedHex;
pub use graph_partitioning::GraphPartitioning;
pub use hamiltonian_circuit::HamiltonianCircuit;
pub use hamiltonian_path::HamiltonianPath;
pub use hamiltonian_path_between_two_vertices::HamiltonianPathBetweenTwoVertices;
pub use integral_flow_bundles::IntegralFlowBundles;
pub use integral_flow_homologous_arcs::IntegralFlowHomologousArcs;
pub use integral_flow_with_multipliers::IntegralFlowWithMultipliers;
pub use isomorphic_spanning_tree::IsomorphicSpanningTree;
pub use kclique::KClique;
pub use kcoloring::KColoring;
pub use kernel::Kernel;
pub use kth_best_spanning_tree::KthBestSpanningTree;
pub use length_bounded_disjoint_paths::LengthBoundedDisjointPaths;
pub use longest_circuit::LongestCircuit;
pub use longest_path::LongestPath;
pub use max_cut::MaxCut;
pub use maximal_is::MaximalIS;
pub use maximum_achromatic_number::MaximumAchromaticNumber;
pub use maximum_clique::MaximumClique;
pub use maximum_domatic_number::MaximumDomaticNumber;
pub use maximum_independent_set::MaximumIndependentSet;
pub use maximum_leaf_spanning_tree::MaximumLeafSpanningTree;
pub use maximum_matching::MaximumMatching;
pub use min_max_multicenter::MinMaxMulticenter;
pub use minimum_capacitated_spanning_tree::MinimumCapacitatedSpanningTree;
pub use minimum_covering_by_cliques::MinimumCoveringByCliques;
pub use minimum_cut_into_bounded_sets::MinimumCutIntoBoundedSets;
pub use minimum_dominating_set::MinimumDominatingSet;
pub use minimum_dummy_activities_pert::MinimumDummyActivitiesPert;
pub use minimum_edge_cost_flow::MinimumEdgeCostFlow;
pub use minimum_feedback_arc_set::MinimumFeedbackArcSet;
pub use minimum_feedback_vertex_set::MinimumFeedbackVertexSet;
pub use minimum_geometric_connected_dominating_set::MinimumGeometricConnectedDominatingSet;
pub use minimum_graph_bandwidth::MinimumGraphBandwidth;
pub use minimum_intersection_graph_basis::MinimumIntersectionGraphBasis;
pub use minimum_maximal_matching::MinimumMaximalMatching;
pub use minimum_metric_dimension::MinimumMetricDimension;
pub use minimum_multiway_cut::MinimumMultiwayCut;
pub use minimum_sum_multicenter::MinimumSumMulticenter;
pub use minimum_vertex_cover::MinimumVertexCover;
pub use mixed_chinese_postman::MixedChinesePostman;
pub use monochromatic_triangle::MonochromaticTriangle;
pub use multiple_choice_branching::MultipleChoiceBranching;
pub use multiple_copy_file_allocation::MultipleCopyFileAllocation;
pub use optimal_linear_arrangement::OptimalLinearArrangement;
pub use partial_feedback_edge_set::PartialFeedbackEdgeSet;
pub use partition_into_cliques::PartitionIntoCliques;
pub use partition_into_forests::PartitionIntoForests;
pub use partition_into_paths_of_length_2::PartitionIntoPathsOfLength2;
pub use partition_into_perfect_matchings::PartitionIntoPerfectMatchings;
pub use partition_into_triangles::PartitionIntoTriangles;
pub use path_constrained_network_flow::PathConstrainedNetworkFlow;
pub use rooted_tree_arrangement::RootedTreeArrangement;
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
    specs.extend(bounded_diameter_spanning_tree::canonical_model_example_specs());
    specs.extend(degree_constrained_spanning_tree::canonical_model_example_specs());
    specs.extend(directed_hamiltonian_path::canonical_model_example_specs());
    specs.extend(maximum_independent_set::canonical_model_example_specs());
    specs.extend(maximum_leaf_spanning_tree::canonical_model_example_specs());
    specs.extend(minimum_vertex_cover::canonical_model_example_specs());
    specs.extend(max_cut::canonical_model_example_specs());
    specs.extend(generalized_hex::canonical_model_example_specs());
    specs.extend(hamiltonian_circuit::canonical_model_example_specs());
    specs.extend(hamiltonian_path::canonical_model_example_specs());
    specs.extend(hamiltonian_path_between_two_vertices::canonical_model_example_specs());
    specs.extend(integral_flow_bundles::canonical_model_example_specs());
    specs.extend(integral_flow_with_multipliers::canonical_model_example_specs());
    specs.extend(isomorphic_spanning_tree::canonical_model_example_specs());
    specs.extend(kclique::canonical_model_example_specs());
    specs.extend(kernel::canonical_model_example_specs());
    specs.extend(kcoloring::canonical_model_example_specs());
    specs.extend(kth_best_spanning_tree::canonical_model_example_specs());
    specs.extend(length_bounded_disjoint_paths::canonical_model_example_specs());
    specs.extend(longest_circuit::canonical_model_example_specs());
    specs.extend(longest_path::canonical_model_example_specs());
    specs.extend(minimum_covering_by_cliques::canonical_model_example_specs());
    specs.extend(monochromatic_triangle::canonical_model_example_specs());
    specs.extend(minimum_intersection_graph_basis::canonical_model_example_specs());
    specs.extend(minimum_dominating_set::canonical_model_example_specs());
    specs.extend(minimum_metric_dimension::canonical_model_example_specs());
    specs.extend(minimum_geometric_connected_dominating_set::canonical_model_example_specs());
    specs.extend(maximum_matching::canonical_model_example_specs());
    specs.extend(minimum_maximal_matching::canonical_model_example_specs());
    specs.extend(traveling_salesman::canonical_model_example_specs());
    specs.extend(maximum_achromatic_number::canonical_model_example_specs());
    specs.extend(maximum_domatic_number::canonical_model_example_specs());
    specs.extend(maximum_clique::canonical_model_example_specs());
    specs.extend(maximal_is::canonical_model_example_specs());
    specs.extend(minimum_cut_into_bounded_sets::canonical_model_example_specs());
    specs.extend(minimum_dummy_activities_pert::canonical_model_example_specs());
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
    specs.extend(partition_into_cliques::canonical_model_example_specs());
    specs.extend(partition_into_forests::canonical_model_example_specs());
    specs.extend(partition_into_perfect_matchings::canonical_model_example_specs());
    specs.extend(partition_into_paths_of_length_2::canonical_model_example_specs());
    specs.extend(path_constrained_network_flow::canonical_model_example_specs());
    specs.extend(rooted_tree_arrangement::canonical_model_example_specs());
    specs.extend(steiner_tree::canonical_model_example_specs());
    specs.extend(steiner_tree_in_graphs::canonical_model_example_specs());
    specs.extend(directed_two_commodity_integral_flow::canonical_model_example_specs());
    specs.extend(disjoint_connecting_paths::canonical_model_example_specs());
    specs.extend(undirected_flow_lower_bounds::canonical_model_example_specs());
    specs.extend(undirected_two_commodity_integral_flow::canonical_model_example_specs());
    specs.extend(strong_connectivity_augmentation::canonical_model_example_specs());
    specs.extend(rural_postman::canonical_model_example_specs());
    specs.extend(integral_flow_homologous_arcs::canonical_model_example_specs());
    specs.extend(minimum_capacitated_spanning_tree::canonical_model_example_specs());
    specs.extend(minimum_edge_cost_flow::canonical_model_example_specs());
    specs.extend(minimum_graph_bandwidth::canonical_model_example_specs());
    specs.extend(minimum_feedback_arc_set::canonical_model_example_specs());
    specs.extend(optimal_linear_arrangement::canonical_model_example_specs());
    specs.extend(partial_feedback_edge_set::canonical_model_example_specs());
    specs.extend(mixed_chinese_postman::canonical_model_example_specs());
    specs.extend(subgraph_isomorphism::canonical_model_example_specs());
    specs.extend(graph_partitioning::canonical_model_example_specs());
    specs
}
