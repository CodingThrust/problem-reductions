//! Eligible-assignment bits with exact demand and resource conflict penalties.
use crate::models::algebraic::QUBO;
use crate::models::decision::Decision;
use crate::models::misc::TimetableDesign;
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionResult, ReduceTo, ReductionError, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionTimetableDesignToQUBO {
    target: Decision<QUBO<i64>>,
    coordinates: Vec<(usize, usize, usize)>,
    craftsmen: usize,
    tasks: usize,
    periods: usize,
}
impl ReductionResult for ReductionTimetableDesignToQUBO {
    type Source = TimetableDesign;
    type Target = Decision<QUBO<i64>>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(&self, bits: &Vec<bool>) -> ExtractionResult<Vec<Vec<Vec<bool>>>> {
        validate_target_witness(
            &self.target,
            bits,
            |v| v.0,
            "QUBO does not certify a feasible timetable",
        )?;
        let mut schedule = vec![vec![vec![false; self.periods]; self.tasks]; self.craftsmen];
        for (&(c, t, h), &selected) in self.coordinates.iter().zip(bits) {
            schedule[c][t][h] = selected;
        }
        Ok(schedule)
    }
}
#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionTimetableDesignToQUBO {}

#[crate::reduction(transform = {
    exact { num_vars = "num_available_assignments" },
    upper_bound { num_quadratic_terms = "num_available_assignments * (num_available_assignments - 1) / 2" },
})]
impl ReduceTo<Decision<QUBO<i64>>> for TimetableDesign {
    type Result = ReductionTimetableDesignToQUBO;
    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        type Builder = QuboBuilder<TimetableDesign, Decision<QUBO<i64>>>;
        let mut coordinates = Vec::with_capacity(self.num_available_assignments());
        let mut demands = Vec::new();
        let mut impossible = false;
        for c in 0..self.num_craftsmen() {
            for t in 0..self.num_tasks() {
                let start = coordinates.len();
                let required = self.requirements()[c][t];
                if required > 0 {
                    for h in 0..self.num_periods() {
                        if self.craftsman_avail()[c][h] && self.task_avail()[t][h] {
                            coordinates.push((c, t, h));
                        }
                    }
                }
                let count =
                    i64::try_from(coordinates.len() - start).map_err(|_| Builder::overflow())?;
                impossible |= required < 0 || required > count;
                demands.push((start..coordinates.len(), required));
            }
        }
        let mut q = Builder::new(coordinates.len());
        let bound: i64 = if impossible {
            // An impossible demand has no timetable; preserve the advertised variable count.
            -1
        } else {
            for (row, required) in demands {
                q.square(&row.collect::<Vec<_>>(), required, 1)?;
            }
            for (a, &(c, t, h)) in coordinates.iter().enumerate() {
                for (b, &(other_c, other_t, other_h)) in coordinates.iter().enumerate().skip(a + 1)
                {
                    if h == other_h && (c == other_c || t == other_t) {
                        q.add(a, b, 1)?;
                    }
                }
            }
            0
        };
        let (target, constant) = q.finish()?;
        let bound = bound.checked_sub(constant).ok_or_else(Builder::overflow)?;
        Ok(ReductionTimetableDesignToQUBO {
            target: Decision::new(target, bound),
            coordinates,
            craftsmen: self.num_craftsmen(),
            tasks: self.num_tasks(),
            periods: self.num_periods(),
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "timetabledesign_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                TimetableDesign::new(
                    3,
                    3,
                    1,
                    vec![
                        vec![true, false, false],
                        vec![false, true, false],
                        vec![false, false, true],
                    ],
                    vec![vec![true; 3]],
                    vec![vec![1]; 3],
                ),
                crate::export::SolutionPair {
                    source_config: serde_json::json!([
                        [[true, false, false]],
                        [[false, true, false]],
                        [[false, false, true]]
                    ]),
                    target_config: serde_json::json!([true, true, true]),
                },
            )
        },
    }]
}
