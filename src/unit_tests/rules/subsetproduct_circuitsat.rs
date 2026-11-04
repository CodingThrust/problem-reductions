use crate::models::{CircuitSAT, SubsetProduct};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::{ReduceTo, ReductionResult};
use crate::types::Or;
use crate::{BruteForce, Problem};

#[test]
fn test_subsetproduct_to_circuitsat_closed_loop() {
    for target in [1u32, 2] {
        let source = SubsetProduct::new(Vec::<u32>::new(), target);
        let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
        if target == 1 {
            assert_satisfaction_round_trip_from_satisfaction_target(
                &source,
                &result,
                "empty product",
            );
        }
        assert_eq!(
            BruteForce::new()
                .solve(result.target_problem())
                .unwrap()
                .is_some(),
            target == 1
        );
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&result, Or(target == 1)),
            Or(target == 1)
        );
    }
}

#[test]
fn test_subsetproduct_to_circuitsat_paper_example() {
    let source = SubsetProduct::new(vec![2u32, 3, 5], 30u32);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
    let target = result.target_problem();
    assert_eq!(target.num_variables(), 235);
    assert_eq!(target.num_assignments(), 240);
    assert_eq!(target.num_assignment_outputs(), 240);
    assert_eq!(target.num_expression_nodes(), 808);
}

#[test]
fn test_subsetproduct_to_circuitsat_rejects_invalid_witnesses() {
    let source = SubsetProduct::new(vec![2u32], 2u32);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
    let size = result.target_problem().num_variables();
    for length in [0, size - 1, size, size + 1] {
        assert!(result.extract_solution(&vec![false; length]).is_err());
    }
}

use super::{dimensions, reserve_assignments, sum_sizes};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;

// Evaluate fresh equations only; repeated outputs remain constraints in evaluate().
fn extension(target: &CircuitSAT, selectors: &[bool]) -> (Vec<bool>, HashMap<String, bool>) {
    let mut values: HashMap<_, _> = selectors
        .iter()
        .enumerate()
        .map(|(i, &v)| (format!("x_{i}"), v))
        .collect();
    for assignment in &target.circuit().assignments {
        for name in assignment.expr.variables() {
            assert!(values.contains_key(&name), "undefined input {name}");
        }
        let value = assignment.expr.evaluate(&values);
        for name in &assignment.outputs {
            values.entry(name.clone()).or_insert(value);
        }
    }
    let witness = target
        .variable_names()
        .iter()
        .map(|name| values[name])
        .collect();
    (witness, values)
}

fn word(values: &HashMap<String, bool>, prefix: &str, suffix: &str, width: usize) -> BigUint {
    (0..width)
        .filter(|b| values[&format!("{prefix}{b}{suffix}")])
        .fold(BigUint::zero(), |a, b| a + (BigUint::one() << b))
}

fn audit(source: &SubsetProduct) {
    let result = ReduceTo::<CircuitSAT>::reduce_to(source).unwrap();
    let target = result.target_problem();
    let d: u64 = source.sizes().iter().map(BigUint::bits).sum();
    let h: u64 = source.sizes().iter().map(BigUint::count_ones).sum();
    let w = usize::try_from((d + 1).max(source.target().bits())).unwrap();
    let n = source.sizes().len();
    let h = usize::try_from(h).unwrap();
    assert_eq!(target.num_variables(), w * (1 + n + 5 * h) + n);
    assert_eq!(target.num_assignments(), w * (2 + n + 5 * h));
    assert_eq!(target.num_assignment_outputs(), target.num_assignments());
    assert_eq!(target.num_expression_nodes(), w * (2 + 8 * n + 15 * h));
    for mask in 0..(1usize << n) {
        let selectors: Vec<_> = (0..n).map(|i| mask & (1 << i) != 0).collect();
        let (witness, values) = extension(target, &selectors);
        let mut prefix = BigUint::one();
        assert_eq!(word(&values, "p_0_", "", w), prefix);
        for (i, a) in source.sizes().iter().enumerate() {
            let mut sum = BigUint::zero();
            for j in (0..a.bits()).filter(|&j| a.bit(j)) {
                sum += &prefix << usize::try_from(j).unwrap();
                assert!(sum.bits() <= u64::try_from(w).unwrap());
                assert_eq!(word(&values, &format!("a_{}_{j}_", i + 1), "_s", w), sum);
                assert!(!values[&format!("a_{}_{j}_{}_d", i + 1, w - 1)]);
            }
            assert_eq!(sum, &prefix * a);
            if selectors[i] {
                prefix *= a;
            }
            assert_eq!(word(&values, &format!("p_{}_", i + 1), "", w), prefix);
        }
        let feasible = prefix == *source.target();
        assert_eq!(source.evaluate(&selectors).unwrap(), Or(feasible));
        assert_eq!(target.evaluate(&witness).unwrap(), Or(feasible));
        if feasible {
            assert_eq!(result.extract_solution(&witness).unwrap(), selectors);
        } else {
            assert!(result.extract_solution(&witness).is_err());
        }
    }
}

