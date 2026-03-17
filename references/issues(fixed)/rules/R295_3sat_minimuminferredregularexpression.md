---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MINIMUM INFERRED REGULAR EXPRESSION"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MINIMUM INFERRED REGULAR EXPRESSION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL10

## Reduction Algorithm

> INSTANCE: Finite alphabet Σ, two finite subsets S, T ⊆ Σ*, positive integer K.
> QUESTION: Is there a regular expression E over Σ that has K or fewer occurrences of symbols from Σ and such that, if L ⊆ Σ* is the language represented by E, then S ⊆ L and T ⊆ Σ*−L?
> Reference: [Angluin, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete even if E is required to contain no "∪" operations or to be "star-free" (contain no "*" operations) [Angluin, 1976].

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Angluin, 1977]**: [`Angluin1977a`] D. Angluin (1977). "On the complexity of minimum inference of regular sets".
- **[Angluin, 1976]**: [`Angluin1976`] D. Angluin (1976). "An Application of the Theory of Computational Complexity to the Study of Inductive Inference". University of California, Berkeley.