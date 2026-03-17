---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to PARTIAL FEEDBACK EDGE SET"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** PARTIAL FEEDBACK EDGE SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT9

## GJ Source Entry

> [GT9] PARTIAL FEEDBACK EDGE SET
> INSTANCE: Graph G = (V,E), positive integers K ≤ |E| and L ≤ |V|.
> QUESTION: Is there a subset E' ⊆ E with |E'| ≤ K such that E' contains at least one edge from every circuit of length L or less in G?
> Reference: [Yannakakis, 1978b]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete for any fixed L ≥ 3 and for bipartite graphs (with fixed L ≥ 4). However, if L = |V|, i.e., if we ask that E' contain an edge from every cycle in G, then the problem is trivially solvable in polynomial time.

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

- **[Yannakakis, 1978b]**: [`Yannakakis1978b`] Mihalis Yannakakis (1978). "Node- and edge-deletion {NP}-complete problems". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 253–264. Association for Computing Machinery.