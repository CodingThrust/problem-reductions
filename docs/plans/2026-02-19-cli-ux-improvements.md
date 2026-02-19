# CLI UX Improvements

Systematic analysis of pitfalls, missing features, and HCI violations.
Edit this file to approve/reject/modify each item before implementation.

Status legend: `[x]` = approved, `[ ]` = pending, `[-]` = rejected

---

## Pitfalls

### P1. `create` without `-o` discards data
- [x] approve

**Problem:** `pred create MIS --edges 0-1,1-2` prints `"Created MaximumIndependentSet instance"` but the JSON data is lost. Every other command without `-o` shows useful output. Here the user created something and got nothing back.

**Proposed fix:** Print the problem JSON to stdout when `-o` is not given (consistent with `reduce`'s behavior of printing JSON to stdout).

---

### P2. `reduce` without `-o` outputs raw JSON (inconsistent)
- [x] approve

**Problem:** Every other command without `-o` shows human-readable text. But `reduce` dumps raw JSON to stdout. This breaks output mode consistency.

**Proposed fix:** Show human-readable summary by default (source, target, steps). Print JSON only when `-o` is given. If users need raw JSON to stdout, add a `--json` flag.

**Comment**: We should allow commands to take JSON inputs from CLI directly to make json output more useful.

---

### P3. `-o` means directory for `path --all`
- [ ] approve

**Problem:** With `--all`, `-o` is treated as a directory and creates `path_1.json`, `path_2.json`, etc. Every other command treats `-o` as a single file.

**Proposed fix:** When `--all` is used, always write a single JSON file containing an array of all paths. Drop the directory behavior.

**Comment**: What if a user want to specify a specific reduction path?

---

### P4. `export-graph` uses positional arg instead of `-o`
- [x] approve

**Problem:** Every other command uses `-o` for output files. `export-graph reduction_graph.json` uses a positional arg. Inconsistent.

**Proposed fix:** Change to `pred export-graph -o reduction_graph.json` or `pred export-graph` (defaults to stdout like other commands).

---

### P5. `solve` hint prints every time
- [x] approve

**Problem:** `"Hint: use -o to save full solution details as JSON."` prints on every invocation without `-o`. Annoying for experienced users and noisy for scripts.

**Proposed fix:** Only show hint when stderr is a TTY (same check as color). Scripts piping stderr won't see it, interactive users still get it.

---

### P6. `solve` ILP auto-reduction is invisible in human output
- [x] approve

**Problem:** When solving MIS with ILP, the output says `"Solver: ilp"` but doesn't mention it was auto-reduced to ILP. Only the JSON (with `-o`) shows `"reduced_to": "ILP"`. The user has no indication a reduction happened.

**Proposed fix:** Show `"Solver: ilp (via ILP)"` or `"Solver: ilp (auto-reduced to ILP)"` in the human text output when auto-reduction occurs.

---

### P7. No stdin/pipe support
- [x] approve

**Problem:** `solve`, `evaluate`, `reduce` all require file paths. The Unix idiom of `cmd1 | cmd2` doesn't work. Users must always create intermediate files.

**Proposed fix:** Accept `-` as input to read from stdin: `pred create MIS --edges 0-1,1-2 | pred solve -`. This is a standard Unix convention.

---

### P8. `create Factoring` is a dead end
- [ ] approve

**Problem:** `pred create Factoring` gives `"Factoring requires complex construction — use a JSON file instead"` but doesn't say what the JSON format should be.

**Proposed fix:** Add a `pred show Factoring` schema reference in the error message: `"See pred show Factoring for the expected JSON format, or check the documentation."`.

**Comment**: We should allow users to create complex problems from CLI directly.

---

## Features

### F1. `pred inspect <file>`
- [x] approve

**Problem:** Given a problem JSON or bundle, users must manually read the JSON to know what's inside.

**Proposed feature:** Show what type it is, its size (number of variables, edges, etc.), variant, available solvers, and possible reductions. Example:
```
$ pred inspect problem.json
Type: MaximumIndependentSet {graph=SimpleGraph, weight=i32}
Size: 5 vertices, 5 edges
Solvers: ilp (default), brute-force
Reduces to: ILP, MinimumVertexCover, QUBO, MaximumSetPacking
```

---

### F2. Quiet mode (`-q`)
- [x] approve

**Problem:** No way to suppress hints and informational stderr messages for scripting.

**Proposed feature:** Add `-q` / `--quiet` global flag that suppresses hints and informational messages on stderr. Only errors go to stderr in quiet mode.

---

### F3. `pred solve --reduce-via <target>`
- [ ] approve

**Problem:** Solving via a specific reduction requires two commands and an intermediate file: `pred reduce ... -o bundle.json && pred solve bundle.json`.

**Proposed feature:** `pred solve problem.json --reduce-via QUBO --solver brute-force` combines reduce + solve in one step, avoiding the intermediate file.

**Comment**: Reject, automated find path finding is tricky, not useful to automate at the current stage.

---

### F4. `pred create --random`
- [x] approve

**Problem:** No way to generate random problem instances for benchmarking or testing.

**Proposed feature:** `pred create MIS --random --num-vertices 100 --edge-prob 0.3 -o big.json`. Support random generation for graph-based problems with configurable size and density.

---

## HCI Violations

### H1. Inconsistent error guidance
- [x] approve

**Problem:** Some errors give excellent guidance (`path` no-path-found shows `pred show` hints), while others are bare (`show Foobar` gives just `"Error: Unknown problem: Foobar"` without suggesting `pred list` or fuzzy matches).

**Proposed fix:** Add `"Did you mean ...?"` fuzzy matching for unknown problem names. Always suggest `pred list` when a problem name is not recognized.

---

### H2. `--direction` is a raw string, not a clap enum
- [x] approve

**Problem:** `--direction` accepts any string and validates at runtime. Invalid values give a custom error, but clap's built-in validation (with auto-completion and help listing) would be better.

**Comment**: Redesign. Maybe use `pred from MIS --hops 2` and `pred to MIS --hops 2` to make it more clear.

**Resolution:** Replaced `pred show --hops --direction` with dedicated `pred to` and `pred from` subcommands. Direction is now determined by the command itself (no runtime string validation). Tree output shows variant-level information (`ProblemName {key=val}`).

---

### H3. No progress feedback for long operations
- [ ] approve

**Problem:** Brute-force on large instances or multi-step reductions give no feedback until completion. The user doesn't know if the tool is working or stuck.

**Proposed fix:** Show a brief progress line on stderr for brute-force (e.g., `"Exploring 2^20 configurations..."`) and for multi-step reductions (e.g., `"Step 1/3: MIS → MVC..."`).

**Comment**: Not very useful. Consider allowing users to add a time limit for job.

---
