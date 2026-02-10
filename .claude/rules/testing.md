# Testing Requirements

**Reference test:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`

## Coverage

New code must have >95% test coverage. Run `make coverage` to check.

## Naming

- Reduction tests: `test_<source>_to_<target>_closed_loop`
- Model tests: `test_<model>_basic`, `test_<model>_serialization`
- Solver tests: `test_<solver>_<problem>`

## File Organization

Unit tests live in `src/unit_tests/`, mirroring `src/` structure. Source files reference them via `#[path]`:

```rust
// In src/rules/foo_bar.rs:
#[cfg(test)]
#[path = "../unit_tests/rules/foo_bar.rs"]
mod tests;
```

Integration tests are in `tests/suites/`, consolidated through `tests/main.rs`.

## Before PR

```bash
make test clippy
```
