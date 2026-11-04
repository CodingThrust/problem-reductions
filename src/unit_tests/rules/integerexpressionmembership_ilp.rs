use super::*;
use crate::models::misc::IntExpr;
use crate::rules::AggregateReductionResult;
use crate::rules::test_helpers::{assert_bf_vs_ilp, assert_parameter_predictions};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::{Extremum, Or};

fn union(left: IntExpr, right: IntExpr) -> IntExpr {
    IntExpr::Union(Box::new(left), Box::new(right))
}
fn sum(left: IntExpr, right: IntExpr) -> IntExpr {
    IntExpr::Sum(Box::new(left), Box::new(right))
}
fn example() -> IntExpr {
    sum(
        sum(
            union(IntExpr::Atom(1), IntExpr::Atom(4)),
            union(IntExpr::Atom(3), IntExpr::Atom(6)),
        ),
        union(IntExpr::Atom(2), IntExpr::Atom(5)),
    )
}
fn reduce(
    expression: IntExpr,
    k: i64,
) -> (
    IntegerExpressionMembership,
    ReductionIntegerExpressionMembershipToILP,
) {
    let source = IntegerExpressionMembership::new(expression, k);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    (source, reduction)
}

#[test]
fn test_integerexpressionmembership_to_ilp_closed_loop() {
    let (source, reduction) = reduce(union(IntExpr::Atom(1), IntExpr::Atom(2)), 2);
    assert_bf_vs_ilp(&source, &reduction);
    let target = ReductionResult::target_problem(&reduction);
    let solutions: Vec<_> = (0..1usize << target.num_vars())
        .map(|mask| {
            (0..target.num_vars())
                .map(|b| i64::from((mask >> b) & 1 == 1))
                .collect::<Vec<_>>()
        })
        .filter(|witness| target.is_feasible(witness).unwrap())
        .collect();
    assert_eq!(solutions.len(), 1);
    for solution in solutions {
        assert_eq!(reduction.extract_solution(&solution).unwrap(), vec![true]);
        assert_eq!(
            reduction.extract_value(target.evaluate(&solution).unwrap()),
            Or(true)
        );
    }
}

#[test]
fn test_canonical_witness_and_no_instance() {
    let (source, reduction) = reduce(example(), 12);
    let target = ReductionResult::target_problem(&reduction);
    let mut witness = vec![1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0];
    witness.extend([
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1,
    ]);
    witness.extend([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0]);
    assert_eq!(
        (
            target.num_vars(),
            target.num_constraints(),
            target.num_nonzeros()
        ),
        (53, 36, 111)
    );
    assert_eq!(
        target.evaluate(&witness).unwrap(),
        Extremum::minimize(Some(0))
    );
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![true, true, false]
    );
    assert_eq!(source.evaluate(&vec![true, true, false]).unwrap(), Or(true));
    assert_parameter_predictions(&source, &reduction);
    assert_bf_vs_ilp(&source, &reduction);
    let (no, reduction) = reduce(example(), 7);
    let target = ReductionResult::target_problem(&reduction);
    assert_eq!(
        (
            target.num_vars(),
            target.num_constraints(),
            target.num_nonzeros()
        ),
        (41, 29, 87)
    );
    assert!(BruteForce::new().solve(&no).unwrap().is_none());
    assert!(ILPSolver::new().solve(target).is_err());
    assert_eq!(reduction.extract_value(Extremum::minimize(None)), Or(false));
}

#[test]
fn test_visited_union_order_and_unused_tail() {
    let (source, reduction) = reduce(
        sum(
            union(union(IntExpr::Atom(1), IntExpr::Atom(2)), IntExpr::Atom(3)),
            union(IntExpr::Atom(4), IntExpr::Atom(5)),
        ),
        8,
    );
    let witness = ILPSolver::new()
        .solve(ReductionResult::target_problem(&reduction))
        .unwrap();
    let config = reduction.extract_solution(&witness).unwrap();
    assert_eq!(config, vec![true, true, false]);
    assert_eq!(source.evaluate(&config).unwrap(), Or(true));
    assert_eq!(source.evaluate(&vec![true, true, true]).unwrap(), Or(true));
}

