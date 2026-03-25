use crate::models::algebraic::*;
use crate::models::formula::*;
use crate::models::graph::*;
use crate::models::misc::*;
use crate::models::set::*;
use crate::topology::{BipartiteGraph, DirectedGraph, SimpleGraph};
use crate::traits::Problem;
use crate::variant::K3;

fn check_problem_trait<P: Problem>(problem: &P, name: &str) {
    let dims = problem.dims();
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
fn test_all_problems_implement_trait_correctly() {
    check_problem_trait(
        &MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]),
        "MaximumIndependentSet",
    );
    check_problem_trait(
        &MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]),
        "MinimumVertexCover",
    );
    check_problem_trait(
        &MaxCut::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32]),
        "MaxCut",
    );
    check_problem_trait(
        &KColoring::<K3, _>::new(SimpleGraph::new(3, vec![(0, 1)])),
        "KColoring",
    );
    check_problem_trait(
        &MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]),
        "MinimumDominatingSet",
    );
    check_problem_trait(
        &MaximalIS::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]),
        "MaximalIS",
    );
    check_problem_trait(
        &MaximumMatching::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32]),
        "MaximumMatching",
    );
    check_problem_trait(
        &BiconnectivityAugmentation::new(SimpleGraph::path(4), vec![(0, 3, 2)], 2),
        "BiconnectivityAugmentation",
    );
    check_problem_trait(
        &Satisfiability::new(3, vec![CNFClause::new(vec![1])]),
        "SAT",
    );
    check_problem_trait(
        &SpinGlass::new(3, vec![((0, 1), 1.0)], vec![0.0; 3]),
        "SpinGlass",
    );
    check_problem_trait(&QUBO::from_matrix(vec![vec![1.0; 3]; 3]), "QUBO");
    check_problem_trait(
        &MinimumSetCovering::<i32>::new(3, vec![vec![0, 1]]),
        "MinimumSetCovering",
    );
    check_problem_trait(
        &MaximumSetPacking::<i32>::new(vec![vec![0, 1]]),
        "MaximumSetPacking",
    );
    check_problem_trait(&PaintShop::new(vec!["a", "a"]), "PaintShop");
    check_problem_trait(&BMF::new(vec![vec![true]], 1), "BMF");
    check_problem_trait(
        &ConsecutiveBlockMinimization::new(vec![vec![true, false], vec![false, true]], 2),
        "ConsecutiveBlockMinimization",
    );
    check_problem_trait(
        &BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0)]), 1),
        "BicliqueCover",
    );
    check_problem_trait(
        &BalancedCompleteBipartiteSubgraph::new(
            BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
            2,
        ),
        "BalancedCompleteBipartiteSubgraph",
    );
    check_problem_trait(&Factoring::new(6, 2, 2), "Factoring");
    check_problem_trait(&Partition::new(vec![3, 1, 1, 2, 2, 1]), "Partition");
    check_problem_trait(
        &QuadraticAssignment::new(vec![vec![0, 1], vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
        "QuadraticAssignment",
    );

    let circuit = Circuit::new(vec![Assignment::new(
        vec!["x".to_string()],
        BooleanExpr::constant(true),
    )]);
    check_problem_trait(&CircuitSAT::new(circuit), "CircuitSAT");
    check_problem_trait(
        &StrongConnectivityAugmentation::new(
            DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
            vec![(0, 2, 1)],
            1,
        ),
        "StrongConnectivityAugmentation",
    );
    check_problem_trait(
        &KthBestSpanningTree::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
            vec![1, 1, 1],
            1,
            2,
        ),
        "KthBestSpanningTree",
    );
    check_problem_trait(
        &HamiltonianCircuit::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)])),
        "HamiltonianCircuit",
    );
    check_problem_trait(
        &MinMaxMulticenter::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            vec![1i32; 3],
            vec![1i32; 2],
            1,
            1,
        ),
        "MinMaxMulticenter",
    );
    check_problem_trait(
        &HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)])),
        "HamiltonianPath",
    );
    check_problem_trait(
        &ShortestWeightConstrainedPath::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            vec![1i32; 2],
            vec![1i32; 2],
            0,
            2,
            2,
            2,
        ),
        "ShortestWeightConstrainedPath",
    );
    check_problem_trait(
        &MultipleCopyFileAllocation::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
            vec![1; 3],
            vec![1; 3],
        ),
        "MultipleCopyFileAllocation",
    );
    check_problem_trait(
        &UndirectedTwoCommodityIntegralFlow::new(
            SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]),
            vec![1, 1, 2],
            0,
            3,
            1,
            3,
            1,
            1,
        ),
        "UndirectedTwoCommodityIntegralFlow",
    );
    check_problem_trait(
        &LengthBoundedDisjointPaths::new(
            SimpleGraph::new(4, vec![(0, 1), (1, 3), (0, 2), (2, 3)]),
            0,
            3,
            2,
            2,
        ),
        "LengthBoundedDisjointPaths",
    );
    check_problem_trait(
        &OptimalLinearArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)])),
        "OptimalLinearArrangement",
    );
    check_problem_trait(
        &IsomorphicSpanningTree::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
            SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        ),
        "IsomorphicSpanningTree",
    );
    check_problem_trait(
        &ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]),
        "ShortestCommonSupersequence",
    );
    check_problem_trait(
        &FlowShopScheduling::new(2, vec![vec![1, 2], vec![3, 4]], 10),
        "FlowShopScheduling",
    );
    check_problem_trait(
        &SequencingToMinimizeWeightedTardiness::new(vec![3, 4, 2], vec![2, 3, 1], vec![5, 8, 4], 4),
        "SequencingToMinimizeWeightedTardiness",
    );
    check_problem_trait(
        &MinimumTardinessSequencing::new(3, vec![2, 3, 1], vec![(0, 2)]),
        "MinimumTardinessSequencing",
    );
    check_problem_trait(
        &PartitionIntoPathsOfLength2::new(SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (3, 4), (4, 5)],
        )),
        "PartitionIntoPathsOfLength2",
    );
    check_problem_trait(
        &ResourceConstrainedScheduling::new(3, vec![20], vec![vec![6], vec![7], vec![7]], 2),
        "ResourceConstrainedScheduling",
    );
    check_problem_trait(
        &PartiallyOrderedKnapsack::new(vec![2, 3], vec![3, 2], vec![(0, 1)], 5),
        "PartiallyOrderedKnapsack",
    );
    check_problem_trait(
        &SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]),
        "SequencingWithReleaseTimesAndDeadlines",
    );
    check_problem_trait(
        &SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3),
        "SumOfSquaresPartition",
    );
    check_problem_trait(
        &ConsecutiveOnesSubmatrix::new(vec![vec![true, false], vec![false, true]], 1),
        "ConsecutiveOnesSubmatrix",
    );
}
