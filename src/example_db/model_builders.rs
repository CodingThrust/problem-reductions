use crate::export::ModelExample;

pub fn build_model_examples() -> Vec<ModelExample> {
    crate::models::graph::canonical_model_example_specs()
        .into_iter()
        .chain(crate::models::formula::canonical_model_example_specs())
        .chain(crate::models::set::canonical_model_example_specs())
        .chain(crate::models::algebraic::canonical_model_example_specs())
        .chain(crate::models::misc::canonical_model_example_specs())
        .chain(decision_model_examples())
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

fn decision_model_examples() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::example_db::specs::ModelExampleSpec;
    use crate::models::decision::Decision;
    use crate::models::graph::{LongestCircuit, MinimumVertexCover};
    use crate::models::misc::OpenShopScheduling;
    use crate::topology::SimpleGraph;
    use crate::types::One;

    vec![
        ModelExampleSpec {
            id: "decision_open_shop_scheduling",
            instance: Box::new(Decision::new(
                OpenShopScheduling::new(2, vec![vec![1, 1]]),
                2,
            )),
            optimal_config: serde_json::json!([0, 1]),
            optimal_value: serde_json::json!(true),
        },
        ModelExampleSpec {
            id: "decision_longest_circuit",
            instance: Box::new(Decision::new(
                LongestCircuit::new(SimpleGraph::cycle(3), vec![1_i64; 3]),
                3,
            )),
            optimal_config: serde_json::json!([true, true, true]),
            optimal_value: serde_json::json!(true),
        },
        ModelExampleSpec {
            id: "decision_minimum_vertex_cover_one",
            instance: Box::new(Decision::new(
                MinimumVertexCover::new(SimpleGraph::path(3), vec![One; 3]),
                1,
            )),
            optimal_config: serde_json::json!([false, true, false]),
            optimal_value: serde_json::json!(true),
        },
    ]
}
