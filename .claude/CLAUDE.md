# CLAUDE.md

## Project Overview
Rust library for NP-hard problem reductions. Implements computational problems with reduction rules for transforming between equivalent formulations.

## Skills
These repo-local skills live under `.claude/skills/*/SKILL.md`.

- [run-pipeline](skills/run-pipeline/SKILL.md) -- Pick a Ready issue from the GitHub Project board, move it through In Progress -> issue-to-pr -> Review pool. One issue at a time; forever-loop handles iteration.
- [issue-to-pr](skills/issue-to-pr/SKILL.md) -- Convert a GitHub issue into a PR with an implementation plan. Default rule: one item per PR. Exception: a `[Model]` issue that explicitly claims direct ILP solvability should implement the model and its direct `<Model> -> ILP` rule together; `[Rule]` issues still require both models to exist on `main`.
- [add-model](skills/add-model/SKILL.md) -- Add a new problem model. Can be used standalone (brainstorms with user) or called from `issue-to-pr`.
- [add-rule](skills/add-rule/SKILL.md) -- Add a new reduction rule. Can be used standalone (brainstorms with user) or called from `issue-to-pr`.
- [review-structural](skills/review-structural/SKILL.md) -- Project-specific structural completeness check: model/rule checklists, build, semantic correctness, issue compliance. Read-only, no code changes. Called by `review-pipeline`.
- [review-quality](skills/review-quality/SKILL.md) -- Generic code quality review: DRY, KISS, cohesion/coupling, test quality, HCI. Read-only, no code changes. Called by `review-pipeline`.
- [fix-pr](skills/fix-pr/SKILL.md) -- Resolve PR review comments, fix CI failures, and address codecov coverage gaps. Uses `gh api` for codecov (not local `cargo-llvm-cov`).
- [write-model-in-paper](skills/write-model-in-paper/SKILL.md) -- Write or improve a problem-def entry in the Typst paper (standalone, for improving existing entries). Core instructions are inlined in `add-model` Step 6.
- [write-rule-in-paper](skills/write-rule-in-paper/SKILL.md) -- Write or improve a reduction-rule entry in the Typst paper (standalone, for improving existing entries). Core instructions are inlined in `add-rule` Step 5.
- [release](skills/release/SKILL.md) -- Create a new crate release. Determines version bump from diff, verifies tests/clippy, then runs `make release`.
- [check-issue](skills/check-issue/SKILL.md) -- Quality gate for `[Rule]` and `[Model]` issues. Checks usefulness, non-triviality, correctness of literature, and writing quality. Posts structured report and adds failure labels.
- [fix-issue](skills/fix-issue/SKILL.md) -- Fix quality issues found by check-issue — auto-fixes mechanical problems, brainstorms substantive issues with human, then re-checks and moves to Ready.
- [topology-sanity-check](skills/topology-sanity-check/SKILL.md) -- Run sanity checks on the reduction graph: detect orphan (isolated) problems and redundant reduction rules.
  - `topology-sanity-check orphans` -- Detect isolated problem types (runs `examples/detect_isolated_problems.rs`)
  - `topology-sanity-check np-hardness` -- Verify NP-hardness proof chains from 3-SAT (runs `examples/detect_unreachable_from_3sat.rs`)
  - `topology-sanity-check redundancy [source target]` -- Check for dominated reduction rules
- [review-pipeline](skills/review-pipeline/SKILL.md) -- Agentic review for PRs in Review pool: runs structural check, quality check, and agentic feature tests (no code changes), posts combined verdict, always moves to Final review.
- [propose](skills/propose/SKILL.md) -- Interactive brainstorming to help domain experts propose a new model or rule. Asks one question at a time, uses mathematical language (no programming jargon), and files a GitHub issue.
- [final-review](skills/final-review/SKILL.md) -- Interactive maintainer review for PRs in "Final review" column. Merges main, walks through agentic review bullets with human, then merge or hold.
- [dev-setup](skills/dev-setup/SKILL.md) -- Interactive wizard to install and configure all development tools for new maintainers.
- [tutorial](skills/tutorial/SKILL.md) -- Interactive tutorial — walk through the pred CLI to explore, reduce, and solve NP-hard problems. No Rust internals.

