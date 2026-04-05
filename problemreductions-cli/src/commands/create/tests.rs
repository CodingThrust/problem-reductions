use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;

use super::ensure_attribute_indices_in_range;
use super::parse_bool_rows;
use super::schema_support::*;
use super::*;
use crate::cli::{Cli, Commands};
use crate::output::OutputConfig;

fn temp_output_path(name: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("{}_{}.json", name, suffix))
}

#[test]
fn test_problem_help_uses_bound_for_length_bounded_disjoint_paths() {
    assert_eq!(
        problem_help_flag_name("LengthBoundedDisjointPaths", "max_length", "usize", false),
        "max-length"
    );
}

#[test]
fn test_problem_help_preserves_generic_field_kebab_case() {
    assert_eq!(
        problem_help_flag_name("LengthBoundedDisjointPaths", "max_paths", "usize", false,),
        "max-paths"
    );
}

#[test]
fn test_help_flag_name_mentions_m_alias_for_scheduling_processors() {
    assert_eq!(
        help_flag_name("SchedulingWithIndividualDeadlines", "num_processors"),
        "num-processors/--m"
    );
    assert_eq!(
        help_flag_name("FlowShopScheduling", "num_processors"),
        "num-processors/--m"
    );
}

#[test]
fn test_parse_field_value_parses_simple_graph_to_json() {
    let value = parse_field_value("SimpleGraph", "graph", "0-1,1-2", &CreateContext::default())
        .expect("parse graph");

    assert_eq!(
        value,
        serde_json::json!({
            "num_vertices": 3,
            "edges": [[0, 1], [1, 2]],
        })
    );
}

#[test]
fn test_parse_field_value_parses_dependency_pairs() {
    let value = parse_field_value(
        "Vec<(Vec<usize>, Vec<usize>)>",
        "dependencies",
        "0,1>2,3;2>4",
        &CreateContext::default(),
    )
    .expect("parse dependencies");

    assert_eq!(value, serde_json::json!([[[0, 1], [2, 3]], [[2], [4]],]));
}

#[test]
fn test_parse_field_value_parses_job_shop_jobs() {
    let value = parse_field_value(
        "Vec<Vec<(usize, u64)>>",
        "jobs",
        "0:3,1:4;1:2,0:3,1:2",
        &CreateContext::default(),
    )
    .expect("parse jobs");

    assert_eq!(
        value,
        serde_json::json!([[[0, 3], [1, 4]], [[1, 2], [0, 3], [1, 2]],])
    );
}

#[test]
fn test_parse_field_value_parses_quantifiers_using_context_num_vars() {
    let context = CreateContext::default().with_field("num_vars", serde_json::json!(3));
    let value = parse_field_value("Vec<Quantifier>", "quantifiers", "E,A,E", &context)
        .expect("parse quantifiers");

    assert_eq!(value, serde_json::json!(["Exists", "ForAll", "Exists"]));
}

#[test]
fn test_schema_driven_supported_problem_includes_cli_creatable_problem() {
    assert!(
            schema_driven_supported_problem("ConjunctiveBooleanQuery"),
            "all CLI-creatable problems should opt into schema-driven create unless explicitly excluded"
        );
    assert!(!schema_driven_supported_problem("ILP"));
    assert!(!schema_driven_supported_problem("CircuitSAT"));
}

#[test]
fn test_create_schema_driven_builds_job_shop_scheduling() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "JobShopScheduling",
        "--jobs",
        "0:3,1:4;1:2,0:3,1:2",
        "--num-processors",
        "2",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let (data, variant) = create_schema_driven(&args, "JobShopScheduling", &BTreeMap::new())
        .expect("schema-driven create should parse")
        .expect("schema-driven path should support JobShopScheduling");

    let entry = problemreductions::registry::find_variant_entry("JobShopScheduling", &variant)
        .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["num_processors"], 2);
    assert_eq!(data["jobs"][0], serde_json::json!([[0, 3], [1, 4]]));
}

#[test]
fn test_create_schema_driven_builds_quantified_boolean_formulas() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "QuantifiedBooleanFormulas",
        "--num-vars",
        "3",
        "--quantifiers",
        "E,A,E",
        "--clauses",
        "1,2;-1,3",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let (data, variant) =
        create_schema_driven(&args, "QuantifiedBooleanFormulas", &BTreeMap::new())
            .expect("schema-driven create should parse")
            .expect("schema-driven path should support QBF");

    let entry =
        problemreductions::registry::find_variant_entry("QuantifiedBooleanFormulas", &variant)
            .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(
        data["quantifiers"],
        serde_json::json!(["Exists", "ForAll", "Exists"])
    );
}

#[test]
fn test_create_schema_driven_builds_undirected_flow_lower_bounds() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "UndirectedFlowLowerBounds",
        "--graph",
        "0-1,0-2,1-3,2-3",
        "--capacities",
        "2,2,2,2",
        "--lower-bounds",
        "1,0,0,1",
        "--source",
        "0",
        "--sink",
        "3",
        "--requirement",
        "2",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let (data, variant) =
        create_schema_driven(&args, "UndirectedFlowLowerBounds", &BTreeMap::new())
            .expect("schema-driven create should parse")
            .expect("schema-driven path should support UndirectedFlowLowerBounds");

    let entry =
        problemreductions::registry::find_variant_entry("UndirectedFlowLowerBounds", &variant)
            .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["graph"]["num_vertices"], 4);
    assert_eq!(data["capacities"], serde_json::json!([2, 2, 2, 2]));
    assert_eq!(data["lower_bounds"], serde_json::json!([1, 0, 0, 1]));
}

#[test]
fn test_create_schema_driven_builds_conjunctive_boolean_query() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "ConjunctiveBooleanQuery",
        "--domain-size",
        "6",
        "--relations",
        "2:0,3|1,3;3:0,1,5|1,2,5",
        "--conjuncts-spec",
        "0:v0,c3;0:v1,c3;1:v0,v1,c5",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let (data, variant) = create_schema_driven(&args, "ConjunctiveBooleanQuery", &BTreeMap::new())
        .expect("schema-driven create should parse")
        .expect("schema-driven path should support CBQ");

    let entry =
        problemreductions::registry::find_variant_entry("ConjunctiveBooleanQuery", &variant)
            .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["num_variables"], 2);
    assert_eq!(data["relations"][0]["arity"], 2);
    assert_eq!(
        data["conjuncts"][1],
        serde_json::json!([0, [{"Variable": 1}, {"Constant": 3}]])
    );
}

#[test]
fn test_create_schema_driven_builds_closest_vector_problem_with_default_bounds() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "CVP",
        "--basis",
        "1,0;0,1",
        "--target-vec",
        "0.5,0.5",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let resolved_variant = variant_map(&[("weight", "i32")]);
    let (data, variant) = create_schema_driven(&args, "ClosestVectorProblem", &resolved_variant)
        .expect("schema-driven create should parse")
        .expect("schema-driven path should support CVP");

    let entry = problemreductions::registry::find_variant_entry("ClosestVectorProblem", &variant)
        .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["basis"], serde_json::json!([[1, 0], [0, 1]]));
    assert_eq!(
        data["bounds"],
        serde_json::json!([
            {"lower": -10, "upper": 10},
            {"lower": -10, "upper": 10},
        ])
    );
}

