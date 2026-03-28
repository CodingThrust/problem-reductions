//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`AdditionalKey`]: Determine whether a relational schema has an additional candidate key
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`BoyceCoddNormalFormViolation`]: Boyce-Codd Normal Form Violation (BCNF)
//! - [`ConsistencyOfDatabaseFrequencyTables`]: Pairwise frequency-table consistency
//! - [`ConjunctiveBooleanQuery`]: Evaluate a conjunctive Boolean query over relations
//! - [`ConjunctiveQueryFoldability`]: Conjunctive Query Foldability
//! - [`ExpectedRetrievalCost`]: Allocate records to circular sectors within a latency bound
//! - [`Factoring`]: Integer factorization
//! - [`FlowShopScheduling`]: Flow Shop Scheduling (meet deadline on m processors)
//! - [`GroupingBySwapping`]: Group equal symbols into contiguous blocks by adjacent swaps
//! - [`JobShopScheduling`]: Minimize makespan with per-job processor routes
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`MultiprocessorScheduling`]: Schedule tasks on processors to meet a deadline
//! - [`LongestCommonSubsequence`]: Longest Common Subsequence
//! - [`MinimumTardinessSequencing`]: Minimize tardy tasks in single-machine scheduling
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`Partition`]: Partition a multiset into two equal-sum subsets
//! - [`PartiallyOrderedKnapsack`]: Knapsack with precedence constraints
//! - [`PrecedenceConstrainedScheduling`]: Schedule unit tasks on processors by deadline
//! - [`RectilinearPictureCompression`]: Cover 1-entries with bounded rectangles
//! - [`RegisterSufficiency`]: Evaluate DAG computation with bounded registers
//! - [`ResourceConstrainedScheduling`]: Schedule unit-length tasks on processors with resource constraints
//! - [`SchedulingWithIndividualDeadlines`]: Meet per-task deadlines on parallel processors
//! - [`StackerCrane`]: Minimize the total length of a closed walk through required arcs
//! - [`SequencingToMinimizeMaximumCumulativeCost`]: Keep every cumulative schedule cost prefix under a bound
//! - [`SequencingToMinimizeWeightedCompletionTime`]: Minimize total weighted completion time
//! - [`SequencingToMinimizeWeightedTardiness`]: Decide whether a schedule meets a weighted tardiness bound
//! - [`SequencingWithReleaseTimesAndDeadlines`]: Single-machine scheduling feasibility
//! - [`SequencingWithinIntervals`]: Schedule tasks within time windows
//! - [`ShortestCommonSupersequence`]: Find a common supersequence of bounded length
//! - [`TimetableDesign`]: Schedule craftsmen on tasks across work periods
//! - [`StringToStringCorrection`]: String-to-String Correction (derive target via deletions and swaps)
//! - [`SubsetSum`]: Find a subset summing to exactly a target value
//! - [`SumOfSquaresPartition`]: Partition integers into K groups minimizing sum of squared group sums

pub(crate) mod additional_key;

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

/// Return the Lehmer-code dimension vector `[n, n-1, ..., 1]`.
pub(crate) fn lehmer_dims(n: usize) -> Vec<usize> {
    (0..n).rev().map(|i| i + 1).collect()
}
mod bin_packing;
mod boyce_codd_normal_form_violation;
mod capacity_assignment;
pub(crate) mod conjunctive_boolean_query;
pub(crate) mod conjunctive_query_foldability;
mod consistency_of_database_frequency_tables;
mod ensemble_computation;
pub(crate) mod expected_retrieval_cost;
pub(crate) mod factoring;
mod flow_shop_scheduling;
mod grouping_by_swapping;
mod job_shop_scheduling;
mod knapsack;
mod longest_common_subsequence;
mod minimum_tardiness_sequencing;
mod multiprocessor_scheduling;
pub(crate) mod paintshop;
pub(crate) mod partially_ordered_knapsack;
pub(crate) mod partition;
mod precedence_constrained_scheduling;
mod rectilinear_picture_compression;
mod register_sufficiency;
pub(crate) mod resource_constrained_scheduling;
mod scheduling_with_individual_deadlines;
mod sequencing_to_minimize_maximum_cumulative_cost;
mod sequencing_to_minimize_weighted_completion_time;
mod sequencing_to_minimize_weighted_tardiness;
mod sequencing_with_release_times_and_deadlines;
mod sequencing_within_intervals;
pub(crate) mod shortest_common_supersequence;
mod stacker_crane;
mod staff_scheduling;
pub(crate) mod string_to_string_correction;
mod subset_sum;
pub(crate) mod sum_of_squares_partition;
mod three_partition;
mod timetable_design;

