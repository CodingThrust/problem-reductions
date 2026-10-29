use problemreductions_web::{edge_growth, find_paths, summarize_paths};
use serde_json::{json, Value};

fn graph() -> Value {
    json!({
        "nodes": [
            {"name":"Source", "variant":{}, "parameter_names":["m", "n"]},
            {"name":"Target", "variant":{"weight":"One"}, "parameter_names":["n"]},
            {"name":"Target", "variant":{"weight":"i64"}, "parameter_names":["n"]}
        ],
        "edges": [
            {"source":0, "target":2, "witness":true, "turing":false,
             "parameters":[{"field":"n", "contract":"exact", "formula":"n^2"}]},
            {"source":0, "target":1, "witness":true, "turing":false,
             "parameters":[{"field":"n", "contract":"exact", "formula":"n"}]},
            {"source":1, "target":2, "witness":true, "turing":false,
             "parameters":[{"field":"n", "contract":"exact", "formula":"n"}]}
        ]
    })
}

#[test]
fn paths_arrive_without_composing_or_comparing_parameter_formulas() {
    let mut graph = graph();
    graph["edges"][0]["parameter_contract_error"] = json!("Unavailable certificate");
    let result: Value = serde_json::from_str(
        &find_paths(
            &json!({
                "graph":graph, "source":0, "target":2, "limit":1
            })
            .to_string(),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        result,
        json!({"truncated":true, "paths":[{
            "steps":1, "path":[{"from":{"name":"Source", "variant":{}},
                "to":{"name":"Target", "variant":{"weight":"i64"}}}]
        }]})
    );
}

#[test]
fn comparison_ranks_the_supplied_exact_variant_routes() {
    let graph = graph();
    let result: Value = serde_json::from_str(
        &find_paths(
            &json!({
                "graph":graph, "source":0, "target":2, "limit":20
            })
            .to_string(),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(result["truncated"], false);
    assert_eq!(result["paths"].as_array().unwrap().len(), 2);
    assert_eq!(
        result["paths"][1]["path"][0]["to"]["variant"]["weight"],
        "One"
    );
    let comparison: Value = serde_json::from_str(
        &summarize_paths(
            &json!({
                "graph":graph, "paths":[result["paths"][1],result["paths"][0]]
            })
            .to_string(),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(comparison["front"], json!([1]));
    assert_eq!(
        comparison["profiles"][0]["source_coordinates"],
        json!(["m", "n"])
    );
    assert_eq!(
        comparison["profiles"][0]["fields"]["n"],
        json!({
            "certificate":"n", "accuracy":{"kind":"exact"}
        })
    );
    assert_eq!(comparison["profiles"].as_array().unwrap().len(), 2);
    assert_eq!(comparison["counts"]["Dominated"], 1);
}

#[test]
fn browser_summary_keeps_profiles_counts_and_front_without_pair_payloads() {
    let graph = graph();
    let routes: Value = serde_json::from_str(
        &find_paths(
            &json!({
                "graph": graph, "source": 0, "target": 2, "limit": 20
            })
            .to_string(),
        )
        .unwrap(),
    )
    .unwrap();
    let input = json!({"graph": graph, "paths": routes["paths"]}).to_string();
    let summary: Value = serde_json::from_str(&summarize_paths(&input).unwrap()).unwrap();
    assert!(summary.get("schema_version").is_none());
    assert!(summary.get("mode").is_none());
    assert_eq!(
        summary["counts"],
        json!({"Dominated":1,"Equivalent":0,"Incomparable":0,"Unknown":0})
    );
    assert_eq!(summary["front"], json!([2]));
    assert!(summary.get("pairs").is_none());
}

#[test]
fn comparison_retains_exact_wide_rational_coefficients() {
    use num_bigint::{BigInt, BigUint};
    use num_rational::BigRational;
    use problemreductions_web::{
        expr::{Expr, Symbol},
        polynomial::Poly,
    };
    use std::collections::BTreeMap;

    let mut graph = graph();
    graph["edges"] = json!([graph["edges"][0]]);
    graph["edges"][0]["parameters"][0]["formula"] = json!("9007199254740993 / 7 * n");
    graph["edges"][0]["parameters"][0]["contract"] = json!("upper_bound");
    let routes: Value = serde_json::from_str(
        &find_paths(
            &json!({
                "graph": graph, "source": 0, "target": 2, "limit": 1
            })
            .to_string(),
        )
        .unwrap(),
    )
    .unwrap();
    let input = json!({"graph":graph, "paths":routes["paths"]}).to_string();
    let result: Value = serde_json::from_str(&summarize_paths(&input).unwrap()).unwrap();
    let field = &result["profiles"][0]["fields"]["n"];
    let certificate = field["certificate"].as_str().unwrap();
    assert_eq!(field["accuracy"]["kind"], "upper_bound");
    let poly = Poly::from_expr(&Expr::try_parse(certificate).unwrap()).unwrap();
    assert_eq!(
        poly.terms(),
        &BTreeMap::from([(
            BTreeMap::from([(Symbol::new("n").unwrap(), BigUint::from(1u8))]),
            BigRational::new(BigInt::from(9007199254740993u64), BigInt::from(7u8))
        )])
    );
    assert!(result.get("pairs").is_none());
}

#[test]
fn comparison_retains_explicit_null_certificates_and_typed_unavailable_reasons() {
    for (parameter, expected_reason) in [
        (
            json!({"field":"n", "contract":"unavailable", "reason":"Target count is not certified"}),
            "unavailable",
        ),
        (
            json!({"field":"n", "contract":"exact", "formula":"n^-1"}),
            "not_polynomial",
        ),
    ] {
        let mut graph = graph();
        graph["edges"] = json!([graph["edges"][0]]);
        graph["edges"][0]["parameters"] = json!([parameter]);
        let routes: Value = serde_json::from_str(
            &find_paths(
                &json!({
                    "graph":graph, "source":0, "target":2, "limit":1
                })
                .to_string(),
            )
            .unwrap(),
        )
        .unwrap();
        let result: Value = serde_json::from_str(
            &summarize_paths(
                &json!({
                    "graph":graph, "paths":routes["paths"]
                })
                .to_string(),
            )
            .unwrap(),
        )
        .unwrap();
        let field = &result["profiles"][0]["fields"]["n"];
        assert_eq!(field.get("certificate"), Some(&Value::Null));
        assert_eq!(field["accuracy"]["kind"], "unavailable");
        assert_eq!(field["accuracy"]["reason"]["kind"], expected_reason);
        assert!(field["accuracy"]["reason"]["detail"]
            .as_str()
            .is_some_and(|detail| !detail.is_empty()));
    }
}

#[test]
fn nondominated_comparisons_keep_both_routes() {
    for (formula, verdict) in [
        ("n", "Equivalent"),
        ("m", "Incomparable"),
        ("n^-1", "Unknown"),
    ] {
        let mut graph = graph();
        graph["edges"][0]["parameters"][0]["formula"] = json!(formula);
        let routes: Value = serde_json::from_str(
            &find_paths(&json!({"graph":graph, "source":0, "target":2, "limit":20}).to_string())
                .unwrap(),
        )
        .unwrap();
        let input = json!({"graph":graph, "paths":routes["paths"]}).to_string();
        let summary: Value = serde_json::from_str(&summarize_paths(&input).unwrap()).unwrap();
        assert_eq!(summary["front"], json!([1, 2]));
        assert_eq!(summary["counts"][verdict], 1);
        assert!(summary.get("pairs").is_none());
    }
}

#[test]
fn unreachable_and_identical_endpoints_have_no_routes() {
    for (source, target) in [(2, 0), (0, 0)] {
        let result: Value = serde_json::from_str(
            &find_paths(
                &json!({
                    "graph":graph(), "source":source, "target":target, "limit":20
                })
                .to_string(),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result, json!({"paths":[],"truncated":false}));
    }
}

#[test]
fn edge_growth_certifies_each_reduction_alone_and_keeps_its_accuracy() {
    let mut graph = graph();
    graph["edges"][1]["parameters"] = json!([
        {"field":"n", "contract":"upper_bound", "formula":"n + m^2"},
        {"field":"k", "contract":"unavailable", "reason":"data dependent"}
    ]);
    let growth: Value = serde_json::from_str(&edge_growth(&graph.to_string()).unwrap()).unwrap();
    assert_eq!(growth.as_array().unwrap().len(), 3);
    assert_eq!(growth[0]["n"]["certificate"], "n^2");
    assert_eq!(growth[0]["n"]["accuracy"]["kind"], "exact");
    assert_eq!(growth[1]["n"]["certificate"], "n + m^2");
    assert_eq!(growth[1]["n"]["accuracy"]["kind"], "upper_bound");
    assert_eq!(growth[1]["k"]["certificate"], Value::Null);
    assert_eq!(growth[1]["k"]["accuracy"]["kind"], "unavailable");
}
