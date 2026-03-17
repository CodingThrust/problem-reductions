---
name: Problem
about: Propose a new problem type
title: "[Model] GraphHomomorphism"
labels: model
assignees: ''
---

## Motivation

GRAPH HOMOMORPHISM (P63) from Garey & Johnson, A1.4 GT52. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT52

**Mathematical definition:**

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2).
QUESTION: Can a graph isomorphic to H be obtained from G by a sequence of identifications of non-adjacent vertices, i.e., a sequence in which each step replaces two non-adjacent vertices u,v by a single vertex w adjacent to exactly those vertices that were previously adjacent to at least one of u and v?

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

INSTANCE: Graphs G = (V_1,E_1), H = (V_2,E_2).
QUESTION: Can a graph isomorphic to H be obtained from G by a sequence of identifications of non-adjacent vertices, i.e., a sequence in which each step replaces two non-adjacent vertices u,v by a single vertex w adjacent to exactly those vertices that were previously adjacent to at least one of u and v?

Reference: [Levin, 1973]. Transformation from GRAPH K-COLORABILITY.
Comment: Remains NP-complete for H fixed to be a triangle, but can be solved in polynomial time if H is just a single edge.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
