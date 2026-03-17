---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Rooted Tree Arrangement to Rooted Tree Storage Assignment"
labels: rule
assignees: ''
---

**Source:** Rooted Tree Arrangement
**Target:** Rooted Tree Storage Assignment
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.1, p.227

## GJ Source Entry

> [SR5] ROOTED TREE STORAGE ASSIGNMENT
> INSTANCE: Finite set X, collection C = {X_1, X_2, ..., X_n} of subsets of X, positive integer K.
> QUESTION: Is there a collection C' = {X_1', X_2', ..., X_n'} of subsets of X such that X_i ⊆ X_i' for 1 <= i <= n, such that sum_{i=1}^{n} |X_i' - X_i| <= K, and such that there is a directed rooted tree T = (X,A) in which the elements of each X_i', 1 <= i <= n, form a directed path?
> Reference: [Gavril, 1977a]. Transformation from ROOTED TREE ARRANGEMENT.

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

- **[Gavril, 1977a]**: [`Gavril1977a`] F. Gavril (1977). "Some {NP}-complete problems on graphs". In: *Proceedings of the 11th Conference on Information Sciences and Systems*, pp. 91–95. Johns Hopkins University.