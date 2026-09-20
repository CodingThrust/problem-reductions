//! Reduction from ConsistencyOfDatabaseFrequencyTables to ILP.
//!
//! The reduction uses a binary one-hot encoding:
//! - `y_{v,a,x}` is 1 iff object `v` receives value `x` for attribute `a`
//! - `z_{t,v,x,y}` is 1 iff, for table `t`, object `v` realizes cell `(x, y)`
//!
//! The pair-count equalities are linearized with standard McCormick constraints.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::ConsistencyOfDatabaseFrequencyTables;
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ConsistencyOfDatabaseFrequencyTables to ILP.
#[derive(Debug, Clone)]
pub struct ReductionCDFTToILP {
    target: ILP<bool>,
    source: ConsistencyOfDatabaseFrequencyTables,
}

impl ReductionCDFTToILP {
    fn assignment_block_size(&self) -> usize {
        self.source.attribute_domains().iter().sum()
    }

    fn attribute_offset(&self, attribute: usize) -> usize {
        self.source.attribute_domains()[..attribute].iter().sum()
    }

    fn assignment_var_index(&self, object: usize, attribute: usize, value: usize) -> usize {
        object * self.assignment_block_size() + self.attribute_offset(attribute) + value
    }

    fn auxiliary_block_start(&self, table_index: usize) -> usize {
        self.source.num_objects() * self.assignment_block_size()
            + self.source.frequency_tables()[..table_index]
                .iter()
                .map(|table| self.source.num_objects() * table.num_cells())
                .sum::<usize>()
    }

    fn auxiliary_var_index(
        &self,
        table_index: usize,
        object: usize,
        value_a: usize,
        value_b: usize,
    ) -> usize {
        let table = &self.source.frequency_tables()[table_index];
        let cols = self.source.attribute_domains()[table.attribute_b()];
        self.auxiliary_block_start(table_index)
            + object * table.num_cells()
            + value_a * cols
            + value_b
    }

    /// Encode a satisfying source assignment as a concrete ILP variable vector.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn encode_source_solution(&self, source_solution: &[usize]) -> Vec<i64> {
        let mut target_solution = vec![0_i64; self.target.num_vars()];
        let num_attributes = self.source.num_attributes();

        for object in 0..self.source.num_objects() {
            for attribute in 0..num_attributes {
                let source_index = object * num_attributes + attribute;
                let value = source_solution[source_index];
                let var = self.assignment_var_index(object, attribute, value);
                target_solution[var] = 1;
            }
        }

        for (table_index, table) in self.source.frequency_tables().iter().enumerate() {
            for object in 0..self.source.num_objects() {
                let value_a = source_solution[object * num_attributes + table.attribute_a()];
                let value_b = source_solution[object * num_attributes + table.attribute_b()];
                let var = self.auxiliary_var_index(table_index, object, value_a, value_b);
                target_solution[var] = 1;
            }
        }

        target_solution
    }
}

impl ReductionResult for ReductionCDFTToILP {
    type Source = ConsistencyOfDatabaseFrequencyTables;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if value.value.is_none() {
            return Err(crate::rules::ExtractionError::invalid(
                "target ILP assignment is infeasible",
            ));
        }

