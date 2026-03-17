---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to MINIMUM MAXIMAL MATCHING"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** MINIMUM MAXIMAL MATCHING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT10

## GJ Source Entry

> [GT10] MINIMUM MAXIMAL MATCHING
> INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≤ K such that E' is a maximal matching, i.e., no two edges in E' share a common endpoint and every edge in E−E' shares a common endpoint with some edge in E'?
> Reference: [Yannakakis and Gavril, 1978]. Transformation from VERTEX COVER for cubic graphs.
> Comment: Remains NP-complete for planar graphs and for bipartite graphs, in both cases even if no vertex degree exceeds 3. The problem of finding a maximum "maximal matching" is just the usual graph matching problem and is solvable in polynomial time (e.g., see [Lawler, 1976a]).

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

- **[Yannakakis and Gavril, 1978]**: [`Yannakakis and Gavril1978`] Mihalis Yannakakis and Fanica Gavril (1978). "Edge dominating sets in graphs".
- **[Lawler, 1976a]**: [`Lawler1976a`] Eugene L. Lawler (1976). "Combinatorial Optimization: Networks and Matroids". Holt, Rinehart and Winston, New York.