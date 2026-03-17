---
name: Problem
about: Propose a new problem type
title: "[Model] IntersectionPattern"
labels: model
assignees: ''
---

## Motivation

INTERSECTION PATTERN (P136) from Garey & Johnson, A3 SP9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP9

**Mathematical definition:**

INSTANCE: An n×n matrix A = (a_{ij}) with entries in Z_0^+.
QUESTION: Is there a collection C = {C_1,C_2,…,C_n} of sets such that for all i,j, 1 ≤ i,j ≤ n, a_{ij} = |C_i ∩ C_j|?

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

INSTANCE: An n×n matrix A = (a_{ij}) with entries in Z_0^+.
QUESTION: Is there a collection C = {C_1,C_2,…,C_n} of sets such that for all i,j, 1 ≤ i,j ≤ n, a_{ij} = |C_i ∩ C_j|?
Reference: [Chvátal, 1978]. Transformation from GRAPH 3-COLORABILITY.
Comment: Remains NP-complete even if all a_{ii} = 3, 1 ≤ i ≤ m (and hence all C_i must have cardinality 3). If all a_{ii} = 2, it is equivalent to edge graph recognition and hence can be solved in polynomial time (e.g., see [Harary, 1969]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
