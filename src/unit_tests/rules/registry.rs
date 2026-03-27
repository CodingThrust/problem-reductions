use super::*;
use crate::expr::Expr;
use crate::rules::registry::EdgeCapabilities;
use std::path::Path;

/// Dummy reduce_fn for unit tests that don't exercise runtime reduction.
fn dummy_reduce_fn(_: &dyn std::any::Any) -> Box<dyn crate::rules::traits::DynReductionResult> {
    unimplemented!("dummy reduce_fn for testing")
}

fn dummy_reduce_aggregate_fn(
    _: &dyn std::any::Any,
) -> Box<dyn crate::rules::traits::DynAggregateReductionResult> {
    unimplemented!("dummy reduce_aggregate_fn for testing")
}

fn dummy_overhead_eval_fn(_: &dyn std::any::Any) -> ProblemSize {
    ProblemSize::new(vec![])
}

#[test]
fn test_reduction_overhead_evaluate() {
    let overhead = ReductionOverhead::new(vec![
        ("n", Expr::Const(3.0) * Expr::Var("m")),
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
        overhead_fn: || ReductionOverhead::new(vec![("n", Expr::Const(2.0) * Expr::Var("n"))]),
        module_path: "test::module",
        reduce_fn: Some(dummy_reduce_fn),
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
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
        reduce_fn: Some(dummy_reduce_fn),
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
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
        reduce_fn: Some(dummy_reduce_fn),
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
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
        reduce_fn: Some(dummy_reduce_fn),
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
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
        reduce_fn: Some(dummy_reduce_fn),
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
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
        reduce_fn: Some(dummy_reduce_fn),
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
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
        reduce_fn: Some(dummy_reduce_fn),
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
        overhead_eval_fn: dummy_overhead_eval_fn,
    };
    assert!(entry.is_base_reduction());
}

#[test]
fn test_reduction_entry_can_store_aggregate_executor() {
    let entry = ReductionEntry {
        source_name: "A",
        target_name: "B",
        source_variant_fn: || vec![("graph", "SimpleGraph")],
        target_variant_fn: || vec![("graph", "SimpleGraph")],
        overhead_fn: || ReductionOverhead::default(),
        module_path: "test::module",
        reduce_fn: None,
        reduce_aggregate_fn: Some(dummy_reduce_aggregate_fn),
        capabilities: EdgeCapabilities::aggregate_only(),
        overhead_eval_fn: dummy_overhead_eval_fn,
    };

    assert!(entry.reduce_fn.is_none());
    assert!(entry.reduce_aggregate_fn.is_some());
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
    use crate::models::misc::Factoring;

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
    use crate::models::misc::Factoring;
    use crate::registry::VariantEntry;

    let problem = Factoring::new(8, 8, 100);

    let entry = inventory::iter::<VariantEntry>()
        .find(|e| e.name == "Factoring")
        .unwrap();

    let input = ProblemSize::new(vec![("m", problem.m()), ("n", problem.n())]);
    cross_check_complexity(entry, &problem as &dyn std::any::Any, &input);
}

type EndpointKey = (String, Vec<(String, String)>, String, Vec<(String, String)>);

fn exact_endpoint_key(entry: &ReductionEntry) -> EndpointKey {
    let source_variant = entry
        .source_variant()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let target_variant = entry
        .target_variant()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    (
        entry.source_name.to_string(),
        source_variant,
        entry.target_name.to_string(),
        target_variant,
    )
}

fn walk_rust_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            walk_rust_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

fn reduction_attribute_has_extra_top_level_field(path: &Path) -> bool {
    let contents = std::fs::read_to_string(path).unwrap();
    let mut in_reduction_attr = false;
    let mut attr_text = String::new();

    for line in contents.lines() {
        if !in_reduction_attr
            && (line.contains("#[reduction(") || line.contains("#[$crate::reduction("))
        {
            in_reduction_attr = true;
            attr_text.clear();
        }
        if in_reduction_attr {
            attr_text.push_str(line.trim());
            attr_text.push(' ');
        }
        if in_reduction_attr && line.contains(")]") {
            let normalized = attr_text.split_whitespace().collect::<Vec<_>>().join(" ");
            let body = normalized
                .strip_prefix("#[reduction(")
                .or_else(|| normalized.strip_prefix("#[$crate::reduction("))
                .unwrap_or(&normalized);
            let body = body.strip_suffix(")]").unwrap_or(body).trim();
            if !body.starts_with("overhead =") {
                return true;
            }
            in_reduction_attr = false;
        }
    }

    false
}

#[test]
fn every_registered_reduction_has_unique_exact_endpoints() {
    let entries = reduction_entries();
    let mut seen = std::collections::HashMap::new();
    for entry in &entries {
        let key = exact_endpoint_key(entry);
        if let Some(prev) = seen.insert(key.clone(), entry) {
            panic!(
                "Duplicate exact reduction endpoint {:?}: {} {:?} -> {} {:?} vs {} {:?} -> {} {:?}",
                key,
                prev.source_name,
                prev.source_variant(),
                prev.target_name,
                prev.target_variant(),
                entry.source_name,
                entry.source_variant(),
                entry.target_name,
                entry.target_variant(),
            );
        }
    }
}

#[test]
fn every_registered_reduction_has_non_empty_names() {
    for entry in reduction_entries() {
        assert!(
            !entry.source_name.is_empty(),
            "Empty source_name for reduction targeting {}",
            entry.target_name,
        );
        assert!(
            !entry.target_name.is_empty(),
            "Empty target_name for reduction sourced from {}",
            entry.source_name,
        );
    }
}

#[test]
fn repo_reductions_use_overhead_only_attribute() {
    let mut rust_files = Vec::new();
    walk_rust_files(Path::new("src/rules"), &mut rust_files);

    let offenders: Vec<_> = rust_files
        .into_iter()
        .filter(|path| reduction_attribute_has_extra_top_level_field(path))
        .collect();

    assert!(
        offenders.is_empty(),
        "extra top-level reduction attribute still present in: {:?}",
        offenders,
    );
}

#[test]
fn test_edge_capabilities_constructors() {
    let wo = EdgeCapabilities::witness_only();
    assert!(wo.witness);
    assert!(!wo.aggregate);

    let ao = EdgeCapabilities::aggregate_only();
    assert!(!ao.witness);
    assert!(ao.aggregate);

    let both = EdgeCapabilities::both();
    assert!(both.witness);
    assert!(both.aggregate);
}

#[test]
fn test_edge_capabilities_default_is_witness_only() {
    let default = EdgeCapabilities::default();
    assert_eq!(default, EdgeCapabilities::witness_only());
}

#[test]
fn test_edge_capabilities_serde_roundtrip() {
    let caps = EdgeCapabilities::both();
    let json = serde_json::to_string(&caps).unwrap();
    let back: EdgeCapabilities = serde_json::from_str(&json).unwrap();
    assert_eq!(caps, back);
}
