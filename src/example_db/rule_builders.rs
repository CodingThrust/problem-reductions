use crate::config::DimsIterator;
use crate::export::{
    overhead_to_json, lookup_overhead, variant_to_map, ProblemSide, RuleExample, SolutionPair,
};
use crate::models::algebraic::{
    ClosestVectorProblem, ILP, LinearConstraint, ObjectiveSense, QUBO, VarBounds, VariableDomain,
};
use crate::models::formula::{
    Assignment, BooleanExpr, CNFClause, Circuit, CircuitSAT, KSatisfiability, Satisfiability,
};
use crate::models::graph::{
    KColoring, MaxCut, MaximumClique, MaximumIndependentSet, MaximumMatching,
    MinimumDominatingSet, MinimumVertexCover, SpinGlass, TravelingSalesman,
};
use crate::models::misc::{
    BinPacking, Factoring, LongestCommonSubsequence, ShortestCommonSupersequence, SubsetSum,
};
use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
use crate::prelude::{OptimizationProblem, Problem, ReduceTo, ReductionResult};
use crate::rules::{Minimize, MinimizeSteps, PathCostFn, ReductionGraph};
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::small_graphs::{house, octahedral, petersen};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;
use crate::types::ProblemSize;
use crate::variant::K3;
use serde::Serialize;
use std::collections::HashMap;

fn assemble_rule_example<S, T>(
    source: &S,
    target: &T,
    overhead: crate::rules::ReductionOverhead,
    solutions: Vec<SolutionPair>,
) -> RuleExample
where
    S: Problem + Serialize,
    T: Problem + Serialize,
{
    RuleExample {
        source: ProblemSide::from_problem(source),
        target: ProblemSide::from_problem(target),
        overhead: overhead_to_json(&overhead),
        solutions,
    }
}

fn direct_overhead<S, T>() -> crate::rules::ReductionOverhead
where
    S: Problem,
    T: Problem,
{
    let source_variant = variant_to_map(S::variant());
    let target_variant = variant_to_map(T::variant());
    lookup_overhead(S::NAME, &source_variant, T::NAME, &target_variant).unwrap_or_default()
}

fn direct_best_example<S, T, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<T>,
    T: OptimizationProblem + Serialize,
    T::Metric: Serialize,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<T>::reduce_to(&source);
    let target = reduction.target_problem();
    let solutions = BruteForce::new()
        .find_all_best(target)
        .into_iter()
        .filter_map(|target_config| {
            let source_config = reduction.extract_solution(&target_config);
            keep(&source, &source_config).then_some(SolutionPair {
                source_config,
                target_config,
            })
        })
        .collect();
    assemble_rule_example(&source, target, direct_overhead::<S, T>(), solutions)
}

fn direct_satisfying_example<S, T, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<T>,
    T: Problem<Metric = bool> + Serialize,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<T>::reduce_to(&source);
    let target = reduction.target_problem();
    let solutions = BruteForce::new()
        .find_all_satisfying(target)
        .into_iter()
        .filter_map(|target_config| {
            let source_config = reduction.extract_solution(&target_config);
            keep(&source, &source_config).then_some(SolutionPair {
                source_config,
                target_config,
            })
        })
        .collect();
    assemble_rule_example(&source, target, direct_overhead::<S, T>(), solutions)
}

fn direct_ilp_example<S, V, Keep>(source: S, keep: Keep) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<ILP<V>>,
    ILP<V>: Serialize,
    V: VariableDomain,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let reduction = ReduceTo::<ILP<V>>::reduce_to(&source);
    let target = reduction.target_problem();
    let target_config = ILPSolver::new()
        .solve(target)
        .expect("canonical ILP target example should solve");
    let source_config = reduction.extract_solution(&target_config);
    let solutions = if keep(&source, &source_config) {
        vec![SolutionPair {
            source_config,
            target_config,
        }]
    } else {
        Vec::new()
    };
    assemble_rule_example(&source, target, direct_overhead::<S, ILP<V>>(), solutions)
}

