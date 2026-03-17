---
name: Problem
about: Propose a new problem type
title: "[Model] K^thLargestMTuple"
labels: model
assignees: ''
---

## Motivation

K^th LARGEST m-TUPLE (P148) from Garey & Johnson, A3 SP21. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP21

**Mathematical definition:**

INSTANCE: Sets X_1,X_2,…,X_m ⊆ Z^+, a size s(x) ∈ Z^+ for each x ∈ X_i, 1 ≤ i ≤ m, and positive integers K and B.
QUESTION: Are there K or more distinct m-tuples (x_1,x_2,…,x_m) in X_1×X_2×···×X_m for which Σ_{i=1}^{m} s(x_i) ≥ B?

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

INSTANCE: Sets X_1,X_2,…,X_m ⊆ Z^+, a size s(x) ∈ Z^+ for each x ∈ X_i, 1 ≤ i ≤ m, and positive integers K and B.
QUESTION: Are there K or more distinct m-tuples (x_1,x_2,…,x_m) in X_1×X_2×···×X_m for which Σ_{i=1}^{m} s(x_i) ≥ B?
Reference: [Johnson and Mizoguchi, 1978]. Transformation from PARTITION.
Comment: Not known to be in NP. Solvable in polynomial time for fixed m, and in pseudo-polynomial time in general (polynomial in K, Σ|X_i|, and log Σs(x)). The corresponding enumeration problem is #P-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
