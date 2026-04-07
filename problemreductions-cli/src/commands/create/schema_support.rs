use super::*;

#[derive(Debug, Clone, Default)]
pub(super) struct CreateContext {
    num_vertices: Option<usize>,
    num_edges: Option<usize>,
    num_arcs: Option<usize>,
    parsed_fields: BTreeMap<String, serde_json::Value>,
}

impl CreateContext {
    pub(super) fn with_field(mut self, name: &str, value: serde_json::Value) -> Self {
        self.parsed_fields.insert(name.to_string(), value);
        self
    }

    fn seed_field<T: Serialize>(&mut self, name: &str, value: T) -> Result<()> {
        let value = serde_json::to_value(value)?;
        if name == "num_vertices" {
            self.num_vertices = value.as_u64().and_then(|raw| usize::try_from(raw).ok());
        }
        self.parsed_fields.insert(name.to_string(), value);
        Ok(())
    }

    fn usize_field(&self, name: &str) -> Option<usize> {
        self.parsed_fields
            .get(name)
            .and_then(serde_json::Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
    }

    fn f64_field(&self, name: &str) -> Option<f64> {
        self.parsed_fields
            .get(name)
            .and_then(serde_json::Value::as_f64)
    }

    fn remember(&mut self, name: &str, concrete_type: &str, value: &serde_json::Value) {
        self.parsed_fields.insert(name.to_string(), value.clone());

        match normalize_type_name(concrete_type).as_str() {
            "SimpleGraph" => {
                self.num_vertices = value
                    .get("num_vertices")
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|raw| usize::try_from(raw).ok());
                self.num_edges = value
                    .get("edges")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            "DirectedGraph" => {
                self.num_vertices = value
                    .get("num_vertices")
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|raw| usize::try_from(raw).ok());
                self.num_arcs = value
                    .get("arcs")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            "KingsSubgraph" | "TriangularSubgraph" => {
                self.num_vertices = value
                    .get("positions")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            "UnitDiskGraph" => {
                self.num_vertices = value
                    .get("positions")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
                self.num_edges = value
                    .get("edges")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            _ => {}
        }
    }
}

pub(super) fn create_schema_driven(
    args: &CreateArgs,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> Result<Option<(serde_json::Value, BTreeMap<String, String>)>> {
    if !schema_driven_supported_problem(canonical) {
        return Ok(None);
    }

    let Some(schema) = collect_schemas()
        .into_iter()
        .find(|schema| schema.name == canonical)
    else {
        return Ok(None);
    };
    let Some(variant_entry) =
        problemreductions::registry::find_variant_entry(canonical, resolved_variant)
    else {
        return Ok(None);
    };

    let graph_type = resolved_graph_type(resolved_variant);
    let is_geometry = matches!(
        graph_type,
        "KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph"
    );
    let flag_map = args.flag_map();
    let mut context = CreateContext::default();
    seed_schema_context_from_cli(args, graph_type, &mut context)?;
    validate_schema_driven_semantics(args, canonical, resolved_variant, &serde_json::Value::Null)
        .map_err(|error| with_schema_usage(error, canonical, resolved_variant))?;
    let mut json_map = serde_json::Map::new();

    for field in &schema.fields {
        let concrete_type = resolve_schema_field_type(&field.type_name, resolved_variant);
        let flag_keys =
            schema_field_flag_keys(canonical, &field.name, &field.type_name, is_geometry);
        let raw_value = get_schema_flag_value(&flag_map, &flag_keys);
        let value = if !schema_field_requires_derived_input(&field.name, &concrete_type) {
            if let Some(raw_value) = raw_value.clone() {
                match parse_schema_field_value(
                    args,
                    canonical,
                    &concrete_type,
                    &field.name,
                    &raw_value,
                    &context,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(with_schema_usage(error, canonical, resolved_variant))
                    }
                }
            } else if let Some(derived) =
                derive_schema_field_value(args, canonical, &field.name, &concrete_type, &context)?
            {
                derived
            } else {
                return Err(with_schema_usage(
                    missing_schema_field_error(
                        canonical,
                        &field.name,
                        &field.type_name,
                        is_geometry,
                    ),
                    canonical,
                    resolved_variant,
                ));
            }
        } else if let Some(derived) =
            derive_schema_field_value(args, canonical, &field.name, &concrete_type, &context)?
        {
            derived
        } else if let Some(raw_value) = raw_value {
            match parse_schema_field_value(
                args,
                canonical,
                &concrete_type,
                &field.name,
                &raw_value,
                &context,
            ) {
                Ok(value) => value,
                Err(error) => return Err(with_schema_usage(error, canonical, resolved_variant)),
            }
        } else {
            return Err(with_schema_usage(
                missing_schema_field_error(canonical, &field.name, &field.type_name, is_geometry),
                canonical,
                resolved_variant,
            ));
        };

        context.remember(&field.name, &concrete_type, &value);
        json_map.insert(field.name.clone(), value);
    }

    // Decision<P> types serialize as {inner: {graph, weights, ...}, bound} but schema
    // fields are flat (graph, weights, bound).  Restructure when the canonical name
    // indicates a Decision wrapper.
    let data = if canonical.starts_with("Decision") {
        let bound = json_map
            .remove("bound")
            .expect("Decision types require a bound field");
        let mut outer = serde_json::Map::new();
        outer.insert("inner".to_string(), serde_json::Value::Object(json_map));
        outer.insert("bound".to_string(), bound);
        serde_json::Value::Object(outer)
    } else {
        serde_json::Value::Object(json_map)
    };
    validate_schema_driven_semantics(args, canonical, resolved_variant, &data)
        .map_err(|error| with_schema_usage(error, canonical, resolved_variant))?;
    (variant_entry.factory)(data.clone()).map_err(|error| {
        with_schema_usage(
            anyhow::anyhow!(
                "Schema-driven factory rejected generated data for {canonical}: {error}"
            ),
            canonical,
            resolved_variant,
        )
    })?;

    Ok(Some((data, resolved_variant.clone())))
}

pub(super) fn missing_schema_field_error(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> anyhow::Error {
    let display = problem_help_flag_name(canonical, field_name, field_type, is_geometry);
    let flags: Vec<String> = display
        .split('/')
        .filter_map(|part| {
            let trimmed = part.trim().trim_start_matches("--");
            (!trimmed.is_empty()).then(|| format!("--{trimmed}"))
        })
        .collect();
    let requirement = match flags.as_slice() {
        [] => format!("--{}", field_name.replace('_', "-")),
        [flag] => flag.clone(),
        [first, second] => format!("{first} or {second}"),
        _ => {
            let last = flags.last().cloned().unwrap_or_default();
            format!("{}, or {}", flags[..flags.len() - 1].join(", "), last)
        }
    };
    anyhow::anyhow!("{canonical} requires {requirement}")
}

pub(super) fn parse_schema_field_value(
    args: &CreateArgs,
    canonical: &str,
    concrete_type: &str,
    field_name: &str,
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    match (canonical, field_name) {
        ("BoyceCoddNormalFormViolation", "functional_deps") => {
            let num_attributes = args.n.ok_or_else(|| {
                anyhow::anyhow!("BoyceCoddNormalFormViolation requires --n, --sets, and --target")
            })?;
            Ok(serde_json::to_value(parse_bcnf_functional_deps(
                raw,
                num_attributes,
            )?)?)
        }
        ("BoundedComponentSpanningForest", "max_weight") => {
            let usage = "Usage: pred create BoundedComponentSpanningForest --graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --max-weight 6";
            let bound_raw = args.bound.ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --max-weight\n\n{usage}")
            })?;
            let max_weight = i32::try_from(bound_raw).map_err(|_| {
                anyhow::anyhow!(
                    "BoundedComponentSpanningForest requires --max-weight within i32 range\n\n{usage}"
                )
            })?;
            Ok(serde_json::json!(max_weight))
        }
        ("ConsecutiveBlockMinimization", "matrix") => {
            let usage = "Usage: pred create ConsecutiveBlockMinimization --matrix '[[true,false,true],[false,true,true]]' --bound-k 2";
            let matrix: Vec<Vec<bool>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array (e.g., '[[true,false,true],[false,true,true]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("FeasibleBasisExtension", "matrix") => {
            let usage = "Usage: pred create FeasibleBasisExtension --matrix '[[1,0,1],[0,1,0]]' --rhs '7,5' --required-columns '0'";
            let matrix: Vec<Vec<i64>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "FeasibleBasisExtension requires --matrix as a JSON 2D integer array (e.g., '[[1,0,1],[0,1,0]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("IntegralFlowBundles", "bundle_capacities") => {
            let usage = "Usage: pred create IntegralFlowBundles --arcs \"0>1,0>2,1>3,2>3,1>2,2>1\" --bundles \"0,1;2,5;3,4\" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --arcs\n\n{usage}"))?;
            let (_, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let bundles = parse_bundles(args, num_arcs, usage)?;
            Ok(serde_json::to_value(parse_bundle_capacities(
                args,
                bundles.len(),
                usage,
            )?)?)
        }
        ("IntegralFlowHomologousArcs", "homologous_pairs") => {
            Ok(serde_json::to_value(parse_homologous_pairs(args)?)?)
        }
        ("LengthBoundedDisjointPaths", "max_length") => {
            let usage = "Usage: pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --max-length 3";
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --max-length\n\n{usage}")
            })?;
            let max_length = usize::try_from(bound).map_err(|_| {
                anyhow::anyhow!(
                    "--max-length must be a nonnegative integer for LengthBoundedDisjointPaths\n\n{usage}"
                )
            })?;
            Ok(serde_json::json!(max_length))
        }
        ("LongestCommonSubsequence", "strings") => {
            let (strings, _) = parse_lcs_strings(raw)?;
            Ok(serde_json::to_value(strings)?)
        }
        ("MinimumDecisionTree", "test_matrix") => {
            let usage = "Usage: pred create MinimumDecisionTree --test-matrix '[[true,true,false,false],[true,false,false,false],[false,true,false,true]]' --num-objects 4 --num-tests 3";
            let matrix: Vec<Vec<bool>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "MinimumDecisionTree requires --test-matrix as a JSON 2D bool array\n\n{usage}\n\nFailed to parse --test-matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("MinimumWeightDecoding", "matrix") => {
            let usage = "Usage: pred create MinimumWeightDecoding --matrix '[[true,false,true],[false,true,true]]' --rhs 'true,true'";
            let matrix: Vec<Vec<bool>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "MinimumWeightDecoding requires --matrix as a JSON 2D bool array (e.g., '[[true,false],[false,true]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("MinimumWeightSolutionToLinearEquations", "matrix") => {
            let usage = "Usage: pred create MinimumWeightSolutionToLinearEquations --matrix '[[1,2,3,1],[2,1,1,3]]' --rhs '5,4'";
            let matrix: Vec<Vec<i64>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "MinimumWeightSolutionToLinearEquations requires --matrix as a JSON 2D integer array (e.g., '[[1,2,3],[4,5,6]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("GroupingBySwapping", "string")
        | ("StringToStringCorrection", "source")
        | ("StringToStringCorrection", "target") => {
            Ok(serde_json::to_value(parse_symbol_list_allow_empty(raw)?)?)
        }
        ("MultipleCopyFileAllocation", "usage") => {
            let (_, num_vertices) = parse_graph(args)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{MULTIPLE_COPY_FILE_ALLOCATION_USAGE}"))?;
            Ok(serde_json::to_value(parse_vertex_i64_values(
                args.usage.as_deref(),
                "usage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?)?)
        }
        ("MultipleCopyFileAllocation", "storage") => {
            let (_, num_vertices) = parse_graph(args)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{MULTIPLE_COPY_FILE_ALLOCATION_USAGE}"))?;
            Ok(serde_json::to_value(parse_vertex_i64_values(
                args.storage.as_deref(),
                "storage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?)?)
        }
        ("SequencingToMinimizeMaximumCumulativeCost", "precedences") => {
            Ok(serde_json::to_value(parse_precedence_pairs(
                args.precedences
                    .as_deref()
                    .or(args.precedence_pairs.as_deref()),
            )?)?)
        }
        ("UndirectedTwoCommodityIntegralFlow", "capacities") => {
            let usage = "Usage: pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            Ok(serde_json::to_value(parse_capacities(
                args,
                graph.num_edges(),
                usage,
            )?)?)
        }
        _ => parse_field_value(concrete_type, field_name, raw, context),
    }
}

pub(super) fn schema_driven_supported_problem(canonical: &str) -> bool {
    canonical != "ILP" && canonical != "CircuitSAT"
}

pub(super) fn schema_field_flag_keys(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> Vec<String> {
    let mut keys = vec![field_name.replace('_', "-")];
    for display_key in problem_help_flag_name(canonical, field_name, field_type, is_geometry)
        .split('/')
        .map(|key| key.trim().trim_start_matches("--").to_string())
        .filter(|key| !key.is_empty())
    {
        if !keys.contains(&display_key) {
            keys.push(display_key);
        }
    }
    // Decision types accept --k as an alias for --bound (backward compat with old VertexCover)
    if field_name == "bound" && canonical.starts_with("Decision") && !keys.contains(&"k".into()) {
        keys.push("k".into());
    }
    keys
}

pub(super) fn get_schema_flag_value(
    flag_map: &std::collections::HashMap<&'static str, Option<String>>,
    keys: &[String],
) -> Option<String> {
    keys.iter()
        .find_map(|key| flag_map.get(key.as_str()).cloned().flatten())
}

pub(super) fn resolve_schema_field_type(
    type_name: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> String {
    let normalized = normalize_type_name(type_name);
    let graph_type = resolved_variant
        .get("graph")
        .map(String::as_str)
        .unwrap_or("SimpleGraph");
    let weight_type = resolved_variant
        .get("weight")
        .map(String::as_str)
        .unwrap_or("One");

    match normalized.as_str() {
        "G" => graph_type.to_string(),
        "W" => weight_type.to_string(),
        "W::Sum" => weight_sum_type(weight_type).to_string(),
        "Vec<W>" => format!("Vec<{weight_type}>"),
        "Vec<Vec<W>>" => format!("Vec<Vec<{weight_type}>>"),
        "Vec<(usize,usize,W)>" => format!("Vec<(usize,usize,{weight_type})>"),
        "Vec<Vec<T>>" => format!("Vec<Vec<{weight_type}>>"),
        other => other.to_string(),
    }
}

pub(super) fn weight_sum_type(weight_type: &str) -> &'static str {
    match weight_type {
        "One" | "i32" => "i32",
        "f64" => "f64",
        _ => "i32",
    }
}

pub(super) fn seed_schema_context_from_cli(
    args: &CreateArgs,
    graph_type: &str,
    context: &mut CreateContext,
) -> Result<()> {
    if let Some(num_vertices) = args.num_vertices {
        context.seed_field("num_vertices", num_vertices)?;
    }
    if graph_type == "UnitDiskGraph" {
        context.seed_field("radius", args.radius.unwrap_or(1.0))?;
    }
    Ok(())
}

pub(super) fn derive_schema_field_value(
    args: &CreateArgs,
    canonical: &str,
    field_name: &str,
    concrete_type: &str,
    context: &CreateContext,
) -> Result<Option<serde_json::Value>> {
    if let Some(defaulted) =
        derive_schema_default_value(canonical, field_name, concrete_type, context)?
    {
        return Ok(Some(defaulted));
    }

    if field_name == "graph" && concrete_type == "MixedGraph" {
        let usage = format!(
            "Usage: pred create {canonical} {}",
            example_for(canonical, None)
        );
        return Ok(Some(serde_json::to_value(parse_mixed_graph(
            args, &usage,
        )?)?));
    }

    if field_name == "graph" && concrete_type == "BipartiteGraph" {
        let left = args
            .left
            .ok_or_else(|| anyhow::anyhow!("{canonical} requires --left"))?;
        let right = args
            .right
            .ok_or_else(|| anyhow::anyhow!("{canonical} requires --right"))?;
        let edges_raw = args
            .biedges
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("{canonical} requires --biedges"))?;
        let edges = util::parse_edge_pairs(edges_raw)?;
        validate_bipartite_edges(canonical, left, right, &edges)?;
        return Ok(Some(serde_json::to_value(BipartiteGraph::new(
            left, right, edges,
        ))?));
    }

    if canonical == "ClosestVectorProblem"
        && field_name == "bounds"
        && normalize_type_name(concrete_type) == "Vec<VarBounds>"
    {
        return Ok(Some(parse_cvp_bounds_value(
            args.bounds.as_deref(),
            context,
        )?));
    }

    if canonical == "ConjunctiveBooleanQuery"
        && field_name == "num_variables"
        && normalize_type_name(concrete_type) == "usize"
    {
        let raw = args
            .conjuncts_spec
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("ConjunctiveBooleanQuery requires --conjuncts-spec"))?;
        return Ok(Some(serde_json::json!(infer_cbq_num_variables(raw)?)));
    }

    if canonical == "GroupingBySwapping"
        && field_name == "alphabet_size"
        && normalize_type_name(concrete_type) == "usize"
    {
        let raw = args
            .string
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("GroupingBySwapping requires --string"))?;
        let string = parse_symbol_list_allow_empty(raw)?;
        let inferred = string.iter().copied().max().map_or(0, |value| value + 1);
        return Ok(Some(serde_json::json!(args
            .alphabet_size
            .unwrap_or(inferred))));
    }

    if canonical == "JobShopScheduling"
        && field_name == "num_processors"
        && normalize_type_name(concrete_type) == "usize"
    {
        let usage = "Usage: pred create JobShopScheduling --jobs \"0:3,1:4;1:2,0:3,1:2;0:4,1:3\" --num-processors 2";
        let inferred_processors = match args.job_tasks.as_deref() {
            Some(job_tasks) => {
                let jobs = parse_job_shop_jobs(job_tasks)?;
                jobs.iter()
                    .flat_map(|job| job.iter().map(|(processor, _)| *processor))
                    .max()
                    .map(|processor| processor + 1)
            }
            None => None,
        };
        let num_processors =
            resolve_processor_count_flags("JobShopScheduling", usage, args.num_processors, args.m)?
                .or(inferred_processors)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Cannot infer num_processors from empty job list; use --num-processors"
                    )
                })?;
        return Ok(Some(serde_json::json!(num_processors)));
    }

    if canonical == "LongestCommonSubsequence"
        && field_name == "alphabet_size"
        && normalize_type_name(concrete_type) == "usize"
    {
        let raw = args
            .strings
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("LongestCommonSubsequence requires --strings"))?;
        let (_, inferred_alphabet_size) = parse_lcs_strings(raw)?;
        return Ok(Some(serde_json::json!(args
            .alphabet_size
            .unwrap_or(inferred_alphabet_size))));
    }

    if canonical == "LongestCommonSubsequence"
        && field_name == "max_length"
        && normalize_type_name(concrete_type) == "usize"
    {
        let strings: Vec<Vec<usize>> =
            serde_json::from_value(context.parsed_fields.get("strings").cloned().ok_or_else(
                || anyhow::anyhow!("LCS max_length derivation requires parsed strings"),
            )?)?;
        let max_length = strings.iter().map(Vec::len).min().unwrap_or(0);
        return Ok(Some(serde_json::json!(max_length)));
    }

    if canonical == "QUBO"
        && field_name == "num_vars"
        && normalize_type_name(concrete_type) == "usize"
    {
        let matrix = parse_matrix(args)?;
        return Ok(Some(serde_json::json!(matrix.len())));
    }

    if canonical == "StringToStringCorrection"
        && field_name == "alphabet_size"
        && normalize_type_name(concrete_type) == "usize"
    {
        let source = parse_symbol_list_allow_empty(args.source_string.as_deref().unwrap_or(""))?;
        let target = parse_symbol_list_allow_empty(args.target_string.as_deref().unwrap_or(""))?;
        let inferred = source
            .iter()
            .chain(target.iter())
            .copied()
            .max()
            .map_or(0, |value| value + 1);
        return Ok(Some(serde_json::json!(args
            .alphabet_size
            .unwrap_or(inferred))));
    }

    if field_name == "precedences"
        && normalize_type_name(concrete_type) == "Vec<(usize,usize)>"
        && args.precedences.is_none()
        && args.precedence_pairs.is_none()
    {
        return Ok(Some(serde_json::json!([])));
    }

    if canonical == "ComparativeContainment"
        && matches!(field_name, "r_weights" | "s_weights")
        && matches!(
            normalize_type_name(concrete_type).as_str(),
            "Vec<One>" | "Vec<i32>" | "Vec<f64>"
        )
    {
        let sets_len = context
            .parsed_fields
            .get(match field_name {
                "r_weights" => "r_sets",
                _ => "s_sets",
            })
            .and_then(serde_json::Value::as_array)
            .map(Vec::len);
        if let Some(len) = sets_len {
            let value = match normalize_type_name(concrete_type).as_str() {
                "Vec<One>" | "Vec<i32>" => serde_json::json!(vec![1_i32; len]),
                "Vec<f64>" => serde_json::json!(vec![1.0_f64; len]),
                _ => unreachable!(),
            };
            return Ok(Some(value));
        }
    }

    if canonical == "ConsistencyOfDatabaseFrequencyTables"
        && field_name == "known_values"
        && normalize_type_name(concrete_type) == "Vec<KnownValue>"
        && args.known_values.is_none()
    {
        return Ok(Some(serde_json::json!([])));
    }

    if canonical == "LengthBoundedDisjointPaths"
        && field_name == "max_paths"
        && normalize_type_name(concrete_type) == "usize"
    {
        let graph_value = context.parsed_fields.get("graph").cloned();
        let source = context.usize_field("source");
        let sink = context.usize_field("sink");
        if let (Some(graph_value), Some(source), Some(sink)) = (graph_value, source, sink) {
            let graph: SimpleGraph =
                serde_json::from_value(graph_value).context("Failed to deserialize graph")?;
            let max_paths = graph
                .neighbors(source)
                .len()
                .min(graph.neighbors(sink).len());
            return Ok(Some(serde_json::json!(max_paths)));
        }
    }

    Ok(None)
}

