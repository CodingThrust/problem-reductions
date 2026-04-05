//! Reduction from ThreeDimensionalMatching to ThreePartition.
//!
//! This follows the classical three-step chain:
//! 1. 3DM -> ABCD-Partition
//! 2. ABCD-Partition -> 4-Partition
//! 3. 4-Partition -> 3-Partition

use crate::models::misc::ThreePartition;
use crate::models::set::ThreeDimensionalMatching;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Step2Item {
    A {
        source_triple: usize,
        w: usize,
        x: usize,
        y: usize,
    },
    B {
        w: usize,
        first_occurrence: bool,
    },
    C {
        x: usize,
        first_occurrence: bool,
    },
    D {
        y: usize,
        first_occurrence: bool,
    },
}

#[derive(Debug, Clone, Copy)]
enum PairingKind {
    U,
    UPrime,
}

#[derive(Debug, Default, Clone, Copy)]
struct PairUsage {
    saw_u: bool,
    uprime_regulars: Option<[usize; 2]>,
}

/// Result of reducing ThreeDimensionalMatching to ThreePartition.
#[derive(Debug, Clone)]
pub struct ReductionThreeDimensionalMatchingToThreePartition {
    target: ThreePartition,
    step2_items: Vec<Step2Item>,
    pair_keys: Vec<(usize, usize)>,
    num_source_triples: usize,
}

impl ReductionThreeDimensionalMatchingToThreePartition {
    fn num_regulars(&self) -> usize {
        self.step2_items.len()
    }

    fn pairing_start(&self) -> usize {
        self.num_regulars()
    }

    fn filler_start(&self) -> usize {
        self.pairing_start() + 2 * self.pair_keys.len()
    }

    fn classify_target_element(&self, element_index: usize) -> TargetElement {
        if element_index < self.num_regulars() {
            return TargetElement::Regular {
                step2_index: element_index,
            };
        }

        if element_index < self.filler_start() {
            let pairing_offset = element_index - self.pairing_start();
            let pair_index = pairing_offset / 2;
            let kind = if pairing_offset % 2 == 0 {
                PairingKind::U
            } else {
                PairingKind::UPrime
            };
            return TargetElement::Pairing { pair_index, kind };
        }

        TargetElement::Filler
    }

    fn decode_real_group(&self, step2_group: [usize; 4]) -> Option<usize> {
        let mut a_item = None;
        let mut b_item = None;
        let mut c_item = None;
        let mut d_item = None;

        for step2_index in step2_group {
            match self.step2_items[step2_index] {
                Step2Item::A {
                    source_triple,
                    w,
                    x,
                    y,
                } => {
                    a_item = Some((source_triple, w, x, y));
                }
                Step2Item::B {
                    w,
                    first_occurrence,
                } => {
                    b_item = Some((w, first_occurrence));
                }
                Step2Item::C {
                    x,
                    first_occurrence,
                } => {
                    c_item = Some((x, first_occurrence));
                }
                Step2Item::D {
                    y,
                    first_occurrence,
                } => {
                    d_item = Some((y, first_occurrence));
                }
            }
        }

        let (source_triple, aw, ax, ay) = a_item?;
        let (bw, b_first) = b_item?;
        let (cx, c_first) = c_item?;
        let (dy, d_first) = d_item?;

        if aw != bw || ax != cx || ay != dy {
            return None;
        }

        if b_first && c_first && d_first {
            Some(source_triple)
        } else {
            None
        }
    }

