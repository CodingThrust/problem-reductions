---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GRAPH K-COLORABILITY to GRAPH HOMOMORPHISM"
labels: rule
assignees: ''
---

**Source:** GRAPH K-COLORABILITY
**Target:** GRAPH HOMOMORPHISM
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT52

## Reduction Algorithm

> INSTANCE: Graphs G = (V1,E1), H = (V2,E2).
> QUESTION: Can a graph isomorphic to H be obtained from G by a sequence of identifications of non-adjacent vertices, i.e., a sequence in which each step replaces two non-adjacent vertices u,v by a single vertex w adjacent to exactly those vertices that were preciously adjacent to at least one of u and v?
>
> Reference: [Levin, 1973]. Transformation from GRAPH K-COLORABILITY.
> Comment: Remains NP-complete for H fixed to be a triangle, but can be solved in polynomial time if H is just a single edge.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Levin, 1973]**: [`Levin1973`] Leonid A. Levin (1973). "Universal sorting problems". *Problemy Peredaci Informacii* 9, pp. 115–116.