---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MINIMUM MAXIMAL MATCHING to MATRIX DOMINATION"
labels: rule
assignees: ''
---

**Source:** MINIMUM MAXIMAL MATCHING
**Target:** MATRIX DOMINATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS12

## GJ Source Entry

> [MS12]  MATRIX DOMINATION
> INSTANCE:  An n×n matrix M with entries from {0,1}, and a positive integer K.
> QUESTION:  Is there a set of K or fewer non-zero entries in M that dominate all others, i.e., s subset C ⊆ {1,2,...,n}×{1,2,...,n} with |C| ≤ K such that Mij = 1 for all (i,j) ∈ C and such that, whenever Mij = 1, there exists an (i',j') ∈ C for which either i = i' or j = j'?
> Reference:  [Yannakakis and Gavril, 1978]. Transformation from MINIMUM MAXIMAL MATCHING.
> Comment:  Remains NP-complete even if M is upper triangular.

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