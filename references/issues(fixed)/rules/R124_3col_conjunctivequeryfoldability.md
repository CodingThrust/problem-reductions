---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Graph 3-Colorability to Conjunctive Query Foldability"
labels: rule
assignees: ''
---

**Source:** Graph 3-Colorability
**Target:** Conjunctive Query Foldability
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.233

## GJ Source Entry

> [SR30] CONJUNCTIVE QUERY FOLDABILITY
> INSTANCE: Finite domain set D, a collection R = {R_1, R_2, ..., R_m} of relations, where each R_i consists of a set of d_i-tuples with entries from D, a set X of distinguished variables, a set Y of undistinguished variables, and two "queries" Q_1 and Q_2 over X, Y, D, and R, where a query Q has the form
>
> (x_1, x_2, ..., x_k)(∃y_1, y_2, ..., y_l)(A_1 ∧ A_2 ∧ ... ∧ A_r)
>
> for some k, l, and r, with X' = {x_1, x_2, ..., x_k} ⊆ X, Y' = {y_1, y_2, ..., y_l} ⊆ Y, and each A_i of the form R_j(u_1, u_2, ..., u_{d_j}) with each u E D ∪ X' ∪ Y' (see reference for interpretation of such expressions in terms of data bases).
> QUESTION: Is there a function σ: Y → X ∪ Y ∪ D such that, if for each y E Y the symbol σ(y) is substituted for every occurrence of y in Q_1, then the result is query Q_2?
> Reference: [Chandra and Merlin, 1977]. Transformation from GRAPH 3-COLORABILITY.
> Comment: The isomorphism problem for conjunctive queries (with two queries being isomorphic if they are the same up to one-to-one renaming of the variables, reordering of conjuncts, and reordering within quantifications) is polynomially equivalent to graph isomorphism.

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

- **[Chandra and Merlin, 1977]**: [`Chandra1977`] A. K. Chandra and P. M. Merlin (1977). "Optimal implementation of conjunctive queries in relational data bases". In: *Proceedings of the 9th Annual ACM Symposium on Theory of Computing*, pp. 77–90. Association for Computing Machinery.