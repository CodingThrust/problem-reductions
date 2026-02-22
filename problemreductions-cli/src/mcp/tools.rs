use problemreductions::registry::collect_schemas;
use problemreductions::rules::{Minimize, MinimizeSteps, ReductionGraph, TraversalDirection};
use problemreductions::types::ProblemSize;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::tool;

use crate::problem_name::{
    aliases_for, parse_problem_spec, resolve_variant, unknown_problem_error,
};

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ShowProblemParams {
    #[schemars(description = "Problem name or alias (e.g., MIS, QUBO, MaximumIndependentSet)")]
    pub problem: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct NeighborsParams {
    #[schemars(description = "Problem name or alias")]
    pub problem: String,
    #[schemars(description = "Number of hops to explore (default: 1)")]
    pub hops: Option<usize>,
    #[schemars(description = "Direction: out (default), in, or both")]
    pub direction: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FindPathParams {
    #[schemars(description = "Source problem name or alias")]
    pub source: String,
    #[schemars(description = "Target problem name or alias")]
    pub target: String,
    #[schemars(description = "Cost function: minimize-steps (default), or minimize:<field>")]
    pub cost: Option<String>,
    #[schemars(description = "Return all paths instead of just the cheapest")]
    pub all: Option<bool>,
}

// ---------------------------------------------------------------------------
// McpServer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct McpServer {
    tool_router: ToolRouter<Self>,
}

// Tool implementations on the server struct.  Each `*_inner` method returns
// `anyhow::Result<String>` (a JSON string) so unit tests can call them directly
// without going through the MCP transport.

impl McpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    // -- inner helpers (return JSON strings) ---------------------------------

    pub fn list_problems_inner(&self) -> anyhow::Result<String> {
        let graph = ReductionGraph::new();
        let mut types = graph.problem_types();
        types.sort();

        let problems: Vec<serde_json::Value> = types
            .iter()
            .map(|name| {
                let aliases = aliases_for(name);
                let num_variants = graph.variants_for(name).len();
                let num_reduces_to = graph.outgoing_reductions(name).len();
                serde_json::json!({
                    "name": name,
                    "aliases": aliases,
                    "num_variants": num_variants,
                    "num_reduces_to": num_reduces_to,
                })
            })
            .collect();

        let json = serde_json::json!({
            "num_types": graph.num_types(),
            "num_reductions": graph.num_reductions(),
            "num_variant_nodes": graph.num_variant_nodes(),
            "problems": problems,
        });
        Ok(serde_json::to_string_pretty(&json)?)
    }

    pub fn show_problem_inner(&self, problem: &str) -> anyhow::Result<String> {
        let spec = parse_problem_spec(problem)?;
        let graph = ReductionGraph::new();

        let variants = graph.variants_for(&spec.name);
        if variants.is_empty() {
            anyhow::bail!("{}", unknown_problem_error(&spec.name));
        }

        let schemas = collect_schemas();
        let schema = schemas.iter().find(|s| s.name == spec.name);

        let outgoing = graph.outgoing_reductions(&spec.name);
        let incoming = graph.incoming_reductions(&spec.name);
        let size_fields = graph.size_field_names(&spec.name);

        let mut json = serde_json::json!({
            "name": spec.name,
            "variants": variants,
            "size_fields": size_fields,
            "reduces_to": outgoing.iter().map(|e| {
                serde_json::json!({
                    "source": {"name": e.source_name, "variant": e.source_variant},
                    "target": {"name": e.target_name, "variant": e.target_variant},
                })
            }).collect::<Vec<_>>(),
            "reduces_from": incoming.iter().map(|e| {
                serde_json::json!({
                    "source": {"name": e.source_name, "variant": e.source_variant},
                    "target": {"name": e.target_name, "variant": e.target_variant},
                })
            }).collect::<Vec<_>>(),
        });
        if let Some(s) = schema {
            if let (Some(obj), Ok(schema_val)) = (json.as_object_mut(), serde_json::to_value(s)) {
                obj.insert("schema".to_string(), schema_val);
            }
        }

        Ok(serde_json::to_string_pretty(&json)?)
    }

    pub fn neighbors_inner(
        &self,
        problem: &str,
        hops: usize,
        direction_str: &str,
    ) -> anyhow::Result<String> {
        let spec = parse_problem_spec(problem)?;
        let graph = ReductionGraph::new();

        let variants = graph.variants_for(&spec.name);
        if variants.is_empty() {
            anyhow::bail!("{}", unknown_problem_error(&spec.name));
        }

        let direction = parse_direction(direction_str)?;

        let variant = if spec.variant_values.is_empty() {
            variants[0].clone()
        } else {
            resolve_variant(&spec, &variants)?
        };

        let neighbors = graph.k_neighbors(&spec.name, &variant, hops, direction);

        let json = serde_json::json!({
            "source": spec.name,
            "hops": hops,
            "direction": direction_str,
            "neighbors": neighbors.iter().map(|n| {
                serde_json::json!({
                    "name": n.name,
                    "variant": n.variant,
                    "hops": n.hops,
                })
            }).collect::<Vec<_>>(),
        });
        Ok(serde_json::to_string_pretty(&json)?)
    }

    pub fn find_path_inner(
        &self,
        source: &str,
        target: &str,
        cost: &str,
        all: bool,
    ) -> anyhow::Result<String> {
        let src_spec = parse_problem_spec(source)?;
        let dst_spec = parse_problem_spec(target)?;
        let graph = ReductionGraph::new();

        let src_variants = graph.variants_for(&src_spec.name);
        let dst_variants = graph.variants_for(&dst_spec.name);

        if src_variants.is_empty() {
            anyhow::bail!("{}", unknown_problem_error(&src_spec.name));
        }
        if dst_variants.is_empty() {
            anyhow::bail!("{}", unknown_problem_error(&dst_spec.name));
        }

        if all {
            let sv = if src_spec.variant_values.is_empty() {
                src_variants[0].clone()
            } else {
                resolve_variant(&src_spec, &src_variants)?
            };
            let dv = if dst_spec.variant_values.is_empty() {
                dst_variants[0].clone()
            } else {
                resolve_variant(&dst_spec, &dst_variants)?
            };
            let mut all_paths = graph.find_all_paths(&src_spec.name, &sv, &dst_spec.name, &dv);
            if all_paths.is_empty() {
                anyhow::bail!(
                    "No reduction path from {} to {}",
                    src_spec.name,
                    dst_spec.name
                );
            }
            all_paths.sort_by_key(|p| p.len());
            let json: serde_json::Value = all_paths
                .iter()
                .map(|p| format_path_json(&graph, p))
                .collect::<Vec<_>>()
                .into();
            return Ok(serde_json::to_string_pretty(&json)?);
        }

        // Single best path
        let src_resolved = if src_spec.variant_values.is_empty() {
            src_variants.clone()
        } else {
            vec![resolve_variant(&src_spec, &src_variants)?]
        };
        let dst_resolved = if dst_spec.variant_values.is_empty() {
            dst_variants.clone()
        } else {
            vec![resolve_variant(&dst_spec, &dst_variants)?]
        };

        let input_size = ProblemSize::new(vec![]);

        enum CostChoice {
            Steps,
            Field(&'static str),
        }
        let cost_choice = if cost == "minimize-steps" {
            CostChoice::Steps
        } else if let Some(field) = cost.strip_prefix("minimize:") {
            CostChoice::Field(Box::leak(field.to_string().into_boxed_str()))
        } else {
            anyhow::bail!(
                "Unknown cost function: {}. Use 'minimize-steps' or 'minimize:<field>'",
                cost
            );
        };

        let mut best_path: Option<problemreductions::rules::ReductionPath> = None;

        for sv in &src_resolved {
            for dv in &dst_resolved {
                let found = match cost_choice {
                    CostChoice::Steps => graph.find_cheapest_path(
                        &src_spec.name,
                        sv,
                        &dst_spec.name,
                        dv,
                        &input_size,
                        &MinimizeSteps,
                    ),
                    CostChoice::Field(f) => graph.find_cheapest_path(
                        &src_spec.name,
                        sv,
                        &dst_spec.name,
                        dv,
                        &input_size,
                        &Minimize(f),
                    ),
                };
                if let Some(p) = found {
                    let is_better = best_path.as_ref().is_none_or(|bp| p.len() < bp.len());
                    if is_better {
                        best_path = Some(p);
                    }
                }
            }
        }

        match best_path {
            Some(ref reduction_path) => {
                let json = format_path_json(&graph, reduction_path);
                Ok(serde_json::to_string_pretty(&json)?)
            }
            None => {
                anyhow::bail!(
                    "No reduction path from {} to {}",
                    src_spec.name,
                    dst_spec.name
                );
            }
        }
    }

    pub fn export_graph_inner(&self) -> anyhow::Result<String> {
        let graph = ReductionGraph::new();
        let json_str = graph
            .to_json_string()
            .map_err(|e| anyhow::anyhow!("Failed to export: {}", e))?;
        Ok(json_str)
    }
}

// ---------------------------------------------------------------------------
// Tool method implementations (wired via rmcp macros)
// ---------------------------------------------------------------------------

#[rmcp::tool_router]
impl McpServer {
    /// List all registered problem types in the reduction graph
    #[tool(
        name = "list_problems",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn list_problems(&self) -> Result<String, String> {
        self.list_problems_inner().map_err(|e| e.to_string())
    }

    /// Show details for a problem type: variants, fields, size fields, and reductions
    #[tool(
        name = "show_problem",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn show_problem(
        &self,
        Parameters(params): Parameters<ShowProblemParams>,
    ) -> Result<String, String> {
        self.show_problem_inner(&params.problem)
            .map_err(|e| e.to_string())
    }

    /// Find neighboring problems reachable via reduction edges
    #[tool(
        name = "neighbors",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn neighbors(&self, Parameters(params): Parameters<NeighborsParams>) -> Result<String, String> {
        let hops = params.hops.unwrap_or(1);
        let direction = params.direction.as_deref().unwrap_or("out");
        self.neighbors_inner(&params.problem, hops, direction)
            .map_err(|e| e.to_string())
    }

    /// Find a reduction path between two problems
    #[tool(
        name = "find_path",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn find_path(&self, Parameters(params): Parameters<FindPathParams>) -> Result<String, String> {
        let cost = params.cost.as_deref().unwrap_or("minimize-steps");
        let all = params.all.unwrap_or(false);
        self.find_path_inner(&params.source, &params.target, cost, all)
            .map_err(|e| e.to_string())
    }

    /// Export the full reduction graph as JSON
    #[tool(
        name = "export_graph",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn export_graph(&self) -> Result<String, String> {
        self.export_graph_inner().map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// ServerHandler wiring
// ---------------------------------------------------------------------------

#[rmcp::tool_handler]
impl rmcp::ServerHandler for McpServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            protocol_version: rmcp::model::ProtocolVersion::V_2025_03_26,
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability::default()),
                ..Default::default()
            },
            server_info: rmcp::model::Implementation {
                name: "problemreductions".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                ..Default::default()
            },
            instructions: Some(
                "MCP server for querying the NP-hard problem reduction graph. \
                 Use list_problems to discover problems, show_problem for details, \
                 neighbors to explore the graph, find_path for reduction paths, \
                 and export_graph for the full graph JSON."
                    .into(),
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn parse_direction(s: &str) -> anyhow::Result<TraversalDirection> {
    match s {
        "out" => Ok(TraversalDirection::Outgoing),
        "in" => Ok(TraversalDirection::Incoming),
        "both" => Ok(TraversalDirection::Both),
        _ => anyhow::bail!("Unknown direction: {}. Use 'out', 'in', or 'both'.", s),
    }
}

fn format_path_json(
    graph: &ReductionGraph,
    reduction_path: &problemreductions::rules::ReductionPath,
) -> serde_json::Value {
    let overheads = graph.path_overheads(reduction_path);
    let steps_json: Vec<serde_json::Value> = reduction_path
        .steps
        .windows(2)
        .zip(overheads.iter())
        .enumerate()
        .map(|(i, (pair, oh))| {
            serde_json::json!({
                "from": {"name": pair[0].name, "variant": pair[0].variant},
                "to": {"name": pair[1].name, "variant": pair[1].variant},
                "step": i + 1,
                "overhead": oh.output_size.iter().map(|(field, poly)| {
                    serde_json::json!({"field": field, "formula": poly.to_string()})
                }).collect::<Vec<_>>(),
            })
        })
        .collect();

    serde_json::json!({
        "steps": reduction_path.len(),
        "path": steps_json,
    })
}
