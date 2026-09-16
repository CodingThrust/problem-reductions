use super::*;
use crate::models::formula::{CNFClause, NAESatisfiability};
use crate::models::graph::PartitionIntoPerfectMatchings;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::solvers::SolveOutcome;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn sorted_edges(graph: &SimpleGraph) -> Vec<(usize, usize)> {
    let mut edges = graph.edges();
    edges.sort_unstable();
    edges
}

fn yes_example_problem() -> NAESatisfiability {
    NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    )
}

fn no_example_problem() -> NAESatisfiability {
    NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, 3]),
        ],
    )
}

#[test]
fn test_naesatisfiability_to_partitionintoperfectmatchings_closed_loop() {
    let source = NAESatisfiability::new(1, vec![]);
    let reduction = ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_eq!(reduction.target_problem().num_vertices(), 4);
    assert_eq!(reduction.target_problem().num_edges(), 3);
    assert_eq!(reduction.target_problem().num_matchings(), 2);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "NAE-SAT -> PartitionIntoPerfectMatchings empty-formula closed loop",
    );
}

#[test]
fn test_naesatisfiability_to_partitionintoperfectmatchings_unsat_small_instance() {
    let source = NAESatisfiability::new(1, vec![CNFClause::new(vec![1, 1])]);
    let reduction = ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert!(BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .is_none());
}

#[test]
fn test_naesatisfiability_to_partitionintoperfectmatchings_yes_example_structure() {
    let source = yes_example_problem();
    let reduction = ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let mut expected_edges = vec![
        (0, 1),
        (2, 3),
        (0, 2),
        (4, 5),
        (6, 7),
        (4, 6),
        (8, 9),
        (10, 11),
        (8, 10),
        (12, 13),
        (14, 15),
        (16, 17),
        (18, 19),
        (20, 21),
        (22, 23),
        (24, 25),
        (24, 26),
        (24, 27),
        (25, 26),
        (25, 27),
        (26, 27),
        (12, 24),
        (14, 25),
        (16, 26),
        (28, 29),
        (28, 30),
        (28, 31),
        (29, 30),
        (29, 31),
        (30, 31),
        (18, 28),
        (20, 29),
        (22, 30),
        (32, 33),
        (0, 32),
        (12, 32),
        (34, 35),
        (2, 34),
        (18, 34),
        (36, 37),
        (4, 36),
        (14, 36),
        (38, 39),
        (14, 38),
        (20, 38),
        (40, 41),
        (8, 40),
        (16, 40),
        (42, 43),
        (10, 42),
        (22, 42),
    ];
    expected_edges.sort_unstable();

    assert_eq!(target.num_vertices(), 44);
    assert_eq!(target.num_edges(), 51);
    assert_eq!(target.num_matchings(), 2);
    assert_eq!(sorted_edges(target.graph()), expected_edges);
}

#[test]
fn test_naesatisfiability_to_partitionintoperfectmatchings_no_example_structure() {
    let source = no_example_problem();
    let reduction = ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let mut expected_edges = vec![
        (0, 1),
        (2, 3),
        (0, 2),
        (4, 5),
        (6, 7),
        (4, 6),
        (8, 9),
        (10, 11),
        (8, 10),
        (12, 13),
        (14, 15),
        (16, 17),
        (18, 19),
        (20, 21),
        (22, 23),
        (24, 25),
        (26, 27),
        (28, 29),
        (30, 31),
        (32, 33),
        (34, 35),
        (36, 37),
        (36, 38),
        (36, 39),
        (37, 38),
        (37, 39),
        (38, 39),
        (12, 36),
        (14, 37),
        (16, 38),
        (40, 41),
        (40, 42),
        (40, 43),
        (41, 42),
        (41, 43),
        (42, 43),
        (18, 40),
        (20, 41),
        (22, 42),
        (44, 45),
        (44, 46),
        (44, 47),
        (45, 46),
        (45, 47),
        (46, 47),
        (24, 44),
        (26, 45),
        (28, 46),
        (48, 49),
        (48, 50),
        (48, 51),
        (49, 50),
        (49, 51),
        (50, 51),
        (30, 48),
        (32, 49),
        (34, 50),
        (52, 53),
        (0, 52),
        (12, 52),
        (54, 55),
        (12, 54),
        (18, 54),
        (56, 57),
        (18, 56),
        (24, 56),
        (58, 59),
        (2, 58),
        (30, 58),
        (60, 61),
        (4, 60),
        (14, 60),
        (62, 63),
        (14, 62),
        (20, 62),
        (64, 65),
        (20, 64),
        (32, 64),
        (66, 67),
        (6, 66),
        (26, 66),
        (68, 69),
        (8, 68),
        (16, 68),
        (70, 71),
        (16, 70),
        (28, 70),
        (72, 73),
        (28, 72),
        (34, 72),
        (74, 75),
        (10, 74),
        (22, 74),
    ];
    expected_edges.sort_unstable();

    assert_eq!(target.num_vertices(), 76);
    assert_eq!(target.num_edges(), 93);
    assert_eq!(target.num_matchings(), 2);
    assert_eq!(sorted_edges(target.graph()), expected_edges);
}

