use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::models::misc::{ResourceConstrainedScheduling, ThreePartition};
use crate::models::set::ThreeDimensionalMatching;
use crate::rules::{ReduceTo, ReductionGraph, ReductionResult};
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

fn canonical_problem() -> ThreeDimensionalMatching {
    ThreeDimensionalMatching::new(
        3,
        vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
    )
}

fn singleton_problem() -> ThreeDimensionalMatching {
    ThreeDimensionalMatching::new(1, vec![(0, 0, 0)])
}

fn constraint_signature(constraint: &(Comparison, i64, Vec<(usize, i64)>)) -> String {
    let cmp = match constraint.0 {
        Comparison::Le => "<=",
        Comparison::Ge => ">=",
        Comparison::Eq => "=",
    };
    let terms = constraint
        .2
        .iter()
        .map(|&(variable, coefficient)| format!("{variable}:{coefficient}"))
        .collect::<Vec<_>>()
        .join(",");
    format!("{cmp}|{}|{terms}", constraint.1)
}

#[test]
fn test_threedimensionalmatching_to_ilp_structure() {
    let problem = canonical_problem();
    let reduction: ReductionThreeDimensionalMatchingToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 5);
    assert_eq!(ilp.constraints().len(), 9);
    assert!(ilp.objective().is_empty());
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);

    type Constraint = (Comparison, i64, Vec<(usize, i64)>);
    let actual_constraints: Vec<Constraint> = ilp
        .constraints()
        .iter()
        .map(|constraint| {
            let mut terms = constraint.terms().to_vec();
            terms.sort_by_key(|(var, _)| *var);
            (constraint.comparison(), constraint.rhs(), terms)
        })
        .collect();
    let expected_constraints = vec![
        (Comparison::Eq, 1, vec![(0, 1), (3, 1)]),
        (Comparison::Eq, 1, vec![(1, 1), (4, 1)]),
        (Comparison::Eq, 1, vec![(2, 1)]),
        (Comparison::Eq, 1, vec![(1, 1), (3, 1)]),
        (Comparison::Eq, 1, vec![(0, 1)]),
        (Comparison::Eq, 1, vec![(2, 1), (4, 1)]),
        (Comparison::Eq, 1, vec![(2, 1), (3, 1)]),
        (Comparison::Eq, 1, vec![(1, 1)]),
        (Comparison::Eq, 1, vec![(0, 1), (4, 1)]),
    ];

    let mut actual_signatures: Vec<_> = actual_constraints
        .iter()
        .map(constraint_signature)
        .collect();
    let mut expected_signatures: Vec<_> = expected_constraints
        .iter()
        .map(constraint_signature)
        .collect();
    actual_signatures.sort();
    expected_signatures.sort();

    assert_eq!(actual_signatures, expected_signatures);
}

#[test]
fn test_threedimensionalmatching_to_ilp_closed_loop() {
    let problem = canonical_problem();
    let reduction: ReductionThreeDimensionalMatchingToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let bf_witness = BruteForce::new()
        .solve(&problem)
        .unwrap()
        .expect("canonical 3DM instance should be feasible");
    assert_eq!(bf_witness, vec![true, true, true, false, false]);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("direct ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert_eq!(extracted, vec![true, true, true, false, false]);
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_threedimensionalmatching_to_ilp_infeasible_instance() {
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (0, 1, 1)]);
    let reduction: ReductionThreeDimensionalMatchingToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert!(
        BruteForce::new().solve(&problem).unwrap().is_none(),
        "source instance should be infeasible"
    );
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_err(),
        "reduced ILP should be infeasible"
    );
}

#[test]
fn test_threedimensionalmatching_to_ilp_direct_path_beats_indirect_chain() {
    let problem = singleton_problem();
    let direct = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let to_three_partition =
        ReduceTo::<ThreePartition>::reduce_to(&problem).expect("reduction should succeed");
    let to_resource_constrained =
        ReduceTo::<ResourceConstrainedScheduling>::reduce_to(to_three_partition.target_problem())
            .expect("reduction should succeed");
    let indirect = ReduceTo::<ILP<bool>>::reduce_to(to_resource_constrained.target_problem())
        .expect("reduction should succeed");

    let solver = ILPSolver::new();
    let direct_solution = solver
        .solve(direct.target_problem())
        .expect("direct ILP should solve");
    let direct_source = direct.extract_solution(&direct_solution).unwrap();

    assert_eq!(problem.evaluate(&direct_source).unwrap(), Or(true));
    let indirect_solution = solver.solve(indirect.target_problem());
    assert!(
        matches!(indirect_solution, Err(ILPSolveError::InvalidSolution(_))),
        "the numerically unstable indirect ILP should be rejected: {indirect_solution:?}"
    );
    assert!(direct.target_problem().num_vars() < indirect.target_problem().num_vars());
    assert!(
        direct.target_problem().constraints().len() < indirect.target_problem().constraints().len()
    );

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&ThreeDimensionalMatching::variant());
    let dst = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph
        .find_all_paths("ThreeDimensionalMatching", &src, "ILP", &dst)
        .into_iter()
        .find(|path| path.type_names() == ["ThreeDimensionalMatching", "ILP"])
        .expect("reduction graph should contain the direct 3DM -> ILP path");

    assert_eq!(path.type_names(), vec!["ThreeDimensionalMatching", "ILP"]);
}
