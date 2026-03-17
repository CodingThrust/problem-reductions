---
name: Problem
about: Propose a new problem type
title: "[Model] ChordalGraphCompletion"
labels: model
assignees: ''
---

## Motivation

CHORDAL GRAPH COMPLETION (P335) from Garey & Johnson, A13 OPEN4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN4

**Mathematical definition:**

INSTANCE: Graph G = (V, E) and a positive integer K.
QUESTION: Is there a superset E' containing E of unordered pairs of vertices from V that satisfies |E' − E| ≤ K and such that G' = (V, E') is chordal, i.e., such that for every simple cycle of more than 3 vertices in G', there is some edge in E' that is not involved in the cycle but that joins two vertices in the cycle?

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

INSTANCE: Graph G = (V, E) and a positive integer K.
QUESTION: Is there a superset E' containing E of unordered pairs of vertices from V that satisfies |E' − E| ≤ K and such that G' = (V, E') is chordal, i.e., such that for every simple cycle of more than 3 vertices in G', there is some edge in E' that is not involved in the cycle but that joins two vertices in the cycle?

Comment: This problem is equivalent to the undirected version of DIRECTED ELIMINATION ORDERING and corresponds to the problem of minimizing "fill-in" when applying Gaussian elimination to symmetric matrices (e.g., see [Rose, Tarjan, and Lueker, 1976]). See [Gavril, 1974b] for an alternative characterization of chordal graphs.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
