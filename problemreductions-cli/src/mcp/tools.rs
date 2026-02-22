use problemreductions::models::graph::{
    KColoring, MaxCut, MaximumClique, MaximumIndependentSet, MaximumMatching,
    MinimumDominatingSet, MinimumVertexCover, TravelingSalesman,
};
use problemreductions::models::optimization::{SpinGlass, QUBO};
use problemreductions::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
use problemreductions::models::specialized::Factoring;
use problemreductions::registry::collect_schemas;
use problemreductions::rules::{
    Minimize, MinimizeSteps, ReductionGraph, ReductionPath, TraversalDirection,
};
use problemreductions::topology::{Graph, SimpleGraph};
use problemreductions::types::ProblemSize;
use problemreductions::variant::{K2, K3, KN};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::tool;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::dispatch::{
    load_problem, serialize_any_problem, PathStep, ProblemJson, ProblemJsonOutput, ReductionBundle,
};
use crate::problem_name::{
    aliases_for, parse_problem_spec, resolve_alias, resolve_variant, unknown_problem_error,
};

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
// Parameter structs — instance tools
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateProblemParams {
    #[schemars(
        description = "Problem type (e.g., MIS, SAT, QUBO, MaxCut). Use list_problems to see all types."
    )]
    pub problem_type: String,
    #[schemars(
        description = "Problem parameters as JSON object. Graph problems: {\"edges\": \"0-1,1-2\", \"weights\": \"1,2,3\"}. SAT: {\"num_vars\": 3, \"clauses\": \"1,2;-1,3\"}. QUBO: {\"matrix\": \"1,0.5;0.5,2\"}. KColoring: {\"edges\": \"0-1,1-2\", \"k\": 3}. Factoring: {\"target\": 15, \"bits_m\": 4, \"bits_n\": 4}. Random graph: {\"random\": true, \"num_vertices\": 10, \"edge_prob\": 0.3}"
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
    #[schemars(description = "Target problem type (e.g., QUBO, ILP, SpinGlass)")]
    pub target: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SolveParams {
    #[schemars(description = "Problem JSON string (from create_problem or reduce)")]
    pub problem_json: String,
    #[schemars(description = "Solver: 'ilp' (default) or 'brute-force'")]
    pub solver: Option<String>,
    #[schemars(description = "Timeout in seconds (0 = no limit, default: 0)")]
    pub timeout: Option<u64>,
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

    // -- instance tool inner helpers ------------------------------------------

    pub fn create_problem_inner(
        &self,
        problem_type: &str,
        params: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let canonical = resolve_alias(problem_type);

        // Check for random generation
        let is_random = params
            .get("random")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if is_random {
            return self.create_random_inner(&canonical, params);
        }

        let (data, variant) = match canonical.as_str() {
            // Graph problems with vertex weights
            "MaximumIndependentSet" | "MinimumVertexCover" | "MaximumClique"
            | "MinimumDominatingSet" => {
                let (graph, n) = parse_graph_from_params(params)?;
                let weights = parse_vertex_weights_from_params(params, n)?;
                let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                let data = match canonical.as_str() {
                    "MaximumIndependentSet" => ser(MaximumIndependentSet::new(graph, weights))?,
                    "MinimumVertexCover" => ser(MinimumVertexCover::new(graph, weights))?,
                    "MaximumClique" => ser(MaximumClique::new(graph, weights))?,
                    "MinimumDominatingSet" => ser(MinimumDominatingSet::new(graph, weights))?,
                    _ => unreachable!(),
                };
                (data, variant)
            }

            // Graph problems with edge weights
            "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
                let (graph, _) = parse_graph_from_params(params)?;
                let edge_weights = parse_edge_weights_from_params(params, graph.num_edges())?;
                let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                let data = match canonical.as_str() {
                    "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                    "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                    "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                    _ => unreachable!(),
                };
                (data, variant)
            }

            // KColoring
            "KColoring" => {
                let (graph, _) = parse_graph_from_params(params)?;
                let k = params
                    .get("k")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);
                let variant;
                let data;
                match k {
                    Some(2) => {
                        variant = variant_map(&[("k", "K2"), ("graph", "SimpleGraph")]);
                        data = ser(KColoring::<K2, SimpleGraph>::new(graph))?;
                    }
                    Some(3) => {
                        variant = variant_map(&[("k", "K3"), ("graph", "SimpleGraph")]);
                        data = ser(KColoring::<K3, SimpleGraph>::new(graph))?;
                    }
                    Some(kv) => {
                        variant = variant_map(&[("k", "KN"), ("graph", "SimpleGraph")]);
                        data = ser(KColoring::<KN, SimpleGraph>::with_k(graph, kv))?;
                    }
                    None => anyhow::bail!("KColoring requires 'k' parameter (number of colors)"),
                }
                (data, variant)
            }

            // SAT
            "Satisfiability" => {
                let num_vars = params
                    .get("num_vars")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .ok_or_else(|| anyhow::anyhow!("Satisfiability requires 'num_vars'"))?;
                let clauses = parse_clauses_from_params(params)?;
                let variant = BTreeMap::new();
                (ser(Satisfiability::new(num_vars, clauses))?, variant)
            }
            "KSatisfiability" => {
                let num_vars = params
                    .get("num_vars")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .ok_or_else(|| anyhow::anyhow!("KSatisfiability requires 'num_vars'"))?;
                let clauses = parse_clauses_from_params(params)?;
                let k = params
                    .get("k")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);
                let variant;
                let data;
                match k {
                    Some(2) => {
                        variant = variant_map(&[("k", "K2")]);
                        data = ser(KSatisfiability::<K2>::new(num_vars, clauses))?;
                    }
                    Some(3) => {
                        variant = variant_map(&[("k", "K3")]);
                        data = ser(KSatisfiability::<K3>::new(num_vars, clauses))?;
                    }
                    _ => {
                        variant = variant_map(&[("k", "KN")]);
                        data = ser(KSatisfiability::<KN>::new(num_vars, clauses))?;
                    }
                }
                (data, variant)
            }

            // QUBO
            "QUBO" => {
                let matrix = parse_matrix_from_params(params)?;
                let variant = BTreeMap::new();
                (ser(QUBO::from_matrix(matrix))?, variant)
            }

            // SpinGlass
            "SpinGlass" => {
                let (graph, n) = parse_graph_from_params(params)?;
                let edge_weights = parse_edge_weights_from_params(params, graph.num_edges())?;
                let fields = vec![0i32; n];
                let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                (
                    ser(SpinGlass::from_graph(graph, edge_weights, fields))?,
                    variant,
                )
            }

            // Factoring
            "Factoring" => {
                let target = params
                    .get("target")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| anyhow::anyhow!("Factoring requires 'target'"))?;
                let bits_m = params
                    .get("bits_m")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .ok_or_else(|| anyhow::anyhow!("Factoring requires 'bits_m'"))?;
                let bits_n = params
                    .get("bits_n")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .ok_or_else(|| anyhow::anyhow!("Factoring requires 'bits_n'"))?;
                let variant = BTreeMap::new();
                (ser(Factoring::new(bits_m, bits_n, target))?, variant)
            }

            _ => anyhow::bail!("{}", unknown_problem_error(&canonical)),
        };

        let output = ProblemJsonOutput {
            problem_type: canonical,
            variant,
            data,
        };
        Ok(serde_json::to_string_pretty(&serde_json::to_value(&output)?)?)
    }

    fn create_random_inner(
        &self,
        canonical: &str,
        params: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let num_vertices = params
            .get("num_vertices")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .ok_or_else(|| {
                anyhow::anyhow!("Random generation requires 'num_vertices' parameter")
            })?;
        let edge_prob = params
            .get("edge_prob")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.5);
        if !(0.0..=1.0).contains(&edge_prob) {
            anyhow::bail!("edge_prob must be between 0.0 and 1.0");
        }
        let seed = params
            .get("seed")
            .and_then(|v| v.as_u64());

        let graph = create_random_graph(num_vertices, edge_prob, seed);
        let num_edges = graph.num_edges();

        let (data, variant) = match canonical {
            "MaximumIndependentSet" | "MinimumVertexCover" | "MaximumClique"
            | "MinimumDominatingSet" => {
                let weights = vec![1i32; num_vertices];
                let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                let data = match canonical {
                    "MaximumIndependentSet" => ser(MaximumIndependentSet::new(graph, weights))?,
                    "MinimumVertexCover" => ser(MinimumVertexCover::new(graph, weights))?,
                    "MaximumClique" => ser(MaximumClique::new(graph, weights))?,
                    "MinimumDominatingSet" => ser(MinimumDominatingSet::new(graph, weights))?,
                    _ => unreachable!(),
                };
                (data, variant)
            }
            "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
                let edge_weights = vec![1i32; num_edges];
                let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                let data = match canonical {
                    "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                    "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                    "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                    _ => unreachable!(),
                };
                (data, variant)
            }
            "SpinGlass" => {
                let couplings = vec![1i32; num_edges];
                let fields = vec![0i32; num_vertices];
                let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                (
                    ser(SpinGlass::from_graph(graph, couplings, fields))?,
                    variant,
                )
            }
            "KColoring" => {
                let k = params
                    .get("k")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize)
                    .unwrap_or(3);
                let variant;
                let data;
                match k {
                    2 => {
                        variant = variant_map(&[("k", "K2"), ("graph", "SimpleGraph")]);
                        data = ser(KColoring::<K2, SimpleGraph>::new(graph))?;
                    }
                    3 => {
                        variant = variant_map(&[("k", "K3"), ("graph", "SimpleGraph")]);
                        data = ser(KColoring::<K3, SimpleGraph>::new(graph))?;
                    }
                    _ => {
                        variant = variant_map(&[("k", "KN"), ("graph", "SimpleGraph")]);
                        data = ser(KColoring::<KN, SimpleGraph>::with_k(graph, k))?;
                    }
                }
                (data, variant)
            }
            _ => anyhow::bail!(
                "Random generation is not supported for {}. \
                 Supported: graph-based problems (MIS, MVC, MaxCut, MaxClique, \
                 MaximumMatching, MinimumDominatingSet, SpinGlass, KColoring, TravelingSalesman)",
                canonical
            ),
        };

        let output = ProblemJsonOutput {
            problem_type: canonical.to_string(),
            variant,
            data,
        };
        Ok(serde_json::to_string_pretty(&serde_json::to_value(&output)?)?)
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

        let size_names = problem.problem_size_names_dyn();
        let size_values = problem.problem_size_values_dyn();

        let outgoing = graph.outgoing_reductions(name);
        let mut targets: Vec<String> = outgoing.iter().map(|e| e.target_name.to_string()).collect();
        targets.sort();
        targets.dedup();

        let result = serde_json::json!({
            "kind": "problem",
            "type": name,
            "variant": variant,
            "size": size_names.iter().zip(size_values.iter())
                .map(|(n, v)| serde_json::json!({"field": n, "value": v}))
                .collect::<Vec<_>>(),
            "num_variables": problem.num_variables_dyn(),
            "solvers": ["ilp", "brute-force"],
            "reduces_to": targets,
        });
        Ok(serde_json::to_string_pretty(&result)?)
    }

    pub fn evaluate_inner(
        &self,
        problem_json: &str,
        config: &[usize],
    ) -> anyhow::Result<String> {
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

    pub fn reduce_inner(&self, problem_json: &str, target: &str) -> anyhow::Result<String> {
        let pj: ProblemJson = serde_json::from_str(problem_json)?;
        let source = load_problem(&pj.problem_type, &pj.variant, pj.data.clone())?;

        let source_name = source.problem_name();
        let source_variant = source.variant_map();
        let graph = ReductionGraph::new();

        let dst_spec = parse_problem_spec(target)?;
        let dst_variants = graph.variants_for(&dst_spec.name);
        if dst_variants.is_empty() {
            anyhow::bail!("{}", unknown_problem_error(&dst_spec.name));
        }

        // Auto-discover cheapest path
        let input_size = ProblemSize::new(vec![]);
        let mut best_path: Option<ReductionPath> = None;

        for dv in &dst_variants {
            if let Some(p) = graph.find_cheapest_path(
                source_name,
                &source_variant,
                &dst_spec.name,
                dv,
                &input_size,
                &MinimizeSteps,
            ) {
                let is_better = best_path
                    .as_ref()
                    .is_none_or(|bp: &ReductionPath| p.len() < bp.len());
                if is_better {
                    best_path = Some(p);
                }
            }
        }

        let reduction_path = best_path.ok_or_else(|| {
            anyhow::anyhow!(
                "No reduction path from {} to {}",
                source_name,
                dst_spec.name
            )
        })?;

        // Execute reduction chain
        let chain = graph
            .reduce_along_path(&reduction_path, source.as_any())
            .ok_or_else(|| anyhow::anyhow!("Failed to execute reduction chain"))?;

        // Serialize target
        let target_step = reduction_path.steps.last().unwrap();
        let target_data = serialize_any_problem(
            &target_step.name,
            &target_step.variant,
            chain.target_problem_any(),
        )?;

        // Build reduction bundle
        let bundle = ReductionBundle {
            source: ProblemJsonOutput {
                problem_type: source_name.to_string(),
                variant: source_variant,
                data: pj.data,
            },
            target: ProblemJsonOutput {
                problem_type: target_step.name.clone(),
                variant: target_step.variant.clone(),
                data: target_data,
            },
            path: reduction_path
                .steps
                .iter()
                .map(|s| PathStep {
                    name: s.name.clone(),
                    variant: s.variant.clone(),
                })
                .collect(),
        };

        Ok(serde_json::to_string_pretty(&serde_json::to_value(&bundle)?)?)
    }

    pub fn solve_inner(
        &self,
        problem_json: &str,
        solver: Option<&str>,
        timeout: Option<u64>,
    ) -> anyhow::Result<String> {
        let solver_name = solver.unwrap_or("ilp");
        if solver_name != "brute-force" && solver_name != "ilp" {
            anyhow::bail!(
                "Unknown solver: {}. Available solvers: brute-force, ilp",
                solver_name
            );
        }

        let json: serde_json::Value = serde_json::from_str(problem_json)?;
        let timeout_secs = timeout.unwrap_or(0);

        // Detect if it's a bundle or a problem
        let is_bundle = json.get("source").is_some()
            && json.get("target").is_some()
            && json.get("path").is_some();

        if timeout_secs > 0 {
            let json_clone = json.clone();
            let solver_name = solver_name.to_string();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let result = if is_bundle {
                    match serde_json::from_value::<ReductionBundle>(json_clone) {
                        Ok(b) => solve_bundle_inner(b, &solver_name),
                        Err(e) => Err(anyhow::Error::from(e)),
                    }
                } else {
                    match serde_json::from_value::<ProblemJson>(json_clone) {
                        Ok(pj) => solve_problem_inner(
                            &pj.problem_type,
                            &pj.variant,
                            pj.data,
                            &solver_name,
                        ),
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
            solve_bundle_inner(bundle, solver_name)
        } else {
            let pj: ProblemJson = serde_json::from_value(json)?;
            solve_problem_inner(&pj.problem_type, &pj.variant, pj.data, solver_name)
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
    fn evaluate(
        &self,
        Parameters(params): Parameters<EvaluateParams>,
    ) -> Result<String, String> {
        self.evaluate_inner(&params.problem_json, &params.config)
            .map_err(|e| e.to_string())
    }

    /// Reduce a problem instance to a target problem type, returning a reduction bundle
    #[tool(
        name = "reduce",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn reduce(
        &self,
        Parameters(params): Parameters<ReduceParams>,
    ) -> Result<String, String> {
        self.reduce_inner(&params.problem_json, &params.target)
            .map_err(|e| e.to_string())
    }

    /// Solve a problem instance using brute-force or ILP solver
    #[tool(
        name = "solve",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    fn solve(
        &self,
        Parameters(params): Parameters<SolveParams>,
    ) -> Result<String, String> {
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
                "MCP server for NP-hard problem reductions. \
                 Graph query tools: list_problems, show_problem, neighbors, find_path, export_graph. \
                 Instance tools: create_problem to build instances, inspect_problem for details, \
                 evaluate to test configurations, reduce to transform between problem types, \
                 solve to find optimal solutions."
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

// ---------------------------------------------------------------------------
// Instance tool helpers
// ---------------------------------------------------------------------------

fn ser<T: Serialize>(problem: T) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::to_value(problem)?)
}

fn variant_map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Parse `edges` field from JSON params into a SimpleGraph.
fn parse_graph_from_params(params: &serde_json::Value) -> anyhow::Result<(SimpleGraph, usize)> {
    let edges_str = params
        .get("edges")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("This problem requires 'edges' parameter (e.g., \"0-1,1-2,2-3\")"))?;

    let edges: Vec<(usize, usize)> = edges_str
        .split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split('-').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid edge '{}': expected format u-v", pair.trim());
            }
            let u: usize = parts[0].parse()?;
            let v: usize = parts[1].parse()?;
            Ok((u, v))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let num_vertices = edges
        .iter()
        .flat_map(|(u, v)| [*u, *v])
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);

    Ok((SimpleGraph::new(num_vertices, edges), num_vertices))
}

/// Parse `weights` field from JSON params as vertex weights (i32), defaulting to all 1s.
fn parse_vertex_weights_from_params(
    params: &serde_json::Value,
    num_vertices: usize,
) -> anyhow::Result<Vec<i32>> {
    match params.get("weights").and_then(|v| v.as_str()) {
        Some(w) => {
            let weights: Vec<i32> = w
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if weights.len() != num_vertices {
                anyhow::bail!(
                    "Expected {} weights but got {}",
                    num_vertices,
                    weights.len()
                );
            }
            Ok(weights)
        }
        None => Ok(vec![1i32; num_vertices]),
    }
}

/// Parse `weights` field from JSON params as edge weights (i32), defaulting to all 1s.
fn parse_edge_weights_from_params(
    params: &serde_json::Value,
    num_edges: usize,
) -> anyhow::Result<Vec<i32>> {
    match params.get("weights").and_then(|v| v.as_str()) {
        Some(w) => {
            let weights: Vec<i32> = w
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if weights.len() != num_edges {
                anyhow::bail!(
                    "Expected {} edge weights but got {}",
                    num_edges,
                    weights.len()
                );
            }
            Ok(weights)
        }
        None => Ok(vec![1i32; num_edges]),
    }
}

/// Parse `clauses` field from JSON params as semicolon-separated clauses.
fn parse_clauses_from_params(params: &serde_json::Value) -> anyhow::Result<Vec<CNFClause>> {
    let clauses_str = params
        .get("clauses")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("SAT problems require 'clauses' parameter (e.g., \"1,2;-1,3\")"))?;

    clauses_str
        .split(';')
        .map(|clause| {
            let literals: Vec<i32> = clause
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(CNFClause::new(literals))
        })
        .collect()
}

