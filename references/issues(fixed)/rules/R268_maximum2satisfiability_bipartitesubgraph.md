---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MAXIMUM 2-SATISFIABILITY to BIPARTITE SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** MAXIMUM 2-SATISFIABILITY
**Target:** BIPARTITE SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT25

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that G' = (V,E') is bipartite?
>
> Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from MAXIMUM 2-SATISFIABILITY.
> Comment: Remains NP-complete for graphs with no vertex degree exceeding 3 and no triangles and/or if we require that the subgraph be connected [Yannakakis, 1978b]. Solvable in polynomial time if G is planar [Hadlock, 1975], [Orlova and Dorfman, 1972], or if K = |E|.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.
- **[Hadlock, 1975]**: [`Hadlock1975`] F. O. Hadlock (1975). "Finding a maximum cut of a planar graph in polynomial time". *SIAM Journal on Computing* 4, pp. 221–225.
- **[Orlova and Dorfman, 1972]**: [`Orlova1972`] G. I. Orlova and Y. G. Dorfman (1972). "Finding the maximum cut in a graph". *Engineering Cybernetics* 10, pp. 502–506.