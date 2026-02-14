// Shared JSON parsing helpers for Julia parity tests.
// Include this file with `include!("../jl_helpers.rs")` from rule tests
// or `include!("../../jl_helpers.rs")` from model tests.

use std::collections::HashSet;

#[allow(dead_code)]
fn jl_parse_edges(instance: &serde_json::Value) -> Vec<(usize, usize)> {
    instance["edges"]
        .as_array()
        .expect("edges should be an array")
        .iter()
        .map(|e| {
            let arr = e.as_array().expect("edge should be a [u, v] array");
            (
                arr[0].as_u64().expect("edge vertex should be u64") as usize,
                arr[1].as_u64().expect("edge vertex should be u64") as usize,
            )
        })
        .collect()
}

#[allow(dead_code)]
fn jl_parse_weighted_edges(instance: &serde_json::Value) -> Vec<(usize, usize, i32)> {
    let edges = jl_parse_edges(instance);
    let weights: Vec<i32> = instance["weights"]
        .as_array()
        .expect("weights should be an array")
        .iter()
        .map(|w| w.as_i64().expect("weight should be i64") as i32)
        .collect();
    edges
        .into_iter()
        .zip(weights)
        .map(|((u, v), w)| (u, v, w))
        .collect()
}

#[allow(dead_code)]
fn jl_parse_config(val: &serde_json::Value) -> Vec<usize> {
    val.as_array()
        .expect("config should be an array")
        .iter()
        .map(|v| v.as_u64().expect("config element should be u64") as usize)
        .collect()
}

#[allow(dead_code)]
fn jl_parse_configs_set(val: &serde_json::Value) -> HashSet<Vec<usize>> {
    val.as_array()
        .expect("configs set should be an array")
        .iter()
        .map(jl_parse_config)
        .collect()
}

#[allow(dead_code)]
fn jl_parse_i32_vec(val: &serde_json::Value) -> Vec<i32> {
    val.as_array()
        .expect("should be an array of integers")
        .iter()
        .map(|v| v.as_i64().expect("element should be i64") as i32)
        .collect()
}

#[allow(dead_code)]
fn jl_parse_sets(val: &serde_json::Value) -> Vec<Vec<usize>> {
    val.as_array()
        .expect("sets should be an array")
        .iter()
        .map(|s| {
            s.as_array()
                .expect("set should be an array")
                .iter()
                .map(|v| v.as_u64().expect("set element should be u64") as usize)
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn jl_parse_sat_clauses(
    instance: &serde_json::Value,
) -> (usize, Vec<crate::models::satisfiability::CNFClause>) {
    let num_vars = instance["num_variables"]
        .as_u64()
        .expect("num_variables should be a u64") as usize;
    let clauses = instance["clauses"]
        .as_array()
        .expect("clauses should be an array")
        .iter()
        .map(|clause| {
            let literals: Vec<i32> = clause["literals"]
                .as_array()
                .expect("clause.literals should be an array")
                .iter()
                .map(|lit| {
                    let var = lit["variable"]
                        .as_u64()
                        .expect("literal.variable should be a u64") as i32
                        + 1;
                    let negated = lit["negated"]
                        .as_bool()
                        .expect("literal.negated should be a bool");
                    if negated { -var } else { var }
                })
                .collect();
            crate::models::satisfiability::CNFClause::new(literals)
        })
        .collect();
    (num_vars, clauses)
}

/// Flip a binary config: 0<->1 for SpinGlass spin convention mapping.
#[allow(dead_code)]
fn jl_flip_config(config: &[usize]) -> Vec<usize> {
    config.iter().map(|&x| 1 - x).collect()
}

#[allow(dead_code)]
fn jl_flip_configs_set(configs: &HashSet<Vec<usize>>) -> HashSet<Vec<usize>> {
    configs.iter().map(|c| jl_flip_config(c)).collect()
}

#[allow(dead_code)]
fn jl_find_instance_by_label<'a>(
    data: &'a serde_json::Value,
    label: &str,
) -> &'a serde_json::Value {
    data["instances"]
        .as_array()
        .expect("instances should be an array")
        .iter()
        .find(|inst| inst["label"].as_str().expect("instance label should be a string") == label)
        .unwrap_or_else(|| panic!("Instance '{label}' not found"))
}
