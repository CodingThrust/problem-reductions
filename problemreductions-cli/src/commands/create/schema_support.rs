use super::*;

#[derive(Debug, Clone, Default)]
pub(super) struct CreateContext {
    num_vertices: Option<usize>,
    num_edges: Option<usize>,
    num_arcs: Option<usize>,
    parsed_fields: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputValueKind {
    Text,
    Usize,
    U64,
    I32,
    I64,
    F64,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateInput {
    pub name: String,
    pub kind: InputValueKind,
}

impl CreateContext {
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
) -> Result<(serde_json::Value, BTreeMap<String, String>)> {
    let schema = problemreductions::registry::find_problem_type(canonical)
        .ok_or_else(|| anyhow::anyhow!("No schema is registered for {canonical}"))?;
    let variant_entry =
        problemreductions::registry::find_variant_entry(canonical, resolved_variant).ok_or_else(
            || {
                anyhow::anyhow!(
                    "No concrete variant is registered for {canonical} with {resolved_variant:?}"
                )
            },
        )?;

    if let Some(inputs) = variant_entry.create_inputs {
        let data = normalize_registered_create_inputs(args, inputs, resolved_variant)
            .map_err(|error| with_registered_usage(error, canonical, inputs))?;
        return construct_canonical(canonical, resolved_variant, data)
            .map_err(|error| with_registered_usage(error, canonical, inputs));
    }

    let graph_type = resolved_graph_type(resolved_variant);
    let is_geometry = matches!(
        graph_type,
        "KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph"
    );
    let mut context = CreateContext::default();
    seed_schema_context_from_cli(args, graph_type, &mut context)?;
    let mut json_map = serde_json::Map::new();

    for field in schema.fields {
        let concrete_type = resolve_schema_field_type(field.type_name, resolved_variant);
        let flag_name = problem_help_flag_name(field.name, field.type_name, is_geometry);
        let raw_value = args.raw(&flag_name).ok_or_else(|| {
            with_schema_usage(
                missing_schema_field_error(canonical, field.name, field.type_name, is_geometry),
                canonical,
                resolved_variant,
            )
        })?;
        let value = parse_schema_field_value(&concrete_type, field.name, raw_value, &context)
            .map_err(|error| with_schema_usage(error, canonical, resolved_variant))?;

        context.remember(field.name, &concrete_type, &value);
        json_map.insert(field.name.to_string(), value);
    }

    let data = serde_json::Value::Object(json_map);
    construct_canonical(canonical, resolved_variant, data)
}

fn construct_canonical(
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
    data: serde_json::Value,
) -> Result<(serde_json::Value, BTreeMap<String, String>)> {
    let problem = problemreductions::registry::construct_dyn(canonical, resolved_variant, data)?;
    let constructed_variant = problem.variant_map();
    anyhow::ensure!(
        problem.problem_name() == canonical && constructed_variant == *resolved_variant,
        "registered constructor for {canonical} {resolved_variant:?} returned {} {constructed_variant:?}",
        problem.problem_name(),
    );
    Ok((problem.serialize_json(), constructed_variant))
}

fn normalize_registered_create_inputs(
    args: &CreateArgs,
    inputs: &[problemreductions::registry::CreateInputInfo],
    resolved_variant: &BTreeMap<String, String>,
) -> Result<serde_json::Value> {
    let mut values = serde_json::Map::new();
    for input in inputs {
        let flag_name = input.name.replace('_', "-");
        if let Some(raw) = args.raw(&flag_name) {
            let concrete_type = resolve_schema_field_type(input.type_name, resolved_variant);
            values.insert(
                input.name.to_string(),
                normalize_registered_input(input, &concrete_type, raw)?,
            );
        }
    }
    Ok(serde_json::Value::Object(values))
}

fn normalize_registered_input(
    input: &problemreductions::registry::CreateInputInfo,
    concrete_type: &str,
    raw: &str,
) -> Result<serde_json::Value> {
    use problemreductions::registry::CreateInputCodec;

    let value = match input.codec {
        CreateInputCodec::Json => serde_json::from_str(raw).map_err(|error| {
            anyhow::anyhow!(
                "Invalid JSON for --{}: {error}",
                input.name.replace('_', "-")
            )
        })?,
        CreateInputCodec::EdgeList | CreateInputCodec::BipartiteEdgeList => {
            serde_json::to_value(util::parse_edge_pairs(raw)?)?
        }
        CreateInputCodec::ArcList => serde_json::to_value(parse_registered_arcs(raw)?)?,
        CreateInputCodec::EqualityPairList => {
            serde_json::to_value(parse_registered_equality_pairs(raw)?)?
        }
        CreateInputCodec::FunctionalDependencyList => {
            serde_json::to_value(parse_registered_functional_dependencies(raw)?)?
        }
        CreateInputCodec::CharacterRows => {
            serde_json::to_value(parse_registered_character_rows(raw))?
        }
        CreateInputCodec::Auto
        | CreateInputCodec::Scalar
        | CreateInputCodec::CommaSeparated
        | CreateInputCodec::SemicolonSeparated => {
            parse_field_value(concrete_type, input.name, raw, &CreateContext::default())?
        }
    };
    Ok(value)
}

fn parse_registered_character_rows(raw: &str) -> Vec<Vec<usize>> {
    let mut alphabet = BTreeMap::new();
    raw.split(';')
        .map(|row| {
            row.chars()
                .map(|symbol| {
                    let next = alphabet.len();
                    *alphabet.entry(symbol).or_insert(next)
                })
                .collect()
        })
        .collect()
}

fn parse_registered_arcs(raw: &str) -> Result<Vec<(usize, usize)>> {
    raw.split(',')
        .map(|arc| {
            let (source, target) = arc.trim().split_once('>').ok_or_else(|| {
                anyhow::anyhow!("Invalid arc '{}': expected format u>v", arc.trim())
            })?;
            Ok((source.trim().parse()?, target.trim().parse()?))
        })
        .collect()
}

fn parse_registered_equality_pairs(raw: &str) -> Result<Vec<(usize, usize)>> {
    raw.split(';')
        .map(|pair| {
            let (left, right) = pair.trim().split_once('=').ok_or_else(|| {
                anyhow::anyhow!("Invalid pair '{}': expected format left=right", pair.trim())
            })?;
            Ok((left.trim().parse()?, right.trim().parse()?))
        })
        .collect()
}

fn parse_registered_functional_dependencies(raw: &str) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
    raw.split(';')
        .map(|dependency| {
            let (left, right) = dependency.trim().split_once(':').ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid functional dependency '{}': expected format lhs:rhs",
                    dependency.trim()
                )
            })?;
            Ok((
                util::parse_comma_list(left)?,
                util::parse_comma_list(right)?,
            ))
        })
        .collect()
}

