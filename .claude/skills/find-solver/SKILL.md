---
name: find-solver
description: Interactive guide — match a real-world problem to a library model, explore reduction paths, recommend solvers (built-in + external), and generate a solution doc
---

# Find Solver

Guide users from a real-world algorithmic problem to a concrete solving strategy. Produces a static solution doc in `docs/solutions/`.

## Invocation

```
/find-solver               — start from Step 1 (clarify problem)
/find-solver <ProblemName> — skip to Step 3 (explore reductions for a known model)
```

<HARD-GATE>
Do NOT modify project source files, write Rust code, or create PRs.
Only outputs: `pred` CLI commands executed live, web searches, conversational commentary, and one solution doc in `docs/solutions/`.
If the user asks about contributing code, point them to `/add-model`, `/add-rule`, or `/propose`.
</HARD-GATE>

## Audience

This skill serves two types of users:

- **Practitioners** who have a fuzzy real-world problem (e.g., "I need to assign tasks to machines minimizing makespan") and need help identifying which NP-hard problem it maps to
- **Researchers** who already know the formal problem name and want to find the best reduction path + solver

Adapt the flow: if the user provides a formal problem name, validate it with `pred show` and skip directly to Step 3.

## Flow Overview

```
Step 1: Clarify Problem (skip if user knows the formal name)
Step 2: Match to Library Models (web search + pred list)
Step 3: Explore Reduction Paths (auto-explore via pred from --hops 3)
Step 4: Recommend Solvers (web search + pred solve options)
Step 5: Generate Solution Doc (docs/solutions/<name>.md)
```

## CRITICAL: Output Visibility

Bash tool results are hidden from the user in the Claude Code UI. **After every `pred` command, you MUST copy-paste the full stdout/stderr into your response as text.** The pattern for every command is:

1. Announce the command and why: "Let me run `pred to MIS` to see what MIS can be reduced to:"
2. Run the command via the Bash tool
3. Copy the COMPLETE output into your text response inside a fenced code block
4. Then add your brief explanation

Never skip step 1 or 3.

---

## Step 1: Clarify Problem

**Goal:** Understand the user's problem well enough to form a search query for matching models.

**Researcher shortcut:** If the user invoked `/find-solver <ProblemName>` or describes their problem using a formal name (e.g., "maximum independent set", "graph coloring"), run `pred show <name>` to validate it exists. If it does, skip directly to Step 3 with that model. If it doesn't exist in the library, tell the user and fall through to Step 2.

**For practitioners, ask one question at a time:**

1. "Describe your problem in plain language — what are you trying to optimize, decide, or compute?"

2. Based on the answer, if the input structure is ambiguous, ask:
   "Is your input a graph/network? A set of items? A boolean formula? Numbers/matrices?"

3. If the objective is unclear:
   "What's the objective — minimize something, maximize something, or check feasibility?"

4. "Roughly how large are your instances? (e.g., 10 nodes, 1000 variables)"

Use `AskUserQuestion` for each question. Format options as **(a)**/**(b)**/**(c)** when multiple choice is natural.

**Exit condition:** You have enough context to form a search query like "task scheduling minimize makespan NP-hard" or "maximum weight independent set unit disk graph". Proceed to Step 2.

---

## Step 2: Match to Library Models

**Goal:** Identify which problem model(s) in the library match the user's problem.

**Key rule:** Web search happens BEFORE presenting options. Never guess model matches from internal knowledge alone.

**Actions:**

1. **Web search** the clarified problem description together with terms like "NP-hard", "computational complexity", or "reduction" to find formal problem names and known relationships in the literature. Use `WebSearch` tool.

2. **Run `pred list`** to get the full catalog of available models. Copy-paste the full output into your response.

3. **Cross-reference** the web search results against the `pred list` catalog. For each candidate model that exists in the library (3-5 max), present a table:

| # | Model | Why it might match | Caveat |
|---|-------|--------------------|--------|
| 1 | ... | ... | ... |
| 2 | ... | ... | ... |
| 3 | ... | ... | ... |

4. For each candidate, run `pred show <model>` and show the output — fields, complexity, available reductions. This helps the user see what data they would need to provide.

5. **Ask the user to pick one** using `AskUserQuestion`. If none fit, ask the user for more detail and re-run the web search with refined keywords.

**Proceed to Step 3 with the chosen model.**

---

## Step 3: Explore Reduction Paths

**Goal:** Discover all solver-ready targets reachable from the chosen model and present them ranked.

**Actions:**

1. **Run `pred from <model> --hops 3`** to find all problems reachable via outgoing reductions within 3 hops. Copy-paste the full output.

2. **For each reachable problem**, gather info:
   - Run `pred path <model> <target>` to get the cheapest witness-capable reduction path and composed overhead
   - Run `pred show <target>` to get its best-known complexity
   - Check if it's a solver-ready target (ILP, QUBO, SAT) or has a path to one via `pred path <target> ILP`

3. **Present a ranked table** (most practical paths first — fewest hops, lowest overhead):

   | # | Target | Hops | Composed Overhead | Target Complexity | Solver-Ready? |
   |---|--------|------|-------------------|-------------------|---------------|
   | 1 | ILP | 2 | num_vars = 2*n + m | O(2^num_vars) | Yes (is ILP) |
   | 2 | QUBO | 1 | num_vars = n | O(2^num_vars) | Yes (is QUBO) |
   | 3 | MaxSetPacking | 1 | num_sets = n | O(2^num_sets) | Yes (ILP in 2 steps) |