#[test]
fn test_naesatisfiability_to_partitionintoperfectmatchings_constructed_witness_round_trips() {
    let source = yes_example_problem();
    let source_solution = vec![true, true, false];
    let reduction = ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target_solution = reduction.construct_target_solution(&source_solution);

    assert!(source.evaluate(&source_solution).unwrap());
    assert!(reduction
        .target_problem()
        .evaluate(&target_solution)
        .unwrap());
    assert_eq!(
        reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), target_solution.clone()).unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        source_solution
    );
}

#[test]
fn test_naesatisfiability_to_partitionintoperfectmatchings_two_literal_clause_normalization() {
    let source = NAESatisfiability::new(2, vec![CNFClause::new(vec![1, -2])]);
    let reduction = ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let source_solution = vec![true, true];
    let target_solution = reduction.construct_target_solution(&source_solution);

    assert_eq!(target.num_vertices(), 24);
    assert_eq!(target.num_edges(), 27);
    assert_eq!(target.num_matchings(), 2);
    assert!(target.evaluate(&target_solution).unwrap());
    assert_eq!(
        reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), target_solution.clone()).unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        source_solution
    );
}

#[test]
fn long_nae_clauses_preserve_assignments_with_auxiliary_variables() {
    let entry = inventory::iter::<crate::rules::ReductionEntry>
        .into_iter()
        .find(|e| {
            e.source_name == NAESatisfiability::NAME
                && e.target_name == PartitionIntoPerfectMatchings::<SimpleGraph>::NAME
        })
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    let transform = contract.transform().unwrap();
    for literals in [vec![1, 2, 3, 4], vec![1, -2, 1, 3, -4, 2], vec![1, 1, 1, 1]] {
        let source = NAESatisfiability::new(4, vec![CNFClause::new(literals)]);
        let reduction =
            ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source).unwrap();
        let layout = &reduction.layout;
        let bound = transform.evaluate(&source.parameters()).unwrap();
        assert!(layout.num_vertices as u64 <= bound.get("num_vertices").unwrap());
        assert!(layout.edges.len() as u64 <= bound.get("num_edges").unwrap());
        for bits in 0..16 {
            let assignment: Vec<_> = (0..4).map(|i| bits & (1 << i) != 0).collect();
            let mut extendible = false;
            for aux in 0..(1 << (layout.variables.len() - 4)) {
                let mut extended = assignment.clone();
                extended.extend((0..layout.variables.len() - 4).map(|i| aux & (1 << i) != 0));
                let satisfies = layout.clauses.iter().all(|clause| {
                    let values = clause
                        .literals
                        .map(|l| extended[l.unsigned_abs() as usize - 1] == (l > 0));
                    values.iter().any(|&v| v) && values.iter().any(|&v| !v)
                });
                extendible |= satisfies;
            }
            assert_eq!(extendible, source.evaluate(&assignment).unwrap().0);
            if extendible {
                let target = reduction.construct_target_solution(&assignment);
                let recovered = reduction
                    .recover_result(
                        &source,
                        SolveOutcome::optimal(reduction.target_problem(), target).unwrap(),
                    )
                    .unwrap()
                    .into_solution()
                    .unwrap();
                assert_eq!(recovered, assignment);
            }
        }
    }
}

#[cfg(feature = "example-db")]
#[test]
fn test_naesatisfiability_to_partitionintoperfectmatchings_canonical_example_spec() {
    let specs =
        crate::rules::naesatisfiability_partitionintoperfectmatchings::canonical_rule_example_specs(
        );
    assert_eq!(specs.len(), 1);

    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "NAESatisfiability");
    assert_eq!(example.target.problem, "PartitionIntoPerfectMatchings");
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(
        example.solutions[0].source_config,
        serde_json::json!([true, true, false])
    );
}
