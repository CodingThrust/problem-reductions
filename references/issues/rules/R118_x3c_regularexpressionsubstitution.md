---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to Regular Expression Substitution"
labels: rule
assignees: ''
canonical_source_name: 'Exact Cover by 3-Sets (X3C)'
canonical_target_name: 'Regular Expression Substitution'
source_in_codebase: false
target_in_codebase: false
specialization_of: 'MinimumSetCovering'
milestone: 'Garey & Johnson'
---

**Source:** X3C
**Target:** Regular Expression Substitution
**Motivation:** SKIP_SPECIALIZATION — Source problem X3C (Exact Cover by 3-Sets) is a specialization of Set Covering (each set has exactly 3 elements, exact cover required). Implement general Set Covering reductions first.
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.231-232

## GJ Source Entry

> [SR24] REGULAR EXPRESSION SUBSTITUTION
> INSTANCE: Two finite alphabets X = {x_1, x_2, ..., x_n} and Y = {y_1, y_2, ..., y_m}, a regular expression R over X ∪ Y, regular expressions R_1, R_2, ..., R_n over Y, and a string w E Y*.
> QUESTION: Is there a string z in the language determined by R and for each i, 1 <= i <= n, a string w_i in the language determined by R_i such that, if each string w_i is substituted for every occurrence of the symbol x_i in z, then the resulting string is identical to w?
> Reference: [Aho and Ullman, 1977]. Transformation from X3C.

## Specialization Note

This rule reduces from X3C (Exact Cover by 3-Sets), which is a specialization of Set Covering where:
- Each set in the collection has exactly 3 elements
- The goal is to find an exact cover (every element covered exactly once)

X3C (P129 in GJ) does not yet exist as a separate model in the codebase. The general Set Covering model (`MinimumSetCovering`) is implemented. Consider implementing X3C as a constrained variant of Set Covering before adding this reduction.

## Reduction Algorithm

(Deferred — waiting for X3C model)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (Deferred) | (Deferred) |

## Validation Method

(Deferred — waiting for X3C model)

## Example

(Deferred — waiting for X3C model)


## References

- **[Aho and Ullman, 1977]**: [`Aho1977e`] A. V. Aho and J. D. Ullman (1977). "".
