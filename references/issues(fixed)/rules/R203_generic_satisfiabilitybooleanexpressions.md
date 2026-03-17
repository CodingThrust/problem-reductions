---
name: Rule
about: Propose a new reduction rule
title: "[Rule] (generic transformation) to SATISFIABILITY OF BOOLEAN EXPRESSIONS"
labels: rule
assignees: ''
---

**Source:** (generic transformation)
**Target:** SATISFIABILITY OF BOOLEAN EXPRESSIONS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.1, p.260-261

## GJ Source Entry

> [LO7] SATISFIABILITY OF BOOLEAN EXPRESSIONS
> INSTANCE: Variable set U, a subset B of the set of 16 possible binary Boolean connectives, and a well-formed Boolean expression E over U and B.
> QUESTION: Is there a truth assignment for U that satisfies E?
> Reference: [Cook, 1971a]. Generic transformation.
> Comment: Remains NP-complete if B is restricted to {∧,∨,→,¬}, or any other truth-functionally complete set of connectives. Also NP-complete for any truth-functionally incomplete set of connectives containing {↑}, {↓}, {≢,∨}, or {≢,∧} as a subset [Lewis, 1978]. Problem is solvable in polynomial time for any truth-functionally incomplete set of connectives not containing one of these four sets as a subset.

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

- **[Cook, 1971a]**: [`Cook1971a`] S. A. Cook (1971). "The complexity of theorem-proving procedures". In: *Proceedings of the 3rd Annual ACM Symposium on Theory of Computing*, pp. 151–158. Association for Computing Machinery.
- **[Lewis, 1978]**: [`Lewis1978a`] Harry R. Lewis (1978). "Satisfiability problems for propositional calculi".