use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAllocator;

static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static COUNT_ALLOCATIONS: Cell<bool> = const { Cell::new(false) };
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        COUNT_ALLOCATIONS.with(|enabled| {
            if enabled.get() {
                ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        });
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        COUNT_ALLOCATIONS.with(|enabled| {
            if enabled.get() {
                ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        });
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

fn count_allocations<T>(f: impl FnOnce() -> T) -> (T, usize) {
    ALLOCATION_COUNT.store(0, Ordering::Relaxed);
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(true));
    let result = f();
    COUNT_ALLOCATIONS.with(|enabled| enabled.set(false));
    let allocations = ALLOCATION_COUNT.swap(0, Ordering::Relaxed);
    (result, allocations)
}

fn yes_instance() -> BoundedComponentSpanningForest<SimpleGraph, i32> {
    let graph = SimpleGraph::new(
        8,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (0, 7),
            (1, 5),
            (2, 6),
        ],
    );
    BoundedComponentSpanningForest::new(graph, vec![2, 3, 1, 2, 3, 1, 2, 1], 3, 6)
}

fn no_instance() -> BoundedComponentSpanningForest<SimpleGraph, i32> {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]);
    BoundedComponentSpanningForest::new(graph, vec![1, 1, 1, 1, 1, 1], 2, 2)
}

#[test]
fn test_bounded_component_spanning_forest_creation() {
    let problem = yes_instance();
    assert_eq!(problem.graph().num_vertices(), 8);
    assert_eq!(problem.graph().num_edges(), 10);
    assert_eq!(problem.weights(), &[2, 3, 1, 2, 3, 1, 2, 1]);
    assert_eq!(problem.max_components(), 3);
    assert_eq!(problem.max_weight(), &6);
    assert_eq!(problem.num_vertices(), 8);
    assert_eq!(problem.num_edges(), 10);
    assert_eq!(problem.dims(), vec![3; 8]);
    assert!(problem.is_weighted());
}

#[test]
fn test_bounded_component_spanning_forest_yes_instance() {
    let problem = yes_instance();
    assert!(problem.evaluate(&[0, 0, 1, 1, 1, 2, 2, 0]));
    assert!(problem.is_valid_solution(&[0, 0, 1, 1, 1, 2, 2, 0]));
}

#[test]
fn test_bounded_component_spanning_forest_rejects_weight_overflow() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 0, 1, 1, 1, 1, 2, 0]));
}

#[test]
fn test_bounded_component_spanning_forest_rejects_disconnected_component() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 1, 0, 1, 1, 2, 2, 0]));
}

#[test]
fn test_bounded_component_spanning_forest_rejects_out_of_range_component() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 0, 1, 1, 1, 2, 2, 3]));
}

#[test]
fn test_bounded_component_spanning_forest_rejects_wrong_length() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 0, 1]));
}

#[test]
fn test_bounded_component_spanning_forest_evaluate_uses_fixed_allocation_budget() {
    let problem = BoundedComponentSpanningForest::new(SimpleGraph::empty(16), vec![1; 16], 16, 1);
    let config: Vec<usize> = (0..16).collect();

    let (is_valid, allocations) = count_allocations(|| problem.evaluate(&config));

    assert!(is_valid);
    assert!(
        allocations <= 6,
        "expected evaluate to use only a fixed number of allocations, got {allocations}"
    );
}

#[test]
fn test_bounded_component_spanning_forest_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: BoundedComponentSpanningForest<SimpleGraph, i32> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(round_trip.graph().num_vertices(), 8);
    assert_eq!(round_trip.weights(), &[2, 3, 1, 2, 3, 1, 2, 1]);
    assert_eq!(round_trip.max_components(), 3);
    assert_eq!(round_trip.max_weight(), &6);
}

#[test]
fn test_bounded_component_spanning_forest_solver_yes_and_no_instances() {
    let solver = BruteForce::new();

    let yes_problem = yes_instance();
    let solution = solver.find_witness(&yes_problem);
    assert!(solution.is_some());
    assert!(yes_problem.evaluate(solution.as_ref().unwrap()));

    let no_problem = no_instance();
    assert!(solver.find_witness(&no_problem).is_none());
}

#[test]
fn test_bounded_component_spanning_forest_paper_example() {
    let problem = yes_instance();
    let config = vec![0, 0, 1, 1, 1, 2, 2, 0];
    assert!(problem.evaluate(&config));

    let solver = BruteForce::new();
    let all_solutions = solver.find_all_witnesses(&problem);
    assert!(all_solutions.iter().any(|solution| solution == &config));
}

#[test]
#[should_panic(expected = "max_components must be at least 1")]
fn test_bounded_component_spanning_forest_rejects_zero_max_components_in_constructor() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let _ = BoundedComponentSpanningForest::new(graph, vec![1, 1], 0, 1);
}

#[test]
fn test_bounded_component_spanning_forest_accepts_k_larger_than_num_vertices() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let problem = BoundedComponentSpanningForest::new(graph, vec![1, 1], 5, 2);
    // K > |V| is mathematically harmless — just means fewer than K components possible
    assert_eq!(problem.max_components(), 5);
    assert!(problem.evaluate(&[0, 0]));
}

#[test]
#[should_panic(expected = "weights must be nonnegative")]
fn test_bounded_component_spanning_forest_rejects_negative_weights_in_constructor() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let _ = BoundedComponentSpanningForest::new(graph, vec![1, -1], 1, 1);
}

#[test]
#[should_panic(expected = "max_weight must be positive")]
fn test_bounded_component_spanning_forest_rejects_nonpositive_bound_in_constructor() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let _ = BoundedComponentSpanningForest::new(graph, vec![1, 1], 1, 0);
}
