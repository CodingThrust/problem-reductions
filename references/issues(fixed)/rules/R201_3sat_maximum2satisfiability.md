---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MAXIMUM 2-SATISFIABILITY"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MAXIMUM 2-SATISFIABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.1, p.259-260

## GJ Source Entry

> [LO5] MAXIMUM 2-SATISFIABILITY
> INSTANCE: Set U of variables, collection C of clauses over U such that each clause c E C has |c| = 2, positive integer K ≤ |C|.
> QUESTION: Is there a truth assignment for U that simultaneously satisfies at least K of the clauses in C?
> Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from 3SAT.
> Comment: Solvable in polynomial time if K = |C| (e.g.,see [Even, Itai, and Shamir, 1976]).

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

- **[Garey, Johnson, and Stockmeyer, 1976]**: [`Garey1976g`] M. R. Garey and D. S. Johnson and L. Stockmeyer (1976). "Some simplified {NP}-complete graph problems". *Theoretical Computer Science* 1, pp. 237–267.
- **[Even, Itai, and Shamir, 1976]**: [`Even1976a`] S. Even and A. Itai and A. Shamir (1976). "On the complexity of timetable and multicommodity flow problems". *SIAM Journal on Computing* 5, pp. 691–703.