## Codex Compatibility
- Claude slash commands such as `/issue-to-pr 42 --execute` are aliases for the matching repo-local skill files under `.claude/skills/`.
- In Codex, read the relevant `SKILL.md` directly and follow it; do not assume slash-command support exists.
- The Makefile targets `run-plan`, `run-issue`, `run-pipeline`, and `run-review` already translate these workflows into explicit `SKILL.md` prompts for Codex.
- The default Codex model in the Makefile is `gpt-5.4`. Override it with `CODEX_MODEL=<model>` if needed.
- The Step 0/Step 1 packet builders under `scripts/pipeline_skill_context.py` and `scripts/pipeline_checks.py` are expensive GitHub-backed calls. Per top-level skill invocation, generate each packet at most once and reuse the resulting text/JSON for all later steps unless the skill explicitly requires a fresh rerun.

## Commands
```bash
make help           # Show all available targets
make build          # Build the project
make test           # Run all tests
make fmt            # Format code with rustfmt
make fmt-check      # Check code formatting
make clippy         # Run clippy lints
make doc            # Build mdBook documentation (includes reduction graph export)
make mdbook         # Build and serve mdBook with live reload
make paper          # Build Typst paper from checked-in example fixtures
make coverage       # Generate coverage report (>95% required)
make check          # Quick pre-commit check (fmt + clippy + test)
make rust-export    # Generate Julia parity test data (mapping stages)
make export-schemas # Regenerate problem schemas JSON
make qubo-testdata  # Regenerate QUBO ground truth JSON
make clean          # Clean build artifacts
make diagrams      # Generate SVG diagrams from Typst (light + dark)
make compare       # Generate and compare Rust mapping exports
make jl-testdata   # Regenerate Julia parity test data (requires julia)
make cli           # Build the pred CLI tool (without MCP, fast)
make mcp           # Build the pred CLI tool with MCP server support
make cli-demo      # Run closed-loop CLI demo (exercises all commands)
make mcp-test      # Run MCP server tests (unit + integration)
make run-plan      # Execute a plan with Codex or Claude
make run-issue N=42 # Run issue-to-pr --execute for a GitHub issue
make run-pipeline  # Pick next Ready issue from project board, implement, move to Review pool
make run-pipeline N=97 # Process a specific issue from the project board
make run-pipeline-forever # Poll Ready column, run-pipeline when new issues appear
make run-review    # Pick next PR from Review pool column, run agentic review, move to Final review
make run-review N=570 # Process a specific PR from the Review pool column
make run-review-forever # Poll Review pool for eligible PRs, dispatch run-review
make copilot-review # (Optional) Request Copilot code review on current PR
make release V=x.y.z  # Tag and push a new release (CI publishes to crates.io)
# Set RUNNER=claude to use Claude instead of Codex (default: codex)
# Default Codex model: CODEX_MODEL=gpt-5.4
```

## Git Safety
- **NEVER force push** (`git push --force`, `git push -f`, `git push --force-with-lease`). This is an absolute rule with no exceptions. Force push can silently destroy other people's work and stashed changes.

## Architecture

### Core Modules
- `src/models/` - Problem implementations organized by input structure:
  - `graph/` - Graph-input problems
  - `formula/` - Boolean formulas and circuits
  - `set/` - Set systems (universe + subsets)
  - `algebraic/` - Matrices, linear systems, lattices
  - `misc/` - Unique input structures
  - Run `pred list` for the full catalog of problems, variants, and reductions; `pred show <name>` for details on a specific problem
- `src/rules/` - Reduction rules + inventory registration
- `src/solvers/` - BruteForce solver for aggregate values plus witness recovery when supported, ILP solver (feature-gated, witness-only). To check if a problem supports ILP solving via a witness-capable reduction path, run `pred path <ProblemName> ILP`
- `src/traits.rs` - `Problem` trait
- `src/rules/traits.rs` - `ReduceTo<T>`, `ReduceToAggregate<T>`, `ReductionResult`, `AggregateReductionResult` traits
- `src/registry/` - Compile-time reduction metadata collection
- `problemreductions-cli/` - `pred` CLI tool (separate crate in workspace)
- `src/unit_tests/` - Unit test files (mirroring `src/` structure, referenced via `#[path]`)
- `tests/main.rs` - Integration tests (modules in `tests/suites/`); example tests use `include!` for direct invocation (no subprocess)
- `tests/data/` - Ground truth JSON for integration tests
- `scripts/` - Python test data generation scripts (managed with `uv`)
- `docs/plans/` - Implementation plans

