use crate::export::RuleExample;

pub fn build_rule_examples() -> Vec<RuleExample> {
    crate::rules::canonical_rule_example_specs()
        .into_iter()
        .map(|spec| (spec.build)())
        .collect()
}