fn path_best_example<S, T, C, Keep>(
    source: S,
    input_size: ProblemSize,
    cost: C,
    keep: Keep,
) -> RuleExample
where
    S: Problem + Serialize + 'static,
    T: OptimizationProblem + Serialize + 'static,
    T::Metric: Serialize,
    C: PathCostFn,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let graph = ReductionGraph::new();
    let source_variant = variant_to_map(S::variant());
    let target_variant = variant_to_map(T::variant());
    let path = graph
        .find_cheapest_path(
            S::NAME,
            &source_variant,
            T::NAME,
            &target_variant,
            &input_size,
            &cost,
        )
        .expect("canonical path example should exist");
    let chain = graph
        .reduce_along_path(&path, &source as &dyn std::any::Any)
        .expect("canonical path example should execute");
    let target = chain.target_problem::<T>();
    let solutions = BruteForce::new()
        .find_all_best(target)
        .into_iter()
        .filter_map(|target_config| {
            let source_config = chain.extract_solution(&target_config);
            keep(&source, &source_config).then_some(SolutionPair {
                source_config,
                target_config,
            })
        })
        .collect();
    assemble_rule_example(&source, target, graph.compose_path_overhead(&path), solutions)
}

fn path_ilp_example<S, V, C, Keep>(
    source: S,
    input_size: ProblemSize,
    cost: C,
    keep: Keep,
) -> RuleExample
where
    S: Problem + Serialize + 'static,
    ILP<V>: Serialize + 'static,
    V: VariableDomain,
    C: PathCostFn,
    Keep: Fn(&S, &[usize]) -> bool,
{
    let graph = ReductionGraph::new();
    let source_variant = variant_to_map(S::variant());
    let target_variant = variant_to_map(ILP::<V>::variant());
    let path = graph
        .find_cheapest_path(
            S::NAME,
            &source_variant,
            ILP::<V>::NAME,
            &target_variant,
            &input_size,
            &cost,
        )
        .expect("canonical ILP path example should exist");
    let chain = graph
        .reduce_along_path(&path, &source as &dyn std::any::Any)
        .expect("canonical ILP path example should execute");
    let target = chain.target_problem::<ILP<V>>();
    let target_config = ILPSolver::new()
        .solve(target)
        .expect("canonical ILP path target should solve");
    let source_config = chain.extract_solution(&target_config);
    let solutions = if keep(&source, &source_config) {
        vec![SolutionPair {
            source_config,
            target_config,
        }]
    } else {
        Vec::new()
    };
    assemble_rule_example(&source, target, graph.compose_path_overhead(&path), solutions)
}

fn petersen_graph() -> SimpleGraph {
    let (n, edges) = petersen();
    SimpleGraph::new(n, edges)
}

fn house_graph() -> SimpleGraph {
    let (n, edges) = house();
    SimpleGraph::new(n, edges)
}

fn octahedral_graph() -> SimpleGraph {
    let (n, edges) = octahedral();
    SimpleGraph::new(n, edges)
}

fn path_graph_p4() -> SimpleGraph {
    SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)])
}

fn path_graph_p5() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)])
}

fn full_adder_circuit_sat() -> CircuitSAT {
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["t".to_string()],
            BooleanExpr::xor(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
        ),
        Assignment::new(
            vec!["sum".to_string()],
            BooleanExpr::xor(vec![BooleanExpr::var("t"), BooleanExpr::var("cin")]),
        ),
        Assignment::new(
            vec!["ab".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
        ),
        Assignment::new(
            vec!["cin_t".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("cin"), BooleanExpr::var("t")]),
        ),
        Assignment::new(
            vec!["cout".to_string()],
            BooleanExpr::or(vec![BooleanExpr::var("ab"), BooleanExpr::var("cin_t")]),
        ),
    ]);
    CircuitSAT::new(circuit)
}

fn sat_three_clause_example() -> Satisfiability {
    Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2]),
            CNFClause::new(vec![2, 3]),
        ],
    )
}

fn sat_seven_clause_example() -> Satisfiability {
    Satisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -4, 5]),
            CNFClause::new(vec![-2, 3, -5]),
            CNFClause::new(vec![1, -3, 5]),
            CNFClause::new(vec![-1, -2, 4]),
            CNFClause::new(vec![3, -4, -5]),
        ],
    )
}

fn sat_unit_clause_example() -> Satisfiability {
    Satisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1]),
            CNFClause::new(vec![-3]),
            CNFClause::new(vec![5]),
        ],
    )
}

fn sat_mixed_clause_example() -> Satisfiability {
    Satisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1]),
            CNFClause::new(vec![2, -3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -4, 5]),
            CNFClause::new(vec![1, -2, 3, -5]),
            CNFClause::new(vec![-1, 2, -3, 4, 5]),
        ],
    )
}

