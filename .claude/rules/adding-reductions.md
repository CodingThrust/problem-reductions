---
paths:
  - "src/rules/**/*.rs"
---

# Adding a Reduction Rule (A -> B)

**Reference implementations — read these first:**
- **Reduction rule:** `src/rules/minimumvertexcover_maximumindependentset.rs` — `ReductionResult` + `ReduceTo` + `#[reduction]` macro
- **Unit test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs` — closed-loop + edge cases
- **Example program:** `examples/reduction_minimumvertexcover_to_maximumindependentset.rs` — create, reduce, solve, extract, verify, export
- **Paper entry:** `docs/paper/reductions.typ` (search for `MinimumVertexCover` `MaximumIndependentSet`)
- **Traits:** `src/rules/traits.rs` — `ReductionResult` and `ReduceTo` trait definitions

## 0. Before Writing Code

1. **Brainstorm** — use `superpowers:brainstorming` to discuss with the user:
   - The math (variable mapping, constraint encoding, penalty terms)
   - Which example instance to use in `examples/` (must be small, human-explainable, and agreed with the user)
2. **Generate ground truth** — use Python scripts in `scripts/` (run with `uv`) to create test data in `tests/data/<target>/`.
3. **Write plan** — save to `docs/plans/` using `superpowers:writing-plans`.

## 1. Implement

Create `src/rules/<source>_<target>.rs` following the reference. Key pieces:

- **`ReductionResult` struct + impl** — `target_problem()` + `extract_solution()` (see reference)
- **`ReduceTo` impl with `#[reduction(...)]` macro** — auto-generates `inventory::submit!`; only `overhead` attribute needed (graph/weight types are inferred, defaulting to `SimpleGraph`/`Unweighted`)
- **`#[cfg(test)] #[path = ...]`** linking to unit tests

Register in `src/rules/mod.rs`.

## 2. Test

- **Unit tests** in `src/unit_tests/rules/<source>_<target>.rs` — closed-loop + edge cases (see reference test).
- **Integration tests** in `tests/suites/reductions.rs` — compare against JSON ground truth.

## 3. Example Program

Add `examples/reduction_<source>_to_<target>.rs` — create, reduce, solve, extract, verify, export JSON (see reference example).

Examples must expose `pub fn run()` with `fn main() { run() }` so they can be tested directly via `include!` (no subprocess). Use regular comments (`//`) not inner doc comments (`//!`), and hardcode the example name instead of using `env!("CARGO_BIN_NAME")`.

Register the example in `tests/suites/examples.rs` by adding:
```rust
example_test!(reduction_<source>_to_<target>);
example_fn!(test_<source>_to_<target>, reduction_<source>_to_<target>);
```

## 4. Document

Update `docs/paper/reductions.typ` — add `reduction-rule("Source", "Target", ...)` with proof sketch (see `rules/documentation.md`).

## 5. Regenerate Graph

```bash
cargo run --example export_graph
```