pub(super) fn derive_schema_default_value(
    canonical: &str,
    field_name: &str,
    concrete_type: &str,
    context: &CreateContext,
) -> Result<Option<serde_json::Value>> {
    let normalized = normalize_type_name(concrete_type);

    let one_list = |len: usize| match normalized.as_str() {
        "Vec<One>" | "Vec<i32>" => Some(serde_json::json!(vec![1_i32; len])),
        "Vec<u64>" => Some(serde_json::json!(vec![1_u64; len])),
        "Vec<i64>" => Some(serde_json::json!(vec![1_i64; len])),
        "Vec<usize>" => Some(serde_json::json!(vec![1_usize; len])),
        "Vec<f64>" => Some(serde_json::json!(vec![1.0_f64; len])),
        _ => None,
    };

    let derived = match field_name {
        "weights" | "vertex_weights" => context.num_vertices.and_then(one_list),
        "edge_weights" | "edge_lengths" => context.num_edges.and_then(one_list),
        "arc_weights" | "arc_lengths" if context.num_arcs.is_some() => {
            context.num_arcs.and_then(one_list)
        }
        "capacities" if canonical == "PathConstrainedNetworkFlow" => {
            context.num_arcs.and_then(one_list)
        }
        "couplings" if canonical == "SpinGlass" => context.num_edges.and_then(one_list),
        "fields" if canonical == "SpinGlass" => match normalized.as_str() {
            "Vec<i32>" => context
                .num_vertices
                .map(|len| serde_json::json!(vec![0_i32; len])),
            "Vec<f64>" => context
                .num_vertices
                .map(|len| serde_json::json!(vec![0.0_f64; len])),
            _ => None,
        },
        _ => None,
    };

    Ok(derived)
}

pub(super) fn schema_field_requires_derived_input(field_name: &str, concrete_type: &str) -> bool {
    field_name == "graph" && matches!(concrete_type, "MixedGraph" | "BipartiteGraph")
}

