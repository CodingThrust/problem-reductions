---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to TRUTH-FUNCTIONALLY COMPLETE CONNECTIVES"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** TRUTH-FUNCTIONALLY COMPLETE CONNECTIVES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.261

## GJ Source Entry

> [LO10] TRUTH-FUNCTIONALLY COMPLETE CONNECTIVES
> INSTANCE: Set U of variables, collection C of well-formed Boolean expressions over U.
> QUESTION: Is C truth-functionally complete, i.e., is there a truth-functionally complete set of logical connectives (unary and binary operators) D = {θ_1,θ_2,...,θ_k} such that for each θ_i E D there is an expression E E C and a substitution s: U → {a,b} for which s(E) ≡ aθ_i b or s(E) ≡ θ_i a (depending on whether θ_i is binary or unary)?
> Reference: [Statman, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete even if |C| = 2.

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

- **[Statman, 1976]**: [`Statman1976`] Richard Statman (1976). "private communication".