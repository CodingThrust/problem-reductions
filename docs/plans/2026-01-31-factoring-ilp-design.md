# Factoring to ILP Reduction Design

**Issue:** #21 - Implement integer programming based Factoring problem solver
**Date:** 2026-01-31
**Status:** Approved

## Overview

Implement a reduction from the Factoring problem to Integer Linear Programming (ILP). Given a target number N and bit widths m and n, find binary factors p and q such that p × q = N.

The key challenge is linearizing the multiplication constraint, which we solve using McCormick relaxation for binary products combined with carry-propagation to handle bit-position sums.

## ILP Formulation

### Variables

| Variable | Range | Count | Description |
|----------|-------|-------|-------------|
| `p_i` | {0,1} | m | First factor bits (i = 0..m-1) |
| `q_j` | {0,1} | n | Second factor bits (j = 0..n-1) |
| `z_ij` | {0,1} | m×n | Product terms `p_i × q_j` |
| `c_k` | [0, min(m,n)] | m+n | Carry at bit position k |

**Total variables:** m + n + m×n + (m+n) = O(m×n)

### Constraints

**1. Product linearization** (for each i ∈ [0,m), j ∈ [0,n)):
```
z_ij ≤ p_i
z_ij ≤ q_j
z_ij ≥ p_i + q_j - 1
```
These McCormick constraints ensure z_ij = p_i × q_j for binary variables.

**2. Bit-position equations** (for each k ∈ [0, m+n)):
```
Σ_{i+j=k} z_ij + c_{k-1} = N_k + 2·c_k
```
Where:
- N_k is the k-th bit of target N
- c_{-1} = 0 (no incoming carry at position 0)

**3. No overflow:**
```
c_{m+n-1} = 0
```

**Total constraints:** 3×m×n + (m+n) + 1 = O(m×n)

### Objective

Feasibility problem: minimize 0.

## Implementation

### File Structure

```
src/rules/factoring_ilp.rs
```

### Data Structures

```rust
pub struct ReductionFactoringToILP {
    target: ILP,
    source_size: ProblemSize,
    m: usize,  // bits for first factor
    n: usize,  // bits for second factor
}
```

### Variable Layout

Contiguous indices in ILP:
```
[0, m)                    → p_i (first factor bits)
[m, m+n)                  → q_j (second factor bits)
[m+n, m+n+m*n)            → z_ij (products, row-major order)
[m+n+m*n, 2(m+n)+m*n)     → c_k (carries)
```

### Helper Methods

```rust
impl ReductionFactoringToILP {
    fn p_var(&self, i: usize) -> usize { i }
    fn q_var(&self, j: usize) -> usize { self.m + j }
    fn z_var(&self, i: usize, j: usize) -> usize {
        self.m + self.n + i * self.n + j
    }
    fn carry_var(&self, k: usize) -> usize {
        self.m + self.n + self.m * self.n + k
    }
}
```

### Solution Extraction

1. Read p_i values from indices [0, m) → first factor bits
2. Read q_j values from indices [m, m+n) → second factor bits
3. Return concatenated bit vector matching Factoring problem format

## Testing Strategy

### Correctness Tests

| Test | Target | Bits | Expected Factors |
|------|--------|------|------------------|
| factor_6 | 6 | 2,2 | 2×3 or 3×2 |
| factor_15 | 15 | 3,3 | 3×5, 5×3, 1×15, 15×1 |
| factor_35 | 35 | 3,3 | 5×7 or 7×5 |

### Edge Cases

| Test | Description |
|------|-------------|
| factor_one | 1 = 1×1 |
| factor_prime | 7 = 1×7 or 7×1 |
| factor_square | 9 = 3×3 |
| infeasible | Target 100 with 2-bit factors (max 9) |

### Validation Tests

- Compare ILP solutions with brute force solver
- Verify solution extraction produces valid factorization
- Check variable and constraint counts match formulas

### Closed-Loop Pattern (Issue #3)

```rust
let problem = Factoring::new(m, n, target);
let reduction = ReduceTo::<ILP>::reduce_to(&problem);
let ilp = reduction.target_problem();
let ilp_solution = ILPSolver::new().solve(ilp)?;
let extracted = reduction.extract_solution(&ilp_solution);
assert!(problem.is_valid_factorization(&extracted));
```

## Documentation Updates

Per issue #3 requirements:

1. Add Factoring → ILP to `docs/paper/reductions.typ`:
   - Theorem with proof sketch
   - Add to summary table with overhead O(m×n)

2. Update reduction graph:
   - Register in inventory
   - Add ILP node (already exists from Coloring PR)
   - Add edge Factoring → ILP

3. Regenerate `docs/paper/reduction_graph.json`

## Complexity Analysis

| Metric | Value |
|--------|-------|
| Variables | m + n + m×n + (m+n) |
| Constraints | 3×m×n + (m+n) + 1 |
| Time complexity | O(m×n) for reduction |
| Space complexity | O(m×n) |

## References

- McCormick, G. P. (1976). Computability of global solutions to factorable nonconvex programs. Mathematical Programming.
- Issue #3: Coding rules for AI agents
- Existing implementation: `src/rules/vertexcovering_ilp.rs` (pattern reference)
