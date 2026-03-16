use rmcp::model::{GetPromptResult, Prompt, PromptArgument, PromptMessage, PromptMessageRole};

/// Return the list of available MCP prompt templates.
pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt::new(
            "what_is",
            Some(
                "Explain a problem type: what it models, its variants, and how it connects to \
                 other problems",
            ),
            Some(vec![PromptArgument::new("problem")
                .with_description("Problem name or alias (e.g., MIS, QUBO, MaxCut)")
                .with_required(true)]),
        ),
        Prompt::new(
            "model_my_problem",
            Some(
                "Map a real-world problem to the closest NP-hard problem type in the reduction \
                 graph",
            ),
            Some(vec![PromptArgument::new("description")
                .with_description("Free-text description of your real-world problem")
                .with_required(true)]),
        ),
        Prompt::new(
            "compare",
            Some(
                "Compare two problem types: their relationship, differences, and reduction path \
                 between them",
            ),
            Some(vec![
                PromptArgument::new("problem_a")
                    .with_description("First problem name or alias")
                    .with_required(true),
                PromptArgument::new("problem_b")
                    .with_description("Second problem name or alias")
                    .with_required(true),
            ]),
        ),
        Prompt::new(
            "reduce",
            Some(
                "Step-by-step reduction walkthrough: create an instance, reduce it, solve it, \
                 and map the solution back",
            ),
            Some(vec![
                PromptArgument::new("source")
                    .with_description("Source problem name or alias")
                    .with_required(true),
                PromptArgument::new("target")
                    .with_description("Target problem name or alias")
                    .with_required(true),
            ]),
        ),
        Prompt::new(
            "solve",
            Some("Create and solve a problem instance, showing the optimal solution"),
            Some(vec![
                PromptArgument::new("problem_type")
                    .with_description("Problem name or alias (e.g., MIS, QUBO, MaxCut)")
                    .with_required(true),
                PromptArgument::new("instance")
                    .with_description(
                        "Instance parameters (e.g., \"edges: 0-1,1-2\" or \"clauses: 1,2;-1,3\")",
                    )
                    .with_required(true),
            ]),
        ),
        Prompt::new(
            "find_reduction",
            Some("Find the best reduction path between two problems, with cost analysis"),
            Some(vec![
                PromptArgument::new("source")
                    .with_description("Source problem name or alias")
                    .with_required(true),
                PromptArgument::new("target")
                    .with_description("Target problem name or alias")
                    .with_required(true),
            ]),
        ),
        Prompt::new(
            "overview",
            Some("Explore the full landscape of NP-hard problems and reductions in the graph"),
            None,
        ),
    ]
}

fn prompt_result(description: &str, user_message: &str) -> GetPromptResult {
    GetPromptResult::new(vec![PromptMessage::new_text(
        PromptMessageRole::User,
        user_message,
    )])
    .with_description(description)
}

/// Return the content for the named prompt, or `None` if the name is unknown.
pub fn get_prompt(
    name: &str,
    arguments: &serde_json::Map<String, serde_json::Value>,
) -> Option<GetPromptResult> {
    let get = |key: &str, default: &str| -> String {
        arguments
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .to_string()
    };

    match name {
        "what_is" => {
            let problem = get("problem", "MIS");
            Some(prompt_result(
                &format!("Explain the {problem} problem"),
                &format!(
                    "Explain the \"{problem}\" problem to me.\n\n\
                     What does it model in the real world? What are its variants (graph types, \
                     weight types)? What other problems can it reduce to, and which problems \
                     reduce to it?\n\n\
                     Give me a concise summary suitable for someone encountering this problem \
                     for the first time, then show the technical details."
                ),
            ))
        }

        "model_my_problem" => {
            let description = get("description", "(no description provided)");
            Some(prompt_result(
                "Map a real-world problem to an NP-hard problem type",
                &format!(
                    "I have a real-world problem and I need help identifying which NP-hard \
                     problem type it maps to.\n\n\
                     Here's my problem: \"{description}\"\n\n\
                     Look through the available problem types in the reduction graph and \
                     identify which one(s) best model my problem. Explain why it's a good \
                     fit, what the variables and constraints map to, and suggest how I could \
                     encode my specific instance."
                ),
            ))
        }

        "compare" => {
            let a = get("problem_a", "MIS");
            let b = get("problem_b", "VertexCover");
            Some(prompt_result(
                &format!("Compare {a} and {b}"),
                &format!(
                    "Compare \"{a}\" and \"{b}\".\n\n\
                     How are they related? Is there a direct reduction between them, or do \
                     they connect through intermediate problems? What are the key differences \
                     in what they model? If one can be reduced to the other, what is the \
                     overhead?"
                ),
            ))
        }

        "reduce" => {
            let source = get("source", "MIS");
            let target = get("target", "QUBO");
            Some(prompt_result(
                &format!("Step-by-step reduction from {source} to {target}"),
                &format!(
                    "Walk me through reducing a \"{source}\" instance to \"{target}\", step \
                     by step.\n\n\
                     1. Find the reduction path and explain the overhead.\n\
                     2. Create a small, concrete example instance of \"{source}\".\n\
                     3. Reduce it to \"{target}\" and show what the transformed instance \
                        looks like.\n\
                     4. Solve the reduced instance.\n\
                     5. Explain how the solution maps back to the original problem.\n\n\
                     Use a small example so I can follow each transformation by hand."
                ),
            ))
        }

        "solve" => {
            let problem_type = get("problem_type", "MIS");
            let instance = get("instance", "edges: 0-1,1-2,2-0");
            Some(prompt_result(
                &format!("Solve a {problem_type} instance"),
                &format!(
                    "Create a {problem_type} instance with these parameters: {instance}\n\n\
                     Solve it and show me:\n\
                     - The problem instance details (size, structure)\n\
                     - The optimal solution and its objective value\n\
                     - Why this solution is optimal (briefly)"
                ),
            ))
        }

        "find_reduction" => {
            let source = get("source", "SAT");
            let target = get("target", "QUBO");
            Some(prompt_result(
                &format!("Find reduction path from {source} to {target}"),
                &format!(
                    "Find the best way to reduce \"{source}\" to \"{target}\".\n\n\
                     Show me the cheapest reduction path and explain the cost at each step. \
                     Are there alternative paths? If so, compare them — which is better for \
                     small instances vs. large instances?"
                ),
            ))
        }

        "overview" => Some(prompt_result(
            "Overview of the NP-hard problem reduction landscape",
            "Give me an overview of the NP-hard problem reduction landscape.\n\n\
             How many problem types are registered? What are the major categories (graph, \
             SAT, optimization)? Which problems are the most connected hubs? Which problems \
             can reach the most targets through reductions?\n\n\
             Summarize the structure so I understand what's available and where to start \
             exploring.",
        )),

        _ => None,
    }
}
