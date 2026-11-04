use super::*;
use crate::models::formula::CNFClause;
use crate::rules::test_helpers::{
    assert_parameter_predictions, assert_satisfaction_round_trip_from_satisfaction_target,
};
use crate::solvers::{BruteForce, ILPSolveError, ILPSolver};
use crate::traits::Problem;
use std::collections::BTreeSet;

fn reduce(source: &BoyceCoddNormalFormViolation) -> ReductionBCNFViolationToSAT {
    ReduceTo::<Satisfiability>::reduce_to(source).expect("reduction should succeed")
}

fn bcnf(n: usize, fds: &[(&[usize], &[usize])], r: &[usize]) -> BoyceCoddNormalFormViolation {
    BoyceCoddNormalFormViolation::new(
        n,
        fds.iter()
            .map(|(l, rr)| (l.to_vec(), rr.to_vec()))
            .collect(),
        r.to_vec(),
    )
}

fn closure(source: &BoyceCoddNormalFormViolation, selected: &BTreeSet<usize>) -> BTreeSet<usize> {
    let mut closed = selected.clone();
    loop {
        let before = closed.len();
        for (lhs, rhs) in source.functional_deps() {
            if lhs.iter().all(|a| closed.contains(a)) {
                closed.extend(rhs.iter().copied());
            }
        }
        if closed.len() == before {
            return closed;
        }
    }
}

/// Forced assignment of the target for a selection: the layers of the closure
/// iteration, the triggers and the gain bits, in the documented variable order.
fn forced_assignment(source: &BoyceCoddNormalFormViolation, bits: &[bool]) -> Vec<bool> {
    let n = source.num_attributes();
    let target_set = source.target_subset();
    let mut layer: BTreeSet<usize> = target_set
        .iter()
        .zip(bits)
        .filter(|&(_, &b)| b)
        .map(|(&a, _)| a)
        .collect();
    let selected = layer.clone();
    let mut out = bits.to_vec();
    let mut triggers = Vec::new();
    for _ in 0..n {
        out.extend((0..n).map(|a| layer.contains(&a)));
        let mut next = layer.clone();
        for (lhs, rhs) in source.functional_deps() {
            let on = lhs.iter().all(|a| layer.contains(a));
            triggers.push(on);
            if on {
                next.extend(rhs.iter().copied());
            }
        }
        layer = next;
    }
    out.extend((0..n).map(|a| layer.contains(&a)));
    out.extend(triggers);
    out.extend(
        target_set
            .iter()
            .map(|a| !selected.contains(a) && layer.contains(a)),
    );
    out
}

fn selections(r: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << r).map(move |mask| (0..r).map(|i| (mask >> i) & 1 == 1).collect())
}

/// For every selection: the forced assignment satisfies the target exactly when
/// the selection is a source solution, its final layer is the closure, and
/// extraction agrees.
fn assert_selection_equivalence(source: &BoyceCoddNormalFormViolation) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    let n = source.num_attributes();
    let r = source.num_target_attributes();
    for bits in selections(r) {
        let assignment = forced_assignment(source, &bits);
        assert_eq!(assignment.len(), target.num_vars());
        let source_ok = source.evaluate(&bits).unwrap().0;
        assert_eq!(
            target.evaluate(&assignment).unwrap().0,
            source_ok,
            "disagreement on {bits:?}"
        );
        let selected: BTreeSet<usize> = source
            .target_subset()
            .iter()
            .zip(&bits)
            .filter(|&(_, &b)| b)
            .map(|(&a, _)| a)
            .collect();
        let layers_end = r + (n + 1) * n;
        let last: BTreeSet<usize> = (0..n).filter(|&a| assignment[layers_end - n + a]).collect();
        assert_eq!(last, closure(source, &selected));
        match reduction.extract_solution(&assignment) {
            Ok(extracted) => {
                assert!(source_ok);
                assert_eq!(extracted, bits);
            }
            Err(_) => assert!(!source_ok),
        }
    }
}

