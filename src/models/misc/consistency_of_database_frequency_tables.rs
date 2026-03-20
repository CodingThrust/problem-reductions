//! Consistency of Database Frequency Tables problem implementation.
//!
//! Given a finite set of objects, categorical attributes with finite domains,
//! pairwise frequency tables for selected attribute pairs, and some known
//! object-attribute values, determine whether there exists a complete
//! assignment of attribute values to all objects that matches every published
//! frequency table and every known value.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrequencyTable {
    attribute_a: usize,
    attribute_b: usize,
    counts: Vec<Vec<usize>>,
}

impl FrequencyTable {
    /// Create a new pairwise frequency table.
    pub fn new(attribute_a: usize, attribute_b: usize, counts: Vec<Vec<usize>>) -> Self {
        Self {
            attribute_a,
            attribute_b,
            counts,
        }
    }

    /// Returns the first attribute index.
    pub fn attribute_a(&self) -> usize {
        self.attribute_a
    }

    /// Returns the second attribute index.
    pub fn attribute_b(&self) -> usize {
        self.attribute_b
    }

    /// Returns the table counts.
    pub fn counts(&self) -> &[Vec<usize>] {
        &self.counts
    }

    /// Returns the number of table cells.
    pub fn num_cells(&self) -> usize {
        self.counts.iter().map(Vec::len).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownValue {
    object: usize,
    attribute: usize,
    value: usize,
}

impl KnownValue {
    /// Create a new known object-attribute value.
    pub fn new(object: usize, attribute: usize, value: usize) -> Self {
        Self {
            object,
            attribute,
            value,
        }
    }

    /// Returns the object index.
    pub fn object(&self) -> usize {
        self.object
    }

    /// Returns the attribute index.
    pub fn attribute(&self) -> usize {
        self.attribute
    }

    /// Returns the fixed categorical value.
    pub fn value(&self) -> usize {
        self.value
    }
}

inventory::submit! {
    ProblemSchemaEntry {
        name: "ConsistencyOfDatabaseFrequencyTables",
        display_name: "Consistency of Database Frequency Tables",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether pairwise frequency tables and known values admit a consistent complete database assignment",
        fields: &[
            FieldInfo { name: "num_objects", type_name: "usize", description: "Number of objects in the database" },
            FieldInfo { name: "attribute_domains", type_name: "Vec<usize>", description: "Domain size for each attribute" },
            FieldInfo { name: "frequency_tables", type_name: "Vec<FrequencyTable>", description: "Published pairwise frequency tables" },
            FieldInfo { name: "known_values", type_name: "Vec<KnownValue>", description: "Known object-attribute-value triples" },
        ],
    }
}

/// The Consistency of Database Frequency Tables decision problem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsistencyOfDatabaseFrequencyTables {
    num_objects: usize,
    attribute_domains: Vec<usize>,
    frequency_tables: Vec<FrequencyTable>,
    known_values: Vec<KnownValue>,
}

impl ConsistencyOfDatabaseFrequencyTables {
    /// Create a new consistency-of-database-frequency-tables instance.
    pub fn new(
        num_objects: usize,
        attribute_domains: Vec<usize>,
        frequency_tables: Vec<FrequencyTable>,
        known_values: Vec<KnownValue>,
    ) -> Self {
        for (attribute, &domain_size) in attribute_domains.iter().enumerate() {
            assert!(
                domain_size > 0,
                "attribute domain size at index {attribute} must be positive"
            );
        }

        let num_attributes = attribute_domains.len();
        let mut seen_pairs = BTreeSet::new();
        for table in &frequency_tables {
            let attribute_a = table.attribute_a();
            let attribute_b = table.attribute_b();
            assert!(
                attribute_a < num_attributes,
                "frequency table attribute_a {attribute_a} out of range for {num_attributes} attributes"
            );
            assert!(
                attribute_b < num_attributes,
                "frequency table attribute_b {attribute_b} out of range for {num_attributes} attributes"
            );
            assert!(
                attribute_a != attribute_b,
                "frequency table attributes must be distinct"
            );

            let pair = if attribute_a < attribute_b {
                (attribute_a, attribute_b)
            } else {
                (attribute_b, attribute_a)
            };
            assert!(
                seen_pairs.insert(pair),
                "duplicate frequency table pair ({}, {})",
                pair.0,
                pair.1
            );

            let expected_rows = attribute_domains[attribute_a];
            assert_eq!(
                table.counts().len(),
                expected_rows,
                "frequency table rows ({}) must equal attribute_domains[{attribute_a}] ({expected_rows})",
                table.counts().len()
            );

            let expected_cols = attribute_domains[attribute_b];
            for (row, row_counts) in table.counts().iter().enumerate() {
                assert_eq!(
                    row_counts.len(),
                    expected_cols,
                    "frequency table columns ({}) in row {row} must equal attribute_domains[{attribute_b}] ({expected_cols})",
                    row_counts.len()
                );
            }

            let total: usize = table.counts().iter().flatten().copied().sum();
            assert_eq!(
                total, num_objects,
                "frequency table total ({total}) must equal num_objects ({num_objects})"
            );
        }

        for known_value in &known_values {
            assert!(
                known_value.object() < num_objects,
                "known value object {} out of range for num_objects {}",
                known_value.object(),
                num_objects
            );
            assert!(
                known_value.attribute() < num_attributes,
                "known value attribute {} out of range for {num_attributes} attributes",
                known_value.attribute()
            );
            let domain_size = attribute_domains[known_value.attribute()];
            assert!(
                known_value.value() < domain_size,
                "known value value {} out of range for attribute {} with domain size {}",
                known_value.value(),
                known_value.attribute(),
                domain_size
            );
        }

        Self {
            num_objects,
            attribute_domains,
            frequency_tables,
            known_values,
        }
    }