#[test]
fn test_create_schema_driven_builds_cdft() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "ConsistencyOfDatabaseFrequencyTables",
        "--num-objects",
        "6",
        "--attribute-domains",
        "2,3,2",
        "--frequency-tables",
        "0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1",
        "--known-values",
        "0,0,0;3,0,1;1,2,1",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let (data, variant) = create_schema_driven(
        &args,
        "ConsistencyOfDatabaseFrequencyTables",
        &BTreeMap::new(),
    )
    .expect("schema-driven create should parse")
    .expect("schema-driven path should support CDFT");

    let entry = problemreductions::registry::find_variant_entry(
        "ConsistencyOfDatabaseFrequencyTables",
        &variant,
    )
    .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["num_objects"], 6);
    assert_eq!(data["frequency_tables"][0]["attribute_a"], 0);
    assert_eq!(data["known_values"][2]["attribute"], 2);
}

#[test]
fn test_create_schema_driven_builds_balanced_complete_bipartite_subgraph() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "BalancedCompleteBipartiteSubgraph",
        "--left",
        "4",
        "--right",
        "4",
        "--biedges",
        "0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3",
        "--k",
        "3",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let (data, variant) =
        create_schema_driven(&args, "BalancedCompleteBipartiteSubgraph", &BTreeMap::new())
            .expect("schema-driven create should parse")
            .expect("schema-driven path should support balanced biclique");

    let entry = problemreductions::registry::find_variant_entry(
        "BalancedCompleteBipartiteSubgraph",
        &variant,
    )
    .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["graph"]["left_size"], 4);
    assert_eq!(data["graph"]["right_size"], 4);
    assert_eq!(data["k"], 3);
}

#[test]
fn test_create_schema_driven_builds_mixed_chinese_postman() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "MixedChinesePostman/i32",
        "--graph",
        "0-2,1-3,0-4,4-2",
        "--arcs",
        "0>1,1>2,2>3,3>0",
        "--edge-weights",
        "2,3,1,2",
        "--arc-weights",
        "2,3,1,4",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let resolved_variant = variant_map(&[("weight", "i32")]);
    let (data, variant) = create_schema_driven(&args, "MixedChinesePostman", &resolved_variant)
        .expect("schema-driven create should parse")
        .expect("schema-driven path should support mixed chinese postman");

    let entry = problemreductions::registry::find_variant_entry("MixedChinesePostman", &variant)
        .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["graph"]["num_vertices"], 5);
    assert_eq!(data["arc_weights"], serde_json::json!([2, 3, 1, 4]));
    assert_eq!(data["edge_weights"], serde_json::json!([2, 3, 1, 2]));
}

#[test]
fn test_create_schema_driven_builds_unit_disk_graph_problem_with_default_radius() {
    let cli = Cli::parse_from([
        "pred",
        "create",
        "MIS/UnitDiskGraph",
        "--positions",
        "0,0;1,0;0.5,0.8",
    ]);

    let Commands::Create(args) = cli.command else {
        panic!("expected create command");
    };

    let resolved_variant = variant_map(&[("graph", "UnitDiskGraph"), ("weight", "One")]);
    let (data, variant) = create_schema_driven(&args, "MaximumIndependentSet", &resolved_variant)
        .expect("schema-driven create should parse")
        .expect("schema-driven path should support UnitDiskGraph variants");

    let entry = problemreductions::registry::find_variant_entry("MaximumIndependentSet", &variant)
        .expect("variant entry");
    (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
    assert_eq!(data["graph"]["positions"].as_array().unwrap().len(), 3);
    assert_eq!(
        data["graph"]["edges"],
        serde_json::json!([[0, 1], [0, 2], [1, 2]])
    );
}

#[test]
fn test_schema_help_example_for_qbf_uses_example_db() {
    let example = schema_help_example_for("QuantifiedBooleanFormulas", &BTreeMap::new()).unwrap();
    assert_eq!(
        example,
        "--num-vars 2 --quantifiers E,A --clauses \"1,2;1,-2\""
    );
}

#[test]
fn test_schema_help_example_for_cbm_uses_json_matrix_syntax() {
    let example =
        schema_help_example_for("ConsecutiveBlockMinimization", &BTreeMap::new()).unwrap();
    assert!(example.contains("--matrix \"[[false,true,false,false,false,false],[true,false,true,false,false,false],[false,true,false,true,false,false],[false,false,true,false,true,false],[false,false,false,true,false,true],[false,false,false,false,true,false]]\""));
    assert!(example.contains("--bound-k 6"));
}

#[test]
fn test_problem_help_flag_name_uses_bound_for_grouping_by_swapping_budget() {
    assert_eq!(
        problem_help_flag_name("GroupingBySwapping", "budget", "usize", false),
        "bound"
    );
}

#[test]
fn test_problem_help_flag_name_preserves_edge_lengths_for_shortest_weight_constrained_path() {
    assert_eq!(
        problem_help_flag_name(
            "ShortestWeightConstrainedPath",
            "edge_lengths",
            "Vec<W>",
            false
        ),
        "edge-lengths"
    );
}

#[test]
fn test_problem_help_flag_name_uses_edge_weights_for_longest_circuit_edge_lengths() {
    assert_eq!(
        problem_help_flag_name("LongestCircuit", "edge_lengths", "Vec<W>", false),
        "edge-weights"
    );
}

#[test]
fn test_ensure_attribute_indices_in_range_rejects_out_of_range_index() {
    let err = ensure_attribute_indices_in_range(&[0, 4], 3, "Functional dependency '0:4' rhs")
        .unwrap_err();
    assert!(
        err.to_string().contains("out of range"),
        "unexpected error: {err}"
    );
}

#[test]
fn test_create_scheduling_with_individual_deadlines_accepts_m_alias() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "SchedulingWithIndividualDeadlines",
        "--num-tasks",
        "3",
        "--deadlines",
        "1,1,2",
        "--m",
        "2",
    ])
    .expect("parse create command");

    let Commands::Create(args) = cli.command else {
        panic!("expected create subcommand");
    };

    let out = OutputConfig {
        output: Some(
            std::env::temp_dir()
                .join("pred_test_create_scheduling_with_individual_deadlines_m_alias.json"),
        ),
        quiet: true,
        json: false,
        auto_json: false,
    };
    create(&args, &out).expect("`--m` should satisfy --num-processors alias");

    let created: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.output.as_ref().unwrap()).unwrap())
            .unwrap();
    std::fs::remove_file(out.output.as_ref().unwrap()).ok();

    assert_eq!(created["type"], "SchedulingWithIndividualDeadlines");
    assert_eq!(created["data"]["num_processors"], 2);
}

#[test]
fn test_create_prime_attribute_name_accepts_canonical_flags() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "PrimeAttributeName",
        "--universe",
        "6",
        "--dependencies",
        "0,1>2,3,4,5;2,3>0,1,4,5",
        "--query-attribute",
        "3",
    ])
    .expect("parse create command");

    let Commands::Create(args) = cli.command else {
        panic!("expected create subcommand");
    };

    let output_path = temp_output_path("prime_attribute_name");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).expect("create PrimeAttributeName JSON");

    let created: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output_path).unwrap()).unwrap();
    fs::remove_file(output_path).ok();

    assert_eq!(created["type"], "PrimeAttributeName");
    assert_eq!(created["data"]["query_attribute"], 3);
    assert_eq!(
        created["data"]["dependencies"][0],
        serde_json::json!([[0, 1], [2, 3, 4, 5]])
    );
}