    #[cfg(test)]
    fn build_target_witness(&self, source_solution: &[usize]) -> Vec<usize> {
        let mut a_indices = vec![0usize; self.num_source_triples];
        let mut first_b_by_w = HashMap::new();
        let mut first_c_by_x = HashMap::new();
        let mut first_d_by_y = HashMap::new();
        let mut dummy_bs_by_w: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut dummy_cs_by_x: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut dummy_ds_by_y: HashMap<usize, Vec<usize>> = HashMap::new();

        for (step2_index, item) in self.step2_items.iter().copied().enumerate() {
            match item {
                Step2Item::A { source_triple, .. } => {
                    a_indices[source_triple] = step2_index;
                }
                Step2Item::B {
                    w,
                    first_occurrence,
                } => {
                    if first_occurrence {
                        first_b_by_w.insert(w, step2_index);
                    } else {
                        dummy_bs_by_w.entry(w).or_default().push(step2_index);
                    }
                }
                Step2Item::C {
                    x,
                    first_occurrence,
                } => {
                    if first_occurrence {
                        first_c_by_x.insert(x, step2_index);
                    } else {
                        dummy_cs_by_x.entry(x).or_default().push(step2_index);
                    }
                }
                Step2Item::D {
                    y,
                    first_occurrence,
                } => {
                    if first_occurrence {
                        first_d_by_y.insert(y, step2_index);
                    } else {
                        dummy_ds_by_y.entry(y).or_default().push(step2_index);
                    }
                }
            }
        }

        let mut step2_groups = Vec::with_capacity(self.num_source_triples);
        for source_triple in 0..self.num_source_triples {
            let Step2Item::A { w, x, y, .. } = self.step2_items[a_indices[source_triple]] else {
                unreachable!("A indices are populated from A items");
            };

            let group = if source_solution[source_triple] == 1 {
                [
                    a_indices[source_triple],
                    *first_b_by_w
                        .get(&w)
                        .expect("selected triple must have a first-occurrence B item"),
                    *first_c_by_x
                        .get(&x)
                        .expect("selected triple must have a first-occurrence C item"),
                    *first_d_by_y
                        .get(&y)
                        .expect("selected triple must have a first-occurrence D item"),
                ]
            } else {
                [
                    a_indices[source_triple],
                    dummy_bs_by_w
                        .get_mut(&w)
                        .and_then(|items| items.pop())
                        .expect("unselected triple must have a dummy B item"),
                    dummy_cs_by_x
                        .get_mut(&x)
                        .and_then(|items| items.pop())
                        .expect("unselected triple must have a dummy C item"),
                    dummy_ds_by_y
                        .get_mut(&y)
                        .and_then(|items| items.pop())
                        .expect("unselected triple must have a dummy D item"),
                ]
            };

            step2_groups.push(group);
        }

        let pair_to_index: HashMap<(usize, usize), usize> = self
            .pair_keys
            .iter()
            .copied()
            .enumerate()
            .map(|(pair_index, pair)| (pair, pair_index))
            .collect();
        let mut pair_used = vec![false; self.pair_keys.len()];
        let mut target_solution = vec![0usize; self.target.num_elements()];
        let mut next_group = 0usize;

        for mut step2_group in step2_groups {
            step2_group.sort_unstable();
            let pair_key = (step2_group[0], step2_group[1]);
            let pair_index = *pair_to_index
                .get(&pair_key)
                .expect("chosen regular pair must exist in the pairing gadget");
            pair_used[pair_index] = true;

            let u_index = self.pairing_start() + 2 * pair_index;
            let uprime_index = u_index + 1;

            target_solution[step2_group[0]] = next_group;
            target_solution[step2_group[1]] = next_group;
            target_solution[u_index] = next_group;
            next_group += 1;

            target_solution[step2_group[2]] = next_group;
            target_solution[step2_group[3]] = next_group;
            target_solution[uprime_index] = next_group;
            next_group += 1;
        }

        let mut filler_index = self.filler_start();
        for (pair_index, used) in pair_used.into_iter().enumerate() {
            if used {
                continue;
            }

            let u_index = self.pairing_start() + 2 * pair_index;
            let uprime_index = u_index + 1;
            target_solution[u_index] = next_group;
            target_solution[uprime_index] = next_group;
            target_solution[filler_index] = next_group;
            filler_index += 1;
            next_group += 1;
        }

        assert_eq!(filler_index, self.target.num_elements());
        assert_eq!(next_group, self.target.num_groups());

        target_solution
    }
}

impl ReductionResult for ReductionThreeDimensionalMatchingToThreePartition {
    type Source = ThreeDimensionalMatching;
    type Target = ThreePartition;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Reverse the 4-Partition -> 3-Partition pairing gadget, then decode the
    /// surviving real ABCD groups back into selected source triples.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut groups = vec![Vec::new(); self.target.num_groups()];
        for (element_index, &group_index) in target_solution.iter().enumerate() {
            groups[group_index].push(element_index);
        }

        let mut pair_usage: HashMap<(usize, usize), PairUsage> = HashMap::new();

        for members in groups.into_iter().filter(|members| !members.is_empty()) {
            let mut regulars = Vec::new();
            let mut pairing = None;
            let mut has_filler = false;

            for element_index in members {
                match self.classify_target_element(element_index) {
                    TargetElement::Regular { step2_index } => regulars.push(step2_index),
                    TargetElement::Pairing { pair_index, kind } => {
                        pairing = Some((pair_index, kind))
                    }
                    TargetElement::Filler => has_filler = true,
                }
            }

            if has_filler || regulars.len() != 2 {
                continue;
            }

            let Some((pair_index, kind)) = pairing else {
                continue;
            };

            let pair_key = self.pair_keys[pair_index];
            let regular_pair = sorted_pair(regulars[0], regulars[1]);
            let usage = pair_usage.entry(pair_key).or_default();

            match kind {
                PairingKind::U => {
                    if regular_pair == [pair_key.0, pair_key.1] {
                        usage.saw_u = true;
                    }
                }
                PairingKind::UPrime => {
                    usage.uprime_regulars = Some(regular_pair);
                }
            }
        }

