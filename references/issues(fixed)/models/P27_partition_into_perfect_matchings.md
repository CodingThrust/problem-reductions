---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoPerfectMatchings"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO PERFECT MATCHINGS (P27) from Garey & Johnson, A1.1 GT16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT16

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Can the vertices of G be partitioned into k ≤ K disjoints sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i is a perfect matching (consists entirely of vertices with degree one)?

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
QUESTION: Can the vertices of G be partitioned into k ≤ K disjoints sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i is a perfect matching (consists entirely of vertices with degree one)?
Reference: [Schaefer, 1978b]. Transformation from NOT-ALL-EQUAL 3SAT.
Comment: Remains NP-complete for K = 2 and for planar cubic graphs.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
