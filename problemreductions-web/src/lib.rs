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

use expr::{Expr, Symbol};
use parameters::{ParameterRelation, ParameterTransform};
use poly_compare::{compare_all_verdicts, compare_summary, Profile, Side, Verdict};
use serde::{Deserialize, Serialize};
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
pub fn find_paths(input: &str) -> Result<String, JsError> {
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
    let endpoint = |i: usize| Endpoint {
        name: nodes[i].name.clone(),
        variant: nodes[i].variant.clone(),
    };
    let routes = paths
        .iter()
        .map(|path| Route {
            steps: path.len() - 1,
            path: path
                .windows(2)
                .map(|pair| Step {
                    from: endpoint(pair[0]),
                    to: endpoint(pair[1]),
                })
                .collect(),
        })
        .collect();
    Ok(serde_json::to_string(&WasmPathResult {
        schema_version: 1,
        paths: routes,
        truncated,
    })?)
}

#[derive(Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
struct Endpoint {
    #[cfg_attr(feature = "schema-export", schemars(length(min = 1)))]
    name: String,
    variant: BTreeMap<String, String>,
}
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
struct Step {
    from: Endpoint,
    to: Endpoint,
}
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
struct Route {
    #[cfg_attr(
        feature = "schema-export",
        schemars(range(min = 1, max = 9007199254740991u64))
    )]
    steps: usize,
    #[cfg_attr(feature = "schema-export", schemars(length(min = 1)))]
    path: Vec<Step>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields,
    extend("$id" = "https://codingthrust.github.io/problem-reductions/contracts/wasm-paths")))]
struct WasmPathResult {
    #[cfg_attr(feature = "schema-export", schemars(extend("const" = 1)))]
    schema_version: u8,
    #[cfg_attr(feature = "schema-export", schemars(length(max = 999)))]
    paths: Vec<Route>,
    truncated: bool,
}

/// Generate the browser response contract without adding schema tooling to WASM builds.
#[cfg(feature = "schema-export")]
pub fn path_response_schema() -> schemars::Schema {
    let mut settings = schemars::generate::SchemaSettings::draft2020_12().for_serialize();
    // JSON numeric bounds carry the domain; Rust's integer formats are nonstandard.
    settings
        .transforms
        .push(Box::new(schemars::transform::RecursiveTransform(
            |schema: &mut schemars::Schema| {
                if matches!(
                    schema.get("format").and_then(|value| value.as_str()),
                    Some("uint" | "uint8")
                ) {
                    schema.remove("format");
                }
            },
        )));
    settings
        .into_generator()
        .into_root_schema_for::<WasmPathResult>()
}
#[derive(Deserialize)]
struct ComparisonQuery {
    graph: Graph,
    paths: Vec<Route>,
}

impl Graph {
    fn route_indices(&self, route: &Route) -> Result<Vec<usize>, JsError> {
        if route.path.is_empty() || route.steps != route.path.len() {
            return Err(JsError::new("Choose a nonempty registered reduction path."));
        }
        let index = |endpoint: &Endpoint| {
            self.nodes
                .iter()
                .position(|node| node.name == endpoint.name && node.variant == endpoint.variant)
                .ok_or_else(|| JsError::new("Path references an unregistered variant."))
        };
        let mut indices = vec![index(&route.path[0].from)?];
        for step in &route.path {
            let source = index(&step.from)?;
            let target = index(&step.to)?;
            if indices.last() != Some(&source)
                || !self.edges.iter().any(|edge| {
                    edge.source == source && edge.target == target && edge.witness && !edge.turing
                })
            {
                return Err(JsError::new("Path references an unregistered reduction."));
            }
            indices.push(target);
        }
        Ok(indices)
    }
}

/// Compare exactly the routes returned by `find_paths`, retaining their one-based indices.
#[wasm_bindgen]
pub fn compare_paths(input: &str) -> Result<String, JsError> {
    compare_paths_impl(input, false)
}

/// Browser result: certificates, all pair counts and the front, without pair records.
#[wasm_bindgen]
pub fn summarize_paths(input: &str) -> Result<String, JsError> {
    compare_paths_impl(input, true)
}

fn compare_paths_impl(input: &str, summary_only: bool) -> Result<String, JsError> {
    let query: ComparisonQuery = serde_json::from_str(input)?;
    let Graph { nodes, edges } = &query.graph;
    let mut profiles = Vec::new();
    for route in query.paths {
        let path = query.graph.route_indices(&route)?;
        let mut composed: Option<ParameterTransform> = None;
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
            composed = Some(if let Some(prefix) = composed {
                prefix.compose(
                    &transform,
                    format!("{} -> {}", nodes[path[0]].name, nodes[pair[1]].name),
                )?
            } else {
                transform
            });
        }
        let mut profile =
            Profile::from_transform(&composed.expect("validated nonempty reduction path"));
        for field in profile.fields.values_mut() {
            field.original_signed = None;
        }
        profile.domain.extend(
            nodes[path[0]]
                .parameter_names
                .iter()
                .filter_map(|name| Symbol::new(name.as_str()).ok()),
        );
        profiles.push(profile);
    }
    let mut response = if summary_only {
        let result = compare_summary(&profiles);
        json!({"front":result.front.iter().map(|i|i+1).collect::<Vec<_>>(),
            "counts":Verdict::ALL.iter().zip(result.counts).collect::<BTreeMap<_,_>>()})
    } else {
        let result = compare_all_verdicts(&profiles);
        let pairs: Vec<_> = result
            .pairs
            .iter()
            .map(|(i, j, comparison)| {
                let mut pair = json!({"paths":[i+1,j+1], "verdict":comparison.verdict});
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
        let mut response = json!({"front":result.front.iter().map(|i|i+1).collect::<Vec<_>>(),
            "counts":Verdict::ALL.iter().zip(result.counts()).collect::<BTreeMap<_,_>>()});
        response["pairs"] = serde_json::Value::Array(pairs);
        response
    };
    let profiles: Vec<_> = profiles.iter().map(|profile| {
        let fields: BTreeMap<_, _> = profile.fields.iter().map(|(name, field)| {
            let (certificate, accuracy) = match &field.certificate {
                Ok((poly, relation)) => (Some(poly.to_expr().to_string()), json!({"kind":relation})),
                Err(reason) => (None, json!({"kind":"unavailable", "reason":reason})),
            };
            (name, json!({"certificate":certificate, "accuracy":accuracy}))
        }).collect();
        json!({"source_coordinates":profile.domain.iter().map(ToString::to_string).collect::<Vec<_>>(), "fields":fields})
    }).collect();
    response["schema_version"] = json!(5);
    response["mode"] = json!("generic");
    response["profiles"] = serde_json::Value::Array(profiles);
    Ok(serde_json::to_string(&response)?)
}