/// Parse `matrix` field from JSON params as semicolon-separated rows.
fn parse_matrix_from_params(params: &serde_json::Value) -> anyhow::Result<Vec<Vec<f64>>> {
    let matrix_str = params
        .get("matrix")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("QUBO requires 'matrix' parameter (e.g., \"1,0.5;0.5,2\")"))?;

    matrix_str
        .split(';')
        .map(|row| {
            row.trim()
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!("Invalid matrix value: {}", e))
                })
                .collect()
        })
        .collect()
}

/// Generate a random Erdos-Renyi graph using a simple LCG PRNG (no external dependency).
fn create_random_graph(num_vertices: usize, edge_prob: f64, seed: Option<u64>) -> SimpleGraph {
    let mut state: u64 = seed.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    });

    let mut edges = Vec::new();
    for i in 0..num_vertices {
        for j in (i + 1)..num_vertices {
            // LCG step
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let rand_val = (state >> 33) as f64 / (1u64 << 31) as f64;
            if rand_val < edge_prob {
                edges.push((i, j));
            }
        }
    }

    SimpleGraph::new(num_vertices, edges)
}

/// Solve a plain problem and return JSON string.
fn solve_problem_inner(
    problem_type: &str,
    variant: &BTreeMap<String, String>,
    data: serde_json::Value,
    solver_name: &str,
) -> anyhow::Result<String> {
    let problem = load_problem(problem_type, variant, data)?;
    let name = problem.problem_name();

    match solver_name {
        "brute-force" => {
            let result = problem.solve_brute_force()?;
            let json = serde_json::json!({
                "problem": name,
                "solver": "brute-force",
                "solution": result.config,
                "evaluation": result.evaluation,
            });
            Ok(serde_json::to_string_pretty(&json)?)
        }
        "ilp" => {
            let result = problem.solve_with_ilp()?;
            let mut json = serde_json::json!({
                "problem": name,
                "solver": "ilp",
                "solution": result.config,
                "evaluation": result.evaluation,
            });
            if name != "ILP" {
                json["reduced_to"] = serde_json::json!("ILP");
            }
            Ok(serde_json::to_string_pretty(&json)?)
        }
        _ => unreachable!(),
    }
}

