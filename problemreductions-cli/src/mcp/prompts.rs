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
            Some(vec![PromptArgument {
                name: "problem".into(),
                title: None,
                description: Some("Problem name or alias (e.g., MIS, QUBO, MaxCut)".into()),
                required: Some(true),
            }]),
        ),
        Prompt::new(
            "model_my_problem",
            Some(
                "Map a real-world problem to the closest NP-hard problem type in the reduction \
                 graph",
            ),
            Some(vec![PromptArgument {
                name: "description".into(),
                title: None,
                description: Some(
                    "Free-text description of your real-world problem".into(),
                ),
                required: Some(true),
            }]),
        ),
        Prompt::new(
            "compare",
            Some(
                "Compare two problem types: their relationship, differences, and reduction path \
                 between them",
            ),
            Some(vec![
                PromptArgument {
                    name: "problem_a".into(),
                    title: None,
                    description: Some("First problem name or alias".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "problem_b".into(),
                    title: None,
                    description: Some("Second problem name or alias".into()),
                    required: Some(true),
                },
            ]),
        ),
        Prompt::new(
            "reduce",
            Some(
                "Step-by-step reduction walkthrough: create an instance, reduce it, solve it, \
                 and map the solution back",
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
            "solve",
            Some("Create and solve a problem instance, showing the optimal solution"),
            Some(vec![
                PromptArgument {
                    name: "problem_type".into(),
                    title: None,
                    description: Some("Problem name or alias (e.g., MIS, QUBO, MaxCut)".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "instance".into(),
                    title: None,
                    description: Some(
                        "Instance parameters (e.g., \"edges: 0-1,1-2\" or \"clauses: 1,2;-1,3\")"
                            .into(),
                    ),
                    required: Some(true),
                },
            ]),
        ),
        Prompt::new(
            "find_reduction",
            Some("Find the best reduction path between two problems, with cost analysis"),
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
            "overview",
            Some(
                "Explore the full landscape of NP-hard problems and reductions in the graph",
            ),
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
        "what_is" => {
            let problem = arguments
                .get("problem")
                .and_then(|v| v.as_str())
                .unwrap_or("MIS");

            Some(GetPromptResult {
                description: Some(format!("Explain the {} problem", problem)),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "Explain the \"{problem}\" problem to me.\n\n\
                         What does it model in the real world? What are its variants (graph types, \
                         weight types)? What other problems can it reduce to, and which problems \
                         reduce to it?\n\n\
                         Give me a concise summary suitable for someone encountering this problem \
                         for the first time, then show the technical details."
                    ),
                )],
            })
        }

        "model_my_problem" => {
            let description = arguments
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("(no description provided)");

            Some(GetPromptResult {
                description: Some("Map a real-world problem to an NP-hard problem type".into()),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "I have a real-world problem and I need help identifying which NP-hard \
                         problem type it maps to.\n\n\
                         Here's my problem: \"{description}\"\n\n\
                         Look through the available problem types in the reduction graph and \
                         identify which one(s) best model my problem. Explain why it's a good \
                         fit, what the variables and constraints map to, and suggest how I could \
                         encode my specific instance."
                    ),
                )],
            })
        }

        "compare" => {
            let problem_a = arguments
                .get("problem_a")
                .and_then(|v| v.as_str())
                .unwrap_or("MIS");
            let problem_b = arguments
                .get("problem_b")
                .and_then(|v| v.as_str())
                .unwrap_or("VertexCover");

            Some(GetPromptResult {
                description: Some(format!("Compare {} and {}", problem_a, problem_b)),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "Compare \"{problem_a}\" and \"{problem_b}\".\n\n\
                         How are they related? Is there a direct reduction between them, or do \
                         they connect through intermediate problems? What are the key differences \
                         in what they model? If one can be reduced to the other, what is the \
                         overhead?"
                    ),
                )],
            })
        }

        "reduce" => {
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
                    "Step-by-step reduction from {} to {}",
                    source, target
                )),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
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
                )],
            })
        }

        "solve" => {
            let problem_type = arguments
                .get("problem_type")
                .and_then(|v| v.as_str())
                .unwrap_or("MIS");
            let instance = arguments
                .get("instance")
                .and_then(|v| v.as_str())
                .unwrap_or("edges: 0-1,1-2,2-0");

            Some(GetPromptResult {
                description: Some(format!("Solve a {} instance", problem_type)),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "Create a {problem_type} instance with these parameters: {instance}\n\n\
                         Solve it and show me:\n\
                         - The problem instance details (size, structure)\n\
                         - The optimal solution and its objective value\n\
                         - Why this solution is optimal (briefly)"
                    ),
                )],
            })
        }

        "find_reduction" => {
            let source = arguments
                .get("source")
                .and_then(|v| v.as_str())
                .unwrap_or("SAT");
            let target = arguments
                .get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("QUBO");

            Some(GetPromptResult {
                description: Some(format!(
                    "Find reduction path from {} to {}",
                    source, target
                )),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "Find the best way to reduce \"{source}\" to \"{target}\".\n\n\
                         Show me the cheapest reduction path and explain the cost at each step. \
                         Are there alternative paths? If so, compare them — which is better for \
                         small instances vs. large instances?"
                    ),
                )],
            })
        }

        "overview" => Some(GetPromptResult {
            description: Some("Overview of the NP-hard problem reduction landscape".into()),
            messages: vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "Give me an overview of the NP-hard problem reduction landscape.\n\n\
                 How many problem types are registered? What are the major categories (graph, \
                 SAT, optimization)? Which problems are the most connected hubs? Which problems \
                 can reach the most targets through reductions?\n\n\
                 Summarize the structure so I understand what's available and where to start \
                 exploring."
                    .to_string(),
            )],
        }),

        _ => None,
    }
}
