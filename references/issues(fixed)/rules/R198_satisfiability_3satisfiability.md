---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SATISFIABILITY to 3-SATISFIABILITY"
labels: rule
assignees: ''
---

**Source:** SATISFIABILITY
**Target:** 3-SATISFIABILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.1, p.259

## GJ Source Entry

> [LO2] 3-SATISFIABILITY (3SAT)
> INSTANCE: Set U of variables, collection C of clauses over U such that each clause c E C has |c| = 3.
> QUESTION: Is there a satisfying truth assignment for C?
> Reference: [Cook, 1971a]. Transformation from SATISFIABILITY.
> Comment: Remains NP-complete even if each clause contains either only negated variables or only un-negated variables (MONOTONE 3SAT) [Gold, 1974], or if for each u E U there are at most 5 clauses in C that contain either u or ū.

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
- **[Gold, 1974]**: [`Gold1974`] E. M. Gold (1974). "Complexity of automaton identification from given data".