// Each example is included as a module and tested directly (no subprocess overhead).
// Individual #[test] functions let cargo's test harness run them in parallel.

macro_rules! example_test {
    ($mod_name:ident) => {
        #[allow(unused)]
        mod $mod_name {
            include!(concat!("../../examples/", stringify!($mod_name), ".rs"));
        }
    };
}

example_test!(chained_reduction_factoring_to_spinglass);
example_test!(chained_reduction_ksat_to_mis);
example_test!(reduction_circuitsat_to_spinglass);
example_test!(reduction_factoring_to_circuitsat);
example_test!(reduction_factoring_to_ilp);
example_test!(reduction_ilp_to_qubo);
example_test!(reduction_kcoloring_to_ilp);
example_test!(reduction_kcoloring_to_qubo);
example_test!(reduction_ksatisfiability_to_qubo);
example_test!(reduction_maxcut_to_spinglass);
example_test!(reduction_maximumclique_to_ilp);
example_test!(reduction_maximumindependentset_to_ilp);
example_test!(reduction_maximumindependentset_to_maximumsetpacking);
example_test!(reduction_maximumindependentset_to_minimumvertexcover);
example_test!(reduction_maximumindependentset_to_qubo);
example_test!(reduction_maximummatching_to_ilp);
example_test!(reduction_maximummatching_to_maximumsetpacking);
example_test!(reduction_maximumsetpacking_to_ilp);
example_test!(reduction_maximumsetpacking_to_qubo);
example_test!(reduction_minimumdominatingset_to_ilp);
example_test!(reduction_minimumsetcovering_to_ilp);
example_test!(reduction_minimumvertexcover_to_ilp);
example_test!(reduction_minimumvertexcover_to_maximumindependentset);
example_test!(reduction_minimumvertexcover_to_minimumsetcovering);
example_test!(reduction_minimumvertexcover_to_qubo);
example_test!(reduction_qubo_to_spinglass);
example_test!(reduction_satisfiability_to_kcoloring);
example_test!(reduction_satisfiability_to_ksatisfiability);
example_test!(reduction_satisfiability_to_maximumindependentset);
example_test!(reduction_satisfiability_to_minimumdominatingset);
example_test!(reduction_spinglass_to_maxcut);
example_test!(reduction_spinglass_to_qubo);
example_test!(reduction_travelingsalesman_to_ilp);

macro_rules! example_fn {
    ($test_name:ident, $mod_name:ident) => {
        #[test]
        fn $test_name() {
            $mod_name::run();
        }
    };
}

example_fn!(
    test_chained_reduction_factoring_to_spinglass,
    chained_reduction_factoring_to_spinglass
);
example_fn!(
    test_chained_reduction_ksat_to_mis,
    chained_reduction_ksat_to_mis
);
example_fn!(
    test_circuitsat_to_spinglass,
    reduction_circuitsat_to_spinglass
);
example_fn!(
    test_factoring_to_circuitsat,
    reduction_factoring_to_circuitsat
);
example_fn!(test_factoring_to_ilp, reduction_factoring_to_ilp);
example_fn!(test_ilp_to_qubo, reduction_ilp_to_qubo);
example_fn!(test_kcoloring_to_ilp, reduction_kcoloring_to_ilp);
example_fn!(test_kcoloring_to_qubo, reduction_kcoloring_to_qubo);
example_fn!(
    test_ksatisfiability_to_qubo,
    reduction_ksatisfiability_to_qubo
);
example_fn!(test_maxcut_to_spinglass, reduction_maxcut_to_spinglass);
example_fn!(test_maximumclique_to_ilp, reduction_maximumclique_to_ilp);
example_fn!(
    test_maximumindependentset_to_ilp,
    reduction_maximumindependentset_to_ilp
);
example_fn!(
    test_maximumindependentset_to_maximumsetpacking,
    reduction_maximumindependentset_to_maximumsetpacking
);
example_fn!(
    test_maximumindependentset_to_minimumvertexcover,
    reduction_maximumindependentset_to_minimumvertexcover
);
example_fn!(
    test_maximumindependentset_to_qubo,
    reduction_maximumindependentset_to_qubo
);
example_fn!(
    test_maximummatching_to_ilp,
    reduction_maximummatching_to_ilp
);
example_fn!(
    test_maximummatching_to_maximumsetpacking,
    reduction_maximummatching_to_maximumsetpacking
);
example_fn!(
    test_maximumsetpacking_to_ilp,
    reduction_maximumsetpacking_to_ilp
);
example_fn!(
    test_maximumsetpacking_to_qubo,
    reduction_maximumsetpacking_to_qubo
);
example_fn!(
    test_minimumdominatingset_to_ilp,
    reduction_minimumdominatingset_to_ilp
);
example_fn!(
    test_minimumsetcovering_to_ilp,
    reduction_minimumsetcovering_to_ilp
);
example_fn!(
    test_minimumvertexcover_to_ilp,
    reduction_minimumvertexcover_to_ilp
);
example_fn!(
    test_minimumvertexcover_to_maximumindependentset,
    reduction_minimumvertexcover_to_maximumindependentset
);
example_fn!(
    test_minimumvertexcover_to_minimumsetcovering,
    reduction_minimumvertexcover_to_minimumsetcovering
);
example_fn!(
    test_minimumvertexcover_to_qubo,
    reduction_minimumvertexcover_to_qubo
);
example_fn!(test_qubo_to_spinglass, reduction_qubo_to_spinglass);
example_fn!(
    test_satisfiability_to_kcoloring,
    reduction_satisfiability_to_kcoloring
);
example_fn!(
    test_satisfiability_to_ksatisfiability,
    reduction_satisfiability_to_ksatisfiability
);
example_fn!(
    test_satisfiability_to_maximumindependentset,
    reduction_satisfiability_to_maximumindependentset
);
example_fn!(
    test_satisfiability_to_minimumdominatingset,
    reduction_satisfiability_to_minimumdominatingset
);
example_fn!(test_spinglass_to_maxcut, reduction_spinglass_to_maxcut);
example_fn!(test_spinglass_to_qubo, reduction_spinglass_to_qubo);
example_fn!(
    test_travelingsalesman_to_ilp,
    reduction_travelingsalesman_to_ilp
);
