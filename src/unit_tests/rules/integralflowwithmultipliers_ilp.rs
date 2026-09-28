use super::*;
use crate::models::algebraic::Bounded;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_integralflowwithmultipliers_to_ilp_closed_loop() {
    // 4 vertices, arcs (0,1),(0,2),(1,3),(2,3), multipliers all 1, caps all 2, req 2
    let source = IntegralFlowWithMultipliers::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
        2,
    );
    let direct = BruteForce::new()
        .solve(&source)
        .unwrap()
        .expect("source instance should be satisfiable");
    assert!(source.evaluate(&direct).unwrap());

    let reduction =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_integralflowwithmultipliers_to_ilp_bf_vs_ilp() {
    let source = IntegralFlowWithMultipliers::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
        2,
    );
    let reduction =
        ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn normalized_multipliers_preserve_all_small_flows() {
    // Unit capacities bound the flow despite arbitrarily large multipliers.
    // Include a loop to exercise merged coefficients.
    for multiplier in [1, 2, 3, 4, i64::MAX] {
        for requirement in [i64::MIN, 0, 1, 2, i64::MAX] {
            let source = IntegralFlowWithMultipliers::new(
                DirectedGraph::new(3, vec![(0, 1), (1, 2), (1, 1)]),
                0,
                2,
                vec![1, multiplier, 1],
                vec![1; 3],
                requirement,
            );
            let reduction = ReduceTo::<ILP<i64, i64, Bounded>>::reduce_to(&source).unwrap();
            let target = reduction.target_problem();
            // Capacity sum is three, so four bounds every normalized coefficient
            // and demand regardless of the original multiplier or requirement.
            assert!(
                target
                    .parameters()
                    .get("max_constraint_magnitude_bits")
                    .unwrap()
                    <= 3
            );
            for mask in 0..8 {
                let assignment: Vec<i64> = (0..3).map(|i| (mask >> i) & 1).collect();
                let incoming = assignment[0] + assignment[2];
                let outgoing = assignment[1] + assignment[2];
                let expected = i128::from(outgoing)
                    == i128::from(multiplier) * i128::from(incoming)
                    && assignment[1] >= requirement;
                assert_eq!(target.is_feasible(&assignment).unwrap(), expected);
                if expected {
                    let recovered = reduction.extract_solution(&assignment).unwrap();
                    assert!(source.evaluate(&recovered).unwrap().0);
                }
            }
        }
    }
}