pub(super) fn with_schema_usage(
    error: anyhow::Error,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> anyhow::Error {
    let message = error.to_string();
    if message.contains("Usage: pred create") {
        return error;
    }
    let graph_type = resolved_variant.get("graph").map(String::as_str);
    anyhow::anyhow!(
        "{message}\n\nUsage: pred create {canonical} {}",
        example_for(canonical, graph_type)
    )
}

pub(super) fn parse_field_value(
    concrete_type: &str,
    field_name: &str,
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    let normalized_type = normalize_type_name(concrete_type);
    let value = match normalized_type.as_str() {
        "SimpleGraph" => parse_simple_graph_value(raw, context)?,
        "DirectedGraph" => parse_directed_graph_value(raw, context)?,
        "KingsSubgraph" => parse_grid_subgraph_value(raw, true)?,
        "TriangularSubgraph" => parse_grid_subgraph_value(raw, false)?,
        "UnitDiskGraph" => parse_unit_disk_graph_value(raw, context)?,
        "Vec<i32>" => parse_numeric_list_value::<i32>(raw)?,
        "Vec<f64>" => parse_numeric_list_value::<f64>(raw)?,
        "Vec<u64>" => parse_numeric_list_value::<u64>(raw)?,
        "Vec<i64>" => parse_numeric_list_value::<i64>(raw)?,
        "Vec<usize>" => parse_numeric_list_value::<usize>(raw)?,
        "Vec<One>" => parse_numeric_list_value::<i32>(raw)?,
        "Vec<bool>" => parse_bool_list_value(raw)?,
        "Vec<Vec<usize>>" => parse_nested_numeric_list_value::<usize>(raw)?,
        "Vec<Vec<u64>>" => parse_nested_numeric_list_value::<u64>(raw)?,
        "Vec<Vec<i32>>" => parse_nested_numeric_list_value::<i32>(raw)?,
        "Vec<Vec<i64>>" => parse_nested_numeric_list_value::<i64>(raw)?,
        "Vec<Vec<f64>>" => parse_nested_numeric_list_value::<f64>(raw)?,
        "Vec<Vec<One>>" => parse_nested_numeric_list_value::<i32>(raw)?,
        "Vec<Vec<bool>>" => parse_bool_rows_value(raw, field_name)?,
        "Vec<Vec<Vec<usize>>>" => parse_3d_numeric_list_value::<usize>(raw)?,
        "Vec<Vec<Vec<i64>>>" => parse_3d_numeric_list_value::<i64>(raw)?,
        "Vec<[usize;3]>" => parse_triple_array_list_value(raw)?,
        "Vec<CNFClause>" => serde_json::to_value(parse_clauses_raw(raw)?)?,
        "Vec<(usize,usize)>" => parse_pair_list_value(raw)?,
        "Vec<(u64,u64)>" => parse_semicolon_tuple_list_value::<u64, 2>(raw)?,
        "Vec<(usize,f64)>" => parse_indexed_numeric_pairs_value::<f64>(raw)?,
        "Vec<(usize,usize,usize)>" => parse_semicolon_tuple_list_value::<usize, 3>(raw)?,
        "Vec<(usize,usize,usize,usize)>" => parse_semicolon_tuple_list_value::<usize, 4>(raw)?,
        "Vec<(usize,usize,One)>" => parse_weighted_edge_list_value::<i32>(raw)?,
        "Vec<(usize,usize,i32)>" => parse_weighted_edge_list_value::<i32>(raw)?,
        "Vec<(usize,usize,i64)>" => parse_weighted_edge_list_value::<i64>(raw)?,
        "Vec<(usize,usize,u64)>" => parse_weighted_edge_list_value::<u64>(raw)?,
        "Vec<(usize,usize,f64)>" => parse_weighted_edge_list_value::<f64>(raw)?,
        "Vec<(Vec<usize>,Vec<usize>)>" => serde_json::to_value(parse_dependencies(raw)?)?,
        "Vec<(Vec<usize>,usize)>" => serde_json::to_value(parse_implications(raw)?)?,
        "Vec<(usize,Vec<QueryArg>)>" => serde_json::to_value(parse_cbq_conjuncts(raw, context)?)?,
        "Vec<(usize,Vec<usize>)>" => parse_indexed_usize_lists_value(raw)?,
        "Vec<Vec<(usize,u64)>>" => serde_json::to_value(parse_job_shop_jobs(raw)?)?,
        "Vec<(f64,f64)>" => serde_json::to_value(util::parse_positions::<f64>(raw, "0.0,0.0")?)?,
        "Vec<FrequencyTable>" => {
            serde_json::to_value(parse_cdft_frequency_tables_value(raw, context)?)?
        }
        "Vec<KnownValue>" => serde_json::to_value(parse_cdft_known_values_value(raw, context)?)?,
        "Vec<Relation>" => serde_json::to_value(parse_cbq_relations(raw, context)?)?,
        "Vec<String>" => parse_string_list_value(raw)?,
        "Vec<VarBounds>" => parse_cvp_bounds_value(Some(raw), context)?,
        "Vec<BigUint>" => parse_biguint_list_value(raw)?,
        "BigUint" => parse_biguint_value(raw)?,
        "Vec<Option<bool>>" => parse_optional_bool_list_value(raw)?,
        "Vec<Quantifier>" => serde_json::to_value(parse_quantifiers_raw(raw, context)?)?,
        "IntExpr" => parse_json_passthrough_value(raw)?,
        "bool" => serde_json::to_value(parse_bool_token(raw.trim())?)?,
        "One" => serde_json::json!(1),
        "usize" => parse_scalar_value::<usize>(raw)?,
        "u64" => parse_scalar_value::<u64>(raw)?,
        "i32" => parse_scalar_value::<i32>(raw)?,
        "i64" => parse_scalar_value::<i64>(raw)?,
        "f64" => parse_scalar_value::<f64>(raw)?,
        other => bail!("Unsupported schema parser for field '{field_name}' with type '{other}'"),
    };

    Ok(value)
}

pub(super) fn normalize_type_name(type_name: &str) -> String {
    type_name.chars().filter(|ch| !ch.is_whitespace()).collect()
}

pub(super) fn parse_scalar_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    Ok(serde_json::to_value(raw.trim().parse::<T>().map_err(
        |err| anyhow::anyhow!("Invalid value '{}': {err}", raw.trim()),
    )?)?)
}

pub(super) fn parse_numeric_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    Ok(serde_json::to_value(util::parse_comma_list::<T>(raw)?)?)
}

pub(super) fn parse_bool_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<bool> = raw
        .split(',')
        .map(|entry| parse_bool_token(entry.trim()))
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(values)?)
}

pub(super) fn parse_bool_rows_value(raw: &str, field_name: &str) -> Result<serde_json::Value> {
    let flag = format!("--{}", field_name.replace('_', "-"));
    let rows = parse_bool_rows(raw)
        .map_err(|err| anyhow::anyhow!("{}", err.to_string().replace("--matrix", &flag)))?;
    Ok(serde_json::to_value(rows)?)
}

pub(super) fn parse_nested_numeric_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let rows: Vec<Vec<T>> = raw
        .split(';')
        .map(|row| util::parse_comma_list::<T>(row.trim()))
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(rows)?)
}

pub(super) fn parse_3d_numeric_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let matrices: Vec<Vec<Vec<T>>> = raw
        .split('|')
        .map(|matrix| {
            matrix
                .split(';')
                .map(|row| util::parse_comma_list::<T>(row.trim()))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(matrices)?)
}

pub(super) fn parse_triple_array_list_value(raw: &str) -> Result<serde_json::Value> {
    let triples: Vec<[usize; 3]> = raw
        .split(';')
        .map(|entry| {
            let values: Vec<usize> = util::parse_comma_list(entry.trim())?;
            anyhow::ensure!(
                values.len() == 3,
                "Expected triple with exactly 3 entries, got {}",
                values.len()
            );
            Ok([values[0], values[1], values[2]])
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(triples)?)
}

pub(super) fn parse_clauses_raw(raw: &str) -> Result<Vec<CNFClause>> {
    raw.split(';')
        .map(|clause| {
            let literals: Vec<i32> = clause
                .trim()
                .split(',')
                .map(|value| value.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(CNFClause::new(literals))
        })
        .collect()
}

pub(super) fn parse_pair_list_value(raw: &str) -> Result<serde_json::Value> {
    let pairs: Vec<(usize, usize)> = raw
        .split(',')
        .map(|entry| {
            let entry = entry.trim();
            let parts: Vec<&str> = if entry.contains('>') {
                entry.split('>').collect()
            } else {
                entry.split('-').collect()
            };
            anyhow::ensure!(
                parts.len() == 2,
                "Invalid pair '{entry}': expected u-v or u>v"
            );
            Ok((
                parts[0].trim().parse::<usize>()?,
                parts[1].trim().parse::<usize>()?,
            ))
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(pairs)?)
}

pub(super) fn infer_cbq_num_variables(raw: &str) -> Result<usize> {
    let mut num_vars = 0usize;
    for conjunct in raw.split(';').filter(|entry| !entry.trim().is_empty()) {
        let (_, args_str) = conjunct.trim().split_once(':').ok_or_else(|| {
            anyhow::anyhow!(
                "Invalid conjunct format: expected 'rel_idx:args', got '{}'",
                conjunct.trim()
            )
        })?;
        for arg in args_str
            .split(',')
            .map(str::trim)
            .filter(|arg| !arg.is_empty())
        {
            if let Some(rest) = arg.strip_prefix('v') {
                let index: usize = rest
                    .parse()
                    .map_err(|err| anyhow::anyhow!("Invalid variable index '{rest}': {err}"))?;
                num_vars = num_vars.max(index + 1);
            }
        }
    }
    Ok(num_vars)
}

pub(super) fn parse_cbq_relations(raw: &str, context: &CreateContext) -> Result<Vec<CbqRelation>> {
    let domain_size = context.usize_field("domain_size").ok_or_else(|| {
        anyhow::anyhow!("CBQ relation parsing requires a prior domain_size field")
    })?;

    raw.split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|rel_str| {
            let rel_str = rel_str.trim();
            let (arity_str, tuples_str) = rel_str.split_once(':').ok_or_else(|| {
                anyhow::anyhow!("Invalid relation format: expected 'arity:tuples', got '{rel_str}'")
            })?;
            let arity: usize = arity_str
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid arity '{arity_str}': {e}"))?;
            let tuples: Vec<Vec<usize>> = if tuples_str.trim().is_empty() {
                Vec::new()
            } else {
                tuples_str
                    .split('|')
                    .filter(|tuple| !tuple.trim().is_empty())
                    .map(|tuple| {
                        let tuple: Vec<usize> = util::parse_comma_list(tuple.trim())?;
                        anyhow::ensure!(
                            tuple.len() == arity,
                            "Relation tuple has {} entries, expected arity {arity}",
                            tuple.len()
                        );
                        for &value in &tuple {
                            anyhow::ensure!(
                                value < domain_size,
                                "Tuple value {value} >= domain-size {domain_size}"
                            );
                        }
                        Ok(tuple)
                    })
                    .collect::<Result<_>>()?
            };
            Ok(CbqRelation { arity, tuples })
        })
        .collect()
}

pub(super) fn parse_cbq_conjuncts(
    raw: &str,
    context: &CreateContext,
) -> Result<Vec<(usize, Vec<QueryArg>)>> {
    let relations: Vec<CbqRelation> =
        serde_json::from_value(context.parsed_fields.get("relations").cloned().ok_or_else(
            || anyhow::anyhow!("CBQ conjunct parsing requires prior relations field"),
        )?)
        .context("Failed to deserialize parsed CBQ relations")?;
    let domain_size = context
        .usize_field("domain_size")
        .ok_or_else(|| anyhow::anyhow!("CBQ conjunct parsing requires prior domain_size field"))?;
    let num_variables = context.usize_field("num_variables").unwrap_or(0);

    raw.split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|conj_str| {
            let conj_str = conj_str.trim();
            let (idx_str, args_str) = conj_str.split_once(':').ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid conjunct format: expected 'rel_idx:args', got '{conj_str}'"
                )
            })?;
            let rel_idx: usize = idx_str
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid relation index '{idx_str}': {e}"))?;
            anyhow::ensure!(
                rel_idx < relations.len(),
                "Conjunct references relation {rel_idx}, but only {} relations exist",
                relations.len()
            );

            let query_args: Vec<QueryArg> = args_str
                .split(',')
                .map(|arg| {
                    let arg = arg.trim();
                    if let Some(rest) = arg.strip_prefix('v') {
                        let variable: usize = rest
                            .parse()
                            .map_err(|e| anyhow::anyhow!("Invalid variable index '{rest}': {e}"))?;
                        anyhow::ensure!(
                            variable < num_variables,
                            "Variable({variable}) >= num_variables ({num_variables})"
                        );
                        Ok(QueryArg::Variable(variable))
                    } else if let Some(rest) = arg.strip_prefix('c') {
                        let constant: usize = rest
                            .parse()
                            .map_err(|e| anyhow::anyhow!("Invalid constant value '{rest}': {e}"))?;
                        anyhow::ensure!(
                            constant < domain_size,
                            "Constant {constant} >= domain-size {domain_size}"
                        );
                        Ok(QueryArg::Constant(constant))
                    } else {
                        Err(anyhow::anyhow!(
                            "Invalid query arg '{arg}': expected vN (variable) or cN (constant)"
                        ))
                    }
                })
                .collect::<Result<_>>()?;
            anyhow::ensure!(
                query_args.len() == relations[rel_idx].arity,
                "Conjunct has {} args, but relation {rel_idx} has arity {}",
                query_args.len(),
                relations[rel_idx].arity
            );
            Ok((rel_idx, query_args))
        })
        .collect()
}

