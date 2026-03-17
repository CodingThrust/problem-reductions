---
name: Problem
about: Propose a new problem type
title: "[Model] GeneralizedHex"
labels: model
assignees: ''
---

## Motivation

GENERALIZED HEX (P238) from Garey & Johnson, A8 GP1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP1

**Mathematical definition:**

INSTANCE: Graph G = (V,E) and two specified vertices s,t ∈ V.
QUESTION: Does player 1 have a forced win in the following game played on G? The players alternate choosing a vertex from V−{s,t}, with those chosen by player 1 being colored "blue" and those chosen by player 2 being colored "red." Play continues until all such vertices have been colored, and player 1 wins if and only if there is a path from s to t in G that passes through only blue vertices.

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

INSTANCE: Graph G = (V,E) and two specified vertices s,t ∈ V.
QUESTION: Does player 1 have a forced win in the following game played on G? The players alternate choosing a vertex from V−{s,t}, with those chosen by player 1 being colored "blue" and those chosen by player 2 being colored "red." Play continues until all such vertices have been colored, and player 1 wins if and only if there is a path from s to t in G that passes through only blue vertices.

Reference: [Even and Tarjan, 1976]. Transformation from QBF.
Comment: PSPACE-complete. The variant in which players alternate choosing an edge instead of a vertex, known as "the Shannon switching game on edges," can be solved in polynomial time [Bruno and Weinberg, 1970]. If G is a directed graph and player 1 wants a "blue" directed path from s to t, both the vertex selection game and the arc selection game are PSPACE-complete [Even and Tarjan, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
