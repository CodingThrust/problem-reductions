---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Path to Consecutive Sets"
labels: rule
assignees: ''
---

**Source:** Hamiltonian Path
**Target:** Consecutive Sets
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230

## GJ Source Entry

> [SR18] CONSECUTIVE SETS
> INSTANCE: Finite alphabet Σ, collection C = {Σ_1, Σ_2, ..., Σ_n} of subsets of Σ, and a positive integer K.
> QUESTION: Is there a string w E Σ* with |w| <= K such that, for each i, the elements of Σ_i occur in a consecutive block of |Σ_i| symbols of W?
> Reference: [Kou, 1977]. Transformation from HAMILTONIAN PATH.
> Comment: The variant in which we ask only that the elements of each Σ_i occur in a consecutive block of |Σ_i| symbols of the string ww (i.e., we allow blocks that circulate from the end of w back to its beginning) is also NP-complete [Booth, 1975]. If K is the number of distinct symbols in the Σ_i, then these problems are equivalent to determining whether a matrix has the consecutive ones property or the circular ones property and are solvable in polynomial time.

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