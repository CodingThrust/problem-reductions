use crate::config::DimsIterator;
use crate::models::graph::{PartialFeedbackEdgeSet, RootedTreeArrangement};
use crate::solvers::CustomizedSolver;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn all_simple_graphs(num_vertices: usize) -> impl Iterator<Item = SimpleGraph> {
    let candidate_edges: Vec<(usize, usize)> = (0..num_vertices)
        .flat_map(|u| ((u + 1)..num_vertices).map(move |v| (u, v)))
        .collect();
    let num_graphs = 1usize << candidate_edges.len();

    (0..num_graphs).map(move |mask| {
        let edges = candidate_edges
            .iter()
            .enumerate()
            .filter_map(|(bit, &edge)| ((mask & (1usize << bit)) != 0).then_some(edge))
            .collect();
        SimpleGraph::new(num_vertices, edges)
    })
}

fn exact_partial_feedback_edge_set_feasible(
    graph: &SimpleGraph,
    budget: usize,
    max_cycle_length: usize,
) -> bool {
    let problem = PartialFeedbackEdgeSet::new(graph.clone(), budget, max_cycle_length);
    DimsIterator::new(problem.dims()).any(|config| problem.evaluate(&config).0)
}

fn exact_rooted_tree_arrangement_min_stretch(graph: &SimpleGraph) -> Option<usize> {
    let problem = RootedTreeArrangement::new(graph.clone(), usize::MAX);
    DimsIterator::new(problem.dims())
        .filter_map(|config| problem.total_edge_stretch(&config))
        .min()
}

#[test]
fn test_customized_solver_returns_none_for_unsupported_problem() {
    let problem = crate::models::misc::GroupingBySwapping::new(3, vec![0, 1, 2, 0, 1, 2], 2);
    let solver = CustomizedSolver::new();
    assert!(solver.solve_dyn(&problem).is_none());
}

// --- FD model parity tests against BruteForce ---

#[test]
fn test_customized_solver_matches_bruteforce_for_minimum_cardinality_key() {
    let problem = crate::models::set::MinimumCardinalityKey::new(
        4,
        vec![(vec![0], vec![1]), (vec![1, 2], vec![3])],
        2,
    );
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
    if let Some(w) = &custom {
        assert!(problem.evaluate(w).0, "witness must satisfy the problem");
    }
}

#[test]
fn test_customized_solver_matches_bruteforce_for_additional_key() {
    let problem = crate::models::misc::AdditionalKey::new(
        3,
        vec![(vec![0], vec![1, 2])],
        vec![0, 1, 2],
        vec![],
    );
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
    if let Some(w) = &custom {
        assert!(problem.evaluate(w).0, "witness must satisfy the problem");
    }
}

#[test]
fn test_customized_solver_matches_bruteforce_for_prime_attribute_name() {
    let problem = crate::models::set::PrimeAttributeName::new(
        4,
        vec![(vec![0, 1], vec![2, 3]), (vec![2], vec![0])],
        0,
    );
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
    if let Some(w) = &custom {
        assert!(problem.evaluate(w).0, "witness must satisfy the problem");
    }
}

#[test]
fn test_customized_solver_matches_bruteforce_for_bcnf_violation() {
    let problem = crate::models::misc::BoyceCoddNormalFormViolation::new(
        4,
        vec![(vec![0], vec![1]), (vec![2], vec![3])],
        vec![0, 1, 2, 3],
    );
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
    if let Some(w) = &custom {
        assert!(problem.evaluate(w).0, "witness must satisfy the problem");
    }
}

// --- Exact witness tests for FD models ---

#[test]
fn test_customized_solver_finds_minimum_cardinality_key_witness() {
    let problem = crate::models::set::MinimumCardinalityKey::new(
        6,
        vec![
            (vec![0, 1], vec![2]),
            (vec![0, 2], vec![3]),
            (vec![1, 3], vec![4]),
            (vec![2, 4], vec![5]),
        ],
        2,
    );
    let witness = CustomizedSolver::new()
        .solve_dyn(&problem)
        .expect("expected witness");
    assert!(problem.evaluate(&witness).0);
}