pub(super) fn parse_semicolon_tuple_list_value<T, const N: usize>(
    raw: &str,
) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let tuples: Vec<Vec<T>> = raw
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let values: Vec<T> = util::parse_comma_list(entry.trim())?;
            anyhow::ensure!(
                values.len() == N,
                "Expected tuple with {N} entries, got {}",
                values.len()
            );
            Ok(values)
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(tuples)?)
}

pub(super) fn parse_weighted_edge_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let edges: Vec<(usize, usize, T)> = raw
        .split(',')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let entry = entry.trim();
            let (edge_part, weight_part) = entry.split_once(':').ok_or_else(|| {
                anyhow::anyhow!("Invalid weighted edge '{entry}': expected u-v:w")
            })?;
            let (u_str, v_str) = if let Some((u, v)) = edge_part.split_once('-') {
                (u, v)
            } else if let Some((u, v)) = edge_part.split_once('>') {
                (u, v)
            } else {
                bail!("Invalid weighted edge '{entry}': expected u-v:w or u>v:w");
            };
            Ok((
                u_str.trim().parse::<usize>()?,
                v_str.trim().parse::<usize>()?,
                weight_part.trim().parse::<T>().map_err(|err| {
                    anyhow::anyhow!("Invalid edge weight '{}': {err}", weight_part.trim())
                })?,
            ))
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(edges)?)
}

pub(super) fn parse_indexed_numeric_pairs_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let pairs: Vec<(usize, T)> =
        raw.split(',')
            .filter(|entry| !entry.trim().is_empty())
            .map(|entry| {
                let entry = entry.trim();
                let (index, value) = entry.split_once(':').ok_or_else(|| {
                    anyhow::anyhow!("Invalid pair '{entry}': expected index:value")
                })?;
                Ok((
                    index.trim().parse::<usize>()?,
                    value.trim().parse::<T>().map_err(|err| {
                        anyhow::anyhow!("Invalid value '{}': {err}", value.trim())
                    })?,
                ))
            })
            .collect::<Result<_>>()?;
    Ok(serde_json::to_value(pairs)?)
}

pub(super) fn parse_indexed_usize_lists_value(raw: &str) -> Result<serde_json::Value> {
    let entries: Vec<(usize, Vec<usize>)> = raw
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let entry = entry.trim();
            let (index, values) = entry
                .split_once(':')
                .ok_or_else(|| anyhow::anyhow!("Invalid entry '{entry}': expected index:values"))?;
            Ok((
                index.trim().parse::<usize>()?,
                if values.trim().is_empty() {
                    Vec::new()
                } else {
                    util::parse_comma_list(values.trim())?
                },
            ))
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(entries)?)
}

pub(super) fn parse_string_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<String> = raw
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| entry.trim().to_string())
        .collect();
    Ok(serde_json::to_value(values)?)
}

pub(super) fn parse_symbol_list_allow_empty(raw: &str) -> Result<Vec<usize>> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    raw.split(',')
        .map(|value| {
            value
                .trim()
                .parse::<usize>()
                .context("invalid symbol index")
        })
        .collect()
}

pub(super) fn parse_lcs_strings(raw: &str) -> Result<(Vec<Vec<usize>>, usize)> {
    let segments: Vec<&str> = raw.split(';').map(str::trim).collect();
    let comma_mode = segments.iter().any(|segment| segment.contains(','));

    if comma_mode {
        let strings = segments
            .iter()
            .map(|segment| parse_symbol_list_allow_empty(segment))
            .collect::<Result<Vec<_>>>()?;
        let inferred_alphabet_size = strings
            .iter()
            .flat_map(|string| string.iter())
            .copied()
            .max()
            .map(|value| value + 1)
            .unwrap_or(0);
        return Ok((strings, inferred_alphabet_size));
    }

    let mut encoding = BTreeMap::new();
    let mut next_symbol = 0usize;
    let strings = segments
        .iter()
        .map(|segment| {
            segment
                .as_bytes()
                .iter()
                .map(|byte| {
                    let entry = encoding.entry(*byte).or_insert_with(|| {
                        let current = next_symbol;
                        next_symbol += 1;
                        current
                    });
                    *entry
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok((strings, next_symbol))
}

pub(super) fn parse_bcnf_functional_deps(
    raw: &str,
    num_attributes: usize,
) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
    raw.split(';')
        .map(|fd_str| {
            let parts: Vec<&str> = fd_str.split(':').collect();
            anyhow::ensure!(
                parts.len() == 2,
                "Each FD must be lhs:rhs, got '{}'",
                fd_str
            );
            let lhs: Vec<usize> = util::parse_comma_list(parts[0])?;
            let rhs: Vec<usize> = util::parse_comma_list(parts[1])?;
            ensure_attribute_indices_in_range(
                &lhs,
                num_attributes,
                &format!("Functional dependency '{fd_str}' lhs"),
            )?;
            ensure_attribute_indices_in_range(
                &rhs,
                num_attributes,
                &format!("Functional dependency '{fd_str}' rhs"),
            )?;
            Ok((lhs, rhs))
        })
        .collect()
}

pub(super) fn parse_cdft_frequency_tables_value(
    raw: &str,
    context: &CreateContext,
) -> Result<Vec<FrequencyTable>> {
    let attribute_domains: Vec<usize> = serde_json::from_value(
        context
            .parsed_fields
            .get("attribute_domains")
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "CDFT frequency table parsing requires prior attribute_domains field"
                )
            })?,
    )
    .context("Failed to deserialize parsed CDFT attribute domains")?;
    let num_objects = context.usize_field("num_objects").ok_or_else(|| {
        anyhow::anyhow!("CDFT frequency table parsing requires prior num_objects field")
    })?;
    parse_cdft_frequency_tables(raw, &attribute_domains, num_objects)
}

pub(super) fn parse_cdft_known_values_value(
    raw: &str,
    context: &CreateContext,
) -> Result<Vec<KnownValue>> {
    let attribute_domains: Vec<usize> = serde_json::from_value(
        context
            .parsed_fields
            .get("attribute_domains")
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!("CDFT known-value parsing requires prior attribute_domains field")
            })?,
    )
    .context("Failed to deserialize parsed CDFT attribute domains")?;
    let num_objects = context.usize_field("num_objects").ok_or_else(|| {
        anyhow::anyhow!("CDFT known-value parsing requires prior num_objects field")
    })?;
    parse_cdft_known_values(Some(raw), num_objects, &attribute_domains)
}

pub(super) fn parse_cvp_bounds_value(
    raw: Option<&str>,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    let basis_len = context
        .parsed_fields
        .get("basis")
        .and_then(serde_json::Value::as_array)
        .map(Vec::len)
        .ok_or_else(|| anyhow::anyhow!("CVP bounds parsing requires a prior basis field"))?;

    let (lower, upper) = match raw {
        Some(raw) => {
            let parts: Vec<i64> = util::parse_comma_list(raw)?;
            anyhow::ensure!(
                parts.len() == 2,
                "--bounds expects \"lower,upper\" (e.g., \"-10,10\")"
            );
            (parts[0], parts[1])
        }
        None => (-10, 10),
    };
    let bounds =
        vec![problemreductions::models::algebraic::VarBounds::bounded(lower, upper); basis_len];
    Ok(serde_json::to_value(bounds)?)
}

pub(super) fn parse_biguint_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<String> = util::parse_biguint_list(raw)?
        .into_iter()
        .map(|value| value.to_string())
        .collect();
    Ok(serde_json::to_value(values)?)
}

pub(super) fn parse_biguint_value(raw: &str) -> Result<serde_json::Value> {
    let value: BigUint = util::parse_decimal_biguint(raw)?;
    Ok(serde_json::Value::String(value.to_string()))
}

pub(super) fn parse_optional_bool_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<Option<bool>> = raw
        .split(',')
        .map(|entry| {
            let entry = entry.trim();
            match entry {
                "?" => Ok(None),
                _ => Ok(Some(parse_bool_token(entry)?)),
            }
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(values)?)
}

pub(super) fn parse_quantifiers_raw(raw: &str, context: &CreateContext) -> Result<Vec<Quantifier>> {
    let quantifiers: Vec<Quantifier> = raw
        .split(',')
        .map(|entry| match entry.trim().to_lowercase().as_str() {
            "e" | "exists" => Ok(Quantifier::Exists),
            "a" | "forall" => Ok(Quantifier::ForAll),
            other => Err(anyhow::anyhow!(
                "Invalid quantifier '{}': expected E/Exists or A/ForAll",
                other
            )),
        })
        .collect::<Result<_>>()?;

    if let Some(num_vars) = context.usize_field("num_vars") {
        anyhow::ensure!(
            quantifiers.len() == num_vars,
            "Expected {num_vars} quantifiers but got {}",
            quantifiers.len()
        );
    }

    Ok(quantifiers)
}

pub(super) fn parse_json_passthrough_value(raw: &str) -> Result<serde_json::Value> {
    serde_json::from_str(raw).context("Invalid JSON input")
}

pub(super) fn parse_bool_token(raw: &str) -> Result<bool> {
    match raw.trim() {
        "1" | "true" | "TRUE" | "True" => Ok(true),
        "0" | "false" | "FALSE" | "False" => Ok(false),
        other => bail!("Invalid boolean entry '{other}': expected 0/1 or true/false"),
    }
}

pub(super) fn parse_simple_graph_value(
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    let raw = raw.trim();
    let num_vertices = context.usize_field("num_vertices").or(context.num_vertices);
    let graph = if raw.is_empty() {
        let num_vertices = num_vertices.ok_or_else(|| {
            anyhow::anyhow!(
                "Empty graph string. To create a graph with isolated vertices, provide num_vertices first."
            )
        })?;
        SimpleGraph::empty(num_vertices)
    } else {
        let edges = util::parse_edge_pairs(raw)?;
        let inferred_num_vertices = edges
            .iter()
            .flat_map(|&(u, v)| [u, v])
            .max()
            .map(|max_vertex| max_vertex + 1)
            .unwrap_or(0);
        let num_vertices = match num_vertices {
            Some(explicit) => {
                anyhow::ensure!(
                    explicit >= inferred_num_vertices,
                    "num_vertices ({explicit}) is too small for the graph: need at least {inferred_num_vertices}"
                );
                explicit
            }
            None => inferred_num_vertices,
        };
        SimpleGraph::new(num_vertices, edges)
    };
    Ok(serde_json::to_value(graph)?)
}

pub(super) fn parse_directed_graph_value(
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    let (graph, _) = parse_directed_graph(
        raw,
        context.usize_field("num_vertices").or(context.num_vertices),
    )?;
    Ok(serde_json::to_value(graph)?)
}

pub(super) fn parse_grid_subgraph_value(raw: &str, kings: bool) -> Result<serde_json::Value> {
    let positions = util::parse_positions::<i32>(raw, "0,0")?;
    if kings {
        Ok(serde_json::to_value(KingsSubgraph::new(positions))?)
    } else {
        Ok(serde_json::to_value(TriangularSubgraph::new(positions))?)
    }
}

