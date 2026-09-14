use super::*;
use crate::solvers::SolveOutcome;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = MaximumSetPacking::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars(), 3, "Should have one variable per set");
    // Elements 1 and 2 each appear in 2 sets → 2 element constraints
    assert_eq!(
        ilp.constraints().len(),
        2,
        "Should have one constraint per shared element"
    );
    assert_eq!(ilp.sense(), ObjectiveSense::Maximize, "Should maximize");

    for constraint in ilp.constraints() {
        assert!(constraint.terms().len() >= 2);
        assert_eq!(constraint.rhs(), 1);
    }
}

#[test]
fn test_reduction_weighted() {
    let problem =
        MaximumSetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]).unwrap();
    let reduction: ReductionSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let mut coeffs: Vec<i64> = vec![0; 2];
    for &(var, coef) in ilp.objective() {
        coeffs[var] = coef;
    }
    assert_eq!(coeffs, vec![5, 10]);
}

#[test]
fn test_maximumsetpacking_to_ilp_closed_loop() {
    let problem = MaximumSetPacking::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction
        .recover_result(
            &problem,
            SolveOutcome::optimal(reduction.target_problem(), ilp_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

    let bf_size: usize = bf_solutions[0].iter().filter(|&&selected| selected).count();
    let ilp_size: usize = extracted.iter().filter(|&&selected| selected).count();
    assert_eq!(bf_size, 2);
    assert_eq!(ilp_size, 2);

    assert!(
        problem.evaluate(&extracted).unwrap().is_valid(),
        "Extracted solution should be valid"
    );
}

#[test]
fn test_ilp_solution_equals_brute_force_weighted() {
    let problem = MaximumSetPacking::with_weights(
        vec![vec![0, 1, 2, 3], vec![0, 1], vec![2, 3]],
        vec![5, 3, 3],
    )
    .unwrap();
    let reduction: ReductionSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_solutions = bf.find_all_witnesses(&problem).unwrap();
    let bf_obj = problem.evaluate(&bf_solutions[0]).unwrap();

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction
        .recover_result(
            &problem,
            SolveOutcome::optimal(reduction.target_problem(), ilp_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
    let ilp_obj = problem.evaluate(&extracted).unwrap();

    assert_eq!(bf_obj, Max(Some(6)));
    assert_eq!(ilp_obj, Max(Some(6)));
    assert_eq!(extracted, vec![false, true, true]);
}

#[test]
fn test_solution_extraction() {
    let problem = MaximumSetPacking::new(vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7]]);
    let reduction: ReductionSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let ilp_solution = vec![1, 0, 1, 0];
    let extracted = reduction
        .recover_result(
            &problem,
            SolveOutcome::optimal(reduction.target_problem(), ilp_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");
    assert_eq!(extracted, vec![true, false, true, false]);
    assert!(problem.evaluate(&extracted).unwrap().is_valid());
}

#[test]
fn test_disjoint_sets() {
    let problem = MaximumSetPacking::new(vec![vec![0], vec![1], vec![2], vec![3]]);
    let reduction: ReductionSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    assert_eq!(ilp.constraints().len(), 0);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction
        .recover_result(
            &problem,
            SolveOutcome::optimal(reduction.target_problem(), ilp_solution.clone()).unwrap(),
        )
        .unwrap()
        .into_solution()
        .expect("qualifying target result must recover a source solution");

    assert_eq!(extracted, vec![true, true, true, true]);
    assert!(problem.evaluate(&extracted).unwrap().is_valid());
    assert_eq!(problem.evaluate(&extracted).unwrap(), Max(Some(4)));
}

#[test]
fn test_solve_via_ilp_pipeline() {
    let problem: MaximumSetPacking<i64> =
        MaximumSetPacking::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver
        .solve(&problem)
        .expect("ILP pipeline should solve the problem");

    assert!(problem.evaluate(&solution).unwrap().is_valid());
    assert_eq!(problem.evaluate(&solution).unwrap(), Max(Some(2)));
}

#[test]
fn test_maximumsetpacking_to_ilp_bf_vs_ilp() {
    let problem = MaximumSetPacking::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let reduction: ReductionSPToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn extraction_maps_feasible_witnesses_through_typed_and_dynamic_paths() {
    use crate::rules::{DynReductionResult, ReductionGraph};
    use serde_json::json;

    let source = MaximumSetPacking::with_weights(vec![vec![0]], vec![1i64]).unwrap();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    let graph = ReductionGraph::new();
    let path = graph
        .find_all_paths(
            MaximumSetPacking::<i64>::NAME,
            &ReductionGraph::variant_to_map(&MaximumSetPacking::<i64>::variant()),
            ILP::<bool>::NAME,
            &ReductionGraph::variant_to_map(&ILP::<bool>::variant()),
        )
        .into_iter()
        .find(|path| path.len() == 1)
        .unwrap();
    let chain = graph.reduce_along_path(&path, &source).unwrap().unwrap();
    assert_eq!(
        reduction
            .recover_result(
                &source,
                SolveOutcome::optimal(reduction.target_problem(), vec![1].clone()).unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        vec![true]
    );
    let extracted = reduction
        .recover_result_dyn(
            &source,
            crate::solvers::erase_outcome(
                SolveOutcome::optimal(reduction.target_problem(), vec![1i64]).unwrap(),
            ),
        )
        .unwrap();
    assert_eq!(
        crate::solvers::downcast_outcome::<Vec<bool>, crate::types::Max<i64>>(extracted)
            .unwrap()
            .into_solution()
            .unwrap(),
        vec![true]
    );
    // An unselected set is feasible even though it is not optimal.
    assert_eq!(
        reduction
            .recover_result(
                &source,
                SolveOutcome::feasible(reduction.target_problem(), vec![0].clone()).unwrap()
            )
            .unwrap()
            .into_solution()
            .expect("qualifying target result must recover a source solution"),
        vec![false]
    );
    assert_eq!(
        chain
            .recover_result_json(
                &source,
                SolveOutcome::Feasible {
                    solution: json!([0]),
                    evaluation: String::new(),
                }
            )
            .unwrap()
            .0,
        SolveOutcome::Feasible {
            solution: json!([false]),
            evaluation: "Max(0)".into()
        }
    );
}

#[test]
fn parameter_upper_bounds_cover_single_and_shared_elements() {
    use crate::parameters::ParameterRelation;
    use crate::rules::registry::ReductionEntry;
    let entry = inventory::iter::<ReductionEntry>
        .into_iter()
        .find(|entry| {
            entry.source_name == MaximumSetPacking::<i64>::NAME
                && entry.target_name == ILP::<bool>::NAME
                && (entry.source_variant_fn)() == MaximumSetPacking::<i64>::variant()
                && (entry.target_variant_fn)() == ILP::<bool>::variant()
        })
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    let transform = contract.transform().unwrap();
    assert_eq!(transform.relation(), ParameterRelation::UpperBound);
    for (sets, constraints) in [
        (vec![vec![0]], 0),
        (vec![vec![0, 1], vec![1, 2], vec![2, 3]], 2),
    ] {
        let source = MaximumSetPacking::<i64>::new(sets);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let actual = reduction.target_problem().parameters();
        let declared = transform.evaluate(&source.parameters()).unwrap();
        assert_eq!(actual.get("num_constraints"), Some(constraints));
        assert_eq!(actual.get("num_vars"), declared.get("num_vars"));
        assert!(actual.get("num_constraints").unwrap() <= declared.get("num_constraints").unwrap());
    }
}
