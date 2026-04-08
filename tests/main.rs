#[path = "suites/consecutive_ones_matrix_augmentation.rs"]
mod consecutive_ones_matrix_augmentation;
#[path = "suites/examples.rs"]
mod examples;
#[path = "suites/integration.rs"]
mod integration;
#[path = "suites/jl_parity.rs"]
mod jl_parity;
#[path = "suites/ksatisfiability_simultaneous_incongruences.rs"]
mod ksatisfiability_simultaneous_incongruences;
#[path = "suites/reductions.rs"]
mod reductions;
#[cfg(feature = "ilp-solver")]
#[path = "suites/register_assignment_reductions.rs"]
mod register_assignment_reductions;
#[path = "suites/simultaneous_incongruences.rs"]
mod simultaneous_incongruences;
