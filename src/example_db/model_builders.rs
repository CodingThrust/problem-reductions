use crate::export::{ModelExample, SampleEval};
use crate::models::algebraic::{
    ClosestVectorProblem, LinearConstraint, ObjectiveSense, VarBounds, BMF, ILP, QUBO,
};
use crate::models::formula::{
    Assignment, BooleanExpr, CNFClause, Circuit, CircuitSAT, KSatisfiability, Satisfiability,
};
use crate::models::graph::{
    BicliqueCover, HamiltonianPath, IsomorphicSpanningTree, KColoring, MaxCut, MaximalIS,
    MaximumClique, MaximumIndependentSet, MaximumMatching, MinimumDominatingSet,
    MinimumFeedbackVertexSet, MinimumSumMulticenter, MinimumVertexCover, PartitionIntoTriangles,
    SpinGlass, TravelingSalesman,
};
use crate::models::misc::{Factoring, PaintShop, ShortestCommonSupersequence};
use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
use crate::solvers::BruteForce;
use crate::topology::{BipartiteGraph, DirectedGraph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::variant::K3;
use serde::Serialize;

fn sample_eval<P>(problem: &P, config: Vec<usize>) -> SampleEval
where
    P: Problem,
    P::Metric: Serialize,
{
    let metric =
        serde_json::to_value(problem.evaluate(&config)).expect("Failed to serialize metric");
    SampleEval { config, metric }
}

fn optimization_example<P>(problem: P, samples: Vec<Vec<usize>>) -> ModelExample
where
    P: OptimizationProblem + Serialize,
    P::Metric: Serialize,
{
    let sample_evals = samples
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    let optimal = BruteForce::new()
        .find_all_best(&problem)
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    ModelExample::from_problem(&problem, sample_evals, optimal)
}

fn satisfaction_example<P>(problem: P, samples: Vec<Vec<usize>>) -> ModelExample
where
    P: Problem<Metric = bool> + Serialize,
{
    let sample_evals = samples
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    let satisfying = BruteForce::new()
        .find_all_satisfying(&problem)
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    ModelExample::from_problem(&problem, sample_evals, satisfying)
}

fn explicit_example<P>(
    problem: P,
    samples: Vec<Vec<usize>>,
    optimal_configs: Vec<Vec<usize>>,
) -> ModelExample
where
    P: Problem + Serialize,
    P::Metric: Serialize,
{
    let sample_evals = samples
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    let optimal = optimal_configs
        .into_iter()
        .map(|config| sample_eval(&problem, config))
        .collect();
    ModelExample::from_problem(&problem, sample_evals, optimal)
}

fn house_graph() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)])
}

fn petersen_graph() -> SimpleGraph {
    SimpleGraph::new(
        10,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 0),
            (5, 7),
            (7, 9),
            (9, 6),
            (6, 8),
            (8, 5),
            (0, 5),
            (1, 6),
            (2, 7),
            (3, 8),
            (4, 9),
        ],
    )
}

fn complete_graph_k4() -> SimpleGraph {
    SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)])
}

fn maximum_independent_set_example() -> ModelExample {
    let problem = MaximumIndependentSet::new(petersen_graph(), vec![1i32; 10]);
    optimization_example(problem, vec![vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 1]])
}

fn minimum_vertex_cover_example() -> ModelExample {
    let problem = MinimumVertexCover::new(house_graph(), vec![1i32; 5]);
    optimization_example(problem, vec![vec![1, 0, 0, 1, 1]])
}

fn max_cut_example() -> ModelExample {
    let problem = MaxCut::<_, i32>::unweighted(house_graph());
    optimization_example(problem, vec![vec![1, 0, 0, 1, 0]])
}

fn hamiltonian_path_example() -> ModelExample {
    let problem = HamiltonianPath::new(SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (3, 5),
            (4, 2),
            (5, 1),
        ],
    ));
    satisfaction_example(problem, vec![vec![0, 2, 4, 3, 1, 5]])
}

