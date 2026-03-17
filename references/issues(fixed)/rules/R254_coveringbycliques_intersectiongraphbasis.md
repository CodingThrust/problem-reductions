---
name: Rule
about: Propose a new reduction rule
title: "[Rule] COVERING BY CLIQUES to INTERSECTION GRAPH BASIS"
labels: rule
assignees: ''
---

**Source:** COVERING BY CLIQUES
**Target:** INTERSECTION GRAPH BASIS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT59

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
> QUESTION: Is G the intersection graph for a family of sets whose union has cardinality K or less, i.e., is there a K-element set S and for each v ∈ V a subset S[v] ⊆ S such that {u,v} ∈ E if and only if S[u] and S[v] are not disjoint?
>
> Reference: [Kou, Stockmeyer, and Wong, 1978]. Transformation from COVERING BY CLIQUES.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Kou, Stockmeyer, and Wong, 1978]**: [`Kou1978`] Lawrence T. Kou and Lawrence J. Stockmeyer and Chak K. Wong (1978). "Covering edges by cliques with regard to keyword conflicts and intersection graphs". *Communications of the ACM* 21, pp. 135–138.