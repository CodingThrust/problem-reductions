---
name: Problem
about: Propose a new problem type
title: "[Model] PathGraphCompletion"
labels: model
assignees: ''
---

## Motivation

PATH GRAPH COMPLETION (P47) from Garey & Johnson, A1.2 GT36. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT36

**Mathematical definition:**

INSTANCE: Graph G = (V,E), non-negative integer K.
QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') is the intersection graph of a family of paths on an undirected tree?

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

INSTANCE: Graph G = (V,E), non-negative integer K.
QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') is the intersection graph of a family of paths on an undirected tree?
Reference: [Gavril, 1977b]. Transformation from INTERVAL GRAPH COMPLETION.
Comment: Corresponding problem in which G' must be the intersection graph of a family of directed paths on an oriented tree (i.e., rooted, with all arcs directed away from the root) is also NP-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