    /// Returns the number of objects.
    pub fn num_objects(&self) -> usize {
        self.num_objects
    }

    /// Returns the number of attributes.
    pub fn num_attributes(&self) -> usize {
        self.attribute_domains.len()
    }

    /// Returns the attribute-domain sizes.
    pub fn attribute_domains(&self) -> &[usize] {
        &self.attribute_domains
    }

    /// Returns the published frequency tables.
    pub fn frequency_tables(&self) -> &[FrequencyTable] {
        &self.frequency_tables
    }

    /// Returns the known values.
    pub fn known_values(&self) -> &[KnownValue] {
        &self.known_values
    }

    /// Returns the product of attribute domain sizes.
    pub fn domain_size_product(&self) -> usize {
        self.attribute_domains.iter().copied().product()
    }

    /// Returns the number of object-attribute assignment variables in the direct encoding.
    pub fn num_assignment_variables(&self) -> usize {
        self.num_objects * self.num_attributes()
    }

    /// Returns the number of published frequency tables.
    pub fn num_frequency_tables(&self) -> usize {
        self.frequency_tables.len()
    }

    /// Returns the number of known value constraints.
    pub fn num_known_values(&self) -> usize {
        self.known_values.len()
    }

    /// Returns the number of one-hot assignment indicators used by the ILP reduction.
    pub fn num_assignment_indicators(&self) -> usize {
        self.num_objects * self.attribute_domains.iter().sum::<usize>()
    }

    /// Returns the total number of published frequency-table cells.
    pub fn num_frequency_cells(&self) -> usize {
        self.frequency_tables.iter().map(FrequencyTable::num_cells).sum()
    }

    /// Returns the number of auxiliary ILP indicators used for frequency-cell counting.
    pub fn num_auxiliary_frequency_indicators(&self) -> usize {
        self.num_objects * self.num_frequency_cells()
    }

    fn config_index(&self, object: usize, attribute: usize) -> usize {
        object * self.num_attributes() + attribute
    }
}

impl Problem for ConsistencyOfDatabaseFrequencyTables {
    const NAME: &'static str = "ConsistencyOfDatabaseFrequencyTables";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        let mut dims = Vec::with_capacity(self.num_assignment_variables());
        for _ in 0..self.num_objects {
            dims.extend(self.attribute_domains.iter().copied());
        }
        dims
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.num_assignment_variables() {
            return false;
        }

        for object in 0..self.num_objects {
            for (attribute, &domain_size) in self.attribute_domains.iter().enumerate() {
                if config[self.config_index(object, attribute)] >= domain_size {
                    return false;
                }
            }
        }

        for known_value in &self.known_values {
            if config[self.config_index(known_value.object(), known_value.attribute())]
                != known_value.value()
            {
                return false;
            }
        }

        for table in &self.frequency_tables {
            let rows = self.attribute_domains[table.attribute_a()];
            let cols = self.attribute_domains[table.attribute_b()];
            let mut observed = vec![vec![0usize; cols]; rows];

            for object in 0..self.num_objects {
                let value_a = config[self.config_index(object, table.attribute_a())];
                let value_b = config[self.config_index(object, table.attribute_b())];
                observed[value_a][value_b] += 1;
            }

            if observed != table.counts {
                return false;
            }
        }

        true
    }
}

impl SatisfactionProblem for ConsistencyOfDatabaseFrequencyTables {}

crate::declare_variants! {
    default sat ConsistencyOfDatabaseFrequencyTables => "domain_size_product^num_objects",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "consistency_of_database_frequency_tables",
        instance: Box::new(ConsistencyOfDatabaseFrequencyTables::new(
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
        )),
        optimal_config: vec![0, 0, 0, 0, 1, 1, 0, 2, 1, 1, 0, 1, 1, 1, 1, 1, 2, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/consistency_of_database_frequency_tables.rs"]
mod tests;
