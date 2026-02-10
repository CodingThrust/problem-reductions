---
paths:
  - "src/rules/**/*.rs"
---

# Adding a Reduction Rule (A -> B)

**Reference implementation:** `src/rules/minimumvertexcover_maximumindependentset.rs`
**Reference test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
**Reference example:** `examples/reduction_minimumvertexcover_to_maximumindependentset.rs`
**Reference paper entry:** `docs/paper/reductions.typ` (search for `MinimumVertexCover` `MaximumIndependentSet`)

## 0. Before Writing Code

1. **Brainstorm** — use `superpowers:brainstorming` to discuss with the user:
   - The math (variable mapping, constraint encoding, penalty terms)
   - Which example instance to use in `examples/` (must be small, human-explainable, and agreed with the user)
2. **Generate ground truth** — use Python scripts in `scripts/` (run with `uv`) to create test data in `tests/data/<target>/`.
3. **Write plan** — save to `docs/plans/` using `superpowers:writing-plans`.

## 1. Implement

Create `src/rules/<source>_<target>.rs` following the reference. Key pieces:
- `ReductionResult` struct + impl (`target_problem`, `extract_solution`, `source_size`, `target_size`)
- `#[reduction(...)]` macro on `ReduceTo<Target> for Source` impl (auto-generates `inventory::submit!`)
- `#[cfg(test)] #[path = ...]` linking to unit tests

Register in `src/rules/mod.rs`.

## 2. Test

- **Unit tests** in `src/unit_tests/rules/<source>_<target>.rs` — closed-loop + edge cases (see reference test).
- **Integration tests** in `tests/suites/reductions.rs` — compare against JSON ground truth.

## 3. Example Program

Add `examples/reduction_<source>_to_<target>.rs` — create, reduce, solve, extract, verify, export JSON (see reference example).

## 4. Document

Update `docs/paper/reductions.typ` — add `reduction-rule("Source", "Target", ...)` with proof sketch (see `rules/documentation.md`).

## 5. Regenerate Graph

```bash
cargo run --example export_graph
```
