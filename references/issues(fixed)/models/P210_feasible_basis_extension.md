---
name: Problem
about: Propose a new problem type
title: "[Model] FeasibleBasisExtension"
labels: model
assignees: ''
---

## Motivation

FEASIBLE BASIS EXTENSION (P210) from Garey & Johnson, A6 MP4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP4

**Mathematical definition:**

INSTANCE: An m×n integer matrix A, m < n, a column vector ā of length m, and a subset S of the columns of A with |S| < m.
QUESTION: Is there a feasible basis B for Ax̄ = ā, x̄ ≥ 0, i.e., a nonsingular m×m submatrix B of A such that B⁻¹ā ≥ 0, and such that B contains all the columns in S?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: An m×n integer matrix A, m < n, a column vector ā of length m, and a subset S of the columns of A with |S| < m.
QUESTION: Is there a feasible basis B for Ax̄ = ā, x̄ ≥ 0, i.e., a nonsingular m×m submatrix B of A such that B⁻¹ā ≥ 0, and such that B contains all the columns in S?

Reference: [Murty, 1972]. Transformation from HAMILTONIAN CIRCUIT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