/// Solve the target with an exact solver (not a re-simulation of the construction).
fn solve_target(target: &Satisfiability) -> Option<Vec<bool>> {
    match ILPSolver::new().solve(target) {
        Err(ILPSolveError::Infeasible) => None,
        result => Some(result.expect("target solve")),
    }
}

/// With the selection pinned, the only model is the forced assignment: pin the
/// selection bits, forbid the forced assignment, and require infeasibility.
fn assert_auxiliaries_unique(source: &BoyceCoddNormalFormViolation, bits: &[bool]) {
    let reduction = reduce(source);
    let target = reduction.target_problem();
    let forced = forced_assignment(source, bits);
    let mut clauses = target.clauses().to_vec();
    clauses.extend((0..bits.len()).map(|i| {
        let v = i64::try_from(i + 1).unwrap();
        CNFClause::new(vec![if bits[i] { v } else { -v }])
    }));
    clauses.push(CNFClause::new(
        forced
            .iter()
            .enumerate()
            .map(|(i, &b)| {
                let v = i64::try_from(i + 1).unwrap();
                if b { -v } else { v }
            })
            .collect(),
    ));
    let pinned = Satisfiability::new(target.num_vars(), clauses);
    assert!(
        solve_target(&pinned).is_none(),
        "auxiliaries not determined for {bits:?}"
    );
}

fn yes_source() -> BoyceCoddNormalFormViolation {
    bcnf(4, &[(&[0], &[1]), (&[1], &[2])], &[0, 1, 3])
}

fn no_source() -> BoyceCoddNormalFormViolation {
    bcnf(3, &[(&[0], &[1]), (&[1], &[2]), (&[2], &[0])], &[0, 1, 2])
}

