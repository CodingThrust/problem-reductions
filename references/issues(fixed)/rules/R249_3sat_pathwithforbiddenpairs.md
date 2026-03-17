---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PATH WITH FORBIDDEN PAIRS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** PATH WITH FORBIDDEN PAIRS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT54

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A), specified vertices s,t ∈ V, collection C = {(a1,b1), . . . ,(an,bn)} of pairs of vertices from V.
> QUESTION: Is there a directed path from s to t in G that contains at most one vertex from each pair in C?
>
> Reference: [Gabow, Maheshwari, and Osterweil, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete even if G is acyclic with no in- or out-degree exceeding 2. Variant in which the "forbidden pairs" are arcs instead of vertices is also NP-complete under the same restrictions. Both problems remain NP-complete even if all the given pairs are required to be disjoint.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Gabow, Maheshwari, and Osterweil, 1976]**: [`Gabow1976b`] H. N. Gabow and S. N. Maheshwari and L. Osterweil (1976). "On two problems in the generation of program test paths". *IEEE Transactions on Software Engineering* SE-2, pp. 227–231.