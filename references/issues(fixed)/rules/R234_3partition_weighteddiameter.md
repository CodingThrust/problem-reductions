---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-PARTITION to WEIGHTED DIAMETER"
labels: rule
assignees: ''
---

**Source:** 3-PARTITION
**Target:** WEIGHTED DIAMETER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT65

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), collection C of |E| not necessarily distinct non-negative integers, positive integer K.
> QUESTION: Is there a one-to-one function f: E → C such that, if f(e) is taken as the length of edge e, then G has diameter K or less, i.e., every pair of points u,v ∈ V is joined by a path in G of length K or less.
>
> Reference: [Perl and Zaks, 1978]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense, even if G is a tree. The variant in which "diameter" is replaced by "radius" has the same complexity. If C consists entirely of 0's and 1's, then both the diameter and radius versions are solvable in polynomial time for trees, but are NP-complete for general graphs, even if K is fixed at 2 (diameter) or 1 (radius). The variant in which we ask for an assignment yielding diameter K or greater is NP-complete in the strong sense for general graphs, is solvable in polynomial time for trees in the diameter case, and is NP-complete for trees in the radius case.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Perl and Zaks, 1978]**: [`Perl1978b`] Y. Perl and S. Zaks (1978). "".