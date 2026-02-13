# Design: Test Against ProblemReductions.jl (#64)

## Goal

Full parity check between Rust `problemreductions` crate and Julia `ProblemReductions.jl`: reductions, evaluations, and solver results must match for all reductions in the Julia package.

## Approach

Julia script generates JSON fixtures from ProblemReductions.jl test cases. Rust integration tests load fixtures and verify parity. Reductions not yet implemented in Rust get `#[ignore]` test stubs.

## Julia Environment

Location: `scripts/jl/`

```
scripts/jl/
├── Project.toml            # ProblemReductions, Graphs, JSON3
├── Manifest.toml           # Locked versions
└── generate_testdata.jl    # Generates tests/data/jl_*.json
```

Install ProblemReductions.jl from the Julia registry (`Pkg.add`).

## JSON Fixture Format

### Model fixtures (`tests/data/jl_<problem>.json`)

```json
{
  "problem_type": "IndependentSet",
  "instances": [
    {
      "label": "4v_cycle",
      "instance": { "num_vertices": 4, "edges": [[0,1], [0,2], [1,2], [2,3]] },
      "weights": [1, 1, 1, 1],
      "evaluations": [
        { "config": [1,0,0,1], "size": 2, "is_valid": true },
        { "config": [0,1,1,0], "size": null, "is_valid": false }
      ],
      "best_solutions": [[1,0,0,1], [0,1,0,1]]
    }
  ]
}
```

### Reduction fixtures (`tests/data/jl_<source>_to_<target>.json`)

```json
{
  "source_type": "IndependentSet",
  "target_type": "VertexCovering",
  "cases": [
    {
      "label": "petersen",
      "source": { ... },
      "target": { ... },
      "best_source": [[1,0,...], ...],
      "best_target": [[0,1,...], ...],
      "extracted_solutions": [[1,0,...], ...]
    }
  ]
}
```

All indices 0-based (Julia script handles 1→0 conversion).

## Reductions Covered (17 directed)

From `pkgref/ProblemReductions.jl/test/rules/rules.jl`:

| # | Source | Target | In Rust? |
|---|--------|--------|----------|
| 1 | CircuitSAT | SpinGlass | Yes |
| 2 | MaxCut | SpinGlass | Yes |
| 3 | SpinGlass | MaxCut | Yes |
| 4 | QUBO | SpinGlass | Yes |
| 5 | SpinGlass | QUBO | Yes |
| 6 | SAT | KSat{3} | Yes |
| 7 | KSat | SAT | Yes |
| 8 | SAT | Coloring{3} | No → #[ignore] |
| 9 | SAT | IndependentSet | No → #[ignore] |
| 10 | SAT | DominatingSet | No → #[ignore] |
| 11 | IndependentSet | SetPacking | Yes |
| 12 | IndependentSet(HyperGraph) | SetPacking | Yes |
| 13 | SetPacking | IndependentSet | Yes |
| 14 | IndependentSet | VertexCovering | Yes |
| 15 | VertexCovering | SetCovering | Yes |
| 16 | Matching | SetPacking | No → #[ignore] |
| 17 | Factoring | CircuitSAT | Yes |

## Julia ↔ Rust Mapping

| Aspect | Julia | Rust | Handling |
|--------|-------|------|----------|
| Indexing | 1-based | 0-based | Julia script subtracts 1 |
| Names | `IndependentSet` | `MaximumIndependentSet` | Mapping in Rust tests |
| Names | `VertexCovering` | `MinimumVertexCover` | Mapping in Rust tests |
| Graph | `fadjlist` | edge list | Julia exports edges |
| Weights | `UnitWeight` | `Unweighted` | Julia exports `[1,1,...]` |
| SAT vars | Symbols `:a` | Integers | Julia maps to 0-based ints |

## Rust Test Structure

New file: `tests/suites/jl_parity.rs` (added to `tests/main.rs`)

Tests per model: `test_jl_parity_<problem>_evaluation`
Tests per reduction: `test_jl_parity_<source>_to_<target>`
Missing reductions: `#[ignore]` stubs

## Verification

- `make test` passes (new tests + all existing)
- `make clippy` passes
- Julia script runs cleanly: `cd scripts/jl && julia --project=. generate_testdata.jl`
