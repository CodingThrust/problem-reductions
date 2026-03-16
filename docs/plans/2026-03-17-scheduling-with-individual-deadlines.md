# Scheduling With Individual Deadlines Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `SchedulingWithIndividualDeadlines` satisfaction model, wire it into registry/CLI/example-db, and document it in the paper with the issue's worked schedule.

**Architecture:** Implement the model in `src/models/misc/` using one variable per task, where each variable stores the task's start slot and feasibility checks enforce deadlines, precedence, and per-slot processor capacity. Use the issue's 7-task feasible schedule as the canonical example, expose `max_deadline()` for the registry complexity expression `max_deadline^num_tasks`, and keep paper work in a separate batch after code and exports are stable.

**Tech Stack:** Rust workspace crate, registry-backed CLI discovery, serde, Typst paper, `make test`, `make clippy`, `make paper`

---

**Implementation skill reference:** Follow repo-local [`.claude/skills/add-model/SKILL.md`](/Users/jinguomini/rcode/problem-reductions/.worktrees/issue-503-scheduling-with-individual-deadlines/.claude/skills/add-model/SKILL.md) Steps 1-7.

**Issue context to preserve:** `SchedulingWithIndividualDeadlines` is a new `misc` satisfaction model. The worked example is the 7-task, 3-processor instance from issue `#503`, with feasible start times `[0, 0, 0, 1, 2, 1, 1]`. Associated inbound rule already exists as open issue `#478` (`[Rule] Vertex Cover to Scheduling with Individual Deadlines`), so the model will not be orphaned.

**Batching:** Batch 1 covers implementation, registration, CLI, example-db, and tests. Batch 2 covers the paper entry only.

## Batch 1

### Task 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/misc/scheduling_with_individual_deadlines.rs`
- Reference: `src/unit_tests/models/misc/minimum_tardiness_sequencing.rs`
- Reference: `src/unit_tests/models/misc/flow_shop_scheduling.rs`

**Step 1: Write the failing tests**

Add tests for:
- constructor/getters/dims/variant/name on the issue example
- `evaluate()` accepts the issue's feasible schedule `[0, 0, 0, 1, 2, 1, 1]`
- `evaluate()` rejects a deadline violation, a precedence violation, and a processor-capacity violation
- brute-force solver finds a satisfying schedule on a small satisfiable instance and returns `None` on a small unsatisfiable instance
- serde round-trip
- canonical paper/example-db instance stays satisfiable

**Step 2: Run the targeted test to verify RED**

Run: `cargo test scheduling_with_individual_deadlines --lib`

Expected: FAIL because the model file and exports do not exist yet.

**Step 3: Commit**

Do not commit yet. This task stays uncommitted until the model exists and the red-green cycle for the batch is complete.

### Task 2: Implement the model and canonical example

**Files:**
- Create: `src/models/misc/scheduling_with_individual_deadlines.rs`
- Reference: `src/models/misc/minimum_tardiness_sequencing.rs`

**Step 1: Write the minimal implementation**

Implement:
- `ProblemSchemaEntry` with constructor-facing fields `num_tasks`, `num_processors`, `deadlines`, `precedences`
- `SchedulingWithIndividualDeadlines` with serde derives
- constructor validation: `deadlines.len() == num_tasks`, precedence indices in range
- getters: `num_tasks()`, `num_processors()`, `deadlines()`, `precedences()`, `num_precedences()`, `max_deadline()`
- `dims()` returning `deadlines.clone()` so each task chooses a start slot in `0..d(t)-1`
- `evaluate()` returning `true` iff config length matches, every precedence `(u, v)` satisfies `start[u] + 1 <= start[v]`, and every slot has at most `num_processors` assigned tasks
- `declare_variants!` with `default sat SchedulingWithIndividualDeadlines => "max_deadline^num_tasks"`
- `canonical_model_example_specs()` using the issue's 7-task feasible schedule

**Step 2: Run the targeted test to verify GREEN**

Run: `cargo test scheduling_with_individual_deadlines --lib`

Expected: PASS for the new model tests.

### Task 3: Register the model in the crate and example-db

**Files:**
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Wire exports**

Add the new module, public re-exports, prelude export, and include its canonical example spec in the `misc::canonical_model_example_specs()` aggregator.

**Step 2: Run a focused regression check**

Run: `cargo test scheduling_with_individual_deadlines minimum_tardiness_sequencing flow_shop_scheduling --lib`

Expected: PASS for the new model and adjacent scheduling models.

### Task 4: Add CLI creation support

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Implement `pred create SchedulingWithIndividualDeadlines`**

Add a new `create()` match arm that reuses existing flags:
- `--n` for `num_tasks`
- `--num-processors`
- `--deadlines`
- optional `--precedence-pairs`

Validate lengths and precedence indices before constructing the model. Update CLI help text so the problem appears in the "Flags by problem type" list with a concrete usage shape.

**Step 2: Verify the CLI path**

Run: `cargo run -p problemreductions-cli -- create SchedulingWithIndividualDeadlines --n 7 --num-processors 3 --deadlines 2,1,2,2,3,3,2 --precedence-pairs "0>3,1>3,1>4,2>4,2>5"`

Expected: JSON for the issue example instance, with the canonical problem name resolved from the registry.

### Task 5: Run batch verification and review before paper work

**Files:**
- No new files; verify the batch state

**Step 1: Run verification**

Run: `make test clippy`

Expected: PASS.

**Step 2: Run implementation review**

Run the repo-local review flow from the worktree root:
- `python3 scripts/pipeline_skill_context.py review-implementation --repo-root . --format text`
- Follow [`.claude/skills/review-implementation/SKILL.md`](/Users/jinguomini/rcode/problem-reductions/.worktrees/issue-503-scheduling-with-individual-deadlines/.claude/skills/review-implementation/SKILL.md) to address structural and quality findings before the batch commit.

## Batch 2

### Task 6: Add the paper entry and paper-example test

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/misc/scheduling_with_individual_deadlines.rs`

**Step 1: Add the Typst entry**

Add:
- display-name dictionary entry for `SchedulingWithIndividualDeadlines`
- `problem-def("SchedulingWithIndividualDeadlines")` with the formal definition from issue `#503`
- background paragraph citing Garey & Johnson and the in-tree/out-tree complexity distinction
- worked example built from the canonical 7-task instance and feasible schedule `[0, 0, 0, 1, 2, 1, 1]`

Prefer a schedule-style figure or concise textual walkthrough; the key requirement is that the example and evaluation match the canonical model example and unit test exactly.

**Step 2: Add/finish the paper-example unit test**

Assert the paper's instance and schedule evaluate to `true`, and brute-force finds at least one satisfying schedule for that instance.

**Step 3: Verify the paper build**

Run: `make paper`

Expected: PASS.

### Task 7: Final verification, commit, and PR summary inputs

**Files:**
- Verify current diff only

**Step 1: Final verification**

Run:
- `make test`
- `make clippy`
- `make paper`

Expected: all PASS.

**Step 2: Prepare commit set**

Create the implementation commit after verification, then remove the plan file in the follow-up cleanup commit required by the `issue-to-pr` skill.

**Step 3: Capture PR summary notes**

Record:
- files added/modified
- any deviations from the plan
- any open questions for reviewers
