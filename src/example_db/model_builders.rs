use crate::export::ModelExample;

pub fn build_model_examples() -> Vec<ModelExample> {
    crate::models::graph::canonical_model_example_specs()
        .into_iter()
        .chain(crate::models::formula::canonical_model_example_specs())
        .chain(crate::models::set::canonical_model_example_specs())
        .chain(crate::models::algebraic::canonical_model_example_specs())
        .chain(crate::models::misc::canonical_model_example_specs())
        .map(|spec| {
            let problem_name = spec.instance.problem_name().to_string();
            let variant = spec.instance.variant_map();
            let instance_json = spec.instance.serialize_json();
            ModelExample::new(
                &problem_name,
                variant,
                instance_json,
                spec.optimal_config,
                spec.optimal_value,
            )
        })
        .collect()
}