#[test]
fn test_problem_help_uses_prime_attribute_name_cli_overrides() {
    assert_eq!(
        problem_help_flag_name("PrimeAttributeName", "num_attributes", "usize", false),
        "universe"
    );
    assert_eq!(
        problem_help_flag_name(
            "PrimeAttributeName",
            "dependencies",
            "Vec<(Vec<usize>, Vec<usize>)>",
            false,
        ),
        "dependencies"
    );
    assert_eq!(
        problem_help_flag_name("PrimeAttributeName", "query_attribute", "usize", false),
        "query-attribute"
    );
}

#[test]
fn test_problem_help_uses_problem_specific_lcs_strings_hint() {
    assert_eq!(
        help_flag_hint(
            "LongestCommonSubsequence",
            "strings",
            "Vec<Vec<usize>>",
            None,
        ),
        "raw strings: \"ABAC;BACA\" or symbol lists: \"0,1,0;1,0,1\""
    );
}

#[test]
fn test_problem_help_uses_string_to_string_correction_cli_flags() {
    assert_eq!(
        problem_help_flag_name("StringToStringCorrection", "source", "Vec<usize>", false),
        "source-string"
    );
    assert_eq!(
        problem_help_flag_name("StringToStringCorrection", "target", "Vec<usize>", false),
        "target-string"
    );
    assert_eq!(
        problem_help_flag_name("StringToStringCorrection", "bound", "usize", false),
        "bound"
    );
}

#[test]
fn test_problem_help_keeps_generic_vec_vec_usize_hint_for_other_models() {
    assert_eq!(
        help_flag_hint("SetBasis", "sets", "Vec<Vec<usize>>", None),
        "semicolon-separated sets: \"0,1;1,2;0,2\""
    );
}

#[test]
fn test_problem_help_uses_k_for_staff_scheduling() {
    assert_eq!(
        help_flag_name("StaffScheduling", "shifts_per_schedule"),
        "k"
    );
    assert_eq!(
        problem_help_flag_name("StaffScheduling", "shifts_per_schedule", "usize", false),
        "k"
    );
}

#[test]
fn test_parse_bool_rows_reports_generic_invalid_boolean_entry() {
    let err = parse_bool_rows("1,maybe").unwrap_err().to_string();
    assert_eq!(
        err,
        "Invalid boolean entry 'maybe': expected 0/1 or true/false"
    );
}

#[test]
fn test_create_staff_scheduling_outputs_problem_json() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "StaffScheduling",
        "--schedules",
        "1,1,1,1,1,0,0;0,1,1,1,1,1,0;0,0,1,1,1,1,1;1,0,0,1,1,1,1;1,1,0,0,1,1,1",
        "--requirements",
        "2,2,2,3,3,2,1",
        "--num-workers",
        "4",
        "--k",
        "5",
    ])
    .unwrap();

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let output_path = std::env::temp_dir().join(format!("staff-scheduling-create-{suffix}.json"));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&output_path).unwrap()).unwrap();
    assert_eq!(json["type"], "StaffScheduling");
    assert_eq!(json["data"]["num_workers"], 4);
    assert_eq!(
        json["data"]["requirements"],
        serde_json::json!([2, 2, 2, 3, 3, 2, 1])
    );
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_create_path_constrained_network_flow_outputs_problem_json() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "PathConstrainedNetworkFlow",
        "--arcs",
        "0>1,0>2,1>3,1>4,2>4,3>5,4>5,4>6,5>7,6>7",
        "--capacities",
        "2,1,1,1,1,1,1,1,2,1",
        "--source",
        "0",
        "--sink",
        "7",
        "--paths",
        "0,2,5,8;0,3,6,8;0,3,7,9;1,4,6,8;1,4,7,9",
        "--requirement",
        "3",
    ])
    .expect("parse create command");

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let output_path = temp_output_path("path_constrained_network_flow");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).expect("create PathConstrainedNetworkFlow JSON");

    let created: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output_path).unwrap()).unwrap();
    fs::remove_file(output_path).ok();

    assert_eq!(created["type"], "PathConstrainedNetworkFlow");
    assert_eq!(created["data"]["source"], 0);
    assert_eq!(created["data"]["sink"], 7);
    assert_eq!(created["data"]["requirement"], 3);
    assert_eq!(created["data"]["paths"][0], serde_json::json!([0, 2, 5, 8]));
}

#[test]
fn test_create_path_constrained_network_flow_rejects_invalid_paths() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "PathConstrainedNetworkFlow",
        "--arcs",
        "0>1,1>2,2>3",
        "--capacities",
        "1,1,1",
        "--source",
        "0",
        "--sink",
        "3",
        "--paths",
        "0,3",
        "--requirement",
        "1",
    ])
    .expect("parse create command");

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("out of bounds") || err.contains("not contiguous"));
}

#[test]
fn test_create_staff_scheduling_reports_invalid_schedule_without_panic() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "StaffScheduling",
        "--schedules",
        "1,1,1,1,1,0,0;0,1,1,1,1,1",
        "--requirements",
        "2,2,2,3,3,2,1",
        "--num-workers",
        "4",
        "--k",
        "5",
    ])
    .unwrap();

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let result = std::panic::catch_unwind(|| create(&args, &out));
    assert!(result.is_ok(), "create should return an error, not panic");
    let err = result.unwrap().unwrap_err().to_string();
    // parse_bool_rows catches ragged rows before validate_staff_scheduling_args
    assert!(
        err.contains("All rows") || err.contains("schedule 1 has 6 periods, expected 7"),
        "expected row-length validation error, got: {err}"
    );
}

#[test]
fn test_problem_help_uses_num_tasks_for_timetable_design() {
    assert_eq!(
        problem_help_flag_name("TimetableDesign", "num_tasks", "usize", false),
        "num-tasks"
    );
    assert_eq!(
        help_flag_hint("TimetableDesign", "craftsman_avail", "Vec<Vec<bool>>", None),
        "semicolon-separated 0/1 rows: \"1,1,0;0,1,1\""
    );
}

#[test]
fn test_example_for_path_constrained_network_flow_mentions_paths_flag() {
    let example = example_for("PathConstrainedNetworkFlow", None);
    assert!(example.contains("--paths"));
    assert!(example.contains("--requirement"));
}

#[test]
fn test_example_for_three_partition_mentions_sizes_and_bound() {
    let example = example_for("ThreePartition", None);
    assert!(example.contains("--sizes"));
    assert!(example.contains("--bound"));
}

#[test]
fn test_create_three_partition_outputs_problem_json() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "ThreePartition",
        "--sizes",
        "4,5,6,4,6,5",
        "--bound",
        "15",
    ])
    .expect("parse create command");

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let output_path = temp_output_path("three_partition_create");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).expect("create ThreePartition JSON");

    let created: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output_path).unwrap()).unwrap();
    fs::remove_file(output_path).ok();

    assert_eq!(created["type"], "ThreePartition");
    assert_eq!(
        created["data"]["sizes"],
        serde_json::json!([4, 5, 6, 4, 6, 5])
    );
    assert_eq!(created["data"]["bound"], 15);
}

#[test]
fn test_create_three_partition_requires_bound() {
    let cli = Cli::try_parse_from(["pred", "create", "ThreePartition", "--sizes", "4,5,6,4,6,5"])
        .expect("parse create command");

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("ThreePartition requires --bound"));
}

#[test]
fn test_create_three_partition_rejects_invalid_instance() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "ThreePartition",
        "--sizes",
        "4,5,6,4,6,5",
        "--bound",
        "14",
    ])
    .expect("parse create command");

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("must equal m * bound"));
}