#[test]
fn test_rejects_malformed_and_infeasible_witnesses() {
    let (_, reduction) = reduce(example(), 12);
    let target = ReductionResult::target_problem(&reduction);
    let good = ILPSolver::new().solve(target).unwrap();
    assert!(
        reduction
            .extract_solution(&good[..good.len() - 1].to_vec())
            .is_err()
    );
    let mut extra = good.clone();
    extra.push(0);
    assert!(reduction.extract_solution(&extra).is_err());
    let mut nonbinary = good.clone();
    nonbinary[0] = 2;
    assert!(reduction.extract_solution(&nonbinary).is_err());
    for index in 0..good.len() {
        let mut bad = good.clone();
        bad[index] ^= 1;
        assert!(!target.is_feasible(&bad).unwrap());
        assert!(reduction.extract_solution(&bad).is_err());
    }
}

fn lift(source: &IntegerExpressionMembership, config: &[bool]) -> Vec<i64> {
    fn visit(
        e: &IntExpr,
        active: bool,
        config: &mut std::slice::Iter<'_, bool>,
        activations: &mut Vec<i64>,
        leaves: &mut Vec<(i64, bool)>,
    ) {
        activations.push(i64::from(active));
        match e {
            IntExpr::Atom(value) => leaves.push((*value, active)),
            IntExpr::Sum(left, right) => {
                visit(left, active, config, activations, leaves);
                visit(right, active, config, activations, leaves);
            }
            IntExpr::Union(left, right) => {
                let choice = active && *config.next().unwrap();
                visit(left, active && !choice, config, activations, leaves);
                visit(right, active && choice, config, activations, leaves);
            }
        }
    }
    let mut witness = Vec::new();
    let mut leaves = Vec::new();
    visit(
        source.expression(),
        true,
        &mut config.iter(),
        &mut witness,
        &mut leaves,
    );
    let largest = leaves
        .iter()
        .map(|&(v, _)| v)
        .chain([source.target()])
        .max()
        .unwrap();
    let width = usize::try_from(i64::BITS - largest.leading_zeros()).unwrap();
    let mut previous = 0u128;
    let mut carries = Vec::new();
    for (value, active) in leaves {
        let amount = if active {
            u128::try_from(value).unwrap()
        } else {
            0
        };
        let total = previous + amount;
        witness.extend((0..width).map(|b| i64::from((total >> b) & 1 == 1)));
        for b in 1..width {
            let mask = (1u128 << b) - 1;
            carries.push(i64::try_from(((previous & mask) + (amount & mask)) >> b).unwrap());
        }
        previous = total & ((1u128 << width) - 1);
    }
    witness.extend(carries);
    witness
}

fn audit(expression: IntExpr, k: i64, solve: bool) {
    use std::collections::HashSet;
    let (source, reduction) = reduce(expression, k);
    let target = ReductionResult::target_problem(&reduction);
    let mut expected = HashSet::new();
    for mask in 0..1usize << source.num_union_nodes() {
        let config: Vec<_> = (0..source.num_union_nodes())
            .map(|b| (mask >> b) & 1 == 1)
            .collect();
        let good = source.evaluate(&config).unwrap().0;
        let witness = lift(&source, &config);
        assert_eq!(target.is_feasible(&witness).unwrap(), good);
        if good {
            assert_eq!(
                source
                    .evaluate(&reduction.extract_solution(&witness).unwrap())
                    .unwrap(),
                Or(true)
            );
            expected.insert(witness);
        }
    }
    if target.num_vars() <= 12 {
        let mut actual = HashSet::new();
        for mask in 0..1usize << target.num_vars() {
            let witness: Vec<i64> = (0..target.num_vars())
                .map(|b| i64::from((mask >> b) & 1 == 1))
                .collect();
            if target.is_feasible(&witness).unwrap() {
                let config = reduction.extract_solution(&witness).unwrap();
                assert_eq!(source.evaluate(&config).unwrap(), Or(true));
                actual.insert(witness);
            } else {
                assert!(reduction.extract_solution(&witness).is_err());
            }
        }
        assert_eq!(actual, expected);
    }
    let atoms = source.num_atoms();
    let nodes = source.expression_size();
    let unions = source.num_union_nodes();
    let sums = atoms - 1 - unions;
    let mut flat = Vec::new();
    flatten(source.expression(), &mut flat);
    let values: Vec<_> = flat
        .iter()
        .filter_map(|n| {
            if let Node::Atom(v) = n {
                Some(*v)
            } else {
                None
            }
        })
        .collect();
    let width = bit_width(*values.iter().chain([&k]).max().unwrap()).unwrap();
    let popcount: usize = values
        .iter()
        .map(|v| usize::try_from(v.count_ones()).unwrap())
        .sum();
    assert_eq!(target.num_vars(), nodes + atoms * (2 * width - 1));
    assert_eq!(
        target.num_constraints(),
        1 + 2 * sums + unions + atoms * width + width
    );
    assert_eq!(
        target.num_nonzeros(),
        atoms * (4 * width - 2) + popcount + 1 + 4 * sums + 3 * unions
    );
    assert_eq!(
        target.max_constraint_magnitude_bits(),
        if width == 1 { 1 } else { 2 }
    );
    assert_parameter_predictions(&source, &reduction);
    if solve {
        match ILPSolver::new().solve(target) {
            Ok(witness) => {
                assert!(!expected.is_empty());
                assert!(expected.contains(&witness));
                assert_eq!(
                    source
                        .evaluate(&reduction.extract_solution(&witness).unwrap())
                        .unwrap(),
                    Or(true)
                );
                assert_eq!(
                    reduction.extract_value(target.evaluate(&witness).unwrap()),
                    Or(true)
                );
            }
            Err(_) => {
                assert!(expected.is_empty());
                assert_eq!(reduction.extract_value(Extremum::minimize(None)), Or(false));
            }
        }
    }
}

