---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoTriangles"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO TRIANGLES (P22) from Garey & Johnson, A1.1 GT11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT11

**Mathematical definition:**

INSTANCE: Graph G = (V,E), with |V| = 3q for some integer q.
QUESTION: Can the vertices of G be partitioned into q disjoint sets V_1, V_2, . . . , V_q, each containing exactly 3 vertices, such that for each V_i = {u_i, v_i, w_i}, 1 ≤ i ≤ q, all three of the edges {u_i,v_i}, {u_i,w_i}, and {v_i,w_i} belong to E?

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

INSTANCE: Graph G = (V,E), with |V| = 3q for some integer q.
QUESTION: Can the vertices of G be partitioned into q disjoint sets V_1, V_2, . . . , V_q, each containing exactly 3 vertices, such that for each V_i = {u_i, v_i, w_i}, 1 ≤ i ≤ q, all three of the edges {u_i,v_i}, {u_i,w_i}, and {v_i,w_i} belong to E?
Reference: [Schaefer, 1974]. Transformation from 3DM (see Chapter 3).
Comment: See next problem for a generalization.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
