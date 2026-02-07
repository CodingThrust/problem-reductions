use crate::models::graph::*;
use crate::models::optimization::*;
use crate::models::satisfiability::*;
use crate::models::set::*;
use crate::models::specialized::*;
use crate::prelude::*;
use crate::topology::SimpleGraph;

fn check_problem_trait<P: Problem>(problem: &P, name: &str)
where
    P::Size: std::fmt::Debug,
{
    assert!(
        problem.num_variables() > 0 || name.contains("empty"),
        "{} should have variables",
        name
    );
    assert!(
        problem.num_flavors() >= 2,
        "{} should have at least 2 flavors",
        name
    );

    let size = problem.problem_size();
    // Check that problem_size returns some meaningful data
    assert!(
        size.get("num_vertices").is_some()
            || size.get("num_vars").is_some()
            || size.get("num_sets").is_some()
            || size.get("num_cars").is_some()
            || size.get("rows").is_some()
            || size.get("left_size").is_some()
            || size.get("target").is_some()
            || size.get("num_variables").is_some()
            || size.get("num_colors").is_some()
            || size.get("num_spins").is_some()
            || size.get("num_edges").is_some(),
        "{} problem_size should have meaningful data",
        name
    );
}

#[test]
fn test_all_problems_implement_trait_correctly() {
    check_problem_trait(
        &IndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]),
        "IndependentSet",
    );
    check_problem_trait(
        &VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1)]),
        "VertexCovering",
    );
    check_problem_trait(&MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1)]), "MaxCut");
    check_problem_trait(&KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1)]), "KColoring");
    check_problem_trait(&DominatingSet::<SimpleGraph, i32>::new(3, vec![(0, 1)]), "DominatingSet");
    check_problem_trait(&MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]), "MaximalIS");
    check_problem_trait(&Matching::<SimpleGraph, i32>::new(3, vec![(0, 1, 1)]), "Matching");
    check_problem_trait(
        &Satisfiability::<i32>::new(3, vec![CNFClause::new(vec![1])]),
        "SAT",
    );
    check_problem_trait(
        &SpinGlass::new(3, vec![((0, 1), 1.0)], vec![0.0; 3]),
        "SpinGlass",
    );
    check_problem_trait(&QUBO::from_matrix(vec![vec![1.0; 3]; 3]), "QUBO");
    check_problem_trait(&SetCovering::<i32>::new(3, vec![vec![0, 1]]), "SetCovering");
    check_problem_trait(&SetPacking::<i32>::new(vec![vec![0, 1]]), "SetPacking");
    check_problem_trait(&PaintShop::new(vec!["a", "a"]), "PaintShop");
    check_problem_trait(&BMF::new(vec![vec![true]], 1), "BMF");
    check_problem_trait(&BicliqueCover::new(2, 2, vec![(0, 2)], 1), "BicliqueCover");
    check_problem_trait(&Factoring::new(6, 2, 2), "Factoring");

    let circuit = Circuit::new(vec![Assignment::new(
        vec!["x".to_string()],
        BooleanExpr::constant(true),
    )]);
    check_problem_trait(&CircuitSAT::<i32>::new(circuit), "CircuitSAT");
}

#[test]
fn test_energy_modes() {
    // Minimization problems
    assert!(VertexCovering::<SimpleGraph, i32>::new(2, vec![(0, 1)])
        .energy_mode()
        .is_minimization());
    assert!(DominatingSet::<SimpleGraph, i32>::new(2, vec![(0, 1)])
        .energy_mode()
        .is_minimization());
    assert!(SetCovering::<i32>::new(2, vec![vec![0, 1]])
        .energy_mode()
        .is_minimization());
    assert!(PaintShop::new(vec!["a", "a"])
        .energy_mode()
        .is_minimization());
    assert!(QUBO::from_matrix(vec![vec![1.0]])
        .energy_mode()
        .is_minimization());
    assert!(SpinGlass::new(1, vec![], vec![0.0])
        .energy_mode()
        .is_minimization());
    assert!(BMF::new(vec![vec![true]], 1)
        .energy_mode()
        .is_minimization());
    assert!(Factoring::new(6, 2, 2).energy_mode().is_minimization());
    assert!(KColoring::<2, SimpleGraph, i32>::new(2, vec![(0, 1)])
        .energy_mode()
        .is_minimization());
    assert!(BicliqueCover::new(2, 2, vec![(0, 2)], 1)
        .energy_mode()
        .is_minimization());

    // Maximization problems
    assert!(IndependentSet::<SimpleGraph, i32>::new(2, vec![(0, 1)])
        .energy_mode()
        .is_maximization());
    assert!(MaximalIS::<SimpleGraph, i32>::new(2, vec![(0, 1)])
        .energy_mode()
        .is_maximization());
    assert!(MaxCut::<SimpleGraph, i32>::new(2, vec![(0, 1, 1)])
        .energy_mode()
        .is_maximization());
    assert!(Matching::<SimpleGraph, i32>::new(2, vec![(0, 1, 1)])
        .energy_mode()
        .is_maximization());
    assert!(SetPacking::<i32>::new(vec![vec![0]])
        .energy_mode()
        .is_maximization());
    assert!(Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])])
        .energy_mode()
        .is_maximization());

    let circuit = Circuit::new(vec![]);
    assert!(CircuitSAT::<i32>::new(circuit)
        .energy_mode()
        .is_maximization());
}