fn ksat_embedding_example() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        4,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
        ],
    )
}

fn ksat_subsetsum_example() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    )
}

fn ksat_qubo_example() -> KSatisfiability<K3> {
    KSatisfiability::<K3>::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -4, 5]),
            CNFClause::new(vec![-2, 3, -5]),
            CNFClause::new(vec![1, -3, 5]),
            CNFClause::new(vec![-1, -2, 4]),
            CNFClause::new(vec![3, -4, -5]),
        ],
    )
}

fn binpacking_example() -> BinPacking<i32> {
    BinPacking::new(vec![6, 5, 5, 4, 3], 10)
}

fn factoring_35_example() -> Factoring {
    Factoring::new(3, 3, 35)
}

fn lcs_example() -> LongestCommonSubsequence {
    LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'A', b'C'], vec![b'B', b'A', b'C', b'A']])
}

fn mis_petersen() -> MaximumIndependentSet<SimpleGraph, i32> {
    MaximumIndependentSet::new(petersen_graph(), vec![1i32; 10])
}

fn vc_petersen() -> MinimumVertexCover<SimpleGraph, i32> {
    MinimumVertexCover::new(petersen_graph(), vec![1i32; 10])
}

fn matching_petersen() -> MaximumMatching<SimpleGraph, i32> {
    MaximumMatching::unit_weights(petersen_graph())
}

fn dominating_petersen() -> MinimumDominatingSet<SimpleGraph, i32> {
    MinimumDominatingSet::new(petersen_graph(), vec![1i32; 10])
}

fn clique_path_p4() -> MaximumClique<SimpleGraph, i32> {
    MaximumClique::new(path_graph_p4(), vec![1i32; 4])
}

fn clique_octahedral() -> MaximumClique<SimpleGraph, i32> {
    MaximumClique::new(octahedral_graph(), vec![1i32; 6])
}

fn coloring_petersen() -> KColoring<K3, SimpleGraph> {
    KColoring::<K3, _>::new(petersen_graph())
}

fn coloring_house() -> KColoring<K3, SimpleGraph> {
    KColoring::<K3, _>::new(house_graph())
}

fn maxcut_petersen() -> MaxCut<SimpleGraph, i32> {
    MaxCut::unweighted(petersen_graph())
}

fn tsp_k3() -> TravelingSalesman<SimpleGraph, i32> {
    TravelingSalesman::new(SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]), vec![1, 2, 3])
}

fn tsp_k4() -> TravelingSalesman<SimpleGraph, i32> {
    TravelingSalesman::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![10, 15, 20, 35, 25, 30],
    )
}

fn setpacking_five_sets() -> MaximumSetPacking<i32> {
    let sets = vec![
        vec![0, 1, 2],
        vec![2, 3],
        vec![4, 5, 6],
        vec![1, 5, 7],
        vec![3, 6],
    ];
    MaximumSetPacking::with_weights(sets, vec![1i32; 5])
}

fn setpacking_six_sets_i32() -> MaximumSetPacking<i32> {
    MaximumSetPacking::new(vec![
        vec![0, 1, 2],
        vec![2, 3, 4],
        vec![4, 5, 6],
        vec![6, 7, 0],
        vec![1, 3, 5],
        vec![0, 4, 7],
    ])
}

fn setpacking_six_sets_f64() -> MaximumSetPacking<f64> {
    MaximumSetPacking::new(vec![
        vec![0, 1, 2],
        vec![2, 3, 4],
        vec![4, 5, 6],
        vec![6, 7, 0],
        vec![1, 3, 5],
        vec![0, 4, 7],
    ])
}

fn setcover_six_sets() -> MinimumSetCovering<i32> {
    MinimumSetCovering::new(
        8,
        vec![
            vec![0, 1, 2],
            vec![2, 3, 4],
            vec![4, 5, 6],
            vec![6, 7, 0],
            vec![1, 3, 5],
            vec![0, 4, 7],
        ],
    )
}

fn qubo_to_ilp_source() -> QUBO<f64> {
    let mut matrix = vec![vec![0.0; 4]; 4];
    matrix[0][0] = -2.0;
    matrix[1][1] = -3.0;
    matrix[2][2] = -1.0;
    matrix[3][3] = -4.0;
    matrix[0][1] = 1.0;
    matrix[1][2] = 2.0;
    matrix[2][3] = -1.0;
    QUBO::from_matrix(matrix)
}

