use super::*;
use crate::rules::ReductionEntry;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn five_cycle_with_chord() -> HamiltonianCircuit<SimpleGraph> {
    HamiltonianCircuit::new(SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (0, 2)],
    ))
}

fn position_assignment(circuit: &[usize]) -> Vec<usize> {
    let n = circuit.len();
    let mut assignment = vec![0; n * n];
    for (position, &vertex) in circuit.iter().enumerate() {
        assignment[vertex * n + position] = 1;
    }
    assignment
}

fn permutations(values: &mut [usize], start: usize, visit: &mut impl FnMut(&[usize])) {
    if start == values.len() {
        visit(values);
        return;
    }
    for index in start..values.len() {
        values.swap(start, index);
        permutations(values, start + 1, visit);
        values.swap(start, index);
    }
}

#[test]
fn test_hamiltoniancircuit_to_satisfiability_closed_loop() {
    let source = HamiltonianCircuit::new(SimpleGraph::cycle(3));
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
    let target_solution = BruteForce::new()
        .find_witness(reduction.target_problem())
        .expect("the triangle encoding must be satisfiable");
    let source_solution = reduction.extract_solution(&target_solution);

    assert!(source.evaluate(&source_solution));
}

#[test]
fn test_five_cycle_with_chord_structure() {
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&five_cycle_with_chord());
    let target = reduction.target_problem();

    assert_eq!(target.num_vars(), 25);
    assert_eq!(target.num_clauses(), 175);
    assert_eq!(target.num_literals(), 380);
    assert_eq!(target.clauses()[0], CNFClause::new(vec![1, 6, 11, 16, 21]));
    assert_eq!(target.clauses()[1], CNFClause::new(vec![-1, -6]));
    assert_eq!(target.clauses()[55], CNFClause::new(vec![1, 2, 3, 4, 5]));

    let clauses = target.clauses();
    assert!(clauses.contains(&CNFClause::new(vec![-1, -2])));
    assert!(clauses.contains(&CNFClause::new(vec![-1, -17])));
    assert!(clauses.contains(&CNFClause::new(vec![-10, -16])));
}

#[test]
fn test_extracts_both_circuit_orientations() {
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&five_cycle_with_chord());

    for circuit in [vec![0, 1, 2, 3, 4], vec![0, 4, 3, 2, 1]] {
        let assignment = position_assignment(&circuit);
        assert!(reduction.target_problem().evaluate(&assignment));
        assert_eq!(reduction.extract_solution(&assignment), circuit);
    }
}

#[test]
fn test_five_vertex_path_encoding_is_unsatisfiable() {
    let source = HamiltonianCircuit::new(SimpleGraph::path(5));
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
    let mut circuit = vec![0, 1, 2, 3, 4];
    permutations(&mut circuit, 0, &mut |candidate| {
        assert!(!reduction
            .target_problem()
            .evaluate(&position_assignment(candidate)));
    });
}

#[test]
fn test_instances_with_fewer_than_three_vertices_are_unsatisfiable() {
    for n in 0..3 {
        let source = HamiltonianCircuit::new(SimpleGraph::empty(n));
        let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
        let target = reduction.target_problem();

        assert_eq!(target.num_vars(), 1);
        assert_eq!(
            target.clauses(),
            &[CNFClause::new(vec![1]), CNFClause::new(vec![-1])]
        );
        assert!(BruteForce::new().find_witness(target).is_none());
    }
}

#[test]
fn test_isolated_vertex_blocks_satisfiability() {
    let source = HamiltonianCircuit::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 0)]));
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
    let mut circuit = vec![0, 1, 2, 3];
    permutations(&mut circuit, 0, &mut |candidate| {
        assert!(!reduction
            .target_problem()
            .evaluate(&position_assignment(candidate)));
    });
}

#[test]
fn test_self_loops_do_not_allow_repeated_vertices() {
    let source = HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 0), (0, 1), (1, 2), (2, 0)]));
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert!(reduction
        .target_problem()
        .clauses()
        .contains(&CNFClause::new(vec![-1, -2])));
    assert!(reduction
        .target_problem()
        .evaluate(&position_assignment(&[0, 1, 2])));
}

#[test]
fn test_parallel_edges_have_simple_adjacency_semantics() {
    let source = HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 1), (0, 1), (1, 2), (2, 0)]));
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
    let assignment = position_assignment(&[0, 1, 2]);

    assert!(reduction.target_problem().evaluate(&assignment));
    assert_eq!(reduction.extract_solution(&assignment), vec![0, 1, 2]);
}

#[test]
fn test_registered_overhead_covers_constructed_target() {
    let entry = inventory::iter::<ReductionEntry>
        .into_iter()
        .find(|entry| {
            entry.source_name == "HamiltonianCircuit" && entry.target_name == "Satisfiability"
        })
        .expect("HamiltonianCircuit to Satisfiability must be registered");
    let source = five_cycle_with_chord();
    let overhead = (entry.overhead_eval_fn)(&source as &dyn std::any::Any);

    assert_eq!(overhead.get("num_vars"), Some(26));
    assert_eq!(overhead.get("num_clauses"), Some(237));
    assert_eq!(overhead.get("num_literals"), Some(502));

    for source in (0..3)
        .map(|n| HamiltonianCircuit::new(SimpleGraph::empty(n)))
        .chain(std::iter::once(source))
    {
        let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
        let target = reduction.target_problem();
        let overhead = (entry.overhead_eval_fn)(&source as &dyn std::any::Any);
        assert!(target.num_vars() <= overhead.get("num_vars").unwrap());
        assert!(target.num_clauses() <= overhead.get("num_clauses").unwrap());
        assert!(target.num_literals() <= overhead.get("num_literals").unwrap());
    }
}

#[cfg(feature = "example-db")]
#[test]
fn test_canonical_example_witness_and_metrics() {
    let example = (canonical_rule_example_specs()[0].build)();
    let source: HamiltonianCircuit<SimpleGraph> =
        serde_json::from_value(example.source.instance).unwrap();
    let target: Satisfiability = serde_json::from_value(example.target.instance).unwrap();
    let solution = &example.solutions[0];
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert_eq!(solution.source_config, vec![0, 1, 2, 3, 4]);
    assert_eq!(
        solution.target_config,
        position_assignment(&[0, 1, 2, 3, 4])
    );
    assert!(source.evaluate(&solution.source_config));
    assert!(target.evaluate(&solution.target_config));
    assert_eq!(
        reduction.extract_solution(&solution.target_config),
        solution.source_config
    );
    assert_eq!(target.num_vars(), 25);
    assert_eq!(target.num_clauses(), 175);
    assert_eq!(target.num_literals(), 380);
}
