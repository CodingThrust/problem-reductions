---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Integer Programming"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Integer Programming
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A6, p.245

## GJ Source Entry

> [MP1] INTEGER PROGRAMMING
> INSTANCE: Finite set X of pairs (x-bar, b), where x-bar is an m-tuple of integers and b is an integer, an m-tuple c-bar of integers, and an integer B.
> QUESTION: Is there an m-tuple y-bar of integers such that x-bar·y-bar <= b for all (x-bar, b) E X and such that c-bar·y-bar >= B (where the dot-product u-bar·v-bar of two m-tuples u-bar = (u_1, u_2, ..., u_m) and v-bar = (v_1, v_2, ..., v_m) is given by sum_{i=1}^{m} u_i v_i)?
> Reference: [Karp, 1972], [Borosh and Treybig, 1976]. Transformation from 3SAT. The second reference proves membership in NP.
> Comment: NP-complete in the strong sense. Variant in which all components of y-bar are required to belong to {0,1} (ZERO-ONE INTEGER PROGRAMMING) is also NP-complete, even if each b, all components of each x-bar, and all components of c-bar are required to belong to {0,1}. Also NP-complete are the questions of whether a y-bar with non-negative integer entries exists such that x-bar·y-bar = b for all (x-bar, b) E X, and the question of whether there exists any y-bar with integer entries such that x-bar·y-bar >= 0 for all (x-bar, b) E X [Sahni, 1974].

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

- **[Karp, 1972]**: [`Karp1972`] Richard M. Karp (1972). "Reducibility among combinatorial problems". In: *Complexity of Computer Computations*. Plenum Press.
- **[Borosh and Treybig, 1976]**: [`Borosh1976`] I. Borosh and L. B. Treybig (1976). "Bounds on positive integral solutions of linear {Diophantine} equations". *Proceedings of the American Mathematical Society* 55, pp. 299–304.
- **[Sahni, 1974]**: [`Sahni1974`] S. Sahni (1974). "Computationally related problems". *SIAM Journal on Computing* 3, pp. 262–279.