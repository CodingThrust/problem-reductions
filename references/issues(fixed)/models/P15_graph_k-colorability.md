---
name: Problem
about: Propose a new problem type
title: "[Model] GraphKColorability(chromaticNumber)"
labels: model
assignees: ''
---

## Motivation

GRAPH K-COLORABILITY (CHROMATIC NUMBER) (P15) from Garey & Johnson, A1.1 GT4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT4

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is G K-colorable, i.e., does there exist a function f: V → {1,2, . . . ,K} such that f(u) ≠ f(v) whenever {u,v} ∈ E?

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
QUESTION: Is G K-colorable, i.e., does there exist a function f: V → {1,2, . . . ,K} such that f(u) ≠ f(v) whenever {u,v} ∈ E?
Reference: [Karp, 1972]. Transformation from 3SAT.
Comment: Solvable in polynomial time for K = 2, but remains NP-complete for all fixed K ≥ 3 and, for K = 3, for planar graphs having no vertex degree exceeding 4 [Garey, Johnson, and Stockmeyer, 1976]. Also remains NP-complete for K = 3 if G is an intersection graph for straight line segments in the plane [Ehrlich, Even, and Tarjan, 1976]. For arbitrary K, the problem is NP-complete for circle graphs and circular arc graphs (even given their representation as families of arcs), although for circular arc graphs the problem is solvable in polynomial time for any fixed K (given their representation) [Garey, Johnson, Miller, and Papadimitriou, 1978]. The general problem can be solved in polynomial time for comparability graphs [Even, Pnueli, and Lempel, 1972], for chordal graphs [Gavril, 1972], for (3,1) graphs [Walsh and Burkhard, 1977], and for graphs having no vertex degree exceeding 3 [Brooks, 1941].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
