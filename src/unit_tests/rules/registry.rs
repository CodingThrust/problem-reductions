use super::*;
use crate::poly;

#[test]
fn test_reduction_overhead_evaluate() {
    let overhead = ReductionOverhead::new(vec![("n", poly!(3 * m)), ("m", poly!(m ^ 2))]);

    let input = ProblemSize::new(vec![("m", 4)]);
    let output = overhead.evaluate_output_size(&input);

    assert_eq!(output.get("n"), Some(12)); // 3 * 4
    assert_eq!(output.get("m"), Some(16)); // 4^2
}

#[test]
fn test_reduction_overhead_default() {
    let overhead = ReductionOverhead::default();
    assert!(overhead.output_size.is_empty());
}

#[test]
fn test_reduction_entry_overhead() {
    let entry = ReductionEntry {
        source_name: "TestSource",
        target_name: "TestTarget",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        overhead_fn: || ReductionOverhead::new(vec![("n", poly!(2 * n))]),
        module_path: "test::module",
    };

    let overhead = entry.overhead();
    let input = ProblemSize::new(vec![("n", 5)]);
    let output = overhead.evaluate_output_size(&input);
    assert_eq!(output.get("n"), Some(10));
}

#[test]
fn test_reduction_entry_debug() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
    };

    let debug_str = format!("{:?}", entry);
    assert!(debug_str.contains("A"));
    assert!(debug_str.contains("B"));
}

#[test]
fn test_is_base_reduction_unweighted() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
    };
    assert!(entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_source_weighted() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "i32")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
    };
    assert!(!entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_target_weighted() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "f64")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
    };
    assert!(!entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_both_weighted() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "i32")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "f64")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
    };
    assert!(!entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_no_weight_key() {
    // If no weight key is present, assume unweighted (base)
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant: &[("graph", "SimpleGraph")],
        target_variant: &[("graph", "SimpleGraph")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
    };
    assert!(entry.is_base_reduction());
}

#[test]
fn test_reduction_entries_registered() {
    let entries: Vec<_> = inventory::iter::<ReductionEntry>().collect();

    // Should have at least some registered reductions
    assert!(entries.len() >= 10);

    // Check specific reductions exist
    assert!(
        entries
            .iter()
            .any(|e| e.source_name == "MaximumIndependentSet"
                && e.target_name == "MinimumVertexCover")
    );
}
