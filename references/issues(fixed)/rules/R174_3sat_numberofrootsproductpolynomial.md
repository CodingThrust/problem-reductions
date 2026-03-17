---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## GJ Source Entry

> [AN11] NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL (*)
> INSTANCE: Sequences A_i = <(a_i[1],b_i[1]),...,(a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0, and a positive integer K.
> QUESTION: Does the polynomial Π_{i=1}^m (Σ_{j=1}^k a_i[j]·z^{b_i[j]}) have fewer than K distinct complex roots?
> Reference: [Plaisted, 1977a]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1, as does the variant in which the instance also includes an integer M and we are asked whether the product polynomial has fewer than K complex roots of multiplicity M [Plaisted, 1976].

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
- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264–267. IEEE Computer Society.