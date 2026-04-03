use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegerExpressionMembership",
        display_name: "Integer Expression Membership",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Pick one value from each choice set so the total equals a target value",
        fields: &[
            FieldInfo { name: "choices", type_name: "Vec<Vec<u64>>", description: "Choice set for each position" },
            FieldInfo { name: "target", type_name: "u64", description: "Target total value" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "IntegerExpressionMembership",
        fields: &["num_positions"],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerExpressionMembership {
    choices: Vec<Vec<u64>>,
    target: u64,
}

impl IntegerExpressionMembership {
    pub fn new(choices: Vec<Vec<u64>>, target: u64) -> Self {
        assert!(
            choices.iter().all(|choice_set| !choice_set.is_empty()),
            "Each choice set must contain at least one value"
        );
        Self { choices, target }
    }

    pub fn choices(&self) -> &[Vec<u64>] {
        &self.choices
    }

    pub fn target(&self) -> u64 {
        self.target
    }

    pub fn num_positions(&self) -> usize {
        self.choices.len()
    }
}

impl Problem for IntegerExpressionMembership {
    const NAME: &'static str = "IntegerExpressionMembership";
    type Value = Or;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        self.choices
            .iter()
            .map(|choice_set| choice_set.len())
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        if config.len() != self.num_positions() {
            return Or(false);
        }

        let sum = config
            .iter()
            .enumerate()
            .try_fold(0u64, |sum, (position, &choice_index)| {
                let value = *self.choices[position].get(choice_index)?;
                sum.checked_add(value)
            });

        Or(matches!(sum, Some(total) if total == self.target))
    }
}

crate::declare_variants! {
    default IntegerExpressionMembership => "2^num_positions",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "integer_expression_membership",
        instance: Box::new(IntegerExpressionMembership::new(
            vec![vec![1, 2], vec![1, 6], vec![1, 7], vec![1, 9]],
            15,
        )),
        optimal_config: vec![0, 1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/algebraic/integer_expression_membership.rs"]
mod tests;
