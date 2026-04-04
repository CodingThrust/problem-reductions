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
        self.source.num_assignment_indicators()
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
    pub(crate) fn encode_source_solution(&self, source_solution: &[usize]) -> Vec<usize> {
        let mut target_solution = vec![0usize; self.target.num_vars];
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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut source_solution = Vec::with_capacity(self.source.num_assignment_variables());
        for object in 0..self.source.num_objects() {
            for (attribute, &domain_size) in self.source.attribute_domains().iter().enumerate() {
                let value = (0..domain_size)
                    .find(|&candidate| {
                        target_solution
                            .get(self.assignment_var_index(object, attribute, candidate))
                            .copied()
                            .unwrap_or(0)
                            == 1
                    })
                    .unwrap_or(0);
                source_solution.push(value);
            }
        }
        source_solution
    }
}

#[reduction(
    overhead = {
        num_vars = "num_assignment_indicators + num_auxiliary_frequency_indicators",
        num_constraints = "num_assignment_variables + num_known_values + num_frequency_cells + 3 * num_auxiliary_frequency_indicators",
    }
)]
impl ReduceTo<ILP<bool>> for ConsistencyOfDatabaseFrequencyTables {
    type Result = ReductionCDFTToILP;

    fn reduce_to(&self) -> Self::Result {
        let source = self.clone();
        let helper = ReductionCDFTToILP {
            target: ILP::empty(),
            source: source.clone(),
        };

        let mut constraints = Vec::with_capacity(
            source.num_assignment_variables()
                + source.num_known_values()
                + source.num_frequency_cells()
                + 3 * source.num_auxiliary_frequency_indicators(),
        );

        for object in 0..source.num_objects() {
            for (attribute, &domain_size) in source.attribute_domains().iter().enumerate() {
                let terms = (0..domain_size)
                    .map(|value| (helper.assignment_var_index(object, attribute, value), 1.0))
                    .collect();
                constraints.push(LinearConstraint::eq(terms, 1.0));
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
                    1.0,
                )],
                1.0,
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
                                1.0,
                            )
                        })
                        .collect();
                    constraints.push(LinearConstraint::eq(
                        count_terms,
                        table.counts()[value_a][value_b] as f64,
                    ));

                    for object in 0..source.num_objects() {
                        let z = helper.auxiliary_var_index(table_index, object, value_a, value_b);
                        let y_a = helper.assignment_var_index(object, table.attribute_a(), value_a);
                        let y_b = helper.assignment_var_index(object, table.attribute_b(), value_b);

                        constraints.push(LinearConstraint::le(vec![(z, 1.0), (y_a, -1.0)], 0.0));
                        constraints.push(LinearConstraint::le(vec![(z, 1.0), (y_b, -1.0)], 0.0));
                        constraints.push(LinearConstraint::ge(
                            vec![(z, 1.0), (y_a, -1.0), (y_b, -1.0)],
                            -1.0,
                        ));
                    }
                }
            }
        }

        let target = ILP::new(
            source.num_assignment_indicators() + source.num_auxiliary_frequency_indicators(),
            constraints,
            vec![],
            ObjectiveSense::Minimize,
        );

        ReductionCDFTToILP { target, source }
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
