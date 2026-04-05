//! Reduction rules between NP-hard problems.

pub mod analysis;
pub mod cost;
pub mod registry;
pub use cost::{
    CustomCost, Minimize, MinimizeOutputSize, MinimizeSteps, MinimizeStepsThenOverhead, PathCostFn,
};
pub use registry::{EdgeCapabilities, ReductionEntry, ReductionOverhead};

pub(crate) mod circuit_spinglass;
mod closestvectorproblem_qubo;
pub(crate) mod coloring_qubo;
pub(crate) mod exactcoverby3sets_algebraicequationsovergf2;
pub(crate) mod exactcoverby3sets_maximumsetpacking;
pub(crate) mod exactcoverby3sets_minimumweightsolutiontolinearequations;
pub(crate) mod exactcoverby3sets_staffscheduling;
pub(crate) mod exactcoverby3sets_subsetproduct;
pub(crate) mod factoring_circuit;
mod graph;
pub(crate) mod graph_helpers;
pub(crate) mod graphpartitioning_maxcut;
pub(crate) mod graphpartitioning_qubo;
pub(crate) mod hamiltoniancircuit_biconnectivityaugmentation;
pub(crate) mod hamiltoniancircuit_bottlenecktravelingsalesman;
pub(crate) mod hamiltoniancircuit_hamiltonianpath;
pub(crate) mod hamiltoniancircuit_longestcircuit;
pub(crate) mod hamiltoniancircuit_quadraticassignment;
pub(crate) mod hamiltoniancircuit_ruralpostman;
pub(crate) mod hamiltoniancircuit_stackercrane;
pub(crate) mod hamiltoniancircuit_strongconnectivityaugmentation;
pub(crate) mod hamiltoniancircuit_travelingsalesman;
pub(crate) mod hamiltonianpath_consecutiveonessubmatrix;
pub(crate) mod hamiltonianpath_degreeconstrainedspanningtree;
pub(crate) mod hamiltonianpath_isomorphicspanningtree;
pub(crate) mod hamiltonianpathbetweentwovertices_longestpath;
pub(crate) mod ilp_i32_ilp_bool;
pub(crate) mod kclique_balancedcompletebipartitesubgraph;
pub(crate) mod kclique_conjunctivebooleanquery;
pub(crate) mod kclique_subgraphisomorphism;
mod kcoloring_casts;
pub(crate) mod kcoloring_partitionintocliques;
pub(crate) mod kcoloring_twodimensionalconsecutivesets;
mod knapsack_qubo;
mod ksatisfiability_casts;
pub(crate) mod ksatisfiability_kclique;
pub(crate) mod ksatisfiability_kernel;
pub(crate) mod ksatisfiability_minimumvertexcover;
pub(crate) mod ksatisfiability_qubo;
pub(crate) mod ksatisfiability_simultaneousincongruences;
pub(crate) mod ksatisfiability_subsetsum;
pub(crate) mod longestcommonsubsequence_maximumindependentset;
pub(crate) mod maxcut_minimumcutintoboundedsets;
pub(crate) mod maximumclique_maximumindependentset;
mod maximumindependentset_casts;
mod maximumindependentset_gridgraph;
pub(crate) mod maximumindependentset_integralflowbundles;
pub(crate) mod maximumindependentset_maximumclique;
pub(crate) mod maximumindependentset_maximumsetpacking;
mod maximumindependentset_triangular;
pub(crate) mod maximummatching_maximumsetpacking;
mod maximumsetpacking_casts;
pub(crate) mod maximumsetpacking_qubo;
pub(crate) mod minimummultiwaycut_qubo;
pub(crate) mod minimumvertexcover_ensemblecomputation;
pub(crate) mod minimumvertexcover_longestcommonsubsequence;
pub(crate) mod minimumvertexcover_maximumindependentset;
pub(crate) mod minimumvertexcover_minimumfeedbackarcset;
pub(crate) mod minimumvertexcover_minimumfeedbackvertexset;
pub(crate) mod minimumvertexcover_minimumhittingset;
pub(crate) mod minimumvertexcover_minimumsetcovering;
pub(crate) mod naesatisfiability_maxcut;
pub(crate) mod naesatisfiability_partitionintoperfectmatchings;
pub(crate) mod naesatisfiability_setsplitting;
pub(crate) mod paintshop_qubo;
pub(crate) mod partition_binpacking;
pub(crate) mod partition_cosineproductintegration;
pub(crate) mod partition_knapsack;
pub(crate) mod partition_multiprocessorscheduling;
pub(crate) mod partition_openshopscheduling;
pub(crate) mod partition_productionplanning;
pub(crate) mod partition_sequencingtominimizetardytaskweight;
pub(crate) mod partition_sequencingwithinintervals;
pub(crate) mod partition_shortestweightconstrainedpath;
pub(crate) mod partition_subsetsum;
pub(crate) mod partitionintopathsoflength2_boundedcomponentspanningforest;
pub(crate) mod rootedtreearrangement_rootedtreestorageassignment;
pub(crate) mod sat_circuitsat;
pub(crate) mod sat_coloring;
pub(crate) mod sat_ksat;
pub(crate) mod sat_maximumindependentset;
pub(crate) mod sat_minimumdominatingset;
pub(crate) mod satisfiability_naesatisfiability;
pub(crate) mod satisfiability_nontautology;
pub(crate) mod setsplitting_betweenness;
mod spinglass_casts;
pub(crate) mod spinglass_maxcut;
pub(crate) mod spinglass_qubo;
pub(crate) mod subsetsum_capacityassignment;
pub(crate) mod subsetsum_closestvectorproblem;
pub(crate) mod subsetsum_integerexpressionmembership;
pub(crate) mod subsetsum_partition;
#[cfg(test)]
pub(crate) mod test_helpers;
pub(crate) mod threedimensionalmatching_threepartition;
pub(crate) mod threepartition_flowshopscheduling;
pub(crate) mod threepartition_jobshopscheduling;
pub(crate) mod threepartition_resourceconstrainedscheduling;
pub(crate) mod threepartition_sequencingtominimizeweightedtardiness;
pub(crate) mod threepartition_sequencingwithreleasetimesanddeadlines;
mod traits;
pub(crate) mod travelingsalesman_qubo;

