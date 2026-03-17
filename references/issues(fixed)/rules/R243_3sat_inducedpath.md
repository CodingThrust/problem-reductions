---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to INDUCED PATH"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** INDUCED PATH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT23

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Is there a subset V' ⊆ V with |V'| ≥ K such that the subgraph induced by V' is a simple path on |V'| vertices?
>
> Reference: [Yannakakis, 1978c]. Transformation from 3SAT.
> Comment: Note that this is not a hereditary property, so the result is not implied by either of the previous two results. Remains NP-complete if G is bipartite. The same result holds for the variant in which "simple path" is replaced by "simple cycle." The problems of finding the longest simple path or longest simple cycle (not necessarily induced) are also NP-complete.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Yannakakis, 1978c]**: [`Yannakakis1978c`] Mihalis Yannakakis (1978). "private communication".