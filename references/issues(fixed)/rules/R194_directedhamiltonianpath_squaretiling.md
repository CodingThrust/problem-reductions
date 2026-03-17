---
name: Rule
about: Propose a new reduction rule
title: "[Rule] DIRECTED HAMILTONIAN PATH to SQUARE-TILING"
labels: rule
assignees: ''
---

**Source:** DIRECTED HAMILTONIAN PATH
**Target:** SQUARE-TILING
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.257

## GJ Source Entry

> [GP13] SQUARE-TILING
> INSTANCE: Set C of "colors," collection T ⊆ C^4 of "tiles" (where <a,b,c,d> denotes a tile whose top, right, bottom, and left sides are colored a, b, c, and d, respectively), and a positive integer N ≤ |C|.
> QUESTION: Is there a tiling of an N×N square using the tiles in T, i.e., an assignment of a tile A(i,j) E T to each ordered pair i,j, 1 ≤ i ≤ N, 1 ≤ j ≤ N, such that (1) if f(i,j) = <a,b,c,d> and f(i+1,j) = <a',b',c',d'>, then a = c', and (2) if f(i,j) = <a,b,c,d> and f(i,j+1) = <a',b',c',d'>, then b = d'?
> Reference: [Garey, Johnson, and Papadimitriou, 1977]. Transformation from DIRECTED HAMILTONIAN PATH.
> Comment: Variant in which we ask if T can be used to tile the entire plane (Z×Z) "periodically" with period less than N is also NP-complete. In general, the problem of whether a set of tiles can be used to tile the plane is undecidable [Berger, 1966], as is the problem of whether a set of tiles can be used to tile the plane periodically.

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

- **[Garey, Johnson, and Papadimitriou, 1977]**: [`Garey1977e`] M. R. Garey and D. S. Johnson and C. H. Papadimitriou (1977). "Unpublished results".
- **[Berger, 1966]**: [`Berger1966`] R. Berger (1966). "The Undecidability of the Domino Problem". American Mathematical Society, Providence, RI.