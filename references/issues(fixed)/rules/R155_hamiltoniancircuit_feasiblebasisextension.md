---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Circuit to Feasible Basis Extension"
labels: rule
assignees: ''
---

**Source:** Hamiltonian Circuit
**Target:** Feasible Basis Extension
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.246

## GJ Source Entry

> [MP4] FEASIBLE BASIS EXTENSION
> INSTANCE: An m×n integer matrix A, m < n, a column vector a-bar of length m, and a subset S of the columns of A with |S| < m.
> QUESTION: Is there a feasible basis B for Ax-bar = a-bar, x-bar >= 0, i.e., a nonsingular m×m submatrix B of A such that B^{-1}a-bar >= 0, and such that B contains all the columns in S?
> Reference: [Murty, 1972]. Transformation from HAMILTONIAN CIRCUIT.

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

- **[Murty, 1972]**: [`Murty1972`] K. G. Murty (1972). "A fundamental problem in linear inequalities with applications to the traveling salesman problem". *Mathematical Programming* 2, pp. 296–308.