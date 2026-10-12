use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

fn k4_tsp() -> TravelingSalesman<SimpleGraph, i64> {
    TravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![10, 15, 20, 35, 25, 30],
    )
}

#[test]
fn test_reduction_creates_valid_ilp_c4() {
    // C4 cycle: 4 vertices, 4 edges. Unique Hamiltonian cycle (the cycle itself).
    let problem = TravelingSalesman::<_, i64>::unit_weights(SimpleGraph::new(
        4,
        vec![(0, 1), (1, 2), (2, 3), (3, 0)],
    ));
    let reduction: ReductionTSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    crate::rules::test_helpers::assert_parameter_predictions(&problem, &reduction);

    // n=4, m=4: num_vars = 16 + 2*4*4 = 48
    assert_eq!(ilp.num_vars(), 48);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_reduction_c4_closed_loop() {
    // C4 cycle with unit weights: optimal tour cost = 4
    let problem = TravelingSalesman::<_, i64>::unit_weights(SimpleGraph::new(
        4,
        vec![(0, 1), (1, 2), (2, 3), (3, 0)],
    ));
    let reduction: ReductionTSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    // Verify extracted solution is valid on source problem
    let metric = problem.evaluate(&extracted).unwrap();
    assert!(metric.is_valid(), "Extracted solution must be valid");
    assert_eq!(metric, Min(Some(4)));
}

#[test]
fn test_reduction_k4_weighted_closed_loop() {
    // K4 weighted: find minimum weight Hamiltonian cycle
    let problem = k4_tsp();

    // Solve via ILP reduction
    let reduction: ReductionTSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    // Solve via brute force for cross-check
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    let bf_metric = problem.evaluate(&bf_solutions[0]).unwrap();
    let ilp_metric = problem.evaluate(&extracted).unwrap();

    assert!(ilp_metric.is_valid());
    assert_eq!(
        ilp_metric, bf_metric,
        "ILP and brute force must agree on optimal cost"
    );
}

#[test]
fn test_reduction_c5_unweighted_closed_loop() {
    // C5 cycle with unit weights
    let problem = TravelingSalesman::<_, i64>::unit_weights(SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    ));

    let reduction: ReductionTSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    let metric = problem.evaluate(&extracted).unwrap();
    assert!(metric.is_valid());
    assert_eq!(metric, Min(Some(5)));
}

#[test]
fn test_no_hamiltonian_cycle_infeasible() {
    // Path graph 0-1-2-3: no Hamiltonian cycle exists
    let problem = TravelingSalesman::<_, i64>::unit_weights(SimpleGraph::new(
        4,
        vec![(0, 1), (1, 2), (2, 3)],
    ));

    let reduction: ReductionTSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(ilp);

    assert!(
        result.is_err(),
        "Path graph should have no Hamiltonian cycle (infeasible ILP)"
    );
}

#[test]
fn test_solution_extraction_structure() {
    // C4 cycle: verify extraction produces correct edge selection format
    let problem = TravelingSalesman::<_, i64>::unit_weights(SimpleGraph::new(
        4,
        vec![(0, 1), (1, 2), (2, 3), (3, 0)],
    ));
    let reduction: ReductionTSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    // Should have one value per edge
    assert_eq!(extracted.len(), 4);
    // All edges should be selected (C4 has unique cycle = all edges)
    assert_eq!(extracted.iter().filter(|&&selected| selected).count(), 4);
}

#[test]
fn test_solve_via_ilp_pipeline() {
    let problem = k4_tsp();

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve(&problem)
        .expect("ILP pipeline should solve the problem");

    let metric = problem.evaluate(&solution).unwrap();
    assert!(metric.is_valid());

    // Cross-check with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    assert_eq!(metric, problem.evaluate(&bf_solutions[0]).unwrap());
}