### Trait Hierarchy

```
Problem (core trait — all problems must implement)
│
├── const NAME: &'static str           // e.g., "MaximumIndependentSet"
├── type Value: Clone                  // aggregate value: Max/Min/Sum/Or/And/Extremum/...
├── fn dims(&self) -> Vec<usize>       // config space: [2, 2, 2] for 3 binary variables
├── fn evaluate(&self, config) -> Value
├── fn variant() -> Vec<(&str, &str)>  // e.g., [("graph","SimpleGraph"), ("weight","i32")]
├── fn num_variables(&self) -> usize   // default: dims().len()
└── fn problem_type() -> ProblemType   // catalog bridge: registry lookup by NAME
```

**Witness-capable objective problems** (e.g., `MaximumIndependentSet`) typically use `Value = Max<W::Sum>`, `Min<W::Sum>`, or `Extremum<W::Sum>`.

**Witness-capable feasibility problems** (e.g., `Satisfiability`) typically use `Value = Or`.

**Aggregate-only problems** use fold values such as `Sum<W>` or `And`; these solve to a value but have no representative witness configuration.

Common aggregate wrappers live in `src/types.rs`:
```rust
Max<V>, Min<V>, Sum<W>, Or, And, Extremum<V>, ExtremumSense
```

### Key Patterns
- `variant_params!` macro implements `Problem::variant()` — e.g., `crate::variant_params![G, W]` for two type params, `crate::variant_params![]` for none (see `src/variant.rs`)
- `declare_variants!` proc macro registers concrete type instantiations with best-known complexity and registry-backed load/serialize/value-solve/witness-solve metadata. One entry per problem may be marked `default`, and variable names in complexity strings are validated at compile time against actual getter methods.
- Problems parameterized by graph type `G` and optionally weight type `W` (problem-dependent)
- `Solver::solve()` computes the aggregate value for any `Problem` whose `Value` implements `Aggregate`
- `BruteForce::find_witness()` / `find_all_witnesses()` recover witnesses only when `P::Value::supports_witnesses()`
- `ReductionResult` provides `target_problem()` and `extract_solution()` for witness/config workflows; `AggregateReductionResult` provides `extract_value()` for aggregate/value workflows
- CLI-facing dynamic formatting uses aggregate wrapper names directly (for example `Max(2)`, `Min(None)`, `Or(true)`, or `Sum(56)`)
- Graph types: SimpleGraph, PlanarGraph, BipartiteGraph, UnitDiskGraph, KingsSubgraph, TriangularSubgraph
- Weight types: `One` (unit weight marker), `i32`, `f64` — all implement `WeightElement` trait
- `WeightElement` trait: `type Sum: NumericSize` + `fn to_sum(&self)` — converts weight to a summable numeric type
- Weight management via inherent methods (`weights()`, `set_weights()`, `is_weighted()`), not traits
- `NumericSize` supertrait bundles common numeric bounds (`Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static`)

### Overhead System
Reduction overhead is expressed using `Expr` AST (in `src/expr.rs`) with the `#[reduction]` macro. The `overhead` attribute is **required** — omitting it is a compile error:
```rust
#[reduction(overhead = {
    num_vertices = "num_vertices + num_clauses",
    num_edges = "3 * num_clauses",
})]
impl ReduceTo<Target> for Source { ... }
```
- Expression strings are parsed at compile time by a Pratt parser in the proc macro crate
- Variable names are validated against actual getter methods on the source type — typos cause compile errors
- Each problem type provides inherent getter methods (e.g., `num_vertices()`, `num_edges()`) that the overhead expressions reference
- `ReductionOverhead` stores `Vec<(&'static str, Expr)>` — field name to symbolic expression mappings
- `ReductionEntry` has both symbolic (`overhead_fn`) and compiled (`overhead_eval_fn`) evaluation — the compiled version calls getters directly
- `VariantEntry` has both a complexity string and compiled `complexity_eval_fn` — same pattern
- Expressions support: constants, variables, `+`, `-`, `*`, `/`, `^`, `exp()`, `log()`, `sqrt()`, `factorial()`
- Complexity strings must use **concrete numeric values only** (e.g., `"2^(2.372 * num_vertices / 3)"`, not `"2^(omega * num_vertices / 3)"`)
- `Expr::parse()` provides runtime parsing for cross-check tests that compare compiled vs symbolic evaluation

