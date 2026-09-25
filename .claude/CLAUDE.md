# CLAUDE.md

## Project Overview
Rust library for NP-hard problem reductions. Implements computational problems with reduction rules for transforming between equivalent formulations.

## Philosophy
- **Simple logic, maximum reuse.** Prefer straightforward code with fewer branches (less if-else).
  Try best to reuse existing logic rather than adding ad hoc special cases.
- **Root-cause fixes over patches.** When a bug surfaces, trace it to its origin.
  A fix that prevents a class of bugs is better than one that handles a single case.
- **Tests over implementation.** Spend more time designing tests than implementing code.
  Well-designed tests catch bugs early and document intended behavior.

## Skills
Repo-local skills live under `.claude/skills/*/SKILL.md`; any agent can read and follow them. There is no project board or pipeline: agents pick up the `how-to-*` guides automatically while working, and humans merge.

Guides (auto-invoked):
- [how-to-code](skills/how-to-code/SKILL.md) -- Implement or modify a problem model or reduction rule.
- [how-to-verify](skills/how-to-verify/SKILL.md) -- Certify a reduction (type gate, Typst proof, constructor + adversary checks, PR verification certificate) and check reduction-graph topology.
- [how-to-write-manual](skills/how-to-write-manual/SKILL.md) -- Write or audit Typst manual entries (`docs/paper/reductions.typ`) and mdBook docs.
- [how-to-review](skills/how-to-review/SKILL.md) -- Fresh-context PR review: structural, quality, and `pred` feature test; posts an Agentic Review Report.
- [how-to-triage-issue](skills/how-to-triage-issue/SKILL.md) -- Quality-check `[Model]`/`[Rule]` issues, label them, fix mechanical problems, discuss substantive ones.
- [how-to-ship](skills/how-to-ship/SKILL.md) -- Issue to merge-ready PR: branch, gates, review subagent, CI/comments/codecov fixes. One item per PR, except a `[Model]` claiming direct ILP solvability ships its `<Model> -> ILP` rule too.

Tools (invoked on request):
- [propose](skills/propose/SKILL.md) -- Help a domain expert turn an idea into a well-formed model/rule issue.
- [find-solver](skills/find-solver/SKILL.md) -- Match a real-world problem to a model, route, and solver; writes a doc to `docs/solutions/`.
- [find-problem](skills/find-problem/SKILL.md) -- Given a solver for a model, find source problems it handles via incoming reductions.
- [dev-setup](skills/dev-setup/SKILL.md) -- Install and configure development tools.
- [release](skills/release/SKILL.md) -- Guarded crate release via `make release`.
- [update-papers](skills/update-papers/SKILL.md) -- Refresh the research paper collection.

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
make paper          # Generate example data and build the Typst paper
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
make copilot-review # (Optional) Request Copilot code review on current PR
make release V=x.y.z  # Tag and push a new release (CI publishes to crates.io)
make papers        # Full paper fetch: lookup + download + scihub
make papers-status # Show research paper collection stats
make papers-push   # Push PDFs to shared remote (requires rclone + PAPERS_REMOTE)
make papers-pull   # Pull PDFs from shared remote
# Set PAPERS_REMOTE=gdrive:folder for paper sync (requires rclone)
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
- `src/models/decision.rs` - Generic `Decision<P>` wrapper converting optimization problems to decision problems
- `src/solvers/` - BruteForce reference solver returning problem solutions, ILP solver (feature-gated), decision search (binary search via Decision queries), and the exact-variant solver capability registry. Solver dispatch uses only registered customized implementations and fixed ILP pipelines; reduction-graph reachability does not imply solver availability. Run `pred inspect <instance>` to see the registered capabilities for that instance.
- `src/traits.rs` - `Problem` trait
- `src/rules/traits.rs` - `ReduceTo<T>`, `ReduceToAggregate<T>`, `ReductionResult`, `AggregateReductionResult` traits
- `src/registry/` - Compile-time reduction metadata collection
- `problemreductions-cli/` - `pred` CLI tool (separate crate in workspace)
- `src/unit_tests/` - Unit test files (mirroring `src/` structure, referenced via `#[path]`)
- `tests/main.rs` - Integration tests (modules in `tests/suites/`); example tests use `include!` for direct invocation (no subprocess)
- `tests/data/` - Ground truth JSON for integration tests
- `scripts/` - Python test data generation scripts (managed with `uv`)

