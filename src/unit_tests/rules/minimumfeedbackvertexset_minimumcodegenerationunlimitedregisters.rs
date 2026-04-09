use super::{issue_example_source, ReductionFVSToCodeGen};
use crate::models::misc::MinimumCodeGenerationUnlimitedRegisters;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;

#[test]
fn test_minimumfeedbackvertexset_to_minimumcodegenerationunlimitedregisters_closed_loop() {
    let source = issue_example_source();
    let reduction: ReductionFVSToCodeGen =
        ReduceTo::<MinimumCodeGenerationUnlimitedRegisters>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumFeedbackVertexSet -> MinimumCodeGenerationUnlimitedRegisters closed loop",
    );
}