#[test]
fn test_create_timetable_design_outputs_problem_json() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "TimetableDesign",
        "--num-periods",
        "3",
        "--num-craftsmen",
        "5",
        "--num-tasks",
        "5",
        "--craftsman-avail",
        "1,1,1;1,1,0;0,1,1;1,0,1;1,1,1",
        "--task-avail",
        "1,1,0;0,1,1;1,0,1;1,1,1;1,1,1",
        "--requirements",
        "1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0",
    ])
    .unwrap();

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let output_path = std::env::temp_dir().join(format!("timetable-design-create-{suffix}.json"));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&output_path).unwrap()).unwrap();
    assert_eq!(json["type"], "TimetableDesign");
    assert_eq!(json["data"]["num_periods"], 3);
    assert_eq!(json["data"]["num_craftsmen"], 5);
    assert_eq!(json["data"]["num_tasks"], 5);
    assert_eq!(
        json["data"]["craftsman_avail"],
        serde_json::json!([
            [true, true, true],
            [true, true, false],
            [false, true, true],
            [true, false, true],
            [true, true, true]
        ])
    );
    assert_eq!(
        json["data"]["task_avail"],
        serde_json::json!([
            [true, true, false],
            [false, true, true],
            [true, false, true],
            [true, true, true],
            [true, true, true]
        ])
    );
    assert_eq!(
        json["data"]["requirements"],
        serde_json::json!([
            [1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0]
        ])
    );
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_create_timetable_design_reports_invalid_matrix_without_panic() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "TimetableDesign",
        "--num-periods",
        "3",
        "--num-craftsmen",
        "5",
        "--num-tasks",
        "5",
        "--craftsman-avail",
        "1,1,1;1,1",
        "--task-avail",
        "1,1,0;0,1,1;1,0,1;1,1,1;1,1,1",
        "--requirements",
        "1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0",
    ])
    .unwrap();

    let args = match cli.command {
        Commands::Create(args) => args,
        _ => panic!("expected create command"),
    };

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let result = std::panic::catch_unwind(|| create(&args, &out));
    assert!(result.is_ok(), "create should return an error, not panic");
    let err = result.unwrap().unwrap_err().to_string();
    assert!(
        err.contains("--craftsman-avail"),
        "expected timetable matrix validation error, got: {err}"
    );
    assert!(err.contains("Usage: pred create TimetableDesign"));
}

#[test]
fn test_create_generalized_hex_serializes_problem_json() {
    let output = temp_output_path("generalized_hex_create");
    let cli = Cli::try_parse_from([
        "pred",
        "-o",
        output.to_str().unwrap(),
        "create",
        "GeneralizedHex",
        "--graph",
        "0-1,0-2,0-3,1-4,2-4,3-4,4-5",
        "--source",
        "0",
        "--sink",
        "5",
    ])
    .unwrap();
    let out = OutputConfig {
        output: cli.output.clone(),
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
    fs::remove_file(&output).unwrap();
    assert_eq!(json["type"], "GeneralizedHex");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["data"]["source"], 0);
    assert_eq!(json["data"]["target"], 5);
}

#[test]
fn test_create_generalized_hex_requires_sink() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "GeneralizedHex",
        "--graph",
        "0-1,1-2,2-3",
        "--source",
        "0",
    ])
    .unwrap();
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err();
    assert!(err.to_string().contains("GeneralizedHex requires --sink"));
}

#[test]
fn test_create_capacity_assignment_serializes_problem_json() {
    let output = temp_output_path("capacity_assignment_create");
    let cli = Cli::try_parse_from([
        "pred",
        "-o",
        output.to_str().unwrap(),
        "create",
        "CapacityAssignment",
        "--capacities",
        "1,2,3",
        "--cost-matrix",
        "1,3,6;2,4,7;1,2,5",
        "--delay-matrix",
        "8,4,1;7,3,1;6,3,1",
        "--delay-budget",
        "12",
    ])
    .expect("parse create command");
    let out = OutputConfig {
        output: cli.output.clone(),
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
    fs::remove_file(&output).unwrap();
    assert_eq!(json["type"], "CapacityAssignment");
    assert_eq!(json["data"]["capacities"], serde_json::json!([1, 2, 3]));
    assert_eq!(json["data"]["delay_budget"], 12);
}

#[test]
fn test_create_production_planning_serializes_problem_json() {
    let output = temp_output_path("production_planning_create");
    let cli = Cli::try_parse_from([
        "pred",
        "-o",
        output.to_str().unwrap(),
        "create",
        "ProductionPlanning",
        "--num-periods",
        "6",
        "--demands",
        "5,3,7,2,8,5",
        "--capacities",
        "12,12,12,12,12,12",
        "--setup-costs",
        "10,10,10,10,10,10",
        "--production-costs",
        "1,1,1,1,1,1",
        "--inventory-costs",
        "1,1,1,1,1,1",
        "--cost-bound",
        "80",
    ])
    .expect("parse create command");
    let out = OutputConfig {
        output: cli.output.clone(),
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
    fs::remove_file(&output).unwrap();
    assert_eq!(json["type"], "ProductionPlanning");
    assert_eq!(json["data"]["num_periods"], 6);
    assert_eq!(
        json["data"]["demands"],
        serde_json::json!([5, 3, 7, 2, 8, 5])
    );
    assert_eq!(
        json["data"]["capacities"],
        serde_json::json!([12, 12, 12, 12, 12, 12])
    );
    assert_eq!(
        json["data"]["setup_costs"],
        serde_json::json!([10, 10, 10, 10, 10, 10])
    );
    assert_eq!(
        json["data"]["production_costs"],
        serde_json::json!([1, 1, 1, 1, 1, 1])
    );
    assert_eq!(
        json["data"]["inventory_costs"],
        serde_json::json!([1, 1, 1, 1, 1, 1])
    );
    assert_eq!(json["data"]["cost_bound"], 80);
}

#[test]
fn test_create_production_planning_requires_all_period_vectors() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "ProductionPlanning",
        "--num-periods",
        "6",
        "--demands",
        "5,3,7,2,8,5",
        "--capacities",
        "12,12,12,12,12,12",
        "--setup-costs",
        "10,10,10,10,10,10",
        "--inventory-costs",
        "1,1,1,1,1,1",
        "--cost-bound",
        "80",
    ])
    .expect("parse create command");
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err();
    assert!(err
        .to_string()
        .contains("ProductionPlanning requires --production-costs"));
}

#[test]
fn test_create_production_planning_rejects_mismatched_period_lengths() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "ProductionPlanning",
        "--num-periods",
        "6",
        "--demands",
        "5,3,7,2,8",
        "--capacities",
        "12,12,12,12,12,12",
        "--setup-costs",
        "10,10,10,10,10,10",
        "--production-costs",
        "1,1,1,1,1,1",
        "--inventory-costs",
        "1,1,1,1,1,1",
        "--cost-bound",
        "80",
    ])
    .expect("parse create command");
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err();
    assert!(err
        .to_string()
        .contains("--demands must contain exactly 6 entries"));
}

