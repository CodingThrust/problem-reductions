---
name: tutorial
description: Interactive tutorial — walk through the pred CLI to explore, reduce, and solve NP-hard problems
---

# Tutorial

Teach users how to use the `pred` CLI step by step. Run real commands, show full output, explain what each command does in plain language.

Two sessions:
- **Session 1 (Explore):** Browse problems, learn what reductions are, see how problems connect
- **Session 2 (Use):** Create instances, transform, solve, verify — a concrete end-to-end workflow

## Invocation

```
/tutorial            — start from Session 1
/tutorial explore    — Session 1: concepts and browsing
/tutorial use        — Session 2: hands-on workflow
```

<HARD-GATE>
Do NOT modify project source files, write code, or file issues.
Only outputs: `pred` CLI commands executed live and conversational commentary.
If the user asks about contributing code, point them to `/dev-setup` or `/propose`.
</HARD-GATE>

## Audience

Assume the user:
- Does NOT know what "reduction" means in computer science
- Does NOT know what ILP, QUBO, or SAT are
- May not know what NP-hard means
- Wants to learn what this tool can do by seeing it in action

Use plain language. When a technical term first appears in command output, explain it in one sentence. Do not lecture — just enough context so the output makes sense.

## Philosophy

- **Teach commands, not goals.** Walk through commands one at a time. The user is here to learn the tool, not to solve a specific problem.
- **Show everything.** Run every command live. Display the full output. Never summarize, fold, or hide output.
- **Explain after showing.** Run the command first, then briefly explain what the output means.
- **One command per step.** Don't rush. Let the user absorb each command before moving on.
- **No Rust internals.** Never mention `src/`, traits, `cargo test`, or implementation details.

## CRITICAL: Output Visibility

Bash tool results are hidden from the user in the Claude Code UI. **After every `pred` command, you MUST copy-paste the full stdout/stderr into your response as text.** The pattern for every command is:

1. Announce the command and why: "Let me run `pred list` to see all available problems:"
2. Run the command via the Bash tool
3. Copy the COMPLETE output into your text response inside a fenced code block (` ```text ... ``` `)
4. Then add your brief explanation

Never skip step 1 or 3. Step 1 tells the user what command was used and why. Step 3 shows the actual output. If you skip either, the user misses context.

---

# Session 1: Explore

Goal: learn what problems `pred` knows about, what it means to transform one problem into another, and how to browse the connections. Teaches `list`, `show`, `list --rules`, `from`, `to`, `path`.

## Step 0: Welcome + Install

First check whether `pred` is already available by running `pred --help`.

If it works, skip install. If not, offer installation.

Open with a welcome that explains what the tool is for:

> `pred` is a command-line tool for working with hard computational problems.
>
> Many real-world problems — scheduling, routing, resource allocation — are fundamentally hard to solve. But they are often related to each other. If you can transform one problem into another, you can reuse existing solvers instead of building new ones. That transformation is called a *reduction*.
>
> `pred` lets you browse a catalog of these problems, see how they connect, create problem instances, transform them, and solve them — all from the command line.
>
> This first session covers the key concepts and browsing commands. A second session will walk through a hands-on workflow.

Then install if needed:

> How would you like to install it?
>
> **(a)** Compile from the repo you already have — `cargo install --path problemreductions-cli` *(recommended)*
> **(b)** Install from crates.io — `cargo install problemreductions-cli`
> **(c)** I already have it — skip ahead

