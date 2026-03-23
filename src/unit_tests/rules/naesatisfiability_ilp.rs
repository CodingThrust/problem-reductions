use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // NAE-SAT: (x1 ∨ x2) — two variables, one clause
    use crate::models::formula::CNFClause;
    let problem = NAESatisfiability::new(2, vec![CNFClause::new(vec![1, 2])]);
    let reduction: ReductionNAESATToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars, 2, "one ILP var per Boolean variable");
    assert_eq!(
        ilp.constraints.len(),
        2,
        "two constraints per clause (ge + le)"
    );
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty(), "feasibility: no objective terms");
}

#[test]
fn test_naesatisfiability_to_ilp_bf_vs_ilp() {
    // NAE-SAT: (x1 ∨ x2 ∨ x3) ∧ (¬x1 ∨ ¬x2 ∨ x3)
    // Solution x1=T, x2=F, x3=F: clause1 T,F,F (NAE ✓); clause2 F,T,F (NAE ✓)
    use crate::models::formula::CNFClause;
    let problem = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),   // x1 ∨ x2 ∨ x3
            CNFClause::new(vec![-1, -2, 3]), // ¬x1 ∨ ¬x2 ∨ x3
        ],
    );
    let reduction: ReductionNAESATToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf
        .find_witness(&problem)
        .expect("NAE-SAT instance should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_naesatisfiability_to_ilp_infeasible() {
    // Single clause with one literal: (x1) — cannot be NAE (only one literal, never both T and F)
    // NAESatisfiability requires ≥2 literals per clause, so use: (x1 ∨ x1)
    // Actually we need a proper infeasible NAE instance.
    // Simplest: (x1 ∨ x1) — both literals same variable: NAE requires T and F but both are x1.
    // Wait, we can use a 1-variable, 1-clause with both polarities excluded:
    // Use (x1 ∨ ¬x1): if x1=T → T,F ✓ NAE; if x1=F → F,T ✓ NAE. This is always satisfiable.
    // True infeasible: ((x1 ∨ x2) ∧ (x1 ∨ ¬x2) ∧ (¬x1 ∨ x2) ∧ (¬x1 ∨ ¬x2))
    // x1=T,x2=T: clause4 (F,F) — not NAE. x1=T,x2=F: clause3 (F,F) — not NAE.
    // x1=F,x2=T: clause2 (F,T) — NAE ✓; clause4: (T,F) — NAE ✓;
    //   clause1 (F,T) — NAE ✓; clause3 (T,T) — NOT NAE. So not infeasible.
    // Use all-same-sign clause: (x1 ∨ x2) ∧ (¬x1 ∨ ¬x2) ∧ (x1 ∨ ¬x1) makes it tricky.
    // Simplest provably infeasible NAE instance: 1 variable, clause (x1 ∨ x1)
    // but NAESatisfiability requires ≥2 literals. Let's verify with ILP that it's infeasible.
    // Known infeasible NAE: only one variable, clause (x1 ∨ x1) — requires x1=T (for one true)
    // but then both are T so no false literal. Requires x1=F too — contradiction.
    // But NAESat requires ≥2 literals per clause, so use (x1, x1):
    use crate::models::formula::CNFClause;
    let problem = NAESatisfiability::new(1, vec![CNFClause::new(vec![1, 1])]);
    let reduction: ReductionNAESATToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    // The ILP should be infeasible: x1 ≥ 1 (at least one true) AND x1 ≤ 0 (at least one false)
    assert!(
        ilp_solver.solve(ilp).is_none(),
        "ILP should be infeasible for unsatisfiable NAE-SAT"
    );
}

#[test]
fn test_naesatisfiability_to_ilp_negative_literals() {
    // NAE-SAT with mixed literals: (¬x1 ∨ x2) — negative literal encoding
    // Solution: x1=true, x2=false → ¬x1=F, x2=F — not NAE.
    // Solution: x1=false, x2=true → ¬x1=T, x2=T — not NAE.
    // Solution: x1=true, x2=true → ¬x1=F, x2=T — NAE ✓
    // Solution: x1=false, x2=false → ¬x1=T, x2=F — NAE ✓
    use crate::models::formula::CNFClause;
    let problem = NAESatisfiability::new(2, vec![CNFClause::new(vec![-1, 2])]);
    let reduction: ReductionNAESATToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 2);
    assert_eq!(ilp.constraints.len(), 2);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(ilp)
        .expect("NAE-SAT with (¬x1 ∨ x2) is feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(
        problem.evaluate(&extracted),
        Or(true),
        "extracted solution should satisfy NAE condition"
    );
}
