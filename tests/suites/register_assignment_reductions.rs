use problemreductions::models::algebraic::ILP;
use problemreductions::models::formula::{CNFClause, KSatisfiability};
use problemreductions::models::misc::FeasibleRegisterAssignment;
use problemreductions::prelude::*;
use problemreductions::rules::{ReductionGraph, ReductionPath};
use problemreductions::solvers::ILPSolver;
use problemreductions::types::Or;
use problemreductions::variant::K3;

fn ksat_to_fra_path() -> ReductionPath {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&KSatisfiability::<K3>::variant());
    let dst = ReductionGraph::variant_to_map(&FeasibleRegisterAssignment::variant());
    graph
        .find_all_paths("KSatisfiability", &src, "FeasibleRegisterAssignment", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("expected direct route")
}

fn fra_to_ilp_path() -> ReductionPath {
    let graph = ReductionGraph::new();
    let src = ReductionGraph::variant_to_map(&FeasibleRegisterAssignment::variant());
    let dst = ReductionGraph::variant_to_map(&ILP::<i64>::variant());
    graph
        .find_all_paths("FeasibleRegisterAssignment", &src, "ILP", &dst)
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("expected direct route")
}

#[test]
fn test_ksat_to_fra_structure_and_closed_loop_via_ilp() {
    let source = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    );

    let graph = ReductionGraph::new();
    let ksat_path = ksat_to_fra_path();
    assert_eq!(
        ksat_path.type_names(),
        vec!["KSatisfiability", "FeasibleRegisterAssignment"]
    );
    let ksat_chain = graph
        .reduce_along_path(&ksat_path, &source as &dyn std::any::Any)
        .expect("KSAT -> FRA reduction should not fail")
        .expect("KSAT -> FRA reduction should execute");
    let fra = ksat_chain.target_problem::<FeasibleRegisterAssignment>();

    assert_eq!(fra.num_vertices(), 30);
    assert_eq!(fra.num_arcs(), 30);
    assert_eq!(fra.num_registers(), 21);

    let fra_path = fra_to_ilp_path();
    assert_eq!(
        fra_path.type_names(),
        vec!["FeasibleRegisterAssignment", "ILP"]
    );
    let fra_chain = graph
        .reduce_along_path(&fra_path, fra as &dyn std::any::Any)
        .expect("FRA -> ILP reduction should not fail")
        .expect("FRA -> ILP reduction should execute");
    let ilp = fra_chain.target_problem::<ILP<i64>>();

    let ilp_solution = ILPSolver::new()
        .solve(ilp)
        .expect("satisfiable FRA instance should reduce to a feasible ILP");
    let fra_solution = fra_chain.extract_solution(&ilp_solution).unwrap();
    assert_eq!(fra.evaluate(&fra_solution).unwrap(), Or(true));

    let sat_solution = ksat_chain.extract_solution(&fra_solution).unwrap();
    assert_eq!(source.evaluate(&sat_solution).unwrap(), Or(true));
}

#[test]
fn test_unsatisfiable_ksat_stays_infeasible_through_fra_to_ilp() {
    let source = KSatisfiability::<K3>::new(
        1,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
    );

    let graph = ReductionGraph::new();
    let ksat_chain = graph
        .reduce_along_path(&ksat_to_fra_path(), &source as &dyn std::any::Any)
        .expect("KSAT -> FRA reduction should not fail")
        .expect("KSAT -> FRA reduction should execute");
    let fra = ksat_chain.target_problem::<FeasibleRegisterAssignment>();
    let fra_chain = graph
        .reduce_along_path(&fra_to_ilp_path(), fra as &dyn std::any::Any)
        .expect("FRA -> ILP reduction should not fail")
        .expect("FRA -> ILP reduction should execute");

    assert!(
        ILPSolver::new()
            .solve(fra_chain.target_problem::<ILP<i64>>())
            .is_err(),
        "unsatisfiable source instance should yield an infeasible ILP"
    );
}