fn qubo_petersen_source() -> QUBO<f64> {
    let (n, edges) = petersen();
    let mut matrix = vec![vec![0.0; n]; n];
    for (i, row) in matrix.iter_mut().enumerate() {
        row[i] = -1.0 + 0.2 * i as f64;
    }
    for (idx, &(u, v)) in edges.iter().enumerate() {
        let (i, j) = if u < v { (u, v) } else { (v, u) };
        matrix[i][j] = if idx % 2 == 0 { 2.0 } else { -1.5 };
    }
    QUBO::from_matrix(matrix)
}

fn spinglass_petersen_i32() -> SpinGlass<SimpleGraph, i32> {
    let (n, edges) = petersen();
    let couplings: Vec<((usize, usize), i32)> = edges
        .iter()
        .enumerate()
        .map(|(i, &(u, v))| ((u, v), if i % 2 == 0 { 1 } else { -1 }))
        .collect();
    SpinGlass::new(n, couplings, vec![0; n])
}

fn spinglass_petersen_f64() -> SpinGlass<SimpleGraph, f64> {
    let (n, edges) = petersen();
    let couplings: Vec<((usize, usize), f64)> = edges
        .iter()
        .enumerate()
        .map(|(i, &(u, v))| ((u, v), if i % 2 == 0 { 1.0 } else { -1.0 }))
        .collect();
    SpinGlass::new(n, couplings, vec![0.0; n])
}

fn ilp_knapsack_example() -> ILP<bool> {
    ILP::new(
        6,
        vec![
            LinearConstraint::le(
                vec![(0, 3.0), (1, 2.0), (2, 5.0), (3, 4.0), (4, 2.0), (5, 3.0)],
                10.0,
            ),
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 1.0)], 2.0),
            LinearConstraint::le(vec![(3, 1.0), (4, 1.0), (5, 1.0)], 2.0),
        ],
        vec![(0, 10.0), (1, 7.0), (2, 12.0), (3, 8.0), (4, 6.0), (5, 9.0)],
        ObjectiveSense::Maximize,
    )
}

macro_rules! direct_best_builder {
    ($name:ident, $source:expr, $target:ty) => {
        fn $name() -> RuleExample {
            direct_best_example::<_, $target, _>($source, |_, _| true)
        }
    };
}

macro_rules! direct_best_keep_builder {
    ($name:ident, $source:expr, $target:ty, $keep:expr) => {
        fn $name() -> RuleExample {
            direct_best_example::<_, $target, _>($source, $keep)
        }
    };
}

macro_rules! direct_sat_builder {
    ($name:ident, $source:expr, $target:ty) => {
        fn $name() -> RuleExample {
            direct_satisfying_example::<_, $target, _>($source, |_, _| true)
        }
    };
}

macro_rules! direct_sat_keep_builder {
    ($name:ident, $source:expr, $target:ty, $keep:expr) => {
        fn $name() -> RuleExample {
            direct_satisfying_example::<_, $target, _>($source, $keep)
        }
    };
}

macro_rules! direct_ilp_builder {
    ($name:ident, $source:expr, $var_ty:ty) => {
        fn $name() -> RuleExample {
            direct_ilp_example::<_, $var_ty, _>($source, |_, _| true)
        }
    };
}

macro_rules! direct_ilp_keep_builder {
    ($name:ident, $source:expr, $var_ty:ty, $keep:expr) => {
        fn $name() -> RuleExample {
            direct_ilp_example::<_, $var_ty, _>($source, $keep)
        }
    };
}

macro_rules! path_best_builder {
    ($name:ident, $source:expr, $target:ty, $size:expr, $cost:expr) => {
        fn $name() -> RuleExample {
            path_best_example::<_, $target, _, _>($source, $size, $cost, |_, _| true)
        }
    };
}

macro_rules! path_ilp_builder {
    ($name:ident, $source:expr, $var_ty:ty, $size:expr, $cost:expr) => {
        fn $name() -> RuleExample {
            path_ilp_example::<_, $var_ty, _, _>($source, $size, $cost, |_, _| true)
        }
    };
}

