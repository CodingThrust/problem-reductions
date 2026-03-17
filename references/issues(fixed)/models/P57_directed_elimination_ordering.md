---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedEliminationOrdering"
labels: model
assignees: ''
---

## Motivation

DIRECTED ELIMINATION ORDERING (P57) from Garey & Johnson, A1.3 GT46. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT46

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), non-negative integer K.
QUESTION: Is there an elimination ordering for G with fill-in K or less, i.e., a one-to-one function f: V → {1,2,...,|V|} such that there are at most K pairs of vertices (u,v) ∈ (V×V)−A with the property that G contains a directed path from u to v that only passes through vertices w satisfying f(w) < min{f(u),f(v)}?

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

INSTANCE: Directed graph G = (V,A), non-negative integer K.
QUESTION: Is there an elimination ordering for G with fill-in K or less, i.e., a one-to-one function f: V → {1,2,...,|V|} such that there are at most K pairs of vertices (u,v) ∈ (V×V)−A with the property that G contains a directed path from u to v that only passes through vertices w satisfying f(w) < min{f(u),f(v)}?

Reference: [Rose and Tarjan, 1978]. Transformation from 3SAT.
Comment: Problem arises in performing Gaussian elimination on sparse matrices. Solvable in polynomial time for K = 0. The analogous problem for undirected graphs (symmetric matrices) is equivalent to CHORDAL GRAPH COMPLETION and is open as to complexity.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