#[test]
fn test_create_example_production_planning_uses_canonical_example() {
    let output = temp_output_path("production_planning_example_create");
    let cli = Cli::try_parse_from([
        "pred",
        "-o",
        output.to_str().unwrap(),
        "create",
        "--example",
        "ProductionPlanning",
    ])
    .expect("parse create command");
    let out = OutputConfig {
        output: cli.output.clone(),
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
    fs::remove_file(&output).unwrap();
    assert_eq!(json["type"], "ProductionPlanning");
    assert_eq!(json["data"]["num_periods"], 4);
    assert_eq!(json["data"]["demands"], serde_json::json!([2, 1, 3, 2]));
    assert_eq!(json["data"]["cost_bound"], 16);
}

#[test]
fn test_create_longest_path_serializes_problem_json() {
    let output = temp_output_path("longest_path_create");
    let cli = Cli::try_parse_from([
        "pred",
        "-o",
        output.to_str().unwrap(),
        "create",
        "LongestPath",
        "--graph",
        "0-1,0-2,1-3,2-3,2-4,3-5,4-5,4-6,5-6,1-6",
        "--edge-lengths",
        "3,2,4,1,5,2,3,2,4,1",
        "--source-vertex",
        "0",
        "--target-vertex",
        "6",
    ])
    .unwrap();
    let out = OutputConfig {
        output: cli.output.clone(),
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
    fs::remove_file(&output).unwrap();
    assert_eq!(json["type"], "LongestPath");
    assert_eq!(json["variant"]["graph"], "SimpleGraph");
    assert_eq!(json["variant"]["weight"], "i32");
    assert_eq!(json["data"]["source_vertex"], 0);
    assert_eq!(json["data"]["target_vertex"], 6);
    assert_eq!(
        json["data"]["edge_lengths"],
        serde_json::json!([3, 2, 4, 1, 5, 2, 3, 2, 4, 1])
    );
}

#[test]
fn test_create_undirected_flow_lower_bounds_serializes_problem_json() {
    let output = temp_output_path("undirected_flow_lower_bounds_create");
    let cli = Cli::try_parse_from([
        "pred",
        "-o",
        output.to_str().unwrap(),
        "create",
        "UndirectedFlowLowerBounds",
        "--graph",
        "0-1,0-2,1-3,2-3,1-4,3-5,4-5",
        "--capacities",
        "2,2,2,2,1,3,2",
        "--lower-bounds",
        "1,1,0,0,1,0,1",
        "--source",
        "0",
        "--sink",
        "5",
        "--requirement",
        "3",
    ])
    .unwrap();
    let out = OutputConfig {
        output: cli.output.clone(),
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    create(&args, &out).unwrap();

    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
    fs::remove_file(&output).unwrap();
    assert_eq!(json["type"], "UndirectedFlowLowerBounds");
    assert_eq!(json["data"]["source"], 0);
    assert_eq!(json["data"]["sink"], 5);
    assert_eq!(json["data"]["requirement"], 3);
    assert_eq!(
        json["data"]["lower_bounds"],
        serde_json::json!([1, 1, 0, 0, 1, 0, 1])
    );
}

#[test]
fn test_create_capacity_assignment_rejects_non_monotone_cost_row() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "CapacityAssignment",
        "--capacities",
        "1,2,3",
        "--cost-matrix",
        "1,3,2;2,4,7;1,2,5",
        "--delay-matrix",
        "8,4,1;7,3,1;6,3,1",
        "--delay-budget",
        "12",
    ])
    .expect("parse create command");
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("cost row 0"));
    assert!(err.contains("non-decreasing"));
}

#[test]
fn test_create_capacity_assignment_rejects_matrix_width_mismatch() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "CapacityAssignment",
        "--capacities",
        "1,2,3",
        "--cost-matrix",
        "1,3;2,4,7;1,2,5",
        "--delay-matrix",
        "8,4,1;7,3,1;6,3,1",
        "--delay-budget",
        "12",
    ])
    .expect("parse create command");
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("cost row 0"));
    assert!(err.contains("capacities length"));
}

#[test]
fn test_create_longest_path_requires_edge_lengths() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "LongestPath",
        "--graph",
        "0-1,1-2",
        "--source-vertex",
        "0",
        "--target-vertex",
        "2",
    ])
    .unwrap();
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err();
    assert!(err
        .to_string()
        .contains("LongestPath requires --edge-lengths"));
}

#[test]
fn test_create_longest_path_rejects_weights_flag() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "LongestPath",
        "--graph",
        "0-1,1-2",
        "--weights",
        "1,1,1",
        "--source-vertex",
        "0",
        "--target-vertex",
        "2",
        "--edge-lengths",
        "5,7",
    ])
    .unwrap();
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err();
    assert!(err
        .to_string()
        .contains("LongestPath uses --edge-lengths, not --weights"));
}

#[test]
fn test_create_undirected_flow_lower_bounds_requires_lower_bounds() {
    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "UndirectedFlowLowerBounds",
        "--graph",
        "0-1,0-2,1-3,2-3,1-4,3-5,4-5",
        "--capacities",
        "2,2,2,2,1,3,2",
        "--source",
        "0",
        "--sink",
        "5",
        "--requirement",
        "3",
    ])
    .unwrap();
    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let err = create(&args, &out).unwrap_err();
    assert!(err
        .to_string()
        .contains("UndirectedFlowLowerBounds requires --lower-bounds"));
}

fn empty_args() -> CreateArgs {
    CreateArgs {
        problem: Some("BiconnectivityAugmentation".to_string()),
        example: None,
        example_target: None,
        example_side: crate::cli::ExampleSide::Source,
        graph: None,
        weights: None,
        edge_weights: None,
        edge_lengths: None,
        capacities: None,
        demands: None,
        setup_costs: None,
        production_costs: None,
        inventory_costs: None,
        bundle_capacities: None,
        cost_matrix: None,
        delay_matrix: None,
        lower_bounds: None,
        multipliers: None,
        source: None,
        sink: None,
        requirement: None,
        num_paths_required: None,
        paths: None,
        couplings: None,
        fields: None,
        clauses: None,
        disjuncts: None,
        num_vars: None,
        matrix: None,
        k: None,
        num_partitions: None,
        random: false,
        source_vertex: None,
        target_vertex: None,
        num_vertices: None,
        edge_prob: None,
        seed: None,
        target: None,
        m: None,
        n: None,
        positions: None,
        radius: None,
        source_1: None,
        sink_1: None,
        source_2: None,
        sink_2: None,
        requirement_1: None,
        requirement_2: None,
        sizes: None,
        probabilities: None,
        capacity: None,
        sequence: None,
        sets: None,
        r_sets: None,
        s_sets: None,
        r_weights: None,
        s_weights: None,
        partition: None,
        partitions: None,
        bundles: None,
        universe: None,
        biedges: None,
        left: None,
        right: None,
        rank: None,
        basis: None,
        target_vec: None,
        bounds: None,
        release_times: None,
        lengths: None,
        terminals: None,
        terminal_pairs: None,
        tree: None,
        required_edges: None,
        bound: None,
        latency_bound: None,
        length_bound: None,
        weight_bound: None,
        diameter_bound: None,
        cost_bound: None,
        delay_budget: None,
        pattern: None,
        strings: None,
        string: None,
        arc_costs: None,
        arcs: None,
        left_arcs: None,
        right_arcs: None,
        values: None,
        precedences: None,
        distance_matrix: None,
        potential_edges: None,
        budget: None,
        max_cycle_length: None,
        candidate_arcs: None,
        deadlines: None,
        precedence_pairs: None,
        task_lengths: None,
        job_tasks: None,
        resource_bounds: None,
        resource_requirements: None,
        deadline: None,
        num_processors: None,
        alphabet_size: None,
        deps: None,
        query: None,
        dependencies: None,
        num_attributes: None,
        source_string: None,
        target_string: None,
        schedules: None,
        requirements: None,
        num_workers: None,
        num_periods: None,
        num_craftsmen: None,
        num_tasks: None,
        craftsman_avail: None,
        task_avail: None,
        num_groups: None,
        num_sectors: None,
        domain_size: None,
        relations: None,
        conjuncts_spec: None,
        relation_attrs: None,
        known_keys: None,
        num_objects: None,
        attribute_domains: None,
        frequency_tables: None,
        known_values: None,
        costs: None,
        cut_bound: None,
        size_bound: None,
        usage: None,
        storage: None,
        quantifiers: None,
        homologous_pairs: None,
        pointer_cost: None,
        expression: None,
        coeff_a: None,
        coeff_b: None,
        rhs: None,
        coeff_c: None,
        pairs: None,
        required_columns: None,
        compilers: None,
        setup_times: None,
        w_sizes: None,
        x_sizes: None,
        y_sizes: None,
        equations: None,
        assignment: None,
        initial_marking: None,
        output_arcs: None,
        gate_types: None,
        true_sentences: None,
        implications: None,
        loop_length: None,
        loop_variables: None,
        inputs: None,
        outputs: None,
        assignments: None,
        num_variables: None,
        truth_table: None,
        test_matrix: None,
        num_tests: None,
        tiles: None,
        grid_size: None,
        num_colors: None,
    }
}

