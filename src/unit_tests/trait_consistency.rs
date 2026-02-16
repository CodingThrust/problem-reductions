use crate::models::graph::*;
use crate::models::optimization::*;
use crate::models::satisfiability::*;
use crate::models::set::*;
use crate::models::specialized::*;
use crate::topology::SimpleGraph;
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
        &MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1)]),
        "MinimumVertexCover",
    );
    check_problem_trait(
        &MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1)]),
        "MaxCut",
    );
    check_problem_trait(
        &KColoring::<K3, SimpleGraph>::new(3, vec![(0, 1)]),
        "KColoring",
    );
    check_problem_trait(
        &MinimumDominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]),
        "MinimumDominatingSet",
    );
    check_problem_trait(
        &MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]),
        "MaximalIS",
    );
    check_problem_trait(
        &MaximumMatching::<SimpleGraph, i32>::new(3, vec![(0, 1, 1)]),
        "MaximumMatching",
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
    check_problem_trait(&BicliqueCover::new(2, 2, vec![(0, 2)], 1), "BicliqueCover");
    check_problem_trait(&Factoring::new(6, 2, 2), "Factoring");

    let circuit = Circuit::new(vec![Assignment::new(
        vec!["x".to_string()],
        BooleanExpr::constant(true),
    )]);
    check_problem_trait(&CircuitSAT::new(circuit), "CircuitSAT");
}

#[test]
fn test_direction() {
    use crate::traits::OptimizationProblem;
    use crate::types::Direction;

    // Minimization problems
    assert_eq!(
        MinimumVertexCover::<SimpleGraph, i32>::new(2, vec![(0, 1)]).direction(),
        Direction::Minimize
    );
    assert_eq!(
        MinimumDominatingSet::<SimpleGraph, i32>::new(2, vec![(0, 1)]).direction(),
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
        BicliqueCover::new(2, 2, vec![(0, 2)], 1).direction(),
        Direction::Minimize
    );

    // Maximization problems
    assert_eq!(
        MaximumIndependentSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i32; 2]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximalIS::<SimpleGraph, i32>::new(2, vec![(0, 1)]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaxCut::<SimpleGraph, i32>::new(2, vec![(0, 1, 1)]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximumMatching::<SimpleGraph, i32>::new(2, vec![(0, 1, 1)]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximumSetPacking::<i32>::new(vec![vec![0]]).direction(),
        Direction::Maximize
    );
    assert_eq!(
        MaximumClique::<SimpleGraph, i32>::new(2, vec![(0, 1)]).direction(),
        Direction::Maximize
    );
}
