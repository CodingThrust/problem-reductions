//! Browser bindings compile the native symbolic and path-search sources directly.
#[path = "../../src/expr.rs"]
pub mod expr;
#[path = "../../src/parameters.rs"]
pub mod parameters;
#[path = "../../src/path_search.rs"]
mod path_search;
#[path = "../../src/poly_compare/mod.rs"]
pub mod poly_compare;
#[path = "../../src/polynomial.rs"]
pub mod polynomial;
#[path = "../../src/problem_parameters.rs"]
pub mod types;

use expr::Expr;
use parameters::{ParameterRelation, ParameterTransform};
use poly_compare::{compare_all, Profile, Side, Verdict};
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct Node {
    name: String,
    variant: BTreeMap<String, String>,
    parameter_names: Vec<String>,
}
#[derive(Deserialize)]
struct Parameter {
    field: String,
    contract: Contract,
    formula: Option<String>,
    reason: Option<String>,
}
#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Contract {
    Exact,
    UpperBound,
    Unavailable,
}
#[derive(Deserialize)]
struct Edge {
    source: usize,
    target: usize,
    witness: bool,
    turing: bool,
    parameters: Vec<Parameter>,
    parameter_contract_error: Option<String>,
}
#[derive(Deserialize)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
#[derive(Deserialize)]
struct Query {
    graph: Graph,
    source: usize,
    target: usize,
    limit: usize,
}

#[wasm_bindgen]
pub fn select_paths(input: &str) -> Result<String, JsError> {
    let query: Query = serde_json::from_str(input)?;
    let Graph { nodes, edges } = query.graph;
    if !(1..=999).contains(&query.limit)
        || query.source >= nodes.len()
        || query.target >= nodes.len()
    {
        return Err(JsError::new(
            "Choose registered endpoints and a path limit from 1 to 999.",
        ));
    }
    let mut order: Vec<_> = (0..nodes.len()).collect();
    order.sort_by_key(|&i| (&nodes[i].name, &nodes[i].variant));
    let mut ranks = vec![0; nodes.len()];
    for (rank, &i) in order.iter().enumerate() {
        ranks[i] = rank;
    }
    let mut adjacency = vec![Vec::new(); nodes.len()];
    for edge in &edges {
        if edge.source >= nodes.len() || edge.target >= nodes.len() {
            return Err(JsError::new(
                "Reduction graph contains an invalid endpoint.",
            ));
        }
        if edge.witness && !edge.turing {
            adjacency[edge.source].push(edge.target);
        }
    }
    for neighbors in &mut adjacency {
        neighbors.sort_by_key(|&i| ranks[i]);
    }
    let mut paths = path_search::find_paths(
        &adjacency,
        &ranks,
        query.source,
        query.target,
        query.limit + 1,
        nodes.len(),
    );
    let truncated = paths.len() > query.limit;
    paths.truncate(query.limit);
    let mut profiles = Vec::new();
    for path in &paths {
        let mut transforms = Vec::new();
        for pair in path.windows(2) {
            let edge = edges
                .iter()
                .find(|edge| edge.source == pair[0] && edge.target == pair[1])
                .ok_or_else(|| JsError::new("Path references an unregistered reduction."))?;
            if let Some(error) = &edge.parameter_contract_error {
                return Err(JsError::new(error));
            }
            let mut fields = Vec::new();
            for parameter in &edge.parameters {
                if parameter.contract == Contract::Unavailable {
                    continue;
                }
                let formula = parameter
                    .formula
                    .as_ref()
                    .ok_or_else(|| JsError::new("Missing parameter formula."))?;
                let relation = if parameter.contract == Contract::Exact {
                    ParameterRelation::Exact
                } else {
                    ParameterRelation::UpperBound
                };
                fields.push((parameter.field.clone(), relation, Expr::try_parse(formula)?));
            }
            let mut transform = ParameterTransform::from_fields(
                format!("{} -> {}", nodes[pair[0]].name, nodes[pair[1]].name),
                fields,
            )?;
            for parameter in &edge.parameters {
                if parameter.contract == Contract::Unavailable {
                    transform.declare_unavailable(
                        &parameter.field,
                        parameter.reason.as_deref().ok_or_else(|| {
                            JsError::new("Missing reason for unavailable parameter.")
                        })?,
                    );
                }
            }
            transforms.push(transform);
        }
        let mut composed = transforms[0].clone();
        for (offset, transform) in transforms.iter().enumerate().skip(1) {
            composed = composed.compose(
                transform,
                format!(
                    "{} -> {}",
                    nodes[query.source].name,
                    nodes[path[offset + 1]].name
                ),
            )?;
        }
        profiles.push(Profile::from_path(
            &nodes[query.source].parameter_names,
            &transforms,
            &composed,
        ));
    }
    let result = compare_all(&profiles);
    let pairs: Vec<_> = result
        .pairs
        .iter()
        .map(|(i, j, comparison)| {
            let mut pair =
                json!({"paths":[i+1,j+1], "verdict":comparison.verdict,"fields":comparison.fields});
            if let Verdict::Dominated { worse } = comparison.verdict {
                let (winner, loser) = if worse == Side::Left {
                    (j + 1, i + 1)
                } else {
                    (i + 1, j + 1)
                };
                pair["dominates"] = json!({"winner":winner,"loser":loser});
            }
            pair
        })
        .collect();
    let endpoint = |i: usize| json!({"name":nodes[i].name,"variant":nodes[i].variant});
    let routes: Vec<_> = paths.iter().map(|path| json!({"steps":path.len()-1,
        "path":path.windows(2).map(|p| json!({"from":endpoint(p[0]),"to":endpoint(p[1])})).collect::<Vec<_>>()
    })).collect();
    Ok(serde_json::to_string(
        &json!({"paths":routes,"truncated":truncated,"polynomial_comparison":{
            "schema_version":5,"mode":"generic","profiles":profiles,"pairs":pairs,
            "front":result.front.iter().map(|i|i+1).collect::<Vec<_>>(),
            "counts":Verdict::ALL.iter().zip(result.counts()).collect::<BTreeMap<_,_>>()
        }}),
    )?)
}