fn isomorphic_spanning_tree_example() -> ModelExample {
    let problem = IsomorphicSpanningTree::new(
        complete_graph_k4(),
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
    );
    satisfaction_example(problem, vec![vec![0, 1, 2, 3]])
}

fn kcoloring_example() -> ModelExample {
    let problem = KColoring::<K3, _>::new(house_graph());
    satisfaction_example(problem, vec![vec![0, 1, 1, 0, 2]])
}

fn minimum_dominating_set_example() -> ModelExample {
    let problem = MinimumDominatingSet::new(house_graph(), vec![1i32; 5]);
    optimization_example(problem, vec![vec![0, 0, 1, 1, 0]])
}

fn maximum_matching_example() -> ModelExample {
    let problem = MaximumMatching::<_, i32>::unit_weights(house_graph());
    optimization_example(problem, vec![vec![1, 0, 0, 0, 1, 0]])
}

fn traveling_salesman_example() -> ModelExample {
    let problem = TravelingSalesman::new(complete_graph_k4(), vec![1, 3, 2, 2, 3, 1]);
    optimization_example(problem, vec![vec![1, 0, 1, 1, 0, 1]])
}

fn maximum_clique_example() -> ModelExample {
    let problem = MaximumClique::new(house_graph(), vec![1i32; 5]);
    optimization_example(problem, vec![vec![0, 0, 1, 1, 1]])
}

fn maximal_is_example() -> ModelExample {
    let problem = MaximalIS::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    optimization_example(problem, vec![vec![0, 1, 0, 1, 0], vec![1, 0, 1, 0, 1]])
}

fn minimum_feedback_vertex_set_example() -> ModelExample {
    let problem = MinimumFeedbackVertexSet::new(
        DirectedGraph::new(
            5,
            vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 1), (4, 2)],
        ),
        vec![1i32; 5],
    );
    optimization_example(problem, vec![vec![1, 0, 0, 0, 0]])
}

fn minimum_sum_multicenter_example() -> ModelExample {
    let graph = SimpleGraph::new(
        7,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (0, 6),
            (2, 5),
        ],
    );
    let problem = MinimumSumMulticenter::new(graph, vec![1i32; 7], vec![1i32; 8], 2);
    optimization_example(problem, vec![vec![0, 0, 1, 0, 0, 1, 0]])
}

fn maximum_set_packing_example() -> ModelExample {
    let problem =
        MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]);
    optimization_example(problem, vec![vec![1, 0, 1, 0]])
}

fn minimum_set_covering_example() -> ModelExample {
    let problem = MinimumSetCovering::<i32>::new(5, vec![vec![0, 1, 2], vec![1, 3], vec![2, 3, 4]]);
    optimization_example(problem, vec![vec![1, 0, 1]])
}

fn spin_glass_example() -> ModelExample {
    let problem = SpinGlass::<SimpleGraph, i32>::without_fields(
        5,
        vec![
            ((0, 1), 1),
            ((1, 2), 1),
            ((3, 4), 1),
            ((0, 3), 1),
            ((1, 3), 1),
            ((1, 4), 1),
            ((2, 4), 1),
        ],
    );
    optimization_example(problem, vec![vec![1, 0, 1, 1, 0]])
}

fn qubo_example() -> ModelExample {
    let problem = QUBO::from_matrix(vec![
        vec![-1.0, 2.0, 0.0],
        vec![0.0, -1.0, 2.0],
        vec![0.0, 0.0, -1.0],
    ]);
    optimization_example(problem, vec![vec![1, 0, 1]])
}

fn ilp_example() -> ModelExample {
    let problem = ILP::<i32>::new(
        2,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 5.0),
            LinearConstraint::le(vec![(0, 4.0), (1, 7.0)], 28.0),
        ],
        vec![(0, -5.0), (1, -6.0)],
        ObjectiveSense::Minimize,
    );
    explicit_example(problem, vec![vec![0, 4]], vec![vec![3, 2]])
}

