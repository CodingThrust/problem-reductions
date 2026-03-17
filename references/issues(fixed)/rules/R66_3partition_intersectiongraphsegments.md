---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-PARTITION to INTERSECTION GRAPH FOR SEGMENTS ON A GRID"
labels: rule
assignees: ''
---

**Source:** 3-PARTITION
**Target:** INTERSECTION GRAPH FOR SEGMENTS ON A GRID
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND46, p.219

## GJ Source Entry

> [ND46] INTERSECTION GRAPH FOR SEGMENTS ON A GRID
> INSTANCE: Graph G=(V,E), positive integers M,N.
> QUESTION: Is G the intersection graph for a set of line segments on an M×N grid, i.e., is there a one-to-one function f that maps each v∈V to a line segment f(v)=[(x,y),(z,w)], where 1≤x≤z≤M, 1≤y≤w≤N, and either x=z or y=w, such that {u,v}∈E if and only if the line segments f(u) and f(v) intersect?
> Reference: [Gavril, 1977a]. Transformation from 3-PARTITION.
> Comment: The analogous problem, which asks if G is the intersection graph for a set of rectangles on an M×N grid, is also NP-complete [Gavril, 1977a].

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.