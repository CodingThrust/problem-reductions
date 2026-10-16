use problemreductions_web::{compare_paths, find_paths, summarize_paths};
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
        &compare_paths(
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
    assert_eq!(
        comparison["pairs"],
        json!([{
            "paths":[1,2], "verdict":"Dominated", "dominates":{"winner":1,"loser":2}
        }])
    );
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
    let full: Value = serde_json::from_str(&compare_paths(&input).unwrap()).unwrap();
    let summary: Value = serde_json::from_str(&summarize_paths(&input).unwrap()).unwrap();
    assert_eq!(summary["profiles"], full["profiles"]);
    assert_eq!(
        summary["counts"],
        json!({"Dominated":1,"Equivalent":0,"Incomparable":0,"Unknown":0})
    );
    assert_eq!(summary["front"], json!([2]));
    assert!(summary.get("pairs").is_none());
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
