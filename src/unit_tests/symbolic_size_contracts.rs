use crate::expr::{Expr, ExprNode};
use crate::models::graph::{MaximumClique, MaximumIndependentSet};
use crate::registry::{declared_size_fields, load_dyn};
use crate::rules::registry::{reduction_entries, ReductionSizeContract, ReductionSizeDeclarations};
use crate::rules::{ReduceTo, ReductionEdgeData, ReductionGraph, ReductionMode, ReductionResult};
use crate::size_bound::{BoundVector, SizeBound, SizeBoundError};
use crate::size_map::{SizeMap, SizeMapError};
use crate::topology::{Graph, SimpleGraph};
use crate::types::ProblemSize;
use crate::Problem;
use std::collections::{BTreeMap, BTreeSet};

fn contract(exact: &[(&'static str, &str)], bounds: &[(&'static str, &str)]) -> ReductionEdgeData {
    ReductionEdgeData {
        size_contract: ReductionSizeContract::new(
            "synthetic edge",
            ReductionSizeDeclarations {
                exact: exact
                    .iter()
                    .map(|(field, expression)| (*field, Expr::try_parse(expression).unwrap()))
                    .collect(),
                bounds: bounds
                    .iter()
                    .map(|(field, expression)| (*field, Expr::try_parse(expression).unwrap()))
                    .collect(),
                unavailable: vec![],
            },
        ),
        reduce_fn: Some(|_| panic!("symbolic size search must not execute reductions")),
        reduce_aggregate_fn: None,
        turing: false,
    }
}

fn variant_map(fields: Vec<(&'static str, &'static str)>) -> BTreeMap<String, String> {
    fields
        .into_iter()
        .map(|(key, value)| {
            let value = if key == "graph" && value.is_empty() {
                "SimpleGraph"
            } else {
                value
            };
            (key.to_string(), value.to_string())
        })
        .collect()
}