### Problem Names
Problem types use explicit optimization prefixes (`Maximum...`, `Minimum...`) or no prefix. Run `pred list` for the full catalog. Common aliases (e.g., `MIS` → `MaximumIndependentSet`, `MVC` → `MinimumVertexCover`) are shown in the `Aliases` column.

### Problem Variants
Reduction graph nodes use variant key-value pairs from `Problem::variant()`:
- Base: `MaximumIndependentSet` (empty variant = defaults)
- Graph variant: `MaximumIndependentSet {graph: "KingsSubgraph", weight: "One"}`
- Weight variant: `MaximumIndependentSet {graph: "SimpleGraph", weight: "f64"}`
- Default variant ranking: `SimpleGraph`, `One`, `KN` are considered default values; variants with the most default values sort first
- Nodes come exclusively from `#[reduction]` registrations; natural edges between same-name variants are inferred from the graph/weight subtype partial order
- Each primitive reduction is determined by the exact `(source_variant, target_variant)` endpoint pair
- Reduction edges carry `EdgeCapabilities { witness, aggregate }`; graph search defaults to witness mode, and aggregate mode is available through `ReductionMode::Aggregate`
- `#[reduction]` accepts only `overhead = { ... }` and currently registers witness/config reductions; aggregate-only edges require manual `ReductionEntry` registration with `reduce_aggregate_fn`

### Extension Points
- New models register dynamic load/serialize/brute-force dispatch through `declare_variants!` in the model file, not by adding manual match arms in the CLI
- **CLI creation is schema-driven:** `pred create` automatically maps `ProblemSchemaEntry` fields to CLI flags via `snake_case → kebab-case` convention. New models need only: (1) matching CLI flags in `CreateArgs` + `flag_map()`, and (2) type parser support in `parse_field_value()` if using a new field type. No match arm in `create.rs` is needed.
- **CLI flag names must match schema field names.** The canonical name for a CLI flag is the schema field name in kebab-case (e.g., schema field `universe_size` → `--universe-size`, field `subsets` → `--subsets`). Old aliases (e.g., `--universe`, `--sets`) may exist as clap `alias` for backward compatibility at the clap level, but `flag_map()`, help text, error messages, and documentation must use the schema-derived name. Do not add new backward-compat aliases; if a field is renamed in the schema, update the CLI flag name to match.
- Aggregate-only models are first-class in `declare_variants!`; aggregate-only reduction edges still need manual `ReductionEntry` wiring because `#[reduction]` only registers witness/config reductions today
- Exact registry dispatch lives in `src/registry/`; alias resolution and partial/default variant resolution live in `problemreductions-cli/src/problem_name.rs`
- `pred create` schema-driven dispatch lives in `problemreductions-cli/src/commands/create.rs` (`create_schema_driven()`)
- Canonical paper and CLI examples live in `src/example_db/model_builders.rs` and `src/example_db/rule_builders.rs`

## Conventions

### File Naming
- Reduction files: `src/rules/<source>_<target>.rs` (e.g., `maximumindependentset_qubo.rs`)
- Model files: `src/models/<category>/<name>.rs` — category is by input structure: `graph/` (graph input), `formula/` (boolean formula/circuit), `set/` (universe + subsets), `algebraic/` (matrix/linear system/lattice), `misc/` (other)
- Canonical examples: builder functions in `src/example_db/rule_builders.rs` and `src/example_db/model_builders.rs`
- Example binaries in `examples/`: utility/export tools and pedagogical demos only (not per-reduction files)
- Test naming: `test_<source>_to_<target>_closed_loop`

### Paper (docs/paper/reductions.typ)
- `problem-def(name)[def][body]` — defines a problem with auto-generated schema, reductions list, and label `<def:ProblemName>`. Title comes from `display-name` dict.
- `reduction-rule(source, target, example: bool, ...)[rule][proof]` — generates a theorem with label `<thm:Source-to-Target>` and registers in `covered-rules` state. Overhead auto-derived from JSON edge data.
- Every directed reduction needs its own `reduction-rule` entry
- Completeness warnings auto-check that all JSON graph nodes/edges are covered in the paper
- `display-name` dict maps `ProblemName` to display text

