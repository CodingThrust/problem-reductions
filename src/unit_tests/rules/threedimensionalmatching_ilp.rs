use super::*;
use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::models::misc::{ResourceConstrainedScheduling, ThreePartition};
use crate::models::set::ThreeDimensionalMatching;
use crate::rules::{MinimizeSteps, ReduceTo, ReductionGraph, ReductionResult};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::{Or, ProblemSize};

fn canonical_problem() -> ThreeDimensionalMatching {
    ThreeDimensionalMatching::new(
        3,
        vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
    )
}

fn singleton_problem() -> ThreeDimensionalMatching {
    ThreeDimensionalMatching::new(1, vec![(0, 0, 0)])
}

fn constraint_signature(constraint: &(Comparison, f64, Vec<(usize, f64)>)) -> String {
    let cmp = match constraint.0 {
        Comparison::Le => "<=",
        Comparison::Ge => ">=",
        Comparison::Eq => "=",
    };
    let terms = constraint
        .2
        .iter()
        .map(|&(var, coeff)| format!("{var}:{}", (coeff * 1_000_000.0).round() as i64))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{cmp}|{}|{terms}",
        (constraint.1 * 1_000_000.0).round() as i64
    )
}

#[test]
fn test_threedimensionalmatching_to_ilp_structure() {
    let problem = canonical_problem();
    let reduction: ReductionThreeDimensionalMatchingToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 5);
    assert_eq!(ilp.constraints.len(), 9);
    assert!(ilp.objective.is_empty());
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    type Constraint = (Comparison, f64, Vec<(usize, f64)>);
    let actual_constraints: Vec<Constraint> = ilp
        .constraints
        .iter()
        .map(|constraint| {
            let mut terms = constraint.terms.clone();
            terms.sort_by_key(|(var, _)| *var);
            (constraint.cmp, constraint.rhs, terms)
        })
        .collect();
    let expected_constraints = vec![
        (Comparison::Eq, 1.0, vec![(0, 1.0), (3, 1.0)]),
        (Comparison::Eq, 1.0, vec![(1, 1.0), (4, 1.0)]),
        (Comparison::Eq, 1.0, vec![(2, 1.0)]),
        (Comparison::Eq, 1.0, vec![(1, 1.0), (3, 1.0)]),
        (Comparison::Eq, 1.0, vec![(0, 1.0)]),
        (Comparison::Eq, 1.0, vec![(2, 1.0), (4, 1.0)]),
        (Comparison::Eq, 1.0, vec![(2, 1.0), (3, 1.0)]),
        (Comparison::Eq, 1.0, vec![(1, 1.0)]),
        (Comparison::Eq, 1.0, vec![(0, 1.0), (4, 1.0)]),
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
        ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_witness = BruteForce::new()
        .find_witness(&problem)
        .expect("canonical 3DM instance should be feasible");
    assert_eq!(bf_witness, vec![1, 1, 1, 0, 0]);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("direct ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, vec![1, 1, 1, 0, 0]);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_threedimensionalmatching_to_ilp_infeasible_instance() {
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (0, 1, 1)]);
    let reduction: ReductionThreeDimensionalMatchingToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert!(
        BruteForce::new().find_witness(&problem).is_none(),
        "source instance should be infeasible"
    );
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "reduced ILP should be infeasible"
    );
}

#[test]
fn test_threedimensionalmatching_to_ilp_direct_path_beats_indirect_chain() {
    let problem = singleton_problem();
    let direct = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let to_three_partition = ReduceTo::<ThreePartition>::reduce_to(&problem);
    let to_resource_constrained =
        ReduceTo::<ResourceConstrainedScheduling>::reduce_to(to_three_partition.target_problem());
    let indirect = ReduceTo::<ILP<bool>>::reduce_to(to_resource_constrained.target_problem());

    let solver = ILPSolver::new();
    let direct_solution = solver
        .solve(direct.target_problem())
        .expect("direct ILP should solve");
    let direct_source = direct.extract_solution(&direct_solution);

    assert_eq!(problem.evaluate(&direct_source), Or(true));
    assert!(
        solver.solve(indirect.target_problem()).is_some(),
        "indirect ILP should agree on feasibility"
    );
    assert!(direct.target_problem().num_vars < indirect.target_problem().num_vars);
    assert!(
        direct.target_problem().constraints.len() < indirect.target_problem().constraints.len()
    );

    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&ThreeDimensionalMatching::variant());
    let dst = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let path = graph
        .find_cheapest_path(
            "ThreeDimensionalMatching",
            &src,
            "ILP",
            &dst,
            &ProblemSize::new(vec![
                ("universe_size", problem.universe_size()),
                ("num_triples", problem.num_triples()),
            ]),
            &MinimizeSteps,
        )
        .expect("reduction graph should find a direct 3DM -> ILP path");

    assert_eq!(path.type_names(), vec!["ThreeDimensionalMatching", "ILP"]);
}
