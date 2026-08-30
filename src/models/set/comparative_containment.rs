//! Comparative Containment problem implementation.
//!
//! Given two weighted families of sets over a common universe, determine
//! whether there exists a subset of the universe whose containment weight
//! in the first family is at least its containment weight in the second.

use crate::registry::{ConstructionError, CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::traits::Problem;
use crate::types::{One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "ComparativeContainment",
        display_name: "Comparative Containment",
        aliases: &[],
        dimensions: &[VariantDimension::new("weight", "i64", &["One", "i64", "f64"])],
        category: crate::registry::ProblemCategory::Set,
        module_path: module_path!(),
        description: "Compare containment-weight sums for two set families over a shared universe",
        fields: ComparativeContainmentI64CreateSpec::FIELDS,
    }
}

/// Comparative Containment.
///
/// Given a universe `X`, two set families `R` and `S`, and positive weights
/// on those sets, determine whether there exists a subset `Y ⊆ X` such that
/// the total weight of `R`-sets containing `Y` is at least the total weight
/// of `S`-sets containing `Y`.
#[derive(Debug, Clone, Serialize)]
pub struct ComparativeContainment<W = i64> {
    universe_size: usize,
    r_sets: Vec<Vec<usize>>,
    s_sets: Vec<Vec<usize>>,
    r_weights: Vec<W>,
    s_weights: Vec<W>,
}

macro_rules! comparative_containment_create_spec {
    ($name:ident, $weight:ty, $one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            /// Size of the common universe.
            universe_size: usize,
            /// First set family.
            #[create(codec = "semicolon-separated")]
            r_sets: Vec<Vec<usize>>,
            /// Second set family.
            #[create(codec = "semicolon-separated")]
            s_sets: Vec<Vec<usize>>,
            /// Positive weights for the first family; defaults to one.
            #[create(codec = "comma-separated")]
            r_weights: Option<Vec<$weight>>,
            /// Positive weights for the second family; defaults to one.
            #[create(codec = "comma-separated")]
            s_weights: Option<Vec<$weight>>,
        }

        impl TryFrom<$name> for ComparativeContainment<$weight> {
            type Error = ConstructionError;
            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                let r_weights = spec
                    .r_weights
                    .unwrap_or_else(|| vec![$one; spec.r_sets.len()]);
                let s_weights = spec
                    .s_weights
                    .unwrap_or_else(|| vec![$one; spec.s_sets.len()]);
                ComparativeContainment::with_weights(
                    spec.universe_size,
                    spec.r_sets,
                    spec.s_sets,
                    r_weights,
                    s_weights,
                )
            }
        }
    };
}

fn validate_create_set_family(
    label: &str,
    universe_size: usize,
    sets: &[Vec<usize>],
) -> Result<(), ConstructionError> {
    for (set_index, set) in sets.iter().enumerate() {
        for &element in set {
            if element >= universe_size {
                return Err(ConstructionError::Conversion(format!("{label} set {set_index} contains element {element} outside universe of size {universe_size}")));
            }
        }
    }
    Ok(())
}

fn validate_create_weights<W: WeightElement>(
    label: &str,
    count: usize,
    weights: &[W],
) -> Result<(), ConstructionError> {
    if weights.len() != count {
        return Err(ConstructionError::Conversion(format!(
            "number of {label} sets and weights must match"
        )));
    }
    for (index, weight) in weights.iter().enumerate() {
        match weight.to_sum().partial_cmp(&W::Sum::zero()) {
            None => {
                return Err(ConstructionError::NonFiniteFloat(format!(
                    "{label} weight at index {index} must be finite"
                )));
            }
            Some(std::cmp::Ordering::Greater) => {}
            Some(_) => {
                return Err(ConstructionError::Conversion(format!(
                    "{label} weight at index {index} must be positive"
                )));
            }
        }
    }
    Ok(())
}

comparative_containment_create_spec!(ComparativeContainmentI64CreateSpec, i64, 1_i64);
comparative_containment_create_spec!(ComparativeContainmentF64CreateSpec, f64, 1.0_f64);
comparative_containment_create_spec!(ComparativeContainmentOneCreateSpec, One, One);

impl<W: WeightElement> ComparativeContainment<W> {
    /// Create a new instance with unit weights.
    pub fn new(
        universe_size: usize,
        r_sets: Vec<Vec<usize>>,
        s_sets: Vec<Vec<usize>>,
    ) -> Result<Self, ConstructionError>
    where
        W: WeightElement,
    {
        let r_weights = vec![W::unit(); r_sets.len()];
        let s_weights = vec![W::unit(); s_sets.len()];
        Self::with_weights(universe_size, r_sets, s_sets, r_weights, s_weights)
    }

    /// Create a new instance with explicit weights.
    pub fn with_weights(
        universe_size: usize,
        r_sets: Vec<Vec<usize>>,
        s_sets: Vec<Vec<usize>>,
        r_weights: Vec<W>,
        s_weights: Vec<W>,
    ) -> Result<Self, ConstructionError> {
        validate_create_set_family("R", universe_size, &r_sets)?;
        validate_create_set_family("S", universe_size, &s_sets)?;
        validate_create_weights("R", r_sets.len(), &r_weights)?;
        validate_create_weights("S", s_sets.len(), &s_weights)?;
        Ok(Self {
            universe_size,
            r_sets,
            s_sets,
            r_weights,
            s_weights,
        })
    }