        Ok({
            let mut source_solution = Vec::with_capacity(self.source.num_assignment_variables());
            for object in 0..self.source.num_objects() {
                for (attribute, &domain_size) in self.source.attribute_domains().iter().enumerate()
                {
                    let mut selected = (0..domain_size).filter(|&candidate| {
                        target_solution[self.assignment_var_index(object, attribute, candidate)]
                            == 1
                    });
                    let value = match (selected.next(), selected.next()) {
                        (Some(value), None) => value,
                        (None, _) => {
                            return Err(crate::rules::ExtractionError::invalid(format!(
                                "object {object}, attribute {attribute} has no selected value"
                            )))
                        }
                        (Some(_), Some(_)) => {
                            return Err(crate::rules::ExtractionError::invalid(format!(
                                "object {object}, attribute {attribute} has multiple selected values"
                            )))
                        }
                    };
                    source_solution.push(value);
                }
            }
            source_solution
        })
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionCDFTToILP {
    type Source = ConsistencyOfDatabaseFrequencyTables;
    type Target = ILP<bool>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_value(&self, value: crate::types::Extremum<i64>) -> crate::types::Or {
        crate::types::Or(value.value.is_some())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_objects * total_domain_size + num_objects * num_frequency_cells",
        num_constraints = "num_objects * num_attributes + num_known_values + num_frequency_cells + 3 * num_objects * num_frequency_cells",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for ConsistencyOfDatabaseFrequencyTables {
    type Result = ReductionCDFTToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(
                "representing the database ILP encoding",
            )
        };
        let assignments = self
            .num_objects()
            .checked_mul(self.total_domain_size())
            .ok_or_else(overflow)?;
        let auxiliaries = self
            .num_objects()
            .checked_mul(self.num_frequency_cells())
            .ok_or_else(overflow)?;
        let num_vars = assignments.checked_add(auxiliaries).ok_or_else(overflow)?;
        let num_constraints = auxiliaries
            .checked_mul(3)
            .and_then(|count| count.checked_add(self.num_assignment_variables()))
            .and_then(|count| count.checked_add(self.num_known_values()))
            .and_then(|count| count.checked_add(self.num_frequency_cells()))
            .ok_or_else(overflow)?;
        let source = self.clone();
        let helper = ReductionCDFTToILP {
            target: ILP::empty(),
            source: source.clone(),
        };

        let allocation_error = |error| {
            crate::rules::ReductionError::invalid_target::<Self, ILP<bool>>(format!(
                "cannot allocate database ILP encoding: {error}"
            ))
        };
        let mut constraints = Vec::new();
        constraints
            .try_reserve_exact(num_constraints)
            .map_err(allocation_error)?;

        for object in 0..source.num_objects() {
            for (attribute, &domain_size) in source.attribute_domains().iter().enumerate() {
                let mut terms = Vec::new();
                terms
                    .try_reserve_exact(domain_size)
                    .map_err(allocation_error)?;
                terms.extend(
                    (0..domain_size)
                        .map(|value| (helper.assignment_var_index(object, attribute, value), 1)),
                );
                constraints.push(LinearConstraint::eq(terms, 1));
            }
        }

        for known_value in source.known_values() {
            constraints.push(LinearConstraint::eq(
                vec![(
                    helper.assignment_var_index(
                        known_value.object(),
                        known_value.attribute(),
                        known_value.value(),
                    ),
                    1,
                )],
                1,
            ));
        }

        for (table_index, table) in source.frequency_tables().iter().enumerate() {
            let rows = source.attribute_domains()[table.attribute_a()];
            let cols = source.attribute_domains()[table.attribute_b()];

            for value_a in 0..rows {
                for value_b in 0..cols {
                    let count_terms = (0..source.num_objects())
                        .map(|object| {
                            (
                                helper.auxiliary_var_index(table_index, object, value_a, value_b),
                                1,
                            )
                        })
                        .collect();
                    let count = table.counts()[value_a][value_b];
                    constraints.push(LinearConstraint::eq(count_terms, count));

                    for object in 0..source.num_objects() {
                        let z = helper.auxiliary_var_index(table_index, object, value_a, value_b);
                        let y_a = helper.assignment_var_index(object, table.attribute_a(), value_a);
                        let y_b = helper.assignment_var_index(object, table.attribute_b(), value_b);

                        constraints.extend(mccormick_product(z, y_a, y_b));
                    }
                }
            }
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionCDFTToILP { target, source })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::models::misc::{FrequencyTable, KnownValue};

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "consistencyofdatabasefrequencytables_to_ilp",
        build: || {
            let source = ConsistencyOfDatabaseFrequencyTables::new(
                6,
                vec![2, 3, 2],
                vec![
                    FrequencyTable::new(0, 1, vec![vec![1, 1, 1], vec![1, 1, 1]]),
                    FrequencyTable::new(1, 2, vec![vec![1, 1], vec![0, 2], vec![1, 1]]),
                ],
                vec![
                    KnownValue::new(0, 0, 0),
                    KnownValue::new(3, 0, 1),
                    KnownValue::new(1, 2, 1),
                ],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/consistencyofdatabasefrequencytables_ilp.rs"]
mod tests;
