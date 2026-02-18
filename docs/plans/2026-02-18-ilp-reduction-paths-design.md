# ILP Reduction Paths for QUBO and CircuitSAT

**Issue:** #83 — Add ILP reduction paths for CircuitSAT, MaxCut, QUBO, SpinGlass
**Date:** 2026-02-18

## Problem

Four problem types have no reduction path to ILP, causing `pred solve` to fail with the default ILP solver: QUBO, SpinGlass, MaxCut, CircuitSAT.

## Solution

Two new reductions:

1. **QUBO → ILP** — Covers QUBO, SpinGlass (via SpinGlass→QUBO→ILP), and MaxCut (via MaxCut→SpinGlass→QUBO→ILP)
2. **CircuitSAT → ILP** — Direct 1-step path (more efficient than the 3-step CircuitSAT→SpinGlass→QUBO→ILP chain)

## Reduction 1: QUBO → ILP (McCormick Linearization)

QUBO minimizes `x^T Q x` where `x ∈ {0,1}^n` and Q is upper-triangular.

**Linearization:** For each non-zero off-diagonal `Q_ij` (i < j), introduce binary auxiliary `y_ij = x_i · x_j`:
- `y_ij ≤ x_i`
- `y_ij ≤ x_j`
- `y_ij ≥ x_i + x_j - 1`

Diagonal terms are already linear: `Q_ii · x_i² = Q_ii · x_i` for binary x.

**ILP:**
- Variables: `n` original + `m` auxiliary (m = non-zero off-diagonal count)
- All binary bounds
- Objective: `minimize Σ_i Q_ii · x_i + Σ_{i<j} Q_ij · y_ij`
- Constraints: 3m (McCormick envelopes)

**Solution extraction:** First `n` variables of ILP solution.

**Overhead:** `num_vars = poly!(num_vars + num_interactions)` where `num_interactions` = off-diagonal non-zeros.

## Reduction 2: CircuitSAT → ILP (Gate Constraint Encoding)

Walk each Assignment's expression tree, creating an auxiliary binary ILP variable for each internal node with gate-specific linear constraints.

**Gate constraints (c = output, all binary):**

| Gate | Constraints |
|------|------------|
| NOT(a) = c | c + a = 1 |
| AND(a₁,...,aₖ) = c | c ≤ aᵢ (∀i), c ≥ Σaᵢ - (k-1) |
| OR(a₁,...,aₖ) = c | c ≥ aᵢ (∀i), c ≤ Σaᵢ |
| XOR(a,b) = c | c ≤ a+b, c ≥ a-b, c ≥ b-a, c ≤ 2-a-b |
| Const(v) = c | c = v |

Multi-input XOR decomposes into pairwise chain with auxiliary variables.

Each Assignment's root expression variable is constrained to equal its output variable(s).

**ILP:**
- Variables: circuit variables + auxiliary gate variables, all binary
- Objective: minimize 0 (satisfaction problem → any feasible solution works)
- Constraints: gate encoding + output equality

**Solution extraction:** First `n` variables (original CircuitSAT variables), discard gate auxiliaries.

**Overhead:** `num_vars = poly!(num_variables + num_assignments)`

## Files

- `src/rules/qubo_ilp.rs` — QUBO → ILP
- `src/rules/circuit_ilp.rs` — CircuitSAT → ILP
- Unit tests in `src/unit_tests/rules/` (closed-loop pattern)
- Examples in `examples/`
- Paper entries in `docs/paper/reductions.typ`

## Testing

- Closed-loop: reduce → solve both with BruteForce → verify extracted solutions match
- QUBO edge cases: empty, single-variable, diagonal-only, sparse vs dense
- CircuitSAT: AND/OR/NOT, XOR, nested expressions, multi-output assignments
- Integration: verify `pred solve` works for all 4 problem types after implementation