    /// Get the size of the universe.
    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    /// Get the number of sets in the R family.
    pub fn num_r_sets(&self) -> usize {
        self.r_sets.len()
    }

    /// Get the number of sets in the S family.
    pub fn num_s_sets(&self) -> usize {
        self.s_sets.len()
    }

    /// Get the R family.
    pub fn r_sets(&self) -> &[Vec<usize>] {
        &self.r_sets
    }

    /// Get the S family.
    pub fn s_sets(&self) -> &[Vec<usize>] {
        &self.s_sets
    }

    /// Get the R-family weights.
    pub fn r_weights(&self) -> &[W] {
        &self.r_weights
    }

    /// Get the S-family weights.
    pub fn s_weights(&self) -> &[W] {
        &self.s_weights
    }

    /// Check whether the subset selected by `config` is contained in `set`.
    pub fn contains_selected_subset(&self, config: &[bool], set: &[usize]) -> bool {
        self.valid_config(config) && contains_selected_subset_unchecked(config, set)
    }

    fn valid_config(&self, config: &[bool]) -> bool {
        config.len() == self.universe_size
    }
}

impl<'de, W> Deserialize<'de> for ComparativeContainment<W>
where
    W: WeightElement + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw<W> {
            universe_size: usize,
            r_sets: Vec<Vec<usize>>,
            s_sets: Vec<Vec<usize>>,
            r_weights: Vec<W>,
            s_weights: Vec<W>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::with_weights(
            raw.universe_size,
            raw.r_sets,
            raw.s_sets,
            raw.r_weights,
            raw.s_weights,
        )
        .map_err(serde::de::Error::custom)
    }
}

impl<W> ComparativeContainment<W>
where
    W: WeightElement,
{
    /// Total R-family weight for sets containing the selected subset.
    pub fn r_weight_sum(
        &self,
        config: &[bool],
    ) -> Result<Option<W::Sum>, crate::traits::EvaluationError> {
        self.sum_containing_weights(config, &self.r_sets, &self.r_weights)
    }

    /// Total S-family weight for sets containing the selected subset.
    pub fn s_weight_sum(
        &self,
        config: &[bool],
    ) -> Result<Option<W::Sum>, crate::traits::EvaluationError> {
        self.sum_containing_weights(config, &self.s_sets, &self.s_weights)
    }

    /// Check if a configuration is a satisfying solution.
    pub fn is_valid_solution(
        &self,
        config: &[bool],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(
            match (self.r_weight_sum(config)?, self.s_weight_sum(config)?) {
                (Some(r_total), Some(s_total)) => r_total >= s_total,
                _ => false,
            },
        )
    }

    fn sum_containing_weights(
        &self,
        config: &[bool],
        sets: &[Vec<usize>],
        weights: &[W],
    ) -> Result<Option<W::Sum>, crate::traits::EvaluationError> {
        if !self.valid_config(config) {
            return Ok(None);
        }

        let mut total = W::Sum::zero();
        for (set, weight) in sets.iter().zip(weights.iter()) {
            if contains_selected_subset_unchecked(config, set) {
                total = W::checked_add_to_sum(
                    total,
                    weight.to_sum(),
                    "summing comparative containment weights",
                )?;
            }
        }
        Ok(Some(total))
    }
}

impl<W> Problem for ComparativeContainment<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "ComparativeContainment";
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_parameters![
        ("num_r_sets", num_r_sets),
        ("num_s_sets", num_s_sets),
        ("universe_size", universe_size),
    ];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        if config.len() != self.universe_size {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "element-selection length does not match the universe".into(),
            ));
        }
        Ok(crate::types::Or(self.is_valid_solution(config)?))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![W]
    }
}

impl<W> crate::solvers::BruteForceProblem for ComparativeContainment<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.universe_size]
    }
}

crate::declare_variants! {
    ComparativeContainment<One> => "2^universe_size" create ComparativeContainmentOneCreateSpec,
    default ComparativeContainment<i64> => "2^universe_size" create ComparativeContainmentI64CreateSpec,
    ComparativeContainment<f64> => "2^universe_size" create ComparativeContainmentF64CreateSpec,
}

crate::register_brute_force! {
    ComparativeContainment<One> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    ComparativeContainment<i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
    ComparativeContainment<f64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

fn contains_selected_subset_unchecked(config: &[bool], set: &[usize]) -> bool {
    config
        .iter()
        .enumerate()
        .all(|(element, &selected)| !selected || set.contains(&element))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "comparative_containment",
        instance: Box::new(
            ComparativeContainment::with_weights(
                4,
                vec![vec![0, 1, 2, 3], vec![0, 1]],
                vec![vec![0, 1, 2, 3], vec![2, 3]],
                vec![2, 5],
                vec![3, 6],
            )
            .expect("canonical comparative-containment instance must be valid"),
        ),
        optimal_config: serde_json::json!(vec![false, true, false, false]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/comparative_containment.rs"]
mod tests;
