---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QBF to SEQUENTIAL TRUTH ASSIGNMENT"
labels: rule
assignees: ''
---

**Source:** QBF
**Target:** SEQUENTIAL TRUTH ASSIGNMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A8, p.254-255

## GJ Source Entry

> [GP4] SEQUENTIAL TRUTH ASSIGNMENT (*)
> INSTANCE: A sequence U = <u_1,u_2,...,u_n> of variables and a collection C of clauses over U (as in an instance of SATISFIABILITY).
> QUESTION: Does player 1 have a forced win in the following game played on U and C? Players alternate assigning truth values to the variables in U, with player 1 assigning a value to u_{2i-1} and player 2 assigning a value to u_{2i} on their i^{th} turns. Player 1 wins if and only if the resulting truth assignment satisfies all clauses in C.
> Reference: [Stockmeyer and Meyer, 1973]. Transformation from QBF.
> Comment: PSPACE-complete, even if each clause in C has only three literals. Solvable in polynomial time if no clause has more than two literals [Schaefer, 1978b].

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

- **[Stockmeyer and Meyer, 1973]**: [`Stockmeyer and Meyer1973`] Larry J. Stockmeyer and Albert R. Meyer (1973). "Word problems requiring exponential time". In: *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1–9. Association for Computing Machinery.
- **[Schaefer, 1978b]**: [`Schaefer1978b`] T. J. Schaefer (1978). "The complexity of satisfiability problems". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 216–226. Association for Computing Machinery.