fn yes_target_bits() -> Vec<bool> {
    // selection | layers 1000 1100 1110 1110 1110 | triggers 10 11 11 11 | gains
    ["100", "10001100111011101110", "10111111", "010"]
        .concat()
        .chars()
        .map(|ch| ch == '1')
        .collect()
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_closed_loop() {
    // N = 3, one dependency, R = A: 21 target variables, enumerable by brute force.
    let source = bcnf(3, &[(&[0], &[1])], &[0, 1, 2]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vars(), 21);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "BCNF violation -> SAT closed loop",
    );
    // One target model per source solution.
    let models = BruteForce::new()
        .find_all_witnesses(reduction.target_problem())
        .unwrap();
    let solutions = BruteForce::new().find_all_witnesses(&source).unwrap();
    assert_eq!(models.len(), solutions.len());
    assert!(!solutions.is_empty());
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_yes_example() {
    let source = yes_source();
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(
        (
            target.num_vars(),
            target.num_clauses(),
            target.num_literals()
        ),
        (34, 74, 160)
    );
    let bits = yes_target_bits();
    assert_eq!(bits.len(), 34);
    assert!(target.evaluate(&bits).unwrap().0);
    assert_eq!(forced_assignment(&source, &[true, false, false]), bits);
    assert_eq!(
        reduction.extract_solution(&bits).unwrap(),
        vec![true, false, false]
    );
    assert_selection_equivalence(&source);
    for bits in selections(3) {
        assert_auxiliaries_unique(&source, &bits);
    }
    let model = solve_target(target).expect("YES target is satisfiable");
    let extracted = reduction.extract_solution(&model).unwrap();
    assert!(source.evaluate(&extracted).unwrap().0);
    let solutions: Vec<Vec<bool>> = selections(3)
        .filter(|b| source.evaluate(b).unwrap().0)
        .collect();
    assert!(solutions.contains(&vec![true, false, false]));
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_no_example_infeasible() {
    let source = no_source();
    let reduction = reduce(&source);
    let target = reduction.target_problem();
    assert_eq!(
        (
            target.num_vars(),
            target.num_clauses(),
            target.num_literals()
        ),
        (27, 62, 138)
    );
    assert!(BruteForce::new().solve(&source).unwrap().is_none());
    // The exact solver finds no model of the 27-variable target.
    assert!(solve_target(target).is_none());
    assert_selection_equivalence(&source);
    // Each selection determines all auxiliaries, and its forced assignment violates a final clause.
    for bits in selections(3) {
        assert_auxiliaries_unique(&source, &bits);
        assert!(
            !target
                .evaluate(&forced_assignment(&source, &bits))
                .unwrap()
                .0
        );
    }
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_variable_layout_and_clauses() {
    let source = yes_source();
    let target = reduce(&source).target_problem().clone();
    let clauses: Vec<Vec<i64>> = target
        .clauses()
        .iter()
        .map(|c| c.literals.clone())
        .collect();
    // selection bits 1..=3; c[0][0] = 4, c[0][3] = 7; c[1][0] = 8; triggers start at 24; gains at 32
    assert_eq!(&clauses[0], &vec![-4, 1]);
    assert_eq!(&clauses[1], &vec![4, -1]);
    assert_eq!(&clauses[2], &vec![-5, 2]);
    assert_eq!(&clauses[4], &vec![-6]);
    // trigger g[0][0] = 24 for FD {0} -> {1}
    assert!(clauses.contains(&vec![-24, 4]));
    assert!(clauses.contains(&vec![24, -4]));
    // c[1][1] (= 9) is fed by its predecessor c[0][1] (= 5) and by g[0][0] (= 24)
    assert!(clauses.contains(&vec![-5, 9]));
    assert!(clauses.contains(&vec![-24, 9]));
    assert!(clauses.contains(&vec![-9, 5, 24]));
    // gain gate for the first target attribute and the final clauses
    assert!(clauses.contains(&vec![-32, -1]));
    assert!(clauses.contains(&vec![-32, 20]));
    assert!(clauses.contains(&vec![32, 1, -20]));
    assert_eq!(clauses[clauses.len() - 2], vec![32, 33, 34]);
    assert_eq!(clauses[clauses.len() - 1], vec![-20, -21, -23]);
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_parameter_formulas() {
    let sources = vec![
        yes_source(),
        no_source(),
        bcnf(3, &[], &[0, 1, 2]),
        bcnf(1, &[(&[0], &[0])], &[0]),
        bcnf(
            4,
            &[(&[0, 1], &[1, 2]), (&[0, 1], &[1, 2]), (&[2], &[])],
            &[0, 1, 2, 3],
        ),
        bcnf(5, &[(&[0], &[1, 2, 3, 4]), (&[1, 2, 3, 4], &[0])], &[1, 4]),
    ];
    for source in sources {
        let reduction = reduce(&source);
        assert_parameter_predictions(&source, &reduction);
        let (n, d, r) = (
            source.num_attributes(),
            source.num_functional_deps(),
            source.num_target_attributes(),
        );
        let l: usize = source.functional_deps().iter().map(|(a, _)| a.len()).sum();
        let b: usize = source.functional_deps().iter().map(|(_, a)| a.len()).sum();
        let target = reduction.target_problem();
        assert_eq!(target.num_vars(), n * (n + 1) + n * d + 2 * r);
        assert_eq!(
            target.num_clauses(),
            n + 4 * r + n * (l + b + d + 2 * n) + 2
        );
        assert_eq!(
            target.num_literals(),
            n + 12 * r + n * (3 * l + 3 * b + d + 4 * n)
        );
    }
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_special_cases() {
    // empty dependency list: no gain is ever possible
    let empty = bcnf(3, &[], &[0, 1, 2]);
    assert_selection_equivalence(&empty);
    assert!(selections(3).all(|b| !empty.evaluate(&b).unwrap().0));
    // empty right sides only conclude nothing
    let empty_rhs = bcnf(3, &[(&[0], &[]), (&[1], &[])], &[0, 1, 2]);
    assert_selection_equivalence(&empty_rhs);
    assert!(selections(3).all(|b| !empty_rhs.evaluate(&b).unwrap().0));
    // derivation 0 -> 2 -> 1 passes through attribute 2 outside R = {0, 1, 3}
    let through = bcnf(4, &[(&[0], &[2]), (&[2], &[1])], &[0, 1, 3]);
    assert_selection_equivalence(&through);
    let reduction = reduce(&through);
    let witness = forced_assignment(&through, &[true, false, false]);
    assert!(reduction.target_problem().evaluate(&witness).unwrap().0);
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![true, false, false]
    );
    // X = empty is never a solution: every left side is nonempty
    let reduction = reduce(&yes_source());
    let zero = forced_assignment(&yes_source(), &[false; 3]);
    assert!(!reduction.target_problem().evaluate(&zero).unwrap().0);
    assert!(reduction.extract_solution(&zero).is_err());
    // single target attribute, duplicates, left side contained in right side
    assert_selection_equivalence(&bcnf(3, &[(&[0], &[1])], &[0]));
    assert_selection_equivalence(&bcnf(3, &[(&[0], &[1]), (&[0, 0], &[1, 1])], &[0, 1, 2]));
    assert_selection_equivalence(&bcnf(4, &[(&[0, 1], &[1, 2])], &[0, 1, 2, 3]));
    assert_selection_equivalence(&bcnf(1, &[(&[0], &[0])], &[0]));
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_exhaustive_small_instances() {
    // Every instance with N <= 3 and at most two dependencies, over every target set.
    for n in 1..=3usize {
        let sides: Vec<Vec<usize>> = (0..1usize << n)
            .map(|m| (0..n).filter(|a| (m >> a) & 1 == 1).collect())
            .collect();
        let mut deps: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
        for lhs in sides.iter().filter(|s| !s.is_empty()) {
            for rhs in &sides {
                deps.push((lhs.clone(), rhs.clone()));
            }
        }
        let mut lists: Vec<Vec<(Vec<usize>, Vec<usize>)>> = vec![vec![]];
        lists.extend(deps.iter().map(|f| vec![f.clone()]));
        if n <= 2 {
            for f in &deps {
                for g in &deps {
                    lists.push(vec![f.clone(), g.clone()]);
                }
            }
        }
        for list in lists {
            for target_set in sides.iter().filter(|s| !s.is_empty()) {
                let source = BoyceCoddNormalFormViolation::new(n, list.clone(), target_set.clone());
                assert_selection_equivalence(&source);
            }
        }
    }
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_long_chain_needs_all_rounds() {
    // 0 -> 1 -> 2 -> 3 -> 4: gaining attribute 4 from {0} takes four rounds.
    let source = bcnf(
        5,
        &[(&[0], &[1]), (&[1], &[2]), (&[2], &[3]), (&[3], &[4])],
        &[0, 4, 3],
    );
    assert_selection_equivalence(&source);
    assert!(!source.evaluate(&vec![true, false, false]).unwrap().0);
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_extraction_rejects_invalid_witnesses() {
    let source = yes_source();
    let reduction = reduce(&source);
    let good = yes_target_bits();
    assert_eq!(
        reduction.extract_solution(&good).unwrap(),
        vec![true, false, false]
    );
    // wrong lengths
    assert!(reduction.extract_solution(&good[..33].to_vec()).is_err());
    let mut long = good.clone();
    long.push(false);
    assert!(reduction.extract_solution(&long).is_err());
    assert!(reduction.extract_solution(&vec![]).is_err());
    // every single-bit flip violates the encoding (all bits are functionally determined)
    for i in 0..good.len() {
        let mut bad = good.clone();
        bad[i] = !bad[i];
        assert!(
            reduction.extract_solution(&bad).is_err(),
            "flip of bit {i} accepted"
        );
    }
    // a consistent assignment of a non-solution selection
    let non_solution = forced_assignment(&source, &[true, true, false]);
    assert!(!source.evaluate(&vec![true, true, false]).unwrap().0);
    assert!(reduction.extract_solution(&non_solution).is_err());
    // all-false is rejected
    assert!(reduction.extract_solution(&vec![false; 34]).is_err());
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_aggregate_value_matches_both_solvers() {
    use crate::rules::AggregateReductionResult;
    use crate::types::Or;
    for source in [yes_source(), no_source()] {
        let reduction = reduce(&source);
        let target = ReductionResult::target_problem(&reduction);
        let source_value = Or(BruteForce::new().solve(&source).unwrap().is_some());
        let target_value = Or(solve_target(target).is_some());
        assert_eq!(target_value, source_value);
        assert_eq!(reduction.extract_value(target_value), source_value);
    }
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_size_boundaries() {
    // variable numbers must fit i64: N(N+1) + N*D + 2r <= i64::MAX
    let limit = i64::MAX as u128;
    let fits = |n: u128, r: u128| n * (n + 1) + 2 * r <= limit;
    let mut n = 3_037_000_499u128;
    while !fits(n, 1) {
        n -= 1;
    }
    while fits(n + 1, 1) {
        n += 1;
    }
    let n = usize::try_from(n).unwrap();
    assert_eq!(
        variable_count(n, 0, 1).unwrap() as u128,
        (n as u128) * (n as u128 + 1) + 2
    );
    assert!(variable_count(n + 1, 0, 1).is_err());
    // usize overflow in the products is reported, not wrapped
    assert!(variable_count(usize::MAX, 0, 1).is_err());
    assert!(variable_count(2, usize::MAX, 1).is_err());
    assert!(variable_count(1, 0, usize::MAX).is_err());
    // the dependency term is part of the bound
    assert!(variable_count(3_037_000_000, 5_000_000, 1).is_err());
    assert_eq!(variable_count(3, 2, 3).unwrap(), 12 + 6 + 6);
    // clause counts are checked as well
    assert_eq!(clause_count(4, 2, 3, 2, 2), Some(74));
    assert_eq!(clause_count(usize::MAX, 0, 1, 0, 0), None);
    assert_eq!(clause_count(2, 1, usize::MAX / 4 + 1, 1, 1), None);
    assert_eq!(clause_count(1, 0, 1, usize::MAX, 1), None);
    assert_eq!(clause_count(1, usize::MAX, 1, 0, 0), None);
    // largest representable clause count and its successor
    let r = (usize::MAX - 2) / 4;
    assert_eq!(clause_count(0, 0, r, 0, 0), Some(4 * r + 2));
    assert_eq!(clause_count(0, 0, r + 1, 0, 0), None);
    // N = 2: 2l + 12 clauses for l left-side attributes
    let l = (usize::MAX - 12) / 2;
    assert_eq!(clause_count(2, 0, 0, l, 0), Some(2 * l + 12));
    assert_eq!(clause_count(2, 0, 0, l + 1, 0), None);
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_huge_universe_is_an_error() {
    // An implicit universe of 2^32 attributes needs more than 2^63 variables.
    let source =
        BoyceCoddNormalFormViolation::new(1usize << 32, vec![(vec![0], vec![1])], vec![0, 1]);
    let error = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap_err();
    assert!(
        matches!(error, ReductionError::Construction { .. }),
        "{error}"
    );
}

#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_unallocatable_universe_is_an_error() {
    // Variable numbers still fit i64 but the clause list cannot be allocated.
    let source = BoyceCoddNormalFormViolation::new(3_000_000_000, vec![], vec![0, 1]);
    let error = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap_err();
    assert!(
        matches!(error, ReductionError::Construction { .. }),
        "{error}"
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_boycecoddnormalformviolation_to_satisfiability_canonical_example() {
    let specs = canonical_rule_example_specs();
    assert_eq!(specs.len(), 1);
    let example = (specs[0].build)();
    assert_eq!(example.source.problem, "BoyceCoddNormalFormViolation");
    assert_eq!(example.target.problem, "Satisfiability");
    let target: Satisfiability = serde_json::from_value(example.target.instance.clone()).unwrap();
    assert_eq!(
        (
            target.num_vars(),
            target.num_clauses(),
            target.num_literals()
        ),
        (34, 74, 160)
    );
    assert_eq!(example.solutions.len(), 1);
    let pair = &example.solutions[0];
    let target_config: Vec<bool> = serde_json::from_value(pair.target_config.clone()).unwrap();
    let source_config: Vec<bool> = serde_json::from_value(pair.source_config.clone()).unwrap();
    assert!(target.evaluate(&target_config).unwrap().0);
    assert_eq!(target_config, yes_target_bits());
    let reduction = reduce(&yes_source());
    assert_eq!(
        reduction.extract_solution(&target_config).unwrap(),
        source_config
    );
    assert!(yes_source().evaluate(&source_config).unwrap().0);
}
