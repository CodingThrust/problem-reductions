use crate::models::graph::KClique;
use crate::models::misc::ConjunctiveBooleanQuery;
use crate::rules::kclique_conjunctivebooleanquery::ReductionKCliqueToCBQ;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_kclique_to_conjunctivebooleanquery_closed_loop() {
    // Triangle graph (0,1,2) plus extra edges, k=3
    let graph = SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 2), (2, 3), (3, 4)]);
    let problem = KClique::new(graph, 3);
    let reduction: ReductionKCliqueToCBQ = ReduceTo::<ConjunctiveBooleanQuery>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &problem,
        &reduction,
        "KClique -> CBQ triangle",
    );
}

#[test]
fn test_reduction_structure() {
    // Complete graph K4, k=3
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    let problem = KClique::new(graph, 3);
    let reduction: ReductionKCliqueToCBQ = ReduceTo::<ConjunctiveBooleanQuery>::reduce_to(&problem);
    let cbq = reduction.target_problem();

    // domain_size = num_vertices = 4
    assert_eq!(cbq.domain_size(), 4);
    // 1 relation
    assert_eq!(cbq.num_relations(), 1);
    // k=3 variables
    assert_eq!(cbq.num_variables(), 3);
    // k*(k-1)/2 = 3 conjuncts
    assert_eq!(cbq.num_conjuncts(), 3);
    // K4 has 6 edges, so relation has 12 tuples
    assert_eq!(cbq.relations()[0].tuples.len(), 12);
    assert_eq!(cbq.relations()[0].arity, 2);
}

#[test]
fn test_no_clique_infeasible() {
    // Path graph 0-1-2, k=3 → no triangle
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = KClique::new(graph, 3);
    let reduction: ReductionKCliqueToCBQ = ReduceTo::<ConjunctiveBooleanQuery>::reduce_to(&problem);

    let bf = BruteForce::new();
    // Source has no 3-clique
    assert_eq!(bf.find_witness(&problem), None);
    // Target CBQ should also be unsatisfiable
    assert_eq!(bf.find_witness(reduction.target_problem()), None);
}

#[test]
fn test_solution_extraction() {
    // Triangle graph, k=3
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let problem = KClique::new(graph, 3);
    let reduction: ReductionKCliqueToCBQ = ReduceTo::<ConjunctiveBooleanQuery>::reduce_to(&problem);

    let bf = BruteForce::new();
    let cbq_witness = bf
        .find_witness(reduction.target_problem())
        .expect("CBQ should be satisfiable");
    let extracted = reduction.extract_solution(&cbq_witness);
    assert_eq!(problem.evaluate(&extracted), Or(true));
    // All 3 vertices should be selected
    assert_eq!(extracted.iter().sum::<usize>(), 3);
}

#[test]
fn test_trivial_k1() {
    // Any graph with at least 1 vertex, k=1 → always feasible
    let graph = SimpleGraph::new(3, vec![(0, 1)]);
    let problem = KClique::new(graph, 1);
    let reduction: ReductionKCliqueToCBQ = ReduceTo::<ConjunctiveBooleanQuery>::reduce_to(&problem);
    let cbq = reduction.target_problem();

    // k=1: 0 conjuncts, 1 variable
    assert_eq!(cbq.num_variables(), 1);
    assert_eq!(cbq.num_conjuncts(), 0);

    let bf = BruteForce::new();
    let witness = bf
        .find_witness(reduction.target_problem())
        .expect("k=1 should be feasible");
    let extracted = reduction.extract_solution(&witness);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