        let mut source_solution = vec![0; self.num_source_triples];

        for ((left, right), usage) in pair_usage {
            let Some(other_two) = usage.uprime_regulars else {
                continue;
            };
            if !usage.saw_u {
                continue;
            }

            let mut group = [left, right, other_two[0], other_two[1]];
            group.sort_unstable();
            if group.windows(2).any(|window| window[0] == window[1]) {
                continue;
            }

            if let Some(source_triple) = self.decode_real_group(group) {
                source_solution[source_triple] = 1;
            }
        }

        source_solution
    }
}

#[derive(Debug, Clone, Copy)]
enum TargetElement {
    Regular {
        step2_index: usize,
    },
    Pairing {
        pair_index: usize,
        kind: PairingKind,
    },
    Filler,
}

fn checked_mul(lhs: u128, rhs: u128, context: &str) -> u128 {
    lhs.checked_mul(rhs)
        .unwrap_or_else(|| panic!("{context} overflowed during multiplication"))
}

fn checked_add(lhs: u128, rhs: u128, context: &str) -> u128 {
    lhs.checked_add(rhs)
        .unwrap_or_else(|| panic!("{context} overflowed during addition"))
}

fn checked_sub(lhs: u128, rhs: u128, context: &str) -> u128 {
    lhs.checked_sub(rhs)
        .unwrap_or_else(|| panic!("{context} underflowed during subtraction"))
}

fn to_u64(value: u128, context: &str) -> u64 {
    u64::try_from(value).unwrap_or_else(|_| panic!("{context} does not fit into u64"))
}

fn sorted_pair(a: usize, b: usize) -> [usize; 2] {
    if a <= b {
        [a, b]
    } else {
        [b, a]
    }
}

fn enumerate_pair_keys(num_regulars: usize) -> Vec<(usize, usize)> {
    let capacity = num_regulars
        .checked_mul(num_regulars.saturating_sub(1))
        .and_then(|value| value.checked_div(2))
        .expect("pair count overflow for 4-Partition gadget");
    let mut pairs = Vec::with_capacity(capacity);
    for left in 0..num_regulars {
        for right in left + 1..num_regulars {
            pairs.push((left, right));
        }
    }
    pairs
}

