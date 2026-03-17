---
name: Problem
about: Propose a new problem type
title: "[Model] Numerical3DimensionalMatching"
labels: model
assignees: ''
---

## Motivation

NUMERICAL 3-DIMENSIONAL MATCHING (P143) from Garey & Johnson, A3 SP16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP16

**Mathematical definition:**

INSTANCE: Disjoint sets W, X, and Y, each containing m elements, a size s(a) ∈ Z^+ for each element a ∈ W ∪ X ∪ Y, and a bound B ∈ Z^+.
QUESTION: Can W ∪ X ∪ Y be partitioned into m disjoint sets A_1,A_2,…,A_m such that each A_i contains exactly one element from each of W, X, and Y and such that, for 1 ≤ i ≤ m, Σ_{a ∈ A_i} s(a) = B?

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

INSTANCE: Disjoint sets W, X, and Y, each containing m elements, a size s(a) ∈ Z^+ for each element a ∈ W ∪ X ∪ Y, and a bound B ∈ Z^+.
QUESTION: Can W ∪ X ∪ Y be partitioned into m disjoint sets A_1,A_2,…,A_m such that each A_i contains exactly one element from each of W, X, and Y and such that, for 1 ≤ i ≤ m, Σ_{a ∈ A_i} s(a) = B?
Reference: [Garey and Johnson, ——]. Transformation from 3DM (see proof of Theorem 4.4).
Comment: NP-complete in the strong sense.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