pub use additional_key::AdditionalKey;
pub use bin_packing::BinPacking;
pub use boyce_codd_normal_form_violation::BoyceCoddNormalFormViolation;
pub use capacity_assignment::CapacityAssignment;
pub use conjunctive_boolean_query::{ConjunctiveBooleanQuery, QueryArg, Relation as CbqRelation};
pub use conjunctive_query_foldability::{ConjunctiveQueryFoldability, Term};
pub use consistency_of_database_frequency_tables::{
    ConsistencyOfDatabaseFrequencyTables, FrequencyTable, KnownValue,
};
pub use ensemble_computation::EnsembleComputation;
pub use expected_retrieval_cost::ExpectedRetrievalCost;
pub use factoring::Factoring;
pub use flow_shop_scheduling::FlowShopScheduling;
pub use grouping_by_swapping::GroupingBySwapping;
pub use job_shop_scheduling::JobShopScheduling;
pub use knapsack::Knapsack;
pub use longest_common_subsequence::LongestCommonSubsequence;
pub use minimum_tardiness_sequencing::MinimumTardinessSequencing;
pub use multiprocessor_scheduling::MultiprocessorScheduling;
pub use paintshop::PaintShop;
pub use partially_ordered_knapsack::PartiallyOrderedKnapsack;
pub use partition::Partition;
pub use precedence_constrained_scheduling::PrecedenceConstrainedScheduling;
pub use rectilinear_picture_compression::RectilinearPictureCompression;
pub use register_sufficiency::RegisterSufficiency;
pub use resource_constrained_scheduling::ResourceConstrainedScheduling;
pub use scheduling_with_individual_deadlines::SchedulingWithIndividualDeadlines;
pub use sequencing_to_minimize_maximum_cumulative_cost::SequencingToMinimizeMaximumCumulativeCost;
pub use sequencing_to_minimize_weighted_completion_time::SequencingToMinimizeWeightedCompletionTime;
pub use sequencing_to_minimize_weighted_tardiness::SequencingToMinimizeWeightedTardiness;
pub use sequencing_with_release_times_and_deadlines::SequencingWithReleaseTimesAndDeadlines;
pub use sequencing_within_intervals::SequencingWithinIntervals;
pub use shortest_common_supersequence::ShortestCommonSupersequence;
pub use stacker_crane::StackerCrane;
pub use staff_scheduling::StaffScheduling;
pub use string_to_string_correction::StringToStringCorrection;
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
    specs.extend(paintshop::canonical_model_example_specs());
    specs.extend(partition::canonical_model_example_specs());
    specs.extend(rectilinear_picture_compression::canonical_model_example_specs());
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
    specs.extend(sum_of_squares_partition::canonical_model_example_specs());
    specs.extend(precedence_constrained_scheduling::canonical_model_example_specs());
    specs.extend(job_shop_scheduling::canonical_model_example_specs());
    specs.extend(sequencing_with_release_times_and_deadlines::canonical_model_example_specs());
    specs.extend(flow_shop_scheduling::canonical_model_example_specs());
    specs.extend(bin_packing::canonical_model_example_specs());
    specs.extend(knapsack::canonical_model_example_specs());
    specs.extend(subset_sum::canonical_model_example_specs());
    specs.extend(three_partition::canonical_model_example_specs());
    specs.extend(register_sufficiency::canonical_model_example_specs());
    specs
}
