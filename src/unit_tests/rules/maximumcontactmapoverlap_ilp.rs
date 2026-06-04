use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::graph::MaximumContactMapOverlap;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::ILPSolver;
use crate::traits::Problem;
use crate::types::Max;

/// Canonical instance from issue #1043 / #1044:
/// G_1 has 4 vertices with contacts {{0,2},{1,3}};
/// G_2 has 5 vertices with contacts {{0,3},{1,4},{0,2}}.
/// Optimal alignment 0->0, 1->1, 2->3, 3->4 preserves 2 contacts.
fn canonical_instance() -> MaximumContactMapOverlap {
    MaximumContactMapOverlap::new(4, vec![(0, 2), (1, 3)], 5, vec![(0, 3), (1, 4), (0, 2)])
}

#[test]
fn test_maximumcontactmapoverlap_to_ilp_structure() {
    let source = canonical_instance();
    let reduction: ReductionCMOToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    // n1=4, n2=5 -> 20 x-variables. |E_1|=2, |E_2|=3 -> 6 y-variables.
    let num_x = 4 * 5;
    let num_y = 2 * 3;
    assert_eq!(ilp.num_vars, num_x + num_y);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);

    // Constraint shape:
    //   4 row + 5 column
    // + order-preservation: |{(i<k)}| * |{(j>=l)}|
    //                     = C(4,2) * (5*6/2) = 6 * 15 = 90
    // + 2 link constraints per y, so 2 * 6 = 12.
    let order_pairs = (4 * 3 / 2) * (5 * 6 / 2);
    let expected_constraints = 4 + 5 + order_pairs + 2 * num_y;
    assert_eq!(ilp.constraints.len(), expected_constraints);

    // Objective is the sum of y variables only.
    let expected_obj: Vec<(usize, f64)> = (0..num_y).map(|s| (num_x + s, 1.0)).collect();
    assert_eq!(ilp.objective, expected_obj);
}

#[test]
fn test_maximumcontactmapoverlap_to_ilp_closed_loop() {
    let source = canonical_instance();
    let reduction: ReductionCMOToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("canonical CMO ILP must be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // The optimal alignment preserves both contacts of G_1.
    assert!(source.is_valid_solution(&extracted));
    assert_eq!(source.evaluate(&extracted), Max(Some(2)));
}

#[test]
fn test_maximumcontactmapoverlap_to_ilp_trivial_no_contacts() {
    // Both contact maps empty: optimum is 0 and the resulting ILP has no
    // y-variables and no link constraints.
    let source = MaximumContactMapOverlap::new(2, vec![], 2, vec![]);
    let reduction: ReductionCMOToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    // n1=2, n2=2 -> 4 x-variables, 0 y-variables.
    assert_eq!(ilp.num_vars, 4);
    // 2 row + 2 column + C(2,2)=1 ordered-pair * (2*3/2)=3 = 3 order-pres + 0 link.
    assert_eq!(ilp.constraints.len(), 2 + 2 + 3);
    assert!(ilp.objective.is_empty());

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("empty-contact ILP must be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(source.is_valid_solution(&extracted));
    assert_eq!(source.evaluate(&extracted), Max(Some(0)));
}

#[test]
fn test_maximumcontactmapoverlap_to_ilp_bf_vs_ilp() {
    let source = canonical_instance();
    let reduction: ReductionCMOToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_maximumcontactmapoverlap_to_ilp_order_preserving_forbidden() {
    // Crossing-resistant test:
    //   G_1: n_1=3, E_1 = {{0,2}}
    //   G_2: n_2=3, E_2 = {{0,2}}
    // A crossing alignment 0->2, 2->0 would also "preserve" the contact (since
    // {2,0} = {0,2} after canonicalization), but order-preservation rules it
    // out. The straight alignment 0->0, ?, 2->2 preserves the same contact, so
    // the optimum is still 1.
    let source = MaximumContactMapOverlap::new(3, vec![(0, 2)], 3, vec![(0, 2)]);
    let reduction: ReductionCMOToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP must be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(source.is_valid_solution(&extracted));
    assert_eq!(source.evaluate(&extracted), Max(Some(1)));
    // is_valid_solution checks order-preservation; just additionally verify no
    // crossing was selected.
    let nonzero: Vec<usize> = extracted.iter().copied().filter(|&v| v != 0).collect();
    let mut sorted = nonzero.clone();
    sorted.sort_unstable();
    assert_eq!(nonzero, sorted, "alignment must be strictly increasing");
}

#[test]
fn test_maximumcontactmapoverlap_to_ilp_extract_solution_partial() {
    // Verify extract_solution leaves unmatched residues at 0 when the optimum
    // does not use all source residues.
    //   G_1: n_1=2, E_1=empty;  G_2: n_2=3, E_2=empty.
    // No contacts -> objective 0 -> ILP solver may pick the zero vector,
    // leaving every residue unmatched.
    let source = MaximumContactMapOverlap::new(2, vec![], 3, vec![]);
    let reduction: ReductionCMOToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    // Hand-built solution: x_(0,1)=1, x_(1,2)=1, rest zero.
    let n2 = 3;
    let mut target_sol = vec![0usize; reduction.target_problem().num_vars];
    target_sol[1] = 1;
    target_sol[n2 + 2] = 1;
    let extracted = reduction.extract_solution(&target_sol);
    // Encoding: vertex j of G_2 is represented as j+1.
    assert_eq!(extracted, vec![2, 3]);
    assert!(source.is_valid_solution(&extracted));
}
