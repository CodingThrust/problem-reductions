---
name: Rule
about: Propose a new reduction rule
title: "[Rule] OPTIMAL LINEAR ARRANGEMENT to ROOTED TREE ARRANGEMENT"
labels: rule
assignees: ''
---

**Source:** OPTIMAL LINEAR ARRANGEMENT
**Target:** ROOTED TREE ARRANGEMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT45

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K.
> QUESTION: Is there a rooted tree T = (U,F), with |U| = |V|, and a one-to-one function f: V → U such that for every edge {u,v} ∈ E there is a simple path from the root that includes both f(u) and f(v) and such that if d(x,y) is the number of edges on the path from x to y in T, then ∑_{u,v}∈E d(f(u),f(v)) ≤ K?
>
> Reference: [Gavril, 1977a]. Transformation from OPTIMAL LINEAR ARRANGEMENT.

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