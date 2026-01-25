# Contributing

Contributions are welcome!

## Development Setup

```bash
git clone https://github.com/liujinguo/problemreductions
cd problemreductions
cargo build
cargo test
```

## Running Tests

```bash
cargo test                    # Run all tests
cargo test --test integration # Integration tests only
cargo test --test reduction   # Reduction tests only
```

## Code Coverage

```bash
cargo tarpaulin --skip-clean --ignore-tests
```

## Documentation

```bash
cargo doc --open              # Rustdoc
mdbook serve                  # mdBook (requires mdbook installed)
```

## Adding a New Problem

1. Create file in `src/models/<category>/`
2. Implement `Problem` trait
3. Optionally implement `ConstraintSatisfactionProblem`
4. Add tests
5. Export in `mod.rs` and `prelude`

## Adding a New Reduction

1. Create file in `src/rules/`
2. Implement `ReductionResult` for the result type
3. Implement `ReduceTo<Target> for Source`
4. Add edge in `ReductionGraph`
5. Add tests
6. Export in `mod.rs`

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add doc comments for public items
- Include examples in documentation
