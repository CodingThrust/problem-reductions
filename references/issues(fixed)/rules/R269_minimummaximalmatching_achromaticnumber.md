---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MINIMUM MAXIMAL MATCHING to ACHROMATIC NUMBER"
labels: rule
assignees: ''
---

**Source:** MINIMUM MAXIMAL MATCHING
**Target:** ACHROMATIC NUMBER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT5

## GJ Source Entry

> [GT5] ACHROMATIC NUMBER
> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Does G have achromatic number K or greater, i.e., is there a partition of V into disjoint sets V_1, V_2, . . . , V_k, k ≥ K, such that each V_i is an independent set for G (no two vertices in V_i are joined by an edge in E) and such that, for each pair of distinct sets V_i, V_j, V_i ∪ V_j is not an independent set for G?
> Reference: [Yannakakis and Gavril, 1978]. Transformation from MINIMUM MAXIMAL MATCHING.
> Comment: Remains NP-complete even if G is the complement of a bipartite graph and hence has no independent set of more than two vertices.

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