fn trees(nodes: usize) -> Vec<IntExpr> {
    if nodes == 1 {
        return (1..=3).map(IntExpr::Atom).collect();
    }
    let mut result = Vec::new();
    for left_size in (1..nodes - 1).step_by(2) {
        for left in trees(left_size) {
            for right in trees(nodes - 1 - left_size) {
                result.push(union(left.clone(), right.clone()));
                result.push(sum(left.clone(), right));
            }
        }
    }
    result
}

#[test]
fn test_exhaustive_small_expressions() {
    let mut count = 0;
    for size in [1, 3, 5] {
        for expression in trees(size) {
            for k in 1..=10 {
                audit(expression.clone(), k, count % 211 == 0);
                count += 1;
            }
        }
    }
    assert_eq!(count, 2370);
}

fn sampled_cell(unions: usize, sums: usize) {
    // A fixed generator makes every (U,S) cell reproducible without global RNG state.
    use rand::prelude::*;
    let mut rng = rand::rngs::StdRng::seed_from_u64(2026110401);
    fn tree(u: usize, s: usize, rng: &mut rand::rngs::StdRng) -> IntExpr {
        if u + s == 0 {
            return IntExpr::Atom(rng.random_range(1..=31));
        }
        let is_union = rng.random_range(0..u + s) < u;
        let (u, s) = (u - usize::from(is_union), s - usize::from(!is_union));
        let (lu, ls) = (rng.random_range(0..=u), rng.random_range(0..=s));
        let (left, right) = (tree(lu, ls, rng), tree(u - lu, s - ls, rng));
        if is_union {
            union(left, right)
        } else {
            sum(left, right)
        }
    }
    for index in 0..300 {
        let expression = tree(unions, sums, &mut rng);
        assert_eq!(expression.count_union_nodes(), unions);
        assert_eq!(expression.count_atoms(), unions + sums + 1);
        audit(expression, rng.random_range(1..=100), index < 3);
    }
}

macro_rules! sampled_cells {
    ($(($name:ident, $u:expr, $s:expr)),* $(,)?) => {$(
        #[test] fn $name() { sampled_cell($u, $s); }
    )*};
}
sampled_cells!(
    (test_samples_unions_zero_sums_zero, 0, 0),
    (test_samples_unions_zero_sums_one, 0, 1),
    (test_samples_unions_zero_sums_two, 0, 2),
    (test_samples_unions_zero_sums_three, 0, 3),
    (test_samples_unions_zero_sums_four, 0, 4),
    (test_samples_unions_one_sums_zero, 1, 0),
    (test_samples_unions_one_sums_one, 1, 1),
    (test_samples_unions_one_sums_two, 1, 2),
    (test_samples_unions_one_sums_three, 1, 3),
    (test_samples_unions_one_sums_four, 1, 4),
    (test_samples_unions_two_sums_zero, 2, 0),
    (test_samples_unions_two_sums_one, 2, 1),
    (test_samples_unions_two_sums_two, 2, 2),
    (test_samples_unions_two_sums_three, 2, 3),
    (test_samples_unions_two_sums_four, 2, 4),
    (test_samples_unions_three_sums_zero, 3, 0),
    (test_samples_unions_three_sums_one, 3, 1),
    (test_samples_unions_three_sums_two, 3, 2),
    (test_samples_unions_three_sums_three, 3, 3),
    (test_samples_unions_three_sums_four, 3, 4),
    (test_samples_unions_four_sums_zero, 4, 0),
    (test_samples_unions_four_sums_one, 4, 1),
    (test_samples_unions_four_sums_two, 4, 2),
    (test_samples_unions_four_sums_three, 4, 3),
    (test_samples_unions_four_sums_four, 4, 4),
);

