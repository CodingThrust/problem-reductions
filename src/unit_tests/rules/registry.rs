use super::*;
use crate::expr::Expr;

/// Dummy reduce_fn for unit tests that don't exercise runtime reduction.
fn dummy_reduce_fn(_: &dyn std::any::Any) -> Box<dyn crate::rules::traits::DynReductionResult> {
    unimplemented!("dummy reduce_fn for testing")
}

fn dummy_overhead_eval_fn(_: &dyn std::any::Any) -> ProblemSize {
    ProblemSize::new(vec![])
}

#[test]
fn test_reduction_overhead_evaluate() {
    let overhead = ReductionOverhead::new(vec![
        ("n", Expr::mul(Expr::Const(3.0), Expr::Var("m"))),
        ("m", Expr::pow(Expr::Var("m"), Expr::Const(2.0))),
    ]);

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
        source_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        target_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        overhead_fn: || {
            ReductionOverhead::new(vec![("n", Expr::mul(Expr::Const(2.0), Expr::Var("n")))])
        },
        module_path: "test::module",
        reduce_fn: dummy_reduce_fn,
        overhead_eval_fn: dummy_overhead_eval_fn,
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
        source_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        target_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
        reduce_fn: dummy_reduce_fn,
        overhead_eval_fn: dummy_overhead_eval_fn,
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
        source_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        target_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
        reduce_fn: dummy_reduce_fn,
        overhead_eval_fn: dummy_overhead_eval_fn,
    };
    assert!(entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_source_weighted() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "i32")],
        target_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
        reduce_fn: dummy_reduce_fn,
        overhead_eval_fn: dummy_overhead_eval_fn,
    };
    assert!(!entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_target_weighted() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
        target_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "f64")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
        reduce_fn: dummy_reduce_fn,
        overhead_eval_fn: dummy_overhead_eval_fn,
    };
    assert!(!entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_both_weighted() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "i32")],
        target_variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "f64")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
        reduce_fn: dummy_reduce_fn,
        overhead_eval_fn: dummy_overhead_eval_fn,
    };
    assert!(!entry.is_base_reduction());
}

#[test]
fn test_is_base_reduction_no_weight_key() {
    // If no weight key is present, assume unweighted (base)
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant_fn: || vec![("graph", "SimpleGraph")],
        target_variant_fn: || vec![("graph", "SimpleGraph")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
        reduce_fn: dummy_reduce_fn,
        overhead_eval_fn: dummy_overhead_eval_fn,
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

/// Build a ProblemSize from an overhead's input variables by calling the eval fn
/// on the source problem instance and collecting field values via the overhead.
///
/// This cross-checks compiled eval (calls getters directly) against symbolic eval
/// (looks up variables in a ProblemSize hashmap).
fn cross_check_overhead(entry: &ReductionEntry, src: &dyn std::any::Any, input: &ProblemSize) {
    let compiled = (entry.overhead_eval_fn)(src);
    let symbolic = entry.overhead().evaluate_output_size(input);

    for (field, _) in &entry.overhead().output_size {
        assert_eq!(
            compiled.get(field),
            symbolic.get(field),
            "overhead field '{}' mismatch for {}→{}: compiled={:?}, symbolic={:?}",
            field,
            entry.source_name,
            entry.target_name,
            compiled.get(field),
            symbolic.get(field),
        );
    }
}

/// Cross-check complexity_eval_fn against symbolic Expr evaluation.
fn cross_check_complexity(
    entry: &crate::registry::VariantEntry,
    src: &dyn std::any::Any,
    input: &ProblemSize,
) {
    let compiled = (entry.complexity_eval_fn)(src);
    let parsed = crate::expr::Expr::parse(entry.complexity);
    let symbolic = parsed.eval(input);

    let diff = (compiled - symbolic).abs();
    let tol = 1e-6 * symbolic.abs().max(1.0);
    assert!(
        diff < tol,
        "complexity mismatch for {} ({}): compiled={compiled}, symbolic={symbolic}, expr=\"{}\"",
        entry.name,
        entry
            .variant()
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", "),
        entry.complexity,
    );
}

#[test]
fn test_overhead_eval_fn_cross_check_mis_to_mvc() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 5)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i32; 6]);

    let entry = inventory::iter::<ReductionEntry>()
        .find(|e| e.source_name == "MaximumIndependentSet" && e.target_name == "MinimumVertexCover")
        .unwrap();

    let input = ProblemSize::new(vec![
        ("num_vertices", problem.num_vertices()),
        ("num_edges", problem.num_edges()),
    ]);
    cross_check_overhead(entry, &problem as &dyn std::any::Any, &input);
}

#[test]
fn test_overhead_eval_fn_cross_check_factoring_to_ilp() {
    use crate::models::specialized::Factoring;

    let problem = Factoring::new(3, 4, 42);

    let entry = inventory::iter::<ReductionEntry>()
        .find(|e| e.source_name == "Factoring" && e.target_name == "ILP")
        .unwrap();

    let input = ProblemSize::new(vec![
        ("num_bits_first", problem.num_bits_first()),
        ("num_bits_second", problem.num_bits_second()),
    ]);
    cross_check_overhead(entry, &problem as &dyn std::any::Any, &input);
}

#[test]
fn test_complexity_eval_fn_cross_check_mis() {
    use crate::models::graph::MaximumIndependentSet;
    use crate::registry::VariantEntry;
    use crate::topology::SimpleGraph;

    let graph = SimpleGraph::new(10, vec![(0, 1), (1, 2)]);
    let problem = MaximumIndependentSet::new(graph, vec![1i32; 10]);

    let entry = inventory::iter::<VariantEntry>()
        .find(|e| {
            e.name == "MaximumIndependentSet"
                && e.variant()
                    .iter()
                    .any(|(k, v)| *k == "graph" && *v == "SimpleGraph")
                && e.variant()
                    .iter()
                    .any(|(k, v)| *k == "weight" && *v == "i32")
        })
        .unwrap();

    let input = ProblemSize::new(vec![("num_vertices", problem.num_vertices())]);
    cross_check_complexity(entry, &problem as &dyn std::any::Any, &input);
}

#[test]
fn test_complexity_eval_fn_cross_check_factoring() {
    use crate::models::specialized::Factoring;
    use crate::registry::VariantEntry;

    let problem = Factoring::new(8, 8, 100);

    let entry = inventory::iter::<VariantEntry>()
        .find(|e| e.name == "Factoring")
        .unwrap();

    let input = ProblemSize::new(vec![("m", problem.m()), ("n", problem.n())]);
    cross_check_complexity(entry, &problem as &dyn std::any::Any, &input);
}
