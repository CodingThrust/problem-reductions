---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to BALANCED COMPLETE BIPARTITE SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** CLIQUE
**Target:** BALANCED COMPLETE BIPARTITE SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT24

## GJ Source Entry

> [GT24]  BALANCED COMPLETE BIPARTITE SUBGRAPH
> INSTANCE:  Bipartite graph G = (V,E), positive integer K ≤ |V|.
> QUESTION:  Are there two disjoint subsets V_1, V_2 ⊆ V such that |V_1| = |V_2| = K and such that u ∈ V_1, v ∈ V_2 implies that {u,v} ∈ E?
>
> Reference:  [Garey and Johnson, ——]. Transformation from CLIQUE.
> Comment:  The related problem in which the requirement "|V_1| = |V_2| = K" is replaced by "|V_1|+|V_2| = K" is solvable in polynomial time for bipartite graphs (because of the connection between matchings and independent sets in such graphs, e.g., see [Harary, 1969]), but is NP-complete for general graphs [Yannakakis, 1978b].

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

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Harary, 1969]**: [`Harary1969`] F. Harary (1969). "Graph Theory". Addison-Wesley, Reading, MA.
- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.