#[test]
fn test_customized_solver_finds_additional_key_witness() {
    let problem = crate::models::misc::AdditionalKey::new(
        6,
        vec![
            (vec![0, 1], vec![2, 3]),
            (vec![2, 3], vec![4, 5]),
            (vec![4, 5], vec![0, 1]),
            (vec![0, 2], vec![3]),
            (vec![3, 5], vec![1]),
        ],
        vec![0, 1, 2, 3, 4, 5],
        vec![vec![0, 1], vec![2, 3], vec![4, 5]],
    );
    let witness = CustomizedSolver::new()
        .solve_dyn(&problem)
        .expect("expected witness");
    assert!(problem.evaluate(&witness).0);
}

#[test]
fn test_customized_solver_finds_prime_attribute_name_witness() {
    let problem = crate::models::set::PrimeAttributeName::new(
        6,
        vec![
            (vec![0, 1], vec![2, 3, 4, 5]),
            (vec![2, 3], vec![0, 1, 4, 5]),
            (vec![0, 3], vec![1, 2, 4, 5]),
        ],
        3,
    );
    let witness = CustomizedSolver::new()
        .solve_dyn(&problem)
        .expect("expected witness");
    assert!(problem.evaluate(&witness).0);
}

#[test]
fn test_customized_solver_finds_bcnf_violation_witness() {
    let problem = crate::models::misc::BoyceCoddNormalFormViolation::new(
        6,
        vec![
            (vec![0, 1], vec![2]),
            (vec![2], vec![3]),
            (vec![3, 4], vec![5]),
        ],
        vec![0, 1, 2, 3, 4, 5],
    );
    let witness = CustomizedSolver::new()
        .solve_dyn(&problem)
        .expect("expected witness");
    assert!(problem.evaluate(&witness).0);
}

#[test]
fn test_customized_solver_no_witness_when_no_solution_exists() {
    // All attributes uniquely determine all others — {0} is the only key and is known
    let problem = crate::models::misc::AdditionalKey::new(
        3,
        vec![
            (vec![0], vec![1, 2]),
            (vec![1], vec![0, 2]),
            (vec![2], vec![0, 1]),
        ],
        vec![0, 1, 2],
        vec![vec![0], vec![1], vec![2]],
    );
    assert!(CustomizedSolver::new().solve_dyn(&problem).is_none());
}

#[test]
fn test_customized_solver_minimum_cardinality_key_no_solution() {
    // bound=1 but no single attribute determines all
    let problem = crate::models::set::MinimumCardinalityKey::new(3, vec![(vec![0, 1], vec![2])], 1);
    // BruteForce should also return None
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
}

// --- PartialFeedbackEdgeSet tests ---

#[test]
fn test_customized_solver_solves_partial_feedback_edge_set_yes_and_no() {
    let yes = crate::models::graph::PartialFeedbackEdgeSet::new(
        crate::topology::SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 0),
                (2, 3),
                (3, 4),
                (4, 2),
                (3, 5),
                (5, 4),
                (0, 3),
            ],
        ),
        3,
        4,
    );
    let no = crate::models::graph::PartialFeedbackEdgeSet::new(
        crate::topology::SimpleGraph::new(
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 0),
                (2, 3),
                (3, 4),
                (4, 2),
                (3, 5),
                (5, 4),
                (0, 3),
            ],
        ),
        1,
        4,
    );

    let solver = CustomizedSolver::new();
    let yes_result = solver.solve_dyn(&yes);
    assert!(yes_result.is_some(), "expected a solution for yes instance");
    assert!(
        yes.is_valid_solution(yes_result.as_ref().unwrap()),
        "witness must satisfy the problem"
    );

    assert!(
        solver.solve_dyn(&no).is_none(),
        "no instance should have no solution"
    );
}

#[test]
fn test_customized_solver_matches_bruteforce_for_partial_feedback_edge_set() {
    // Small instance for parity check
    let problem = crate::models::graph::PartialFeedbackEdgeSet::new(
        crate::topology::SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 0), (2, 3)]),
        1,
        3,
    );
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
    if let Some(w) = &custom {
        assert!(problem.evaluate(w).0, "witness must satisfy the problem");
    }
}

