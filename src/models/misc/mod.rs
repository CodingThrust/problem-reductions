//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`FlowShopScheduling`]: Flow Shop Scheduling (meet deadline on m processors)
//! - [`Knapsack`]: 0-1 Knapsack (maximize value subject to weight capacity)
//! - [`MultiprocessorScheduling`]: Schedule tasks on processors to meet a deadline
//! - [`LongestCommonSubsequence`]: Longest Common Subsequence
//! - [`MinimumTardinessSequencing`]: Minimize tardy tasks in single-machine scheduling
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling
//! - [`PartiallyOrderedKnapsack`]: Knapsack with precedence constraints
//! - [`PrecedenceConstrainedScheduling`]: Schedule unit tasks on processors by deadline
//! - [`SequencingWithReleaseTimesAndDeadlines`]: Single-machine scheduling feasibility
//! - [`SequencingWithinIntervals`]: Schedule tasks within time windows
//! - [`ShortestCommonSupersequence`]: Find a common supersequence of bounded length
//! - [`StringToStringCorrection`]: String-to-String Correction (derive target via deletions and swaps)
//! - [`SubsetSum`]: Find a subset summing to exactly a target value
//! - [`SumOfSquaresPartition`]: Partition integers into K groups minimizing sum of squared group sums

mod bin_packing;
pub(crate) mod factoring;
mod flow_shop_scheduling;
mod knapsack;
mod longest_common_subsequence;
mod minimum_tardiness_sequencing;
mod multiprocessor_scheduling;
pub(crate) mod paintshop;
pub(crate) mod partially_ordered_knapsack;
mod precedence_constrained_scheduling;
mod sequencing_with_release_times_and_deadlines;
mod sequencing_within_intervals;
pub(crate) mod shortest_common_supersequence;
mod staff_scheduling;
pub(crate) mod string_to_string_correction;
mod subset_sum;
pub(crate) mod sum_of_squares_partition;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use flow_shop_scheduling::FlowShopScheduling;
pub use knapsack::Knapsack;
pub use longest_common_subsequence::LongestCommonSubsequence;
pub use minimum_tardiness_sequencing::MinimumTardinessSequencing;
pub use multiprocessor_scheduling::MultiprocessorScheduling;
pub use paintshop::PaintShop;
pub use partially_ordered_knapsack::PartiallyOrderedKnapsack;
pub use precedence_constrained_scheduling::PrecedenceConstrainedScheduling;
pub use sequencing_with_release_times_and_deadlines::SequencingWithReleaseTimesAndDeadlines;
pub use sequencing_within_intervals::SequencingWithinIntervals;
pub use shortest_common_supersequence::ShortestCommonSupersequence;
pub use staff_scheduling::StaffScheduling;
pub use string_to_string_correction::StringToStringCorrection;
pub use subset_sum::SubsetSum;
pub use sum_of_squares_partition::SumOfSquaresPartition;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(factoring::canonical_model_example_specs());
    specs.extend(longest_common_subsequence::canonical_model_example_specs());
    specs.extend(multiprocessor_scheduling::canonical_model_example_specs());
    specs.extend(paintshop::canonical_model_example_specs());
    specs.extend(sequencing_within_intervals::canonical_model_example_specs());
    specs.extend(staff_scheduling::canonical_model_example_specs());
    specs.extend(shortest_common_supersequence::canonical_model_example_specs());
    specs.extend(partially_ordered_knapsack::canonical_model_example_specs());
    specs.extend(string_to_string_correction::canonical_model_example_specs());
    specs.extend(minimum_tardiness_sequencing::canonical_model_example_specs());
    specs.extend(sum_of_squares_partition::canonical_model_example_specs());
    specs.extend(precedence_constrained_scheduling::canonical_model_example_specs());
    specs.extend(sequencing_with_release_times_and_deadlines::canonical_model_example_specs());
    specs
}
