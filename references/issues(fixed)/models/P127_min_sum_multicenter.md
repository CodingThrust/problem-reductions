---
name: Problem
about: Propose a new problem type
title: "[Model] MinSumMulticenter"
labels: model
assignees: ''
---

## Motivation

MIN-SUM MULTICENTER (P127) from Garey & Johnson, A2 ND51. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND51

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z_0^+ for each v ∈ V, length l(e) ∈ Z_0^+ for each e ∈ E, positive integer K ≤ |V|, positive rational number B.
QUESTION: Is there a set P of K "points on G" such that if d(v) is the length of the shortest path from v to the closest point in P, then Σ_{v ∈ V} d(v)·w(v) ≤ B?

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

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z_0^+ for each v ∈ V, length l(e) ∈ Z_0^+ for each e ∈ E, positive integer K ≤ |V|, positive rational number B.
QUESTION: Is there a set P of K "points on G" such that if d(v) is the length of the shortest path from v to the closest point in P, then Σ_{v ∈ V} d(v)·w(v) ≤ B?
Reference: [Kariv and Hakimi, 1976b]. Transformation from DOMINATING SET.
Comment: Also known as the "p-median" problem. It can be shown that there is no loss of generality in restricting P to being a subset of V. Remains NP-complete if w(v) = 1 for all v ∈ V and l(e) = 1 for all e ∈ E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
