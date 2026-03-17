---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Maximum 2-Satisfiability to Open Hemisphere"
labels: rule
assignees: ''
---

**Source:** Maximum 2-Satisfiability
**Target:** Open Hemisphere
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.246

## GJ Source Entry

> [MP6] OPEN HEMISPHERE
> INSTANCE: Finite set X of m-tuples of integers, and a positive integer K <= |X|.
> QUESTION: Is there an m-tuple y-bar of rational numbers such that x-bar·y-bar > 0 for at least K m-tuples x-bar E X?
> Reference: [Johnson and Preparata, 1978]. Transformation from MAXIMUM 2-SATISFIABILITY.
> Comment: NP-complete in the strong sense, but solvable in polynomial time for any fixed m, even in a "weighted" version of the problem. The same results hold for the related CLOSED HEMISPHERE problem in which we ask that y-bar satisfy x-bar·y-bar >= 0 for at least K m-tuples x-bar E X [Johnson and Preparata, 1978]. If K = 0 or K = |X|, both problems are polynomially equivalent to linear programming [Reiss and Dobkin, 1976].

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

- **[Johnson and Preparata, 1978]**: [`Johnson1978c`] David S. Johnson and Franco P. Preparata (1978). "The densest hemisphere problem". *Theoretical Computer Science* 6, pp. 93–107.
- **[Reiss and Dobkin, 1976]**: [`Reiss1976`] S. P. Reiss and D. P. Dobkin (1976). "The complexity of linear programming". Dept. of Computer Science, Yale University.