pub(super) fn parse_unit_disk_graph_value(
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    let positions = util::parse_positions::<f64>(raw, "0.0,0.0")?;
    let radius = context
        .f64_field("radius")
        .ok_or_else(|| anyhow::anyhow!("UnitDiskGraph parsing requires a prior radius field"))?;
    Ok(serde_json::to_value(UnitDiskGraph::new(positions, radius))?)
}

pub(super) fn type_format_hint(type_name: &str, graph_type: Option<&str>) -> &'static str {
    match type_name {
        "SimpleGraph" => "edge list: 0-1,1-2,2-3",
        "G" => match graph_type {
            Some("KingsSubgraph" | "TriangularSubgraph") => "integer positions: \"0,0;1,0;1,1\"",
            Some("UnitDiskGraph") => "float positions: \"0.0,0.0;1.0,0.0\"",
            _ => "edge list: 0-1,1-2,2-3",
        },
        "Vec<(Vec<usize>, Vec<usize>)>" => "semicolon-separated dependencies: \"0,1>2;0,2>3\"",
        "Vec<u64>" => "comma-separated integers: 4,5,3,2,6",
        "Vec<W>" => "comma-separated: 1,2,3",
        "W" | "N" | "W::Sum" | "N::Sum" => "numeric value: 10",
        "Vec<usize>" => "comma-separated indices: 0,2,4",
        "Vec<(usize, usize, W)>" | "Vec<(usize,usize,W)>" => {
            "comma-separated weighted edges: 0-2:3,1-3:5"
        }
        "Vec<Vec<usize>>" => "semicolon-separated sets: \"0,1;1,2;0,2\"",
        "Vec<CNFClause>" => "semicolon-separated clauses: \"1,2;-1,3\"",
        "Vec<Vec<bool>>" => "JSON 2D bool array: '[[true,false],[false,true]]'",
        "Vec<Vec<W>>" => "semicolon-separated rows: \"1,0.5;0.5,2\"",
        "usize" => "integer",
        "u64" => "integer",
        "i64" => "integer",
        "BigUint" => "nonnegative decimal integer",
        "Vec<BigUint>" => "comma-separated nonnegative decimal integers: 3,7,1,8",
        "Vec<i64>" => "comma-separated integers: 3,7,1,8",
        "DirectedGraph" => "directed arcs: 0>1,1>2,2>0",
        _ => "value",
    }
}