fn closest_vector_problem_example() -> ModelExample {
    let problem = ClosestVectorProblem::new(
        vec![vec![2, 0], vec![1, 2]],
        vec![2.8, 1.5],
        vec![VarBounds::bounded(-2, 4), VarBounds::bounded(-2, 4)],
    );
    optimization_example(problem, vec![vec![3, 3]])
}

fn satisfiability_example() -> ModelExample {
    let problem = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-2, -3]),
        ],
    );
    satisfaction_example(problem, vec![vec![1, 0, 1]])
}

fn ksatisfiability_example() -> ModelExample {
    let problem = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
            CNFClause::new(vec![1, -2, -3]),
        ],
    );
    satisfaction_example(problem, vec![vec![1, 0, 1]])
}

fn circuit_sat_example() -> ModelExample {
    let problem = CircuitSAT::new(Circuit::new(vec![
        Assignment::new(
            vec!["a".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x1"), BooleanExpr::var("x2")]),
        ),
        Assignment::new(
            vec!["b".to_string()],
            BooleanExpr::or(vec![BooleanExpr::var("x1"), BooleanExpr::var("x2")]),
        ),
        Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::xor(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
        ),
    ]));
    satisfaction_example(problem, vec![vec![0, 1, 1, 0, 1], vec![0, 1, 1, 1, 0]])
}

fn factoring_example() -> ModelExample {
    let problem = Factoring::new(2, 3, 15);
    optimization_example(problem, vec![vec![1, 1, 1, 0, 1]])
}

fn bmf_example() -> ModelExample {
    let problem = BMF::new(
        vec![
            vec![true, true, false],
            vec![true, true, true],
            vec![false, true, true],
        ],
        2,
    );
    optimization_example(problem, vec![vec![1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1]])
}

fn paintshop_example() -> ModelExample {
    let problem = PaintShop::new(vec!["A", "B", "A", "C", "B", "C"]);
    let sample = BruteForce::new()
        .find_all_best(&problem)
        .into_iter()
        .next()
        .expect("paintshop example should solve");
    optimization_example(problem, vec![sample])
}

fn biclique_cover_example() -> ModelExample {
    let problem = BicliqueCover::new(
        BipartiteGraph::new(2, 3, vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
        2,
    );
    optimization_example(problem, vec![vec![1, 0, 0, 1, 1, 0, 1, 1, 0, 1]])
}

fn partition_into_triangles_example() -> ModelExample {
    let problem = PartitionIntoTriangles::new(SimpleGraph::new(
        6,
        vec![(0, 1), (0, 2), (1, 2), (3, 4), (3, 5), (4, 5), (0, 3)],
    ));
    satisfaction_example(problem, vec![vec![0, 0, 0, 1, 1, 1]])
}

fn shortest_common_supersequence_example() -> ModelExample {
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]], 4);
    satisfaction_example(problem, vec![vec![1, 0, 1, 2]])
}

pub fn build_model_examples() -> Vec<ModelExample> {
    vec![
        maximum_independent_set_example(),
        minimum_vertex_cover_example(),
        max_cut_example(),
        hamiltonian_path_example(),
        isomorphic_spanning_tree_example(),
        kcoloring_example(),
        minimum_dominating_set_example(),
        maximum_matching_example(),
        traveling_salesman_example(),
        maximum_clique_example(),
        maximal_is_example(),
        minimum_feedback_vertex_set_example(),
        minimum_sum_multicenter_example(),
        maximum_set_packing_example(),
        minimum_set_covering_example(),
        spin_glass_example(),
        qubo_example(),
        ilp_example(),
        closest_vector_problem_example(),
        satisfiability_example(),
        ksatisfiability_example(),
        circuit_sat_example(),
        factoring_example(),
        bmf_example(),
        paintshop_example(),
        biclique_cover_example(),
        partition_into_triangles_example(),
        shortest_common_supersequence_example(),
    ]
}
