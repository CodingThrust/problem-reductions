---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to K-Relevancy"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
skip_reason: "X3C (Exact Cover by 3-Sets) is a specialization of Set Covering, not yet implemented as a separate model"
specialization_of: 'MinimumSetCovering'
milestone: 'Garey & Johnson'
---

**Source:** X3C
**Target:** K-Relevancy
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.246

**Status:** SKIP_SPECIALIZATION — The source problem X3C (Exact Cover by 3-Sets) is a specialization of Set Covering that is not yet implemented as a separate model in the codebase. This rule will be revisited once X3C is available as a standalone problem type.

## GJ Source Entry

> [MP7] K-RELEVANCY
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of integers and b is an integer, and a positive integer K <= |X|.
> QUESTION: Is there a subset X' ⊆ X with |X'| <= K such that, for all m-tuples y-bar of rational numbers, if x-bar·y-bar <= b for all (x-bar, b) E X', then x-bar·y-bar <= b for all (x-bar, b) E X?
> Reference: [Reiss and Dobkin, 1976]. Transformation from X3C.
> Comment: NP-complete in the strong sense. Equivalent to linear programming if K = |X| - 1 [Reiss and Dobkin, 1976]. Other NP-complete problems of this form, where a standard linear programming problem is modified by asking that the desired property hold for some subset of K constraints, can be found in the reference.

## References

- **[Reiss and Dobkin, 1976]**: [`Reiss1976`] S. P. Reiss and D. P. Dobkin (1976). "The complexity of linear programming". Dept. of Computer Science, Yale University.
