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
            let kind = if pairing_offset.is_multiple_of(2) {
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
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
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

            let mut source_solution = vec![false; self.num_source_triples];

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
                    source_solution[source_triple] = true;
                }
            }

            source_solution
        })
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

fn sorted_pair(a: usize, b: usize) -> [usize; 2] {
    if a <= b {
        [a, b]
    } else {
        [b, a]
    }
}

fn enumerate_pair_keys(num_regulars: usize) -> Option<Vec<(usize, usize)>> {
    let capacity = num_regulars
        .checked_mul(num_regulars.saturating_sub(1))
        .and_then(|value| value.checked_div(2))?;
    let mut pairs = Vec::with_capacity(capacity);
    for left in 0..num_regulars {
        for right in left + 1..num_regulars {
            pairs.push((left, right));
        }
    }
    Some(pairs)
}

#[reduction(
    size = exact {
        num_elements = "24 * num_triples * num_triples - 3 * num_triples",
        num_groups = "8 * num_triples * num_triples - num_triples",
    })]
impl ReduceTo<ThreePartition> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToThreePartition;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let q = self.universe_size();
        let t = self.num_triples();

        if q == 0 {
            return Err(crate::rules::ReductionError::invalid_target::<
                ThreeDimensionalMatching,
                ThreePartition,
            >("source universe must be nonempty"));
        }
        if t == 0 {
            return Err(crate::rules::ReductionError::invalid_target::<
                ThreeDimensionalMatching,
                ThreePartition,
            >("source must contain at least one triple"));
        }

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
            return Ok(ReductionThreeDimensionalMatchingToThreePartition {
                target: ThreePartition::new(vec![6, 6, 6, 6, 7, 9], 20),
                step2_items: Vec::new(),
                pair_keys: Vec::new(),
                num_source_triples: t,
            });
        }

        let arithmetic_overflow = |context| {
            crate::rules::ReductionError::integer_overflow::<ThreeDimensionalMatching, ThreePartition>(
                context,
            )
        };
        let q = i64::try_from(q).map_err(|_| arithmetic_overflow("converting q to i64"))?;
        let r = 32_i64
            .checked_mul(q)
            .ok_or_else(|| arithmetic_overflow("computing r = 32q"))?;
        let r2 = r
            .checked_mul(r)
            .ok_or_else(|| arithmetic_overflow("computing r^2"))?;
        let r3 = r2
            .checked_mul(r)
            .ok_or_else(|| arithmetic_overflow("computing r^3"))?;
        let r4 = r3
            .checked_mul(r)
            .ok_or_else(|| arithmetic_overflow("computing r^4"))?;
        let target1 = 40_i64
            .checked_mul(r4)
            .ok_or_else(|| arithmetic_overflow("computing the ABCD-Partition target"))?;

        let mut step2_items = Vec::with_capacity(4 * t);
        let mut step2_values = Vec::with_capacity(4 * t);

        let mut seen_w = std::collections::HashSet::new();
        let mut seen_x = std::collections::HashSet::new();
        let mut seen_y = std::collections::HashSet::new();

        for (source_triple, &(w, x, y)) in self.triples().iter().enumerate() {
            let w_num = i64::try_from(w).map_err(|_| arithmetic_overflow("converting w to i64"))?;
            let x_num = i64::try_from(x).map_err(|_| arithmetic_overflow("converting x to i64"))?;
            let y_num = i64::try_from(y).map_err(|_| arithmetic_overflow("converting y to i64"))?;

            let a_value = 10_i64
                .checked_mul(r4)
                .and_then(|value| value.checked_sub(y_num.checked_mul(r3)?))
                .and_then(|value| value.checked_sub(x_num.checked_mul(r2)?))
                .and_then(|value| value.checked_sub(w_num.checked_mul(r)?))
                .ok_or_else(|| arithmetic_overflow("computing an A item"))?;
            let step2_a = 16_i64
                .checked_mul(a_value)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| arithmetic_overflow("encoding an A item"))?;
            step2_values.push(step2_a);
            step2_items.push(Step2Item::A {
                source_triple,
                w,
                x,
                y,
            });

            let w_first = seen_w.insert(w);
            let b_digit: i64 = if w_first { 10 } else { 11 };
            let b_value = b_digit
                .checked_mul(r4)
                .and_then(|value| value.checked_add(w_num.checked_mul(r)?))
                .ok_or_else(|| arithmetic_overflow("computing a B item"))?;
            let step2_b = 16_i64
                .checked_mul(b_value)
                .and_then(|value| value.checked_add(2))
                .ok_or_else(|| arithmetic_overflow("encoding a B item"))?;
            step2_values.push(step2_b);
            step2_items.push(Step2Item::B {
                w,
                first_occurrence: w_first,
            });

            let x_first = seen_x.insert(x);
            let c_digit: i64 = if x_first { 10 } else { 11 };
            let c_value = c_digit
                .checked_mul(r4)
                .and_then(|value| value.checked_add(x_num.checked_mul(r2)?))
                .ok_or_else(|| arithmetic_overflow("computing a C item"))?;
            let step2_c = 16_i64
                .checked_mul(c_value)
                .and_then(|value| value.checked_add(4))
                .ok_or_else(|| arithmetic_overflow("encoding a C item"))?;
            step2_values.push(step2_c);
            step2_items.push(Step2Item::C {
                x,
                first_occurrence: x_first,
            });

            let y_first = seen_y.insert(y);
            let d_digit: i64 = if y_first { 10 } else { 8 };
            let d_value = d_digit
                .checked_mul(r4)
                .and_then(|value| value.checked_add(y_num.checked_mul(r3)?))
                .ok_or_else(|| arithmetic_overflow("computing a D item"))?;
            let step2_d = 16_i64
                .checked_mul(d_value)
                .and_then(|value| value.checked_add(8))
                .ok_or_else(|| arithmetic_overflow("encoding a D item"))?;
            step2_values.push(step2_d);
            step2_items.push(Step2Item::D {
                y,
                first_occurrence: y_first,
            });
        }

        let target2 = 16_i64
            .checked_mul(target1)
            .and_then(|value| value.checked_add(15))
            .ok_or_else(|| arithmetic_overflow("computing the 4-Partition target"))?;
        let pair_keys = enumerate_pair_keys(step2_values.len())
            .ok_or_else(|| arithmetic_overflow("computing the 4-Partition pair count"))?;

        let subtract_fillers =
            3usize.checked_mul(t).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    ThreeDimensionalMatching,
                    ThreePartition,
                >("computing the filler count")
            })?;
        let num_fillers = 8usize
            .checked_mul(t)
            .and_then(|value| value.checked_mul(t))
            .and_then(|value| value.checked_sub(subtract_fillers))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    ThreeDimensionalMatching,
                    ThreePartition,
                >("computing the filler count")
            })?;

        let pair_elements =
            2usize.checked_mul(pair_keys.len()).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    ThreeDimensionalMatching,
                    ThreePartition,
                >("computing the number of pair elements")
            })?;
        let total_elements = step2_values
            .len()
            .checked_add(pair_elements)
            .and_then(|value| value.checked_add(num_fillers))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    ThreeDimensionalMatching,
                    ThreePartition,
                >("computing the target element count")
            })?;

        let mut sizes = Vec::with_capacity(total_elements);

        for &step2_value in &step2_values {
            let regular = 5_i64
                .checked_mul(target2)
                .and_then(|value| value.checked_add(step2_value))
                .and_then(|value| value.checked_mul(4))
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| arithmetic_overflow("computing a regular element"))?;
            sizes.push(regular);
        }

        for &(left, right) in &pair_keys {
            let pair_sum = step2_values[left]
                .checked_add(step2_values[right])
                .ok_or_else(|| arithmetic_overflow("summing paired 4-Partition elements"))?;
            let u_value = 6_i64
                .checked_mul(target2)
                .and_then(|value| value.checked_sub(pair_sum))
                .and_then(|value| value.checked_mul(4))
                .and_then(|value| value.checked_add(2))
                .ok_or_else(|| arithmetic_overflow("computing a pairing u element"))?;
            sizes.push(u_value);

            let uprime_value = 5_i64
                .checked_mul(target2)
                .and_then(|value| value.checked_add(pair_sum))
                .and_then(|value| value.checked_mul(4))
                .and_then(|value| value.checked_add(2))
                .ok_or_else(|| arithmetic_overflow("computing a pairing u' element"))?;
            sizes.push(uprime_value);
        }

        let filler_value = 20_i64
            .checked_mul(target2)
            .ok_or_else(|| arithmetic_overflow("computing a filler element"))?;
        sizes.extend(std::iter::repeat_n(filler_value, num_fillers));

        let bound = 64_i64
            .checked_mul(target2)
            .and_then(|value| value.checked_add(4))
            .ok_or_else(|| arithmetic_overflow("computing the 3-Partition bound"))?;

        Ok(ReductionThreeDimensionalMatchingToThreePartition {
            target: ThreePartition::new(sizes, bound),
            step2_items,
            pair_keys,
            num_source_triples: t,
        })
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
                    source_config: serde_json::json!(vec![true]),
                    target_config: serde_json::json!(vec![
                        0, 0, 1, 1, 0, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 2, 3, 4, 5, 6,
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threedimensionalmatching_threepartition.rs"]
mod tests;