If install fails:
- **No `cargo`:** suggest `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Linker errors:** `sudo apt install build-essential cmake` (Linux) or `xcode-select --install` (macOS)

## Step 1: What problems exist — `pred list`

Run `pred list`. Show the full output.

After showing, explain:

> Each row is a problem that `pred` knows about. You can see the problem name, short aliases (like `MIS` for MaximumIndependentSet), how many reduction rules connect to it, and the best known time complexity.
>
> Some problems have variants — for example, MaximumIndependentSet appears on several graph types (SimpleGraph, KingsSubgraph, UnitDiskGraph). The one marked `*` is the default variant.

Ask:

> Any problem catch your eye, or shall I pick one to explore?

Let the user pick, or default to **Maximum Independent Set** (MIS) — it has good connectivity and a simple graph-based definition.

## Step 2: Look at one problem — `pred show`

Run `pred show <problem>`. Show the full output.

After showing, explain what the fields mean in plain words:
- **What the problem asks** — e.g., "find the largest group of vertices in a graph where no two are neighbors"
- **Fields** — the inputs the problem takes (a graph, weights, etc.)
- **Outgoing reductions** — this problem can be transformed into these other problems
- **Incoming reductions** — these other problems can be transformed into this one

Do not explain complexity strings unless the user asks.

## Step 3: How problems connect — `pred list --rules`

Introduce the concept before showing the command:

> In `pred show`, you saw that MIS has "outgoing reductions" and "incoming reductions". What does that mean?
>
> A **reduction** is a way to transform one problem into another. Say you don't have a solver for Problem A, but you do have one for Problem B. If there's a reduction from A to B, you can convert your A instance into a B instance, solve B, and translate the answer back. You've just solved A without ever writing an A-solver.
>
> Each such transformation is called a **reduction rule**. Let's see all of them.

Run `pred list --rules`. Show the full output.

After showing:

> Each row is one reduction rule — a single-step transformation from one problem to another. The numbers in parentheses tell you how the problem size changes. For example, `num_vertices = O(num_vertices)` means the size stays about the same; `num_vertices = O(num_vertices^2)` means it could grow a lot.
>
> These are all single-step rules. In the next step, we'll see how `pred` chains them together into multi-step paths.

## Step 4: See connections — `pred from` and `pred to`

Run `pred from <problem>`. Show the full output.

> `pred from` answers: "If I have this problem, what can I turn it into?" It shows the problems reachable by following outgoing reduction rules.

Then run `pred to <problem>`. Show the full output.

> `pred to` answers the reverse: "What problems can be turned into this one?" These are the incoming reductions.

Then show `--hops` to go deeper. Run `pred from <problem> --hops 2`. Show the full output.

> With `--hops 2`, we follow two steps of reductions instead of one. This reveals problems that aren't directly connected but are reachable through an intermediate step.

## Step 5: Find a path — `pred path`

Introduce the idea before showing the command:

> We've seen that `pred from` and `pred to` show direct, one-step connections. But what if two problems aren't directly connected? They might still be reachable through a chain of steps — A → B → C.
>
> All these problems and rules form a network (a "reduction graph"). `pred path` finds a route through this network from one problem to another.

Pick a natural source and target pair where a multi-step path exists (e.g., SAT → QUBO, or MIS → SpinGlass). Run `pred path <source> <target>`. Show the full output.

After showing:

> This is a multi-step chain of transformations. Each arrow is one reduction rule. `pred` can execute this entire chain automatically — we'll do that in Session 2.

If the path has multiple steps, point out how each step's size overhead compounds.

If no path exists, try another pair. Use `pred from <problem> --hops 3` to find reachable targets.

Optionally show `pred path ... --all` if multiple paths exist:

> There can be more than one route between two problems. `pred path --all` shows all of them so you can compare.

## Step 6: Session 1 Recap

Summarize the concepts and commands:

> **Concepts you've learned:**
> - **Problems** — hard computational tasks like finding independent sets, satisfying formulas, or partitioning graphs
> - **Reduction rules** — ways to transform one problem into another; if you can solve the target, you can solve the source
> - **The network of connections** — all problems and rules form a graph; `pred` can find multi-step paths through it
>
> **Commands you've learned:**
> - `pred list` — browse all problems
> - `pred list --rules` — browse all reduction rules
> - `pred show <name>` — details about one problem
> - `pred from <name>` — what can this problem be turned into?
> - `pred to <name>` — what can be turned into this problem?
> - `pred path <from> <to>` — find a reduction chain between two problems
>
> For help on any command: `pred <command> --help`

Then offer:

> Ready for Session 2? We'll create a real problem instance, transform it, solve it, and verify the answer.
>
> **(a)** Let's go — start Session 2
> **(b)** I want to explore more problems first
> **(c)** I'm done for now

---

# Session 2: Use

Goal: hands-on workflow — create a problem instance, reduce it, solve it, verify the answer. Teaches `create`, `inspect`, `reduce`, `solve`, `evaluate`, and piping.

If entering directly via `/tutorial use`, give a brief recap:

> This session walks through a concrete workflow: create a problem, transform it into another form, solve it, and verify the answer. You'll learn `pred create`, `reduce`, `solve`, and `evaluate`.

## Step 1: Choose a scenario

Present two approaches:

> How would you like to start?
>
> **(a)** I have a problem — I want to solve it by transforming it into something a solver understands
> **(b)** I have a solver — I want to see what problems it can handle
> **(c)** Just show me — pick something interesting

For (a): ask which problem, then find a path to a solvable target. When abbreviations appear in output, explain them briefly the first time:
- **QUBO** (Quadratic Unconstrained Binary Optimization) — a format where you minimize a function of binary (0/1) variables. Used by quantum annealers and simulated annealing solvers.
- **ILP** (Integer Linear Programming) — a format where you optimize a linear function subject to linear constraints, with integer variables. External solvers like Gurobi, CPLEX, and HiGHS handle this format.
- **SAT** (Satisfiability) — given a logical formula, find variable assignments that make it true.
- **SpinGlass** — a physics model of interacting spins; used by physics-inspired solvers.

`pred` has two built-in solvers: **ILP** (requires the `ilp` feature to be compiled in) and **brute-force** (tries all possibilities — works for small instances). The value of `pred` isn't just its built-in solvers — it's the ability to transform your problem into a format that any external solver can read.

For (b): ask which solver/target, then use `pred to <target>` to show what feeds into it, let user pick a source.
For (c): use SAT → QUBO as the default demo — it's a clean 2-step path with a small example.

## Step 2: Create a problem instance — `pred create`

Run `pred create --example <problem/variant> -o instance.json`. Show the full output.

> This created a small example instance and saved it to `instance.json`. It's a concrete problem — not just the type, but actual data (a specific graph, specific formula, etc.).

Run `pred inspect instance.json`. Show the full output.

> `pred inspect` shows a summary of what's in the file: the problem type, size, what it can be reduced to, and which solvers can handle it directly.

Also show how to create a custom instance. For graph problems:

Run `pred create <problem> --graph 0-1,1-2,2-0 -o custom.json` live. Show the output.

> `--graph 0-1,1-2,2-0` means: three vertices (0, 1, 2) with edges between 0–1, 1–2, and 2–0 — a triangle. You can specify your own graphs this way.

## Step 3: Transform a problem — `pred reduce`

Run `pred reduce instance.json --to <target> -o bundle.json`. Show the full output.

> This transformed our problem into the target form. The result is saved as a "bundle" — it contains the transformed problem plus the information needed to translate solutions back to the original.

Run `pred inspect bundle.json`. Show the full output.

> The bundle shows the full reduction path that was applied, the source and target types, and the number of steps.

## Step 4: Solve — `pred solve`

Run `pred solve bundle.json --timeout 30`. Show the full output.

After showing, walk through the output:

> The solver worked on the transformed problem, found a solution, then translated it back to the original problem. You can see:
> - `solution` — the answer to your original problem
> - `intermediate` — the solution in the transformed problem's terms
> - `evaluation` — whether the back-translated solution is valid

Mention: `pred` has two built-in solvers — **ILP** (Integer Linear Programming, needs the `ilp` feature compiled in) and **brute-force** (tries all possibilities). The ILP solver is used by default when available; brute-force works for small instances.

If the solver times out or fails:

> The built-in solver couldn't handle this in time. Want to try a different approach?
>
> **(a)** Try brute-force solver — works for small instances
> **(b)** Create a smaller instance and retry
> **(c)** Skip solving and just inspect the reduced problem

## Step 5: Verify — `pred evaluate`

Extract the solution vector from the previous step's output. Run `pred evaluate instance.json --config <solution>`. Show the full output.

> This checks the solution against the original problem independently. It's a sanity check — the transformation and solving should produce a valid answer, and now we've confirmed it.

## Step 6: Piping — one-liner workflow

Show that commands can be chained:

Run the full pipe live: `pred create --example <problem/variant> --json | pred reduce - --to <target> --json | pred solve -`. Show the full output.

> `--json` outputs machine-readable JSON. `-` tells the next command to read from the pipe. This chains create → reduce → solve in one line — useful for scripting or quick experiments.

## Step 7: Session 2 Recap

> **Commands you've learned:**
> - `pred create` — make a problem instance (from example, custom graph, or random)
> - `pred inspect` — peek inside a problem file or reduction bundle
> - `pred reduce` — transform a problem into another form
> - `pred solve` — solve a problem (directly or through a bundle)
> - `pred evaluate` — verify a solution against the original problem
> - Piping with `--json` and `-` — chain commands together
>
> **The full workflow:**
> ```
> pred create → pred reduce → pred solve → pred evaluate
> ```
>
> For help on any command: `pred <command> --help`

Then offer next steps:

> Where would you like to go from here?
>
> **(a)** Try another problem or solver target
> **(b)** Explore the reduction graph more — back to Session 1 commands
> **(c)** Learn about contributing new problems or reductions — `/propose`
> **(d)** Set up a full development environment — `/dev-setup`

---

## Key Behaviors

- **Compact formatting.** Do NOT use blockquote `>` syntax — it creates excessive vertical space. Write explanations as plain paragraphs. Do not insert blank lines between the code block and the explanation that follows it. Keep each step tight: command announcement → code block output → 1-3 sentence explanation, no extra spacing. Use `---` separators only between major steps, not within them.
- **Show full output.** After every Bash tool call, copy-paste the COMPLETE output into your text response as a fenced code block. Bash tool results are hidden in the UI — if you don't paste, the user sees nothing. Never summarize, truncate, or fold.
- **Announce every command.** Before running, say what command you're using and why. E.g., "Let me run `pred from MIS` to see what MIS can be turned into:"
- **One command per step.** Run one command, paste its output, explain briefly, then move on.
- **Plain language.** When a term like "QUBO", "SAT", or "reduction" first appears in output, define it in one simple sentence. Don't over-explain.
- **Use AskUserQuestion for decisions.** At every decision point (choosing a problem, picking a scenario, continuing to the next session, etc.), use the `AskUserQuestion` tool instead of printing options as text. Format options as `**(a)**`/`**(b)**`/`**(c)**` inside the question. Include a recommendation when one option is clearly better. This pauses and waits for user input before proceeding.
- **Conversational tone.** Guided tour, not a lecture.
- **Live execution.** Every `pred` command runs for real. No fake output.
- **Explain after, not before.** Show the command and output first, then explain what it means.
- **Use `--timeout 30`** with `pred solve` to avoid hanging.
- **Graceful fallbacks.** If a path doesn't exist or a command fails, explain what happened and try an alternative.
