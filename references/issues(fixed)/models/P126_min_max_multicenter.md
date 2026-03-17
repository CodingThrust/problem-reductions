---
name: Problem
about: Propose a new problem type
title: "[Model] MinMaxMulticenter"
labels: model
assignees: ''
---

## Motivation

MIN-MAX MULTICENTER (P126) from Garey & Johnson, A2 ND50. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND50

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z_0^+ for each v ∈ V, length l(e) ∈ Z_0^+ for each e ∈ E, positive integer K ≤ |V|, positive rational number B.
QUESTION: Is there a set P of K "points on G" (where a point on G can be either a vertex in V or a point on an edge e ∈ E, with e regarded as a line segment of length l(e)) such that if d(v) is the length of the shortest path from v to the closest point in P, then max{d(v)·w(v): v ∈ V} ≤ B?

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
QUESTION: Is there a set P of K "points on G" (where a point on G can be either a vertex in V or a point on an edge e ∈ E, with e regarded as a line segment of length l(e)) such that if d(v) is the length of the shortest path from v to the closest point in P, then max{d(v)·w(v): v ∈ V} ≤ B?
Reference: [Kariv and Hakimi, 1976a]. Transformation from DOMINATING SET.
Comment: Also known as the "p-center" problem. Remains NP-complete if w(v) = 1 for all v ∈ V and l(e) = 1 for all e ∈ E. Solvable in polynomial time for any fixed K and for arbitrary K if G is a tree [Kariv and Hakimi, 1976a]. Variant in which we must choose a subset P ⊆ V is also NP-complete but solvable for fixed K and for trees [Slater, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
