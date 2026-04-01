use super::*;
use crate::models::formula::CNFClause;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_naesatisfiability_to_setsplitting_closed_loop() {
    // YES instance: NAE-SAT with n=4
    let source = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
            CNFClause::new(vec![1, -2, 4]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify overhead
    assert_eq!(target.universe_size(), 8);
    assert_eq!(target.num_subsets(), 8); // 4 vars + 4 clauses

    // Solve and extract
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(target);
    assert!(!witnesses.is_empty());

    for witness in &witnesses {
        let extracted = reduction.extract_solution(witness);
        assert_eq!(extracted.len(), 4);
        let val = source.evaluate(&extracted);
        assert!(val.0, "Extracted solution should be NAE-satisfying");
    }
}

#[test]
fn test_naesatisfiability_to_setsplitting_infeasible() {
    // NO instance: n=3, clauses that force all-equal on variable 1
    let source = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![1, -2, -3]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify overhead
    assert_eq!(target.universe_size(), 6);
    assert_eq!(target.num_subsets(), 7); // 3 vars + 4 clauses

    // Verify subsets match expected
    let subsets = target.subsets();
    assert_eq!(subsets[0], vec![0, 1]); // x1 complementarity
    assert_eq!(subsets[1], vec![2, 3]); // x2 complementarity
    assert_eq!(subsets[2], vec![4, 5]); // x3 complementarity
    assert_eq!(subsets[3], vec![0, 2, 4]); // clause (1,2,3)
    assert_eq!(subsets[4], vec![0, 2, 5]); // clause (1,2,-3)
    assert_eq!(subsets[5], vec![0, 3, 4]); // clause (1,-2,3)
    assert_eq!(subsets[6], vec![0, 3, 5]); // clause (1,-2,-3)

    // Source is infeasible
    let solver = BruteForce::new();
    let source_witnesses = solver.find_all_witnesses(&source);
    assert!(source_witnesses.is_empty(), "Source should be infeasible");

    // Target should also be infeasible
    let target_witnesses = solver.find_all_witnesses(target);
    assert!(target_witnesses.is_empty(), "Target should be infeasible");
}

#[test]
fn test_naesatisfiability_to_setsplitting_reduction_structure() {
    let source = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, -2]),
            CNFClause::new(vec![-1, 2, 3]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 6);
    assert_eq!(target.num_subsets(), 5); // 3 vars + 2 clauses

    let subsets = target.subsets();
    // Complementarity subsets
    assert_eq!(subsets[0], vec![0, 1]);
    assert_eq!(subsets[1], vec![2, 3]);
    assert_eq!(subsets[2], vec![4, 5]);
    // Clause (1, -2) -> elements {0, 3}
    assert_eq!(subsets[3], vec![0, 3]);
    // Clause (-1, 2, 3) -> elements {1, 2, 4}
    assert_eq!(subsets[4], vec![1, 2, 4]);
}

#[test]
fn test_naesatisfiability_to_setsplitting_all_witnesses_map_back() {
    // Small instance: verify every Set Splitting solution maps to a valid NAE-SAT solution
    let source = NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, 3]),
        ],
    );

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let ss_solutions = solver.find_all_witnesses(target);

    assert!(!ss_solutions.is_empty());

    for ss_sol in &ss_solutions {
        let nae_sol = reduction.extract_solution(ss_sol);
        assert_eq!(nae_sol.len(), 3);
        assert!(
            source.evaluate(&nae_sol).0,
            "Extracted solution {:?} from SS solution {:?} does not satisfy NAE-SAT",
            nae_sol,
            ss_sol
        );
    }
}

#[test]
fn test_naesatisfiability_to_setsplitting_known_assignment() {
    // Verify that known assignment [1,0,1,0] for YES instance works
    let source = NAESatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
            CNFClause::new(vec![1, -2, 4]),
        ],
    );

    // Assignment [1,0,1,0] should be NAE-satisfying
    let config = vec![1, 0, 1, 0];
    assert!(source.evaluate(&config).0);

    let reduction = ReduceTo::<SetSplitting>::reduce_to(&source);
    let target = reduction.target_problem();

    // Construct corresponding SS config:
    // var 0 (x1=true): elem 0 in part 0, elem 1 in part 1 -> config[0]=0, config[1]=1
    // var 1 (x2=false): elem 2 in part 1, elem 3 in part 0 -> config[2]=1, config[3]=0
    // var 2 (x3=true): elem 4 in part 0, elem 5 in part 1 -> config[4]=0, config[5]=1
    // var 3 (x4=false): elem 6 in part 1, elem 7 in part 0 -> config[6]=1, config[7]=0
    let ss_config = vec![0, 1, 1, 0, 0, 1, 1, 0];
    assert!(target.evaluate(&ss_config).0, "SS config should be valid");

    // Extract should recover [1,0,1,0]
    let extracted = reduction.extract_solution(&ss_config);
    assert_eq!(extracted, vec![1, 0, 1, 0]);
}