#[test]
fn test_travelingsalesman_to_ilp_bf_vs_ilp() {
    let problem = TravelingSalesman::<_, i64>::unit_weights(SimpleGraph::new(
        4,
        vec![(0, 1), (1, 2), (2, 3), (3, 0)],
    ));
    let reduction: ReductionTSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn simple_graphs_with_fewer_than_three_vertices_produce_infeasible_ilps() {
    for graph in [
        SimpleGraph::empty(0),
        SimpleGraph::empty(1),
        SimpleGraph::empty(2),
        SimpleGraph::path(2),
    ] {
        for weight in [-3, 0, 5] {
            let source = TravelingSalesman::new(graph.clone(), vec![weight; graph.num_edges()]);
            assert!(BruteForce::new().solve(&source).unwrap().is_none());
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
            crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
            let target = reduction.target_problem();
            assert!(!target.is_feasible(&vec![0; target.num_vars()]).unwrap());
            assert_eq!(
                ILPSolver::new().solve(target),
                Err(crate::solvers::ILPSolveError::Infeasible),
                "n={}, m={}",
                graph.num_vertices(),
                graph.num_edges()
            );
            assert!(reduction
                .extract_solution(&vec![0; target.num_vars()])
                .is_err());
        }
    }
}

#[test]
fn extraction_rejects_a_tour_with_incorrect_auxiliary_products() {
    let source = TravelingSalesman::new(SimpleGraph::cycle(3), vec![-3, 0, 5]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let mut assignment = vec![0; reduction.target_problem().num_vars()];
    for vertex in 0..3 {
        assignment[vertex * 3 + vertex] = 1;
    }
    assert!(!reduction.target_problem().is_feasible(&assignment).unwrap());
    assert!(reduction.extract_solution(&assignment).is_err());
}

fn permutations(n: usize) -> Vec<Vec<usize>> {
    (0..n.pow(n as u32))
        .filter_map(|mut code| {
            let order: Vec<_> = (0..n)
                .map(|_| {
                    let v = code % n;
                    code /= n;
                    v
                })
                .collect();
            (0..n).all(|v| order.contains(&v)).then_some(order)
        })
        .collect()
}

fn simple_graph_isomorphism_classes(n: usize) -> Vec<SimpleGraph> {
    let pairs: Vec<_> = (0..n)
        .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
        .collect();
    let orders = permutations(n);
    let mut classes = std::collections::BTreeSet::new();
    for mask in 0..1usize << pairs.len() {
        let canonical = orders
            .iter()
            .map(|order| {
                pairs
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .fold(0, |bits, (_, &(u, v))| {
                        let pair = (order[u].min(order[v]), order[u].max(order[v]));
                        bits | (1 << pairs.iter().position(|&p| p == pair).unwrap())
                    })
            })
            .min()
            .unwrap();
        classes.insert(canonical);
    }
    classes
        .into_iter()
        .map(|mask| {
            SimpleGraph::new(
                n,
                pairs
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, &p)| p)
                    .collect(),
            )
        })
        .collect()
}

fn assignment_for_order(graph: &SimpleGraph, order: &[usize]) -> Vec<i64> {
    let n = graph.num_vertices();
    let mut assignment = vec![0; n * n + 2 * n * graph.num_edges()];
    for (k, &v) in order.iter().enumerate() {
        assignment[v * n + k] = 1;
    }
    for (e, (u, v)) in graph.edges().into_iter().enumerate() {
        for k in 0..n {
            assignment[n * n + 2 * n * e + 2 * k] =
                assignment[u * n + k] * assignment[v * n + (k + 1) % n];
            assignment[n * n + 2 * n * e + 2 * k + 1] =
                assignment[v * n + k] * assignment[u * n + (k + 1) % n];
        }
    }
    assignment
}

