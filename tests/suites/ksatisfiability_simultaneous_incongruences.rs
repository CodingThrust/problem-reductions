use problemreductions::Problem;
use problemreductions::models::algebraic::SimultaneousIncongruences;
use problemreductions::models::formula::{CNFClause, KSatisfiability};
use problemreductions::rules::{ReduceTo, ReductionGraph, ReductionResult};
use problemreductions::solvers::BruteForce;
use problemreductions::variant::K3;

#[test]
fn test_ksatisfiability_to_simultaneous_incongruences_closed_loop() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, 3]),
        ],
    );

    let reduction = ReduceTo::<SimultaneousIncongruences>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.lcm_moduli(), 105);
    assert_eq!(target.num_pairs(), 11);

    let graph = ReductionGraph::new();
    let source_variant = ReductionGraph::variant_to_map(&KSatisfiability::<K3>::variant());
    let target_variant = ReductionGraph::variant_to_map(&SimultaneousIncongruences::variant());
    let entry = graph
        .find_entry(
            KSatisfiability::<K3>::NAME,
            &source_variant,
            SimultaneousIncongruences::NAME,
            &target_variant,
        )
        .unwrap();
    let contract = entry.parameter_contract.unwrap();
    let transform = contract.transform().unwrap();
    let complexity = graph
        .variant_complexity(SimultaneousIncongruences::NAME, &target_variant)
        .unwrap();
    let source_cost = problemreductions::Expr::parse(complexity)
        .substitute_complete(&transform.expressions().collect())
        .unwrap();
    assert!(!source_cost.is_polynomial());
    assert_eq!(target.parameters().get("lcm_bits"), Some(7));

    let solver = BruteForce::new();
    let target_solution = solver
        .solve(target)
        .unwrap()
        .expect("target should be satisfiable");
    let extracted = reduction.extract_solution(&target_solution).unwrap();

    assert!(source.evaluate(&extracted).unwrap());
}