#[test]
fn test_customized_solver_partial_feedback_edge_set_no_cycles() {
    // Tree graph: no cycles at all
    let problem = crate::models::graph::PartialFeedbackEdgeSet::new(
        crate::topology::SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        0,
        3,
    );
    let result = CustomizedSolver::new().solve_dyn(&problem);
    assert!(result.is_some());
    // All zeros: no edges removed
    assert_eq!(result.unwrap(), vec![0, 0, 0]);
}

#[test]
fn test_customized_solver_matches_exhaustive_search_for_small_partial_feedback_edge_set_instances()
{
    for graph in all_simple_graphs(4) {
        for max_cycle_length in 3..=4 {
            for budget in 0..=graph.num_edges() {
                let problem =
                    PartialFeedbackEdgeSet::new(graph.clone(), budget, max_cycle_length);
                let exact_feasible =
                    exact_partial_feedback_edge_set_feasible(&graph, budget, max_cycle_length);
                let custom = CustomizedSolver::new().solve_dyn(&problem);

                assert_eq!(
                    custom.is_some(),
                    exact_feasible,
                    "graph={:?}, budget={budget}, max_cycle_length={max_cycle_length}",
                    graph.edges()
                );
                if let Some(witness) = custom {
                    assert!(
                        problem.evaluate(&witness).0,
                        "customized witness must satisfy graph={:?}, budget={budget}, max_cycle_length={max_cycle_length}",
                        graph.edges()
                    );
                }
            }
        }
    }
}

// --- RootedTreeArrangement tests ---

#[test]
fn test_customized_solver_finds_rooted_tree_arrangement_witness() {
    let problem = crate::models::graph::RootedTreeArrangement::new(
        crate::topology::SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 4)]),
        7,
    );
    let witness = CustomizedSolver::new()
        .solve_dyn(&problem)
        .expect("expected arrangement witness");
    assert!(problem.evaluate(&witness).0, "witness must be valid");
}

#[test]
fn test_customized_solver_matches_bruteforce_for_rooted_tree_arrangement() {
    // Small 3-vertex instance
    let problem = crate::models::graph::RootedTreeArrangement::new(
        crate::topology::SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        3,
    );
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
    if let Some(w) = &custom {
        assert!(problem.evaluate(w).0, "witness must be valid");
    }
}

#[test]
fn test_customized_solver_rooted_tree_arrangement_tight_bound() {
    // Tight bound that rejects — path graph 0-1-2 needs at least stretch 2
    let problem = crate::models::graph::RootedTreeArrangement::new(
        crate::topology::SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        1,
    );
    // With bound=1, we need total stretch=1, but path 0-1-2 needs at minimum 2
    let custom = CustomizedSolver::new().solve_dyn(&problem);
    let brute = crate::solvers::BruteForce::new().find_witness(&problem);
    assert_eq!(custom.is_some(), brute.is_some());
}

#[test]
fn test_customized_solver_rooted_tree_arrangement_canonical_example() {
    // The canonical example from the model file: 4 vertices, bound=5
    let problem = crate::models::graph::RootedTreeArrangement::new(
        crate::topology::SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]),
        5,
    );
    let witness = CustomizedSolver::new()
        .solve_dyn(&problem)
        .expect("expected witness");
    assert!(problem.evaluate(&witness).0, "witness must be valid");
}

#[test]
fn test_customized_solver_matches_exhaustive_search_for_small_rooted_tree_arrangement_instances() {
    for graph in all_simple_graphs(4) {
        let exact_min_stretch = exact_rooted_tree_arrangement_min_stretch(&graph);
        let max_bound = graph.num_edges().saturating_mul(graph.num_vertices().saturating_sub(1));

        for bound in 0..=max_bound {
            let problem = RootedTreeArrangement::new(graph.clone(), bound);
            let custom = CustomizedSolver::new().solve_dyn(&problem);
            let exact_feasible = exact_min_stretch.is_some_and(|stretch| stretch <= bound);

            assert_eq!(
                custom.is_some(),
                exact_feasible,
                "graph={:?}, bound={bound}, exact_min_stretch={exact_min_stretch:?}",
                graph.edges()
            );
            if let Some(witness) = custom {
                assert!(
                    problem.evaluate(&witness).0,
                    "customized witness must satisfy graph={:?}, bound={bound}",
                    graph.edges()
                );
            }
        }
    }
}
