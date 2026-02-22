# MCP Prompts Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace 3 tool-centric MCP prompts with 7 task-oriented prompts that map to real user journeys.

**Architecture:** Rewrite `prompts.rs` with new `list_prompts()` and `get_prompt()` functions. Update integration test to match new prompt set. No tool or server changes.

**Tech Stack:** Rust, rmcp crate (MCP SDK), serde_json

---

### Task 1: Rewrite `list_prompts()` with 7 new prompt definitions

**Files:**
- Modify: `problemreductions-cli/src/mcp/prompts.rs:4-43`

**Step 1: Replace `list_prompts()` body**

Replace the entire `list_prompts()` function body with 7 new `Prompt::new(...)` entries:

```rust
pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt::new(
            "what_is",
            Some("Explain a problem type: what it models, its variants, and how it connects to other problems"),
            Some(vec![PromptArgument {
                name: "problem".into(),
                title: None,
                description: Some("Problem name or alias (e.g., MIS, QUBO, MaxCut)".into()),
                required: Some(true),
            }]),
        ),
        Prompt::new(
            "model_my_problem",
            Some("Map a real-world problem to the closest NP-hard problem type in the reduction graph"),
            Some(vec![PromptArgument {
                name: "description".into(),
                title: None,
                description: Some("Free-text description of your real-world problem".into()),
                required: Some(true),
            }]),
        ),
        Prompt::new(
            "compare",
            Some("Compare two problem types: their relationship, differences, and reduction path between them"),
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
            Some("Step-by-step reduction walkthrough: create an instance, reduce it, solve it, and map the solution back"),
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
                    description: Some("Problem type (e.g., MIS, SAT, QUBO, MaxCut)".into()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "instance".into(),
                    title: None,
                    description: Some(
                        "Instance parameters (e.g., \"edges: 0-1,1-2\" or \"clauses: 1,2;-1,3\")".into(),
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
            Some("Explore the full landscape of NP-hard problems and reductions in the graph"),
            None,
        ),
    ]
}
```

**Step 2: Run `cargo check` to verify compilation**

Run: `cargo check -p problemreductions-cli`
Expected: compiles without errors

**Step 3: Commit**

```bash
git add problemreductions-cli/src/mcp/prompts.rs
git commit -m "refactor(mcp): replace prompt definitions with 7 task-oriented prompts"
```

---

### Task 2: Rewrite `get_prompt()` with new prompt texts

**Files:**
- Modify: `problemreductions-cli/src/mcp/prompts.rs:46-128`

**Step 1: Replace `get_prompt()` body**

Replace the entire `get_prompt()` match block with 7 new arms:

```rust
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
                         What does it model in the real world? What are its variants \
                         (graph types, weight types)? What other problems can it reduce \
                         to, and which problems reduce to it?\n\n\
                         Give me a concise summary suitable for someone encountering this \
                         problem for the first time, then show the technical details."
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
                        "I have a real-world problem and I need help identifying which \
                         NP-hard problem type it maps to.\n\n\
                         Here's my problem: \"{description}\"\n\n\
                         Look through the available problem types in the reduction graph \
                         and identify which one(s) best model my problem. Explain why it's \
                         a good fit, what the variables and constraints map to, and suggest \
                         how I could encode my specific instance."
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
                         How are they related? Is there a direct reduction between them, \
                         or do they connect through intermediate problems? What are the \
                         key differences in what they model? If one can be reduced to the \
                         other, what is the overhead?"
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
                    "Reduction walkthrough from {} to {}",
                    source, target
                )),
                messages: vec![PromptMessage::new_text(
                    PromptMessageRole::User,
                    format!(
                        "Walk me through reducing a \"{source}\" instance to \"{target}\", \
                         step by step.\n\n\
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
                         Show me the cheapest reduction path and explain the cost at each \
                         step. Are there alternative paths? If so, compare them — which is \
                         better for small instances vs. large instances?"
                    ),
                )],
            })
        }

        "overview" => Some(GetPromptResult {
            description: Some("Overview of the NP-hard problem reduction landscape".into()),
            messages: vec![PromptMessage::new_text(
                PromptMessageRole::User,
                "Give me an overview of the NP-hard problem reduction landscape.\n\n\
                 How many problem types are registered? What are the major categories \
                 (graph, SAT, optimization)? Which problems are the most connected hubs? \
                 Which problems can reach the most targets through reductions?\n\n\
                 Summarize the structure so I understand what's available and where to \
                 start exploring."
                    .to_string(),
            )],
        }),

        _ => None,
    }
}
```

**Step 2: Run `cargo check` to verify compilation**

Run: `cargo check -p problemreductions-cli`
Expected: compiles without errors

**Step 3: Commit**

```bash
git add problemreductions-cli/src/mcp/prompts.rs
git commit -m "refactor(mcp): rewrite prompt texts to be task-oriented, not tool-centric"
```

---

### Task 3: Update integration test

**Files:**
- Modify: `problemreductions-cli/tests/mcp_integration.rs` (the `test_mcp_server_prompts_list` test)

**Step 1: Update the test assertions**

Change the prompt count from 3 to 7 and update the name checks:

```rust
assert_eq!(
    prompts.len(),
    7,
    "Expected 7 prompts, got {}: {:?}",
    prompts.len(),
    prompts
        .iter()
        .map(|p| p["name"].as_str().unwrap_or("?"))
        .collect::<Vec<_>>()
);

let prompt_names: Vec<&str> = prompts.iter().filter_map(|p| p["name"].as_str()).collect();
assert!(prompt_names.contains(&"what_is"));
assert!(prompt_names.contains(&"model_my_problem"));
assert!(prompt_names.contains(&"compare"));
assert!(prompt_names.contains(&"reduce"));
assert!(prompt_names.contains(&"solve"));
assert!(prompt_names.contains(&"find_reduction"));
assert!(prompt_names.contains(&"overview"));
```

**Step 2: Run the integration test**

Run: `cargo test -p problemreductions-cli --test mcp_integration test_mcp_server_prompts_list`
Expected: PASS

**Step 3: Run full MCP test suite**

Run: `make mcp-test`
Expected: all tests pass

**Step 4: Commit**

```bash
git add problemreductions-cli/tests/mcp_integration.rs
git commit -m "test(mcp): update prompt integration test for 7 task-oriented prompts"
```

---

### Task 4: Rebuild CLI and manual verification

**Step 1: Rebuild the CLI**

Run: `make cli`
Expected: successful install

**Step 2: Verify prompts list via JSON-RPC**

Run the MCP server and list prompts to confirm all 7 appear with correct names and argument counts.

**Step 3: Verify one prompt get**

Call `prompts/get` for `what_is` with `{"problem": "MaxCut"}` and confirm the response contains the new task-oriented text (no tool names like `show_problem`).

**Step 4: Verify another prompt get**

Call `prompts/get` for `solve` with `{"problem_type": "MIS", "instance": "edges: 0-1,1-2"}` and confirm the response text.
