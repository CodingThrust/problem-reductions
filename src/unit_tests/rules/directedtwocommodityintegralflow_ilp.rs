use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn feasible_instance() -> DirectedTwoCommodityIntegralFlow {
    // 6-vertex network: s1=0, s2=1, t1=4, t2=5
    // Arcs: (0,2),(0,3),(1,2),(1,3),(2,4),(2,5),(3,4),(3,5), all cap=1
    DirectedTwoCommodityIntegralFlow::new(
        DirectedGraph::new(
            6,
            vec![
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 3),
                (2, 4),
                (2, 5),
                (3, 4),
                (3, 5),
            ],
        ),
        vec![1; 8],
        0,
        4,
        1,
        5,
        1,
        1,
    )
}

fn infeasible_instance() -> DirectedTwoCommodityIntegralFlow {
    // Two commodities competing on a single arc with cap=1
    // s1=0→t1=2 and s2=0→t2=2 both need to route 1 unit through the single arc (0,2)
    DirectedTwoCommodityIntegralFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1],
        0,
        2,
        0,
        2,
        1,
        1,
    )
}

#[test]
fn test_directedtwocommodityintegralflow_to_ilp_structure() {
    let problem = feasible_instance();
    let reduction: ReductionD2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 8 arcs → 2*8 = 16 variables
    assert_eq!(ilp.num_vars, 16);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());

    // 8 capacity + some conservation (4 non-terminals: vertices 2,3) + 2 sink req = 8 + 4 + 2 = 14
    // Actually conservation: for vertex 2 (c1): arcs (0,2),(1,2) in; (2,4),(2,5) out → 4 terms → 1 eq per commodity per vertex
    // Terminals: 0,4,1,5 — so non-terminals are: 2,3
    // vertex 2: c1 terms and c2 terms → 2 constraints; vertex 3: 2 constraints → 4 total
    assert_eq!(ilp.constraints.len(), 8 + 4 + 2);
}

#[test]
fn test_directedtwocommodityintegralflow_to_ilp_closed_loop() {
    let problem = feasible_instance();
    let bf = BruteForce::new();
    let bf_solution = bf
        .find_witness(&problem)
        .expect("feasible instance has a witness");
    assert!(
        problem.evaluate(&bf_solution).0,
        "brute force solution is valid"
    );

    let reduction: ReductionD2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(
        problem.evaluate(&extracted).0,
        "ILP extracted solution should be a valid flow"
    );
}

#[test]
fn test_directedtwocommodityintegralflow_to_ilp_infeasible() {
    let problem = infeasible_instance();
    let reduction: ReductionD2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible flow instance should produce infeasible ILP"
    );
}

#[test]
fn test_directedtwocommodityintegralflow_to_ilp_extract_solution() {
    let problem = feasible_instance();
    let reduction: ReductionD2CIFToILP = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // f1 routes via (0,2),(2,4): arcs 0,4 = 1; rest 0 for commodity 1
    // f2 routes via (1,3),(3,5): arcs 3,7 = 1; rest 0 for commodity 2
    let mut target_solution = vec![0usize; 16];
    target_solution[0] = 1; // f1 on arc (0,2)
    target_solution[4] = 1; // f1 on arc (2,4)
    target_solution[8 + 3] = 1; // f2 on arc (1,3)
    target_solution[8 + 7] = 1; // f2 on arc (3,5)

    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted.len(), 16);
    assert!(
        problem.evaluate(&extracted).0,
        "manually extracted solution should be valid"
    );
}
