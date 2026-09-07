---
name: find-problem
description: Reverse of find-solver — given a solver for a model, discover what other problems it can handle via incoming reductions, ranked by effective complexity
---

# Find Problem

Given a solver for a specific model, discover what other problems it can handle by exploring the reduction graph in the incoming direction. Produces a solution doc ranking all reachable problems by effective complexity.

## Invocation

```
/find-problem               — start from Step 1 (identify solver)
/find-problem <ModelName>   — skip model identification, ask for complexity
```

<HARD-GATE>
Do NOT modify project source files, write Rust code, or create PRs.
Only outputs: `pred` CLI commands executed live, web searches, conversational commentary, and one solution doc in `docs/solutions/`.
If the user asks about contributing code, point them to `/add-model`, `/add-rule`, or `/propose`.
</HARD-GATE>

## Audience

Users who have built or have access to a solver for a specific problem model and want to understand the full scope of problems their solver can handle through reductions.

## Flow Overview

```
Step 1: Identify Solver (user provides model + complexity)
Step 2: Discover Reachable Problems (pred from --hops 3, compute effective complexity)
Step 3: Rank and Present (table ranked by effective complexity, web search for applications)
Step 4: Generate Solution Doc (docs/solutions/<name>.md)
```

## Prerequisites

Build the CLI tools before starting: `make cli` (builds `pred` and `pred-sym`). All commands below assume `pred` and `pred-sym` are available via `cargo run -p problemreductions-cli --bin pred --` and `cargo run -p problemreductions-cli --bin pred-sym --` respectively.

## CRITICAL: Output Visibility

Bash tool results are hidden from the user in the Claude Code UI. **After every `pred` / `pred-sym` command, you MUST copy-paste the full stdout/stderr into your response as text.** The pattern for every command is:

1. Announce the command and why: "Let me run `pred to MIS --hops 3` to discover all problems that can reduce to MIS:"
2. Run the command via the Bash tool
3. Copy the COMPLETE output into your text response inside a fenced code block
4. Then add your brief explanation

Never skip step 1 or 3.

---

## Step 1: Identify Solver

**Goal:** Get the user's model name and solver complexity.

**If invoked as `/find-problem <ModelName>`:** validate with `pred show <ModelName>`. If it exists, show the output (including parameters), then ask for solver complexity.

**If invoked as `/find-problem`:** ask using `AskUserQuestion`: "Which problem model does your solver handle?" Validate the answer with `pred show`.

**Ask for complexity** using `AskUserQuestion`: "What is your solver's time complexity? Use the size field names from the output above (e.g., `O(1.1996^num_vertices)`, `O(2^(num_variables/3))`)."

- Variable names should match the model's parameters shown in `pred show` output
- If the user gives informal notation (e.g., "exponential in n"), help them formalize it using the model's actual size field names

**Exit condition:** Validated model name + complexity expression with variables matching the model's parameters. Proceed to Step 2.

---

## Step 2: Discover Reachable Problems

**Goal:** Find all problems that can reduce to the user's model and compute effective complexity for each.

**Actions:**

1. **Run `pred to <model> --hops 3`** to find all problems that can reduce to the user's model within 3 hops (incoming direction). Copy-paste the full output.

2. **For each discovered problem**, run:
   - `pred path <source> <model>` — get the cheapest witness-capable reduction path
   - **IMPORTANT:** Use the exact variant-qualified name from `pred to` output (e.g., `SpinGlass/SimpleGraph/f64`, not bare `SpinGlass`). Bare names resolve to the default variant, which may differ from the reachable variant and cause false "no path" errors.
   - `pred show <source>` — get best-known brute-force complexity

3. **Compute effective complexity** for each source problem:
   - Take the user's solver complexity expression (e.g., `O(1.1996^num_vertices)`)
   - Substitute the overhead expressions from the reduction path into the solver's variables
   - Example: if MVC→MIS has overhead `num_vertices = num_vertices`, then solving MVC via MIS costs `O(1.1996^num_vertices)` — same as MIS
   - Example: if overhead is `num_vertices = num_clauses * 3`, then effective complexity is `O(1.1996^(3 * num_clauses))`
   - **Use `pred-sym` to verify:** after manual substitution, run `pred-sym big-o "<effective_expr>"` to normalize the expression. Use `pred-sym eval --vars <bindings> "<expr>"` at a concrete size (e.g., n=20) to numerically verify the simplification.

4. **Compare to best-known**: for each source, compare effective complexity to the source's own best-known complexity from `pred show`. Classify as:
   - **Better** — effective complexity has a smaller base or exponent than best-known
   - **Similar** — comparable asymptotic behavior
   - **Worse** — effective complexity exceeds best-known (reduction overhead makes it impractical)
   - **When effective and best-known use different variables** (e.g., `O(1.5^num_subsets)` vs `O(2^universe_size)`): this happens when a problem has multiple independent parameters and the best-known algorithm's dominant variable differs from the reduction overhead's. In this case, use `pred-sym eval` at representative concrete values to determine the comparison. State the result conditionally: "Better when num_subsets ≤ c·universe_size" with the crossover ratio.