## Testing Requirements

**Reference implementations — read these first:**
- **Reduction test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs` — closed-loop pattern
- **Model test:** `src/unit_tests/models/graph/maximum_independent_set.rs` — evaluation, serialization
- **Solver test:** `src/unit_tests/solvers/brute_force.rs` — aggregate `solve()` plus witness recovery helpers
- **Trait definitions:** `src/traits.rs` (`Problem`), `src/solvers/mod.rs` (`Solver`)

### Coverage

New code must have >95% test coverage. Run `make coverage` to check.

### Naming

- Reduction tests: `test_<source>_to_<target>_closed_loop`
- Model tests: descriptive names — e.g., `test_<model>_creation`, `test_<model>_evaluate_*`, `test_<model>_direction`, `test_<model>_solver`, `test_<model>_serialization`. Use whichever are relevant; there is no fixed per-model naming set.
- Solver tests: `test_<solver>_<problem>`

### Key Testing Patterns

See Key Patterns above for solver API signatures. Follow the reference files for exact usage.

### File Organization

Unit tests in `src/unit_tests/` linked via `#[path]` (see Core Modules above). Integration tests in `tests/suites/`, consolidated through `tests/main.rs`. Canonical example-db coverage lives in `src/unit_tests/example_db.rs`.

Model review automation checks for a dedicated test file under `src/unit_tests/models/...` with at least 3 test functions. The exact split of coverage is judged per model during review.

## Documentation Locations
- `README.md` — Project overview and quickstart
- `.claude/` — Claude Code instructions and skills
- `docs/book/` — mdBook user documentation (built with `make doc`)
- `docs/paper/reductions.typ` — Typst paper with problem definitions and reduction theorems
- `src/example_db/` — Canonical model/rule examples: `model_builders.rs`, `rule_builders.rs` (in-memory builders), `specs.rs` (per-module invariant specs), consumed by `pred create --example` and paper exports
- `examples/` — Export utilities, graph-analysis helpers, and pedagogical demos

## Documentation Requirements

**Reference:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet` to see a complete problem-def + reduction-rule example.

### Adding a Problem Definition

```typst
#problem-def("ProblemName")[
  Mathematical definition...
][
  Background, examples, algorithms...
]
```

Also add to the `display-name` dictionary:
```typst
"ProblemName": [Problem Name],
```

### Adding a Reduction Theorem

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [caption text],
)[
  Rule statement...
][
  Proof sketch...
]
```

Every directed reduction in the graph needs its own `reduction-rule` entry. The paper auto-checks completeness against the generated `reduction_graph.json` export.

## Complexity Verification Requirements

### Variant Worst-Case Complexity (`declare_variants!`)
The complexity string represents the **worst-case time complexity of the best known algorithm** for that problem variant. To verify correctness:
1. Identify the best known exact algorithm for the problem (name, author, year, citation)
2. Confirm the worst-case time bound from the original paper or a survey
3. Check that polynomial-time problems (e.g., MaximumMatching, 2-SAT, 2-Coloring) are NOT declared with exponential complexity
4. For NP-hard problems, verify the base of the exponential matches the literature (e.g., 1.1996^n for MIS, not 2^n)
5. Use only concrete numeric values — no symbolic constants (epsilon, omega); inline the actual numbers with citations
6. Variable names must match getter methods on the problem type (enforced at compile time)

### Reduction Overhead (`#[reduction(overhead = {...})]`)
Overhead expressions describe how target problem size relates to source problem size. To verify correctness:
1. Read the `reduce_to()` implementation and count the actual output sizes
2. Check that each field (e.g., `num_vertices`, `num_edges`, `num_sets`) matches the constructed target problem
3. Watch for common errors: universe elements mismatch (edge indices vs vertex indices), worst-case edge counts in intersection graphs (quadratic, not linear), constant factors in circuit constructions
4. Test with concrete small instances: construct a source problem, run the reduction, and compare target sizes against the formula
5. Ensure there is only one primitive reduction registration for each exact source/target variant pair; wrap shared helpers instead of registering duplicate endpoints
