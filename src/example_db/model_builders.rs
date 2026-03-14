use crate::export::ModelExample;

pub fn build_model_examples() -> Vec<ModelExample> {
    crate::models::graph::canonical_model_example_specs()
        .into_iter()
        .chain(crate::models::formula::canonical_model_example_specs())
        .chain(crate::models::set::canonical_model_example_specs())
        .chain(crate::models::algebraic::canonical_model_example_specs())
        .chain(crate::models::misc::canonical_model_example_specs())
        .map(|spec| (spec.build)())
        .collect()
}