#[test]
fn test_carries_repeated_factors_and_unreachable_products() {
    for (sizes, target) in [
        (vec![3, 3], 9),
        (vec![3, 3], 7),
        (vec![3, 3], 15),
        (vec![7, 7], 49),
        (vec![2, 2, 2], 8),
        (vec![2, 3, 5], 7),
        (vec![1, 1, 1], 1),
        (vec![1], 2),
        (vec![5], 5),
        (vec![5], 1),
        (vec![5], 3),
        (vec![2, 3], 4096),
    ] {
        audit(&SubsetProduct::new(sizes, target));
    }
}

fn exhaustive(n: usize, masks: std::ops::Range<usize>) {
    for mask in masks {
        let sizes: Vec<u32> = (0..n)
            .map(|i| if mask & (1 << i) == 0 { 1 } else { 2 })
            .collect();
        for target in 1..=(1u32 << n) + 1 {
            audit(&SubsetProduct::new(sizes.clone(), target));
        }
    }
}

#[test]
fn test_exhaustive_zero_to_two_elements() {
    for n in 0..=2 {
        exhaustive(n, 0..(1 << n));
    }
}
#[test]
fn test_exhaustive_three_elements() {
    exhaustive(3, 0..8);
}
#[test]
fn test_exhaustive_four_elements() {
    exhaustive(4, 0..16);
}
// Partition the exhaustive five-element family to keep each test below five seconds.
macro_rules! five_element_tests {
    ($($name:ident: $start:expr),* $(,)?) => { $(
        #[test] fn $name() { exhaustive(5, $start..$start + 4); }
    )* };
}
five_element_tests! {
    test_five_elements_ending_in_one_one_one: 0,
    test_five_elements_ending_in_two_one_one: 4,
    test_five_elements_ending_in_one_two_one: 8,
    test_five_elements_ending_in_two_two_one: 12,
    test_five_elements_ending_in_one_one_two: 16,
    test_five_elements_ending_in_two_one_two: 20,
    test_five_elements_ending_in_one_two_two: 24,
    test_five_elements_ending_in_two_two_two: 28,
}

#[test]
fn test_wide_selected_and_unselected_branches() {
    let power = BigUint::one() << 128usize;
    audit(&SubsetProduct::new(vec![power.clone()], power.clone()));
    for target in [BigUint::from(3u32), (&power + 1u32) * 3u32] {
        audit(&SubsetProduct::new(
            vec![&power + 1u32, BigUint::from(3u32)],
            target,
        ));
    }
    audit(&SubsetProduct::new(
        vec![BigUint::one()],
        (BigUint::one() << 257usize) + 1u32,
    ));
}

#[test]
fn test_selector_names_preserve_numeric_order() {
    let source = SubsetProduct::new(vec![1u32; 12], 1u32);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
    let selectors: Vec<_> = (0..12).map(|i| [2, 9, 11].contains(&i)).collect();
    let (witness, _) = extension(result.target_problem(), &selectors);
    assert_eq!(result.extract_solution(&witness).unwrap(), selectors);
}

