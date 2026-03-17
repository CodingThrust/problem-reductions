---
name: Problem
about: Propose a new problem type
title: "[Model] InducedPath"
labels: model
assignees: ''
---

## Motivation

INDUCED PATH (P34) from Garey & Johnson, A1.2 GT23. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT23

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph induced by V' is a simple path on |V'| vertices?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph induced by V' is a simple path on |V'| vertices?
Reference: [Yannakakis, 1978c]. Transformation from 3SAT.
Comment: Note that this is not a hereditary property, so the result is not implied by either of the previous two results. Remains NP-complete if G is bipartite. The same result holds for the variant in which "simple path" is replaced by "simple cycle." The problems of finding the longest simple path or longest simple cycle (not necessarily induced) are also NP-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
