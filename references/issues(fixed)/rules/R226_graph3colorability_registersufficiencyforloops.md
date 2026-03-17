---
name: Rule
about: Propose a new reduction rule
title: "[Rule] permutation generation to REGISTER SUFFICIENCY FOR LOOPS"
labels: rule
assignees: ''
---

**Source:** permutation generation
**Target:** REGISTER SUFFICIENCY FOR LOOPS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO3

## GJ Source Entry

> [PO3]  REGISTER SUFFICIENCY FOR LOOPS
> INSTANCE:  Set V of loop variables, a loop length N ∈ Z+, for each variable v ∈ V a start time s(v) ∈ Z0+ and a duration l(v) ∈ Z+, and a positive integer K.
> QUESTION:  Can the loop variables be safely stored in K registers, i.e., is their an assignment f: V → {1,2,...,K} such that if f(v) = f(u) for some u≠v ∈ V, then s(u) ≤ s(v) implies s(u) + l(u) ≤ s(v) and s(v) + l(v)(modN) ≤ s(u)?
> Reference:  [Garey, Johnson, Miller, and Papadimitriou, 1978]. Transformation from permutation generation.
> Comment:  Solvable in polynomial time for any fixed K.

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

- **[Garey, Johnson, Miller, and Papadimitriou, 1978]**: [`Garey1978c`] M. R. Garey and D. S. Johnson and G. L. Miller and C. H. Papadimitriou (1978). "Unpublished results".