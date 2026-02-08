---
paths:
  - "src/rules/**/*.rs"
---

# Adding a Reduction Rule (A → B)

## 1. Implementation
Create `src/rules/<source>_<target>.rs`:

```rust
use problemreductions::reduction;

#[reduction(
    overhead = { ReductionOverhead::new(vec![...]) }
)]
impl ReduceTo<TargetProblem<Unweighted>> for SourceProblem<Unweighted> {
    type Result = ReductionSourceToTarget;
    fn reduce_to(&self) -> Self::Result { ... }
}
```

The `#[reduction]` macro auto-generates the `inventory::submit!` call. Optional attributes: `source_graph`, `target_graph`, `source_weighted`, `target_weighted`.

Register module in `src/rules/mod.rs`:
```rust
mod source_target;
pub use source_target::ReductionSourceToTarget;
```

## 2. Closed-Loop Test (Required)

See `rules/testing.md` for the full pattern. Test name: `test_<source>_to_<target>_closed_loop`.

## 3. Documentation
Update `docs/paper/reductions.typ` (see `rules/documentation.md` for the pattern):
- Add theorem + proof sketch
- Add code example
- Add to summary table with overhead and citation

Citations must be verifiable. Use `[Folklore]` or `—` for trivial reductions.

## 4. Regenerate Reduction Graph
```bash
make export-graph
```

## Anti-patterns
- Don't create reductions without closed-loop tests
- Don't forget `inventory::submit!` registration (reduction graph won't update)
- Don't hardcode weights - use generic `W` parameter
- Don't skip overhead polynomial specification
