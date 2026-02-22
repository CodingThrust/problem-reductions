use rmcp::model::{
    GetPromptResult, Prompt, PromptArgument, PromptMessage, PromptMessageRole,
};

/// Return the list of available MCP prompt templates.
pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt::new(
            "analyze_problem",
            Some("Analyze a problem type: show its definition, variants, size fields, and reductions"),
            Some(vec![PromptArgument {
                name: "problem_type".into(),
                title: None,
                description: Some("Problem name or alias (e.g., MIS, QUBO, MaxCut)".into()),
                required: Some(true),
            }]),
        ),
        Prompt::new(
            "reduction_walkthrough",
            Some(
                "End-to-end reduction walkthrough: find a path, create an instance, \
                 reduce it, and solve the result",
            ),
            Some(vec![
                PromptArgument {
                    name: "source".into(),
                    title: None,
                    description: Some("Source problem name or alias".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "target".into(),
                    title: None,
                    description: Some("Target problem name or alias".into()),
                    required: Some(true),
                },
            ]),
        ),
        Prompt::new(
            "explore_graph",
            Some("Explore the reduction graph: list all problems, export the graph, and analyze its structure"),
            None,
        ),
    ]
}

/// Return the content for the named prompt, or `None` if the name is unknown.
pub fn get_prompt(
    name: &str,
    arguments: &serde_json::Map<String, serde_json::Value>,
) -> Option<GetPromptResult> {
    match name {
        "analyze_problem" => {
            let problem_type = arguments
                .get("problem_type")
                .and_then(|v| v.as_str())
                .unwrap_or("MIS");

            Some(GetPromptResult {
                description: Some(format!("Analyze the {} problem type", problem_type)),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "I want to understand the \"{problem_type}\" problem type in the \
                         reduction graph.\n\n\
                         Please:\n\
                         1. Use the `show_problem` tool with \"{problem_type}\" to get its \
                            definition, variants, size fields, and reduction edges.\n\
                         2. Use the `neighbors` tool to find which problems it can reduce to \
                            (direction: out) and which problems reduce to it (direction: in).\n\
                         3. Summarize the problem: what it models, its variants, and its role in \
                            the reduction graph."
                    ),
                )],
            })
        }

        "reduction_walkthrough" => {
            let source = arguments
                .get("source")
                .and_then(|v| v.as_str())
                .unwrap_or("MIS");
            let target = arguments
                .get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("QUBO");

            Some(GetPromptResult {
                description: Some(format!(
                    "End-to-end reduction walkthrough from {} to {}",
                    source, target
                )),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "Walk me through an end-to-end reduction from \"{source}\" to \
                         \"{target}\".\n\n\
                         Please:\n\
                         1. Use `find_path` to find the cheapest reduction path from \
                            \"{source}\" to \"{target}\".\n\
                         2. Use `create_problem` to create a small example instance of \
                            \"{source}\".\n\
                         3. Use `reduce` to transform the instance to \"{target}\".\n\
                         4. Use `solve` to find the optimal solution of the reduced instance.\n\
                         5. Explain each step: how the reduction works, what the overhead is, \
                            and how the solution maps back."
                    ),
                )],
            })
        }

        "explore_graph" => Some(GetPromptResult {
            description: Some("Explore the reduction graph structure".into()),
            messages: vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "I want to explore the NP-hard problem reduction graph.\n\n\
                 Please:\n\
                 1. Use `list_problems` to get all registered problem types.\n\
                 2. Use `export_graph` to get the full reduction graph as JSON.\n\
                 3. Analyze the graph structure: how many problem types are there, how many \
                    reductions, which problems are the most connected hubs, and which problems \
                    can reach the most targets.\n\
                 4. Identify any interesting clusters or long reduction chains."
                    .to_string(),
            )],
        }),

        _ => None,
    }
}
