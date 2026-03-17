---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Cost-Parametric Linear Programming"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Cost-Parametric Linear Programming
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.245

## GJ Source Entry

> [MP3] COST-PARAMETRIC LINEAR PROGRAMMING
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of integers and b is an integer, a set J ⊆ {1, 2, ..., m}, and a positive rational number q.
> QUESTION: Is there an m-tuple c-bar with rational entries such that (c-bar·c-bar)^{1/2} <= q and such that, if Y is the set of all m-tuples y-bar with non-negative rational entries satisfying x-bar·y-bar >= b for all (x-bar, b) E X, then the minimum of sum_{j E J} c_j y_j over all y-bar E Y exceeds
> 1/2 max {|c_j|: j E J} + sum_{j E J} min {0, c_j} ?
> Reference: [Jeroslow, 1976]. Transformation from 3SAT.
> Comment: Remains NP-complete for any fixed q > 0. The problem arises from first order error analysis for linear programming.

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

- **[Jeroslow, 1976]**: [`Jeroslow1976`] Robert G. Jeroslow (1976). "Bracketing discrete problems by two problems of linear optimization". In: *Proceedings of the First Symposium on Operations Research (at Heidelberg)*, pp. 205–216. Verlag Anton Hain.