pub mod unitdiskmapping;

#[cfg(feature = "ilp-solver")]
pub(crate) mod acyclicpartition_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod balancedcompletebipartitesubgraph_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod bicliquecover_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod biconnectivityaugmentation_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod binpacking_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod bmf_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod bottlenecktravelingsalesman_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod boundedcomponentspanningforest_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod capacityassignment_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod circuit_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod coloring_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod consecutiveblockminimization_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod consecutiveonesmatrixaugmentation_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod consecutiveonessubmatrix_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod consistencyofdatabasefrequencytables_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod directedhamiltonianpath_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod directedtwocommodityintegralflow_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod disjointconnectingpaths_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod exactcoverby3sets_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod expectedretrievalcost_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod factoring_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod flowshopscheduling_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod graphpartitioning_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod hamiltonianpath_ilp;
#[cfg(feature = "ilp-solver")]
mod ilp_bool_ilp_i32;
#[cfg(feature = "ilp-solver")]
pub(crate) mod ilp_helpers;
#[cfg(feature = "ilp-solver")]
pub(crate) mod ilp_qubo;
#[cfg(feature = "ilp-solver")]
pub(crate) mod integralflowbundles_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod integralflowhomologousarcs_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod integralflowwithmultipliers_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod isomorphicspanningtree_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod kclique_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod knapsack_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod lengthboundeddisjointpaths_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod longestcircuit_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod longestcommonsubsequence_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod longestpath_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximalis_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximum2satisfiability_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximumclique_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximumdomaticnumber_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximumleafspanningtree_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximumlikelihoodranking_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximummatching_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod maximumsetpacking_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumcapacitatedspanningtree_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumcutintoboundedsets_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumdominatingset_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumedgecostflow_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumexternalmacrodatacompression_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumfeedbackarcset_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumfeedbackvertexset_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumgraphbandwidth_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumhittingset_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimuminternalmacrodatacompression_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimummatrixcover_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimummaximalmatching_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimummetricdimension_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimummultiwaycut_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumsetcovering_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumsummulticenter_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumtardinesssequencing_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minimumweightdecoding_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod minmaxmulticenter_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod mixedchinesepostman_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod multiplecopyfileallocation_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod multiprocessorscheduling_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod naesatisfiability_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod numericalmatchingwithtargetsums_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod openshopscheduling_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod optimallineararrangement_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod optimumcommunicationspanningtree_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod paintshop_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod partiallyorderedknapsack_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod partitionintopathsoflength2_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod partitionintotriangles_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod pathconstrainednetworkflow_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod precedenceconstrainedscheduling_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod preemptivescheduling_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod quadraticassignment_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod qubo_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod rectilinearpicturecompression_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod resourceconstrainedscheduling_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod rootedtreestorageassignment_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod ruralpostman_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod schedulingtominimizeweightedcompletiontime_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod schedulingwithindividualdeadlines_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sequencingtominimizemaximumcumulativecost_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sequencingtominimizetardytaskweight_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sequencingtominimizeweightedcompletiontime_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sequencingtominimizeweightedtardiness_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sequencingwithdeadlinesandsetuptimes_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sequencingwithinintervals_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sequencingwithreleasetimesanddeadlines_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod setsplitting_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod shortestcommonsupersequence_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod shortestweightconstrainedpath_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sparsematrixcompression_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod stackercrane_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod steinertree_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod steinertreeingraphs_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod stringtostringcorrection_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod strongconnectivityaugmentation_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod subgraphisomorphism_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod sumofsquarespartition_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod timetabledesign_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod travelingsalesman_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod undirectedflowlowerbounds_ilp;
#[cfg(feature = "ilp-solver")]
pub(crate) mod undirectedtwocommodityintegralflow_ilp;

