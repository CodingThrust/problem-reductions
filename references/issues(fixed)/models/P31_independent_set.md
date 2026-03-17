---
name: Problem
about: Propose a new problem type
title: "[Model] IndependentSet"
labels: model
assignees: ''
---

## Motivation

INDEPENDENT SET (P31) from Garey & Johnson, A1.2 GT20. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT20

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Does G contain an independent set of size K or more, i.e., a subset V' ⊆ V such that |V'| ≥ K and such that no two vertices in V' are joined by an edge in E?

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
QUESTION: Does G contain an independent set of size K or more, i.e., a subset V' ⊆ V such that |V'| ≥ K and such that no two vertices in V' are joined by an edge in E?
Reference: Transformation from VERTEX COVER (see Chapter 3).
Comment: Remains NP-complete for cubic planar graphs [Garey, Johnson, and Stockmeyer, 1976], [Garey and Johnson, 1977a], [Maier and Storer, 1977], for edge graphs of directed graphs [Gavril, 1977a], for total graphs of bipartite graphs [Yannakakis and Gavril, 1978], and for graphs containing no triangles [Poljak, 1974]. Solvable in polynomial time for bipartite graphs (by matching, e.g., see [Harary, 1969]), for edge graphs (by matching), for graphs with no vertex degree exceeding 2, for chordal graphs [Gavril, 1972], for circle graphs [Gavril, 1973], for circular arc graphs (given their representation as families of arcs) [Gavril, 1974a], for comparability graphs [Golumbic, 1977], and for claw-free graphs [Minty, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
