---
name: Rule
about: Propose a new reduction rule
title: "[Rule] GRAPH 3-COLORABILITY to CLUSTERING"
labels: rule
assignees: ''
---

**Source:** GRAPH 3-COLORABILITY
**Target:** CLUSTERING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS9

## GJ Source Entry

> [MS9]  CLUSTERING
> INSTANCE:  Finite set X, a distance d(x,y) ∈ Z0+ for each pair x,y ∈ X, and two positive integers K and B.
> QUESTION:  Is there a partition of X into disjoint sets X1,X2,...,Xk such that, for 1 ≤ i ≤ k and all pairs x,y ∈ Xi, d(x,y) ≤ B?
> Reference:  [Brucker, 1978]. Transformation from GRAPH 3-COLORABILITY.
> Comment:  Remains NP-complete even for fixed K = 3 and all distances in {0,1}. Solvable in polynomial time for K = 2. Variants in which we ask that the sum, over all Xi, of max{d(x,y): x,y ∈ Xi} or of ∑x,y ∈ Xi d(x,y) be at most B, are similarly NP-complete (with the last one NP-complete even for K = 2).

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

- **[Brucker, 1978]**: [`Brucker1978`] P. Brucker (1978). "On the complexity of clustering problems". In: *Optimierung und Operations Research*. Springer.