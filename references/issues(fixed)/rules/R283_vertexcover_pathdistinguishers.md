---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to PATH DISTINGUISHERS"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** PATH DISTINGUISHERS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT60

## Reduction Algorithm

> INSTANCE: Acyclic directed graph G = (V,A), specified vertices s,t ∈ V, positive integer K ≤ |A|.
> QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that, for any pair p1,p2 of paths from s to t in G, there is some arc in A' that is in one of p1 and p2 but not both?
>
> Reference: [Maheshwari, 1976]. Transformation from VERTEX COVER.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Maheshwari, 1976]**: [`Maheshwari1976`] S. N. Maheshwari (1976). "Traversal marker placement problems are {NP}-complete". Dept. of Computer Science, University of Colorado.