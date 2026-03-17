---
name: Problem
about: Propose a new problem type
title: "[Model] GeneralizedKayles"
labels: model
assignees: ''
---

## Motivation

GENERALIZED KAYLES (P240) from Garey & Johnson, A8 GP3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP3

**Mathematical definition:**

INSTANCE: Graph G = (V,E).
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a vertex in the graph, removing that vertex and all vertices adjacent to it from the graph. Player 1 wins if and only if player 2 is the first player left with no vertices to choose from.

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

INSTANCE: Graph G = (V,E).
QUESTION: Does player 1 have a forced win in the following game played on G? Players alternate choosing a vertex in the graph, removing that vertex and all vertices adjacent to it from the graph. Player 1 wins if and only if player 2 is the first player left with no vertices to choose from.

Reference: [Schaefer, 1978a]. Transformation from QBF.
Comment: PSPACE-complete. The variant in which G = (V1 ∪ V2,E) is bipartite, with each edge involving one vertex from V1 and one from V2, and player i can only choose vertices from the set Vi (but still removes all adjacent vertices as before) is also PSPACE-complete. For a description of the game Kayles upon which this generalization is based, see [Conway, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
