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
fn test_all_reduction_examples() {
    let examples = [
        "reduction_circuitsat_to_spinglass",
        "reduction_maximumclique_to_ilp",
        "reduction_kcoloring_to_ilp",
        "reduction_kcoloring_to_qubo",
        "reduction_minimumdominatingset_to_ilp",
        "reduction_factoring_to_circuitsat",
        "reduction_factoring_to_ilp",
        "reduction_ilp_to_qubo",
        "reduction_maximumindependentset_to_ilp",
        "reduction_maximumindependentset_to_qubo",
        "reduction_maximumindependentset_to_maximumsetpacking",
        "reduction_maximumindependentset_to_minimumvertexcover",
        "reduction_ksatisfiability_to_qubo",
        "reduction_maximummatching_to_ilp",
        "reduction_maximummatching_to_maximumsetpacking",
        "reduction_maxcut_to_spinglass",
        "reduction_qubo_to_spinglass",
        "reduction_satisfiability_to_kcoloring",
        "reduction_satisfiability_to_minimumdominatingset",
        "reduction_satisfiability_to_maximumindependentset",
        "reduction_satisfiability_to_ksatisfiability",
        "reduction_minimumsetcovering_to_ilp",
        "reduction_maximumsetpacking_to_ilp",
        "reduction_maximumsetpacking_to_qubo",
        "reduction_spinglass_to_maxcut",
        "reduction_spinglass_to_qubo",
        "reduction_minimumvertexcover_to_ilp",
        "reduction_minimumvertexcover_to_maximumindependentset",
        "reduction_minimumvertexcover_to_qubo",
        "reduction_minimumvertexcover_to_minimumsetcovering",
    ];

    for name in &examples {
        run_example(name);
    }
}
