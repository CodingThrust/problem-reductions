use crate::export::RuleExample;

pub fn build_rule_examples() -> Vec<RuleExample> {
    crate::rules::canonical_rule_example_specs()
        .into_iter()
        .map(|spec| (spec.build)())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_all_42_canonical_rule_examples() {
        let examples = build_rule_examples();

        assert_eq!(examples.len(), 42);
        assert!(examples
            .iter()
            .all(|example| !example.source.problem.is_empty()));
        assert!(examples
            .iter()
            .all(|example| !example.target.problem.is_empty()));
        assert!(examples
            .iter()
            .all(|example| example.source.instance.is_object()));
        assert!(examples
            .iter()
            .all(|example| example.target.instance.is_object()));
    }

    #[test]
    fn satisfiability_to_kcoloring_uses_full_problem_serialization() {
        let specs = crate::rules::canonical_rule_example_specs();
        let spec = specs.iter().find(|s| s.id == "satisfiability_to_kcoloring").unwrap();
        let example = (spec.build)();

        assert_eq!(example.source.problem, "Satisfiability");
        assert_eq!(example.target.problem, "KColoring");
        assert!(example.source.instance.get("num_vars").is_some());
        assert!(example.target.instance.get("graph").is_some());
    }

    #[test]
    fn factoring_to_circuitsat_contains_complete_solution_pairs() {
        let specs = crate::rules::canonical_rule_example_specs();
        let spec = specs.iter().find(|s| s.id == "factoring_to_circuitsat").unwrap();
        let example = (spec.build)();

        assert!(!example.solutions.is_empty());
        assert!(example
            .solutions
            .iter()
            .all(|pair| !pair.source_config.is_empty() && !pair.target_config.is_empty()));
    }
}
