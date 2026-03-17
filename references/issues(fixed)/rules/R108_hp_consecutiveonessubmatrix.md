---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hamiltonian Path to Consecutive Ones Submatrix"
labels: rule
assignees: ''
---

**Source:** Hamiltonian Path
**Target:** Consecutive Ones Submatrix
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.229

## GJ Source Entry

> [SR14] CONSECUTIVE ONES SUBMATRIX
> INSTANCE: An m×n matrix A of 0's and 1's and a positive integer K.
> QUESTION: Is there an m×K submatrix B of A that has the "consecutive ones" property, i.e., such that the columns of B can be permuted so that in each row all the 1's occur consecutively?
> Reference: [Booth, 1975]. Transformation from HAMILTONIAN PATH.
> Comment: The variant in which we ask instead that B have the "circular ones" property, i.e., that the columns of B can be permuted so that in each row either all the 1's or all the 0's occur consecutively, is also NP-complete. Both problems can be solved in polynomial time if K = n (in which case we are asking if A has the desired property), e.g., see [Fulkerson and Gross, 1965], [Tucker, 1971], and [Booth and Lueker, 1976].

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

- **[Booth, 1975]**: [`Booth1975`] K. S. Booth (1975). "{PQ} Tree Algorithms". University of California, Berkeley.
- **[Fulkerson and Gross, 1965]**: [`Fulkerson1965`] D. R. Fulkerson and D. A. Gross (1965). "Incidence matrices and interval graphs". *Pacific Journal of Mathematics* 15, pp. 835–855.
- **[Tucker, 1971]**: [`Tucker1971`] A. Tucker (1971). "A structure theorem for the consecutive ones property". In: *Proceedings of the 2nd Annual ACM Symposium on Theory of Computing*.
- **[Booth and Lueker, 1976]**: [`Booth1976`] K. S. Booth and G. S. Lueker (1976). "Testing for the consecutive ones property, interval graphs, and graph planarity using {PQ}-tree algorithms". *Journal of Computer and System Sciences* 13, pp. 335–379.