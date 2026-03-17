---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumWeightSolutionToLinearEquations"
labels: model
assignees: ''
---

## Motivation

MINIMUM WEIGHT SOLUTION TO LINEAR EQUATIONS (P211) from Garey & Johnson, A6 MP5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP5

**Mathematical definition:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, and a positive integer K ≤ m.
QUESTION: Is there an m-tuple ȳ with rational entries such that ȳ has at most K non-zero entries and such that x̄·ȳ = b for all (x̄,b) ∈ X?

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

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, and a positive integer K ≤ m.
QUESTION: Is there an m-tuple ȳ with rational entries such that ȳ has at most K non-zero entries and such that x̄·ȳ = b for all (x̄,b) ∈ X?

Reference: [Garey and Johnson, ——]. Transformation from X3C.
Comment: NP-complete in the strong sense. Solvable in polynomial time if K = m.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
