---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Path to Consecutive Block Minimization"
labels: rule
assignees: ''
---

**Source:** Hamiltonian Path
**Target:** Consecutive Block Minimization
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230

## GJ Source Entry

> [SR17] CONSECUTIVE BLOCK MINIMIZATION
> INSTANCE: An m×n matrix A of 0's and 1's and a positive integer K.
> QUESTION: Is there a permutation of the columns of A that results in a matrix B having at most K blocks of consecutive 1's, i.e., having at most K entries b_{ij} such that b_{ij} = 1 and either b_{i,j+1} = 0 or j = n?
> Reference: [Kou, 1977]. Transformation from HAMILTONIAN PATH.
> Comment: Remains NP-complete if "j = n" is replaced by "j = n and b_{i,1} = 0" [Booth, 1975]. If K equals the number of rows of A that are not all 0, then these problems are equivalent to testing A for the consecutive ones property or the circular ones property, respectively, and can be solved in polynomial time.

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

- **[Kou, 1977]**: [`Kou1977`] Lawrence T. Kou (1977). "Polynomial complete consecutive information retrieval problems". *SIAM Journal on Computing* 6, pp. 67–75.
- **[Booth, 1975]**: [`Booth1975`] K. S. Booth (1975). "{PQ} Tree Algorithms". University of California, Berkeley.