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
use poly_compare::{compare_all_verdicts, compare_summary, Profile, Reason, Side, Verdict};
use serde::{Deserialize, Serialize};
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
    response_schema::<WasmPathResult>()
}

/// Generate the version-5 comparison response contract.
#[cfg(feature = "schema-export")]
pub fn comparison_response_schema() -> schemars::Schema {
    response_schema::<WasmComparisonResult>()
}

#[cfg(feature = "schema-export")]
fn response_schema<T: schemars::JsonSchema>() -> schemars::Schema {
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
    settings.into_generator().into_root_schema_for::<T>()
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields,
    extend("$id" = "https://codingthrust.github.io/problem-reductions/contracts/wasm-comparison")))]
struct WasmComparisonResult {
    #[cfg_attr(feature = "schema-export", schemars(extend("const" = 5)))]
    schema_version: u8,
    #[cfg_attr(feature = "schema-export", schemars(extend("const" = "generic")))]
    mode: &'static str,
    profiles: Vec<ComparisonProfile>,
    #[cfg_attr(
        feature = "schema-export",
        schemars(inner(range(min = 1, max = 9007199254740991u64)))
    )]
    front: Vec<usize>,
    counts: ComparisonCounts,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema-export", schemars(with = "Vec<ComparisonPair>"))]
    pairs: Option<Vec<ComparisonPair>>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
struct ComparisonCounts {
    #[cfg_attr(feature = "schema-export", schemars(range(max = 9007199254740991u64)))]
    dominated: usize,
    #[cfg_attr(feature = "schema-export", schemars(range(max = 9007199254740991u64)))]
    equivalent: usize,
    #[cfg_attr(feature = "schema-export", schemars(range(max = 9007199254740991u64)))]
    incomparable: usize,
    #[cfg_attr(feature = "schema-export", schemars(range(max = 9007199254740991u64)))]
    unknown: usize,
}

impl From<[usize; 4]> for ComparisonCounts {
    fn from([dominated, equivalent, incomparable, unknown]: [usize; 4]) -> Self {
        Self {
            dominated,
            equivalent,
            incomparable,
            unknown,
        }
    }
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
struct ComparisonPair {
    #[cfg_attr(
        feature = "schema-export",
        schemars(inner(range(min = 1, max = 9007199254740991u64)))
    )]
    paths: [usize; 2],
    #[cfg_attr(feature = "schema-export", schemars(extend("enum" = Verdict::ALL)))]
    verdict: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema-export", schemars(with = "Dominance"))]
    dominates: Option<Dominance>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
struct Dominance {
    #[cfg_attr(
        feature = "schema-export",
        schemars(range(min = 1, max = 9007199254740991u64))
    )]
    winner: usize,
    #[cfg_attr(
        feature = "schema-export",
        schemars(range(min = 1, max = 9007199254740991u64))
    )]
    loser: usize,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
struct ComparisonProfile {
    #[cfg_attr(feature = "schema-export", schemars(inner(length(min = 1))))]
    source_coordinates: Vec<String>,
    #[cfg_attr(feature = "schema-export", schemars(extend("propertyNames" = {"type": "string", "minLength": 1})))]
    fields: BTreeMap<String, ComparisonField>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
struct ComparisonField {
    #[cfg_attr(feature = "schema-export", schemars(length(min = 1)))]
    certificate: Option<String>,
    accuracy: ComparisonAccuracy,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ComparisonAccuracy {
    Exact,
    UpperBound,
    Unavailable { reason: ComparisonReason },
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema-export", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-export", schemars(deny_unknown_fields))]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ComparisonReason {
    NotPolynomial { detail: String },
    Unavailable { detail: String },
    MissingField,
    Validity,
    ResourceLimit { detail: String },
}

impl From<&Reason> for ComparisonReason {
    fn from(reason: &Reason) -> Self {
        match reason {
            Reason::NotPolynomial(detail) => Self::NotPolynomial {
                detail: detail.clone(),
            },
            Reason::Unavailable(error) => Self::Unavailable {
                detail: error.to_string(),
            },
            Reason::MissingField => Self::MissingField,
            Reason::Validity => Self::Validity,
            Reason::ResourceLimit(detail) => Self::ResourceLimit {
                detail: (*detail).to_string(),
            },
        }
    }
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
    let (front, counts, pairs) = if summary_only {
        let result = compare_summary(&profiles);
        (result.front, result.counts, None)
    } else {
        let result = compare_all_verdicts(&profiles);
        let pairs = result
            .pairs
            .iter()
            .map(|(i, j, comparison)| {
                let dominates = if let Verdict::Dominated { worse } = comparison.verdict {
                    let (winner, loser) = if worse == Side::Left {
                        (j + 1, i + 1)
                    } else {
                        (i + 1, j + 1)
                    };
                    Some(Dominance { winner, loser })
                } else {
                    None
                };
                ComparisonPair {
                    paths: [i + 1, j + 1],
                    verdict: comparison.verdict.name(),
                    dominates,
                }
            })
            .collect();
        let counts = result.counts();
        (result.front, counts, Some(pairs))
    };
    let profiles = profiles
        .iter()
        .map(|profile| {
            let fields = profile
                .fields
                .iter()
                .map(|(name, field)| {
                    let (certificate, accuracy) = match &field.certificate {
                        Ok((poly, relation)) => (
                            Some(poly.to_expr().to_string()),
                            match relation {
                                ParameterRelation::Exact => ComparisonAccuracy::Exact,
                                ParameterRelation::UpperBound => ComparisonAccuracy::UpperBound,
                            },
                        ),
                        Err(reason) => (
                            None,
                            ComparisonAccuracy::Unavailable {
                                reason: reason.into(),
                            },
                        ),
                    };
                    (
                        name.clone(),
                        ComparisonField {
                            certificate,
                            accuracy,
                        },
                    )
                })
                .collect();
            ComparisonProfile {
                source_coordinates: profile.domain.iter().map(ToString::to_string).collect(),
                fields,
            }
        })
        .collect();
    Ok(serde_json::to_string(&WasmComparisonResult {
        schema_version: 5,
        mode: "generic",
        profiles,
        front: front.into_iter().map(|index| index + 1).collect(),
        counts: counts.into(),
        pairs,
    })?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_reason_wire_format_matches_native_proof_format() {
        for reason in [
            Reason::MissingField,
            Reason::Validity,
            Reason::ResourceLimit("Exact arithmetic budget exhausted"),
        ] {
            assert_eq!(
                serde_json::to_value(ComparisonReason::from(&reason)).unwrap(),
                serde_json::to_value(reason).unwrap()
            );
        }
    }
}