5. **Web search** only the **Better** and **Similar** candidates for real-world applications (not the Worse ones). Use `WebSearch` tool with query "<problem name> real-world applications".

**If `--hops 3` returns more than 15 results:** present only the top 10 by effective complexity and mention the rest are available if the user wants to see them.

**Proceed to Step 3.**

---

## Step 3: Rank and Present

**Goal:** Show all discovered problems ranked by practical usefulness.

Present a ranked table (most practical first). **Mark a recommendation** — highlight the "Better" entries as the most valuable discoveries:

| # | Problem | Hops | Overhead | Effective Complexity | vs Best-Known | Applications |
|---|---------|------|----------|---------------------|---------------|--------------|
| 1 | **MinimumVertexCover** | 1 | same size | O(1.1996^n) | **Better** | Network monitoring |
| 2 | **MaximumClique** | 2 | complement graph | O(1.1996^n) | **Better** | Social network cliques |
| 3 | GraphColoring | 3 | n^2 vars | O(1.1996^(n^2)) | Worse | Register allocation |

Ask using `AskUserQuestion`: "Which problems would you like included in the solution doc? Pick numbers, or 'all practical' for only the Better/Similar ones."

**Proceed to Step 4 with the selected problems.**

---

## Step 4: Generate Solution Doc

**Goal:** Write a static reference document listing all selected problems and how to solve them via the user's model.

**File path:** `docs/solutions/problems-solvable-via-<Model>-<solver>.md`

Where:
- `<Model>` is the library model name (e.g., `MIS`, `QUBO`)
- `<solver>` is a short label for the user's solver (e.g., `custom-1.1996`, `ILP`)

Ask the user to confirm the filename before writing.

**Before writing the doc**, run `pred create <Source> --help` for each selected problem to verify the correct CLI flag names. Use the flags exactly as shown in the help output.

**Doc template — write all sections:**

```markdown
# Problems Solvable via <Model> (<Solver Complexity>)

## Overview

<One paragraph: your solver for X can handle these Y problems via reductions. Brief explanation of the ranking methodology.>

## Summary Table

| Problem | Hops | Overhead | Effective Complexity | vs Best-Known | Applications |
|---------|------|----------|---------------------|---------------|--------------|
| ... | ... | ... | ... | ... | ... |

## <Problem 1> -> <Model>

- **What it is:** <brief description + real-world applications from web search>
- **Reduction path:** <Source> -> ... -> <Model>
- **Overhead:** <field-by-field>
- **Effective complexity:** <composed expression>
- **vs best-known:** <Better/Similar/Worse — with the source's brute-force complexity for comparison>

### CLI Commands

```bash
# Create a source problem instance
pred create <Source> <flags> -o input.json

# Reduce to your solver's model
pred reduce input.json --to <Model> -o bundle.json

# Solve (built-in ILP or your external solver)
pred solve bundle.json --solver ilp --timeout 60
```

## <Problem 2> -> <Model>

...
```

**After writing the doc:**

1. Show the user the generated filename and a brief summary of what's in it.
2. **If a built-in solver covers the model** (brute-force or ILP), offer to run a live demo with one of the "Better" problems: "Want me to run an example end-to-end so you can see it in action?"
3. Ask if they want to make any changes before finishing.

---

## Key Behaviors

- **One question at a time.** Never ask multiple questions in one message. Use `AskUserQuestion` for every decision point.
- **Web search only Better/Similar candidates.** In Step 2, web search only the problems classified as Better or Similar for real-world use cases. Skip Worse ones unless the user asks for all. Never guess applications from internal knowledge alone.
- **Show full output.** After every Bash tool call, copy-paste the COMPLETE output into your text response as a fenced code block. Bash tool results are hidden in the UI.
- **Announce every command.** Before running, say what command you're using and why.
- **Always use variant-qualified names in `pred path`.** When `pred to` returns names like `SpinGlass/SimpleGraph/f64`, use that exact string in subsequent `pred path` calls. Bare names (e.g., `SpinGlass`) resolve to the default variant, which may differ from the reachable variant and cause false "no path" errors.
- **Recommend, don't just list.** When presenting the ranked table in Step 3, bold the "Better" entries as the most valuable discoveries. The user can still pick freely.
- **Compact formatting.** Write explanations as plain paragraphs. Do not use blockquote `>` syntax for explanations. Keep tight: command announcement, code block output, 1-3 sentence explanation.
- **Conversational tone.** Guided consultation, not a lecture.
- **Live execution.** Every `pred` command runs for real. No fake output.
- **Graceful fallbacks.** If `pred to` returns no results (no incoming reductions), suggest trying with more hops or a different model. If `pred path` fails for a specific source, skip it and note it in the table.
- **Help with complexity notation.** If the user gives informal complexity, show `pred show <model>` parameters and help them write a formal expression.
- **Cap results at 10.** If discovery returns many problems, show top 10 by effective complexity and offer to show more.
