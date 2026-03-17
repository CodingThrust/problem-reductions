---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.250

## GJ Source Entry

> [AN6] NON-DIVISIBILITY OF A PRODUCT POLYNOMIAL
> INSTANCE: Sequences A_i = <(a_i[1],b_i[1]),...,(a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0, and an integer N.
> QUESTION: Is Π_{i=1}^m (Σ_{j=1}^k a_i[j]·z^{b_i[j]}) not divisible by z^N - 1?
> Reference: [Plaisted, 1977a], [Plaisted, 1977b]. Transformation from 3SAT. Proof of membership in NP is non-trivial and appears in the second reference.
> Comment: The related problem in which we are given two sequences <a_1,a_2,...,a_m> and <b_1,b_2,...,b_n> of positive integers and are asked whether Π_{i=1}^m (z^{a_i} - 1) does not divide Π_{j=1}^n (z^{b_j} - 1) is also NP-complete [Plaisted, 1976].

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

- **[Plaisted, 1977a]**: [`Plaisted1977a`] D. Plaisted (1977). "Sparse complex polynomials and polynomial reducibility". *Journal of Computer and System Sciences* 14, pp. 210–221.
- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241–253. IEEE Computer Society.
- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264–267. IEEE Computer Society.