pub(super) fn example_for(canonical: &str, graph_type: Option<&str>) -> &'static str {
    match canonical {
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet" => match graph_type {
            Some("KingsSubgraph") => "--positions \"0,0;1,0;1,1;0,1\"",
            Some("TriangularSubgraph") => "--positions \"0,0;0,1;1,0;1,1\"",
            Some("UnitDiskGraph") => "--positions \"0,0;1,0;0.5,0.8\" --radius 1.5",
            _ => "--graph 0-1,1-2,2-3 --weights 1,1,1,1",
        },
        "DecisionMinimumVertexCover" => match graph_type {
            Some("KingsSubgraph") => {
                "--positions \"0,0;1,0;1,1;0,1\" --weights 1,1,1,1 --bound 2"
            }
            Some("TriangularSubgraph") => {
                "--positions \"0,0;0,1;1,0;1,1\" --weights 1,1,1,1 --bound 2"
            }
            Some("UnitDiskGraph") => {
                "--positions \"0,0;1,0;0.5,0.8\" --radius 1.5 --weights 1,1,1 --bound 2"
            }
            _ => "--graph 0-1,1-2,0-2,2-3 --weights 1,1,1,1 --bound 2",
        },
        "KClique" => "--graph 0-1,0-2,1-3,2-3,2-4,3-4 --k 3",
        "GeneralizedHex" => "--graph 0-1,0-2,0-3,1-4,2-4,3-4,4-5 --source 0 --sink 5",
        "IntegralFlowBundles" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>2,2>1\" --bundles \"0,1;2,5;3,4\" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4"
        }
        "IntegralFlowWithMultipliers" => {
            "--arcs \"0>1,0>2,1>3,2>3\" --capacities 1,1,2,2 --source 0 --sink 3 --multipliers 1,2,3,1 --requirement 2"
        }
        "MinimumCutIntoBoundedSets" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1 --source 0 --sink 3 --size-bound 3"
        }
        "BoundedComponentSpanningForest" => {
            "--graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --max-weight 6"
        }
        "HamiltonianPath" => "--graph 0-1,1-2,2-3",
        "HamiltonianPathBetweenTwoVertices" => {
            "--graph 0-1,0-3,1-2,1-4,2-5,3-4,4-5,2-3 --source-vertex 0 --target-vertex 5"
        }
        "GraphPartitioning" => "--graph 0-1,1-2,2-3,3-0 --num-partitions 2",
        "LongestPath" => {
            "--graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,4-6,5-6,1-6 --edge-lengths 3,2,4,1,5,2,3,2,4,1 --source-vertex 0 --target-vertex 6"
        }
        "UndirectedFlowLowerBounds" => {
            "--graph 0-1,0-2,1-3,2-3,1-4,3-5,4-5 --capacities 2,2,2,2,1,3,2 --lower-bounds 1,1,0,0,1,0,1 --source 0 --sink 5 --requirement 3"
        }
        "UndirectedTwoCommodityIntegralFlow" => {
            "--graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1"
        },
        "DisjointConnectingPaths" => {
            "--graph 0-1,1-3,0-2,1-4,2-4,3-5,4-5 --terminal-pairs 0-3,2-5"
        }
        "IntegralFlowHomologousArcs" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>4,2>4,3>5,4>5\" --capacities 1,1,1,1,1,1,1,1 --source 0 --sink 5 --requirement 2 --homologous-pairs \"2=5;4=3\""
        }
        "LengthBoundedDisjointPaths" => {
            "--graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --max-length 4"
        }
        "PathConstrainedNetworkFlow" => {
            "--arcs \"0>1,0>2,1>3,1>4,2>4,3>5,4>5,4>6,5>7,6>7\" --capacities 2,1,1,1,1,1,1,1,2,1 --source 0 --sink 7 --paths \"0,2,5,8;0,3,6,8;0,3,7,9;1,4,6,8;1,4,7,9\" --requirement 3"
        }
        "IsomorphicSpanningTree" => "--graph 0-1,1-2,0-2 --tree 0-1,1-2",
        "BoundedDiameterSpanningTree" => {
            "--graph 0-1,0-2,0-3,1-2,1-4,2-3,3-4 --edge-weights 1,2,1,1,2,1,1 --weight-bound 5 --diameter-bound 3"
        }
        "KthBestSpanningTree" => "--graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3",
        "LongestCircuit" => {
            "--graph 0-1,1-2,2-3,3-4,4-5,5-0,0-3,1-4,2-5,3-5 --edge-weights 3,2,4,1,5,2,3,2,1,2"
        }
        "BottleneckTravelingSalesman" | "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1"
        }
        "ShortestWeightConstrainedPath" => {
            "--graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4 --edge-lengths 2,4,3,1,5,4,2,6 --edge-weights 5,1,2,3,2,3,1,1 --source-vertex 0 --target-vertex 5 --weight-bound 8"
        }
        "SteinerTreeInGraphs" => "--graph 0-1,1-2,2-3 --edge-weights 1,1,1 --terminals 0,3",
        "BiconnectivityAugmentation" => {
            "--graph 0-1,1-2,2-3 --potential-weights 0-2:3,0-3:4,1-3:2 --budget 5"
        }
        "PartialFeedbackEdgeSet" => {
            "--graph 0-1,1-2,2-0,2-3,3-4,4-2,3-5,5-4,0-3 --budget 3 --max-cycle-length 4"
        }
        "Satisfiability" => "--num-vars 3 --clauses \"1,2;-1,3\"",
        "NAESatisfiability" => "--num-vars 3 --clauses \"1,2,-3;-1,2,3\"",
        "QuantifiedBooleanFormulas" => {
            "--num-vars 3 --clauses \"1,2;-1,3\" --quantifiers \"E,A,E\""
        }
        "KSatisfiability" => "--num-vars 3 --clauses \"1,2,3;-1,2,-3\" --k 3",
        "Maximum2Satisfiability" => "--num-vars 4 --clauses \"1,2;1,-2;-1,3;-1,-3;2,4;-3,-4;3,4\"",
        "NonTautology" => {
            "--num-vars 3 --disjuncts \"1,2,3;-1,-2,-3\""
        }
        "OneInThreeSatisfiability" => {
            "--num-vars 4 --clauses \"1,2,3;-1,3,4;2,-3,-4\""
        }
        "Planar3Satisfiability" => {
            "--num-vars 4 --clauses \"1,2,3;-1,2,4;1,-3,4;-2,3,-4\""
        }
        "QUBO" => "--matrix \"1,0.5;0.5,2\"",
        "QuadraticAssignment" => "--matrix \"0,5;5,0\" --distance-matrix \"0,1;1,0\"",
        "SpinGlass" => "--graph 0-1,1-2 --couplings 1,1",
        "KColoring" => "--graph 0-1,1-2,2-0 --k 3",
        "HamiltonianCircuit" => "--graph 0-1,1-2,2-3,3-0",
        "MaximumLeafSpanningTree" => "--graph 0-1,0-2,0-3,1-4,2-4,2-5,3-5,4-5,1-3",
        "EnsembleComputation" => "--universe-size 4 --subsets \"0,1,2;0,1,3\"",
        "RootedTreeStorageAssignment" => {
            "--universe-size 5 --subsets \"0,2;1,3;0,4;2,4\" --bound 1"
        }
        "MinMaxMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "MinimumSumMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "BalancedCompleteBipartiteSubgraph" => {
            "--left 4 --right 4 --biedges 0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3 --k 3"
        }
        "MaximumAchromaticNumber" => "--graph 0-1,1-2,2-3,3-4,4-5,5-0",
        "MaximumDomaticNumber" => "--graph 0-1,1-2,0-2",
        "MinimumCoveringByCliques" => "--graph 0-1,1-2,0-2,2-3",
        "MinimumIntersectionGraphBasis" => "--graph 0-1,1-2",
        "MinimumMaximalMatching" => "--graph 0-1,1-2,2-3,3-4,4-5",
        "DegreeConstrainedSpanningTree" => "--graph 0-1,0-2,0-3,1-2,1-4,2-3,3-4 --k 2",
        "MonochromaticTriangle" => "--graph 0-1,0-2,0-3,1-2,1-3,2-3",
        "PartitionIntoTriangles" => "--graph 0-1,1-2,0-2",
        "PartitionIntoCliques" => "--graph 0-1,0-2,1-2,3-4,3-5,4-5 --k 3",
        "PartitionIntoForests" => "--graph 0-1,1-2,2-0,3-4,4-5,5-3 --k 2",
        "PartitionIntoPerfectMatchings" => "--graph 0-1,2-3,0-2,1-3 --k 2",
        "Factoring" => "--target 15 --m 4 --n 4",
        "CapacityAssignment" => {
            "--capacities 1,2,3 --cost-matrix \"1,3,6;2,4,7;1,2,5\" --delay-matrix \"8,4,1;7,3,1;6,3,1\" --delay-budget 12"
        }
        "ProductionPlanning" => {
            "--num-periods 6 --demands 5,3,7,2,8,5 --capacities 12,12,12,12,12,12 --setup-costs 10,10,10,10,10,10 --production-costs 1,1,1,1,1,1 --inventory-costs 1,1,1,1,1,1 --cost-bound 80"
        }
        "MultiprocessorScheduling" => "--lengths 4,5,3,2,6 --num-processors 2 --deadline 10",
        "PreemptiveScheduling" => {
            "--lengths 2,1,3,2,1 --num-processors 2 --precedences \"0>2,1>3\""
        }
        "SchedulingToMinimizeWeightedCompletionTime" => {
            "--lengths 1,2,3,4,5 --weights 6,4,3,2,1 --num-processors 2"
        }
        "JobShopScheduling" => {
            "--jobs \"0:3,1:4;1:2,0:3,1:2;0:4,1:3;1:5,0:2;0:2,1:3,0:1\" --num-processors 2"
        }
        "MinimumMultiwayCut" => "--graph 0-1,1-2,2-3 --terminals 0,2 --edge-weights 1,1,1",
        "ExpectedRetrievalCost" => EXPECTED_RETRIEVAL_COST_EXAMPLE_ARGS,
        "SequencingWithinIntervals" => "--release-times 0,0,5 --deadlines 11,11,6 --lengths 3,1,1",
        "StaffScheduling" => {
            "--schedules \"1,1,1,1,1,0,0;0,1,1,1,1,1,0;0,0,1,1,1,1,1;1,0,0,1,1,1,1;1,1,0,0,1,1,1\" --requirements 2,2,2,3,3,2,1 --num-workers 4 --k 5"
        }
        "TimetableDesign" => {
            "--num-periods 3 --num-craftsmen 5 --num-tasks 5 --craftsman-avail \"1,1,1;1,1,0;0,1,1;1,0,1;1,1,1\" --task-avail \"1,1,0;0,1,1;1,0,1;1,1,1;1,1,1\" --requirements \"1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0\""
        }
        "SteinerTree" => "--graph 0-1,1-2,1-3,3-4 --edge-weights 2,2,1,1 --terminals 0,2,4",
        "MultipleCopyFileAllocation" => {
            MULTIPLE_COPY_FILE_ALLOCATION_EXAMPLE_ARGS
        }
        "AcyclicPartition" => {
            "--arcs \"0>1,0>2,1>3,1>4,2>4,2>5,3>5,4>5\" --weights 2,3,2,1,3,1 --arc-weights 1,1,1,1,1,1,1,1 --weight-bound 5 --cost-bound 5"
        }
        "OptimalLinearArrangement" => "--graph 0-1,1-2,2-3",
        "RootedTreeArrangement" => "--graph 0-1,0-2,1-2,2-3,3-4 --bound 7",
        "DirectedTwoCommodityIntegralFlow" => {
            "--arcs \"0>2,0>3,1>2,1>3,2>4,2>5,3>4,3>5\" --capacities 1,1,1,1,1,1,1,1 --source-1 0 --sink-1 4 --source-2 1 --sink-2 5 --requirement-1 1 --requirement-2 1"
        }
        "MinimumEdgeCostFlow" => {
            "--arcs \"0>1,0>2,0>3,1>4,2>4,3>4\" --edge-weights 3,1,2,0,0,0 --capacities 2,2,2,2,2,2 --source 0 --sink 4 --requirement 3"
        }
        "MinimumFeedbackArcSet" => "--arcs \"0>1,1>2,2>0\"",
        "DirectedHamiltonianPath" => {
            "--arcs \"0>1,0>3,1>3,1>4,2>0,2>4,3>2,3>5,4>5,5>1\" --num-vertices 6"
        }
        "Kernel" => "--arcs \"0>1,0>2,1>3,2>3,3>4,4>0,4>1\"",
        "MinimumGeometricConnectedDominatingSet" => {
            "--positions \"0,0;3,0;6,0;9,0;0,3;3,3;6,3;9,3\" --radius 3.5"
        }
        "MinimumDummyActivitiesPert" => "--arcs \"0>2,0>3,1>3,1>4,2>5\" --num-vertices 6",
        "FeasibleRegisterAssignment" => {
            "--arcs \"0>1,0>2,1>3\" --assignment 0,1,0,0 --k 2 --num-vertices 4"
        }
        "MinimumFaultDetectionTestSet" => {
            "--arcs \"0>2,0>3,1>3,1>4,2>5,3>5,3>6,4>6\" --inputs 0,1 --outputs 5,6 --num-vertices 7"
        }
        "MinimumWeightAndOrGraph" => {
            "--arcs \"0>1,0>2,1>3,1>4,2>5,2>6\" --source 0 --gate-types \"AND,OR,OR,L,L,L,L\" --weights 1,2,3,1,4,2 --num-vertices 7"
        }
        "MinimumRegisterSufficiencyForLoops" => {
            "--loop-length 6 --loop-variables \"0,3;2,3;4,3\""
        }
        "RegisterSufficiency" => {
            "--arcs \"2>0,2>1,3>1,4>2,4>3,5>0,6>4,6>5\" --bound 3 --num-vertices 7"
        }
        "StrongConnectivityAugmentation" => {
            "--arcs \"0>1,1>2\" --candidate-arcs \"2>0:1\" --bound 1"
        }
        "MixedChinesePostman" => {
            "--graph 0-2,1-3,0-4,4-2 --arcs \"0>1,1>2,2>3,3>0\" --edge-weights 2,3,1,2 --arc-weights 2,3,1,4"
        }
        "RuralPostman" => {
            "--graph 0-1,1-2,2-3,3-0 --edge-weights 1,1,1,1 --required-edges 0,2"
        }
        "StackerCrane" => {
            "--arcs \"0>4,2>5,5>1,3>0,4>3\" --graph \"0-1,1-2,2-3,3-5,4-5,0-3,1-5\" --arc-lengths 3,4,2,5,3 --edge-lengths 2,1,3,2,1,4,3 --num-vertices 6"
        }
        "MultipleChoiceBranching" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --threshold 10"
        }
        "AdditionalKey" => "--num-attributes 6 --dependencies \"0,1:2,3;2,3:4,5;4,5:0,1\" --relation-attrs 0,1,2,3,4,5 --known-keys \"0,1;2,3;4,5\"",
        "ConsistencyOfDatabaseFrequencyTables" => {
            "--num-objects 6 --attribute-domains \"2,3,2\" --frequency-tables \"0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1\" --known-values \"0,0,0;3,0,1;1,2,1\""
        }
        "SubgraphIsomorphism" => "--graph 0-1,1-2,2-0 --pattern 0-1",
        "RectilinearPictureCompression" => {
            "--matrix \"1,1,0,0;1,1,0,0;0,0,1,1;0,0,1,1\" --bound 2"
        }
        "SequencingToMinimizeWeightedTardiness" => {
            "--lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
        }
        "IntegerKnapsack" => "--sizes 3,4,5,2,7 --values 4,5,7,3,9 --capacity 15",
        "SubsetProduct" => "--sizes 2,3,5,7,6,10 --target 210",
        "SubsetSum" => "--sizes 3,7,1,8,2,4 --target 11",
        "MinimumAxiomSet" => {
            "--n 8 --true-sentences 0,1,2,3,4,5,6,7 --implications \"0>2;0>3;1>4;1>5;2,4>6;3,5>7;6,7>0;6,7>1\""
        }
        "IntegerExpressionMembership" => {
            "--expression '{\"Sum\":[{\"Sum\":[{\"Union\":[{\"Atom\":1},{\"Atom\":4}]},{\"Union\":[{\"Atom\":3},{\"Atom\":6}]}]},{\"Union\":[{\"Atom\":2},{\"Atom\":5}]}]}' --target 12"
        }
        "NonLivenessFreePetriNet" => {
            "--n 4 --m 3 --arcs \"0>0,1>1,2>2\" --output-arcs \"0>1,1>2,2>3\" --initial-marking 1,0,0,0"
        }
        "Betweenness" => "--n 5 --sets \"0,1,2;2,3,4;0,2,4;1,3,4\"",
        "CyclicOrdering" => "--n 5 --sets \"0,1,2;2,3,0;1,3,4\"",
        "Numerical3DimensionalMatching" => "--w-sizes 4,5 --x-sizes 4,5 --y-sizes 5,7 --bound 15",
        "ThreePartition" => "--sizes 4,5,6,4,6,5 --bound 15",
        "DynamicStorageAllocation" => "--release-times 0,0,1,2,3 --deadlines 3,2,4,5,5 --sizes 2,3,1,3,2 --capacity 6",
        "KthLargestMTuple" => "--sets \"2,5,8;3,6;1,4,7\" --k 14 --bound 12",
        "AlgebraicEquationsOverGF2" => "--num-vars 3 --equations \"0,1:2;1,2:0:;0:1:2:\"",
        "QuadraticCongruences" => "--coeff-a 4 --coeff-b 15 --coeff-c 10",
        "QuadraticDiophantineEquations" => "--coeff-a 3 --coeff-b 5 --coeff-c 53",
        "SimultaneousIncongruences" => "--pairs \"2,2;1,3;2,5;3,7\"",
        "BoyceCoddNormalFormViolation" => {
            "--n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
        }
        "Clustering" => {
            "--distance-matrix \"0,1,1,3;1,0,1,3;1,1,0,3;3,3,3,0\" --k 2 --diameter-bound 1"
        }
        "SumOfSquaresPartition" => "--sizes 5,3,8,2,7,1 --num-groups 3",
        "ComparativeContainment" => {
            "--universe-size 4 --r-sets \"0,1,2,3;0,1\" --s-sets \"0,1,2,3;2,3\" --r-weights 2,5 --s-weights 3,6"
        }
        "SetBasis" => "--universe-size 4 --subsets \"0,1;1,2;0,2;0,1,2\" --k 3",
        "SetSplitting" => "--universe-size 6 --subsets \"0,1,2;2,3,4;0,4,5;1,3,5\"",
        "LongestCommonSubsequence" => {
            "--strings \"010110;100101;001011\" --alphabet-size 2"
        }
        "GroupingBySwapping" => "--string \"0,1,2,0,1,2\" --bound 5",
        "MinimumExternalMacroDataCompression" | "MinimumInternalMacroDataCompression" => {
            "--string \"0,1,0,1\" --pointer-cost 2 --alphabet-size 2"
        }
        "MinimumCardinalityKey" => {
            "--num-attributes 6 --dependencies \"0,1>2;0,2>3;1,3>4;2,4>5\""
        }
        "PrimeAttributeName" => {
            "--universe 6 --dependencies \"0,1>2,3,4,5;2,3>0,1,4,5\" --query-attribute 3"
        }
        "TwoDimensionalConsecutiveSets" => {
            "--alphabet-size 6 --subsets \"0,1,2;3,4,5;1,3;2,4;0,5\""
        }
        "ShortestCommonSupersequence" => "--strings \"0,1,2;1,2,0\"",
        "ConsecutiveBlockMinimization" => "--matrix '[[true,false,true],[false,true,true]]' --bound-k 2",
        "ConsecutiveOnesMatrixAugmentation" => {
            "--matrix \"1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0\" --bound 2"
        }
        "SparseMatrixCompression" => "--matrix \"1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0\" --bound-k 2",
        "MaximumLikelihoodRanking" => "--matrix \"0,4,3,5;1,0,4,3;2,1,0,4;0,2,1,0\"",
        "MinimumMatrixCover" => "--matrix \"0,3,1,0;3,0,0,2;1,0,0,4;0,2,4,0\"",
        "MinimumMatrixDomination" => "--matrix \"0,1,0;1,0,1;0,1,0\"",
        "MinimumWeightDecoding" => {
            "--matrix '[[true,false,true,true],[false,true,true,false],[true,true,false,true]]' --rhs 'true,true,false'"
        }
        "MinimumWeightSolutionToLinearEquations" => {
            "--matrix '[[1,2,3,1],[2,1,1,3]]' --rhs '5,4'"
        }
        "ConjunctiveBooleanQuery" => {
            "--domain-size 6 --relations \"2:0,3|1,3|2,4;3:0,1,5|1,2,5\" --conjuncts-spec \"0:v0,c3;0:v1,c3;1:v0,v1,c5\""
        }
        "ConjunctiveQueryFoldability" => "(use --example ConjunctiveQueryFoldability)",
        "EquilibriumPoint" => "(use --example EquilibriumPoint)",
        "SequencingToMinimizeMaximumCumulativeCost" => {
            "--costs 2,-1,3,-2,1,-3 --precedence-pairs \"0>2,1>2,1>3,2>4,3>5,4>5\""
        }
        "StringToStringCorrection" => {
            "--source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2"
        }
        "FeasibleBasisExtension" => {
            "--matrix '[[1,0,1,2,-1,0],[0,1,0,1,1,2],[0,0,1,1,0,1]]' --rhs '7,5,3' --required-columns '0,1'"
        }
        "MinimumCodeGenerationParallelAssignments" => {
            "--num-variables 4 --assignments \"0:1,2;1:0;2:3;3:1,2\""
        }
        "MinimumDecisionTree" => {
            "--test-matrix '[[true,true,false,false],[true,false,false,false],[false,true,false,true]]' --num-objects 4 --num-tests 3"
        }
        "MinimumDisjunctiveNormalForm" => {
            "--num-vars 3 --truth-table 0,1,1,1,1,1,1,0"
        }
        "SquareTiling" => {
            "--num-colors 3 --tiles \"0,1,2,0;0,0,2,1;2,1,0,0;2,0,0,1\" --grid-size 2"
        }
        _ => "",
    }
}

