---
name: Rule
about: Propose a new reduction rule
title: "[Rule] MINIMUM COVER to MINIMUM DISJUNCTIVE NORMAL FORM"
labels: rule
assignees: ''
---

**Source:** MINIMUM COVER
**Target:** MINIMUM DISJUNCTIVE NORMAL FORM
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.261

## GJ Source Entry

> [LO9] MINIMUM DISJUNCTIVE NORMAL FORM
> INSTANCE: Set U = {u_1,u_2,...,u_n} of variables, set A ⊆ {T,F}^n of "truth assignments," and a positive integer K.
> QUESTION: Is there a disjunctive normal form expression E over U, having no more than K disjuncts, such that E is true for precisely those truth assignments in A, and no others?
> Reference: [Gimpel, 1965]. Transformation from MINIMUM COVER.
> Comment: Variant in which the instance contains a complete truth table, i.e., disjoint sets A and B ⊆ {T,F}^n such that A ∪ B = {T,F}^n, and E must be true for all truth assignments in A and false for all those in B, is also NP-complete, despite the possibly much larger instance size [Masek, 1978].

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

- **[Gimpel, 1965]**: [`Gimpel1965`] J. F. Gimpel (1965). "A method of producing a {Boolean} function having an arbitrarily prescribed prime implicant table". *IEEE Transactions on Computers* 14, pp. 485–488.
- **[Masek, 1978]**: [`Masek1978`] William J. Masek (1978). "Some {NP}-complete set covering problems".