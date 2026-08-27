//! Prime Attribute Name problem implementation.
//!
//! Given a set of attributes A, a collection of functional dependencies F on A,
//! and a query attribute x, determine if x belongs to any candidate key of <A, F>.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "PrimeAttributeName",
        display_name: "Prime Attribute Name",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Set,
        module_path: module_path!(),
        description: "Determine if an attribute belongs to any candidate key under functional dependencies",
        fields: PrimeAttributeNameCreateSpec::FIELDS,
    }
}

/// Prime Attribute Name decision problem.
///
/// Given a set A = {0, 1, ..., n-1} of attribute names, a collection F of
/// functional dependencies on A, and a specified attribute x in A, determine
/// whether x is a *prime attribute* -- i.e., whether there exists a candidate
/// key K for <A, F> such that x is in K.
///
/// A *candidate key* is a minimal set K of attributes whose closure under F
/// equals A. An attribute is *prime* if it belongs to at least one candidate key.
///
/// This is a classical NP-complete problem from relational database theory
/// (Garey & Johnson SR28, Lucchesi & Osborne 1978).
///
/// # Example
///
/// ```
/// use problemreductions::models::set::PrimeAttributeName;
/// use problemreductions::{Problem, BruteForce};
///
/// // 6 attributes, FDs: {0,1}->rest, {2,3}->rest, {0,3}->rest
/// let problem = PrimeAttributeName::new(
///     6,
///     vec![
///         (vec![0, 1], vec![2, 3, 4, 5]),
///         (vec![2, 3], vec![0, 1, 4, 5]),
///         (vec![0, 3], vec![1, 2, 4, 5]),
///     ],
///     3,
/// );
///
/// // {2, 3} is a candidate key containing attribute 3
/// assert!(problem
///     .evaluate(&vec![false, false, true, true, false, false])
///     .unwrap());
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeAttributeName {
    /// Number of attributes (elements are 0..num_attributes).
    num_attributes: usize,
    /// Functional dependencies as (lhs, rhs) pairs.
    dependencies: Vec<(Vec<usize>, Vec<usize>)>,
    /// The query attribute index.
    query_attribute: usize,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct PrimeAttributeNameCreateSpec {
    /// Number of attributes.
    universe_size: usize,
    /// Functional dependencies (lhs, rhs) pairs.
    dependencies: Vec<(Vec<usize>, Vec<usize>)>,
    /// The query attribute index.
    query_attribute: usize,
}

impl TryFrom<PrimeAttributeNameCreateSpec> for PrimeAttributeName {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: PrimeAttributeNameCreateSpec) -> Result<Self, Self::Error> {
        if spec.query_attribute >= spec.universe_size {
            return Err(format!(
                "query_attribute {} is outside universe of size {}",
                spec.query_attribute, spec.universe_size
            )
            .into());
        }
        for (dependency_index, (lhs, rhs)) in spec.dependencies.iter().enumerate() {
            if lhs.is_empty() {
                return Err(
                    format!("dependencies[{dependency_index}] has an empty left side").into(),
                );
            }
            if let Some(&attribute) = lhs
                .iter()
                .chain(rhs)
                .find(|&&attribute| attribute >= spec.universe_size)
            {
                return Err(format!(
                    "dependencies[{dependency_index}] contains attribute {attribute} outside universe of size {}",
                    spec.universe_size
                ).into());
            }
        }
        Ok(Self::new(
            spec.universe_size,
            spec.dependencies,
            spec.query_attribute,
        ))
    }
}

impl PrimeAttributeName {
    /// Create a new Prime Attribute Name problem.
    ///
    /// # Panics
    ///
    /// Panics if `query_attribute >= num_attributes`, if any attribute index
    /// in a dependency is out of range, or if any LHS is empty.
    pub fn new(
        num_attributes: usize,
        dependencies: Vec<(Vec<usize>, Vec<usize>)>,
        query_attribute: usize,
    ) -> Self {
        assert!(
            query_attribute < num_attributes,
            "Query attribute {} is outside attribute set of size {}",
            query_attribute,
            num_attributes
        );
        for (i, (lhs, rhs)) in dependencies.iter().enumerate() {
            assert!(!lhs.is_empty(), "Dependency {} has empty LHS", i);
            for &attr in lhs.iter().chain(rhs.iter()) {
                assert!(
                    attr < num_attributes,
                    "Dependency {} references attribute {} which is outside attribute set of size {}",
                    i,
                    attr,
                    num_attributes
                );
            }
        }
        Self {
            num_attributes,
            dependencies,
            query_attribute,
        }
    }

    /// Get the number of attributes.
    pub fn num_attributes(&self) -> usize {
        self.num_attributes
    }

    /// Get the number of functional dependencies.
    pub fn num_dependencies(&self) -> usize {
        self.dependencies.len()
    }

    /// Get the query attribute index.
    pub fn query_attribute(&self) -> usize {
        self.query_attribute
    }

    /// Get the functional dependencies.
    pub fn dependencies(&self) -> &[(Vec<usize>, Vec<usize>)] {
        &self.dependencies
    }

    /// Compute the attribute closure of a set under the functional dependencies.
    ///
    /// Starting from the given boolean mask of attributes, repeatedly applies
    /// all functional dependencies until a fixpoint is reached.
    pub fn compute_closure(&self, attrs: &[bool]) -> Vec<bool> {
        let mut closure = attrs.to_vec();
        loop {
            let mut changed = false;
            for (lhs, rhs) in &self.dependencies {
                if lhs.iter().all(|&a| closure[a]) {
                    for &a in rhs {
                        if !closure[a] {
                            closure[a] = true;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        closure
    }
}

impl Problem for PrimeAttributeName {
    const NAME: &'static str = "PrimeAttributeName";
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_size![
        ("num_attributes", num_attributes),
        ("num_dependencies", num_dependencies),
    ];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                if config.len() != self.num_attributes {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "attribute-selection length does not match the relation".into(),
                    ));
                }

                // query_attribute must be in K
                if !config[self.query_attribute] {
                    return Ok(crate::types::Or(false));
                }

                // Compute closure(K) -- must equal all attributes (K is a superkey)
                let closure = self.compute_closure(config);
                if closure.iter().any(|&v| !v) {
                    return Ok(crate::types::Or(false));
                }

                // Check minimality: removing any attribute from K must break the superkey property
                for i in 0..self.num_attributes {
                    if config[i] {
                        let mut reduced = config.clone();
                        reduced[i] = false;
                        let reduced_closure = self.compute_closure(&reduced);
                        if reduced_closure.iter().all(|&v| v) {
                            // K \ {i} is still a superkey, so K is not minimal
                            return Ok(crate::types::Or(false));
                        }
                    }
                }

                true
            })
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for PrimeAttributeName {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_attributes]
    }
}

crate::declare_variants! {
    default PrimeAttributeName => "2^num_attributes * num_dependencies * num_attributes" create PrimeAttributeNameCreateSpec,
}

crate::register_brute_force! {
    PrimeAttributeName decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "prime_attribute_name",
        // Issue Example 1: 6 attributes, 3 FDs, query=3 -> YES
        instance: Box::new(PrimeAttributeName::new(
            6,
            vec![
                (vec![0, 1], vec![2, 3, 4, 5]),
                (vec![2, 3], vec![0, 1, 4, 5]),
                (vec![0, 3], vec![1, 2, 4, 5]),
            ],
            3,
        )),
        // {2, 3} is a candidate key containing attribute 3
        optimal_config: serde_json::json!(vec![false, false, true, true, false, false]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/prime_attribute_name.rs"]
mod tests;