#[test]
fn test_single_atoms_sums_widths_and_overflow() {
    for width in 1..=63 {
        let power = 1i64 << (width - 1);
        audit(IntExpr::Atom(power), power, true);
        if power > 1 {
            audit(sum(IntExpr::Atom(power - 1), IntExpr::Atom(1)), power, true);
        }
    }
    for (expression, target) in [
        (sum(IntExpr::Atom(1), IntExpr::Atom(1)), 1),
        (union(IntExpr::Atom(1), IntExpr::Atom(2)), 8),
        (IntExpr::Atom(i64::MAX), i64::MAX),
        (sum(IntExpr::Atom(i64::MAX - 1), IntExpr::Atom(1)), i64::MAX),
        (sum(IntExpr::Atom(i64::MAX), IntExpr::Atom(1)), i64::MAX),
        (
            union(
                sum(IntExpr::Atom(i64::MAX), IntExpr::Atom(1)),
                IntExpr::Atom(3),
            ),
            3,
        ),
    ] {
        audit(expression, target, true);
    }
    let (_, reduction) = reduce(
        sum(IntExpr::Atom((1i64 << 62) - 1), IntExpr::Atom(1)),
        1i64 << 62,
    );
    let witness = ILPSolver::new()
        .solve(ReductionResult::target_problem(&reduction))
        .unwrap();
    assert_eq!(&witness[3 + 2 * 63..], &[vec![0; 62], vec![1; 62]].concat());
}

#[test]
fn test_checked_layout_and_numeric_boundaries() {
    for (value, expected) in [(1, 1), (2, 2), (i64::MAX, 63)] {
        assert_eq!(bit_width(value).unwrap(), expected);
    }
    assert!(bit_width(0).is_err());
    assert!(bit_width(-1).is_err());
    assert_eq!(bit_at(i64::MAX, 62).unwrap(), 1);
    assert_eq!(bit_at(i64::MAX, 63).unwrap(), 0);
    assert!(bit_at(i64::MAX, 64).is_err());
    assert!(bit_at(1, usize::MAX).is_err());
    for width in [0, 64] {
        assert!(checked_layout(1, 0, 0, width).is_err());
    }
    assert!(checked_layout(0, 0, 0, 1).is_err());
    for h in [1usize, 63] {
        for only_sums in [false, true] {
            let limit = usize::MAX;
            // Independent u128 formulas avoid overflowing the reference calculation.
            let variables =
                (u128::try_from(limit).unwrap() + 1) / u128::try_from(2 * h + 1).unwrap();
            let rows = if only_sums {
                (u128::try_from(limit - h + 1).unwrap()) / u128::try_from(h + 2).unwrap()
            } else {
                u128::try_from(limit - h).unwrap() / u128::try_from(h + 1).unwrap()
            };
            let last = usize::try_from(variables.min(rows)).unwrap();
            let counts = |a| if only_sums { (a - 1, 0) } else { (0, a - 1) };
            let (s, u) = counts(last);
            let (t, n, m) = checked_layout(last, s, u, h).unwrap();
            assert_eq!(
                u128::try_from(t).unwrap(),
                2 * u128::try_from(last).unwrap() - 1
            );
            assert_eq!(
                u128::try_from(n).unwrap(),
                u128::try_from(last).unwrap() * u128::try_from(2 * h + 1).unwrap() - 1
            );
            assert_eq!(
                u128::try_from(m).unwrap(),
                1 + 2 * u128::try_from(s).unwrap()
                    + u128::try_from(u).unwrap()
                    + u128::try_from(last).unwrap() * u128::try_from(h).unwrap()
                    + u128::try_from(h).unwrap()
            );
            let (s, u) = counts(last + 1);
            assert!(checked_layout(last + 1, s, u, h).is_err());
        }
    }
    for (a, s, u, h) in [
        (usize::MAX, 0, 0, 1),
        (usize::MAX / 2, 0, 0, 63),
        (1, usize::MAX, 0, 1),
        (1, 0, usize::MAX, 1),
    ] {
        assert!(checked_layout(a, s, u, h).is_err());
    }
}
