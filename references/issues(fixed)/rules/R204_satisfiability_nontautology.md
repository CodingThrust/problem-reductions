---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SATISFIABILITY to NON-TAUTOLOGY"
labels: rule
assignees: ''
---

**Source:** SATISFIABILITY
**Target:** NON-TAUTOLOGY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.261

## GJ Source Entry

> [LO8] NON-TAUTOLOGY
> INSTANCE: Boolean expression E over a set U of variables, using the connectives "¬" (not), "V" (or), "∧" (and), and "→" (implies).
> QUESTION: Is E not a tautology, i.e., is there a truth assignment for U that makes E false?
> Reference: [Cook, 1971a]. Transformation from SATISFIABILITY.
> Comment: Remains NP-complete even if E is in "disjunctive normal form" with at most 3 literals per disjunct.

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