### Trait Hierarchy

```
Problem (core trait — all problems must implement)
│
├── const NAME: &'static str           // e.g., "MaximumIndependentSet"
├── type Solution                      // mathematical witness representation
├── type Value: Clone                  // per-solution evaluation value
├── fn parameter_names()               // canonical problem-owned parameter schema
├── fn parameters(&self) -> ProblemParameters // concrete instance parameter values
├── fn evaluate(&self, solution) -> Result<Value, EvaluationError>
├── fn variant() -> Vec<(&str, &str)>  // e.g., [("graph","SimpleGraph"), ("weight","i64")]
└── fn problem_type() -> ProblemType   // catalog bridge: registry lookup by NAME
```

`BruteForceProblem` is a separate reference-solver capability. Its
`dimensions()` method describes only the finite Cartesian coordinate space used
by the registered brute-force implementation.

**Objective problems** (e.g., `MaximumIndependentSet`) typically use `Value = Max<W::Sum>`, `Min<W::Sum>`, or `Extremum<W::Sum>`.

**Feasibility problems** (e.g., `Satisfiability`) typically use `Value = Or`.

A successful `Problem` solve always returns `Problem::Solution`. Global counting
or statistics without a representative solution are not modeled as `Problem`
solves.

**Decision problems** wrap an optimization problem with a bound: `Decision<P>` where `P::Value: OptimizationValue`. Evaluates to `Or(true)` when the inner objective meets the bound (≤ for Min, ≥ for Max).

Common aggregate wrappers live in `src/types.rs`:
```rust
Max<V>, Min<V>, Sum<W>, Or, And, Extremum<V>, ExtremumSense
```

`OptimizationValue` trait (in `src/types.rs`) enables generic Decision conversion:
- `Min<V>`: meets bound when value ≤ bound
- `Max<V>`: meets bound when value ≥ bound

