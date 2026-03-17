---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to DIRECTED ELIMINATION ORDERING"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** DIRECTED ELIMINATION ORDERING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT46

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A), non-negative integer K.
> QUESTION: Is there an elimination ordering for G with fill-in K or less, i.e., a one-to-one function f: V → {1,2,...,|V|} such that there are at most K pairs of vertices (u,v) ∈ (V×V)-A with the property that G contains a directed path from u to v that only passes through vertices w satisfying f(w) < min{f(u),f(v)}?
>
> Reference: [Rose and Tarjan, 1978]. Transformation from 3SAT.
>
> Comment: Problem arises in performing Gaussian elimination on sparse matrices. Solvable in polynomial time for K = 0. The analogous problem for undirected graphs (symmetric matrices) is equivalent to CHORDAL GRAPH COMPLETION and is open as to complexity.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Rose and Tarjan, 1978]**: [`Rose1978`] D. J. Rose and R. E. Tarjan (1978). "Algorithmic aspects of vertex elimination on directed graphs". *SIAM Journal on Applied Mathematics* 34, pp. 176–197.