direct_ilp_builder!(binpacking_to_ilp, binpacking_example(), bool);
direct_best_keep_builder!(
    circuitsat_to_ilp,
    full_adder_circuit_sat(),
    ILP<bool>,
    |source: &CircuitSAT, config| source.evaluate(config)
);
direct_best_keep_builder!(
    circuitsat_to_spinglass,
    full_adder_circuit_sat(),
    SpinGlass<SimpleGraph, i32>,
    |source: &CircuitSAT, config| source.evaluate(config)
);
direct_sat_builder!(factoring_to_ilp_dummy, sat_three_clause_example(), CircuitSAT);
direct_best_builder!(ilp_to_qubo, ilp_knapsack_example(), QUBO<f64>);
direct_ilp_builder!(kcoloring_to_ilp, coloring_petersen(), bool);
direct_best_builder!(kcoloring_to_qubo, coloring_house(), QUBO<f64>);
direct_best_builder!(ksatisfiability_to_qubo, ksat_qubo_example(), QUBO<f64>);
direct_sat_builder!(
    ksatisfiability_to_satisfiability,
    ksat_embedding_example(),
    Satisfiability
);
direct_sat_builder!(ksatisfiability_to_subsetsum, ksat_subsetsum_example(), SubsetSum);
direct_ilp_builder!(longestcommonsubsequence_to_ilp, lcs_example(), bool);
direct_best_builder!(maxcut_to_spinglass, maxcut_petersen(), SpinGlass<SimpleGraph, i32>);
direct_ilp_builder!(maximumclique_to_ilp, clique_octahedral(), bool);
direct_best_builder!(maximumclique_to_maximumindependentset, clique_path_p4(), MaximumIndependentSet<SimpleGraph, i32>);
path_ilp_builder!(
    maximumindependentset_to_ilp,
    mis_petersen(),
    bool,
    ProblemSize::new(vec![]),
    MinimizeSteps
);
direct_best_builder!(maximumindependentset_to_maximumclique, MaximumIndependentSet::new(path_graph_p5(), vec![1i32; 5]), MaximumClique<SimpleGraph, i32>);
direct_best_builder!(maximumindependentset_to_maximumsetpacking, mis_petersen(), MaximumSetPacking<i32>);
direct_best_builder!(maximumindependentset_to_minimumvertexcover, mis_petersen(), MinimumVertexCover<SimpleGraph, i32>);
path_best_builder!(
    maximumindependentset_to_qubo,
    mis_petersen(),
    QUBO<f64>,
    ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 15)]),
    Minimize("num_vars")
);
direct_ilp_builder!(maximummatching_to_ilp, matching_petersen(), bool);
direct_best_builder!(maximummatching_to_maximumsetpacking, matching_petersen(), MaximumSetPacking<i32>);
direct_ilp_builder!(maximumsetpacking_to_ilp, setpacking_six_sets_i32(), bool);
direct_best_builder!(maximumsetpacking_to_maximumindependentset, setpacking_five_sets(), MaximumIndependentSet<SimpleGraph, i32>);
direct_best_builder!(maximumsetpacking_to_qubo, setpacking_six_sets_f64(), QUBO<f64>);
direct_ilp_builder!(minimumdominatingset_to_ilp, dominating_petersen(), bool);
direct_ilp_builder!(minimumsetcovering_to_ilp, setcover_six_sets(), bool);
path_ilp_builder!(
    minimumvertexcover_to_ilp,
    vc_petersen(),
    bool,
    ProblemSize::new(vec![]),
    MinimizeSteps
);
direct_best_builder!(minimumvertexcover_to_maximumindependentset, vc_petersen(), MaximumIndependentSet<SimpleGraph, i32>);
direct_best_builder!(minimumvertexcover_to_minimumsetcovering, vc_petersen(), MinimumSetCovering<i32>);
path_best_builder!(
    minimumvertexcover_to_qubo,
    vc_petersen(),
    QUBO<f64>,
    ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 15)]),
    Minimize("num_vars")
);
direct_best_builder!(qubo_to_ilp, qubo_to_ilp_source(), ILP<bool>);
direct_best_builder!(qubo_to_spinglass, qubo_petersen_source(), SpinGlass<SimpleGraph, f64>);
direct_sat_builder!(satisfiability_to_circuitsat, sat_three_clause_example(), CircuitSAT);
direct_sat_builder!(satisfiability_to_kcoloring, sat_unit_clause_example(), KColoring<K3, SimpleGraph>);
direct_sat_builder!(satisfiability_to_ksatisfiability, sat_mixed_clause_example(), KSatisfiability<K3>);
direct_best_builder!(satisfiability_to_maximumindependentset, sat_seven_clause_example(), MaximumIndependentSet<SimpleGraph, One>);
direct_best_keep_builder!(
    satisfiability_to_minimumdominatingset,
    sat_seven_clause_example(),
    MinimumDominatingSet<SimpleGraph, i32>,
    |source: &Satisfiability, config| source.evaluate(config)
);
direct_best_builder!(spinglass_to_maxcut, spinglass_petersen_i32(), MaxCut<SimpleGraph, i32>);
direct_best_builder!(spinglass_to_qubo, spinglass_petersen_f64(), QUBO<f64>);
direct_ilp_builder!(travelingsalesman_to_ilp, tsp_k4(), bool);
direct_best_builder!(travelingsalesman_to_qubo, tsp_k3(), QUBO<f64>);