/// Solve a reduction bundle: solve the target, then map the solution back.
fn solve_bundle_inner(bundle: ReductionBundle, solver_name: &str) -> anyhow::Result<String> {
    let target = load_problem(
        &bundle.target.problem_type,
        &bundle.target.variant,
        bundle.target.data.clone(),
    )?;
    let target_name = target.problem_name();

    let target_result = match solver_name {
        "brute-force" => target.solve_brute_force()?,
        "ilp" => target.solve_with_ilp()?,
        _ => unreachable!(),
    };

    let source = load_problem(
        &bundle.source.problem_type,
        &bundle.source.variant,
        bundle.source.data.clone(),
    )?;
    let source_name = source.problem_name();

    let graph = ReductionGraph::new();

    let reduction_path = problemreductions::rules::ReductionPath {
        steps: bundle
            .path
            .iter()
            .map(|s| problemreductions::rules::ReductionStep {
                name: s.name.clone(),
                variant: s.variant.clone(),
            })
            .collect(),
    };

    let chain = graph
        .reduce_along_path(&reduction_path, source.as_any())
        .ok_or_else(|| {
            anyhow::anyhow!("Failed to re-execute reduction chain for solution extraction")
        })?;

    let source_config = chain.extract_solution(&target_result.config);
    let source_eval = source.evaluate_dyn(&source_config);

    let json = serde_json::json!({
        "problem": source_name,
        "solver": solver_name,
        "reduced_to": target_name,
        "solution": source_config,
        "evaluation": source_eval,
        "intermediate": {
            "problem": target_name,
            "solution": target_result.config,
            "evaluation": target_result.evaluation,
        },
    });
    Ok(serde_json::to_string_pretty(&json)?)
}