#[test]
fn test_all_data_flags_empty_treats_potential_edges_as_input() {
    let mut args = empty_args();
    args.potential_edges = Some("0-2:3,1-3:5".to_string());
    assert!(!all_data_flags_empty(&args));
}

#[test]
fn test_all_data_flags_empty_treats_budget_as_input() {
    let mut args = empty_args();
    args.budget = Some("7".to_string());
    assert!(!all_data_flags_empty(&args));
}

#[test]
fn test_all_data_flags_empty_treats_max_cycle_length_as_input() {
    let mut args = empty_args();
    args.max_cycle_length = Some(4);
    assert!(!all_data_flags_empty(&args));
}

#[test]
fn test_all_data_flags_empty_treats_homologous_pairs_as_input() {
    let mut args = empty_args();
    args.homologous_pairs = Some("2=5;4=3".to_string());
    assert!(!all_data_flags_empty(&args));
}

#[test]
fn test_all_data_flags_empty_treats_job_tasks_as_input() {
    let mut args = empty_args();
    args.job_tasks = Some("0:1,1:1;1:1,0:1".to_string());
    assert!(!all_data_flags_empty(&args));
}

#[test]
fn test_parse_potential_edges() {
    let mut args = empty_args();
    args.potential_edges = Some("0-2:3,1-3:5".to_string());

    let potential_edges = parse_potential_edges(&args).unwrap();

    assert_eq!(potential_edges, vec![(0, 2, 3), (1, 3, 5)]);
}

#[test]
fn test_parse_potential_edges_rejects_missing_weight() {
    let mut args = empty_args();
    args.potential_edges = Some("0-2,1-3:5".to_string());

    let err = parse_potential_edges(&args).unwrap_err().to_string();

    assert!(err.contains("u-v:w"));
}

#[test]
fn test_parse_budget() {
    let mut args = empty_args();
    args.budget = Some("7".to_string());

    assert_eq!(parse_budget(&args).unwrap(), 7);
}

#[test]
fn test_create_disjoint_connecting_paths_json() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::graph::DisjointConnectingPaths;

    let mut args = empty_args();
    args.problem = Some("DisjointConnectingPaths".to_string());
    args.graph = Some("0-1,1-3,0-2,1-4,2-4,3-5,4-5".to_string());
    args.terminal_pairs = Some("0-3,2-5".to_string());

    let output_path = std::env::temp_dir().join(format!("dcp-create-{}.json", std::process::id()));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = std::fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "DisjointConnectingPaths");
    assert_eq!(
        created.variant,
        BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())])
    );

    let problem: DisjointConnectingPaths<SimpleGraph> =
        serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.terminal_pairs(), &[(0, 3), (2, 5)]);

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_create_disjoint_connecting_paths_rejects_overlapping_terminal_pairs() {
    let mut args = empty_args();
    args.problem = Some("DisjointConnectingPaths".to_string());
    args.graph = Some("0-1,1-2,2-3,3-4".to_string());
    args.terminal_pairs = Some("0-2,2-4".to_string());

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("pairwise disjoint"));
}

#[test]
fn test_parse_homologous_pairs() {
    let mut args = empty_args();
    args.homologous_pairs = Some("2=5;4=3".to_string());

    assert_eq!(parse_homologous_pairs(&args).unwrap(), vec![(2, 5), (4, 3)]);
}

#[test]
fn test_parse_homologous_pairs_rejects_invalid_token() {
    let mut args = empty_args();
    args.homologous_pairs = Some("2-5".to_string());

    let err = parse_homologous_pairs(&args).unwrap_err().to_string();

    assert!(err.contains("u=v"));
}

#[test]
fn test_parse_graph_respects_explicit_num_vertices() {
    let mut args = empty_args();
    args.graph = Some("0-1".to_string());
    args.num_vertices = Some(3);

    let (graph, num_vertices) = parse_graph(&args).unwrap();

    assert_eq!(num_vertices, 3);
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.edges(), vec![(0, 1)]);
}

#[test]
fn test_validate_potential_edges_rejects_existing_graph_edge() {
    let err = validate_potential_edges(&SimpleGraph::path(3), &[(0, 1, 5)])
        .unwrap_err()
        .to_string();

    assert!(err.contains("already exists in the graph"));
}

#[test]
fn test_validate_potential_edges_rejects_duplicate_edges() {
    let err = validate_potential_edges(&SimpleGraph::path(4), &[(0, 3, 1), (3, 0, 2)])
        .unwrap_err()
        .to_string();

    assert!(err.contains("Duplicate potential edge"));
}

#[test]
fn test_create_biconnectivity_augmentation_json() {
    let mut args = empty_args();
    args.graph = Some("0-1,1-2,2-3".to_string());
    args.potential_edges = Some("0-2:3,0-3:4,1-3:2".to_string());
    args.budget = Some("5".to_string());

    let output_path = std::env::temp_dir().join("pred_test_create_biconnectivity.json");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let content = std::fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "BiconnectivityAugmentation");
    assert_eq!(json["data"]["budget"], 5);
    assert_eq!(
        json["data"]["potential_weights"][0],
        serde_json::json!([0, 2, 3])
    );

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_create_biconnectivity_augmentation_json_with_isolated_vertices() {
    let mut args = empty_args();
    args.graph = Some("0-1".to_string());
    args.num_vertices = Some(3);
    args.potential_edges = Some("1-2:1".to_string());
    args.budget = Some("1".to_string());

    let output_path = std::env::temp_dir().join("pred_test_create_biconnectivity_isolated.json");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let content = std::fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let problem: BiconnectivityAugmentation<SimpleGraph, i32> =
        serde_json::from_value(json["data"].clone()).unwrap();

    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.potential_weights(), &[(1, 2, 1)]);
    assert_eq!(problem.budget(), &1);

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_create_partial_feedback_edge_set_json() {
    use problemreductions::models::graph::PartialFeedbackEdgeSet;

    let mut args = empty_args();
    args.problem = Some("PartialFeedbackEdgeSet".to_string());
    args.graph = Some("0-1,1-2,2-0".to_string());
    args.budget = Some("1".to_string());
    args.max_cycle_length = Some(3);

    let output_path = std::env::temp_dir().join("pred_test_create_partial_feedback_edge_set.json");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let content = std::fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "PartialFeedbackEdgeSet");
    assert_eq!(json["data"]["budget"], 1);
    assert_eq!(json["data"]["max_cycle_length"], 3);

    let problem: PartialFeedbackEdgeSet<SimpleGraph> =
        serde_json::from_value(json["data"].clone()).unwrap();
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.budget(), 1);
    assert_eq!(problem.max_cycle_length(), 3);

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_create_partial_feedback_edge_set_requires_max_cycle_length() {
    let mut args = empty_args();
    args.problem = Some("PartialFeedbackEdgeSet".to_string());
    args.graph = Some("0-1,1-2,2-0".to_string());
    args.budget = Some("1".to_string());

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("PartialFeedbackEdgeSet requires --max-cycle-length"));
}

