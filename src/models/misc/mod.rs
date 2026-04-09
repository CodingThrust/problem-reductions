//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`AdditionalKey`]: Determine whether a relational schema has an additional candidate key
//! - [`Betweenness`]: Find a linear ordering satisfying betweenness constraints on triples
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Clustering`]: Partition elements into bounded-diameter clusters
//! - [`CyclicOrdering`]: Find a permutation satisfying cyclic ordering constraints on triples
//! - [`BoyceCoddNormalFormViolation`]: Boyce-Codd Normal Form Violation (BCNF)
//! - [`ConsistencyOfDatabaseFrequencyTables`]: Pairwise frequency-table consistency
//! - [`ConjunctiveBooleanQuery`]: Evaluate a conjunctive Boolean query over relations
//! - [`ConjunctiveQueryFoldability`]: Conjunctive Query Foldability
//! - [`DynamicStorageAllocation`]: Assign starting addresses in bounded memory for time-varying items
//! - [`ExpectedRetrievalCost`]: Allocate records to circular sectors within a latency bound
//! - [`Factoring`]: Integer factorization
//! - [`FeasibleRegisterAssignment`]: Determine if a DAG computation can be scheduled without register conflicts under a fixed assignment
//! - [`IntegerExpressionMembership`]: Membership in a set defined by an integer expression tree
//! - [`FlowShopScheduling`]: Flow Shop Scheduling (meet deadline on m processors)
//! - [`GroupingBySwapping`]: Group equal symbols into contiguous blocks by adjacent swaps
//! - [`JobShopScheduling`]: Minimize makespan with per-job processor routes
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`MultiprocessorScheduling`]: Schedule tasks on processors to meet a deadline
//! - [`Numerical3DimensionalMatching`]: Partition W∪X∪Y into m triples each summing to B
//! - [`NumericalMatchingWithTargetSums`]: Partition X∪Y into m pairs with pair sums matching targets
//! - [`OpenShopScheduling`]: Open Shop Scheduling (minimize makespan, free task order per job)
//! - [`LongestCommonSubsequence`]: Longest Common Subsequence
//! - [`MaximumLikelihoodRanking`]: Find a ranking minimizing total pairwise disagreement
//! - [`MinimumAxiomSet`]: Find smallest axiom subset whose deductive closure covers all true sentences
//! - [`MinimumCodeGenerationOneRegister`]: Minimize instruction count for a one-register machine
//! - [`MinimumCodeGenerationParallelAssignments`]: Minimize backward dependencies when ordering parallel assignments
//! - [`MinimumCodeGenerationUnlimitedRegisters`]: Minimize instruction count for an unlimited-register machine with 2-address instructions
//! - [`MinimumExternalMacroDataCompression`]: Minimize compression cost using external dictionary
//! - [`MinimumFaultDetectionTestSet`]: Find minimum set of input-output paths covering all internal DAG vertices
//! - [`MinimumInternalMacroDataCompression`]: Minimize self-referencing compression cost
//! - [`MinimumRegisterSufficiencyForLoops`]: Minimize registers for loop variable allocation (circular arc coloring)
//! - [`MinimumWeightAndOrGraph`]: Find minimum-weight solution subgraph in a DAG with AND/OR gates
//! - [`MinimumTardinessSequencing`]: Minimize tardy tasks in single-machine scheduling
//! - [`OptimumCommunicationSpanningTree`]: Find spanning tree minimizing total weighted communication cost
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`CosineProductIntegration`]: Balanced sign assignment for integer frequencies
//! - [`NonLivenessFreePetriNet`]: Determine whether a free-choice Petri net is not live
//! - [`Partition`]: Partition a multiset into two equal-sum subsets
//! - [`PartiallyOrderedKnapsack`]: Knapsack with precedence constraints
//! - [`PrecedenceConstrainedScheduling`]: Schedule unit tasks on processors by deadline
//! - [`PreemptiveScheduling`]: Preemptive parallel scheduling with precedences (minimize makespan)
//! - [`ProductionPlanning`]: Meet all period demands within capacity and total-cost bounds
//! - [`RectilinearPictureCompression`]: Cover 1-entries with bounded rectangles
//! - [`RegisterSufficiency`]: Evaluate DAG computation with bounded registers
//! - [`ResourceConstrainedScheduling`]: Schedule unit-length tasks on processors with resource constraints
//! - [`SchedulingWithIndividualDeadlines`]: Meet per-task deadlines on parallel processors
//! - [`StackerCrane`]: Minimize the total length of a closed walk through required arcs
//! - [`SequencingToMinimizeMaximumCumulativeCost`]: Keep every cumulative schedule cost prefix under a bound
//! - [`SequencingToMinimizeTardyTaskWeight`]: Minimize total weight of tardy tasks
//! - [`SequencingToMinimizeWeightedCompletionTime`]: Minimize total weighted completion time
//! - [`SequencingToMinimizeWeightedTardiness`]: Decide whether a schedule meets a weighted tardiness bound
//! - [`SequencingWithDeadlinesAndSetUpTimes`]: Single-machine scheduling feasibility with compiler-switch setup penalties
//! - [`SequencingWithReleaseTimesAndDeadlines`]: Single-machine scheduling feasibility
//! - [`SequencingWithinIntervals`]: Schedule tasks within time windows
//! - [`ShortestCommonSupersequence`]: Find a common supersequence of bounded length
//! - [`SquareTiling`]: Place colored square tiles on an N x N grid with matching edge colors
//! - [`TimetableDesign`]: Schedule craftsmen on tasks across work periods
//! - [`StringToStringCorrection`]: String-to-String Correction (derive target via deletions and swaps)
//! - [`SubsetProduct`]: Find a subset whose product equals exactly a target value
//! - [`SubsetSum`]: Find a subset summing to exactly a target value
//! - [`SumOfSquaresPartition`]: Partition integers into K groups minimizing sum of squared group sums

pub(crate) mod additional_key;
mod betweenness;
pub(crate) mod biguint_serde;
mod cyclic_ordering;

/// Decode a Lehmer code into a permutation of `0..n`.
///
/// Each element of `config` selects from the remaining items:
/// `config[i]` must be `< n - i`. Returns `None` if the config is
/// invalid (wrong length or out-of-range digit).
pub(crate) fn decode_lehmer(config: &[usize], n: usize) -> Option<Vec<usize>> {
    if config.len() != n {
        return None;
    }
    let mut available: Vec<usize> = (0..n).collect();
    let mut schedule = Vec::with_capacity(n);
    for &digit in config {
        if digit >= available.len() {
            return None;
        }
        schedule.push(available.remove(digit));
    }
    Some(schedule)
}

/// Decode a direct permutation configuration.
///
/// Returns `Some(schedule)` if `config` is a valid permutation of `0..n`,
/// or `None` otherwise.
pub(crate) fn decode_permutation(config: &[usize], n: usize) -> Option<Vec<usize>> {
    if config.len() != n {
        return None;
    }
    let mut seen = vec![false; n];
    for &task in config {
        if task >= n || seen[task] {
            return None;
        }
        seen[task] = true;
    }
    Some(config.to_vec())
}

/// Return the Lehmer-code dimension vector `[n, n-1, ..., 1]`.
pub(crate) fn lehmer_dims(n: usize) -> Vec<usize> {
    (0..n).rev().map(|i| i + 1).collect()
}
mod bin_packing;
mod boyce_codd_normal_form_violation;
mod capacity_assignment;
pub(crate) mod clustering;
pub(crate) mod conjunctive_boolean_query;
pub(crate) mod conjunctive_query_foldability;
mod consistency_of_database_frequency_tables;
mod cosine_product_integration;
mod dynamic_storage_allocation;
mod ensemble_computation;
pub(crate) mod expected_retrieval_cost;
pub(crate) mod factoring;
mod feasible_register_assignment;
mod flow_shop_scheduling;
mod grouping_by_swapping;
pub(crate) mod integer_expression_membership;
mod job_shop_scheduling;
mod knapsack;
mod kth_largest_m_tuple;
mod longest_common_subsequence;
pub(crate) mod maximum_likelihood_ranking;
mod minimum_axiom_set;
mod minimum_code_generation_one_register;
pub(crate) mod minimum_code_generation_parallel_assignments;
mod minimum_code_generation_unlimited_registers;
pub(crate) mod minimum_decision_tree;
pub(crate) mod minimum_disjunctive_normal_form;
mod minimum_external_macro_data_compression;
mod minimum_fault_detection_test_set;
mod minimum_internal_macro_data_compression;
mod minimum_register_sufficiency_for_loops;
mod minimum_tardiness_sequencing;
mod minimum_weight_and_or_graph;
mod multiprocessor_scheduling;
mod non_liveness_free_petri_net;
mod numerical_3_dimensional_matching;
mod numerical_matching_with_target_sums;
mod open_shop_scheduling;
pub(crate) mod optimum_communication_spanning_tree;
pub(crate) mod paintshop;
pub(crate) mod partially_ordered_knapsack;
pub(crate) mod partition;
mod precedence_constrained_scheduling;
mod preemptive_scheduling;
mod production_planning;
mod rectilinear_picture_compression;
mod register_sufficiency;
pub(crate) mod resource_constrained_scheduling;
mod scheduling_to_minimize_weighted_completion_time;
mod scheduling_with_individual_deadlines;
mod sequencing_to_minimize_maximum_cumulative_cost;
mod sequencing_to_minimize_tardy_task_weight;
mod sequencing_to_minimize_weighted_completion_time;
mod sequencing_to_minimize_weighted_tardiness;
mod sequencing_with_deadlines_and_set_up_times;
mod sequencing_with_release_times_and_deadlines;
mod sequencing_within_intervals;
pub(crate) mod shortest_common_supersequence;
mod square_tiling;
mod stacker_crane;
mod staff_scheduling;
pub(crate) mod string_to_string_correction;
mod subset_product;
mod subset_sum;
pub(crate) mod sum_of_squares_partition;
mod three_partition;
mod timetable_design;

pub use additional_key::AdditionalKey;
pub use betweenness::Betweenness;
pub use bin_packing::BinPacking;
pub use boyce_codd_normal_form_violation::BoyceCoddNormalFormViolation;
pub use capacity_assignment::CapacityAssignment;
pub use clustering::Clustering;
pub use conjunctive_boolean_query::{ConjunctiveBooleanQuery, QueryArg, Relation as CbqRelation};
pub use conjunctive_query_foldability::{ConjunctiveQueryFoldability, Term};
pub use consistency_of_database_frequency_tables::{
    ConsistencyOfDatabaseFrequencyTables, FrequencyTable, KnownValue,
};
pub use cosine_product_integration::CosineProductIntegration;
pub use cyclic_ordering::CyclicOrdering;
pub use dynamic_storage_allocation::DynamicStorageAllocation;
pub use ensemble_computation::EnsembleComputation;
pub use expected_retrieval_cost::ExpectedRetrievalCost;
pub use factoring::Factoring;
pub use feasible_register_assignment::FeasibleRegisterAssignment;
pub use flow_shop_scheduling::FlowShopScheduling;
pub use grouping_by_swapping::GroupingBySwapping;
pub use integer_expression_membership::{IntExpr, IntegerExpressionMembership};
pub use job_shop_scheduling::JobShopScheduling;
pub use knapsack::Knapsack;
pub use kth_largest_m_tuple::KthLargestMTuple;
pub use longest_common_subsequence::LongestCommonSubsequence;
pub use maximum_likelihood_ranking::MaximumLikelihoodRanking;
pub use minimum_axiom_set::MinimumAxiomSet;
pub use minimum_code_generation_one_register::MinimumCodeGenerationOneRegister;
pub use minimum_code_generation_parallel_assignments::MinimumCodeGenerationParallelAssignments;
pub use minimum_code_generation_unlimited_registers::MinimumCodeGenerationUnlimitedRegisters;
pub use minimum_decision_tree::MinimumDecisionTree;
pub use minimum_disjunctive_normal_form::MinimumDisjunctiveNormalForm;
pub use minimum_external_macro_data_compression::MinimumExternalMacroDataCompression;
pub use minimum_fault_detection_test_set::MinimumFaultDetectionTestSet;
pub use minimum_internal_macro_data_compression::MinimumInternalMacroDataCompression;
pub use minimum_register_sufficiency_for_loops::MinimumRegisterSufficiencyForLoops;
pub use minimum_tardiness_sequencing::MinimumTardinessSequencing;
pub use minimum_weight_and_or_graph::MinimumWeightAndOrGraph;
pub use multiprocessor_scheduling::MultiprocessorScheduling;
pub use non_liveness_free_petri_net::NonLivenessFreePetriNet;
pub use numerical_3_dimensional_matching::Numerical3DimensionalMatching;
pub use numerical_matching_with_target_sums::NumericalMatchingWithTargetSums;
pub use open_shop_scheduling::OpenShopScheduling;
pub use optimum_communication_spanning_tree::OptimumCommunicationSpanningTree;
pub use paintshop::PaintShop;
pub use partially_ordered_knapsack::PartiallyOrderedKnapsack;
pub use partition::Partition;
pub use precedence_constrained_scheduling::PrecedenceConstrainedScheduling;
pub use preemptive_scheduling::PreemptiveScheduling;
pub use production_planning::ProductionPlanning;
pub use rectilinear_picture_compression::RectilinearPictureCompression;
pub use register_sufficiency::RegisterSufficiency;
pub use resource_constrained_scheduling::ResourceConstrainedScheduling;
pub use scheduling_to_minimize_weighted_completion_time::SchedulingToMinimizeWeightedCompletionTime;
pub use scheduling_with_individual_deadlines::SchedulingWithIndividualDeadlines;
pub use sequencing_to_minimize_maximum_cumulative_cost::SequencingToMinimizeMaximumCumulativeCost;
pub use sequencing_to_minimize_tardy_task_weight::SequencingToMinimizeTardyTaskWeight;
pub use sequencing_to_minimize_weighted_completion_time::SequencingToMinimizeWeightedCompletionTime;
pub use sequencing_to_minimize_weighted_tardiness::SequencingToMinimizeWeightedTardiness;
pub use sequencing_with_deadlines_and_set_up_times::SequencingWithDeadlinesAndSetUpTimes;
pub use sequencing_with_release_times_and_deadlines::SequencingWithReleaseTimesAndDeadlines;
pub use sequencing_within_intervals::SequencingWithinIntervals;
pub use shortest_common_supersequence::ShortestCommonSupersequence;
pub use square_tiling::SquareTiling;
pub use stacker_crane::StackerCrane;
pub use staff_scheduling::StaffScheduling;
pub use string_to_string_correction::StringToStringCorrection;
pub use subset_product::SubsetProduct;
pub use subset_sum::SubsetSum;
pub use sum_of_squares_partition::SumOfSquaresPartition;
pub use three_partition::ThreePartition;
pub use timetable_design::TimetableDesign;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(boyce_codd_normal_form_violation::canonical_model_example_specs());
    specs.extend(capacity_assignment::canonical_model_example_specs());
    specs.extend(consistency_of_database_frequency_tables::canonical_model_example_specs());
    specs.extend(conjunctive_boolean_query::canonical_model_example_specs());
    specs.extend(conjunctive_query_foldability::canonical_model_example_specs());
    specs.extend(ensemble_computation::canonical_model_example_specs());
    specs.extend(expected_retrieval_cost::canonical_model_example_specs());
    specs.extend(factoring::canonical_model_example_specs());
    specs.extend(grouping_by_swapping::canonical_model_example_specs());
    specs.extend(longest_common_subsequence::canonical_model_example_specs());
    specs.extend(multiprocessor_scheduling::canonical_model_example_specs());
    specs.extend(open_shop_scheduling::canonical_model_example_specs());
    specs.extend(paintshop::canonical_model_example_specs());
    specs.extend(partition::canonical_model_example_specs());
    specs.extend(production_planning::canonical_model_example_specs());
    specs.extend(rectilinear_picture_compression::canonical_model_example_specs());
    specs.extend(scheduling_to_minimize_weighted_completion_time::canonical_model_example_specs());
    specs.extend(scheduling_with_individual_deadlines::canonical_model_example_specs());
    specs.extend(sequencing_within_intervals::canonical_model_example_specs());
    specs.extend(staff_scheduling::canonical_model_example_specs());
    specs.extend(stacker_crane::canonical_model_example_specs());
    specs.extend(timetable_design::canonical_model_example_specs());
    specs.extend(shortest_common_supersequence::canonical_model_example_specs());
    specs.extend(resource_constrained_scheduling::canonical_model_example_specs());
    specs.extend(partially_ordered_knapsack::canonical_model_example_specs());
    specs.extend(string_to_string_correction::canonical_model_example_specs());
    specs.extend(minimum_tardiness_sequencing::canonical_model_example_specs());
    specs.extend(sequencing_to_minimize_weighted_completion_time::canonical_model_example_specs());
    specs.extend(sequencing_to_minimize_weighted_tardiness::canonical_model_example_specs());
    specs.extend(additional_key::canonical_model_example_specs());
    specs.extend(sequencing_to_minimize_maximum_cumulative_cost::canonical_model_example_specs());
    specs.extend(sequencing_to_minimize_tardy_task_weight::canonical_model_example_specs());
    specs.extend(sequencing_with_deadlines_and_set_up_times::canonical_model_example_specs());
    specs.extend(sum_of_squares_partition::canonical_model_example_specs());
    specs.extend(precedence_constrained_scheduling::canonical_model_example_specs());
    specs.extend(job_shop_scheduling::canonical_model_example_specs());
    specs.extend(sequencing_with_release_times_and_deadlines::canonical_model_example_specs());
    specs.extend(flow_shop_scheduling::canonical_model_example_specs());
    specs.extend(bin_packing::canonical_model_example_specs());
    specs.extend(knapsack::canonical_model_example_specs());
    specs.extend(integer_expression_membership::canonical_model_example_specs());
    specs.extend(subset_product::canonical_model_example_specs());
    specs.extend(subset_sum::canonical_model_example_specs());
    specs.extend(numerical_3_dimensional_matching::canonical_model_example_specs());
    specs.extend(numerical_matching_with_target_sums::canonical_model_example_specs());
    specs.extend(three_partition::canonical_model_example_specs());
    specs.extend(cosine_product_integration::canonical_model_example_specs());
    specs.extend(dynamic_storage_allocation::canonical_model_example_specs());
    specs.extend(minimum_code_generation_one_register::canonical_model_example_specs());
    specs.extend(minimum_code_generation_parallel_assignments::canonical_model_example_specs());
    specs.extend(minimum_code_generation_unlimited_registers::canonical_model_example_specs());
    specs.extend(minimum_decision_tree::canonical_model_example_specs());
    specs.extend(minimum_disjunctive_normal_form::canonical_model_example_specs());
    specs.extend(minimum_external_macro_data_compression::canonical_model_example_specs());
    specs.extend(minimum_internal_macro_data_compression::canonical_model_example_specs());
    specs.extend(minimum_register_sufficiency_for_loops::canonical_model_example_specs());
    specs.extend(register_sufficiency::canonical_model_example_specs());
    specs.extend(feasible_register_assignment::canonical_model_example_specs());
    specs.extend(kth_largest_m_tuple::canonical_model_example_specs());
    specs.extend(preemptive_scheduling::canonical_model_example_specs());
    specs.extend(betweenness::canonical_model_example_specs());
    specs.extend(cyclic_ordering::canonical_model_example_specs());
    specs.extend(non_liveness_free_petri_net::canonical_model_example_specs());
    specs.extend(maximum_likelihood_ranking::canonical_model_example_specs());
    specs.extend(clustering::canonical_model_example_specs());
    specs.extend(minimum_weight_and_or_graph::canonical_model_example_specs());
    specs.extend(minimum_fault_detection_test_set::canonical_model_example_specs());
    specs.extend(minimum_axiom_set::canonical_model_example_specs());
    specs.extend(optimum_communication_spanning_tree::canonical_model_example_specs());
    specs.extend(square_tiling::canonical_model_example_specs());
    specs
}
