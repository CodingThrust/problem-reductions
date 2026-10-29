//! Reduction from Set Splitting to NAE-Satisfiability.
//!
//! Create one Boolean variable for each universe element. Each source subset
//! becomes one NAE clause over the distinct elements it contains. A subset
//! whose entries all equal a single element cannot be split, so emitting the
//! clause `(x_u, x_u)` makes the target an exact mirror of the source.

use crate::models::formula::{CNFClause, NAESatisfiability};
use crate::models::set::SetSplitting;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionSetSplittingToNAESAT {
    target: NAESatisfiability,
    source_universe_size: usize,
}

impl ReductionResult for ReductionSetSplittingToNAESAT {
    type Source = SetSplitting;
    type Target = NAESatisfiability;

    fn target_problem(&self) -> &NAESatisfiability {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0,
            "target witness does not satisfy the target problem",
        )?;

        Ok(target_solution[..self.source_universe_size].to_vec())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionSetSplittingToNAESAT {}

#[reduction(transform = {
    exact {
        num_vars = "universe_size",
        num_clauses = "num_subsets",
    },
    upper_bound {
        num_clause_variables = "universe_size * num_subsets",
        num_literals = "2 * universe_size * num_subsets",
        num_literal_pairs = "2 * universe_size * num_subsets * (2 * universe_size * num_subsets + 1)",
    },
})]
impl ReduceTo<NAESatisfiability> for SetSplitting {
    type Result = ReductionSetSplittingToNAESAT;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let universe_size = self.universe_size();
        crate::rules::sat_helpers::SatVariableAllocator::new(
            "SetSplitting -> NAESatisfiability",
            universe_size,
        )
        .map_err(<Self as ReduceTo<NAESatisfiability>>::target_construction)?;
        let mut clauses = Vec::with_capacity(self.num_subsets());

        for subset in self.subsets() {
            let mut distinct: Vec<i64> = Vec::new();
            for &element in subset {
                // SetSplitting validates element < universe_size; the allocator
                // above checks that every resulting SAT variable fits i64.
                let literal = i64::try_from(element + 1).expect("validated SAT variable");
                if !distinct.contains(&literal) {
                    distinct.push(literal);
                }
            }
            // Source construction rejects empty subsets, but permits repeated
            // occurrences of a single element. Such a subset is unsplittable.
            if distinct.len() == 1 {
                distinct.push(distinct[0]);
            }
            clauses.push(CNFClause::new(distinct));
        }
        let target = NAESatisfiability::try_new(universe_size, clauses)
            .map_err(<Self as ReduceTo<NAESatisfiability>>::target_construction)?;
        Ok(ReductionSetSplittingToNAESAT {
            target,
            source_universe_size: universe_size,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "setsplitting_to_naesatisfiability",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, NAESatisfiability>(
                SetSplitting::new(4, vec![vec![0, 1], vec![1, 2, 3], vec![0, 2, 3]]),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, true, false]),
                    target_config: serde_json::json!(vec![true, false, true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/setsplitting_naesatisfiability.rs"]
mod tests;
