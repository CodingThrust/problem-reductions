---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to FAULT DETECTION IN DIRECTED GRAPHS"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** FAULT DETECTION IN DIRECTED GRAPHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS18

## GJ Source Entry

> [MS18]  FAULT DETECTION IN DIRECTED GRAPHS
> INSTANCE:  Directed acyclic graph G = (V,A), with I ⊆ V denoting those vertices with in-degree 0 and O ⊆ V denoting those vertices with out-degree 0, and a positive integer K.
> QUESTION:  Is there a "test set" of size K or less that can detect every "single fault" in G, i.e., is there a subset T ⊆ I×O with |T| ≤ K such that, for every v ∈ V, there exists some pair (u1,u2) ∈ T such that v is on a directed path from u1 to u2 in G?
> Reference:  [Ibaraki, Kameda, and Toida, 1977]. Transformation from X3C.
> Comment:  Remains NP-complete even if |O| = 1. Variant in which we ask that T be sufficient for "locating" any single fault, i.e., that for every pair v,v' ∈ V there is some (u1,u2) ∈ T such that v is on a directed path from u1 to u2 but v' is on no such path, is also NP-complete for |O| = 1. Both problems can be solved in polynomial time if K ≥ |I|·|O|.

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

- **[Ibaraki, Kameda, and Toida, 1977]**: [`Ibaraki1977`] Toshihide Ibaraki and T. Kameda and Shmuel Toida (1977). "{NP}-complete diagnosis problems on systems graphs".