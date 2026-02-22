# MCP Prompts Redesign: Task-Oriented Prompts

## Problem

The current 3 MCP prompts (`analyze_problem`, `reduction_walkthrough`, `explore_graph`) are tool-centric — they list which tools to call rather than expressing what the user wants to accomplish. This makes them disconnected from how researchers, students, and LLM agents actually think about reductions.

## Design

Replace the 3 existing prompts with 7 task-oriented prompts. All prompt text frames requests as user questions. No tool names appear in prompt text — the LLM decides which tools to call.

### Prompt Inventory

| # | Name | Arguments | User question it answers |
|---|------|-----------|--------------------------|
| 1 | `what_is` | `problem` (req) | "What is MaxCut?" |
| 2 | `model_my_problem` | `description` (req) | "I have a scheduling problem — what maps to it?" |
| 3 | `compare` | `problem_a` (req), `problem_b` (req) | "How do MIS and Vertex Cover relate?" |
| 4 | `reduce` | `source` (req), `target` (req) | "Walk me through reducing MIS to QUBO" |
| 5 | `solve` | `problem_type` (req), `instance` (req) | "Solve this graph for maximum independent set" |
| 6 | `find_reduction` | `source` (req), `target` (req) | "What's the cheapest path from SAT to QUBO?" |
| 7 | `overview` | *(none)* | "Show me the full problem landscape" |

### Prompt Texts

#### 1. `what_is`

**Description:** Explain a problem type: what it models, its variants, and how it connects to other problems

```
Explain the "{problem}" problem to me.

What does it model in the real world? What are its variants (graph types,
weight types)? What other problems can it reduce to, and which problems
reduce to it?

Give me a concise summary suitable for someone encountering this problem
for the first time, then show the technical details.
```

#### 2. `model_my_problem`

**Description:** Map a real-world problem to the closest NP-hard problem type in the reduction graph

```
I have a real-world problem and I need help identifying which NP-hard
problem type it maps to.

Here's my problem: "{description}"

Look through the available problem types in the reduction graph and
identify which one(s) best model my problem. Explain why it's a good fit,
what the variables and constraints map to, and suggest how I could encode
my specific instance.
```

#### 3. `compare`

**Description:** Compare two problem types: their relationship, differences, and reduction path between them

```
Compare "{problem_a}" and "{problem_b}".

How are they related? Is there a direct reduction between them, or do they
connect through intermediate problems? What are the key differences in
what they model? If one can be reduced to the other, what is the overhead?
```

#### 4. `reduce`

**Description:** Step-by-step reduction walkthrough: create an instance, reduce it, solve it, and map the solution back

```
Walk me through reducing a "{source}" instance to "{target}", step by step.

1. Find the reduction path and explain the overhead.
2. Create a small, concrete example instance of "{source}".
3. Reduce it to "{target}" and show what the transformed instance looks like.
4. Solve the reduced instance.
5. Explain how the solution maps back to the original problem.

Use a small example so I can follow each transformation by hand.
```

#### 5. `solve`

**Description:** Create and solve a problem instance, showing the optimal solution

```
Create a {problem_type} instance with these parameters: {instance}

Solve it and show me:
- The problem instance details (size, structure)
- The optimal solution and its objective value
- Why this solution is optimal (briefly)
```

#### 6. `find_reduction`

**Description:** Find the best reduction path between two problems, with cost analysis

```
Find the best way to reduce "{source}" to "{target}".

Show me the cheapest reduction path and explain the cost at each step.
Are there alternative paths? If so, compare them — which is better for
small instances vs. large instances?
```

#### 7. `overview`

**Description:** Explore the full landscape of NP-hard problems and reductions in the graph

```
Give me an overview of the NP-hard problem reduction landscape.

How many problem types are registered? What are the major categories
(graph, SAT, optimization)? Which problems are the most connected hubs?
Which problems can reach the most targets through reductions?

Summarize the structure so I understand what's available and where to
start exploring.
```

## Scope

- **Changed:** `problemreductions-cli/src/mcp/prompts.rs` (prompt definitions)
- **Changed:** `problemreductions-cli/src/mcp/tests.rs` (prompt tests)
- **Unchanged:** All 10 MCP tools, tool handlers, server infrastructure

## Testing

- Unit tests: verify `list_prompts` returns 7 prompts with correct names/arguments
- Unit tests: verify `get_prompt` returns correct message text for each prompt
- Integration test: call each prompt via JSON-RPC and verify response structure
