---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to MINIMUM AXIOM SET"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** MINIMUM AXIOM SET
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.263

## GJ Source Entry

> [LO17] MINIMUM AXIOM SET
> INSTANCE: Finite set S of "sentences," subset T ⊆ S of "true sentences," an "implication relation" R consisting of pairs (A,s) where A ⊆ S and s E S, and a positive integer K ≤ |S|.
> QUESTION: Is there a subset S_0 ⊆ T with |S_0| ≤ K and a positive integer n such that, if we define S_i, 1 ≤ i ≤ n, to consist of exactly those s E S for which either s E S_{i-1} or there exists a U ⊆ S_{i-1} such that (U,s) E R, then S_n = T?
> Reference: [Pudlák, 1975]. Transformation from X3C.
> Comment: Remains NP-complete even if T = S.

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

- **[Pudlák, 1975]**: [`Pudlak1975`] P. Pudl{\'a}k (1975). "Polynomially complete problems in the logic of automated discovery". In: *Mathematical Foundations of Computer Science*. Springer.