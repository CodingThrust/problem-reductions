---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumMaximalMatching"
labels: model
assignees: ''
---

## Motivation

MINIMUM MAXIMAL MATCHING (P21) from Garey & Johnson, A1.1 GT10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT10

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≤ K such that E' is a maximal matching, i.e., no two edges in E' share a common endpoint and every edge in E−E' shares a common endpoint with some edge in E'?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≤ K such that E' is a maximal matching, i.e., no two edges in E' share a common endpoint and every edge in E−E' shares a common endpoint with some edge in E'?
Reference: [Yannakakis and Gavril, 1978]. Transformation from VERTEX COVER for cubic graphs.
Comment: Remains NP-complete for planar graphs and for bipartite graphs, in both cases even if no vertex degree exceeds 3. The problem of finding a maximum "maximal matching" is just the usual graph matching problem and is solvable in polynomial time (e.g., see [Lawler, 1976a]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
