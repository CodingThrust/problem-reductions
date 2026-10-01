use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::AcyclicPartition;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn small_instance() -> AcyclicPartition<i64> {
    // Chain 0->1->2->3, unit weights, unit arc costs, B=3, K=2
    AcyclicPartition::new(
        DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1, 1],
        vec![1, 1, 1],
        3,
        2,
    )
}

#[test]
fn crossing_budget_bounds_the_number_of_occupied_parts() {
    let graph = DirectedGraph::new(4, vec![(0, 1), (1, 3), (0, 2), (2, 3), (1, 2)]);
    let source = AcyclicPartition::new(
        graph.clone(),
        vec![8, 1, 1, 8],
        vec![10, 8, 10, 8, 1],
        10,
        17,
    );
    let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    // Anchors cannot share a part. The two paths already cost at least 16;
    // putting any interior vertex in a third part adds at least 10.
    assert_eq!(reduction.target_problem().num_vars(), 4);
    for bound in [9, 10] {
        for budget in [15, 16, 17, 19, 25, 26] {
            let source = AcyclicPartition::new(
                graph.clone(),
                vec![8, 1, 1, 8],
                vec![10, 8, 10, 8, 1],
                bound,
                budget,
            );
            let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
            let expected = BruteForce::new().solve(&source).unwrap();
            match ILPSolver::new().solve(reduction.target_problem()) {
                Ok(solution) => {
                    assert!(expected.is_some());
                    assert!(
                        source
                            .evaluate(&reduction.extract_solution(&solution).unwrap())
                            .unwrap()
                            .0
                    );
                }
                Err(crate::solvers::ILPSolveError::Infeasible) => assert!(expected.is_none()),
                Err(error) => panic!("{error}"),
            }
        }
    }
}

#[test]
fn fixed_vertex_cardinality_preserves_every_two_part_witness() {
    let mut arcs = Vec::new();
    let mut costs = Vec::new();
    for (v, profit) in [(1, 2), (2, 3), (3, 1), (4, 0), (5, 0), (6, 0)] {
        arcs.extend([(0, v), (v, 7)]);
        costs.extend([20, 20 - profit]);
    }
    // Two distinct neighbor pairs, with two separate items for pair (1,2).
    // Both duplicate items can belong to the selected part simultaneously.
    arcs.extend([(1, 4), (2, 4), (2, 5), (3, 5), (1, 6), (2, 6)]);
    costs.extend([1; 6]);
    let source = AcyclicPartition::new(
        DirectedGraph::new(8, arcs),
        vec![8, 1, 1, 1, 0, 0, 0, 9],
        costs,
        10,
        118,
    );
    let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
    for mask in 0..64 {
        let labels = std::iter::once(0)
            .chain((0..6).map(|v| (mask >> v) & 1))
            .chain([1])
            .collect::<Vec<_>>();
        let assignment = labels
            .iter()
            .map(|&label| i64::try_from(label).unwrap())
            .collect();
        assert_eq!(
            reduction
                .target_problem()
                .evaluate(&assignment)
                .unwrap()
                .value
                .is_some(),
            source.evaluate(&labels).unwrap().0,
            "partition {labels:?}"
        );
    }
}

#[test]
fn test_acyclicpartition_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionAcyclicPartitionToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    // Solve source with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&source).unwrap();
    assert!(!bf_solutions.is_empty(), "source should be satisfiable");

    // Solve ILP
    let ilp_solver = ILPSolver::new();
    let ilp_sol = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol).unwrap();

    assert!(
        source.evaluate(&extracted).unwrap().0,
        "extracted solution must be valid"
    );
}

#[test]
fn test_reduction_num_vars() {
    let source = small_instance();
    let reduction: ReductionAcyclicPartitionToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars(), 27);
    assert_eq!(ilp.num_constraints(), 39);
}

#[test]
fn signed_partition_weights_and_costs_are_checked_after_summing() {
    for (source, witness) in [
        (
            AcyclicPartition::new(DirectedGraph::new(2, vec![]), vec![2, -3], vec![], -1, 0),
            vec![0, 0],
        ),
        (
            AcyclicPartition::new(
                DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![1; 3],
                vec![3, -4],
                1,
                -1,
            ),
            vec![0, 1, 2],
        ),
    ] {
        assert!(source.evaluate(&witness).unwrap().0);
        let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
        crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
    }
    let empty = AcyclicPartition::new(DirectedGraph::new(0, vec![]), vec![], vec![], 0, -1);
    assert!(!empty.evaluate(&vec![]).unwrap().0);
    let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&empty).unwrap();
    assert!(ILPSolver::new().solve(reduction.target_problem()).is_err());
}

#[test]
fn test_extract_solution() {
    let source = small_instance();
    let reduction: ReductionAcyclicPartitionToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol).unwrap();
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted).unwrap().0);
}

#[test]
fn test_infeasible_instance() {
    // Cycle 0->1->2->0, B=1, K=0.
    // Each partition can hold weight <= 1 (one vertex each),
    // so 3 separate partitions with crossing cost = 3 > K=0.
    // Can't merge either since weight > B=1.
    let source = AcyclicPartition::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 1, 1],
        vec![1, 1, 1],
        1,
        0,
    );
    let reduction: ReductionAcyclicPartitionToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    assert!(solver.solve(ilp).is_err());
}

#[test]
fn test_acyclicpartition_to_ilp_bf_vs_ilp() {
    let source = small_instance();
    let reduction: ReductionAcyclicPartitionToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_acyclicpartition_to_ilp_regression_direct_topological_labels() {
    let source = AcyclicPartition::new(
        DirectedGraph::new(6, vec![(2, 1), (1, 0), (4, 3), (3, 2), (5, 4)]),
        vec![8, 3, 1, 9, 4, 4],
        vec![4, 10, 0, 7, 3],
        11,
        10,
    );
    let reduction: ReductionAcyclicPartitionToILP =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("the feasible source instance must yield a feasible ILP");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.evaluate(&extracted).unwrap().0);
}
