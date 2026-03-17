---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to EDGE-SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** EDGE-SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT28

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that the subgraph G' = (V,E') is an edge graph, i.e., there exists a graph H = (U,F) such that G' is isomorphic to the graph having vertex set F and edge set consisting of all pairs {e,f} such that the edges e and f share a common endpoint in H?
>
> Reference: [Yannakakis, 1978b]. Transformation from 3SAT.
> Comment: Remains NP-complete even if G has no vertex with degree exceeding 4. If we require that the subgraph be connected, the degree bound for NP-completeness can be reduced to 3. Edge graphs can be recognized in polynomial time, e.g., see [Harary, 1969] (under the term "line graphs").

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.
- **[Harary, 1969]**: [`Harary1969`] F. Harary (1969). "Graph Theory". Addison-Wesley, Reading, MA.