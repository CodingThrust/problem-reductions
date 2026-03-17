---
name: Problem
about: Propose a new problem type
title: "[Model] PartialFeedbackEdgeSet"
labels: model
assignees: ''
---

## Motivation

PARTIAL FEEDBACK EDGE SET (P20) from Garey & Johnson, A1.1 GT9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT9

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integers K ≤ |E| and L ≤ |V|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≤ K such that E' contains at least one edge from every circuit of length L or less in G?

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

INSTANCE: Graph G = (V,E), positive integers K ≤ |E| and L ≤ |V|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≤ K such that E' contains at least one edge from every circuit of length L or less in G?
Reference: [Yannakakis, 1978b]. Transformation from VERTEX COVER.
Comment: Remains NP-complete for any fixed L ≥ 3 and for bipartite graphs (with fixed L ≥ 4). However, if L = |V|, i.e., if we ask that E' contain an edge from every cycle in G, then the problem is trivially solvable in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
