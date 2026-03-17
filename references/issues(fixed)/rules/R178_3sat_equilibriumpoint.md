---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to EQUILIBRIUM POINT"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** EQUILIBRIUM POINT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.252

## GJ Source Entry

> [AN15] EQUILIBRIUM POINT
> INSTANCE: Set x = {x_1,x_2,...,x_n} of variables, collection {F_i: 1 ≤ i ≤ n} of product polynomials over X and the integers, and a finite "range-set" M_i ⊆ Z for 1 ≤ i ≤ n.
> QUESTION: Does there exist a sequence y_1,y_2,...,y_n of integers, with y_i E M_i, such that for 1 ≤ i ≤ n and all y E M_i,
>
> F_i(y_1,y_2,...,y_{i-1},y_i,y_{i+1},...,y_n) ≥ F_i(y_1,y_2,...,y_{i-1},y,y_{i+1},...,y_n)?
>
> Reference: [Sahni, 1974]. Transformation from 3SAT.
> Comment: Remains NP-complete even if M_i = {0,1} for 1 ≤ i ≤ n.

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

- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.