pub(super) fn uses_edge_weights_flag(canonical: &str) -> bool {
    matches!(
        canonical,
        "BottleneckTravelingSalesman"
            | "BoundedDiameterSpanningTree"
            | "KthBestSpanningTree"
            | "LongestCircuit"
            | "MaxCut"
            | "MaximumMatching"
            | "MixedChinesePostman"
            | "RuralPostman"
            | "TravelingSalesman"
    )
}

pub(super) fn uses_edge_weights_flag_for_edge_lengths(canonical: &str) -> bool {
    matches!(
        canonical,
        "LongestCircuit" | "MinMaxMulticenter" | "MinimumSumMulticenter"
    )
}

pub(super) fn help_flag_name(canonical: &str, field_name: &str) -> String {
    // Problem-specific overrides first
    match (canonical, field_name) {
        ("BoundedComponentSpanningForest", "max_components") => return "k".to_string(),
        ("BoundedComponentSpanningForest", "max_weight") => return "max-weight".to_string(),
        ("BoyceCoddNormalFormViolation", "num_attributes") => return "n".to_string(),
        ("BoyceCoddNormalFormViolation", "functional_deps") => return "sets".to_string(),
        ("BoyceCoddNormalFormViolation", "target_subset") => return "target".to_string(),
        ("CapacityAssignment", "cost") => return "cost-matrix".to_string(),
        ("CapacityAssignment", "delay") => return "delay-matrix".to_string(),
        ("FlowShopScheduling", "num_processors")
        | ("JobShopScheduling", "num_processors")
        | ("OpenShopScheduling", "num_machines")
        | ("SchedulingWithIndividualDeadlines", "num_processors") => {
            return "num-processors/--m".to_string();
        }
        ("JobShopScheduling", "jobs") => return "jobs".to_string(),
        ("LengthBoundedDisjointPaths", "max_length") => return "max-length".to_string(),
        ("ConsecutiveBlockMinimization", "bound") => return "bound-k".to_string(),
        ("GroupingBySwapping", "budget") => return "bound".to_string(),
        ("RectilinearPictureCompression", "bound") => return "bound".to_string(),
        ("PrimeAttributeName", "num_attributes") => return "universe".to_string(),
        ("PrimeAttributeName", "dependencies") => return "dependencies".to_string(),
        ("PrimeAttributeName", "query_attribute") => return "query-attribute".to_string(),
        ("ClosestVectorProblem", "target") => return "target-vec".to_string(),
        ("ConjunctiveBooleanQuery", "conjuncts") => return "conjuncts-spec".to_string(),
        ("MixedChinesePostman", "arc_weights") => return "arc-weights".to_string(),
        ("ConsecutiveOnesMatrixAugmentation", "bound") => return "bound".to_string(),
        ("ConsecutiveOnesSubmatrix", "bound") => return "bound".to_string(),
        ("SparseMatrixCompression", "bound_k") => return "bound-k".to_string(),
        ("MinimumCodeGenerationParallelAssignments", "num_variables") => {
            return "num-variables".to_string();
        }
        ("MinimumCodeGenerationParallelAssignments", "assignments") => {
            return "assignments".to_string();
        }
        ("StackerCrane", "edges") => return "graph".to_string(),
        ("StackerCrane", "arc_lengths") => return "arc-lengths".to_string(),
        ("StackerCrane", "edge_lengths") => return "edge-lengths".to_string(),
        ("StaffScheduling", "shifts_per_schedule") => return "k".to_string(),
        ("TimetableDesign", "num_tasks") => return "num-tasks".to_string(),
        _ => {}
    }
    // Edge-weight problems use --edge-weights instead of --weights
    if field_name == "weights" && uses_edge_weights_flag(canonical) {
        return "edge-weights".to_string();
    }
    if field_name == "edge_lengths" && uses_edge_weights_flag_for_edge_lengths(canonical) {
        return "edge-weights".to_string();
    }
    // General field-name overrides (previously in cli_flag_name)
    match field_name {
        "universe_size" => "universe-size".to_string(),
        "collection" | "subsets" | "sets" => "subsets".to_string(),
        "left_size" => "left".to_string(),
        "right_size" => "right".to_string(),
        "edges" => "biedges".to_string(),
        "vertex_weights" => "weights".to_string(),
        "potential_weights" => "potential-weights".to_string(),
        "num_tasks" => "num-tasks".to_string(),
        "precedences" => "precedences".to_string(),
        "threshold" => "threshold".to_string(),
        "lengths" => "lengths".to_string(),
        _ => field_name.replace('_', "-"),
    }
}

pub(super) fn reject_vertex_weights_for_edge_weight_problem(
    args: &CreateArgs,
    canonical: &str,
    graph_type: Option<&str>,
) -> Result<()> {
    if args.weights.is_some() && uses_edge_weights_flag(canonical) {
        bail!(
            "{canonical} uses --edge-weights, not --weights.\n\n\
             Usage: pred create {} {}",
            match graph_type {
                Some(g) => format!("{canonical}/{g}"),
                None => canonical.to_string(),
            },
            example_for(canonical, graph_type)
        );
    }
    Ok(())
}