fn factoring_to_circuitsat() -> RuleExample {
    fn simulate_circuit(
        circuit: &Circuit,
        initial_assignments: &HashMap<String, bool>,
    ) -> HashMap<String, bool> {
        let mut values = initial_assignments.clone();
        for assignment in &circuit.assignments {
            let result = assignment.expr.evaluate(&values);
            for output in &assignment.outputs {
                values.insert(output.clone(), result);
            }
        }
        values
    }

    let source = factoring_35_example();
    let reduction = ReduceTo::<CircuitSAT>::reduce_to(&source);
    let target = reduction.target_problem();
    let source_solutions = BruteForce::new().find_all_best(&source);
    let var_names = target.variable_names();
    let solutions = source_solutions
        .into_iter()
        .map(|source_config| {
            let mut inputs: HashMap<String, bool> = HashMap::new();
            for (i, &bit) in source_config.iter().enumerate().take(source.m()) {
                inputs.insert(format!("p{}", i + 1), bit == 1);
            }
            for (i, &bit) in source_config[source.m()..]
                .iter()
                .enumerate()
                .take(source.n())
            {
                inputs.insert(format!("q{}", i + 1), bit == 1);
            }
            let values = simulate_circuit(target.circuit(), &inputs);
            let target_config = var_names
                .iter()
                .map(|name| usize::from(*values.get(name).unwrap_or(&false)))
                .collect();
            SolutionPair {
                source_config,
                target_config,
            }
        })
        .collect();
    assemble_rule_example(&source, target, direct_overhead::<Factoring, CircuitSAT>(), solutions)
}

fn factoring_to_ilp() -> RuleExample {
    direct_ilp_example::<_, i32, _>(factoring_35_example(), |_, _| true)
}

pub fn build_rule_examples() -> Vec<RuleExample> {
    vec![
        binpacking_to_ilp(),
        circuitsat_to_ilp(),
        circuitsat_to_spinglass(),
        factoring_to_circuitsat(),
        factoring_to_ilp(),
        ilp_to_qubo(),
        kcoloring_to_ilp(),
        kcoloring_to_qubo(),
        ksatisfiability_to_qubo(),
        ksatisfiability_to_satisfiability(),
        ksatisfiability_to_subsetsum(),
        longestcommonsubsequence_to_ilp(),
        maxcut_to_spinglass(),
        maximumclique_to_ilp(),
        maximumclique_to_maximumindependentset(),
        maximumindependentset_to_ilp(),
        maximumindependentset_to_maximumclique(),
        maximumindependentset_to_maximumsetpacking(),
        maximumindependentset_to_minimumvertexcover(),
        maximumindependentset_to_qubo(),
        maximummatching_to_ilp(),
        maximummatching_to_maximumsetpacking(),
        maximumsetpacking_to_ilp(),
        maximumsetpacking_to_maximumindependentset(),
        maximumsetpacking_to_qubo(),
        minimumdominatingset_to_ilp(),
        minimumsetcovering_to_ilp(),
        minimumvertexcover_to_ilp(),
        minimumvertexcover_to_maximumindependentset(),
        minimumvertexcover_to_minimumsetcovering(),
        minimumvertexcover_to_qubo(),
        qubo_to_ilp(),
        qubo_to_spinglass(),
        satisfiability_to_circuitsat(),
        satisfiability_to_kcoloring(),
        satisfiability_to_ksatisfiability(),
        satisfiability_to_maximumindependentset(),
        satisfiability_to_minimumdominatingset(),
        spinglass_to_maxcut(),
        spinglass_to_qubo(),
        travelingsalesman_to_ilp(),
        travelingsalesman_to_qubo(),
    ]
}