### Key Patterns
- Keep failure phases typed and separate: public construction paths return `ConstructionError`, `Problem::evaluate()` returns `EvaluationError`, and reduction paths return `ReductionError`. A reduction preserves target construction failures as `ReductionError::Construction`; none of these paths returns or creates an error as a bare `String`.
- `variant_params!` macro implements `Problem::variant()` — e.g., `crate::variant_params![G, W]` for two type params, `crate::variant_params![]` for none (see `src/variant.rs`)
- `declare_variants!` proc macro registers concrete type instantiations with best-known complexity and registry-backed load/serialize/solution-solve metadata. One entry per problem may be marked `default`, and variable names in complexity strings are validated against the problem-owned parameter schema. Ordinary models are constructed directly from their construction schema. When user-facing construction differs from persisted JSON, define a model-local `#[derive(CreateSpec)]` DTO plus `TryFrom<CreateSpec>`, use its generated `FIELDS` in `ProblemSchemaEntry`, and register it with `create LocalSpec`; never add model-name branches in CLI or MCP code.
- `decision_problem_meta!` macro registers `DecisionProblemMeta` for a concrete inner type, providing the `DECISION_NAME` constant.
- `register_decision_variant!` macro generates `declare_variants!`, `ProblemSchemaEntry`, and both `ReductionEntry` submissions (aggregate Decision→Opt + Turing Opt→Decision) for a `Decision<P>` variant. Callers must define inherent getters (`num_vertices()`, `num_edges()`, `k()`) on `Decision<P>` before invoking. Accepts an explicit structural `category` plus `dims`, `fields`, and `parameter_getters` parameters for problem-specific parameters.
- Problems parameterized by graph type `G` and optionally weight type `W` (problem-dependent)
- `BruteForce::solve()` returns `Result<Option<P::Solution>, SolveError>`; `None` means exhaustive search proved infeasibility
- `BruteForce::find_all_witnesses()` is a reference-testing helper for collecting every optimal or satisfying solution
- `ReductionResult` provides `target_problem()` and `extract_solution()` for witness/config workflows; `AggregateReductionResult` provides `extract_value()` for aggregate/value workflows. Neither requires a rule-category tag. When both are registered, completed-result recovery borrows both mappings from the same constructed reduction.
- Register a completed-value mapping with `#[aggregate_reduction]` on its concrete `AggregateReductionResult` implementation. Generic implementations use `register_aggregate_reduction!(ResultType)` for each concrete result type. These register implementations, not rule categories. Read resolved edges through `reduction_entries()`, not raw inventory entries.
- Reduction chains expose solution and aggregate-value mappings, not solver outcomes. Every witness reduction preserves existence: source feasibility implies target feasibility. Established target or intermediate infeasibility propagates to the source without a value map or witness extraction. CLI execution coordinates mappings for feasible target results; callers establish optimality or infeasibility under their solver's numerical contract. A missing required mapping or failed witness extraction is an error, not proof of infeasibility. Counting and universal aggregates use `AggregateReductionChain::extract_value()` without a representative witness.
- Every direct `extract_solution()` must call `validate_target_solution()` once before decoding; composed extractors delegate validation to the first direct decoder.
- Decision-equivalence rules map completed `Or` values identically. Decision-to-optimization rules own their feasibility/threshold map; reject target configurations that do not certify YES instead of returning an invalid source witness. Optimization rules decode optimal witnesses and evaluate the source; register a value map only when mathematically defined. Counting and universal rules map completed folds without witnesses. Follow [result mappings](../docs/src/design.md#result-mappings); no mandatory rule-category tags.
- Decode only the reduction's defined mathematical mapping. Reject malformed structure with `ExtractionError`; never panic, truncate, clamp, invent defaults, or add recovery branches. Explicit mathematical alternatives and sentinels are allowed. Test successful decoding and every rejected representation.
- CLI-facing dynamic formatting uses aggregate wrapper names directly (for example `Max(2)`, `Min(None)`, `Or(true)`, or `Sum(56)`)
- Graph types: SimpleGraph, PlanarGraph, BipartiteGraph, UnitDiskGraph, KingsSubgraph, TriangularSubgraph
- Weight types: `One` (unit weight marker), `i64`, `f64` — all implement `WeightElement` trait
- `WeightElement` trait: `type Sum: NumericSize` + `fn to_sum(&self)` — converts weight to a summable numeric type
- Weight management via inherent methods (`weights()`, `set_weights()`, `is_weighted()`), not traits
- `NumericSize` supertrait bundles common numeric bounds (`Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static`)

### Parameter Relations
Each reduction declares one rule-level parameter relation using the `Expr` AST in `src/expr.rs`. The `transform` declaration is required:
```rust
#[reduction(transform = upper_bound {
    num_vertices = "num_vertices + num_clauses",
    num_edges = "3 * num_clauses",
})]
impl ReduceTo<Target> for Source { ... }
```
- Expression strings are parsed at compile time by a Pratt parser in the proc macro crate
- Variable names are validated against the source problem's canonical parameter schema
- Use `transform = exact { ... }` when every formula is an equality and `transform = upper_bound { ... }` when every formula is only an upper bound. One expression block cannot mix relations.
- Use `transform = unavailable { ... }` when no formula is representable, or an auxiliary `unavailable = { ... }` block for omitted target parameters.
- Every target parameter must appear exactly once as a formula or as unavailable with a non-empty reason.
- `ParameterTransform` evaluates and composes formulas with exact rational and arbitrary-precision integer arithmetic. Unsafe upper-bound composition becomes unavailable; it never performs budget pruning or path ranking.
- Concrete instance parameters come from each endpoint instance's `Problem::parameters()` implementation; `ReductionEntry` stores only the symbolic parameter relation.
- `VariantEntry` has both a complexity string and compiled `complexity_eval_fn` — same pattern
- Expressions support: constants, variables, `+`, `-`, `*`, `/`, `^`, `exp()`, `log()`, `sqrt()`, `factorial()`
- Complexity strings must use **concrete numeric values only** (e.g., `"2^(2.372 * num_vertices / 3)"`, not `"2^(omega * num_vertices / 3)"`)
- `Expr::parse()` provides runtime parsing for cross-check tests that compare compiled vs symbolic evaluation

### Problem Names
Problem types use explicit optimization prefixes (`Maximum...`, `Minimum...`) or no prefix. Run `pred list` for the full catalog. Common aliases (e.g., `MIS` → `MaximumIndependentSet`, `MVC` → `MinimumVertexCover`) are shown in the `Aliases` column.

### Problem Variants
Reduction graph nodes use variant key-value pairs from `Problem::variant()`:
- Default: `MaximumIndependentSet {graph: "SimpleGraph", weight: "One"}`
- Graph variant: `MaximumIndependentSet {graph: "KingsSubgraph", weight: "One"}`
- Weight variant: `MaximumIndependentSet {graph: "SimpleGraph", weight: "i64"}`
- Each problem declares one default concrete variant through `declare_variants!`; variant listings place that declaration first
- Nodes come from concrete `declare_variants!` registrations
- Same-name variant relations are explicit `#[reduction]` registrations
- Each primitive reduction is determined by the exact `(source_variant, target_variant)` endpoint pair
- Reduction edges carry `EdgeCapabilities { witness, aggregate, turing }`; graph search defaults to witness mode, aggregate mode is available through `ReductionMode::Aggregate`, and Turing (multi-query) mode via `ReductionMode::Turing`
- `#[reduction]` requires one `transform = exact`, `transform = upper_bound`, or `transform = unavailable` declaration and currently registers witness/config reductions; aggregate-only and Turing edges require manual `ReductionEntry` registration
- `Decision<P> → P` supports both mappings: compare the exact optimum to the bound, and recover a witness only if it meets the bound. `P → Decision<P>` is a Turing edge (binary search over decision bound).

### Extension Points
- New models register dynamic load/serialize metadata through `declare_variants!` and, when finite enumeration exists, register it separately through `register_brute_force!`; neither belongs in CLI match arms
- **Model category is explicit registry metadata.** Every `ProblemSchemaEntry` declares exactly one of `Algebraic`, `Formula`, `Graph`, `Misc`, or `Set`; catalog behavior never derives it from `module_path!()` or source location.
- **CLI creation is registry-driven and two-stage:** the static parser discovers the requested problem spec without registering model subcommands, then a second parse adds flags only for the selected concrete variant. Ordinary models use `ProblemSchemaEntry.fields` directly. Models whose construction differs from persisted JSON own a typed `CreateSpec` and fallible conversion beside the model; CLI and MCP only normalize transport values and invoke the registered constructor.
- **Each construction input has one name and one concrete type per variant.** Do not add compatibility aliases or infer types from flag names. `CreateSpec` field names render as `snake_case → kebab-case` in CLI and remain `snake_case` in MCP. Add a reusable codec only for a genuinely new transport representation, never a model-name parser branch.
- **Random generation is optional and variant-owned.** Not every model has a useful, well-defined random-instance distribution. Add `RandomGenerate` only when the generator has clear semantics and a concrete use (for example, testing or examples); never invent arbitrary bounds or distributions merely to make every model support `--random`. Implement it beside the model (normally through `impl_random_generate!` and a typed `CreateSpec` input DTO), then add `random` only to the applicable `declare_variants!` entries. CLI and MCP discover the exact variant's inputs and callback; never add a model-name random dispatch or advertise random generation on an unsupported variant.
- **Decision variants** of optimization problems use `Decision<P>` wrapper. Add via: (1) `decision_problem_meta!` for the inner type, (2) inherent methods on `Decision<Inner>`, (3) `register_decision_variant!` with `dims`, `fields`, `parameter_getters`. The generated construction spec accepts flat inner fields plus `bound`; persisted JSON remains `{inner: {...}, bound}`. `Decision<P>` delegates canonical parameters to `P`; its objective bound is semantic instance data, not a problem parameter.
- Aggregate-only and Turing reduction edges still need manual `ReductionEntry` wiring because `#[reduction]` only registers solution-mapping reductions today; this edge capability does not imply that a problem may solve successfully without a `Solution`
- Exact registry dispatch lives in `src/registry/`; alias resolution and partial/default variant resolution live in `problemreductions-cli/src/problem_name.rs`
- `pred create` schema-driven dispatch lives in `problemreductions-cli/src/commands/create.rs` (`create_schema_driven()`)
- Canonical model examples live beside each model in `canonical_model_example_specs()` (collected by `src/example_db/model_builders.rs`); rule examples live beside their rules and are collected by `src/rules/mod.rs`

## Conventions

### Numeric Contract

Follow the [numeric types and arithmetic standard](../docs/src/design.md#numeric-types-and-arithmetic)
for every model and reduction. `usize` is reserved for in-memory indices,
collection lengths, and brute-force dimensions; canonical problem parameters
are `u64`; signed mathematical integers use `i64`; and approximate
real values use finite `f64`. Before implementation, identify each numeric
input and domain, each computed total and result type, the largest supported
value, every range/sign-changing conversion, overflow behavior, and whether
arithmetic is exact or approximate. Use `TryFrom` at range boundaries and
checked arithmetic for derived values that may overflow. Rust construction,
serde, CLI, and MCP must enforce the same range.

Issue contributors provide the mathematical definition, domains, and
constraints; implementers derive the Rust representation. Do not require issue
authors to choose implementation types or add implementation-specific numeric
fields to issue templates. Changes to issue templates require user approval.

### File Naming
- Reduction files: `src/rules/<source>_<target>.rs` (e.g., `maximumindependentset_qubo.rs`)
- Model files: `src/models/<category>/<name>.rs` — category is by input structure: `graph/` (graph input), `formula/` (boolean formula/circuit), `set/` (universe + subsets), `algebraic/` (matrix/linear system/lattice), `misc/` (other)
- Canonical examples: model-local `canonical_model_example_specs()` functions collected by `src/example_db/model_builders.rs`; rule-local `canonical_rule_example_specs()` functions collected by `src/rules/mod.rs`
- Example binaries in `examples/`: utility/export tools and pedagogical demos only (not per-reduction files)
- Test naming: `test_<source>_to_<target>_closed_loop`

### Paper (docs/paper/reductions.typ)
- `problem-def(name)[def][body]` — defines a problem with auto-generated schema, reductions list, and label `<def:ProblemName>`. Title comes from `display-name` dict.
- `reduction-rule(source, target, example: bool, ...)[rule][proof]` — generates a theorem with label `<thm:Source-to-Target>` and registers in `covered-rules` state. Overhead auto-derived from JSON edge data.
- Every directed reduction needs its own `reduction-rule` entry (except trivial Decision↔Optimization pairs which are auto-filtered)
- Completeness warnings auto-check that all JSON graph nodes/edges are covered in the paper; `Decision<P> ↔ P` edges are excluded since they are trivial solve-and-compare reductions
- `display-name` dict maps `ProblemName` to display text

## Testing Requirements

**No single test should take more than 5 seconds.** If a test requires solving a large instance (e.g., ILP with thousands of variables), use a smaller test instance or a faster solver. Tests that exceed 5s block CI and must be refactored.

**Reference implementations — read these first:**
- **Reduction test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs` — closed-loop pattern
- **Model test:** `src/unit_tests/models/graph/maximum_independent_set.rs` — evaluation, serialization
- **Solver test:** `src/unit_tests/solvers/brute_force.rs` — solution-returning `solve()` plus all-solution helpers
- **Core definitions:** `src/traits.rs` (`Problem`), `src/solvers/brute_force.rs` (`BruteForce`)

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
- `src/example_db/` — Model builders, shared example specs, and rule-example aggregation consumed by `pred create --example` and paper exports
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

### Reduction Parameter Relation (`#[reduction(transform = exact|upper_bound {...})]`)
Parameter expressions describe how target problem parameters relate to source problem parameters. To verify correctness:
1. Read the `reduce_to()` implementation and count the actual output sizes
2. Check that each field (e.g., `num_vertices`, `num_edges`, `num_sets`) matches the constructed target problem
3. Watch for common errors: universe elements mismatch (edge indices vs vertex indices), worst-case edge counts in intersection graphs (quadratic, not linear), constant factors in circuit constructions
4. Test with concrete small instances: construct a source problem, run the reduction, and compare target parameters against the formula
5. Ensure there is only one primitive reduction registration for each exact source/target variant pair; wrap shared helpers instead of registering duplicate endpoints
