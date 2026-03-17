---
name: Rule
about: Propose a new reduction rule
title: "[Rule] INTERVAL GRAPH COMPLETION to PATH GRAPH COMPLETION"
labels: rule
assignees: ''
---

**Source:** INTERVAL GRAPH COMPLETION
**Target:** PATH GRAPH COMPLETION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT36

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), non-negative integer K.
> QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') is the intersection graph of a family of paths on an undirected tree?
>
> Reference: [Gavril, 1977b]. Transformation from INTERVAL GRAPH COMPLETION.
>
> Comment: Corresponding problem in which G' must be the intersection graph of a family of directed paths on an oriented tree (i.e., rooted, with all arcs directed away from the root) is also NP-complete.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Gavril, 1977b]**: [`Gavril1977b`] F. Gavril (1977). "Private communication".