#[test]
fn test_every_canonical_witness_bit_is_necessary() {
    let source = SubsetProduct::new(vec![2u32, 3, 5], 30u32);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
    let (mut witness, _) = extension(result.target_problem(), &[true; 3]);
    assert_eq!(result.extract_solution(&witness).unwrap(), [true; 3]);
    for i in 0..witness.len() {
        witness[i] = !witness[i];
        assert!(
            result.extract_solution(&witness).is_err(),
            "corruption at {i}"
        );
        witness[i] = !witness[i];
    }
}

#[test]
fn test_size_sums_and_all_dimension_boundaries() {
    assert_eq!(sum_sizes([usize::MAX - 1, 1]).unwrap(), usize::MAX);
    assert!(matches!(
        sum_sizes([usize::MAX, 1]),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
    let max = BigUint::from(usize::MAX);
    let mut values = vec![0, 1, 2, usize::MAX];
    for divisor in [
        2,
        5,
        8,
        15,
        std::mem::size_of::<crate::models::formula::Assignment>(),
        std::mem::size_of::<crate::models::formula::BooleanExpr>(),
        std::mem::size_of::<String>(),
    ] {
        let boundary = usize::MAX / divisor;
        values.extend([boundary - 1, boundary, boundary + 1]);
    }
    for &value in &values {
        for (n, d, h, b) in [
            (value, 0, 0, 1),
            (0, value, 0, 1),
            (0, 0, value, 1),
            (0, 0, 0, value),
            (1, 1, value, 2),
        ] {
            let width = (BigUint::from(d) + 1u32).max(BigUint::from(b));
            let vars = &width * (BigUint::from(n) + BigUint::from(h) * 5u32 + 1u32) + n;
            let assignments = &width * (BigUint::from(n) + BigUint::from(h) * 5u32 + 2u32);
            let nodes = &width * (BigUint::from(n) * 8u32 + BigUint::from(h) * 15u32 + 2u32);
            let fits = [&width, &vars, &assignments, &nodes]
                .into_iter()
                .all(|v| v <= &max)
                && &assignments * std::mem::size_of::<crate::models::formula::Assignment>() <= max
                && &nodes * std::mem::size_of::<crate::models::formula::BooleanExpr>() <= max
                && &vars * std::mem::size_of::<String>() <= max;
            match dimensions(n, d, h, b) {
                Ok(actual) => {
                    assert!(fits, "accepted overflowing dimensions {:?}", (n, d, h, b));
                    assert_eq!(
                        (
                            BigUint::from(actual.0),
                            BigUint::from(actual.1),
                            BigUint::from(actual.2),
                            BigUint::from(actual.3)
                        ),
                        (width, vars, assignments, nodes)
                    );
                }
                Err(error) => {
                    assert!(
                        !fits,
                        "rejected representable dimensions {:?}: {error}",
                        (n, d, h, b)
                    );
                    assert!(matches!(
                        error,
                        crate::rules::ReductionError::IntegerOverflow { .. }
                    ));
                }
            }
        }
    }
}

#[test]
fn test_assignment_allocation_failure_is_typed() {
    assert!(reserve_assignments(1).is_ok());
    assert!(matches!(
        reserve_assignments(usize::MAX),
        Err(crate::rules::ReductionError::Construction { .. })
    ));
}

#[test]
fn test_missing_selector_coordinate_is_rejected() {
    let source = SubsetProduct::new(vec![1u32], 1u32);
    let mut result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
    let (witness, _) = extension(result.target_problem(), &[true]);
    result.source_var_indices[0] = usize::MAX;
    assert!(result.extract_solution(&witness).is_err());
}

#[test]
fn test_registered_decision_mapping_and_unavailable_magnitudes() {
    let entries = crate::rules::registry::reduction_entries();
    let entry = entries
        .iter()
        .find(|e| e.source_name == "SubsetProduct" && e.target_name == "CircuitSAT")
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    assert!(contract.transform().is_none());
    assert_eq!(contract.unavailable().len(), 4);
    for field in [
        "num_variables",
        "num_assignments",
        "num_assignment_outputs",
        "num_expression_nodes",
    ] {
        assert!(
            contract
                .unavailable()
                .iter()
                .any(|entry| entry.field == field && !entry.reason.is_empty())
        );
    }
    let source = SubsetProduct::new(vec![1u32], 1u32);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
    for value in [false, true] {
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(&result, Or(value)),
            Or(value)
        );
    }
}

#[test]
fn test_all_target_witnesses_for_a_free_unit_selector() {
    for target in [1u32, 2] {
        let source = SubsetProduct::new(vec![1u32], target);
        let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
        let witnesses = BruteForce::new()
            .find_all_witnesses(result.target_problem())
            .unwrap();
        let mut extracted: Vec<_> = witnesses
            .iter()
            .map(|w| result.extract_solution(w).unwrap())
            .collect();
        extracted.sort();
        let expected = if target == 1 {
            vec![vec![false], vec![true]]
        } else {
            vec![]
        };
        assert_eq!(extracted, expected);
        assert_eq!(
            crate::rules::AggregateReductionResult::extract_value(
                &result,
                Or(!witnesses.is_empty())
            ),
            Or(target == 1)
        );
    }
}

#[cfg(feature = "example-db")]
#[test]
fn test_canonical_export_has_a_complete_valid_witness() {
    let examples = super::canonical_rule_example_specs();
    let example = (examples[0].build)();
    let source: SubsetProduct = serde_json::from_value(example.source.instance).unwrap();
    let target: CircuitSAT = serde_json::from_value(example.target.instance).unwrap();
    let source_witness: Vec<bool> =
        serde_json::from_value(example.solutions[0].source_config.clone()).unwrap();
    let target_witness: Vec<bool> =
        serde_json::from_value(example.solutions[0].target_config.clone()).unwrap();
    assert_eq!(source.sizes(), &[2u32.into(), 3u32.into(), 5u32.into()]);
    assert_eq!(source.target(), &30u32.into());
    assert_eq!(source.evaluate(&source_witness).unwrap(), Or(true));
    assert_eq!(target.evaluate(&target_witness).unwrap(), Or(true));
    assert_eq!(
        ReduceTo::<CircuitSAT>::reduce_to(&source)
            .unwrap()
            .extract_solution(&target_witness)
            .unwrap(),
        source_witness
    );
}

fn random_extensions(n: usize) {
    use rand::SeedableRng;
    use rand::rngs::SmallRng;
    use rand::seq::IndexedRandom;
    let mut rng = SmallRng::seed_from_u64(42 + u64::try_from(n).unwrap());
    for _ in 0..300 {
        let sizes: Vec<u32> = (0..n)
            .map(|_| *[1, 2, 3, 5, 7, 9, 15, 31].choose(&mut rng).unwrap())
            .collect();
        let selectors: Vec<bool> = (0..n)
            .map(|_| *[false, true].choose(&mut rng).unwrap())
            .collect();
        let product: u64 = sizes
            .iter()
            .zip(&selectors)
            .filter(|(_, selected)| **selected)
            .map(|(&a, _)| u64::from(a))
            .product();
        let target = product + u64::from(*[false, true].choose(&mut rng).unwrap());
        let source = SubsetProduct::new(sizes, target);
        let result = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
        let (witness, _) = extension(result.target_problem(), &selectors);
        assert_eq!(
            result.target_problem().evaluate(&witness).unwrap(),
            Or(product == target)
        );
        if product == target {
            assert_eq!(result.extract_solution(&witness).unwrap(), selectors);
        } else {
            assert!(result.extract_solution(&witness).is_err());
        }
    }
}

#[test]
fn test_random_single_element_extensions() {
    random_extensions(1);
}
#[test]
fn test_random_two_element_extensions() {
    random_extensions(2);
}
#[test]
fn test_random_three_element_extensions() {
    random_extensions(3);
}
#[test]
fn test_random_four_element_extensions() {
    random_extensions(4);
}
#[test]
fn test_random_five_element_extensions() {
    random_extensions(5);
}