fn exhaustive_tsp_ilp(n: usize) {
    let start = std::time::Instant::now();
    let graphs = simple_graph_isomorphism_classes(n);
    assert_eq!(graphs.len(), [1, 1, 2, 4, 11, 34][n]);
    for graph in &graphs {
        let m = graph.num_edges();
        for weights in [
            vec![1; m],
            vec![0; m],
            vec![-2; m],
            (0..m).map(|e| [-3, 0, 5][e % 3]).collect(),
        ] {
            let source = TravelingSalesman::new(graph.clone(), weights);
            let expected = BruteForce::new()
                .solve(&source)
                .unwrap()
                .map(|s| source.evaluate(&s).unwrap().0.unwrap());
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
            crate::rules::test_helpers::assert_parameter_predictions(&source, &reduction);
            let target = reduction.target_problem();
            match ILPSolver::new().solve(target) {
                Ok(solution) => {
                    let decoded = reduction.extract_solution(&solution).unwrap();
                    assert_eq!(source.evaluate(&decoded).unwrap().0, expected);
                    assert_eq!(target.evaluate(&solution).unwrap().value, expected);
                    assert!(expected.is_some());
                }
                Err(crate::solvers::ILPSolveError::Infeasible) => assert_eq!(expected, None),
                Err(error) => panic!("unexpected solver failure: {error}"),
            }
            let mut costs = Vec::new();
            for order in permutations(n) {
                let solution = assignment_for_order(graph, &order);
                let valid = n >= 3 && (0..n).all(|k| graph.has_edge(order[k], order[(k + 1) % n]));
                assert_eq!(target.is_feasible(&solution).unwrap(), valid);
                assert_eq!(reduction.extract_solution(&solution).is_ok(), valid);
                if valid {
                    let decoded = reduction.extract_solution(&solution).unwrap();
                    let source_cost = source.evaluate(&decoded).unwrap().0.unwrap();
                    assert_eq!(target.evaluate(&solution).unwrap().value, Some(source_cost));
                    costs.push(source_cost);
                }
            }
            assert_eq!(costs.into_iter().min(), expected);
        }
    }
    eprintln!(
        "TSP→ILP: n={n}, {} graph classes, four weight sets, {:?}",
        graphs.len(),
        start.elapsed()
    );
}

#[test]
fn exhaustive_tsp_ilp_up_to_four_vertices() {
    for n in 0..=4 {
        exhaustive_tsp_ilp(n);
    }
}

#[test]
fn exhaustive_tsp_ilp_five_vertices() {
    exhaustive_tsp_ilp(5);
}

#[test]
fn tour_constraints_reject_subtours_repetition_nonedges_and_bad_products() {
    for (graph, order) in [
        (
            SimpleGraph::new(6, vec![(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]),
            vec![0, 1, 2, 3, 4, 5],
        ),
        (SimpleGraph::complete(4), vec![0, 1, 1, 3]),
        (SimpleGraph::cycle(4), vec![0, 2, 1, 3]),
        (SimpleGraph::path(4), vec![0, 1, 2, 3]),
    ] {
        let source = TravelingSalesman::unit_weights(graph.clone());
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let assignment = assignment_for_order(&graph, &order);
        assert!(!reduction.target_problem().is_feasible(&assignment).unwrap());
        assert!(reduction.extract_solution(&assignment).is_err());
    }
    let graph = SimpleGraph::complete(4);
    for weights in [vec![-3; 6], vec![0; 6], vec![5; 6]] {
        let source = TravelingSalesman::new(graph.clone(), weights);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let mut assignment = assignment_for_order(&graph, &[0, 1, 2, 3]);
        for y in 16..assignment.len() {
            assignment[y] ^= 1;
            assert!(!reduction.target_problem().is_feasible(&assignment).unwrap());
            assert!(reduction.extract_solution(&assignment).is_err());
            assignment[y] ^= 1;
        }
        for invalid in [
            vec![],
            vec![2; assignment.len()],
            vec![-1; assignment.len()],
        ] {
            assert!(reduction.extract_solution(&invalid).is_err());
        }
    }
}

#[test]
fn exact_tour_evaluation_preserves_representable_i64_boundary_costs() {
    let graph = SimpleGraph::complete(3);
    for weights in [
        vec![i64::MIN, 0, 0],
        vec![i64::MAX, 0, 0],
        vec![i64::MAX, -i64::MAX, 0],
        vec![i64::MIN, i64::MAX, 1],
    ] {
        let source = TravelingSalesman::new(graph.clone(), weights);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        for order in permutations(3) {
            let assignment = assignment_for_order(&graph, &order);
            let decoded = reduction.extract_solution(&assignment).unwrap();
            assert_eq!(
                reduction
                    .target_problem()
                    .evaluate(&assignment)
                    .unwrap()
                    .value,
                source.evaluate(&decoded).unwrap().0
            );
        }
    }
}
