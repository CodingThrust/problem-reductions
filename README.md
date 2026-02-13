![100-Problem-Reductions](docs/logo.svg)

[![Crates.io](https://img.shields.io/crates/v/problemreductions)](https://crates.io/crates/problemreductions)
[![CI](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml/badge.svg)](https://github.com/CodingThrust/problem-reductions/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/CodingThrust/problem-reductions/graph/badge.svg?token=0CdEC8GHN0)](https://codecov.io/github/CodingThrust/problem-reductions)
[![Docs](https://img.shields.io/badge/Docs-API-green)](https://codingthrust.github.io/problem-reductions/)

A Rust library for NP-hard problem definitions and reductions. We aim to implement >100 NP-hard problems and reductions rule between them, under the assistance of AI.

This infrastructure aims to solve two problems:
- Given a hard problem $A$, reduce it to the most vaible problem $B$, to be solved efficiently with an external solver.
- Given a solver $S$ for problem $B$, explore how efficient it can be used for solving other problems.

Download [PDF manual](https://codingthrust.github.io/problem-reductions/reductions.pdf) for humans.

## Installation

Add to your `Cargo.toml`:

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

## Contributing

### Authorship Recognition

**Contribute 10 non-trivial reduction rules and you will be automatically added to the author list of the paper.** AI tools handle the implementation — contributors focus on designing correct reductions and test cases.

### How to Contribute

1. **Open an issue** using the [Problem](https://github.com/CodingThrust/problem-reductions/issues/new?template=problem.md) or [Rule](https://github.com/CodingThrust/problem-reductions/issues/new?template=rule.md) template. Fill in all sections — the templates guide you through the required information (definition, algorithm, size overhead, example instance, etc.).

2. Our AI agents will pick-up the issue and generate a plan to implement the reduction rule.
3. You will be mentioned in the pull request, provide feedback to the AI agents. If you are satisfied with the plan, you can merge the PR.

Optionally, if you prefer to **implement yourself**, I will recommend you to use the [superpowers:brainstorming](https://github.com/obra/superpowers) skill to help you write a detailed plan. Create a PR and let maintainers help review and merge the PR.

### Developer Commands

Run `make help` to see all available targets. See [CLAUDE.md](https://codingthrust.github.io/problem-reductions/claude.html) for the full command list and architecture details.

## Acknowledgments

This project draws inspiration from the following packages:

- **[ProblemReductions.jl](https://github.com/GiggleLiu/ProblemReductions.jl)** — Julia library for computational problem reductions. Our problem trait hierarchy, reduction interface (`ReduceTo`/`ReductionResult`), and graph-based reduction registry are directly inspired by this package.
- **[UnitDiskMapping.jl](https://github.com/QuEraComputing/UnitDiskMapping.jl)** — Julia package for mapping problems to unit disk graphs. Our unit disk graph (King's subgraph / triangular lattice) reductions and the copy-line method are based on this implementation.
- **[qubogen](https://github.com/tamuhey/qubogen)** — Python library for generating QUBO matrices from combinatorial problems. Our QUBO reduction formulas (Vertex Cover, Graph Coloring, Set Packing, Max-2-SAT, binary ILP) reference the implementations in this package.

## Related Projects

- **[Karp](https://github.com/REA1/karp)** — A DSL (built on Racket) for writing and testing Karp reductions between NP-complete problems ([PLDI 2022 paper](https://dl.acm.org/doi/abs/10.1145/3519939.3523732)). Focused on education and proof verification rather than a solver pipeline.
- **[Complexity Zoo](https://complexityzoo.net/)** — Comprehensive catalog of 550+ computational complexity classes (Scott Aaronson).
- **[A Compendium of NP Optimization Problems](https://www.csc.kth.se/tcs/compendium/)** — Online catalog of NP optimization problems with approximability results (Crescenzi & Kann).
- **Computers and Intractability** (Garey & Johnson, 1979) — The classic reference cataloging 300+ NP-complete problems with reductions. The most cited book in computer science.

## License

MIT License - see [LICENSE](LICENSE) for details.
