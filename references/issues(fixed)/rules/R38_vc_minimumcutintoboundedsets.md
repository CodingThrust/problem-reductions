---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to MINIMUM CUT INTO BOUNDED SETS"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** MINIMUM CUT INTO BOUNDED SETS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND17, p.210

## GJ Source Entry

> [ND17] MINIMUM CUT INTO BOUNDED SETS
> INSTANCE: Graph G=(V,E), positive integers K and J.
> QUESTION: Can V be partitioned into J disjoint sets V_1,...,V_J such that each |V_i|≤K and the number of edges with endpoints in different parts is minimized, i.e., such that the number of such edges is no more than some bound B?
> Reference: [Garey and Johnson, 1979]. Transformation from VERTEX COVER.
> Comment: NP-complete even for J=2.

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

- **[Garey and Johnson, 1979]**: [`Garey19xx`] M. R. Garey and D. S. Johnson (1979). "Unpublished results".