4. **Ask the user** using `AskUserQuestion`: "Which reduction path would you like to use? Pick a number."

**If `pred from --hops 3` returns more than 15 results:** present only the top 10 by overhead and mention the rest are available.

**Proceed to Step 4 with the chosen path.**

---

## Step 4: Recommend Solvers

**Goal:** Find the best solver options — both built-in and external — for the target problem.

**Key rule:** Web search happens BEFORE presenting solver options. Do not recommend solvers from internal knowledge alone.

**Actions:**

1. **Web search** the final target problem + "solver" + "benchmark" + "library" to find state-of-the-art external tools. Use `WebSearch` tool. Example queries:
   - "QUBO solver open source benchmark"
   - "integer linear programming solver comparison"
   - "maximum independent set practical solver"

2. **Check built-in solver availability:**
   - Run `pred path <current_model> ILP` — if a path exists, `pred solve --solver ilp` is available
   - `pred solve --solver brute-force` is always available (feasible for small instances, ~25 variables)

3. **Present solver options** in a table:

   | # | Solver | Type | How to Use | Notes |
   |---|--------|------|------------|-------|
   | 1 | pred solve --solver ilp | Built-in | pred solve reduced.json | HiGHS backend |
   | 2 | pred solve --solver brute-force | Built-in | pred solve problem.json --solver brute-force | Exact, small instances only |
   | 3 | (from web search) | External | (brief setup) | (strengths/limitations) |
   | 4 | (from web search) | External | (brief setup) | (strengths/limitations) |

4. **Ask the user** which solver(s) to include in the solution doc using `AskUserQuestion`.

**Proceed to Step 5 with the selected solver(s).**

---

## Step 5: Generate Solution Doc

**Goal:** Write a static reference document with everything the user needs to solve their problem.

**File path:** `docs/solutions/<problem>-via-<model>-<solver>.md`

Where:
- `<problem>` is a short kebab-case description of the real-world problem
- `<model>` is the library model name (e.g., `MIS`, `QUBO`)
- `<solver>` is the primary solver (e.g., `ILP`, `brute-force`, `gurobi`)

Ask the user to confirm the filename before writing.

**Doc template — write all sections:**

```markdown
# <Real-world Problem> via <Model> -> <Solver>

## Problem Description

<What the user described, formalized. One paragraph.>

## Matched Model

- **Name:** <ProblemType> {variant}
- **Why this model:** <reasoning from Step 2>
- **Best-known complexity:** O(...)

## Input Schema

<JSON schema from `pred show --json <model>`, with field explanations.>

Example instance:

```json
<Run `pred create --example <model/variant>` and paste the JSON output>
```

## Reduction Path

<For each step in the path chosen during Step 3:>

### Step N: <Source> -> <Target>

- **Overhead:** <field-by-field from pred show>

```bash
pred reduce input.json --to <Target> -o step_N.json
```

## Solving

```bash
pred solve step_N.json --solver ilp --timeout 60
```

<Explain what the output means for the user's original problem.
E.g., "Max(3) means the maximum independent set has 3 vertices.">

## Solution Extraction

<Explain how the target solution maps back to the original problem.>

Using the reduction bundle workflow (recommended):

```bash
pred reduce input.json --to <FinalTarget> -o bundle.json
pred solve bundle.json --solver ilp --timeout 60
```

The solver automatically extracts the solution back to the original problem space.

## External Solver Alternatives

<For each external solver chosen in Step 4:>

### <Solver Name>

- **What:** <one-line description>
- **When to prefer:** <when this is better than built-in>
- **How to use:** <brief setup or export instructions, if applicable>

## Quick Reference

All commands in sequence:

```bash
# 1. Create your problem instance
pred create <Model> <flags> -o input.json

# 2. Reduce to solver-ready form
pred reduce input.json --to <FinalTarget> -o bundle.json

# 3. Solve
pred solve bundle.json --solver ilp --timeout 60

# 4. Verify (optional)
pred evaluate input.json --config <solution_vector>
```
```

**After writing the doc:**

1. Show the user the generated filename and a brief summary of what's in it.
2. Ask if they want to make any changes before finishing.

---

## Key Behaviors

- **One question at a time.** Never ask multiple questions in one message. Use `AskUserQuestion` for every decision point.
- **Web search before recommendations.** In Step 2 (model matching) and Step 4 (solver recommendation), always web search first. Never rely on internal knowledge alone.
- **Show full output.** After every Bash tool call, copy-paste the COMPLETE output into your text response as a fenced code block. Bash tool results are hidden in the UI.
- **Announce every command.** Before running, say what command you're using and why.
- **Compact formatting.** Write explanations as plain paragraphs. Do not use blockquote `>` syntax for explanations. Keep tight: command announcement, code block output, 1-3 sentence explanation.
- **Conversational tone.** Guided consultation, not a lecture.
- **Live execution.** Every `pred` command runs for real. No fake output.
- **Graceful fallbacks.** If a path doesn't exist or a command fails, explain what happened and suggest alternatives (try another model, use brute-force, backtrack).
- **Adapt to user level.** If the user gives a formal problem name, skip clarification. If they describe a fuzzy real-world problem, ask follow-ups one at a time.
- **Use `--timeout 30`** with `pred solve` in any live demos during the session.