#[test]
fn test_create_ensemble_computation_json() {
    let mut args = empty_args();
    args.problem = Some("EnsembleComputation".to_string());
    args.universe = Some(4);
    args.sets = Some("0,1,2;0,1,3".to_string());
    args.budget = Some("4".to_string());

    let output_path = std::env::temp_dir().join("pred_test_create_ensemble_computation.json");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let content = std::fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "EnsembleComputation");
    assert_eq!(json["data"]["universe_size"], 4);
    assert_eq!(
        json["data"]["subsets"],
        serde_json::json!([[0, 1, 2], [0, 1, 3]])
    );
    assert_eq!(json["data"]["budget"], 4);

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_create_expected_retrieval_cost_json() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::misc::ExpectedRetrievalCost;

    let mut args = empty_args();
    args.problem = Some("ExpectedRetrievalCost".to_string());
    args.probabilities = Some("0.2,0.15,0.15,0.2,0.1,0.2".to_string());
    args.num_sectors = Some(3);

    let output_path = std::env::temp_dir().join(format!(
        "expected-retrieval-cost-{}.json",
        std::process::id()
    ));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = std::fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "ExpectedRetrievalCost");

    let problem: ExpectedRetrievalCost = serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.num_records(), 6);
    assert_eq!(problem.num_sectors(), 3);
    use problemreductions::types::Min;
    assert!(matches!(
        problem.evaluate(&[0, 1, 2, 1, 0, 2]),
        Min(Some(_))
    ));

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_create_job_shop_scheduling_json() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::misc::JobShopScheduling;
    use problemreductions::traits::Problem;
    use problemreductions::types::Min;

    let mut args = empty_args();
    args.problem = Some("JobShopScheduling".to_string());
    args.job_tasks = Some("0:3,1:4;1:2,0:3,1:2;0:4,1:3;1:5,0:2;0:2,1:3,0:1".to_string());

    let output_path =
        std::env::temp_dir().join(format!("job-shop-scheduling-{}.json", std::process::id()));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = std::fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "JobShopScheduling");
    assert!(created.variant.is_empty());

    let problem: JobShopScheduling = serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.num_processors(), 2);
    assert_eq!(problem.num_jobs(), 5);
    assert_eq!(
        problem.evaluate(&[0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0]),
        Min(Some(19))
    );

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_create_job_shop_scheduling_requires_job_tasks() {
    let mut args = empty_args();
    args.problem = Some("JobShopScheduling".to_string());
    args.num_processors = Some(2);

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("JobShopScheduling requires --jobs"));
}

#[test]
fn test_create_job_shop_scheduling_rejects_malformed_operation() {
    let mut args = empty_args();
    args.problem = Some("JobShopScheduling".to_string());
    args.job_tasks = Some("0-3,1:4".to_string());

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("expected 'processor:length'"));
}

#[test]
fn test_create_job_shop_scheduling_rejects_consecutive_same_processor() {
    let mut args = empty_args();
    args.problem = Some("JobShopScheduling".to_string());
    args.job_tasks = Some("0:1,0:1".to_string());

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("must use different processors"));
}

#[test]
fn test_create_rooted_tree_storage_assignment_json() {
    let mut args = empty_args();
    args.problem = Some("RootedTreeStorageAssignment".to_string());
    args.universe = Some(5);
    args.sets = Some("0,2;1,3;0,4;2,4".to_string());
    args.bound = Some(1);

    let output_path =
        std::env::temp_dir().join("pred_test_create_rooted_tree_storage_assignment.json");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let content = std::fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "RootedTreeStorageAssignment");
    assert_eq!(json["data"]["universe_size"], 5);
    assert_eq!(
        json["data"]["subsets"],
        serde_json::json!([[0, 2], [1, 3], [0, 4], [2, 4]])
    );
    assert_eq!(json["data"]["bound"], 1);

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_create_stacker_crane_json() {
    let mut args = empty_args();
    args.problem = Some("StackerCrane".to_string());
    args.num_vertices = Some(6);
    args.arcs = Some("0>4,2>5,5>1,3>0,4>3".to_string());
    args.graph = Some("0-1,1-2,2-3,3-5,4-5,0-3,1-5".to_string());
    args.arc_costs = Some("3,4,2,5,3".to_string());
    args.edge_lengths = Some("2,1,3,2,1,4,3".to_string());

    let output_path = std::env::temp_dir().join("pred_test_create_stacker_crane.json");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let content = std::fs::read_to_string(&output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(json["type"], "StackerCrane");
    assert_eq!(json["data"]["num_vertices"], 6);
    assert_eq!(json["data"]["arcs"][0], serde_json::json!([0, 4]));
    assert_eq!(json["data"]["edge_lengths"][6], 3);

    std::fs::remove_file(output_path).ok();
}

#[test]
fn test_create_stacker_crane_rejects_mismatched_arc_lengths() {
    let mut args = empty_args();
    args.problem = Some("StackerCrane".to_string());
    args.num_vertices = Some(6);
    args.arcs = Some("0>4,2>5,5>1,3>0,4>3".to_string());
    args.graph = Some("0-1,1-2,2-3,3-5,4-5,0-3,1-5".to_string());
    args.arc_costs = Some("3,4,2,5".to_string());
    args.edge_lengths = Some("2,1,3,2,1,4,3".to_string());
    args.bound = Some(20);

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("Expected 5 arc costs but got 4"));
}

#[test]
fn test_create_stacker_crane_rejects_out_of_range_vertices() {
    let mut args = empty_args();
    args.problem = Some("StackerCrane".to_string());
    args.num_vertices = Some(5);
    args.arcs = Some("0>4,2>5,5>1,3>0,4>3".to_string());
    args.graph = Some("0-1,1-2,2-3,3-5,4-5,0-3,1-5".to_string());
    args.arc_costs = Some("3,4,2,5,3".to_string());
    args.edge_lengths = Some("2,1,3,2,1,4,3".to_string());
    args.bound = Some(20);

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("--num-vertices (5) is too small for the arcs"));
}

#[test]
fn test_create_minimum_dummy_activities_pert_json() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::graph::MinimumDummyActivitiesPert;

    let mut args = empty_args();
    args.problem = Some("MinimumDummyActivitiesPert".to_string());
    args.num_vertices = Some(6);
    args.arcs = Some("0>2,0>3,1>3,1>4,2>5".to_string());

    let output_path = temp_output_path("minimum_dummy_activities_pert");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "MinimumDummyActivitiesPert");
    assert!(created.variant.is_empty());

    let problem: MinimumDummyActivitiesPert = serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 5);

    let _ = fs::remove_file(output_path);
}

