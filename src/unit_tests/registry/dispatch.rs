use crate::models::graph::MaximumIndependentSet;
use crate::models::misc::SubsetSum;
use crate::registry::{DynProblem, LoadedDynProblem};
use crate::topology::SimpleGraph;
use crate::{Problem, Solver};
use std::any::Any;

fn solve_subset_sum(any: &dyn Any) -> Option<(Vec<usize>, String)> {
    let p = any.downcast_ref::<SubsetSum>()?;
    let config = crate::BruteForce::new().find_satisfying(p)?;
    let eval = format!("{:?}", p.evaluate(&config));
    Some((config, eval))
}

#[test]
fn test_dyn_problem_blanket_impl_exposes_problem_metadata() {
    let problem = MaximumIndependentSet::new(SimpleGraph::new(3, vec![(0, 1)]), vec![1i32; 3]);
    let dyn_problem: &dyn DynProblem = &problem;

    assert_eq!(dyn_problem.problem_name(), "MaximumIndependentSet");
    assert_eq!(dyn_problem.num_variables_dyn(), 3);
    assert_eq!(dyn_problem.dims_dyn(), vec![2, 2, 2]);
    assert_eq!(dyn_problem.variant_map()["graph"], "SimpleGraph");
    assert!(dyn_problem.serialize_json().is_object());
}

#[test]
fn test_loaded_dyn_problem_delegates_to_solve_fn() {
    let problem = SubsetSum::new(vec![3u32, 7u32, 1u32], 4u32);
    let loaded = LoadedDynProblem::new(Box::new(problem), solve_subset_sum);
    let solved = loaded
        .solve_brute_force()
        .expect("expected satisfying solution");
    assert_eq!(solved.1, "true");
    assert_eq!(solved.0.len(), 3);
}
