use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    let problem = RectilinearPictureCompression::new(vec![vec![true, true], vec![true, false]], 2);
    let reduction: ReductionRPCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    // Number of vars = number of maximal rectangles (precomputed)
    assert!(ilp.num_vars() > 0);
    assert_eq!(ilp.sense(), ObjectiveSense::Minimize);
}

#[test]
fn test_rectilinearpicturecompression_to_ilp_bf_vs_ilp() {
    let problem = RectilinearPictureCompression::new(vec![vec![true, true], vec![true, true]], 1);
    let reduction: ReductionRPCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();

    let bf_witness = bf.solve(&problem).unwrap().expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness).unwrap(), Or(true));

    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_solution_extraction() {
    let problem = RectilinearPictureCompression::new(vec![vec![true, true], vec![true, true]], 2);
    let reduction: ReductionRPCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_rectilinearpicturecompression_to_ilp_trivial() {
    // All-zero matrix: no 1-cells, trivially feasible
    let problem =
        RectilinearPictureCompression::new(vec![vec![false, false], vec![false, false]], 0);
    let reduction: ReductionRPCToILP =
        ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp = reduction.target_problem();
    assert_eq!(ilp.num_vars(), 0); // no maximal rects
}

#[test]
fn test_rectangle_threshold_normalization() {
    for bound in [i64::MIN, -1, 0, 1, i64::MAX] {
        let source = RectilinearPictureCompression::new(vec![vec![true]], bound);
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        assert_eq!(
            reduction.target_problem().max_constraint_magnitude_bits(),
            1
        );
        match ILPSolver::new().solve(reduction.target_problem()) {
            Ok(solution) => {
                assert!(bound >= 1);
                assert_eq!(
                    source
                        .evaluate(&reduction.extract_solution(&solution).unwrap())
                        .unwrap(),
                    Or(true)
                );
            }
            Err(error) => {
                assert!(bound < 1);
                assert_eq!(error, crate::solvers::ILPSolveError::Infeasible);
            }
        }
    }
}
