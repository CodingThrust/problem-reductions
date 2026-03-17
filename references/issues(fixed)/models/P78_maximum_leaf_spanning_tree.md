---
name: Problem
about: Propose a new problem type
title: "[Model] MaximumLeafSpanningTree"
labels: model
assignees: ''
---

## Motivation

MAXIMUM LEAF SPANNING TREE (P78) from Garey & Johnson, A2 ND2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND2

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a spanning tree for G in which K or more vertices have degree 1?

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
QUESTION: Is there a spanning tree for G in which K or more vertices have degree 1?

Reference: [Garey and Johnson, ——]. Transformation from DOMINATING SET.
Comment: Remains NP-complete if G is regular of degree 4 or if G is planar with no degree exceeding 4.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
