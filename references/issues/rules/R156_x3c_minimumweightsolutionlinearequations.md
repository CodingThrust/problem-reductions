---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to Minimum Weight Solution to Linear Equations"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
skip_reason: "X3C (Exact Cover by 3-Sets) is a specialization of Set Covering, not yet implemented as a separate model"
specialization_of: 'MinimumSetCovering'
milestone: 'Garey & Johnson'
---

**Source:** X3C
**Target:** Minimum Weight Solution to Linear Equations
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.246

**Status:** SKIP_SPECIALIZATION — The source problem X3C (Exact Cover by 3-Sets) is a specialization of Set Covering that is not yet implemented as a separate model in the codebase. This rule will be revisited once X3C is available as a standalone problem type.

## GJ Source Entry

> [MP5] MINIMUM WEIGHT SOLUTION TO LINEAR EQUATIONS
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of integers and b is an integer, and a positive integer K <= m.
> QUESTION: Is there an m-tuple y-bar with rational entries such that y-bar has at most K non-zero entries and such that x-bar·y-bar = b for all (x-bar, b) E X?
> Reference: [Garey and Johnson, ——]. Transformation from X3C.
> Comment: NP-complete in the strong sense. Solvable in polynomial time if K = m.

## References

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
