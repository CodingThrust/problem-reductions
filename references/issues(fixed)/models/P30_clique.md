---
name: Problem
about: Propose a new problem type
title: "[Model] Clique"
labels: model
assignees: ''
---

## Motivation

CLIQUE (P30) from Garey & Johnson, A1.2 GT19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT19

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Does G contain a clique of size K or more, i.e., a subset V' ⊆ V with |V'| ≥ K such that every two vertices in V' are joined by an edge in E?

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
QUESTION: Does G contain a clique of size K or more, i.e., a subset V' ⊆ V with |V'| ≥ K such that every two vertices in V' are joined by an edge in E?
Reference: [Karp, 1972]. Transformation from VERTEX COVER (see Chapter 3).
Comment: Solvable in polynomial time for graphs obeying any fixed degree bound d, for planar graphs, for edge graphs, for chordal graphs [Gavril, 1972], for comparability graphs [Even, Pnueli, and Lempel, 1972], for circle graphs [Gavril, 1973], and for circular arc graphs (given their representation as families of arcs) [Gavril, 1974a]. The variant in which, for a given r, 0 < r < 1, we are asked whether G contains a clique of size r|V| or more is NP-complete for any fixed value of r.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
