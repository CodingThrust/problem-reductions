---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to DOMINATING SET"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** DOMINATING SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT2

## GJ Source Entry

> [GT2]  DOMINATING SET
> INSTANCE:  Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION:  Is there a dominating set of size K or less for G, i.e., a subset V' ⊆ V with |V'| ≤ K such that for all u ∈ V−V' there is a v ∈ V' for which {u,v} ∈ E?
>
> Reference:  Transformation from VERTEX COVER.
> Comment:  Remains NP-complete for planar graphs with maximum vertex degree 3 and planar graphs that are regular of degree 4 [Garey and Johnson, ——]. Variation in which the subgraph induced by V' is required to be connected is also NP-complete, even for planar graphs that are regular of degree 4 [Garey and Johnson, ——]. Also NP-complete if V' is required to be both a dominating set and an independent set. Solvable in polynomial time for trees [Cockayne, Goodman, and Hedetniemi, 1975]. The related EDGE DOMINATING SET problem, where we ask for a set E' ⊆ E of K or fewer edges such that every edge in E shares at least one endpoint with some edge in E', is NP-complete, even for planar or bipartite graphs of maximum degree 3, but can be solved in polynomial time for trees [Yannakakis and Gavril, 1978], [Mitchell and Hedetniemi, 1977].

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
- **[Cockayne, Goodman, and Hedetniemi, 1975]**: [`Cockayne1975a`] E. Cockayne and S. Goodman and S. Hedetniemi (1975). "A linear algorithm for the domination number of a tree". *Information Processing Letters* 4, pp. 41–44.
- **[Yannakakis and Gavril, 1978]**: [`Yannakakis and Gavril1978`] Mihalis Yannakakis and Fanica Gavril (1978). "Edge dominating sets in graphs".
- **[Mitchell and Hedetniemi, 1977]**: [`Mitchell1977`] Sandra Mitchell and Steven Hedetniemi (1977). "Edge domination in trees". In: *Proceedings of the 8th Southeastern Conference on Combinatorics, Graph Theory, and Computing*, pp. 489–509. Utilitas Mathematica Publishing.