use super::schema_support::*;
use super::*;

pub(super) fn validate_schema_driven_semantics(
    args: &CreateArgs,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
    _data: &serde_json::Value,
) -> Result<()> {
    match canonical {
        "BalancedCompleteBipartiteSubgraph" => {
            let usage = "pred create BalancedCompleteBipartiteSubgraph --left 4 --right 4 --biedges 0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3 --k 3";
            let _ = parse_bipartite_problem_input(
                args,
                "BalancedCompleteBipartiteSubgraph",
                "balanced biclique size",
                usage,
            )?;
        }
        "BiconnectivityAugmentation" => {
            let usage = "Usage: pred create BiconnectivityAugmentation --graph 0-1,1-2,2-3 --potential-weights 0-2:3,0-3:4,1-3:2 --budget 5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let potential_edges = parse_potential_edges(args)?;
            validate_potential_edges(&graph, &potential_edges)?;
            let _ = parse_budget(args)?;
        }
        "BoundedComponentSpanningForest" => {
            let usage = "Usage: pred create BoundedComponentSpanningForest --graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --max-weight 6";
            let (_, n) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            args.weights.as_deref().ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --weights\n\n{usage}")
            })?;
            let weights = parse_vertex_weights(args, n)?;
            if weights.iter().any(|&weight| weight < 0) {
                bail!("BoundedComponentSpanningForest requires nonnegative --weights\n\n{usage}");
            }
            let max_components = args.k.ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --k\n\n{usage}")
            })?;
            if max_components == 0 {
                bail!("BoundedComponentSpanningForest requires --k >= 1\n\n{usage}");
            }
            let bound_raw = args.bound.ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --max-weight\n\n{usage}")
            })?;
            if bound_raw <= 0 {
                bail!("BoundedComponentSpanningForest requires positive --max-weight\n\n{usage}");
            }
            let _ = i32::try_from(bound_raw).map_err(|_| {
                anyhow::anyhow!(
                    "BoundedComponentSpanningForest requires --max-weight within i32 range\n\n{usage}"
                )
            })?;
        }
        "CapacityAssignment" => {
            let usage = "Usage: pred create CapacityAssignment --capacities 1,2,3 --cost-matrix \"1,3,6;2,4,7;1,2,5\" --delay-matrix \"8,4,1;7,3,1;6,3,1\" --delay-budget 12";
            let capacities_str = args.capacities.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "CapacityAssignment requires --capacities, --cost-matrix, --delay-matrix, and --delay-budget\n\n{usage}"
                )
            })?;
            let cost_matrix_str = args.cost_matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!("CapacityAssignment requires --cost-matrix\n\n{usage}")
            })?;
            let delay_matrix_str = args.delay_matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!("CapacityAssignment requires --delay-matrix\n\n{usage}")
            })?;
            let _ = args.delay_budget.ok_or_else(|| {
                anyhow::anyhow!("CapacityAssignment requires --delay-budget\n\n{usage}")
            })?;

            let capacities: Vec<u64> = util::parse_comma_list(capacities_str)?;
            anyhow::ensure!(
                !capacities.is_empty(),
                "CapacityAssignment requires at least one capacity value\n\n{usage}"
            );
            anyhow::ensure!(
                capacities.iter().all(|&capacity| capacity > 0),
                "CapacityAssignment capacities must be positive\n\n{usage}"
            );
            anyhow::ensure!(
                capacities.windows(2).all(|w| w[0] < w[1]),
                "CapacityAssignment capacities must be strictly increasing\n\n{usage}"
            );

            let cost = parse_u64_matrix_rows(cost_matrix_str, "cost")?;
            let delay = parse_u64_matrix_rows(delay_matrix_str, "delay")?;
            anyhow::ensure!(
                cost.len() == delay.len(),
                "cost matrix row count ({}) must match delay matrix row count ({})\n\n{usage}",
                cost.len(),
                delay.len()
            );

            for (index, row) in cost.iter().enumerate() {
                anyhow::ensure!(
                    row.len() == capacities.len(),
                    "cost row {} length ({}) must match capacities length ({})\n\n{usage}",
                    index,
                    row.len(),
                    capacities.len()
                );
                anyhow::ensure!(
                    row.windows(2).all(|w| w[0] <= w[1]),
                    "cost row {} must be non-decreasing\n\n{usage}",
                    index
                );
            }
            for (index, row) in delay.iter().enumerate() {
                anyhow::ensure!(
                    row.len() == capacities.len(),
                    "delay row {} length ({}) must match capacities length ({})\n\n{usage}",
                    index,
                    row.len(),
                    capacities.len()
                );
                anyhow::ensure!(
                    row.windows(2).all(|w| w[0] >= w[1]),
                    "delay row {} must be non-increasing\n\n{usage}",
                    index
                );
            }
        }
        "BoyceCoddNormalFormViolation" => {
            let n = args.n.ok_or_else(|| {
                anyhow::anyhow!(
                    "BoyceCoddNormalFormViolation requires --n, --sets, and --target\n\n\
                     Usage: pred create BoyceCoddNormalFormViolation --n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
                )
            })?;
            let sets_str = args.sets.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "BoyceCoddNormalFormViolation requires --sets (functional deps as lhs:rhs;...)\n\n\
                     Usage: pred create BoyceCoddNormalFormViolation --n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
                )
            })?;
            let target_str = args.target.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "BoyceCoddNormalFormViolation requires --target (comma-separated attribute indices)\n\n\
                     Usage: pred create BoyceCoddNormalFormViolation --n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
                )
            })?;
            let _ = parse_bcnf_functional_deps(sets_str, n)?;
            let target: Vec<usize> = util::parse_comma_list(target_str)?;
            ensure_attribute_indices_in_range(&target, n, "Target subset")?;
        }
        "ClosestVectorProblem" => {
            let basis_str = args.basis.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "CVP requires --basis, --target-vec\n\n\
                     Usage: pred create CVP --basis \"1,0;0,1\" --target-vec \"0.5,0.5\""
                )
            })?;
            let target_str = args
                .target_vec
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("CVP requires --target-vec (e.g., \"0.5,0.5\")"))?;
            let basis: Vec<Vec<f64>> = basis_str
                .split(';')
                .map(|row| util::parse_comma_list(row.trim()))
                .collect::<Result<Vec<_>>>()?;
            let target: Vec<f64> = util::parse_comma_list(target_str)?;
            let n = basis.len();
            let bounds = serde_json::from_value(parse_cvp_bounds_value(
                args.bounds.as_deref(),
                &CreateContext::default()
                    .with_field("basis", serde_json::json!(vec![serde_json::json!([0]); n])),
            )?)?;
            let _ = ClosestVectorProblem::new(basis, target, bounds);
        }
        "ConsecutiveOnesMatrixAugmentation" => {
            let matrix = parse_bool_matrix(args)?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveOnesMatrixAugmentation requires --matrix and --bound\n\n\
                     Usage: pred create ConsecutiveOnesMatrixAugmentation --matrix \"1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0\" --bound 2"
                )
            })?;
            ConsecutiveOnesMatrixAugmentation::try_new(matrix, bound)
                .map_err(anyhow::Error::msg)?;
        }
        "ConsecutiveBlockMinimization" => {
            let usage = "Usage: pred create ConsecutiveBlockMinimization --matrix '[[true,false,true],[false,true,true]]' --bound-k 2";
            let matrix_str = args.matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array and --bound-k\n\n{usage}"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("ConsecutiveBlockMinimization requires --bound-k\n\n{usage}")
            })?;
            let matrix: Vec<Vec<bool>> = serde_json::from_str(matrix_str).map_err(|err| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array (e.g., '[[true,false,true],[false,true,true]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            ConsecutiveBlockMinimization::try_new(matrix, bound)
                .map_err(|err| anyhow::anyhow!("{err}\n\n{usage}"))?;
        }
        "ComparativeContainment" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "ComparativeContainment requires --universe, --r-sets, and --s-sets\n\n\
                     Usage: pred create ComparativeContainment --universe 4 --r-sets \"0,1,2,3;0,1\" --s-sets \"0,1,2,3;2,3\" [--r-weights 2,5] [--s-weights 3,6]"
                )
            })?;
            let r_sets = parse_named_sets(args.r_sets.as_deref(), "--r-sets")?;
            let s_sets = parse_named_sets(args.s_sets.as_deref(), "--s-sets")?;
            validate_comparative_containment_sets("R", "--r-sets", universe, &r_sets)?;
            validate_comparative_containment_sets("S", "--s-sets", universe, &s_sets)?;
            match resolved_variant.get("weight").map(|value| value.as_str()) {
                Some("One") => {
                    let r_weights = parse_named_set_weights(
                        args.r_weights.as_deref(),
                        r_sets.len(),
                        "--r-weights",
                    )?;
                    let s_weights = parse_named_set_weights(
                        args.s_weights.as_deref(),
                        s_sets.len(),
                        "--s-weights",
                    )?;
                    anyhow::ensure!(
                        r_weights.iter().all(|&w| w == 1) && s_weights.iter().all(|&w| w == 1),
                        "Non-unit weights are not supported for ComparativeContainment/One.\n\n\
                         Use `pred create ComparativeContainment/i32 ... --r-weights ... --s-weights ...` for weighted instances."
                    );
                }
                Some("f64") => {
                    let r_weights = parse_named_set_weights_f64(
                        args.r_weights.as_deref(),
                        r_sets.len(),
                        "--r-weights",
                    )?;
                    validate_comparative_containment_f64_weights("R", "--r-weights", &r_weights)?;
                    let s_weights = parse_named_set_weights_f64(
                        args.s_weights.as_deref(),
                        s_sets.len(),
                        "--s-weights",
                    )?;
                    validate_comparative_containment_f64_weights("S", "--s-weights", &s_weights)?;
                }
                Some("i32") | None => {
                    let r_weights = parse_named_set_weights(
                        args.r_weights.as_deref(),
                        r_sets.len(),
                        "--r-weights",
                    )?;
                    validate_comparative_containment_i32_weights("R", "--r-weights", &r_weights)?;
                    let s_weights = parse_named_set_weights(
                        args.s_weights.as_deref(),
                        s_sets.len(),
                        "--s-weights",
                    )?;
                    validate_comparative_containment_i32_weights("S", "--s-weights", &s_weights)?;
                }
                Some(other) => bail!(
                    "Unsupported ComparativeContainment weight variant: {}",
                    other
                ),
            }
        }
        "DisjointConnectingPaths" => {
            let usage =
                "Usage: pred create DisjointConnectingPaths --graph 0-1,1-3,0-2,1-4,2-4,3-5,4-5 --terminal-pairs 0-3,2-5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_terminal_pairs(args, graph.num_vertices())
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
        }
        "ExactCoverBy3Sets" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "ExactCoverBy3Sets requires --universe and --sets\n\n\
                     Usage: pred create X3C --universe 6 --sets \"0,1,2;3,4,5\""
                )
            })?;
            if universe % 3 != 0 {
                bail!("Universe size must be divisible by 3, got {}", universe);
            }
            let sets = parse_sets(args)?;
            for (i, set) in sets.iter().enumerate() {
                if set.len() != 3 {
                    bail!(
                        "Subset {} has {} elements, but X3C requires exactly 3 elements per subset",
                        i,
                        set.len()
                    );
                }
                if set[0] == set[1] || set[0] == set[2] || set[1] == set[2] {
                    bail!("Subset {} contains duplicate elements: {:?}", i, set);
                }
                for &elem in set {
                    if elem >= universe {
                        bail!(
                            "Subset {} contains element {} which is outside universe of size {}",
                            i,
                            elem,
                            universe
                        );
                    }
                }
            }
        }
        "GeneralizedHex" => {
            let usage =
                "Usage: pred create GeneralizedHex --graph 0-1,0-2,0-3,1-4,2-4,3-4,4-5 --source 0 --sink 5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let num_vertices = graph.num_vertices();
            let source = args
                .source
                .ok_or_else(|| anyhow::anyhow!("GeneralizedHex requires --source\n\n{usage}"))?;
            let sink = args
                .sink
                .ok_or_else(|| anyhow::anyhow!("GeneralizedHex requires --sink\n\n{usage}"))?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            anyhow::ensure!(
                source != sink,
                "GeneralizedHex requires distinct --source and --sink\n\n{usage}"
            );
        }
        "GroupingBySwapping" => {
            let usage =
                "Usage: pred create GroupingBySwapping --string \"0,1,2,0,1,2\" --bound 5 [--alphabet-size 3]";
            let string_str = args.string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("GroupingBySwapping requires --string\n\n{usage}")
            })?;
            let bound = parse_nonnegative_usize_bound(
                args.bound.ok_or_else(|| {
                    anyhow::anyhow!("GroupingBySwapping requires --bound\n\n{usage}")
                })?,
                "GroupingBySwapping",
                usage,
            )?;
            let string = parse_symbol_list_allow_empty(string_str)?;
            let inferred = string.iter().copied().max().map_or(0, |value| value + 1);
            let alphabet_size = args.alphabet_size.unwrap_or(inferred);
            anyhow::ensure!(
                alphabet_size >= inferred,
                "--alphabet-size {} is smaller than max symbol + 1 ({}) in the input string",
                alphabet_size,
                inferred
            );
            anyhow::ensure!(
                alphabet_size > 0 || string.is_empty(),
                "GroupingBySwapping requires a positive alphabet for non-empty strings.\n\n{usage}"
            );
            anyhow::ensure!(
                !string.is_empty() || bound == 0,
                "GroupingBySwapping requires --bound 0 when --string is empty.\n\n{usage}"
            );
        }
        "IntegralFlowBundles" => {
            let usage = "Usage: pred create IntegralFlowBundles --arcs \"0>1,0>2,1>3,2>3,1>2,2>1\" --bundles \"0,1;2,5;3,4\" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --arcs\n\n{usage}"))?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let bundles = parse_bundles(args, num_arcs, usage)?;
            let _ = parse_bundle_capacities(args, bundles.len(), usage)?;
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowBundles requires --source\n\n{usage}")
            })?;
            let sink = args
                .sink
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --sink\n\n{usage}"))?;
            let _ = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowBundles requires --requirement\n\n{usage}")
            })?;
            validate_vertex_index("source", source, graph.num_vertices(), usage)?;
            validate_vertex_index("sink", sink, graph.num_vertices(), usage)?;
            anyhow::ensure!(
                source != sink,
                "IntegralFlowBundles requires distinct --source and --sink\n\n{usage}"
            );
        }
        "IntegralFlowHomologousArcs" => {
            let usage = "Usage: pred create IntegralFlowHomologousArcs --arcs \"0>1,0>2,1>3,2>3,1>4,2>4,3>5,4>5\" --capacities 1,1,1,1,1,1,1,1 --source 0 --sink 5 --requirement 2 --homologous-pairs \"2=5;4=3\"";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --arcs\n\n{usage}")
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities: Vec<u64> = if let Some(ref s) = args.capacities {
                s.split(',')
                    .map(|token| {
                        let trimmed = token.trim();
                        trimmed
                            .parse::<u64>()
                            .with_context(|| format!("Invalid capacity `{trimmed}`\n\n{usage}"))
                    })
                    .collect::<Result<Vec<_>>>()?
            } else {
                vec![1; num_arcs]
            };
            anyhow::ensure!(
                capacities.len() == num_arcs,
                "Expected {} capacities but got {}\n\n{}",
                num_arcs,
                capacities.len(),
                usage
            );
            for (arc_index, &capacity) in capacities.iter().enumerate() {
                let fits = usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some();
                anyhow::ensure!(
                    fits,
                    "capacity {} at arc index {} is too large for this platform\n\n{}",
                    capacity,
                    arc_index,
                    usage
                );
            }
            let num_vertices = graph.num_vertices();
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --sink\n\n{usage}")
            })?;
            let _ = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --requirement\n\n{usage}")
            })?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            let homologous_pairs =
                parse_homologous_pairs(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            for &(a, b) in &homologous_pairs {
                anyhow::ensure!(
                    a < num_arcs && b < num_arcs,
                    "homologous pair ({}, {}) references arc >= num_arcs ({})\n\n{}",
                    a,
                    b,
                    num_arcs,
                    usage
                );
            }
        }
        "IntegralFlowWithMultipliers" => {
            let usage = "Usage: pred create IntegralFlowWithMultipliers --arcs \"0>1,0>2,1>3,2>3\" --capacities 1,1,2,2 --source 0 --sink 3 --multipliers 1,2,3,1 --requirement 2";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --arcs\n\n{usage}")
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities_str = args.capacities.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --capacities\n\n{usage}")
            })?;
            let capacities: Vec<u64> = util::parse_comma_list(capacities_str)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            if capacities.len() != num_arcs {
                bail!(
                    "Expected {} capacities but got {}\n\n{}",
                    num_arcs,
                    capacities.len(),
                    usage
                );
            }
            for (arc_index, &capacity) in capacities.iter().enumerate() {
                let fits = usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some();
                if !fits {
                    bail!(
                        "capacity {} at arc index {} is too large for this platform\n\n{}",
                        capacity,
                        arc_index,
                        usage
                    );
                }
            }
            let num_vertices = graph.num_vertices();
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --sink\n\n{usage}")
            })?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            if source == sink {
                bail!(
                    "IntegralFlowWithMultipliers requires distinct --source and --sink\n\n{}",
                    usage
                );
            }
            let multipliers_str = args.multipliers.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --multipliers\n\n{usage}")
            })?;
            let multipliers: Vec<u64> = util::parse_comma_list(multipliers_str)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            if multipliers.len() != num_vertices {
                bail!(
                    "Expected {} multipliers but got {}\n\n{}",
                    num_vertices,
                    multipliers.len(),
                    usage
                );
            }
            if multipliers
                .iter()
                .enumerate()
                .any(|(vertex, &multiplier)| vertex != source && vertex != sink && multiplier == 0)
            {
                bail!("non-terminal multipliers must be positive\n\n{usage}");
            }
            let _ = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --requirement\n\n{usage}")
            })?;
        }
        "JobShopScheduling" => {
            let usage = "Usage: pred create JobShopScheduling --jobs \"0:3,1:4;1:2,0:3,1:2;0:4,1:3\" --num-processors 2";
            let job_tasks = args
                .job_tasks
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("JobShopScheduling requires --jobs\n\n{usage}"))?;
            let jobs = parse_job_shop_jobs(job_tasks)?;
            let inferred_processors = jobs
                .iter()
                .flat_map(|job| job.iter().map(|(processor, _)| *processor))
                .max()
                .map(|processor| processor + 1);
            let num_processors = resolve_processor_count_flags(
                "JobShopScheduling",
                usage,
                args.num_processors,
                args.m,
            )?
            .or(inferred_processors)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Cannot infer num_processors from empty job list; use --num-processors"
                )
            })?;
            anyhow::ensure!(
                num_processors > 0,
                "JobShopScheduling requires --num-processors > 0\n\n{usage}"
            );
            for (job_index, job) in jobs.iter().enumerate() {
                for (task_index, &(processor, _)) in job.iter().enumerate() {
                    anyhow::ensure!(
                        processor < num_processors,
                        "job {job_index} task {task_index} uses processor {processor}, but num_processors = {num_processors}"
                    );
                }
                for (task_index, pair) in job.windows(2).enumerate() {
                    anyhow::ensure!(
                        pair[0].0 != pair[1].0,
                        "job {job_index} tasks {task_index} and {} must use different processors\n\n{usage}",
                        task_index + 1
                    );
                }
            }
        }
        "KClique" => {
            let usage = "Usage: pred create KClique --graph 0-1,0-2,1-3,2-3,2-4,3-4 --k 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_kclique_threshold(args.k, graph.num_vertices(), usage)?;
        }
        "KColoring" => {
            let usage = "Usage: pred create KColoring --graph 0-1,1-2,2-0 --k 3";
            let _ = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = util::validate_k_param(resolved_variant, args.k, None, "KColoring")
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
        }
        "KthBestSpanningTree" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let usage =
                "Usage: pred create KthBestSpanningTree --graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_edge_weights(args, graph.num_edges())?;
            let _ = util::validate_k_param(resolved_variant, args.k, None, "KthBestSpanningTree")
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = args
                .bound
                .ok_or_else(|| anyhow::anyhow!("KthBestSpanningTree requires --bound\n\n{usage}"))?
                as i32;
        }
        "LengthBoundedDisjointPaths" => {
            let usage = "Usage: pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --max-length 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --sink\n\n{usage}")
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --max-length\n\n{usage}")
            })?;
            let _ = validate_length_bounded_disjoint_paths_args(
                graph.num_vertices(),
                source,
                sink,
                bound,
                Some(usage),
            )?;
        }
        "LongestCommonSubsequence" => {
            let usage =
                "Usage: pred create LCS --strings \"010110;100101;001011\" [--alphabet-size 2]";
            let strings_str = args.strings.as_deref().ok_or_else(|| {
                anyhow::anyhow!("LongestCommonSubsequence requires --strings\n\n{usage}")
            })?;
            let (strings, inferred_alphabet_size) = parse_lcs_strings(strings_str)?;
            let alphabet_size = args.alphabet_size.unwrap_or(inferred_alphabet_size);
            anyhow::ensure!(
                alphabet_size >= inferred_alphabet_size,
                "--alphabet-size {} is smaller than the inferred alphabet size ({})",
                alphabet_size,
                inferred_alphabet_size
            );
            anyhow::ensure!(
                strings.iter().any(|string| !string.is_empty()),
                "LongestCommonSubsequence requires at least one non-empty string.\n\n{usage}"
            );
            anyhow::ensure!(
                alphabet_size > 0,
                "LongestCommonSubsequence requires a positive alphabet. Provide --alphabet-size when all strings are empty.\n\n{usage}"
            );
        }
        "LongestPath" => {
            let usage = "pred create LongestPath --graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,4-6,5-6,1-6 --edge-lengths 3,2,4,1,5,2,3,2,4,1 --source-vertex 0 --target-vertex 6";
            let (graph, _) =
                parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\nUsage: {usage}"))?;
            if args.weights.is_some() {
                bail!("LongestPath uses --edge-lengths, not --weights\n\nUsage: {usage}");
            }
            let edge_lengths_raw = args.edge_lengths.as_ref().ok_or_else(|| {
                anyhow::anyhow!("LongestPath requires --edge-lengths\n\nUsage: {usage}")
            })?;
            let edge_lengths =
                parse_i32_edge_values(Some(edge_lengths_raw), graph.num_edges(), "edge length")?;
            ensure_positive_i32_values(&edge_lengths, "edge lengths")?;
            let source_vertex = args.source_vertex.ok_or_else(|| {
                anyhow::anyhow!("LongestPath requires --source-vertex\n\nUsage: {usage}")
            })?;
            let target_vertex = args.target_vertex.ok_or_else(|| {
                anyhow::anyhow!("LongestPath requires --target-vertex\n\nUsage: {usage}")
            })?;
            ensure_vertex_in_bounds(source_vertex, graph.num_vertices(), "source_vertex")?;
            ensure_vertex_in_bounds(target_vertex, graph.num_vertices(), "target_vertex")?;
        }
        "MixedChinesePostman" => {
            let usage = "Usage: pred create MixedChinesePostman --graph 0-2,1-3,0-4,4-2 --arcs \"0>1,1>2,2>3,3>0\" --edge-weights 2,3,1,2 --arc-weights 2,3,1,4 [--num-vertices N]";
            let graph = parse_mixed_graph(args, usage)?;
            let arc_costs = parse_arc_costs(args, graph.num_arcs())?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            if arc_costs.iter().any(|&cost| cost < 0) {
                bail!("MixedChinesePostman --arc-weights must be non-negative\n\n{usage}");
            }
            if edge_weights.iter().any(|&weight| weight < 0) {
                bail!("MixedChinesePostman --edge-weights must be non-negative\n\n{usage}");
            }
            if resolved_variant.get("weight").map(String::as_str) == Some("One")
                && (arc_costs.iter().any(|&cost| cost != 1)
                    || edge_weights.iter().any(|&weight| weight != 1))
            {
                bail!(
                    "Non-unit lengths are not supported for MixedChinesePostman/One.\n\n\
                     Use the weighted variant instead:\n  pred create MixedChinesePostman/i32 --graph ... --arcs ... --edge-weights ... --arc-weights ..."
                );
            }
        }
        "MinMaxMulticenter" => {
            let usage = "Usage: pred create MinMaxMulticenter --graph 0-1,1-2,2-3 [--weights 1,1,1,1] [--edge-weights 1,1,1] --k 2";
            let (graph, n) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let vertex_weights = parse_vertex_weights(args, n)?;
            let edge_lengths = parse_edge_weights(args, graph.num_edges())?;
            let _ = args.k.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinMaxMulticenter requires --k (number of centers)\n\n\
                     Usage: pred create MinMaxMulticenter --graph 0-1,1-2,2-3 --k 2"
                )
            })?;
            if vertex_weights.iter().any(|&weight| weight < 0) {
                bail!("MinMaxMulticenter --weights must be non-negative");
            }
            if edge_lengths.iter().any(|&length| length < 0) {
                bail!("MinMaxMulticenter --edge-weights must be non-negative");
            }
        }
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet"
        | "MaximalIS" => {
            let graph_type = resolved_graph_type(resolved_variant);
            let num_vertices = match graph_type {
                "KingsSubgraph" | "TriangularSubgraph" => parse_int_positions(args)?.len(),
                "UnitDiskGraph" => parse_float_positions(args)?.len(),
                _ => {
                    parse_graph(args)
                        .map_err(|e| {
                            anyhow::anyhow!(
                            "{e}\n\nUsage: pred create {} --graph 0-1,1-2,2-3 [--weights 1,1,1,1]",
                            canonical
                        )
                        })?
                        .1
                }
            };
            let weights = parse_vertex_weights(args, num_vertices)?;
            reject_nonunit_weights_for_one_variant(
                canonical,
                graph_type,
                resolved_variant,
                &weights,
            )?;
        }
        "MinimumHittingSet" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumHittingSet requires --universe and --sets\n\n\
                     Usage: pred create MinimumHittingSet --universe 6 --sets \"0,1,2;0,3,4;1,3,5;2,4,5;0,1,5;2,3;1,4\""
                )
            })?;
            let sets = parse_sets(args)?;
            for (i, set) in sets.iter().enumerate() {
                for &element in set {
                    if element >= universe {
                        bail!(
                            "Set {} contains element {} which is outside universe of size {}",
                            i,
                            element,
                            universe
                        );
                    }
                }
            }
        }
        "MinimumDummyActivitiesPert" => {
            let usage = "Usage: pred create MinimumDummyActivitiesPert --arcs \"0>2,0>3,1>3,1>4,2>5\" [--num-vertices N]";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("MinimumDummyActivitiesPert requires --arcs\n\n{usage}")
            })?;
            let (graph, _) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let _ = MinimumDummyActivitiesPert::try_new(graph).map_err(anyhow::Error::msg)?;
        }
        "MinimumMultiwayCut" => {
            let usage =
                "Usage: pred create MinimumMultiwayCut --graph 0-1,1-2,2-3 --terminals 0,2 [--edge-weights 1,1,1]";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_terminals(args, graph.num_vertices())?;
            let _ = parse_edge_weights(args, graph.num_edges())?;
        }
        "MultipleChoiceBranching" => {
            let usage = "Usage: pred create MultipleChoiceBranching/i32 --arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --threshold 10";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("MultipleChoiceBranching requires --arcs\n\n{usage}")
            })?;
            let (_, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let _ = parse_arc_weights(args, num_arcs)?;
            let _ = parse_partition_groups(args, num_arcs)?;
            let _ = parse_multiple_choice_branching_threshold(args, usage)?;
        }
        "MultipleCopyFileAllocation" => {
            let (_, num_vertices) = parse_graph(args)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{MULTIPLE_COPY_FILE_ALLOCATION_USAGE}"))?;
            let _ = parse_vertex_i64_values(
                args.usage.as_deref(),
                "usage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?;
            let _ = parse_vertex_i64_values(
                args.storage.as_deref(),
                "storage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?;
        }
        "MultiprocessorScheduling" => {
            let usage = "Usage: pred create MultiprocessorScheduling --lengths 4,5,3,2,6 --num-processors 2 --deadline 10";
            let lengths_str = args.lengths.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MultiprocessorScheduling requires --lengths, --num-processors, and --deadline\n\n{usage}"
                )
            })?;
            let num_processors = args.num_processors.ok_or_else(|| {
                anyhow::anyhow!("MultiprocessorScheduling requires --num-processors\n\n{usage}")
            })?;
            anyhow::ensure!(
                num_processors > 0,
                "MultiprocessorScheduling requires --num-processors > 0\n\n{usage}"
            );
            let _ = args.deadline.ok_or_else(|| {
                anyhow::anyhow!("MultiprocessorScheduling requires --deadline\n\n{usage}")
            })?;
            let _: Vec<u64> = util::parse_comma_list(lengths_str)?;
        }
        "PartialFeedbackEdgeSet" => {
            let usage = "Usage: pred create PartialFeedbackEdgeSet --graph 0-1,1-2,2-0,2-3,3-4,4-2,3-5,5-4,0-3 --budget 3 --max-cycle-length 4";
            let _ = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = args
                .budget
                .as_deref()
                .ok_or_else(|| {
                    anyhow::anyhow!("PartialFeedbackEdgeSet requires --budget\n\n{usage}")
                })?
                .parse::<usize>()
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Invalid --budget value for PartialFeedbackEdgeSet: {e}\n\n{usage}"
                    )
                })?;
            let _ = args.max_cycle_length.ok_or_else(|| {
                anyhow::anyhow!("PartialFeedbackEdgeSet requires --max-cycle-length\n\n{usage}")
            })?;
        }
        "PathConstrainedNetworkFlow" => {
            let usage = "Usage: pred create PathConstrainedNetworkFlow --arcs \"0>1,0>2,1>3,1>4,2>4,3>5,4>5,4>6,5>7,6>7\" --capacities 2,1,1,1,1,1,1,1,2,1 --source 0 --sink 7 --paths \"0,2,5,8;0,3,6,8;0,3,7,9;1,4,6,8;1,4,7,9\" --requirement 3";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --arcs\n\n{usage}")
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities: Vec<u64> = if let Some(ref s) = args.capacities {
                util::parse_comma_list(s)?
            } else {
                vec![1; num_arcs]
            };
            anyhow::ensure!(
                capacities.len() == num_arcs,
                "capacities length ({}) must match number of arcs ({num_arcs})",
                capacities.len()
            );
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --sink\n\n{usage}")
            })?;
            let _ = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --requirement\n\n{usage}")
            })?;
            let paths = parse_prescribed_paths(args, num_arcs, usage)?;
            validate_prescribed_paths_against_graph(&graph, &paths, source, sink, usage)?;
        }
        "ProductionPlanning" => {
            let usage = "Usage: pred create ProductionPlanning --num-periods 6 --demands 5,3,7,2,8,5 --capacities 12,12,12,12,12,12 --setup-costs 10,10,10,10,10,10 --production-costs 1,1,1,1,1,1 --inventory-costs 1,1,1,1,1,1 --cost-bound 80";
            let num_periods = args.num_periods.ok_or_else(|| {
                anyhow::anyhow!("ProductionPlanning requires --num-periods\n\n{usage}")
            })?;
            let demands = parse_named_u64_list(
                args.demands.as_deref(),
                "ProductionPlanning",
                "--demands",
                usage,
            )?;
            let capacities = parse_named_u64_list(
                args.capacities.as_deref(),
                "ProductionPlanning",
                "--capacities",
                usage,
            )?;
            let setup_costs = parse_named_u64_list(
                args.setup_costs.as_deref(),
                "ProductionPlanning",
                "--setup-costs",
                usage,
            )?;
            let production_costs = parse_named_u64_list(
                args.production_costs.as_deref(),
                "ProductionPlanning",
                "--production-costs",
                usage,
            )?;
            let inventory_costs = parse_named_u64_list(
                args.inventory_costs.as_deref(),
                "ProductionPlanning",
                "--inventory-costs",
                usage,
            )?;
            let _ = args.cost_bound.ok_or_else(|| {
                anyhow::anyhow!("ProductionPlanning requires --cost-bound\n\n{usage}")
            })?;

            for (flag, len) in [
                ("--demands", demands.len()),
                ("--capacities", capacities.len()),
                ("--setup-costs", setup_costs.len()),
                ("--production-costs", production_costs.len()),
                ("--inventory-costs", inventory_costs.len()),
            ] {
                ensure_named_len(len, num_periods, flag, usage)?;
            }
        }
        "SchedulingWithIndividualDeadlines" => {
            let usage = "Usage: pred create SchedulingWithIndividualDeadlines --num-tasks 7 --deadlines 2,1,2,2,3,3,2 [--num-processors 3 | --m 3] [--precedences \"0>3,1>3,1>4,2>4,2>5\"]";
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --deadlines, --num-tasks, and a processor count (--num-processors or --m)\n\n{usage}"
                )
            })?;
            let num_tasks = args.num_tasks.or(args.n).ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --num-tasks (number of tasks)\n\n{usage}"
                )
            })?;
            let num_processors = resolve_processor_count_flags(
                "SchedulingWithIndividualDeadlines",
                usage,
                args.num_processors,
                args.m,
            )?
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --num-processors or --m\n\n{usage}"
                )
            })?;
            let deadlines: Vec<usize> = util::parse_comma_list(deadlines_str)?;
            let precedences = parse_precedence_pairs(
                args.precedences
                    .as_deref()
                    .or(args.precedence_pairs.as_deref()),
            )?;
            anyhow::ensure!(
                deadlines.len() == num_tasks,
                "deadlines length ({}) must equal num_tasks ({})",
                deadlines.len(),
                num_tasks
            );
            for &(pred, succ) in &precedences {
                anyhow::ensure!(
                    pred < num_tasks && succ < num_tasks,
                    "precedence index out of range: ({}, {}) but num_tasks = {}",
                    pred,
                    succ,
                    num_tasks
                );
            }
            let _ = SchedulingWithIndividualDeadlines::new(
                num_tasks,
                num_processors,
                deadlines,
                precedences,
            );
        }
        "StringToStringCorrection" => {
            let usage = "Usage: pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2";
            let source_str = args.source_string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("StringToStringCorrection requires --source-string\n\n{usage}")
            })?;
            let target_str = args.target_string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("StringToStringCorrection requires --target-string\n\n{usage}")
            })?;
            let _ = parse_nonnegative_usize_bound(
                args.bound.ok_or_else(|| {
                    anyhow::anyhow!("StringToStringCorrection requires --bound\n\n{usage}")
                })?,
                "StringToStringCorrection",
                usage,
            )?;
            let source = parse_symbol_list_allow_empty(source_str)?;
            let target = parse_symbol_list_allow_empty(target_str)?;
            let inferred = source
                .iter()
                .chain(target.iter())
                .copied()
                .max()
                .map_or(0, |m| m + 1);
            let alphabet_size = args.alphabet_size.unwrap_or(inferred);
            anyhow::ensure!(
                alphabet_size >= inferred,
                "--alphabet-size {} is smaller than max symbol + 1 ({}) in the strings",
                alphabet_size,
                inferred
            );
        }
        "SparseMatrixCompression" => {
            let matrix = parse_bool_matrix(args)?;
            let usage = "Usage: pred create SparseMatrixCompression --matrix \"1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0\" --bound-k 2";
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "SparseMatrixCompression requires --matrix and --bound-k\n\n{usage}"
                )
            })?;
            let bound = parse_nonnegative_usize_bound(bound, "SparseMatrixCompression", usage)?;
            if bound == 0 {
                anyhow::bail!("SparseMatrixCompression requires bound >= 1\n\n{usage}");
            }
            let _ = SparseMatrixCompression::new(matrix, bound);
        }
        "StackerCrane" => {
            let usage = "Usage: pred create StackerCrane --arcs \"0>4,2>5,5>1,3>0,4>3\" --graph \"0-1,1-2,2-3,3-5,4-5,0-3,1-5\" --arc-lengths 3,4,2,5,3 --edge-lengths 2,1,3,2,1,4,3 --num-vertices 6";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("StackerCrane requires --arcs\n\n{usage}"))?;
            let (arcs_graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let (edges_graph, num_vertices) =
                parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            anyhow::ensure!(
                edges_graph.num_vertices() == num_vertices,
                "internal error: inconsistent graph vertex count"
            );
            anyhow::ensure!(
                num_vertices == arcs_graph.num_vertices(),
                "StackerCrane requires the directed and undirected inputs to agree on --num-vertices\n\n{usage}"
            );
            let arc_lengths = parse_arc_costs(args, num_arcs)?;
            let edge_lengths = parse_i32_edge_values(
                args.edge_lengths.as_ref(),
                edges_graph.num_edges(),
                "edge length",
            )?;
            let _ = problemreductions::models::misc::StackerCrane::try_new(
                num_vertices,
                arcs_graph.arcs(),
                edges_graph.edges(),
                arc_lengths,
                edge_lengths,
            )
            .map_err(|e| anyhow::anyhow!(e))?;
        }
        "ThreePartition" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ThreePartition requires --sizes and --bound\n\n\
                     Usage: pred create ThreePartition --sizes 4,5,6,4,6,5 --bound 15"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ThreePartition requires --bound\n\n\
                     Usage: pred create ThreePartition --sizes 4,5,6,4,6,5 --bound 15"
                )
            })?;
            let bound = u64::try_from(bound).map_err(|_| {
                anyhow::anyhow!(
                    "ThreePartition requires a positive integer --bound\n\n\
                     Usage: pred create ThreePartition --sizes 4,5,6,4,6,5 --bound 15"
                )
            })?;
            let sizes: Vec<u64> = util::parse_comma_list(sizes_str)?;
            let _ = ThreePartition::try_new(sizes, bound).map_err(anyhow::Error::msg)?;
        }
        "UndirectedFlowLowerBounds" => {
            let usage = "Usage: pred create UndirectedFlowLowerBounds --graph 0-1,0-2,1-3,2-3,1-4,3-5,4-5 --capacities 2,2,2,2,1,3,2 --lower-bounds 1,1,0,0,1,0,1 --source 0 --sink 5 --requirement 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities = parse_capacities(args, graph.num_edges(), usage)?;
            let lower_bounds = parse_lower_bounds(args, graph.num_edges(), usage)?;
            let num_vertices = graph.num_vertices();
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("UndirectedFlowLowerBounds requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("UndirectedFlowLowerBounds requires --sink\n\n{usage}")
            })?;
            let requirement = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("UndirectedFlowLowerBounds requires --requirement\n\n{usage}")
            })?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            let _ = UndirectedFlowLowerBounds::new(
                graph,
                capacities,
                lower_bounds,
                source,
                sink,
                requirement,
            );
        }
        "SequencingToMinimizeMaximumCumulativeCost" => {
            let costs_str = args.costs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeMaximumCumulativeCost requires --costs\n\n\
                     Usage: pred create SequencingToMinimizeMaximumCumulativeCost --costs 2,-1,3,-2,1,-3 --precedences \"0>2,1>2,1>3,2>4,3>5,4>5\""
                )
            })?;
            let costs: Vec<i64> = util::parse_comma_list(costs_str)?;
            let precedences = parse_precedence_pairs(
                args.precedences
                    .as_deref()
                    .or(args.precedence_pairs.as_deref()),
            )?;
            validate_precedence_pairs(&precedences, costs.len())?;
        }
        "SequencingToMinimizeWeightedTardiness" => {
            let lengths_str = args.lengths.as_deref().or(args.sizes.as_deref()).ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --lengths, --weights, --deadlines, and --bound\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let weights_str = args.weights.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --weights (comma-separated tardiness weights)\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --deadlines (comma-separated job deadlines)\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --bound\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            anyhow::ensure!(bound >= 0, "--bound must be non-negative");
            let lengths: Vec<u64> = util::parse_comma_list(lengths_str)?;
            let weights: Vec<u64> = util::parse_comma_list(weights_str)?;
            let deadlines: Vec<u64> = util::parse_comma_list(deadlines_str)?;
            anyhow::ensure!(
                lengths.len() == weights.len(),
                "lengths length ({}) must equal weights length ({})",
                lengths.len(),
                weights.len()
            );
            anyhow::ensure!(
                lengths.len() == deadlines.len(),
                "lengths length ({}) must equal deadlines length ({})",
                lengths.len(),
                deadlines.len()
            );
        }
        "SequencingWithinIntervals" => {
            let usage =
                "Usage: pred create SequencingWithinIntervals --release-times 0,0,5 --deadlines 11,11,6 --lengths 3,1,1";
            let rt_str = args.release_times.as_deref().ok_or_else(|| {
                anyhow::anyhow!("SequencingWithinIntervals requires --release-times\n\n{usage}")
            })?;
            let dl_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!("SequencingWithinIntervals requires --deadlines\n\n{usage}")
            })?;
            let len_str = args.lengths.as_deref().ok_or_else(|| {
                anyhow::anyhow!("SequencingWithinIntervals requires --lengths\n\n{usage}")
            })?;
            let release_times: Vec<u64> = util::parse_comma_list(rt_str)?;
            let deadlines: Vec<u64> = util::parse_comma_list(dl_str)?;
            let lengths: Vec<u64> = util::parse_comma_list(len_str)?;
            validate_sequencing_within_intervals_inputs(
                &release_times,
                &deadlines,
                &lengths,
                usage,
            )?;
        }
        "SetBasis" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "SetBasis requires --universe, --sets, and --k\n\n\
                     Usage: pred create SetBasis --universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3"
                )
            })?;
            let _ = args.k.ok_or_else(|| {
                anyhow::anyhow!(
                    "SetBasis requires --k\n\n\
                     Usage: pred create SetBasis --universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3"
                )
            })?;
            let sets = parse_sets(args)?;
            for (i, set) in sets.iter().enumerate() {
                for &element in set {
                    if element >= universe {
                        bail!(
                            "Set {} contains element {} which is outside universe of size {}",
                            i,
                            element,
                            universe
                        );
                    }
                }
            }
        }
        "ShortestWeightConstrainedPath" => {
            let usage = "Usage: pred create ShortestWeightConstrainedPath --graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4 --edge-lengths 2,4,3,1,5,4,2,6 --edge-weights 5,1,2,3,2,3,1,1 --source-vertex 0 --target-vertex 5 --weight-bound 8";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            if args.weights.is_some() {
                bail!(
                    "ShortestWeightConstrainedPath uses --edge-weights, not --weights\n\nUsage: {usage}"
                );
            }
            let edge_lengths_raw = args.edge_lengths.as_ref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --edge-lengths\n\nUsage: {usage}"
                )
            })?;
            let edge_weights_raw = args.edge_weights.as_ref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --edge-weights\n\nUsage: {usage}"
                )
            })?;
            let edge_lengths =
                parse_i32_edge_values(Some(edge_lengths_raw), graph.num_edges(), "edge length")?;
            let edge_weights =
                parse_i32_edge_values(Some(edge_weights_raw), graph.num_edges(), "edge weight")?;
            ensure_positive_i32_values(&edge_lengths, "edge lengths")?;
            ensure_positive_i32_values(&edge_weights, "edge weights")?;
            let source_vertex = args.source_vertex.ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --source-vertex\n\nUsage: {usage}"
                )
            })?;
            let target_vertex = args.target_vertex.ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --target-vertex\n\nUsage: {usage}"
                )
            })?;
            let weight_bound = args.weight_bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --weight-bound\n\nUsage: {usage}"
                )
            })?;
            ensure_vertex_in_bounds(source_vertex, graph.num_vertices(), "source_vertex")?;
            ensure_vertex_in_bounds(target_vertex, graph.num_vertices(), "target_vertex")?;
            ensure_positive_i32(weight_bound, "weight_bound")?;
        }
        "SteinerTree" => {
            let usage = "Usage: pred create SteinerTree --graph 0-1,1-2,1-3,3-4 --edge-weights 2,2,1,1 --terminals 0,2,4";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_edge_weights(args, graph.num_edges())?;
            let _ = parse_terminals(args, graph.num_vertices())
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
        }
        "TimetableDesign" => {
            let usage = "Usage: pred create TimetableDesign --num-periods 3 --num-craftsmen 5 --num-tasks 5 --craftsman-avail \"1,1,1;1,1,0;0,1,1;1,0,1;1,1,1\" --task-avail \"1,1,0;0,1,1;1,0,1;1,1,1;1,1,1\" --requirements \"1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0\"";
            let num_periods = args.num_periods.ok_or_else(|| {
                anyhow::anyhow!("TimetableDesign requires --num-periods\n\n{usage}")
            })?;
            let num_craftsmen = args.num_craftsmen.ok_or_else(|| {
                anyhow::anyhow!("TimetableDesign requires --num-craftsmen\n\n{usage}")
            })?;
            let num_tasks = args.num_tasks.ok_or_else(|| {
                anyhow::anyhow!("TimetableDesign requires --num-tasks\n\n{usage}")
            })?;
            let craftsman_avail =
                parse_named_bool_rows(args.craftsman_avail.as_deref(), "--craftsman-avail", usage)?;
            let task_avail =
                parse_named_bool_rows(args.task_avail.as_deref(), "--task-avail", usage)?;
            let requirements = parse_timetable_requirements(args.requirements.as_deref(), usage)?;
            validate_timetable_design_args(
                num_periods,
                num_craftsmen,
                num_tasks,
                &craftsman_avail,
                &task_avail,
                &requirements,
                usage,
            )?;
        }
        "UndirectedTwoCommodityIntegralFlow" => {
            let usage = "Usage: pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities = parse_capacities(args, graph.num_edges(), usage)?;
            for (edge_index, &capacity) in capacities.iter().enumerate() {
                let fits = usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some();
                if !fits {
                    bail!(
                        "capacity {} at edge index {} is too large for this platform\n\n{}",
                        capacity,
                        edge_index,
                        usage
                    );
                }
            }
            let num_vertices = graph.num_vertices();
            let source_1 = args.source_1.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --source-1\n\n{usage}")
            })?;
            let sink_1 = args.sink_1.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --sink-1\n\n{usage}")
            })?;
            let source_2 = args.source_2.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --source-2\n\n{usage}")
            })?;
            let sink_2 = args.sink_2.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --sink-2\n\n{usage}")
            })?;
            let _ = args.requirement_1.ok_or_else(|| {
                anyhow::anyhow!(
                    "UndirectedTwoCommodityIntegralFlow requires --requirement-1\n\n{usage}"
                )
            })?;
            let _ = args.requirement_2.ok_or_else(|| {
                anyhow::anyhow!(
                    "UndirectedTwoCommodityIntegralFlow requires --requirement-2\n\n{usage}"
                )
            })?;
            for (label, vertex) in [
                ("source-1", source_1),
                ("sink-1", sink_1),
                ("source-2", source_2),
                ("sink-2", sink_2),
            ] {
                validate_vertex_index(label, vertex, num_vertices, usage)?;
            }
        }
        _ => {}
    }

    Ok(())
}
