# Design: Issue 24 - PDF Download & README Example Fix

## Overview

This design addresses two related improvements:
1. **Issue 24**: Add PDF manual download to documentation
2. **README fix**: Update Quick Start example to use correct imports and show ILP reduction

## Part 1: README Example Fix

### Problem
The current README example uses `IndependentSetT` and `VertexCoverT` which are not exported in `prelude::*`, causing compilation errors.

### Solution
Replace with `IndependentSet` (which IS in prelude) and demonstrate the more valuable ILP reduction workflow.

### Updated Example

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

This demonstrates the core value proposition: reduce NP-hard problems to ILP for efficient solving.

## Part 2: Issue 24 - PDF Download

### Requirements
- Compile typst paper to PDF in CI
- Host PDF on GitHub Pages
- Add prominent badge in README

### README Badge

Add after existing badges:
```markdown
[![PDF Manual](https://img.shields.io/badge/PDF-Manual-blue)](https://codingthrust.github.io/problem-reductions/reductions.pdf)
```

### Workflow Changes (docs.yml)

Add to build job:

```yaml
- name: Install Typst
  run: |
    curl -sSL https://github.com/typst/typst/releases/download/v0.12.0/typst-x86_64-unknown-linux-musl.tar.xz | tar -xJ
    mv typst-x86_64-unknown-linux-musl/typst "$HOME/bin/"

- name: Build PDF
  run: typst compile docs/paper/reductions.typ book/reductions.pdf
```

## Implementation Checklist

- [ ] Update README.md Quick Start example
- [ ] Add PDF badge to README.md
- [ ] Add typst installation step to docs.yml
- [ ] Add PDF compilation step to docs.yml
- [ ] Test workflow locally if possible

## Files to Modify

1. `README.md` - Fix example, add badge
2. `.github/workflows/docs.yml` - Add typst compilation
