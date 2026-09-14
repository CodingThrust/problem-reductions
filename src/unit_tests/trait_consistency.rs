use crate::solvers::BruteForceProblem;
use crate::models::algebraic::*;
use crate::models::formula::*;
use crate::models::graph::*;
use crate::models::misc::*;
use crate::models::set::*;
use crate::topology::{BipartiteGraph, DirectedGraph, SimpleGraph};
use crate::variant::K3;

fn check_brute_force_problem<P: BruteForceProblem>(problem: &P, name: &str) {
    let dims = crate::solvers::cartesian_dimensions(&problem).unwrap();
    assert!(
        !dims.is_empty() || name.contains("empty"),
        "{} should have dimensions",
        name
    );
    for d in &dims {
        assert!(
            *d >= 2,
            "{} should have at least 2 choices per dimension",
            name
        );
    }
}
#[test]
fn test_all_registered_brute_force_problems_define_dimensions() {
    check_brute_force_problem(
        &MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]).unwrap(), vec![1i64; 3]).unwrap(),
        "MaximumIndependentSet",
    );
    check_brute_force_problem(
        &MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1)]).unwrap(), vec![1i64; 3]).unwrap(),
        "MinimumVertexCover",
    );
    check_brute_force_problem(
        &MaxCut::new(SimpleGraph::new(3, vec![(0, 1)]).unwrap(), vec![1i64]).unwrap(),
        "MaxCut",
    );
    check_brute_force_problem(
        &KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1)]).unwrap()),
        "KColoring",
    );
    check_brute_force_problem(
        &MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]).unwrap(), vec![1i64; 3]).unwrap(),
        "MinimumDominatingSet",
    );
    check_brute_force_problem(
        &MaximalIS::new(SimpleGraph::new(3, vec![(0, 1)]).unwrap(), vec![1i64; 3]).unwrap(),
        "MaximalIS",
    );
    check_brute_force_problem(
        &MaximumMatching::new(SimpleGraph::new(3, vec![(0, 1)]).unwrap(), vec![1i64]).unwrap(),
        "MaximumMatching",
    );
    check_brute_force_problem(
        &BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 3, 2)], 2).unwrap(),
        "BiconnectivityAugmentation",
    );
    check_brute_force_problem(
        &Satisfiability::new(3, vec![CNFClause::new(vec![1])]),
        "SAT",
    );
    check_brute_force_problem(
        &SpinGlass::new(3, vec![((0, 1), 1.0)], vec![0.0; 3]).unwrap(),
        "SpinGlass",
    );
    check_brute_force_problem(
        &QUBO::from_matrix(vec![vec![1.0; 3]; 3]).unwrap(),
        "QUBO",
    );
    check_brute_force_problem(
        &MinimumSetCovering::new(3, vec![vec![0, 1]]).unwrap(),
        "MinimumSetCovering",
    );
    check_brute_force_problem(
        &MaximumSetPacking::new(vec![vec![0, 1]]),
        "MaximumSetPacking",
    );
    check_brute_force_problem(&PaintShop::new(vec!["a", "a"]).unwrap(), "PaintShop");
    check_brute_force_problem(&BMF::new(vec![vec![true]], 1).unwrap(), "BMF");
    check_brute_force_problem(
        &ConsecutiveBlockMinimization::new(vec![vec![true, false], vec![false, true]], 2).unwrap(),
        "ConsecutiveBlockMinimization",
    );
    check_brute_force_problem(
        &BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0)]).unwrap(), 1),
        "BicliqueCover",
    );
    check_brute_force_problem(
        &BalancedCompleteBipartiteSubgraph::new(
            BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]).unwrap(),
            2,
        ),
        "BalancedCompleteBipartiteSubgraph",
    );
    check_brute_force_problem(&Factoring::with_factor_bits(2, 2, 6), "Factoring");
    check_brute_force_problem(&Partition::new(vec![3, 1, 1, 2, 2, 1]), "Partition").unwrap();
    check_brute_force_problem(
        &QuadraticAssignment::new(vec![vec![0, 1], vec![1, 0]], vec![vec![0, 1], vec![1, 0]]).unwrap(),
        "QuadraticAssignment",
    );

    let circuit = Circuit::new(vec![Assignment::new(
        vec!["x".to_string()],
        BooleanExpr::constant(true),
    )]);
    check_brute_force_problem(&CircuitSAT::new(circuit), "CircuitSAT");
    check_brute_force_problem(
        &StrongConnectivityAugmentation::new(
            DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]).unwrap(),
            vec![(0, 2, 1)],
            1,
        ),
        "StrongConnectivityAugmentation",
    );
    check_brute_force_problem(
        &KthBestSpanningTree::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]).unwrap(),
            vec![1, 1, 1],
            1,
            2,
        ).unwrap(),
        "KthBestSpanningTree",
    );
    check_brute_force_problem(
        &HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]).unwrap()),
        "HamiltonianCircuit",
    );
    check_brute_force_problem(
        &MinMaxMulticenter::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap(),
            vec![1i64; 3],
            vec![1i64; 2],
            1,
        ).unwrap(),
        "MinMaxMulticenter",
    );
    check_brute_force_problem(
        &HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap()),
        "HamiltonianPath",
    );
    check_brute_force_problem(
        &DegreeConstrainedSpanningTree::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap(), 2).unwrap(),
        "DegreeConstrainedSpanningTree",
    );
    check_brute_force_problem(
        &ShortestWeightConstrainedPath::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap(),
            vec![1i64; 2],
            vec![1i64; 2],
            0,
            2,
            2,
            2,
        ).unwrap(),
        "ShortestWeightConstrainedPath",
    );
    check_brute_force_problem(
        &MultipleCopyFileAllocation::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap(),
            vec![1; 3],
            vec![1; 3],
        ).unwrap(),
        "MultipleCopyFileAllocation",
    );
    check_brute_force_problem(
        &UndirectedTwoCommodityIntegralFlow::new(
            SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]).unwrap(),
            vec![1, 1, 2],
            0,
            3,
            1,
            3,
            1,
            1,
        ).unwrap(),
        "UndirectedTwoCommodityIntegralFlow",
    );
    check_brute_force_problem(
        &LengthBoundedDisjointPaths::new(
            SimpleGraph::new(4, vec![(0, 1), (1, 3), (0, 2), (2, 3)]).unwrap(),
            0,
            3,
            2,
            2,
        ).unwrap(),
        "LengthBoundedDisjointPaths",
    );
    check_brute_force_problem(
        &OptimalLinearArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap()),
        "OptimalLinearArrangement",
    );
    check_brute_force_problem(
        &IsomorphicSpanningTree::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]).unwrap(),
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]).unwrap(),
        ),
        "IsomorphicSpanningTree",
    );
    check_brute_force_problem(
        &ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]).unwrap(),
        "ShortestCommonSupersequence",
    );
    check_brute_force_problem(
        &FlowShopScheduling::new(2, vec![vec![1, 2], vec![3, 4]], 10).unwrap(),
        "FlowShopScheduling",
    );
    check_brute_force_problem(
        &JobShopScheduling::new(2, vec![vec![(0, 1), (1, 1)], vec![(1, 1), (0, 1)]], 2).unwrap(),
        "JobShopScheduling",
    );
    check_brute_force_problem(
        &SequencingToMinimizeWeightedTardiness::new(vec![3, 4, 2], vec![2, 3, 1], vec![5, 8, 4], 4).unwrap(),
        "SequencingToMinimizeWeightedTardiness",
    );
    check_brute_force_problem(
        &MinimumTardinessSequencing::<One>::new(3, vec![2, 3, 1], vec![(0, 2)]).unwrap(),
        "MinimumTardinessSequencing",
    );
    check_brute_force_problem(
        &PartitionIntoPathsOfLength2::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (3, 4), (4, 5)],
        ).unwrap()).unwrap(),
        "PartitionIntoPathsOfLength2",
    );
    check_brute_force_problem(
        &ResourceConstrainedScheduling::new(3, vec![20], vec![vec![6], vec![7], vec![7]], 2)
            .unwrap(),
        "ResourceConstrainedScheduling",
    );
    check_brute_force_problem(
        &PartiallyOrderedKnapsack::new(vec![2, 3], vec![3, 2], vec![(0, 1)], 5).unwrap(),
        "PartiallyOrderedKnapsack",
    );
    check_brute_force_problem(
        &SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]).unwrap(),
        "SequencingWithReleaseTimesAndDeadlines",
    );
    check_brute_force_problem(
        &SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3),
        "SumOfSquaresPartition",
    );
    check_brute_force_problem(
        &ConsecutiveOnesSubmatrix::new(vec![vec![true, false], vec![false, true]], 1).unwrap(),
        "ConsecutiveOnesSubmatrix",
    );
}
