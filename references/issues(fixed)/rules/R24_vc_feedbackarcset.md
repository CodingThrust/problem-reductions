---
name: Rule
about: Propose a new reduction rule
title: "[Rule] VERTEX COVER to FEEDBACK ARC SET"
labels: rule
assignees: ''
---

**Source:** VERTEX COVER
**Target:** FEEDBACK ARC SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT8

## GJ Source Entry

> [GT8]  FEEDBACK ARC SET
> INSTANCE:  Directed graph G = (V,A), positive integer K ≤ |A|.
> QUESTION:  Is there a subset A' ⊆ A with |A'| ≤ K such that A' contains at least one arc from every directed cycle in G?
>
> Reference:  [Karp, 1972]. Transformation from VERTEX COVER.
> Comment:  Remains NP-complete for digraphs in which no vertex has total indegree and out-degree more than 3, and for edge digraphs [Gavril, 1977a]. Solvable in polynomial time for planar digraphs [Luchesi, 1976]. The corresponding problem for undirected graphs is trivially solvable in polynomial time.

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
- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.
- **[Luchesi, 1976]**: [`Luchesi1976`] Claudio L. Luchesi (1976). "A Minimax Equality for Directed Graphs". University of Waterloo.