#[test]
fn symbolic_size_contracts() {
    let canonical = Expr::try_parse("n * (n - 1) / 2 - m").unwrap();
    assert_eq!(canonical.to_string(), "-1 * m + n * (-1 + n) * 2^-1");
    let encoded = serde_json::to_string(&canonical).unwrap();
    let decoded: Expr = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, canonical);
    assert_eq!(decoded.to_string(), canonical.to_string());

    let decimal = Expr::try_parse("2.372").unwrap();
    assert!(matches!(
        decimal.node(),
        ExprNode::Const(value)
            if value == &num_rational::BigRational::new(593.into(), 250.into())
    ));

    for index in 0..2_000 {
        let name = format!("dynamic_size_{index}");
        let expression = Expr::try_parse(&format!("{name} + 1")).unwrap();
        assert_eq!(expression.variables(), BTreeSet::from([name.as_str()]));
    }

    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1; 5],
    );
    let reduction = <MaximumIndependentSet<SimpleGraph, i32> as ReduceTo<
        MaximumClique<SimpleGraph, i32>,
    >>::reduce_to(&source);
    let target = reduction.target_problem();
    assert_eq!(target.graph().num_vertices(), 5);
    assert_eq!(target.graph().edges().len(), 6);

    let graph = ReductionGraph::new();
    let source_variant =
        ReductionGraph::variant_to_map(&MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target_variant =
        ReductionGraph::variant_to_map(&MaximumClique::<SimpleGraph, i32>::variant());
    let exact = graph
        .exact_size_front(
            MaximumIndependentSet::<SimpleGraph, i32>::NAME,
            &source_variant,
            MaximumClique::<SimpleGraph, i32>::NAME,
            &target_variant,
            ReductionMode::Witness,
            &ProblemSize::new(vec![("num_vertices", 5), ("num_edges", 4)]),
        )
        .unwrap();
    assert!(exact.front.iter().any(|result| {
        result.terminal_size.get("num_vertices") == Some(5)
            && result.terminal_size.get("num_edges") == Some(6)
    }));
    let bounded = graph
        .certified_bound_front(
            MaximumIndependentSet::<SimpleGraph, i32>::NAME,
            &source_variant,
            MaximumClique::<SimpleGraph, i32>::NAME,
            &target_variant,
            ReductionMode::Witness,
            &BoundVector::new([("num_vertices", 5u32), ("num_edges", 4u32)]),
        )
        .unwrap();
    assert!(bounded.front.iter().any(|result| {
        result.terminal_bound.get("num_vertices") == Some(&5u32.into())
            && result.terminal_bound.get("num_edges") == Some(&25u32.into())
    }));

    let mut exact_fields = 0usize;
    let mut bound_only_fields = 0usize;
    let mut unavailable_fields = 0usize;
    let mut unclassified = Vec::new();
    for entry in reduction_entries() {
        let contract = entry.size_contract().unwrap();
        let exact_names: BTreeSet<_> = contract
            .exact()
            .into_iter()
            .flat_map(|map| map.expressions().map(|(field, _)| field))
            .collect();
        let bound_names: BTreeSet<_> = contract
            .bounds()
            .into_iter()
            .flat_map(|map| map.expressions().map(|(field, _)| field))
            .collect();
        let unavailable_names: BTreeSet<_> = contract
            .unavailable()
            .iter()
            .map(|field| field.field)
            .collect();
        exact_fields += exact_names.len();
        bound_only_fields += bound_names.difference(&exact_names).count();
        unavailable_fields += unavailable_names.len();
        for field in declared_size_fields(entry.target_name) {
            if !exact_names.contains(field)
                && !bound_names.contains(field)
                && !unavailable_names.contains(field)
            {
                unclassified.push(format!(
                    "{} -> {}: {field}",
                    entry.source_name, entry.target_name
                ));
            }
        }
    }
    assert!(unclassified.is_empty(), "{}", unclassified.join("\n"));
    println!(
        "accounting: exact={exact_fields}, bound_only={bound_only_fields}, unavailable={unavailable_fields}, unclassified=0"
    );

    let examples = crate::example_db::build_rule_db().unwrap().rules;
    let mut exact_eligible = 0usize;
    let mut exact_checked = 0usize;
    let mut exact_mismatches = Vec::new();
    let mut bound_eligible = 0usize;
    let mut bound_checked = 0usize;
    let mut bound_violations = Vec::new();
    for example in examples {
        let entries: Vec<_> = reduction_entries()
            .into_iter()
            .filter(|entry| {
                entry.source_name == example.source.problem
                    && entry.target_name == example.target.problem
                    && variant_map(entry.source_variant()) == example.source.variant
                    && variant_map(entry.target_variant()) == example.target.variant
            })
            .collect();
        if entries.is_empty() {
            continue;
        }
        let source_problem = load_dyn(
            &example.source.problem,
            &example.source.variant,
            example.source.instance.clone(),
        )
        .unwrap();
        let target_problem = load_dyn(
            &example.target.problem,
            &example.target.variant,
            example.target.instance.clone(),
        )
        .unwrap();
        let target_size = ReductionGraph::compute_source_size(
            &example.target.problem,
            &example.target.variant,
            target_problem.as_any(),
        );
        for entry in entries {
            let contract = entry.size_contract().unwrap();
            let source_size = (entry.source_size_fn)(source_problem.as_any());
            if let Some(map) = contract.exact() {
                let measurable = map.expressions().all(|(field, expression)| {
                    target_size.get(field).is_some()
                        && expression
                            .variables()
                            .iter()
                            .all(|variable| source_size.get(variable).is_some())
                });
                if measurable {
                    exact_eligible += 1;
                    let predicted = map.evaluate(&source_size).unwrap();
                    exact_checked += 1;
                    for (field, value) in predicted.components {
                        if target_size.get(&field) != Some(value) {
                            exact_mismatches.push(format!(
                                "{} -> {} {field}: predicted={value}, measured={:?}",
                                entry.source_name,
                                entry.target_name,
                                target_size.get(&field)
                            ));
                        }
                    }
                }
            }
            if let Some(bounds) = contract.bounds() {
                let measurable = bounds.expressions().all(|(field, expression)| {
                    target_size.get(field).is_some()
                        && expression
                            .variables()
                            .iter()
                            .all(|variable| source_size.get(variable).is_some())
                });
                if measurable {
                    bound_eligible += 1;
                    let input = BoundVector::new(
                        source_size
                            .components
                            .iter()
                            .map(|(field, value)| (field.as_str(), *value)),
                    );
                    let predicted = bounds.evaluate(&input).unwrap();
                    bound_checked += 1;
                    for (field, value) in predicted.components() {
                        let measured = target_size.get(field).unwrap();
                        if value < &measured.into() {
                            bound_violations.push(format!(
                                "{} -> {} {field}: bound={value}, measured={measured}",
                                entry.source_name, entry.target_name
                            ));
                        }
                    }
                }
            }
        }
    }
    assert_eq!(exact_checked, exact_eligible);
    assert!(
        exact_mismatches.is_empty(),
        "{}",
        exact_mismatches.join("\n")
    );
    assert_eq!(bound_checked, bound_eligible);
    assert!(
        bound_violations.is_empty(),
        "{}",
        bound_violations.join("\n")
    );
    println!("exact oracle: eligible={exact_eligible}, checked={exact_checked}, mismatches=0");
    println!("bound oracle: eligible={bound_eligible}, checked={bound_checked}, violations=0");

    let missing = SizeMap::new("negative controls", [("out", Expr::variable("missing"))])
        .unwrap()
        .evaluate(&ProblemSize::default());
    assert!(matches!(
        missing,
        Err(SizeMapError::MissingInputField { .. })
    ));
    let negative = SizeMap::new("negative controls", [("out", Expr::integer(-1))])
        .unwrap()
        .evaluate(&ProblemSize::default());
    assert!(matches!(negative, Err(SizeMapError::NegativeResult { .. })));
    let non_integral = SizeMap::new(
        "negative controls",
        [("out", Expr::try_parse("n / 2").unwrap())],
    )
    .unwrap()
    .evaluate(&ProblemSize::new(vec![("n", 3)]));
    assert!(matches!(
        non_integral,
        Err(SizeMapError::NonIntegralResult { .. })
    ));
    let division_by_zero = SizeMap::new(
        "negative controls",
        [("out", Expr::try_parse("n / m").unwrap())],
    )
    .unwrap()
    .evaluate(&ProblemSize::new(vec![("n", 1), ("m", 0)]));
    assert!(matches!(
        division_by_zero,
        Err(SizeMapError::DivisionByZero { .. })
    ));
    let out_of_range = SizeMap::new(
        "negative controls",
        [(
            "out",
            Expr::integer(num_bigint::BigInt::from(usize::MAX) + 1),
        )],
    )
    .unwrap()
    .evaluate(&ProblemSize::default());
    assert!(matches!(
        out_of_range,
        Err(SizeMapError::OutputOutOfRange { .. })
    ));

    assert!(matches!(
        SizeBound::new(
            "bound controls",
            [("out", Expr::try_parse("n - m").unwrap())]
        ),
        Err(SizeBoundError::NegativeCoefficient { .. })
    ));
    assert!(matches!(
        SizeBound::new(
            "bound controls",
            [("out", Expr::try_parse("n / m").unwrap())]
        ),
        Err(SizeBoundError::NegativePower { .. })
    ));
    assert_eq!(
        SizeBound::new(
            "bound controls",
            [("out", Expr::try_parse("n - n").unwrap())]
        )
        .unwrap()
        .evaluate(&BoundVector::new([("n", 9u32)]))
        .unwrap()
        .get("out"),
        Some(&0u32.into())
    );

    let isolated =
        ReductionGraph::from_test_edges(&["S", "T"], &[("S", "T", contract(&[], &[("x", "x")]))]);
    let empty = BTreeMap::new();
    let exact = isolated
        .exact_size_front(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            &ProblemSize::new(vec![("x", 3)]),
        )
        .unwrap();
    assert!(exact.front.is_empty());
    assert_eq!(exact.unavailable.len(), 1);
    let bounded = isolated
        .certified_bound_front(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            &BoundVector::new([("x", 3u32)]),
        )
        .unwrap();
    assert_eq!(bounded.front.len(), 1);

    let exponential = Expr::try_parse("exp(n)").unwrap();
    assert!(matches!(
        crate::Growth::from_expr(&exponential),
        crate::Growth::Terms(terms) if !terms.is_empty()
    ));
    assert!(matches!(
        SizeMap::new("growth isolation", [("out", exponential)]),
        Err(SizeMapError::UnsupportedOperator { .. })
    ));

    let terminal_only = ReductionGraph::from_test_edges(
        &["S", "A", "B", "T"],
        &[
            ("S", "A", contract(&[("x", "1")], &[])),
            ("S", "B", contract(&[("x", "2")], &[])),
            ("A", "T", contract(&[("x", "x + 10")], &[])),
            ("B", "T", contract(&[("x", "x")], &[])),
        ],
    );
    let result = terminal_only
        .exact_size_front(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            &ProblemSize::new(vec![("x", 0)]),
        )
        .unwrap();
    assert_eq!(result.front[0].path.type_names(), ["S", "B", "T"]);

    let registry_source = include_str!("../rules/registry.rs");
    let graph_source = include_str!("../rules/graph.rs");
    let macro_codegen = include_str!("../../problemreductions-macros/src/expr_codegen.rs");
    for forbidden in [
        "ReductionOverhead",
        "OverheadCompositionError",
        "overhead_eval_fn",
        "compose_path_overhead",
    ] {
        assert!(!registry_source.contains(forbidden));
        assert!(!graph_source.contains(forbidden));
        assert!(!macro_codegen.contains(forbidden));
    }

    println!("PASS symbolic_size_contracts");
}