pub(super) fn help_flag_hint(
    canonical: &str,
    field_name: &str,
    type_name: &str,
    graph_type: Option<&str>,
) -> &'static str {
    match (canonical, field_name) {
        ("BoundedComponentSpanningForest", "max_weight") => "integer",
        ("SequencingWithinIntervals", "release_times") => "comma-separated integers: 0,0,5",
        ("DynamicStorageAllocation", "release_times") => "comma-separated arrival times: 0,0,1,2,3",
        ("DynamicStorageAllocation", "deadlines") => "comma-separated departure times: 3,2,4,5,5",
        ("DynamicStorageAllocation", "sizes") => "comma-separated item sizes: 2,3,1,3,2",
        ("DynamicStorageAllocation", "capacity") => "memory size D: 6",
        ("DisjointConnectingPaths", "terminal_pairs") => "comma-separated pairs: 0-3,2-5",
        ("PrimeAttributeName", "dependencies") => {
            "semicolon-separated dependencies: \"0,1>2,3;2,3>0,1\""
        }
        ("LongestCommonSubsequence", "strings") => {
            "raw strings: \"ABAC;BACA\" or symbol lists: \"0,1,0;1,0,1\""
        }
        ("GroupingBySwapping", "string") => "symbol list: \"0,1,2,0,1,2\"",
        ("MinimumExternalMacroDataCompression", "string")
        | ("MinimumInternalMacroDataCompression", "string") => "symbol list: \"0,1,0,1\"",
        ("MinimumExternalMacroDataCompression", "pointer_cost")
        | ("MinimumInternalMacroDataCompression", "pointer_cost") => "positive integer: 2",
        ("MinimumAxiomSet", "num_sentences") => "total number of sentences: 8",
        ("MinimumAxiomSet", "true_sentences") => "comma-separated indices: 0,1,2,3,4,5,6,7",
        ("MinimumAxiomSet", "implications") => "semicolon-separated rules: \"0>2;0>3;1>4;2,4>6\"",
        ("ShortestCommonSupersequence", "strings") => "symbol lists: \"0,1,2;1,2,0\"",
        ("MultipleChoiceBranching", "partition") => "semicolon-separated groups: \"0,1;2,3\"",
        ("IntegralFlowHomologousArcs", "homologous_pairs") => {
            "semicolon-separated arc-index equalities: \"2=5;4=3\""
        }
        ("ConsistencyOfDatabaseFrequencyTables", "attribute_domains") => {
            "comma-separated domain sizes: 2,3,2"
        }
        ("ConsistencyOfDatabaseFrequencyTables", "frequency_tables") => {
            "semicolon-separated tables: \"0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1\""
        }
        ("ConsistencyOfDatabaseFrequencyTables", "known_values") => {
            "semicolon-separated triples: \"0,0,0;3,0,1;1,2,1\""
        }
        ("IntegralFlowBundles", "bundles") => "semicolon-separated groups: \"0,1;2,5;3,4\"",
        ("IntegralFlowBundles", "bundle_capacities") => "comma-separated capacities: 1,1,1",
        ("PathConstrainedNetworkFlow", "paths") => {
            "semicolon-separated arc-index paths: \"0,2,5,8;1,4,7,9\""
        }
        ("ConsecutiveBlockMinimization", "matrix") => {
            "JSON 2D bool array: '[[true,false,true],[false,true,true]]'"
        }
        ("ConsecutiveOnesMatrixAugmentation", "matrix") => {
            "semicolon-separated 0/1 rows: \"1,0;0,1\""
        }
        ("ConsecutiveOnesSubmatrix", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("SparseMatrixCompression", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("MaximumLikelihoodRanking", "matrix") => {
            "semicolon-separated i32 rows: \"0,4,3,5;1,0,4,3;2,1,0,4;0,2,1,0\""
        }
        ("MinimumMatrixCover", "matrix") => "semicolon-separated i64 rows: \"0,3,1;3,0,2;1,2,0\"",
        ("MinimumMatrixDomination", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("MinimumWeightDecoding", "matrix") => "JSON 2D bool array: '[[true,false],[false,true]]'",
        ("MinimumWeightDecoding", "target") => "comma-separated booleans: \"true,true,false\"",
        ("MinimumWeightSolutionToLinearEquations", "matrix") => {
            "JSON 2D integer array: '[[1,2,3],[4,5,6]]'"
        }
        ("MinimumWeightSolutionToLinearEquations", "rhs") => "comma-separated integers: \"5,4\"",
        ("FeasibleBasisExtension", "matrix") => "JSON 2D integer array: '[[1,0,1],[0,1,0]]'",
        ("FeasibleBasisExtension", "rhs") => "comma-separated integers: \"7,5,3\"",
        ("FeasibleBasisExtension", "required_columns") => "comma-separated column indices: \"0,1\"",
        ("MinimumCodeGenerationParallelAssignments", "assignments") => {
            "semicolon-separated target:reads entries: \"0:1,2;1:0;2:3;3:1,2\""
        }
        ("NonTautology", "disjuncts") => "semicolon-separated disjuncts: \"1,2,3;-1,-2,-3\"",
        ("TimetableDesign", "craftsman_avail") | ("TimetableDesign", "task_avail") => {
            "semicolon-separated 0/1 rows: \"1,1,0;0,1,1\""
        }
        ("TimetableDesign", "requirements") => "semicolon-separated rows: \"1,0,1;0,1,0\"",
        _ => type_format_hint(type_name, graph_type),
    }
}

pub(super) fn parse_nonnegative_usize_bound(
    bound: i64,
    problem_name: &str,
    usage: &str,
) -> Result<usize> {
    usize::try_from(bound)
        .map_err(|_| anyhow::anyhow!("{problem_name} requires nonnegative --bound\n\n{usage}"))
}

pub(super) fn validate_prescribed_paths_against_graph(
    graph: &DirectedGraph,
    paths: &[Vec<usize>],
    source: usize,
    sink: usize,
    usage: &str,
) -> Result<()> {
    let arcs = graph.arcs();
    for path in paths {
        anyhow::ensure!(
            !path.is_empty(),
            "PathConstrainedNetworkFlow paths must be non-empty\n\n{usage}"
        );
        let mut visited_vertices = BTreeSet::from([source]);
        let mut current = source;
        for &arc_index in path {
            let &(tail, head) = arcs.get(arc_index).ok_or_else(|| {
                anyhow::anyhow!(
                    "Path arc index {arc_index} out of bounds for {} arcs\n\n{usage}",
                    arcs.len()
                )
            })?;
            anyhow::ensure!(
                tail == current,
                "prescribed path is not contiguous: expected arc leaving vertex {current}, got {tail}->{head}\n\n{usage}"
            );
            anyhow::ensure!(
                visited_vertices.insert(head),
                "prescribed path repeats vertex {head}, so it is not a simple path\n\n{usage}"
            );
            current = head;
        }
        anyhow::ensure!(
            current == sink,
            "prescribed path must end at sink {sink}, ended at {current}\n\n{usage}"
        );
    }
    Ok(())
}

pub(super) fn resolve_processor_count_flags(
    problem_name: &str,
    usage: &str,
    num_processors: Option<usize>,
    m_alias: Option<usize>,
) -> Result<Option<usize>> {
    match (num_processors, m_alias) {
        (Some(num_processors), Some(m_alias)) => {
            anyhow::ensure!(
                num_processors == m_alias,
                "{problem_name} received conflicting processor counts: --num-processors={num_processors} but --m={m_alias}\n\n{usage}"
            );
            Ok(Some(num_processors))
        }
        (Some(num_processors), None) => Ok(Some(num_processors)),
        (None, Some(m_alias)) => Ok(Some(m_alias)),
        (None, None) => Ok(None),
    }
}

pub(super) fn validate_sequencing_within_intervals_inputs(
    release_times: &[u64],
    deadlines: &[u64],
    lengths: &[u64],
    usage: &str,
) -> Result<()> {
    if release_times.len() != deadlines.len() {
        bail!("release_times and deadlines must have the same length\n\n{usage}");
    }
    if release_times.len() != lengths.len() {
        bail!("release_times and lengths must have the same length\n\n{usage}");
    }

    for (i, ((&release_time, &deadline), &length)) in release_times
        .iter()
        .zip(deadlines.iter())
        .zip(lengths.iter())
        .enumerate()
    {
        let end = release_time.checked_add(length).ok_or_else(|| {
            anyhow::anyhow!("Task {i}: overflow computing r(i) + l(i)\n\n{usage}")
        })?;
        if end > deadline {
            bail!(
                "Task {i}: r({}) + l({}) > d({}), time window is empty\n\n{usage}",
                release_time,
                length,
                deadline
            );
        }
    }

    Ok(())
}

pub(super) fn print_problem_help(
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> Result<()> {
    let graph_type = resolved_variant
        .get("graph")
        .map(String::as_str)
        .filter(|graph_type| *graph_type != "SimpleGraph");
    let is_geometry = matches!(
        graph_type,
        Some("KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph")
    );
    let schemas = collect_schemas();
    let schema = schemas.iter().find(|s| s.name == canonical);

    if let Some(s) = schema {
        eprintln!("{}\n  {}\n", canonical, s.description);
        eprintln!("Parameters:");
        for field in &s.fields {
            let flag_name =
                problem_help_flag_name(canonical, &field.name, &field.type_name, is_geometry);
            // For geometry variants, show --positions instead of --graph
            if field.type_name == "G" && is_geometry {
                let hint = type_format_hint(&field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({hint})", flag_name, field.description);
                if graph_type == Some("UnitDiskGraph") {
                    eprintln!("  --{:<16} Distance threshold [default: 1.0]", "radius");
                }
            } else if field.type_name == "DirectedGraph" {
                // DirectedGraph fields use --arcs, not --graph
                let hint = type_format_hint(&field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({})", "arcs", field.description, hint);
            } else if field.type_name == "MixedGraph" {
                eprintln!(
                    "  --{:<16} Undirected edges E of the mixed graph (edge list: 0-1,1-2,2-3)",
                    "graph"
                );
                eprintln!(
                    "  --{:<16} Directed arcs A of the mixed graph (directed arcs: 0>1,1>2,2>0)",
                    "arcs"
                );
            } else if field.type_name == "BipartiteGraph" {
                eprintln!(
                    "  --{:<16} Vertices in the left partition (integer)",
                    "left"
                );
                eprintln!(
                    "  --{:<16} Vertices in the right partition (integer)",
                    "right"
                );
                eprintln!(
                    "  --{:<16} Bipartite edges as left-right pairs (edge list: 0-0,0-1,1-2)",
                    "biedges"
                );
            } else {
                let hint = help_flag_hint(canonical, &field.name, &field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({})", flag_name, field.description, hint);
            }
        }
        if canonical == "GraphPartitioning" {
            eprintln!(
                "  --{:<16} Number of partitions in the balanced partitioning model (must be 2) (integer)",
                "num-partitions"
            );
        }
    } else {
        bail!("{}", crate::problem_name::unknown_problem_error(canonical));
    }

    let example = schema_help_example_for(canonical, resolved_variant).or_else(|| {
        let fallback = example_for(canonical, graph_type);
        (!fallback.is_empty()).then(|| fallback.to_string())
    });
    if let Some(example) = example {
        eprintln!("\nExample:");
        eprintln!(
            "  pred create {} {}",
            match graph_type {
                Some(g) => format!("{canonical}/{g}"),
                None => canonical.to_string(),
            },
            example
        );
    }
    Ok(())
}

pub(super) fn schema_help_example_for(
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> Option<String> {
    let schema = collect_schemas()
        .into_iter()
        .find(|schema| schema.name == canonical)?;
    let example = problemreductions::example_db::find_model_example(&ProblemRef {
        name: canonical.to_string(),
        variant: resolved_variant.clone(),
    })
    .ok()?;
    let instance = example.instance.as_object()?;
    let graph_type = resolved_variant
        .get("graph")
        .map(String::as_str)
        .filter(|graph_type| *graph_type != "SimpleGraph");
    let is_geometry = matches!(
        graph_type,
        Some("KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph")
    );

    let mut args = Vec::new();
    for field in &schema.fields {
        let value = instance.get(&field.name)?;
        let concrete_type = resolve_schema_field_type(&field.type_name, resolved_variant);
        let flag_name =
            schema_example_flag_name(canonical, &field.name, &field.type_name, is_geometry);
        let rendered =
            format_schema_help_example_value(canonical, &field.name, &concrete_type, value)?;
        args.push(format!("--{flag_name} {}", quote_cli_arg(&rendered)));
    }
    Some(args.join(" "))
}

pub(super) fn schema_example_flag_name(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> String {
    problem_help_flag_name(canonical, field_name, field_type, is_geometry)
        .split('/')
        .next()
        .unwrap_or(field_name)
        .trim_start_matches("--")
        .to_string()
}

pub(super) fn quote_cli_arg(raw: &str) -> String {
    if raw.is_empty()
        || raw.chars().any(|ch| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    ';' | '>' | '|' | '[' | ']' | '{' | '}' | '(' | ')' | '"' | '\''
                )
        })
    {
        format!("\"{}\"", raw.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        raw.to_string()
    }
}

pub(super) fn format_schema_help_example_value(
    canonical: &str,
    field_name: &str,
    concrete_type: &str,
    value: &serde_json::Value,
) -> Option<String> {
    match (canonical, field_name) {
        ("ConsecutiveBlockMinimization", "matrix")
        | ("FeasibleBasisExtension", "matrix")
        | ("MinimumWeightDecoding", "matrix")
        | ("MinimumWeightSolutionToLinearEquations", "matrix") => {
            return serde_json::to_string(value).ok();
        }
        _ => {}
    }
    match normalize_type_name(concrete_type).as_str() {
        "SimpleGraph" => format_simple_graph_example(value),
        "DirectedGraph" => format_directed_graph_example(value),
        "Vec<CNFClause>" => format_cnf_clause_list_example(value),
        "Vec<Quantifier>" => format_quantifier_list_example(value),
        "Vec<Vec<(usize,u64)>>" => format_job_shop_example(value),
        "Vec<(Vec<usize>,Vec<usize>)>" => format_dependency_example(value),
        "Vec<usize>" | "Vec<u64>" | "Vec<i32>" | "Vec<i64>" | "Vec<f64>" | "Vec<BigUint>" => {
            format_scalar_array_example(value)
        }
        "Vec<bool>" => format_bool_array_example(value),
        "Vec<Vec<usize>>" | "Vec<Vec<u64>>" | "Vec<Vec<i32>>" | "Vec<Vec<i64>>"
        | "Vec<Vec<f64>>" => format_nested_numeric_rows(value),
        "Vec<Vec<bool>>" => format_bool_matrix_example(value),
        "Vec<String>" => Some(
            value
                .as_array()?
                .iter()
                .map(|entry| entry.as_str().map(str::to_string))
                .collect::<Option<Vec<_>>>()?
                .join(";"),
        ),
        "usize" | "u64" | "i32" | "i64" | "f64" | "BigUint" => format_scalar_example(value),
        _ => None,
    }
}

pub(super) fn format_scalar_example(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Number(number) => Some(number.to_string()),
        serde_json::Value::String(string) => Some(string.clone()),
        serde_json::Value::Bool(boolean) => Some(boolean.to_string()),
        _ => None,
    }
}

pub(super) fn format_scalar_array_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(format_scalar_example)
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

pub(super) fn format_bool_array_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|entry| {
                entry
                    .as_bool()
                    .map(|boolean| if boolean { "1" } else { "0" }.to_string())
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

pub(super) fn format_nested_numeric_rows(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(format_scalar_array_example)
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

pub(super) fn format_cnf_clause_list_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|clause| format_scalar_array_example(clause.get("literals")?))
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

pub(super) fn format_bool_matrix_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(format_bool_array_example)
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

pub(super) fn format_simple_graph_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .get("edges")?
            .as_array()?
            .iter()
            .map(|edge| {
                let pair = edge.as_array()?;
                Some(format!(
                    "{}-{}",
                    pair.first()?.as_u64()?,
                    pair.get(1)?.as_u64()?
                ))
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

pub(super) fn format_directed_graph_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .get("arcs")?
            .as_array()?
            .iter()
            .map(|arc| {
                let pair = arc.as_array()?;
                Some(format!(
                    "{}>{}",
                    pair.first()?.as_u64()?,
                    pair.get(1)?.as_u64()?
                ))
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

pub(super) fn format_quantifier_list_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|entry| match entry.as_str()? {
                "Exists" => Some("E".to_string()),
                "ForAll" => Some("A".to_string()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

pub(super) fn format_job_shop_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|job| {
                Some(
                    job.as_array()?
                        .iter()
                        .map(|task| {
                            let task = task.as_array()?;
                            Some(format!(
                                "{}:{}",
                                task.first()?.as_u64()?,
                                task.get(1)?.as_u64()?
                            ))
                        })
                        .collect::<Option<Vec<_>>>()?
                        .join(","),
                )
            })
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

pub(super) fn format_dependency_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|dependency| {
                let dependency = dependency.as_array()?;
                let lhs = format_scalar_array_example(dependency.first()?)?;
                let rhs = format_scalar_array_example(dependency.get(1)?)?;
                Some(format!("{lhs}>{rhs}"))
            })
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

pub(super) fn problem_help_flag_name(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> String {
    if field_type == "G" && is_geometry {
        return "positions".to_string();
    }
    if field_type == "DirectedGraph" {
        return "arcs".to_string();
    }
    if field_type == "MixedGraph" {
        return "graph".to_string();
    }
    if canonical == "LengthBoundedDisjointPaths" && field_name == "max_length" {
        return "max-length".to_string();
    }
    if canonical == "GeneralizedHex" && field_name == "target" {
        return "sink".to_string();
    }
    if canonical == "StringToStringCorrection" {
        return match field_name {
            "source" => "source-string".to_string(),
            "target" => "target-string".to_string(),
            "bound" => "bound".to_string(),
            _ => help_flag_name(canonical, field_name),
        };
    }
    help_flag_name(canonical, field_name)
}

pub(super) fn lbdp_validation_error(message: &str, usage: Option<&str>) -> anyhow::Error {
    match usage {
        Some(usage) => anyhow::anyhow!("{message}\n\n{usage}"),
        None => anyhow::anyhow!("{message}"),
    }
}

pub(super) fn validate_length_bounded_disjoint_paths_args(
    num_vertices: usize,
    source: usize,
    sink: usize,
    bound: i64,
    usage: Option<&str>,
) -> Result<usize> {
    let max_length = usize::try_from(bound).map_err(|_| {
        lbdp_validation_error(
            "--max-length must be a nonnegative integer for LengthBoundedDisjointPaths",
            usage,
        )
    })?;
    if source >= num_vertices || sink >= num_vertices {
        return Err(lbdp_validation_error(
            "--source and --sink must be valid graph vertices",
            usage,
        ));
    }
    if source == sink {
        return Err(lbdp_validation_error(
            "--source and --sink must be distinct",
            usage,
        ));
    }
    if max_length == 0 {
        return Err(lbdp_validation_error(
            "--max-length must be positive",
            usage,
        ));
    }
    Ok(max_length)
}
