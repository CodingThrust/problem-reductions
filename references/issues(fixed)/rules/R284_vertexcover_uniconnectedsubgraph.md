---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to UNICONNECTED SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** UNICONNECTED SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT30

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
> QUESTION: Is there a subset A' ⊆ A with |A'| ≥ K such that G' = (V,A') has at most one directed path between any pair of vertices?
>
> Reference: [Maheshwari, 1976]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete for acyclic directed graphs.

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