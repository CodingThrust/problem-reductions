---
name: Rule
about: Propose a new reduction rule
title: "[Rule] BIPARTITE SUBGRAPH to TRANSITIVE SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** BIPARTITE SUBGRAPH
**Target:** TRANSITIVE SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT29

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
> QUESTION: Is there a subset A' ⊆ A with |A'| ≥ K such that G' = (V,A') is transitive, i.e., for all pairs u,v ∈ V, if there exists a w ∈ V for which (u,w),(w,v) ∈ A', then (u,v) ∈ A'?
>
> Reference: [Yannakakis, 1978b] Transformation from BIPARTITE SUBGRAPH with no triangles.
> Comment: The variant in which G is undirected and we ask for a subgraph that is a "comparability graph," i.e., can be made into a transitive digraph by directing each of its edges in one of the two possible directions, is also NP-complete, even if G has no vertex with degree exceeding 3. For both problems, the variant in which we require the subgraph to be connected is also NP-complete.

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