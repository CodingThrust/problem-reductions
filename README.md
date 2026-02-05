# Problem Reductions

[![CI](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml/badge.svg)](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/CodingThrust/problem-reductions/graph/badge.svg?token=0CdEC8GHN0)](https://codecov.io/github/CodingThrust/problem-reductions)
[![Documentation](https://github.com/CodingThrust/problem-reductions/actions/workflows/docs.yml/badge.svg)](https://codingthrust.github.io/problem-reductions/)
[![PDF Manual](https://img.shields.io/badge/PDF-Manual-blue)](https://codingthrust.github.io/problem-reductions/reductions.pdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for NP-hard problem definitions and reductions. We aim to implement >100 NP-hard problems and reductions rule between them, under the help of AI.

## Installation

Add to your `Cargo.toml` (not yet available):

```toml
[dependencies]
problemreductions = "0.1"
```

## Quick Start

```rust
use problemreductions::prelude::*;
use problemreductions::models::optimization::ILP;

// Create an Independent Set problem on a path graph
let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// Reduce to Integer Linear Programming
let reduction = ReduceTo::<ILP>::reduce_to(&problem);
let ilp = reduction.target_problem();

// Solve with ILP solver (efficient for larger instances)
let solver = ILPSolver::new();
let ilp_solution = solver.solve(ilp).unwrap();

// Extract solution back to original problem
let solution = reduction.extract_solution(&ilp_solution);
assert_eq!(solution.iter().sum::<usize>(), 2); // Max IS size is 2
```

## Development

### Using Make

```bash
make help      # Show all available targets
make build     # Build the project
make test      # Run all tests
make fmt       # Format code with rustfmt
make fmt-check # Check code formatting
make clippy    # Run clippy lints
make doc       # Build and open documentation
make coverage  # Generate coverage report (requires cargo-llvm-cov)
make clean     # Clean build artifacts
make check     # Quick check before commit (fmt + clippy + test)
```

## Contributing

### Authorship Recognition

**Contribute 10 non-trivial reduction rules and you will be automatically added to the author list of the paper.** To facilitate the development, we provide the AI tools to help developers implement their *plans*. Developers still need to carefully design the test cases and verify the correctness of the reduction rules.

### Using Claude Code (Recommended)

1. Find or create a GitHub issue describing your contribution
2. Run the issue-to-pr skill:
   ```
   /issue-to-pr <issue-number>
   ```
3. Brainstorm with Claude using `superpowers:brainstorming` to clarify requirements
4. The skill creates a PR starting with `[action]`, which automatically triggers Claude CI to implement the plan

## License

MIT License - see [LICENSE](LICENSE) for details.
