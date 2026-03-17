---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoForests"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO FORESTS (P25) from Garey & Johnson, A1.1 GT14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT14

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Can the vertices of G be partitioned into k ≤ K disjoint sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i contains no circuits?

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
QUESTION: Can the vertices of G be partitioned into k ≤ K disjoint sets V_1, V_2, . . . , V_k such that, for 1 ≤ i ≤ k, the subgraph induced by V_i contains no circuits?
Reference: [Garey and Johnson, ——]. Transformation from GRAPH 3-COLORABILITY.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