pub(super) fn missing_schema_field_error(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> anyhow::Error {
    let flag = problem_help_flag_name(field_name, field_type, is_geometry);
    let requirement = format!("--{flag}");
    anyhow::anyhow!("{canonical} requires {requirement}")
}

pub(super) fn parse_schema_field_value(
    concrete_type: &str,
    field_name: &str,
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    parse_field_value(concrete_type, field_name, raw, context)
}

pub(crate) fn create_inputs_for(
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> Vec<CreateInput> {
    let variant_entry =
        problemreductions::registry::find_variant_entry(canonical, resolved_variant)
            .unwrap_or_else(|| {
                panic!("missing registered variant for `{canonical}` with {resolved_variant:?}")
            });
    let mut inputs = BTreeMap::<String, (InputValueKind, String)>::new();

    if let Some(custom_inputs) = variant_entry.create_inputs {
        for input in custom_inputs {
            let concrete_type = resolve_schema_field_type(input.type_name, resolved_variant);
            insert_create_input(
                &mut inputs,
                &input.name.replace('_', "-"),
                input_value_kind(&concrete_type),
                input.name,
            );
        }
    } else {
        let schema = problemreductions::registry::find_problem_type(canonical)
            .unwrap_or_else(|| panic!("missing schema for `{canonical}`"));
        let graph_type = resolved_graph_type(resolved_variant);
        let is_geometry = matches!(
            graph_type,
            "KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph"
        );
        for field in schema.fields {
            let concrete_type = resolve_schema_field_type(field.type_name, resolved_variant);
            match concrete_type.as_str() {
                "DirectedGraph" => {
                    insert_create_input(&mut inputs, "arcs", InputValueKind::Text, field.name);
                }
                _ => {
                    let name = problem_help_flag_name(field.name, field.type_name, is_geometry);
                    insert_create_input(
                        &mut inputs,
                        &name,
                        input_value_kind(&concrete_type),
                        field.name,
                    );
                }
            }
        }
        if schema.fields.iter().any(|field| {
            let concrete_type = resolve_schema_field_type(field.type_name, resolved_variant);
            matches!(concrete_type.as_str(), "SimpleGraph" | "DirectedGraph")
        }) {
            insert_create_input(
                &mut inputs,
                "num-vertices",
                InputValueKind::Usize,
                "graph vertex count",
            );
        }
        if graph_type == "UnitDiskGraph" {
            insert_create_input(
                &mut inputs,
                "radius",
                InputValueKind::F64,
                "unit-disk graph radius",
            );
        }
    }
    if super::supports_random(canonical) {
        for (name, kind) in [
            ("random", InputValueKind::Bool),
            ("num-vertices", InputValueKind::Usize),
            ("edge-prob", InputValueKind::F64),
            ("seed", InputValueKind::U64),
        ] {
            if !inputs.contains_key(name) {
                insert_create_input(&mut inputs, name, kind, "random generation");
            }
        }
    }

    inputs
        .into_iter()
        .map(|(name, (kind, _))| CreateInput { name, kind })
        .collect()
}

fn insert_create_input(
    inputs: &mut BTreeMap<String, (InputValueKind, String)>,
    name: &str,
    kind: InputValueKind,
    source: &str,
) {
    if let Some((existing_kind, existing_source)) = inputs.get(name) {
        assert_eq!(
            (*existing_kind, existing_source.as_str()),
            (kind, source),
            "create input --{name} is produced by both `{existing_source}` and `{source}`"
        );
        return;
    }
    inputs.insert(name.to_string(), (kind, source.to_string()));
}

fn input_value_kind(concrete_type: &str) -> InputValueKind {
    match normalize_type_name(concrete_type).as_str() {
        "usize" => InputValueKind::Usize,
        "u64" => InputValueKind::U64,
        "i32" => InputValueKind::I32,
        "i64" => InputValueKind::I64,
        "f64" => InputValueKind::F64,
        "bool" => InputValueKind::Bool,
        _ => InputValueKind::Text,
    }
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
        "One" | "i32" => "i64",
        "f64" => "f64",
        _ => "i32",
    }
}

pub(super) fn seed_schema_context_from_cli(
    args: &CreateArgs,
    graph_type: &str,
    context: &mut CreateContext,
) -> Result<()> {
    if let Some(num_vertices) = args.value::<usize>("num-vertices") {
        context.seed_field("num_vertices", num_vertices)?;
    }
    if graph_type == "UnitDiskGraph" {
        context.seed_field("radius", args.value::<f64>("radius").unwrap_or(1.0))?;
    }
    Ok(())
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
    let flags = create_inputs_for(canonical, resolved_variant)
        .into_iter()
        .map(|input| {
            if input.kind == InputValueKind::Bool {
                format!("[--{}]", input.name)
            } else {
                format!("--{} <VALUE>", input.name)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    anyhow::anyhow!("{message}\n\nUsage: pred create {canonical} {flags}",)
}

fn with_registered_usage(
    error: anyhow::Error,
    canonical: &str,
    inputs: &[problemreductions::registry::CreateInputInfo],
) -> anyhow::Error {
    let flags = inputs
        .iter()
        .map(|input| {
            let flag = format!("--{} <VALUE>", input.name.replace('_', "-"));
            if input.required {
                flag
            } else {
                format!("[{flag}]")
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    anyhow::anyhow!("{error}\n\nUsage: pred create {canonical} {flags}")
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
        "LabelledDigraph" => parse_labelled_digraph_value(raw, field_name)?,
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
        "Vec<(i32,i32)>" => serde_json::to_value(util::parse_positions::<i32>(raw, "0,0")?)?,
        "(f64,f64)" => parse_f64_pair_value(raw)?,
        "Vec<Vec<(usize,usize)>>" => parse_nested_pair_list_value(raw)?,
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

pub(super) fn parse_f64_pair_value(raw: &str) -> Result<serde_json::Value> {
    let parts: Vec<&str> = raw.split(',').collect();
    anyhow::ensure!(
        parts.len() == 2,
        "Invalid (f64,f64) pair '{}': expected format x,y (e.g., 2.0,1.0)",
        raw.trim()
    );
    let x: f64 = parts[0]
        .trim()
        .parse()
        .map_err(|err| anyhow::anyhow!("Invalid x in '{}': {err}", raw.trim()))?;
    let y: f64 = parts[1]
        .trim()
        .parse()
        .map_err(|err| anyhow::anyhow!("Invalid y in '{}': {err}", raw.trim()))?;
    Ok(serde_json::to_value((x, y))?)
}

pub(super) fn parse_nested_pair_list_value(raw: &str) -> Result<serde_json::Value> {
    let groups: Vec<Vec<(usize, usize)>> = raw
        .split('|')
        .map(|group| {
            let trimmed = group.trim();
            if trimmed.is_empty() {
                return Ok(Vec::new());
            }
            trimmed
                .split(',')
                .map(|entry| {
                    let entry = entry.trim();
                    let parts: Vec<&str> = entry.split('-').collect();
                    anyhow::ensure!(
                        parts.len() == 2,
                        "Invalid pair '{entry}': expected i-j (e.g., 0-1)"
                    );
                    Ok((
                        parts[0].trim().parse::<usize>()?,
                        parts[1].trim().parse::<usize>()?,
                    ))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(groups)?)
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

pub(super) fn parse_labelled_digraph_value(
    raw: &str,
    field_name: &str,
) -> Result<serde_json::Value> {
    let trimmed = raw.trim();
    let flag = format!("--{}", field_name.replace('_', "-"));
    let (header, arcs_raw) = trimmed.split_once(':').ok_or_else(|| {
        anyhow::anyhow!(
            "{flag} must be \"<num_vertices>:<src>-<label>-<dst>,...\" (e.g., \"5:0-0-1,1-1-2\")"
        )
    })?;
    let num_vertices: usize = header.trim().parse().map_err(|err| {
        anyhow::anyhow!("{flag}: invalid num_vertices '{}': {err}", header.trim())
    })?;
    let arcs_raw = arcs_raw.trim();
    let mut arcs: Vec<LabelledArc> = Vec::new();
    if !arcs_raw.is_empty() {
        for entry in arcs_raw.split(',') {
            let entry = entry.trim();
            let parts: Vec<&str> = entry.split('-').collect();
            anyhow::ensure!(
                parts.len() == 3,
                "{flag}: invalid arc '{entry}': expected <src>-<label>-<dst>"
            );
            let src: usize = parts[0].trim().parse().map_err(|err| {
                anyhow::anyhow!("{flag}: invalid arc source '{}': {err}", parts[0].trim())
            })?;
            let label: u32 = parts[1].trim().parse().map_err(|err| {
                anyhow::anyhow!("{flag}: invalid arc label '{}': {err}", parts[1].trim())
            })?;
            let dst: usize = parts[2].trim().parse().map_err(|err| {
                anyhow::anyhow!(
                    "{flag}: invalid arc destination '{}': {err}",
                    parts[2].trim()
                )
            })?;
            anyhow::ensure!(
                src < num_vertices,
                "{flag}: arc source {src} out of range for num_vertices = {num_vertices}"
            );
            anyhow::ensure!(
                dst < num_vertices,
                "{flag}: arc destination {dst} out of range for num_vertices = {num_vertices}"
            );
            arcs.push(LabelledArc::new(src, label, dst));
        }
    }
    let graph = LabelledDigraph::new(num_vertices, arcs);
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

pub(super) fn help_flag_name(field_name: &str) -> String {
    field_name.replace("_", "-")
}

pub(super) fn parse_nonnegative_usize_bound(
    bound: i64,
    problem_name: &str,
    usage: &str,
) -> Result<usize> {
    usize::try_from(bound)
        .map_err(|_| anyhow::anyhow!("{problem_name} requires nonnegative --bound\n\n{usage}"))
}

pub(super) fn problem_help_flag_name(
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> String {
    if field_type == "G" && is_geometry {
        "positions".to_string()
    } else if field_type == "DirectedGraph" {
        "arcs".to_string()
    } else {
        help_flag_name(field_name)
    }
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
