---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to FIRST ORDER SUBSUMPTION"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** FIRST ORDER SUBSUMPTION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.264

## GJ Source Entry

> [LO18] FIRST ORDER SUBSUMPTION
> INSTANCE: Finite set U of "variable symbols," finite set C of "function symbols," collection E = {E_1,E_2,...,E_m} of expressions over U∪C, collection F = {F_1,F_2,...,F_n} of expressions over C.
> QUESTION: Is there a substitution mapping s that assigns to each u E U an expression s(u) over C such that, if s(E_i) denotes the result of substituting for each occurrence in E_i of each u E U the corresponding expression s(u), then {s(E_1),s(E_2),...,s(E_m)} is a subset of {F_1,F_2,...,F_n}?
> Reference: [Baxter, 1976], [Baxter, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete for any fixed n ≥ 3, but is solvable in polynomial time for any fixed m.

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

- **[Baxter, 1976]**: [`Baxter1976`] L. D. Baxter (1976). "The Complexity of Unification". University of Waterloo.
- **[Baxter, 1977]**: [`Baxter1977`] L. D. Baxter (1977). "The {NP}-completeness of subsumption".