pub use graph::{
    AggregateReductionChain, NeighborInfo, NeighborTree, ReductionChain, ReductionEdgeInfo,
    ReductionGraph, ReductionMode, ReductionPath, ReductionStep, TraversalFlow,
};
pub use traits::{
    AggregateReductionResult, ReduceTo, ReduceToAggregate, ReductionAutoCast, ReductionResult,
};

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(circuit_spinglass::canonical_rule_example_specs());
    specs.extend(exactcoverby3sets_staffscheduling::canonical_rule_example_specs());
    specs.extend(closestvectorproblem_qubo::canonical_rule_example_specs());
    specs.extend(coloring_qubo::canonical_rule_example_specs());
    specs.extend(exactcoverby3sets_algebraicequationsovergf2::canonical_rule_example_specs());
    specs.extend(
        exactcoverby3sets_minimumweightsolutiontolinearequations::canonical_rule_example_specs(),
    );
    specs.extend(exactcoverby3sets_subsetproduct::canonical_rule_example_specs());
    specs.extend(factoring_circuit::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_biconnectivityaugmentation::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_bottlenecktravelingsalesman::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_hamiltonianpath::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_longestcircuit::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_quadraticassignment::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_ruralpostman::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_stackercrane::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_strongconnectivityaugmentation::canonical_rule_example_specs());
    specs.extend(hamiltoniancircuit_travelingsalesman::canonical_rule_example_specs());
    specs.extend(hamiltonianpath_consecutiveonessubmatrix::canonical_rule_example_specs());
    specs.extend(hamiltonianpath_degreeconstrainedspanningtree::canonical_rule_example_specs());
    specs.extend(graphpartitioning_maxcut::canonical_rule_example_specs());
    specs.extend(graphpartitioning_qubo::canonical_rule_example_specs());
    specs.extend(hamiltonianpathbetweentwovertices_longestpath::canonical_rule_example_specs());
    specs.extend(hamiltonianpath_isomorphicspanningtree::canonical_rule_example_specs());
    specs.extend(kclique_balancedcompletebipartitesubgraph::canonical_rule_example_specs());
    specs.extend(kclique_conjunctivebooleanquery::canonical_rule_example_specs());
    specs.extend(kclique_subgraphisomorphism::canonical_rule_example_specs());
    specs.extend(kcoloring_partitionintocliques::canonical_rule_example_specs());
    specs.extend(kcoloring_twodimensionalconsecutivesets::canonical_rule_example_specs());
    specs.extend(knapsack_qubo::canonical_rule_example_specs());
    specs.extend(longestcommonsubsequence_maximumindependentset::canonical_rule_example_specs());
    specs.extend(ksatisfiability_kclique::canonical_rule_example_specs());
    specs.extend(ksatisfiability_kernel::canonical_rule_example_specs());
    specs.extend(ksatisfiability_minimumvertexcover::canonical_rule_example_specs());
    specs.extend(ksatisfiability_qubo::canonical_rule_example_specs());
    specs.extend(ksatisfiability_simultaneousincongruences::canonical_rule_example_specs());
    specs.extend(ksatisfiability_subsetsum::canonical_rule_example_specs());
    specs.extend(maximumclique_maximumindependentset::canonical_rule_example_specs());
    specs.extend(maximumindependentset_integralflowbundles::canonical_rule_example_specs());
    specs.extend(maximumindependentset_maximumclique::canonical_rule_example_specs());
    specs.extend(maximumindependentset_maximumsetpacking::canonical_rule_example_specs());
    specs.extend(maximummatching_maximumsetpacking::canonical_rule_example_specs());
    specs.extend(maximumsetpacking_qubo::canonical_rule_example_specs());
    specs.extend(minimummultiwaycut_qubo::canonical_rule_example_specs());
    specs.extend(paintshop_qubo::canonical_rule_example_specs());
    specs.extend(partition_cosineproductintegration::canonical_rule_example_specs());
    specs.extend(partition_knapsack::canonical_rule_example_specs());
    specs.extend(partition_openshopscheduling::canonical_rule_example_specs());
    specs.extend(partition_productionplanning::canonical_rule_example_specs());
    specs.extend(partition_sequencingtominimizetardytaskweight::canonical_rule_example_specs());
    specs.extend(
        partitionintopathsoflength2_boundedcomponentspanningforest::canonical_rule_example_specs(),
    );
    specs.extend(partition_multiprocessorscheduling::canonical_rule_example_specs());
    specs.extend(partition_sequencingwithinintervals::canonical_rule_example_specs());
    specs.extend(partition_shortestweightconstrainedpath::canonical_rule_example_specs());
    specs.extend(partition_subsetsum::canonical_rule_example_specs());
    specs.extend(rootedtreearrangement_rootedtreestorageassignment::canonical_rule_example_specs());
    specs.extend(naesatisfiability_maxcut::canonical_rule_example_specs());
    specs.extend(naesatisfiability_partitionintoperfectmatchings::canonical_rule_example_specs());
    specs.extend(exactcoverby3sets_maximumsetpacking::canonical_rule_example_specs());
    specs.extend(maxcut_minimumcutintoboundedsets::canonical_rule_example_specs());
    specs.extend(partition_binpacking::canonical_rule_example_specs());
    specs.extend(threedimensionalmatching_threepartition::canonical_rule_example_specs());
    specs.extend(threepartition_flowshopscheduling::canonical_rule_example_specs());
    specs.extend(threepartition_jobshopscheduling::canonical_rule_example_specs());
    specs.extend(threepartition_resourceconstrainedscheduling::canonical_rule_example_specs());
    specs.extend(
        threepartition_sequencingtominimizeweightedtardiness::canonical_rule_example_specs(),
    );
    specs.extend(
        threepartition_sequencingwithreleasetimesanddeadlines::canonical_rule_example_specs(),
    );
    specs.extend(minimumvertexcover_ensemblecomputation::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_longestcommonsubsequence::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_maximumindependentset::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_minimumfeedbackarcset::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_minimumfeedbackvertexset::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_minimumhittingset::canonical_rule_example_specs());
    specs.extend(minimumvertexcover_minimumsetcovering::canonical_rule_example_specs());
    specs.extend(naesatisfiability_setsplitting::canonical_rule_example_specs());
    specs.extend(setsplitting_betweenness::canonical_rule_example_specs());
    specs.extend(satisfiability_naesatisfiability::canonical_rule_example_specs());
    specs.extend(sat_circuitsat::canonical_rule_example_specs());
    specs.extend(sat_coloring::canonical_rule_example_specs());
    specs.extend(sat_ksat::canonical_rule_example_specs());
    specs.extend(sat_maximumindependentset::canonical_rule_example_specs());
    specs.extend(sat_minimumdominatingset::canonical_rule_example_specs());
    specs.extend(satisfiability_nontautology::canonical_rule_example_specs());
    specs.extend(spinglass_maxcut::canonical_rule_example_specs());
    specs.extend(spinglass_qubo::canonical_rule_example_specs());
    specs.extend(subsetsum_capacityassignment::canonical_rule_example_specs());
    specs.extend(subsetsum_closestvectorproblem::canonical_rule_example_specs());
    specs.extend(subsetsum_integerexpressionmembership::canonical_rule_example_specs());
    specs.extend(subsetsum_partition::canonical_rule_example_specs());
    specs.extend(travelingsalesman_qubo::canonical_rule_example_specs());
    #[cfg(feature = "ilp-solver")]
    {
        specs.extend(acyclicpartition_ilp::canonical_rule_example_specs());
        specs.extend(balancedcompletebipartitesubgraph_ilp::canonical_rule_example_specs());
        specs.extend(bicliquecover_ilp::canonical_rule_example_specs());
        specs.extend(biconnectivityaugmentation_ilp::canonical_rule_example_specs());
        specs.extend(binpacking_ilp::canonical_rule_example_specs());
        specs.extend(bmf_ilp::canonical_rule_example_specs());
        specs.extend(bottlenecktravelingsalesman_ilp::canonical_rule_example_specs());
        specs.extend(boundedcomponentspanningforest_ilp::canonical_rule_example_specs());
        specs.extend(capacityassignment_ilp::canonical_rule_example_specs());
        specs.extend(circuit_ilp::canonical_rule_example_specs());
        specs.extend(coloring_ilp::canonical_rule_example_specs());
        specs.extend(consecutiveblockminimization_ilp::canonical_rule_example_specs());
        specs.extend(consecutiveonesmatrixaugmentation_ilp::canonical_rule_example_specs());
        specs.extend(consecutiveonessubmatrix_ilp::canonical_rule_example_specs());
        specs.extend(consistencyofdatabasefrequencytables_ilp::canonical_rule_example_specs());
        specs.extend(directedhamiltonianpath_ilp::canonical_rule_example_specs());
        specs.extend(directedtwocommodityintegralflow_ilp::canonical_rule_example_specs());
        specs.extend(disjointconnectingpaths_ilp::canonical_rule_example_specs());
        specs.extend(exactcoverby3sets_ilp::canonical_rule_example_specs());
        specs.extend(expectedretrievalcost_ilp::canonical_rule_example_specs());
        specs.extend(factoring_ilp::canonical_rule_example_specs());
        specs.extend(flowshopscheduling_ilp::canonical_rule_example_specs());
        specs.extend(graphpartitioning_ilp::canonical_rule_example_specs());
        specs.extend(hamiltonianpath_ilp::canonical_rule_example_specs());
        specs.extend(ilp_qubo::canonical_rule_example_specs());
        specs.extend(integralflowbundles_ilp::canonical_rule_example_specs());
        specs.extend(integralflowhomologousarcs_ilp::canonical_rule_example_specs());
        specs.extend(integralflowwithmultipliers_ilp::canonical_rule_example_specs());
        specs.extend(isomorphicspanningtree_ilp::canonical_rule_example_specs());
        specs.extend(kclique_ilp::canonical_rule_example_specs());
        specs.extend(knapsack_ilp::canonical_rule_example_specs());
        specs.extend(maximumlikelihoodranking_ilp::canonical_rule_example_specs());
        specs.extend(lengthboundeddisjointpaths_ilp::canonical_rule_example_specs());
        specs.extend(longestcircuit_ilp::canonical_rule_example_specs());
        specs.extend(longestcommonsubsequence_ilp::canonical_rule_example_specs());
        specs.extend(longestpath_ilp::canonical_rule_example_specs());
        specs.extend(maximalis_ilp::canonical_rule_example_specs());
        specs.extend(maximum2satisfiability_ilp::canonical_rule_example_specs());
        specs.extend(maximumclique_ilp::canonical_rule_example_specs());
        specs.extend(maximumdomaticnumber_ilp::canonical_rule_example_specs());
        specs.extend(maximumleafspanningtree_ilp::canonical_rule_example_specs());
        specs.extend(maximummatching_ilp::canonical_rule_example_specs());
        specs.extend(maximumsetpacking_ilp::canonical_rule_example_specs());
        specs.extend(minimumcutintoboundedsets_ilp::canonical_rule_example_specs());
        specs.extend(minimumdominatingset_ilp::canonical_rule_example_specs());
        specs.extend(minimummetricdimension_ilp::canonical_rule_example_specs());
        specs.extend(minimummatrixcover_ilp::canonical_rule_example_specs());
        specs.extend(minimummaximalmatching_ilp::canonical_rule_example_specs());
        specs.extend(minimumcapacitatedspanningtree_ilp::canonical_rule_example_specs());
        specs.extend(minimumedgecostflow_ilp::canonical_rule_example_specs());
        specs.extend(minimumexternalmacrodatacompression_ilp::canonical_rule_example_specs());
        specs.extend(minimuminternalmacrodatacompression_ilp::canonical_rule_example_specs());
        specs.extend(minimumfeedbackarcset_ilp::canonical_rule_example_specs());
        specs.extend(minimumfeedbackvertexset_ilp::canonical_rule_example_specs());
        specs.extend(minimumgraphbandwidth_ilp::canonical_rule_example_specs());
        specs.extend(minimumhittingset_ilp::canonical_rule_example_specs());
        specs.extend(minimummultiwaycut_ilp::canonical_rule_example_specs());
        specs.extend(minimumsetcovering_ilp::canonical_rule_example_specs());
        specs.extend(minimumweightdecoding_ilp::canonical_rule_example_specs());
        specs.extend(minimumtardinesssequencing_ilp::canonical_rule_example_specs());
        specs.extend(minimumsummulticenter_ilp::canonical_rule_example_specs());
        specs.extend(minmaxmulticenter_ilp::canonical_rule_example_specs());
        specs.extend(mixedchinesepostman_ilp::canonical_rule_example_specs());
        specs.extend(multiplecopyfileallocation_ilp::canonical_rule_example_specs());
        specs.extend(multiprocessorscheduling_ilp::canonical_rule_example_specs());
        specs.extend(naesatisfiability_ilp::canonical_rule_example_specs());
        specs.extend(numericalmatchingwithtargetsums_ilp::canonical_rule_example_specs());
        specs.extend(openshopscheduling_ilp::canonical_rule_example_specs());
        specs.extend(optimumcommunicationspanningtree_ilp::canonical_rule_example_specs());
        specs.extend(optimallineararrangement_ilp::canonical_rule_example_specs());
        specs.extend(paintshop_ilp::canonical_rule_example_specs());
        specs.extend(partiallyorderedknapsack_ilp::canonical_rule_example_specs());
        specs.extend(partitionintopathsoflength2_ilp::canonical_rule_example_specs());
        specs.extend(partitionintotriangles_ilp::canonical_rule_example_specs());
        specs.extend(pathconstrainednetworkflow_ilp::canonical_rule_example_specs());
        specs.extend(precedenceconstrainedscheduling_ilp::canonical_rule_example_specs());
        specs.extend(preemptivescheduling_ilp::canonical_rule_example_specs());
        specs.extend(quadraticassignment_ilp::canonical_rule_example_specs());
        specs.extend(qubo_ilp::canonical_rule_example_specs());
        specs.extend(rectilinearpicturecompression_ilp::canonical_rule_example_specs());
        specs.extend(resourceconstrainedscheduling_ilp::canonical_rule_example_specs());
        specs.extend(rootedtreestorageassignment_ilp::canonical_rule_example_specs());
        specs.extend(ruralpostman_ilp::canonical_rule_example_specs());
        specs
            .extend(schedulingtominimizeweightedcompletiontime_ilp::canonical_rule_example_specs());
        specs.extend(schedulingwithindividualdeadlines_ilp::canonical_rule_example_specs());
        specs.extend(sequencingtominimizemaximumcumulativecost_ilp::canonical_rule_example_specs());
        specs.extend(sequencingtominimizetardytaskweight_ilp::canonical_rule_example_specs());
        specs.extend(sequencingwithdeadlinesandsetuptimes_ilp::canonical_rule_example_specs());
        specs
            .extend(sequencingtominimizeweightedcompletiontime_ilp::canonical_rule_example_specs());
        specs.extend(sequencingtominimizeweightedtardiness_ilp::canonical_rule_example_specs());
        specs.extend(sequencingwithinintervals_ilp::canonical_rule_example_specs());
        specs.extend(sequencingwithreleasetimesanddeadlines_ilp::canonical_rule_example_specs());
        specs.extend(shortestcommonsupersequence_ilp::canonical_rule_example_specs());
        specs.extend(setsplitting_ilp::canonical_rule_example_specs());
        specs.extend(shortestweightconstrainedpath_ilp::canonical_rule_example_specs());
        specs.extend(sparsematrixcompression_ilp::canonical_rule_example_specs());
        specs.extend(stackercrane_ilp::canonical_rule_example_specs());
        specs.extend(steinertree_ilp::canonical_rule_example_specs());
        specs.extend(steinertreeingraphs_ilp::canonical_rule_example_specs());
        specs.extend(stringtostringcorrection_ilp::canonical_rule_example_specs());
        specs.extend(strongconnectivityaugmentation_ilp::canonical_rule_example_specs());
        specs.extend(subgraphisomorphism_ilp::canonical_rule_example_specs());
        specs.extend(sumofsquarespartition_ilp::canonical_rule_example_specs());
        specs.extend(timetabledesign_ilp::canonical_rule_example_specs());
        specs.extend(travelingsalesman_ilp::canonical_rule_example_specs());
        specs.extend(undirectedflowlowerbounds_ilp::canonical_rule_example_specs());
        specs.extend(undirectedtwocommodityintegralflow_ilp::canonical_rule_example_specs());
    }
    specs
}

