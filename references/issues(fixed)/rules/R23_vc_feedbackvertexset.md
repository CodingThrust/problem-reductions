---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to FEEDBACK VERTEX SET"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** FEEDBACK VERTEX SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT7

## GJ Source Entry

> [GT7]  FEEDBACK VERTEX SET
> INSTANCE:  Directed graph G = (V,A), positive integer K ≤ |V|.
> QUESTION:  Is there a subset V' ⊆ V with |V'| ≤ K such that V' contains at least one vertex from every directed cycle in G?
>
> Reference:  [Karp, 1972]. Transformation from VERTEX COVER.
> Comment:  Remains NP-complete for digraphs having no in- or out-degree exceeding 2, for planar digraphs with no in- or out-degree exceeding 3 [Garey and Johnson, ——], and for edge digraphs [Gavril, 1977a], but can be solved in polynomial time for reducible graphs [Shamir, 1977]. The corresponding problem for undirected graphs is also NP-complete.

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

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.
- **[Shamir, 1977]**: [`Shamir1977`] Adi Shamir (1977). "Finding minimum cutsets in reducible graphs". Laboratory for Computer Science, Massachusetts Institute of Technology.