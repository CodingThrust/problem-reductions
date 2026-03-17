---
name: Problem
about: Propose a new problem type
title: "[Model] DynamicStorageAllocation"
labels: model
assignees: ''
---

## Motivation

DYNAMIC STORAGE ALLOCATION (P150) from Garey & Johnson, A4 SR2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR2

**Mathematical definition:**

INSTANCE: Set A of items to be stored, each a ∈ A having a size s(a) ∈ Z+, an arrival time r(a) ∈ Z0+, and a departure time d(a) ∈ Z+, and a positive integer storage size D.
QUESTION: Is there a feasible allocation of storage for A, i.e., a function σ: A → {1,2,...,D} such that for every a ∈ A the allocated storage interval I(a) = [σ(a),σ(a)+s(a)−1] is contained in [1,D] and such that, for all a,a' ∈ A, if I(a) ∩ I(a') is nonempty then either d(a) ≤ r(a') or d(a') ≤ r(a)?

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

INSTANCE: Set A of items to be stored, each a ∈ A having a size s(a) ∈ Z+, an arrival time r(a) ∈ Z0+, and a departure time d(a) ∈ Z+, and a positive integer storage size D.
QUESTION: Is there a feasible allocation of storage for A, i.e., a function σ: A → {1,2,...,D} such that for every a ∈ A the allocated storage interval I(a) = [σ(a),σ(a)+s(a)−1] is contained in [1,D] and such that, for all a,a' ∈ A, if I(a) ∩ I(a') is nonempty then either d(a) ≤ r(a') or d(a') ≤ r(a)?
Reference: [Stockmeyer, 1976b]. Transformation from 3-PARTITION.
Comment: NP-complete in the strong sense, even if s(a) ∈ {1,2} for all a ∈ A. Solvable in polynomial time if all item sizes are the same, by interval graph coloring algorithms (e.g., see [Gavril, 1972]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