/// Generates a variant-cast `ReduceTo` impl with `#[reduction]` registration.
///
/// Variant casts convert a problem from one variant to another (e.g.,
/// `MIS<KingsSubgraph, i32>` -> `MIS<UnitDiskGraph, i32>`). The solution
/// mapping is identity -- vertex/element indices are preserved.
///
/// The problem name is specified once, followed by `<SourceParams> => <TargetParams>`.
/// This works with any number of type parameters.
///
/// # Example
///
/// ```text
/// impl_variant_reduction!(
///     MaximumIndependentSet,
///     <KingsSubgraph, i32> => <UnitDiskGraph, i32>,
///     fields: [num_vertices, num_edges],
///     |src| MaximumIndependentSet::new(
///         src.graph().cast_to_parent(), src.weights())
/// );
/// ```
#[macro_export]
macro_rules! impl_variant_reduction {
    ($problem:ident,
     < $($src_param:ty),+ > => < $($dst_param:ty),+ >,
     fields: [$($field:ident),+],
     |$src:ident| $body:expr) => {
        #[$crate::reduction(
            overhead = {
                $crate::rules::registry::ReductionOverhead::identity(
                    &[$(stringify!($field)),+]
                )
            }
        )]
        impl $crate::rules::ReduceTo<$problem<$($dst_param),+>>
            for $problem<$($src_param),+>
        {
            type Result = $crate::rules::ReductionAutoCast<
                $problem<$($src_param),+>,
                $problem<$($dst_param),+>,
            >;
            fn reduce_to(&self) -> Self::Result {
                let $src = self;
                $crate::rules::ReductionAutoCast::new($body)
            }
        }
    };
}