#[reduction(overhead = {
    num_elements = "24 * num_triples * num_triples - 3 * num_triples",
    num_groups = "8 * num_triples * num_triples - num_triples",
})]
impl ReduceTo<ThreePartition> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToThreePartition;

    fn reduce_to(&self) -> Self::Result {
        let q = self.universe_size();
        let t = self.num_triples();

        assert!(q > 0, "3DM -> ThreePartition requires universe_size > 0");
        assert!(
            t > 0,
            "3DM -> ThreePartition requires at least one source triple"
        );

        let mut covered_w = vec![false; q];
        let mut covered_x = vec![false; q];
        let mut covered_y = vec![false; q];
        for &(w, x, y) in self.triples() {
            covered_w[w] = true;
            covered_x[x] = true;
            covered_y[y] = true;
        }
        if covered_w.iter().any(|&covered| !covered)
            || covered_x.iter().any(|&covered| !covered)
            || covered_y.iter().any(|&covered| !covered)
        {
            return ReductionThreeDimensionalMatchingToThreePartition {
                target: ThreePartition::new(vec![6, 6, 6, 6, 7, 9], 20),
                step2_items: Vec::new(),
                pair_keys: Vec::new(),
                num_source_triples: t,
            };
        }

        let q128 = q as u128;
        let r = checked_mul(32, q128, "r = 32q");
        let r2 = checked_mul(r, r, "r^2");
        let r3 = checked_mul(r2, r, "r^3");
        let r4 = checked_mul(r3, r, "r^4");
        let target1 = checked_mul(40, r4, "T1 = 40r^4");

        let mut step2_items = Vec::with_capacity(4 * t);
        let mut step2_values = Vec::with_capacity(4 * t);

        let mut seen_w = std::collections::HashSet::new();
        let mut seen_x = std::collections::HashSet::new();
        let mut seen_y = std::collections::HashSet::new();

        for (source_triple, &(w, x, y)) in self.triples().iter().enumerate() {
            let w128 = w as u128;
            let x128 = x as u128;
            let y128 = y as u128;

            let a_value = checked_sub(
                checked_sub(
                    checked_sub(
                        checked_mul(10, r4, "A digit"),
                        checked_mul(y128, r3, "A y-term"),
                        "A after y",
                    ),
                    checked_mul(x128, r2, "A x-term"),
                    "A after x",
                ),
                checked_mul(w128, r, "A w-term"),
                "A after w",
            );
            step2_values.push(to_u64(
                checked_add(checked_mul(16, a_value, "step2 A"), 1, "step2 A tag"),
                "step2 A",
            ));
            step2_items.push(Step2Item::A {
                source_triple,
                w,
                x,
                y,
            });

            let w_first = seen_w.insert(w);
            let b_digit = if w_first { 10 } else { 11 };
            let b_value = checked_add(
                checked_mul(b_digit, r4, "B digit"),
                checked_mul(w128, r, "B coordinate"),
                "B value",
            );
            step2_values.push(to_u64(
                checked_add(checked_mul(16, b_value, "step2 B"), 2, "step2 B tag"),
                "step2 B",
            ));
            step2_items.push(Step2Item::B {
                w,
                first_occurrence: w_first,
            });

            let x_first = seen_x.insert(x);
            let c_digit = if x_first { 10 } else { 11 };
            let c_value = checked_add(
                checked_mul(c_digit, r4, "C digit"),
                checked_mul(x128, r2, "C coordinate"),
                "C value",
            );
            step2_values.push(to_u64(
                checked_add(checked_mul(16, c_value, "step2 C"), 4, "step2 C tag"),
                "step2 C",
            ));
            step2_items.push(Step2Item::C {
                x,
                first_occurrence: x_first,
            });

            let y_first = seen_y.insert(y);
            let d_digit = if y_first { 10 } else { 8 };
            let d_value = checked_add(
                checked_mul(d_digit, r4, "D digit"),
                checked_mul(y128, r3, "D coordinate"),
                "D value",
            );
            step2_values.push(to_u64(
                checked_add(checked_mul(16, d_value, "step2 D"), 8, "step2 D tag"),
                "step2 D",
            ));
            step2_items.push(Step2Item::D {
                y,
                first_occurrence: y_first,
            });
        }

        let target2 = checked_add(checked_mul(16, target1, "T2 base"), 15, "T2");
        let pair_keys = enumerate_pair_keys(step2_values.len());

        let num_fillers = 8usize
            .checked_mul(t)
            .and_then(|value| value.checked_mul(t))
            .and_then(|value| value.checked_sub(3 * t))
            .expect("filler count overflow");

        let total_elements = step2_values
            .len()
            .checked_add(2 * pair_keys.len())
            .and_then(|value| value.checked_add(num_fillers))
            .expect("3-Partition element count overflow");

        let mut sizes = Vec::with_capacity(total_elements);

        for &step2_value in &step2_values {
            let regular = checked_add(
                checked_mul(
                    4,
                    checked_add(
                        checked_mul(5, target2, "regular base"),
                        u128::from(step2_value),
                        "regular inner",
                    ),
                    "regular outer",
                ),
                1,
                "regular tag",
            );
            sizes.push(to_u64(regular, "regular element"));
        }

        for &(left, right) in &pair_keys {
            let a_i = u128::from(step2_values[left]);
            let a_j = u128::from(step2_values[right]);

            let u_value = checked_add(
                checked_mul(
                    4,
                    checked_sub(
                        checked_mul(6, target2, "u base"),
                        checked_add(a_i, a_j, "u pair sum"),
                        "u inner",
                    ),
                    "u outer",
                ),
                2,
                "u tag",
            );
            sizes.push(to_u64(u_value, "pairing u element"));

            let uprime_value = checked_add(
                checked_mul(
                    4,
                    checked_add(
                        checked_mul(5, target2, "u' base"),
                        checked_add(a_i, a_j, "u' pair sum"),
                        "u' inner",
                    ),
                    "u' outer",
                ),
                2,
                "u' tag",
            );
            sizes.push(to_u64(uprime_value, "pairing u' element"));
        }

        let filler_value = to_u64(checked_mul(20, target2, "filler"), "filler element");
        sizes.extend(std::iter::repeat(filler_value).take(num_fillers));

        let bound = to_u64(
            checked_add(
                checked_mul(64, target2, "3-Partition bound"),
                4,
                "3-Partition bound tag",
            ),
            "3-Partition bound",
        );

        ReductionThreeDimensionalMatchingToThreePartition {
            target: ThreePartition::new(sizes, bound),
            step2_items,
            pair_keys,
            num_source_triples: t,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threedimensionalmatching_to_threepartition",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ThreePartition>(
                ThreeDimensionalMatching::new(1, vec![(0, 0, 0)]),
                SolutionPair {
                    source_config: vec![1],
                    target_config: vec![
                        0, 0, 1, 1, 0, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 2, 3, 4, 5, 6,
                    ],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threedimensionalmatching_threepartition.rs"]
mod tests;
