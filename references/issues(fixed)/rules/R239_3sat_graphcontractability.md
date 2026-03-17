---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to GRAPH CONTRACTABILITY"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** GRAPH CONTRACTABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT51

## Reduction Algorithm

> INSTANCE: Graphs G = (V1,E1), H = (V2,E2).
> QUESTION: Can a graph isomorphic to H be obtained from G by a sequence of edge contractions, i.e., a sequence in which each step replaces two adjacent vertices u,v by a single vertex w adjacent to exactly those vertices that were previously adjacent to at least one of u and v?
>
> Reference: [Statman, 1976]. Transformation from 3SAT.
> Comment: Can be solved in polynomial time if H is a triangle.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Statman, 1976]**: [`Statman1976`] Richard Statman (1976). "private communication".