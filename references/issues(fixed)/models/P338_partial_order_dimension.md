---
name: Problem
about: Propose a new problem type
title: "[Model] PartialOrderDimension"
labels: model
assignees: ''
---

## Motivation

PARTIAL ORDER DIMENSION (P338) from Garey & Johnson, A13 OPEN7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN7

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V, A) that is transitive, i.e., whenever (u, v) ∈ A and (v, w) ∈ A, then (u, w) ∈ A, and a positive integer K ≤ |V|^2.
QUESTION: Does there exist a collection of k ≤ K linear orderings of V such that (u, v) ∈ A if and only if u is less than v in each of the orderings?

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

[OPEN7] PARTIAL ORDER DIMENSION
INSTANCE: Directed acyclic graph G = (V, A) that is transitive, i.e., whenever (u, v) ∈ A and (v, w) ∈ A, then (u, w) ∈ A, and a positive integer K ≤ |V|^2.
QUESTION: Does there exist a collection of k ≤ K linear orderings of V such that (u, v) ∈ A if and only if u is less than v in each of the orderings?
Comment: Solvable in polynomial time for K = 2 [Lawler, 1976d]. Open for arbitrary K and for any fixed K ≥ 3.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
