---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION INTO CLIQUES to COVERING BY CLIQUES"
labels: rule
assignees: ''
---

**Source:** PARTITION INTO CLIQUES
**Target:** COVERING BY CLIQUES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT17

## GJ Source Entry

> [GT17] COVERING BY CLIQUES
> INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
> QUESTION: Are there k ≤ K subsets V_1, V_2, . . . , V_k of V such that each V_i induces a complete subgraph of G and such that for each edge {u,v} ∈ E there is some V_i that contains both u and v?
> Reference: [Kou, Stockmeyer, and Wong, 1978], [Orlin, 1976]. Transformation from PARTITION INTO CLIQUES.

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

- **[Kou, Stockmeyer, and Wong, 1978]**: [`Kou1978`] Lawrence T. Kou and Lawrence J. Stockmeyer and Chak K. Wong (1978). "Covering edges by cliques with regard to keyword conflicts and intersection graphs". *Communications of the ACM* 21, pp. 135–138.
- **[Orlin, 1976]**: [`Orlin1976`] J. Orlin (1976). "Contentment in graph theory: covering graphs with cliques".