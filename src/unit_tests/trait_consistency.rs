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
        &HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)])),
        "HamiltonianPath",
    );
    check_problem_trait(
        &OptimalLinearArrangement::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), 3),
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
        &KthBestSpanningTree::new(
            SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
            vec![1, 1, 1],
            1,
            2,
        ),
        "KthBestSpanningTree",
    );
    check_problem_trait(
        &ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]], 3),
        "ShortestCommonSupersequence",
    );
    check_problem_trait(
        &FlowShopScheduling::new(2, vec![vec![1, 2], vec![3, 4]], 10),
        "FlowShopScheduling",
    );
    check_problem_trait(
        &MinimumTardinessSequencing::new(3, vec![2, 3, 1], vec![(0, 2)]),
        "MinimumTardinessSequencing",
    );
}

#[test]
fn test_direction() {
    use crate::traits::OptimizationProblem;
    use crate::types::Direction;

    // Minimization problems
    assert_eq!(
        MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]).direction(),
        Direction::Minimize
    );
    assert_eq!(
        MinimumDominatingSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]).direction(),
        Direction::Minimize
    );
    assert_eq!(
        MinimumSetCovering::<i32>::new(2, vec![vec![0, 1]]).direction(),
        Direction::Minimize
    );
    assert_eq!(
        PaintShop::new(vec!["a", "a"]).direction(),
        Direction::Minimize
    );
    assert_eq!(
        QUBO::from_matrix(vec![vec![1.0]]).direction(),
        Direction::Minimize
    );
    assert_eq!(
        SpinGlass::new(1, vec![], vec![0.0]).direction(),
        Direction::Minimize
    );
    assert_eq!(
        BMF::new(vec![vec![true]], 1).direction(),
        Direction::Minimize
    );
    assert_eq!(Factoring::new(6, 2, 2).direction(), Direction::Minimize);
    assert_eq!(
        BicliqueCover::new(BipartiteGraph::new(2, 2, vec![(0, 0)]), 1).direction(),
        Direction::Minimize
    );

    // Maximization problems
    assert_eq!(
        MaximumIndependentSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximalIS::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaxCut::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximumMatching::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximumSetPacking::<i32>::new(vec![vec![0]]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximumClique::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]).direction(),
        Direction::Maximize
    );
}
