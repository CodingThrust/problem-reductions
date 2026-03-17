---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to MINIMUM WEIGHT AND/OR GRAPH SOLUTION"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** MINIMUM WEIGHT AND/OR GRAPH SOLUTION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS16

## GJ Source Entry

> [MS16]  MINIMUM WEIGHT AND/OR GRAPH SOLUTION
> INSTANCE:  Directed acyclic graph G = (V,A) with a single vertex s ∈ V having in-degree 0, assignment f(v) ∈ {and,or} for each v ∈ V having nonzero out-degree, weight w(a) ∈ Z+ for each a ∈ A, and a positive integer K.
> QUESTION:  Is there a subgraph G' = (V',A') of G such that s ∈ V', such that if v ∈ V' and f(v) = and then all arcs leaving v in A belong to A', such that if v ∈ V' and f(v) = or then at least one of the arcs leaving v in A belongs to A', and such that the sum of the weights of the arcs in A' does not exceed K?
> Reference:  [Sahni, 1974]. Transformation from X3C.
> Comment:  Remains NP-complete even if w(a) = 1 for all a ∈ A [Garey and Johnson, ——]. The general problem is solvable in polynomial time for rooted directed trees by dynamic programming.

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

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.
- **[Garey and Johnson, ——]**: *(not found in bibliography)*