#[test]
fn test_create_minimum_dummy_activities_pert_rejects_cycles() {
    let mut args = empty_args();
    args.problem = Some("MinimumDummyActivitiesPert".to_string());
    args.num_vertices = Some(3);
    args.arcs = Some("0>1,1>2,2>0".to_string());

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("requires the input graph to be a DAG"));
}

#[test]
fn test_create_balanced_complete_bipartite_subgraph() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::graph::BalancedCompleteBipartiteSubgraph;

    let mut args = empty_args();
    args.problem = Some("BalancedCompleteBipartiteSubgraph".to_string());
    args.biedges = Some("0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3".to_string());
    args.left = Some(4);
    args.right = Some(4);
    args.k = Some(3);
    args.graph = None;

    let output_path = std::env::temp_dir().join(format!("bcbs-create-{}.json", std::process::id()));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = std::fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "BalancedCompleteBipartiteSubgraph");
    assert!(created.variant.is_empty());

    let problem: BalancedCompleteBipartiteSubgraph = serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.left_size(), 4);
    assert_eq!(problem.right_size(), 4);
    assert_eq!(problem.num_edges(), 12);
    assert_eq!(problem.k(), 3);

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_create_balanced_complete_bipartite_subgraph_rejects_out_of_range_biedges() {
    let mut args = empty_args();
    args.problem = Some("BalancedCompleteBipartiteSubgraph".to_string());
    args.biedges = Some("4-0".to_string());
    args.left = Some(4);
    args.right = Some(4);
    args.k = Some(3);
    args.graph = None;

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("out of bounds for left partition size 4"));
}

#[test]
fn test_create_kclique() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::graph::KClique;

    let mut args = empty_args();
    args.problem = Some("KClique".to_string());
    args.graph = Some("0-1,0-2,1-3,2-3,2-4,3-4".to_string());
    args.k = Some(3);

    let output_path =
        std::env::temp_dir().join(format!("kclique-create-{}.json", std::process::id()));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = std::fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "KClique");
    assert_eq!(
        created.variant.get("graph").map(String::as_str),
        Some("SimpleGraph")
    );

    let problem: KClique<SimpleGraph> = serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.k(), 3);
    assert_eq!(problem.num_vertices(), 5);
    assert!(problem.evaluate(&[0, 0, 1, 1, 1]));

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_create_kclique_requires_valid_k() {
    let mut args = empty_args();
    args.problem = Some("KClique".to_string());
    args.graph = Some("0-1,0-2,1-3,2-3,2-4,3-4".to_string());
    args.k = None;

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err();
    assert!(
        err.to_string().contains("KClique requires --k"),
        "unexpected error: {err}"
    );

    args.k = Some(6);
    let err = create(&args, &out).unwrap_err();
    assert!(
        err.to_string().contains("k must be <= graph num_vertices"),
        "unexpected error: {err}"
    );
}

#[test]
fn test_create_sparse_matrix_compression_json() {
    use crate::dispatch::ProblemJsonOutput;

    let mut args = empty_args();
    args.problem = Some("SparseMatrixCompression".to_string());
    args.matrix = Some("1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0".to_string());
    args.bound = Some(2);

    let output_path = std::env::temp_dir().join(format!("smc-create-{}.json", std::process::id()));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = std::fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "SparseMatrixCompression");
    assert!(created.variant.is_empty());
    assert_eq!(
        created.data,
        serde_json::json!({
            "matrix": [
                [true, false, false, true],
                [false, true, false, false],
                [false, false, true, false],
                [true, false, false, false],
            ],
            "bound_k": 2,
        })
    );

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_create_sparse_matrix_compression_requires_bound() {
    let mut args = empty_args();
    args.problem = Some("SparseMatrixCompression".to_string());
    args.matrix = Some("1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0".to_string());

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("SparseMatrixCompression requires --matrix and --bound"));
    assert!(err.contains("Usage: pred create SparseMatrixCompression"));
}

#[test]
fn test_create_sparse_matrix_compression_rejects_zero_bound() {
    let mut args = empty_args();
    args.problem = Some("SparseMatrixCompression".to_string());
    args.matrix = Some("1,0;0,1".to_string());
    args.bound = Some(0);

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("bound >= 1"));
}

#[test]
fn test_create_graph_partitioning_with_num_partitions() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::graph::GraphPartitioning;
    use problemreductions::topology::SimpleGraph;

    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "GraphPartitioning",
        "--graph",
        "0-1,1-2,2-3,3-0",
        "--num-partitions",
        "2",
    ])
    .unwrap();
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let output_path = temp_output_path("graph-partitioning-create");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "GraphPartitioning");
    let problem: GraphPartitioning<SimpleGraph> = serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.num_vertices(), 4);

    let _ = fs::remove_file(output_path);
}

#[test]
fn test_create_nontautology_with_disjuncts_flag() {
    use crate::dispatch::ProblemJsonOutput;
    use problemreductions::models::formula::NonTautology;

    let cli = Cli::try_parse_from([
        "pred",
        "create",
        "NonTautology",
        "--num-vars",
        "3",
        "--disjuncts",
        "1,2,3;-1,-2,-3",
    ])
    .unwrap();
    let args = match cli.command {
        Commands::Create(args) => args,
        _ => unreachable!(),
    };

    let output_path = temp_output_path("non-tautology-create");
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "NonTautology");
    let problem: NonTautology = serde_json::from_value(created.data).unwrap();
    assert_eq!(problem.disjuncts(), &[vec![1, 2, 3], vec![-1, -2, -3]]);

    let _ = fs::remove_file(output_path);
}

#[test]
fn test_create_consecutive_ones_matrix_augmentation_json() {
    use crate::dispatch::ProblemJsonOutput;

    let mut args = empty_args();
    args.problem = Some("ConsecutiveOnesMatrixAugmentation".to_string());
    args.matrix = Some("1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0".to_string());
    args.bound = Some(2);

    let output_path = std::env::temp_dir().join(format!("coma-create-{}.json", std::process::id()));
    let out = OutputConfig {
        output: Some(output_path.clone()),
        quiet: true,
        json: false,
        auto_json: false,
    };

    create(&args, &out).unwrap();

    let json = std::fs::read_to_string(&output_path).unwrap();
    let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(created.problem_type, "ConsecutiveOnesMatrixAugmentation");
    assert!(created.variant.is_empty());
    assert_eq!(
        created.data,
        serde_json::json!({
            "matrix": [
                [true, false, false, true, true],
                [true, true, false, false, false],
                [false, true, true, false, true],
                [false, false, true, true, false],
            ],
            "bound": 2,
        })
    );

    let _ = std::fs::remove_file(output_path);
}

#[test]
fn test_create_consecutive_ones_matrix_augmentation_requires_bound() {
    let mut args = empty_args();
    args.problem = Some("ConsecutiveOnesMatrixAugmentation".to_string());
    args.matrix = Some("1,0;0,1".to_string());

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("ConsecutiveOnesMatrixAugmentation requires --matrix and --bound"));
    assert!(err.contains("Usage: pred create ConsecutiveOnesMatrixAugmentation"));
}

#[test]
fn test_create_consecutive_ones_matrix_augmentation_negative_bound() {
    let mut args = empty_args();
    args.problem = Some("ConsecutiveOnesMatrixAugmentation".to_string());
    args.matrix = Some("1,0;0,1".to_string());
    args.bound = Some(-1);

    let out = OutputConfig {
        output: None,
        quiet: true,
        json: false,
        auto_json: false,
    };

    let err = create(&args, &out).unwrap_err().to_string();
    assert!(err.contains("nonnegative"));
}
