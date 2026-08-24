//! Consistency of Database Frequency Tables problem implementation.
//!
//! Given a finite set of objects, categorical attributes with finite domains,
//! pairwise frequency tables for selected attribute pairs, and some known
//! object-attribute values, determine whether there exists a complete
//! assignment of attribute values to all objects that matches every published
//! frequency table and every known value.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrequencyTable {
    attribute_a: usize,
    attribute_b: usize,
    counts: Vec<Vec<i64>>,
}

impl FrequencyTable {
    /// Create a new pairwise frequency table.
    pub fn new(attribute_a: usize, attribute_b: usize, counts: Vec<Vec<i64>>) -> Self {
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
    pub fn counts(&self) -> &[Vec<i64>] {
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
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Determine whether pairwise frequency tables and known values admit a consistent complete database assignment",
        fields: ConsistencyOfDatabaseFrequencyTablesCreateSpec::FIELDS,
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

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct ConsistencyOfDatabaseFrequencyTablesCreateSpec {
    /// Number of database objects.
    num_objects: usize,
    /// Domain size for each attribute.
    #[create(codec = "comma-separated")]
    attribute_domains: Vec<usize>,
    /// Pairwise frequency tables as JSON objects.
    #[create(codec = "json")]
    frequency_tables: Vec<FrequencyTable>,
    /// Known object-attribute values as JSON objects; defaults to empty.
    #[create(codec = "json")]
    known_values: Option<Vec<KnownValue>>,
}

impl TryFrom<ConsistencyOfDatabaseFrequencyTablesCreateSpec>
    for ConsistencyOfDatabaseFrequencyTables
{
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: ConsistencyOfDatabaseFrequencyTablesCreateSpec) -> Result<Self, Self::Error> {
        let known_values = spec.known_values.unwrap_or_default();
        validate_cdft_create(
            spec.num_objects,
            &spec.attribute_domains,
            &spec.frequency_tables,
            &known_values,
        )?;
        Ok(Self {
            num_objects: spec.num_objects,
            attribute_domains: spec.attribute_domains,
            frequency_tables: spec.frequency_tables,
            known_values,
        })
    }
}

fn validate_cdft_create(
    num_objects: usize,
    domains: &[usize],
    tables: &[FrequencyTable],
    known: &[KnownValue],
) -> Result<(), crate::registry::ConstructionError> {
    for (attribute, &size) in domains.iter().enumerate() {
        if size == 0 {
            return Err(
                format!("attribute domain size at index {attribute} must be positive").into(),
            );
        }
    }
    let mut pairs = BTreeSet::new();
    for table in tables {
        let a = table.attribute_a();
        let b = table.attribute_b();
        if a >= domains.len() || b >= domains.len() {
            return Err("frequency table attribute is out of range".into());
        }
        if a == b {
            return Err("frequency table attributes must be distinct".into());
        }
        let pair = if a < b { (a, b) } else { (b, a) };
        if !pairs.insert(pair) {
            return Err(format!("duplicate frequency table pair ({}, {})", pair.0, pair.1).into());
        }
        if table.counts().len() != domains[a] {
            return Err(
                format!("frequency table rows must equal domain size for attribute {a}").into(),
            );
        }
        if table.counts().iter().any(|row| row.len() != domains[b]) {
            return Err(format!(
                "frequency table column count must equal domain size for attribute {b}"
            )
            .into());
        }
        if table.counts().iter().flatten().any(|&count| count < 0) {
            return Err("frequency table counts must be nonnegative".into());
        }
        let total = table
            .counts()
            .iter()
            .flatten()
            .try_fold(0_i64, |sum, &value| {
                sum.checked_add(value)
                    .ok_or("frequency table count total overflows i64")
            })?;
        let expected_total =
            i64::try_from(num_objects).map_err(|_| "num_objects cannot be represented as i64")?;
        if total != expected_total {
            return Err(format!(
                "frequency table total {total} must equal num_objects {num_objects}"
            )
            .into());
        }
    }
    for value in known {
        if value.object() >= num_objects {
            return Err("known value object is out of range".into());
        }
        if value.attribute() >= domains.len() {
            return Err("known value attribute is out of range".into());
        }
        if value.value() >= domains[value.attribute()] {
            return Err("known value value is outside the attribute domain".into());
        }
    }
    Ok(())
}

impl ConsistencyOfDatabaseFrequencyTables {
    /// Create a new consistency-of-database-frequency-tables instance.
    pub fn new(
        num_objects: usize,
        attribute_domains: Vec<usize>,
        frequency_tables: Vec<FrequencyTable>,
        known_values: Vec<KnownValue>,
    ) -> Self {
        validate_cdft_create(
            num_objects,
            &attribute_domains,
            &frequency_tables,
            &known_values,
        )
        .unwrap_or_else(|error| panic!("{error}"));

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
        self.frequency_tables
            .iter()
            .map(FrequencyTable::num_cells)
            .sum()
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
    type Value = crate::types::Or;

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

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.num_assignment_variables() {
                    return Ok(crate::types::Or(false));
                }

                for object in 0..self.num_objects {
                    for (attribute, &domain_size) in self.attribute_domains.iter().enumerate() {
                        if config[self.config_index(object, attribute)] >= domain_size {
                            return Ok(crate::types::Or(false));
                        }
                    }
                }

                for known_value in &self.known_values {
                    if config[self.config_index(known_value.object(), known_value.attribute())]
                        != known_value.value()
                    {
                        return Ok(crate::types::Or(false));
                    }
                }

                for table in &self.frequency_tables {
                    let rows = self.attribute_domains[table.attribute_a()];
                    let cols = self.attribute_domains[table.attribute_b()];
                    let mut observed = vec![vec![0_i64; cols]; rows];

                    for object in 0..self.num_objects {
                        let value_a = config[self.config_index(object, table.attribute_a())];
                        let value_b = config[self.config_index(object, table.attribute_b())];
                        observed[value_a][value_b] =
                            observed[value_a][value_b].checked_add(1).ok_or_else(|| {
                                crate::traits::EvaluationError::IntegerOverflow(
                                    "counting observed database frequencies".to_string(),
                                )
                            })?;
                    }

                    if observed != table.counts {
                        return Ok(crate::types::Or(false));
                    }
                }

                true
            })
        })
    }
}

crate::declare_variants! {
    default ConsistencyOfDatabaseFrequencyTables => "domain_size_product^num_objects" create ConsistencyOfDatabaseFrequencyTablesCreateSpec,
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
