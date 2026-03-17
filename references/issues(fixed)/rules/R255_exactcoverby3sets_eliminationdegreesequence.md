---
name: Rule
about: Propose a new reduction rule
title: "[Rule] EXACT COVER BY 3-SETS to ELIMINATION DEGREE SEQUENCE"
labels: rule
assignees: ''
---

**Source:** EXACT COVER BY 3-SETS
**Target:** ELIMINATION DEGREE SEQUENCE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT47

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), sequence <d_1,d_2,...,d_{|V|}> of non-negative integers not exceeding |V|-1.
> QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that, for 1 ≤ i ≤ |V|, if f(v) = i then there are exactly d_i vertices u such that f(u) > i and {u,v} ∈ E?
>
> Reference: [Garey, Johnson, and Papadimitriou, 1977]. Transformation from EXACT COVER BY 3-SETS.
>
> Comment: The variant in which it is required that f be such that, for 1 ≤ i ≤ |V|, if f(v) = i then there are exactly d_i vertices u such that {u,v} ∈ E, is trivially solvable in polynomial time.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Garey, Johnson, and Papadimitriou, 1977]**: [`Garey1977e`] M. R. Garey and D. S. Johnson and C. H. Papadimitriou (1977). "Unpublished results".