use super::*;
use crate::models::misc::Term;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::{BruteForce, Problem};

fn example(no: bool) -> ConjunctiveQueryFoldability {
    use Term::{Distinguished as X, Undistinguished as Y};
    ConjunctiveQueryFoldability::new(
        0,
        1,
        3,
        vec![2],
        vec![
            (0, vec![X(0), Y(0)]),
            (0, vec![Y(0), Y(1)]),
            (0, vec![Y(1), X(0)]),
            (0, vec![Y(0), Y(0)]),
        ],
        if no {
            vec![
                (0, vec![X(0), Y(1)]),
                (0, vec![Y(1), Y(2)]),
                (0, vec![Y(2), X(0)]),
            ]
        } else {
            vec![
                (0, vec![X(0), Y(2)]),
                (0, vec![Y(2), Y(2)]),
                (0, vec![Y(2), X(0)]),
            ]
        },
    )
}

fn reduce(source: &ConjunctiveQueryFoldability) -> ReductionConjunctiveQueryFoldabilityToSAT {
    ReduceTo::<Satisfiability>::reduce_to(source).unwrap()
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_closed_loop() {
    use Term::Undistinguished as Y;
    let source = ConjunctiveQueryFoldability::new(
        0,
        0,
        2,
        vec![2],
        vec![(0, vec![Y(0), Y(1)])],
        vec![(0, vec![Y(1), Y(0)])],
    );
    let reduction = reduce(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "query fold to SAT",
    );
    let witness = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(reduction.extract_solution(&witness).unwrap(), vec![1, 0]);
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_worked_examples() {
    let yes = example(false);
    let reduction = reduce(&yes);
    let t = reduction.target_problem();
    assert_eq!(
        (t.num_vars(), t.num_clauses(), t.num_literals()),
        (18, 34, 75)
    );
    let witness = [
        false, true, false, true, false, true, true, false, false, false, true, false, false,
        false, true, false, true, false,
    ]
    .to_vec();
    assert!(t.evaluate(&witness).unwrap().0);
    assert_eq!(reduction.extract_solution(&witness).unwrap(), vec![3, 3, 3]);
    assert!(yes.evaluate(&vec![3, 3, 3]).unwrap().0);
    let no = example(true);
    let r = reduce(&no);
    let t = r.target_problem();
    assert_eq!(
        (t.num_vars(), t.num_clauses(), t.num_literals()),
        (21, 39, 87)
    );
    assert!(BruteForce::new().solve(&no).unwrap().is_none());
    for z in 19..=21 {
        assert!(t.clauses().iter().any(|clause| clause.literals == vec![-z]));
    }
    assert!(
        t.clauses()
            .iter()
            .any(|clause| clause.literals == vec![19, 20, 21])
    );
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_rejects_invalid_witnesses() {
    let reduction = reduce(&example(false));
    assert!(reduction.extract_solution(&vec![false; 17]).is_err());
    assert!(reduction.extract_solution(&vec![false; 19]).is_err());
    assert!(reduction.extract_solution(&vec![false; 18]).is_err());
    assert!(reduction.extract_solution(&vec![true; 18]).is_err());
}

#[cfg(feature = "example-db")]
#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_canonical_export() {
    let spec = canonical_rule_example_specs().remove(0);
    let exported = (spec.build)();
    let source: ConjunctiveQueryFoldability =
        serde_json::from_value(exported.source.instance).unwrap();
    let target: Satisfiability = serde_json::from_value(exported.target.instance).unwrap();
    let pair = &exported.solutions[0];
    let w: Vec<usize> = serde_json::from_value(pair.source_config.clone()).unwrap();
    let b: Vec<bool> = serde_json::from_value(pair.target_config.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(&source).unwrap(),
        serde_json::to_value(example(false)).unwrap()
    );
    assert_eq!(
        (
            target.num_vars(),
            target.num_clauses(),
            target.num_literals()
        ),
        (18, 34, 75)
    );
    assert!(source.evaluate(&w).unwrap().0);
    assert!(target.evaluate(&b).unwrap().0);
    assert_eq!(reduce(&source).extract_solution(&b).unwrap(), w);
}

// Generic CNF enumeration with unit propagation; tiny cases are checked against BruteForce.
fn sat_models(target: &Satisfiability) -> Vec<Vec<bool>> {
    fn visit(target: &Satisfiability, mut values: Vec<Option<bool>>, out: &mut Vec<Vec<bool>>) {
        loop {
            let mut forced = None;
            for clause in target.clauses() {
                let mut unknown = Vec::new();
                let mut satisfied = false;
                for &lit in &clause.literals {
                    let v = usize::try_from(lit.unsigned_abs()).unwrap() - 1;
                    match values[v] {
                        Some(value) => satisfied |= value == (lit > 0),
                        None => unknown.push((v, lit > 0)),
                    }
                }
                if satisfied {
                    continue;
                }
                if unknown.is_empty() {
                    return;
                }
                if unknown.len() == 1 {
                    forced = Some(unknown[0]);
                    break;
                }
            }
            if let Some((v, value)) = forced {
                values[v] = Some(value);
            } else {
                break;
            }
        }
        if let Some(v) = values.iter().position(Option::is_none) {
            for value in [false, true] {
                let mut next = values.clone();
                next[v] = Some(value);
                visit(target, next, out);
            }
        } else {
            let bits: Vec<_> = values.into_iter().map(Option::unwrap).collect();
            assert!(target.evaluate(&bits).unwrap().0);
            out.push(bits);
        }
    }
    let mut out = Vec::new();
    visit(target, vec![None; target.num_vars()], &mut out);
    out.sort();
    out
}

fn source_feasible(source: &ConjunctiveQueryFoldability) -> bool {
    fn search(
        s: &ConjunctiveQueryFoldability,
        used: &[usize],
        w: &mut Vec<usize>,
        range: usize,
    ) -> bool {
        let Some((&y, rest)) = used.split_first() else {
            return s.evaluate(w).unwrap().0;
        };
        (0..range).any(|value| {
            w[y] = value;
            search(s, rest, w, range)
        })
    }
    // Unused coordinates cannot change an atom image; still enumerate the full term domain.
    let used: std::collections::BTreeSet<_> = source
        .query1_conjuncts()
        .iter()
        .flat_map(|(_, args)| args)
        .filter_map(|t| match t {
            Term::Undistinguished(y) => Some(*y),
            _ => None,
        })
        .collect();
    search(
        source,
        &used.into_iter().collect::<Vec<_>>(),
        &mut vec![0; source.num_undistinguished()],
        source.domain_size() + source.num_distinguished() + source.num_undistinguished(),
    )
}

fn audit(source: &ConjunctiveQueryFoldability, raw: bool) {
    let r = reduce(source);
    let target = r.target_problem();
    let models = sat_models(target);
    let direct = source_feasible(source);
    assert_eq!(direct, !models.is_empty(), "{source:?}");
    assert_eq!(
        crate::rules::AggregateReductionResult::extract_value(
            &r,
            crate::types::Or(!models.is_empty())
        )
        .0,
        direct
    );
    for bits in &models {
        let w = r.extract_solution(bits).unwrap();
        assert!(source.evaluate(&w).unwrap().0);
        for y in 0..source.num_undistinguished() {
            assert_eq!(
                bits[y * r.candidates.len()..(y + 1) * r.candidates.len()]
                    .iter()
                    .filter(|&&b| b)
                    .count(),
                1
            );
        }
    }
    if raw {
        assert_eq!(direct, BruteForce::new().solve(source).unwrap().is_some());
    }
    if raw && target.num_vars() <= 10 {
        let mut reference = BruteForce::new().find_all_witnesses(target).unwrap();
        reference.sort();
        assert_eq!(models, reference);
        for mask in 0..(1usize << target.num_vars()) {
            let bits: Vec<_> = (0..target.num_vars())
                .map(|i| mask & (1 << i) != 0)
                .collect();
            assert_eq!(
                r.extract_solution(&bits).is_ok(),
                target.evaluate(&bits).unwrap().0
            );
        }
    }
    crate::rules::test_helpers::assert_parameter_predictions(source, &r);
}

fn from_codes(
    d: usize,
    x: usize,
    u: usize,
    arities: Vec<usize>,
    a: Vec<(usize, Vec<usize>)>,
    b: Vec<(usize, Vec<usize>)>,
) -> ConjunctiveQueryFoldability {
    let term = |v| {
        if v < d {
            Term::Constant(v)
        } else if v < d + x {
            Term::Distinguished(v - d)
        } else {
            Term::Undistinguished(v - d - x)
        }
    };
    let query = |atoms: Vec<(usize, Vec<usize>)>| {
        atoms
            .into_iter()
            .map(|(rel, args)| (rel, args.into_iter().map(term).collect()))
            .collect()
    };
    ConjunctiveQueryFoldability::new(d, x, u, arities, query(a), query(b))
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_exhaustive_unary_lists() {
    let mut lists = vec![vec![]];
    for a in 0..2 {
        lists.push(vec![(0, vec![a])]);
        for b in 0..2 {
            lists.push(vec![(0, vec![a]), (0, vec![b])]);
        }
    }
    for (d, x, u) in [(0, 0, 2), (0, 1, 1), (1, 0, 1), (1, 1, 0)] {
        for a in &lists {
            for b in &lists {
                audit(&from_codes(d, x, u, vec![1], a.clone(), b.clone()), true);
            }
        }
    }
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_empty_universe_and_nullary_sets() {
    let mut lists = vec![vec![]];
    for a in 0..2 {
        lists.push(vec![(a, vec![])]);
        for b in 0..2 {
            lists.push(vec![(a, vec![]), (b, vec![])]);
        }
    }
    for a in &lists {
        for b in &lists {
            audit(&from_codes(0, 0, 0, vec![0, 0], a.clone(), b.clone()), true);
        }
    }
    for u in [0, 3] {
        audit(&from_codes(0, 0, u, vec![], vec![], vec![]), true);
    }
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_repeated_full_arity_arguments() {
    for u in 1..=5 {
        let args: Vec<_> = (0..u).collect();
        let images: Vec<_> = (0..u).rev().collect();
        for copies in [1, 2] {
            audit(
                &from_codes(
                    0,
                    0,
                    u,
                    vec![u * copies],
                    vec![(0, args.repeat(copies))],
                    vec![(0, images.repeat(copies))],
                ),
                true,
            );
        }
        if u > 1 {
            let mut conflict = images.repeat(2);
            conflict[2 * u - 1] = 1;
            let source = from_codes(
                0,
                0,
                u,
                vec![2 * u],
                vec![(0, args.repeat(2))],
                vec![(0, conflict)],
            );
            assert!(BruteForce::new().solve(&source).unwrap().is_none());
            audit(&source, true);
        }
    }
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_large_compressed_domains() {
    for d in [1usize << 32, usize::MAX - 2] {
        let s = from_codes(d, 1, 1, vec![1], vec![(0, vec![d + 1])], vec![(0, vec![d])]);
        let r = reduce(&s);
        assert_eq!(r.candidates, vec![0, d]);
        assert_eq!(
            (
                r.target.num_vars(),
                r.target.num_clauses(),
                r.target.num_literals()
            ),
            (3, 6, 10)
        );
        for b in sat_models(&r.target) {
            assert_eq!(r.extract_solution(&b).unwrap(), vec![d]);
            assert!(s.evaluate(&vec![d]).unwrap().0);
        }
        let entry = crate::rules::registry::reduction_entries()
            .into_iter()
            .find(|e| {
                e.source_name == ConjunctiveQueryFoldability::NAME
                    && e.target_name == Satisfiability::NAME
            })
            .unwrap();
        assert!(matches!(
            entry
                .parameter_contract()
                .unwrap()
                .transform()
                .unwrap()
                .evaluate(&s.parameters()),
            Err(crate::parameters::ParameterTransformError::OutputOutOfRange { .. })
        ));
    }
}

#[test]
fn test_conjunctivequeryfoldability_to_satisfiability_numeric_boundaries() {
    // These arithmetic boundaries cannot be exercised by allocating their target vectors.
    assert_eq!(
        variable_counts(usize::MAX, 1, 0, 0).unwrap(),
        (usize::MAX, usize::MAX)
    );
    assert!(matches!(
        variable_counts(usize::MAX, 2, 0, 0),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    assert_eq!(
        variable_counts(0, 0, usize::MAX, 1).unwrap(),
        (0, usize::MAX)
    );
    assert!(matches!(
        variable_counts(0, 0, usize::MAX, 2),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    assert_eq!(
        variable_counts(usize::MAX - 1, 1, 1, 1).unwrap(),
        (usize::MAX - 1, usize::MAX)
    );
    assert!(matches!(
        variable_counts(usize::MAX, 1, 1, 1),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    let limit = usize::try_from(i64::MAX).unwrap();
    assert_eq!(
        <ConjunctiveQueryFoldability as ReduceTo<Satisfiability>>::exact_i64(
            limit,
            "literal index"
        )
        .unwrap(),
        i64::MAX
    );
    let oversized = ConjunctiveQueryFoldability::new(0, 0, limit + 1, vec![], vec![], vec![]);
    assert!(matches!(
        ReduceTo::<Satisfiability>::reduce_to(&oversized),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    let fixed_boundary =
        ConjunctiveQueryFoldability::new(usize::MAX - 1, 1, 0, vec![], vec![], vec![]);
    assert_eq!(reduce(&fixed_boundary).target.num_vars(), 0);
    for (d, x, u) in [(usize::MAX, 1, 0), (usize::MAX - 1, 1, 1)] {
        let s = ConjunctiveQueryFoldability::new(d, x, u, vec![], vec![], vec![]);
        assert!(matches!(
            ReduceTo::<Satisfiability>::reduce_to(&s),
            Err(ReductionError::IntegerOverflow { .. })
        ));
    }
}

fn sampled_cell(u: usize, m: usize) {
    use rand::{RngExt, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(2026101305 + u64::try_from(u * 6 + m).unwrap());
    for _ in 0..300 {
        let d = rng.random_range(0..=1);
        let x = rng.random_range(0..=1);
        let range = d + x + u;
        let arities = if range == 0 {
            vec![0, 0]
        } else {
            vec![0, 0, 1, 2]
        };
        let atoms: Vec<_> = (0..m)
            .map(|_| {
                let rel = rng.random_range(0..arities.len());
                (
                    rel,
                    (0..arities[rel])
                        .map(|_| rng.random_range(0..range))
                        .collect(),
                )
            })
            .collect();
        let p = rng.random_range(0..=m);
        let source = from_codes(d, x, u, arities, atoms[..p].to_vec(), atoms[p..].to_vec());
        audit(&source, false);
    }
}

macro_rules! sampled_cells {
    ($u:expr; $($name:ident => $m:expr),+ $(,)?) => { $(
        #[test]
        fn $name() { sampled_cell($u,$m); }
    )+ };
}

sampled_cells!(0; sample_fixed_empty => 0, sample_fixed_one_atom => 1,
    sample_fixed_two_atoms => 2, sample_fixed_three_atoms => 3,
    sample_fixed_four_atoms => 4, sample_fixed_five_atoms => 5);
sampled_cells!(1; sample_one_variable_empty => 0, sample_one_variable_one_atom => 1,
    sample_one_variable_two_atoms => 2, sample_one_variable_three_atoms => 3,
    sample_one_variable_four_atoms => 4, sample_one_variable_five_atoms => 5);
sampled_cells!(2; sample_two_variables_empty => 0, sample_two_variables_one_atom => 1,
    sample_two_variables_two_atoms => 2, sample_two_variables_three_atoms => 3,
    sample_two_variables_four_atoms => 4, sample_two_variables_five_atoms => 5);
sampled_cells!(3; sample_three_variables_empty => 0, sample_three_variables_one_atom => 1,
    sample_three_variables_two_atoms => 2, sample_three_variables_three_atoms => 3,
    sample_three_variables_four_atoms => 4, sample_three_variables_five_atoms => 5);
sampled_cells!(4; sample_four_variables_empty => 0, sample_four_variables_one_atom => 1,
    sample_four_variables_two_atoms => 2, sample_four_variables_three_atoms => 3,
    sample_four_variables_four_atoms => 4, sample_four_variables_five_atoms => 5);
sampled_cells!(5; sample_five_variables_empty => 0, sample_five_variables_one_atom => 1,
    sample_five_variables_two_atoms => 2, sample_five_variables_three_atoms => 3,
    sample_five_variables_four_atoms => 4, sample_five_variables_five_atoms => 5);
