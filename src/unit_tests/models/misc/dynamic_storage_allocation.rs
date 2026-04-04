use crate::models::misc::DynamicStorageAllocation;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn example_problem() -> DynamicStorageAllocation {
    // 5 items, memory_size = 6
    DynamicStorageAllocation::new(
        vec![(0, 3, 2), (0, 2, 3), (1, 4, 1), (2, 5, 3), (3, 5, 2)],
        6,
    )
}

#[test]
fn test_dynamic_storage_allocation_basic() {
    let problem = example_problem();
    assert_eq!(problem.num_items(), 5);
    assert_eq!(problem.memory_size(), 6);
    assert_eq!(problem.items().len(), 5);
    // dims: D - s(a) + 1 for each item
    // sizes are 2, 3, 1, 3, 2 => dims are 5, 4, 6, 4, 5
    assert_eq!(problem.dims(), vec![5, 4, 6, 4, 5]);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(
        <DynamicStorageAllocation as Problem>::NAME,
        "DynamicStorageAllocation"
    );
    assert_eq!(<DynamicStorageAllocation as Problem>::variant(), vec![]);
}

#[test]
fn test_dynamic_storage_allocation_evaluate_feasible() {
    let problem = example_problem();
    // Solution from the issue: σ = [0, 2, 5, 2, 0] (0-indexed)
    assert_eq!(problem.evaluate(&[0, 2, 5, 2, 0]), Or(true));
}

#[test]
fn test_dynamic_storage_allocation_evaluate_infeasible() {
    let problem = example_problem();
    // All items at address 0 - should overlap
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0]), Or(false));
}

#[test]
fn test_dynamic_storage_allocation_rejects_invalid_config_length() {
    let problem = example_problem();
    assert_eq!(problem.evaluate(&[0, 2, 5]), Or(false));
    assert_eq!(problem.evaluate(&[0, 2, 5, 2, 0, 1]), Or(false));
}

#[test]
fn test_dynamic_storage_allocation_rejects_out_of_bounds() {
    let problem = example_problem();
    // Item 0 has size 2, so max start is 4 (0..=4). Start at 5 => 5+2=7 > 6
    assert_eq!(problem.evaluate(&[5, 0, 0, 0, 0]), Or(false));
}

#[test]
fn test_dynamic_storage_allocation_solver_finds_witness() {
    // Use a small instance for brute-force
    let problem = DynamicStorageAllocation::new(vec![(0, 2, 1), (1, 3, 1)], 2);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&witness), Or(true));
}

#[test]
fn test_dynamic_storage_allocation_unsatisfiable_instance() {
    // Two items overlap in time, both size 3, memory = 4: can't fit without overlap
    let problem = DynamicStorageAllocation::new(vec![(0, 2, 3), (0, 2, 3)], 4);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_dynamic_storage_allocation_serialization_round_trip() {
    let problem = example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "items": [[0, 3, 2], [0, 2, 3], [1, 4, 1], [2, 5, 3], [3, 5, 2]],
            "memory_size": 6,
        })
    );

    let restored: DynamicStorageAllocation = serde_json::from_value(json).unwrap();
    assert_eq!(restored.items(), problem.items());
    assert_eq!(restored.memory_size(), problem.memory_size());
}

#[test]
fn test_dynamic_storage_allocation_deserialization_rejects_invalid() {
    let invalid_cases = [
        // Empty items
        serde_json::json!({
            "items": [],
            "memory_size": 6,
        }),
        // Zero memory_size
        serde_json::json!({
            "items": [[0, 2, 1]],
            "memory_size": 0,
        }),
        // Zero size item
        serde_json::json!({
            "items": [[0, 2, 0]],
            "memory_size": 6,
        }),
        // departure <= arrival
        serde_json::json!({
            "items": [[3, 2, 1]],
            "memory_size": 6,
        }),
        // size > memory_size
        serde_json::json!({
            "items": [[0, 2, 7]],
            "memory_size": 6,
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<DynamicStorageAllocation>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one item")]
fn test_dynamic_storage_allocation_empty_items_panics() {
    DynamicStorageAllocation::new(vec![], 6);
}

#[test]
#[should_panic(expected = "zero size")]
fn test_dynamic_storage_allocation_zero_size_panics() {
    DynamicStorageAllocation::new(vec![(0, 2, 0)], 6);
}

#[test]
#[should_panic(expected = "departure")]
fn test_dynamic_storage_allocation_bad_departure_panics() {
    DynamicStorageAllocation::new(vec![(3, 2, 1)], 6);
}

#[test]
fn test_dynamic_storage_allocation_non_overlapping_time_any_address() {
    // Two items that don't overlap in time can share any addresses
    let problem = DynamicStorageAllocation::new(vec![(0, 2, 3), (2, 4, 3)], 3);
    // Both at address 0, but they don't overlap in time (d(a)=2 <= r(a')=2)
    assert_eq!(problem.evaluate(&[0, 0]), Or(true));
}
