---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to K-Relevancy"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** K-Relevancy
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.246

## GJ Source Entry

> [MP7] K-RELEVANCY
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of integers and b is an integer, and a positive integer K <= |X|.
> QUESTION: Is there a subset X' ⊆ X with |X'| <= K such that, for all m-tuples y-bar of rational numbers, if x-bar·y-bar <= b for all (x-bar, b) E X', then x-bar·y-bar <= b for all (x-bar, b) E X?
> Reference: [Reiss and Dobkin, 1976]. Transformation from X3C.
> Comment: NP-complete in the strong sense. Equivalent to linear programming if K = |X| - 1 [Reiss and Dobkin, 1976]. Other NP-complete problems of this form, where a standard linear programming problem is modified by asking that the desired property hold for some subset of K constraints, can be found in the reference.

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

- **[Reiss and Dobkin, 1976]**: [`Reiss1976`] S. P. Reiss and D. P. Dobkin (1976). "The complexity of linear programming". Dept. of Computer Science, Yale University.