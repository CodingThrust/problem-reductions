---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to CONJUNCTIVE SATISFIABILITY WITH FUNCTIONS AND INEQUALITIES"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** CONJUNCTIVE SATISFIABILITY WITH FUNCTIONS AND INEQUALITIES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.263

## GJ Source Entry

> [LO16] CONJUNCTIVE SATISFIABILITY WITH FUNCTIONS AND INEQUALITIES
> INSTANCE: Set U of variables, set F of univariate function symbols, and a collection C of "clauses" of the form U*V where * is either "≤," ">," "=," or "≠," and U and V are either "0," "1," "u," "f(0)," "f(1)," or "f(u)," for some f E F and u E U.
> QUESTION: Is there an assignment of integer values to all the variables u E U and to all f(u), for u E U and f E F, such that all the clauses in C are satisfied under the usual interpretations of ≤, >, =, and ≠?
> Reference: [Pratt, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete even if = and ≠ are not used. Solvable in polynomial time if ≤ and > are not used [Nelson and Oppen, 1977], or if = and ≠ are not used and no function symbols are allowed [Litvintchouk and Pratt, 1977]. Variant in which W and V are either of the form "u" or "u+c" for some u E U and c E Z is NP-complete if all four relations are allowed, but solvable in polynomial time if only ≤ and > or only = and ≠ are allowed [Chan, 1977].

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

- **[Pratt, 1977]**: [`Pratt1977`] V. Pratt (1977). "Two easy theories whose combination is hard".
- **[Nelson and Oppen, 1977]**: [`Nelson1977`] G. Nelson and D. C. Oppen (1977). "Fast decision algorithms based on union and find". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 114–119. IEEE Computer Society.
- **[Litvintchouk and Pratt, 1977]**: [`Litvintchouk1977`] Steven D. Litvintchouk and V. R. Pratt (1977). "A proof checker for dynamic logic". In: *Proceedings of the 5th International Joint Conference on Artificial Intelligence*, pp. 552–558. International Joint Conferences on Artificial Intelligence, Dept. of Computer Science, Carnegie-Mellon University.
- **[Chan, 1977]**: [`Chan1977`] T. Chan (1977). "An algorithm for checking {PL/CV} arithmetic inferences". Cornell University.