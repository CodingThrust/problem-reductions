use std::process::Command;

fn run_example(name: &str) {
    let output = Command::new("cargo")
        .args(["run", "--all-features", "--example", name])
        .output()
        .unwrap_or_else(|e| panic!("Failed to execute example {}: {}", name, e));

    assert!(
        output.status.success(),
        "Example {} failed with status {:?}\nstdout: {}\nstderr: {}",
        name,
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
#[ignore]
fn test_all_reduction_examples() {
    let examples = [
        "reduction_circuit_to_spinglass",
        "reduction_clique_to_ilp",
        "reduction_coloring_to_ilp",
        "reduction_coloring_to_qubo",
        "reduction_dominatingset_to_ilp",
        "reduction_factoring_to_circuit",
        "reduction_factoring_to_ilp",
        "reduction_ilp_to_qubo",
        "reduction_is_to_ilp",
        "reduction_is_to_qubo",
        "reduction_is_to_setpacking",
        "reduction_is_to_vc",
        "reduction_ksatisfiability_to_qubo",
        "reduction_matching_to_ilp",
        "reduction_matching_to_setpacking",
        "reduction_maxcut_to_spinglass",
        "reduction_qubo_to_spinglass",
        "reduction_sat_to_coloring",
        "reduction_sat_to_dominatingset",
        "reduction_sat_to_is",
        "reduction_sat_to_ksat",
        "reduction_setcovering_to_ilp",
        "reduction_setpacking_to_ilp",
        "reduction_setpacking_to_qubo",
        "reduction_spinglass_to_maxcut",
        "reduction_spinglass_to_qubo",
        "reduction_vc_to_ilp",
        "reduction_vc_to_is",
        "reduction_vc_to_qubo",
        "reduction_vc_to_setcovering",
    ];

    for name in &examples {
        run_example(name);
    }
}
