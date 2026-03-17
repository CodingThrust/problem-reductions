---
name: Rule
about: Propose a new reduction rule
title: "[Rule] CLIQUE to LARGEST COMMON SUBGRAPH"
labels: rule
assignees: ''
---

**Source:** CLIQUE
**Target:** LARGEST COMMON SUBGRAPH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT49

## Reduction Algorithm

> INSTANCE: Graphs G = (V1,E1), H = (V2,E2), positive integer K.
> QUESTION: Do there exist subsets E1' ⊆ E1 and E2' ⊆ E2 with |E1'| = |E2'| ≥ K such that the two subgraphs G' = (V1,E1') and H' = (V2,E2') are isomorphic?
>
> Reference: Transformation from CLIQUE.
> Comment: Can be solved in polynomial time if both G and H are trees [Edmonds and Matula, 1975].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Edmonds and Matula, 1975]**: [`Edmonds1975`] J. Edmonds and D. W. Matula (1975). "Private communication".