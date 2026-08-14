use problemreductions::registry::collect_schemas;
use problemreductions::rules::{ReductionGraph, TraversalFlow};
use problemreductions::solvers::SolverRequest;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::tool;
use std::collections::BTreeMap;

use crate::dispatch::{
    load_problem, solve_result_json, solver_capabilities_view, solver_request, BundleReplay,
    ProblemJson, ProblemJsonOutput, ReductionBundle,
};
use crate::problem_name::{aliases_for, resolve_catalog_problem_ref, resolve_problem_ref};

// ---------------------------------------------------------------------------
// Parameter structs — graph query tools
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
    #[schemars(description = "Traversal direction: out (default), in, or both")]
    pub direction: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FindPathParams {
    #[schemars(description = "Source problem name or alias")]
    pub source: String,
    #[schemars(description = "Target problem name or alias")]
    pub target: String,
    #[schemars(description = "Maximum paths to return (default: 20)")]
    pub max_paths: Option<usize>,
    #[schemars(
        description = "Optional complete source problem JSON. When present, execute every returned path and report actual constructed sizes."
    )]
    pub problem_json: Option<String>,
}

// ---------------------------------------------------------------------------
// Parameter structs — instance tools
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateProblemParams {
    #[schemars(
        description = "Problem type (e.g., MIS, SAT, QUBO, MaxCut). Use list_problems to see all types."
    )]
    pub problem_type: String,
    #[schemars(
        description = "Named JSON construction inputs declared by the selected problem variant. Values must use their JSON types; unknown and missing required inputs are errors. Random graph generation remains available with {\"random\": true, \"num_vertices\": 10, \"edge_prob\": 0.3}."
    )]
    pub params: serde_json::Value,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InspectParams {
    #[schemars(description = "Problem JSON string (from create_problem) or reduction bundle JSON")]
    pub problem_json: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EvaluateParams {
    #[schemars(description = "Problem JSON string (from create_problem)")]
    pub problem_json: String,
    #[schemars(
        description = "Configuration to evaluate as array of integers (e.g., [1, 0, 1, 0])"
    )]
    pub config: Vec<usize>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReduceParams {
    #[schemars(description = "Problem JSON string (from create_problem)")]
    pub problem_json: String,
    #[schemars(description = "One explicit path entry selected from find_path output")]
    pub path_json: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SolveParams {
    #[schemars(description = "Problem JSON string (from create_problem or reduce)")]
    pub problem_json: String,
    #[schemars(description = "Solver override: 'ilp' or 'brute-force'; omit for default dispatch")]
    pub solver: Option<String>,
    #[schemars(description = "Timeout in seconds (0 = no limit, default: 0)")]
    pub timeout: Option<u64>,
}

// ---------------------------------------------------------------------------
// McpServer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct McpServer;

// Tool implementations on the server struct.  Each `*_inner` method returns
// `anyhow::Result<String>` (a JSON string) so unit tests can call them directly
// without going through the MCP transport.

impl McpServer {
    pub fn new() -> Self {
        Self
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
        let graph = ReductionGraph::new();
        let resolved = resolve_problem_ref(problem, &graph)?;
        let name = &resolved.name;
        let variant = &resolved.variant;

        let default_variant = graph.default_variant_for(name);
        let is_default = default_variant.as_ref() == Some(variant);

        let schemas = collect_schemas();
        let schema = schemas.iter().find(|s| s.name == *name);

        let outgoing: Vec<_> = graph
            .outgoing_reductions(name)
            .into_iter()
            .filter(|e| &e.source_variant == variant)
            .collect();
        let incoming: Vec<_> = graph
            .incoming_reductions(name)
            .into_iter()
            .filter(|e| &e.target_variant == variant)
            .collect();
        let size_fields = graph.size_field_names(name);
        let complexity = graph.variant_complexity(name, variant).unwrap_or("");

        let edge_to_json = |e: &problemreductions::rules::ReductionEdgeInfo| {
            serde_json::json!({
                "source": {"name": e.source_name, "variant": e.source_variant},
                "target": {"name": e.target_name, "variant": e.target_variant},
                "size_contract": crate::commands::graph::size_contract_to_json(&e.size_contract),
            })
        };

        let mut json = serde_json::json!({
            "name": name,
            "variant": variant,
            "default": is_default,
            "complexity": complexity,
            "size_fields": &size_fields,
            "reduces_to": outgoing.iter().map(&edge_to_json).collect::<Vec<_>>(),
            "reduces_from": incoming.iter().map(&edge_to_json).collect::<Vec<_>>(),
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
        let graph = ReductionGraph::new();
        let resolved = resolve_problem_ref(problem, &graph)?;

        let direction = parse_direction(direction_str)?;

        let neighbors = graph.k_neighbors(&resolved.name, &resolved.variant, hops, direction);

        let json = serde_json::json!({
            "source": resolved.name,
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
        max_paths: usize,
        problem_json: Option<&str>,
    ) -> anyhow::Result<String> {
        let graph = ReductionGraph::new();
        let src_ref = resolve_problem_ref(source, &graph)?;
        let dst_ref = resolve_problem_ref(target, &graph)?;
        let loaded = problem_json
            .map(|content| {
                let problem: ProblemJson = serde_json::from_str(content)?;
                load_problem(&problem.problem_type, &problem.variant, problem.data)
            })
            .transpose()?;
        if let Some(loaded) = &loaded {
            if loaded.problem_name() != src_ref.name || loaded.variant_map() != src_ref.variant {
                anyhow::bail!(
                    "Source argument resolves to {} with variant {:?} but problem_json contains {} with variant {:?}",
                    src_ref.name,
                    src_ref.variant,
                    loaded.problem_name(),
                    loaded.variant_map(),
                );
            }
        }

        let batch = crate::commands::graph::find_path_batch(
            &graph,
            &src_ref.name,
            &src_ref.variant,
            &dst_ref.name,
            &dst_ref.variant,
            max_paths,
        );
        if batch.paths.is_empty() && !batch.truncated {
            anyhow::bail!(
                "No reduction path from {} to {}",
                src_ref.name,
                dst_ref.name
            );
        }

        let executed = loaded
            .as_ref()
            .map(|source| graph.execute_paths(&batch.paths, source.as_any()))
            .transpose()?;
        let json = crate::commands::graph::path_batch_json(&graph, &batch, executed.as_deref())?;
        Ok(serde_json::to_string_pretty(&json)?)
    }

    pub fn export_graph_inner(&self) -> anyhow::Result<String> {
        let graph = ReductionGraph::new();
        let json_str = graph
            .to_json_string()
            .map_err(|e| anyhow::anyhow!("Failed to export: {}", e))?;
        Ok(json_str)
    }

    // -- instance tool inner helpers ------------------------------------------

    pub fn create_problem_inner(
        &self,
        problem_type: &str,
        params: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let resolved = resolve_catalog_problem_ref(problem_type)?;
        let canonical = resolved.name().to_string();
        let resolved_variant = resolved.variant().clone();
        let entry = problemreductions::registry::find_variant_entry(&canonical, &resolved_variant)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No concrete variant is registered for {canonical} with {resolved_variant:?}"
                )
            })?;

        // Check for random generation
        let is_random = params
            .get("random")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_random {
            return self.generate_registered_random_inner(entry, params);
        }

        let normalized = normalize_mcp_create_inputs(params)?;
        let problem = (entry.construct_fn)(normalized)?;

        let output = ProblemJsonOutput {
            problem_type: problem.problem_name().to_string(),
            variant: problem.variant_map(),
            data: problem.serialize_json(),
        };
        Ok(serde_json::to_string_pretty(&output)?)
    }

    fn generate_registered_random_inner(
        &self,
        entry: &problemreductions::registry::VariantEntry,
        params: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let mut inputs = params
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("random inputs must be a JSON object"))?
            .clone();
        inputs.remove("random");
        let random = entry.random.ok_or_else(|| {
            anyhow::anyhow!(
                "Random generation is not registered for {}",
                problemreductions::registry::variant::variant_label(entry)
            )
        })?;
        let problem = (random.generate)(serde_json::Value::Object(inputs))?;
        let output = ProblemJsonOutput {
            problem_type: problem.problem_name().to_string(),
            variant: problem.variant_map(),
            data: problem.serialize_json(),
        };
        Ok(serde_json::to_string_pretty(&output)?)
    }

    pub fn inspect_problem_inner(&self, problem_json: &str) -> anyhow::Result<String> {
        let json: serde_json::Value = serde_json::from_str(problem_json)?;

        // Detect if it's a bundle or a problem
        if json.get("source").is_some()
            && json.get("target").is_some()
            && json.get("path").is_some()
        {
            let bundle: ReductionBundle = serde_json::from_value(json)?;
            let path_str: Vec<&str> = bundle.path.iter().map(|s| s.name.as_str()).collect();
            let result = serde_json::json!({
                "kind": "bundle",
                "source": bundle.source.problem_type,
                "target": bundle.target.problem_type,
                "steps": bundle.path.len().saturating_sub(1),
                "path": path_str,
            });
            return Ok(serde_json::to_string_pretty(&result)?);
        }

        let pj: ProblemJson = serde_json::from_value(json)?;
        let problem = load_problem(&pj.problem_type, &pj.variant, pj.data)?;
        let name = problem.problem_name();
        let variant = problem.variant_map();
        let graph = ReductionGraph::new();

        let size_fields = graph.size_field_names(name);

        let targets =
            crate::commands::inspect::executable_reduction_targets(&graph, name, &variant);
        let solver_view = solver_capabilities_view(&problem)?;

        let result = serde_json::json!({
            "kind": "problem",
            "type": name,
            "variant": variant,
            "size_fields": size_fields,
            "num_variables": problem.num_variables_dyn(),
            "solvers": solver_view.solvers,
            "default_solver": solver_view.default_solver,
            "solver_capabilities": solver_view.capabilities,
            "reduces_to": targets,
        });
        Ok(serde_json::to_string_pretty(&result)?)
    }

    pub fn evaluate_inner(&self, problem_json: &str, config: &[usize]) -> anyhow::Result<String> {
        let pj: ProblemJson = serde_json::from_str(problem_json)?;
        let problem = load_problem(&pj.problem_type, &pj.variant, pj.data)?;

        let dims = problem.dims_dyn();
        if config.len() != dims.len() {
            anyhow::bail!(
                "Config has {} values but problem has {} variables",
                config.len(),
                dims.len()
            );
        }

        let result = problem.evaluate_dyn(config);
        let json = serde_json::json!({
            "problem": problem.problem_name(),
            "config": config,
            "result": result,
        });
        Ok(serde_json::to_string_pretty(&json)?)
    }

    pub fn reduce_inner(&self, problem_json: &str, path_json: &str) -> anyhow::Result<String> {
        let pj: ProblemJson = serde_json::from_str(problem_json)?;
        let reduction_path = crate::commands::reduce::parse_path_json(path_json)?;
        let bundle = crate::commands::reduce::execute_route(pj, reduction_path)?;
        Ok(serde_json::to_string_pretty(&bundle)?)
    }

    pub fn solve_inner(
        &self,
        problem_json: &str,
        solver: Option<&str>,
        timeout: Option<u64>,
    ) -> anyhow::Result<String> {
        let request = solver_request(solver)?;

        let json: serde_json::Value = serde_json::from_str(problem_json)?;
        let timeout_secs = timeout.unwrap_or(0);

        // Detect if it's a bundle or a problem
        let is_bundle = json.get("source").is_some()
            && json.get("target").is_some()
            && json.get("path").is_some();

        if timeout_secs > 0 {
            let json_clone = json.clone();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let result = if is_bundle {
                    match serde_json::from_value::<ReductionBundle>(json_clone) {
                        Ok(b) => solve_bundle_inner(b, request),
                        Err(e) => Err(anyhow::Error::from(e)),
                    }
                } else {
                    match serde_json::from_value::<ProblemJson>(json_clone) {
                        Ok(pj) => {
                            solve_problem_inner(&pj.problem_type, &pj.variant, pj.data, request)
                        }
                        Err(e) => Err(anyhow::Error::from(e)),
                    }
                };
                tx.send(result).ok();
            });
            match rx.recv_timeout(std::time::Duration::from_secs(timeout_secs)) {
                Ok(result) => result,
                Err(_) => anyhow::bail!("Solve timed out after {} seconds", timeout_secs),
            }
        } else if is_bundle {
            let bundle: ReductionBundle = serde_json::from_value(json)?;
            solve_bundle_inner(bundle, request)
        } else {
            let pj: ProblemJson = serde_json::from_value(json)?;
            solve_problem_inner(&pj.problem_type, &pj.variant, pj.data, request)
        }
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
        let max_paths = params.max_paths.unwrap_or(20);
        self.find_path_inner(
            &params.source,
            &params.target,
            max_paths,
            params.problem_json.as_deref(),
        )
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

    /// Create a problem instance from parameters and return its JSON representation
    #[tool(
        name = "create_problem",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn create_problem(
        &self,
        Parameters(params): Parameters<CreateProblemParams>,
    ) -> Result<String, String> {
        self.create_problem_inner(&params.problem_type, &params.params)
            .map_err(|e| e.to_string())
    }

    /// Inspect a problem JSON string or reduction bundle, returning type, size, and available operations
    #[tool(
        name = "inspect_problem",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn inspect_problem(
        &self,
        Parameters(params): Parameters<InspectParams>,
    ) -> Result<String, String> {
        self.inspect_problem_inner(&params.problem_json)
            .map_err(|e| e.to_string())
    }

    /// Evaluate a configuration against a problem instance and return the result
    #[tool(
        name = "evaluate",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn evaluate(&self, Parameters(params): Parameters<EvaluateParams>) -> Result<String, String> {
        self.evaluate_inner(&params.problem_json, &params.config)
            .map_err(|e| e.to_string())
    }

    /// Reduce a problem instance along an explicit enumerated route
    #[tool(
        name = "reduce",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn reduce(&self, Parameters(params): Parameters<ReduceParams>) -> Result<String, String> {
        self.reduce_inner(&params.problem_json, &params.path_json)
            .map_err(|e| e.to_string())
    }

    /// Solve a problem using deterministic default dispatch or an explicit override
    #[tool(
        name = "solve",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn solve(&self, Parameters(params): Parameters<SolveParams>) -> Result<String, String> {
        self.solve_inner(
            &params.problem_json,
            params.solver.as_deref(),
            params.timeout,
        )
        .map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// ServerHandler wiring
// ---------------------------------------------------------------------------

#[rmcp::tool_handler]
impl rmcp::ServerHandler for McpServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        let capabilities = rmcp::model::ServerCapabilities::builder()
            .enable_tools()
            .enable_prompts()
            .build();
        let server_info =
            rmcp::model::Implementation::new("problemreductions", env!("CARGO_PKG_VERSION"));
        rmcp::model::ServerInfo::new(capabilities)
            .with_server_info(server_info)
            .with_instructions(
                "MCP server for NP-hard problem reductions. \
                 Graph query tools: list_problems, show_problem, neighbors, find_path, export_graph. \
                 Instance tools: create_problem to build instances, inspect_problem for details, \
                 evaluate to test configurations, reduce to transform between problem types, \
                 solve to find optimal solutions.",
            )
    }

    async fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListPromptsResult, rmcp::ErrorData> {
        Ok(rmcp::model::ListPromptsResult::with_all_items(
            super::prompts::list_prompts(),
        ))
    }

    async fn get_prompt(
        &self,
        request: rmcp::model::GetPromptRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::GetPromptResult, rmcp::ErrorData> {
        let args = request.arguments.unwrap_or_default();
        super::prompts::get_prompt(&request.name, &args).ok_or_else(|| {
            rmcp::ErrorData::invalid_params(format!("Unknown prompt: {}", request.name), None)
        })
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn parse_direction(s: &str) -> anyhow::Result<TraversalFlow> {
    match s {
        "out" => Ok(TraversalFlow::Outgoing),
        "in" => Ok(TraversalFlow::Incoming),
        "both" => Ok(TraversalFlow::Both),
        _ => anyhow::bail!("Unknown direction: {}. Use 'out', 'in', or 'both'.", s),
    }
}

// ---------------------------------------------------------------------------
// Instance tool helpers
// ---------------------------------------------------------------------------

fn normalize_mcp_create_inputs(params: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
    let inputs = params
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("construction inputs must be a JSON object"))?;
    Ok(serde_json::Value::Object(inputs.clone()))
}

/// Solve a plain problem and return JSON string.
fn solve_problem_inner(
    problem_type: &str,
    variant: &BTreeMap<String, String>,
    data: serde_json::Value,
    request: SolverRequest,
) -> anyhow::Result<String> {
    let problem = load_problem(problem_type, variant, data)?;
    let name = problem.problem_name();
    let result = problem.solve_deterministically(request)?;
    let json = solve_result_json(name, &result);
    Ok(serde_json::to_string_pretty(&json)?)
}

/// Solve a reduction bundle: solve the target, then map the solution back.
fn solve_bundle_inner(bundle: ReductionBundle, request: SolverRequest) -> anyhow::Result<String> {
    let replay = BundleReplay::prepare(&bundle)?;
    Ok(serde_json::to_string_pretty(
        &replay.solve(request)?.to_json(),
    )?)
}

#[cfg(test)]
mod tests {
    use super::McpServer;
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::formula::NonTautology;

    #[test]
    fn construction_contract_create_problem_uses_typed_json_inputs() {
        let server = McpServer::new();
        let output = server
            .create_problem_inner(
                "NonTautology",
                &serde_json::json!({
                    "num_vars": 3,
                    "disjuncts": [[1, 2, 3], [-1, -2, -3]],
                }),
            )
            .unwrap();

        let created: ProblemJsonOutput = serde_json::from_str(&output).unwrap();
        assert_eq!(created.problem_type, "NonTautology");
        let problem: NonTautology = serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.disjuncts(), &[vec![1, 2, 3], vec![-1, -2, -3]]);
    }

    #[test]
    fn construction_contract_discovers_model_without_mcp_dispatch() {
        let server = McpServer::new();
        let output = server
            .create_problem_inner(
                crate::test_support::AGGREGATE_SOURCE_NAME,
                &serde_json::json!({"values": [2, 5, 7]}),
            )
            .unwrap();

        let created: ProblemJsonOutput = serde_json::from_str(&output).unwrap();
        assert_eq!(
            created.problem_type,
            crate::test_support::AGGREGATE_SOURCE_NAME
        );
        assert!(created.variant.is_empty());
        assert_eq!(created.data, serde_json::json!({"values": [2, 5, 7]}));
    }

    #[test]
    fn construction_contract_mcp_computes_model_owned_derived_fields() {
        let output = McpServer::new()
            .create_problem_inner("SCS", &serde_json::json!({"strings": [[0, 1], [1, 2]]}))
            .unwrap();

        let created: ProblemJsonOutput = serde_json::from_str(&output).unwrap();
        assert_eq!(created.problem_type, "ShortestCommonSupersequence");
        assert_eq!(created.data["alphabet_size"], serde_json::json!(3));
        assert_eq!(created.data["max_length"], serde_json::json!(4));
        assert_eq!(created.data["strings"], serde_json::json!([[0, 1], [1, 2]]));
    }

    #[test]
    fn construction_contract_mcp_builds_composite_model_input() {
        let output = McpServer::new()
            .create_problem_inner(
                "BicliqueCover",
                &serde_json::json!({
                    "left": 2,
                    "right": 3,
                    "biedges": [[0, 0], [0, 1], [1, 2]],
                    "k": 2,
                }),
            )
            .unwrap();

        let created: ProblemJsonOutput = serde_json::from_str(&output).unwrap();
        assert_eq!(created.problem_type, "BicliqueCover");
        assert_eq!(created.data["graph"]["left_size"], serde_json::json!(2));
        assert_eq!(created.data["graph"]["right_size"], serde_json::json!(3));
        assert_eq!(created.data["k"], serde_json::json!(2));
    }

    #[test]
    fn construction_contract_rejects_unknown_mcp_input() {
        let error = McpServer::new()
            .create_problem_inner(
                "NonTautology",
                &serde_json::json!({
                    "num_vars": 3,
                    "disjuncts": [[1, 2, 3]],
                    "removed": true,
                }),
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("unknown construction input(s): removed"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn construction_contract_rejects_missing_mcp_input() {
        let error = McpServer::new()
            .create_problem_inner("NonTautology", &serde_json::json!({"num_vars": 3}))
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("missing required construction input(s): disjuncts"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn construction_contract_rejects_non_object_mcp_input() {
        let error = McpServer::new()
            .create_problem_inner("NonTautology", &serde_json::json!([]))
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "construction inputs must be a JSON object"
        );
    }

    #[test]
    fn random_contract_mcp_uses_the_selected_variant_generator() {
        let output = McpServer::new()
            .create_problem_inner(
                "MaximumIndependentSet",
                &serde_json::json!({"random": true, "num_vertices": 4, "seed": 7}),
            )
            .unwrap();

        let created: ProblemJsonOutput = serde_json::from_str(&output).unwrap();
        assert_eq!(created.variant["graph"], "SimpleGraph");
        assert_eq!(created.variant["weight"], "One");
        assert_eq!(created.data["graph"]["num_vertices"], 4);
    }

    #[test]
    fn random_contract_mcp_rejects_inputs_outside_model_contract() {
        let error = McpServer::new()
            .create_problem_inner(
                "MaximumIndependentSet",
                &serde_json::json!({
                    "random": true,
                    "num_vertices": 4,
                    "bound": 2,
                }),
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("unknown construction input(s): bound"),
            "unexpected error: {error}"
        );
    }
}
