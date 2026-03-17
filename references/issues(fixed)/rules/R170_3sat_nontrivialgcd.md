---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to NON-TRIVIAL GREATEST COMMON DIVISOR"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** NON-TRIVIAL GREATEST COMMON DIVISOR
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.250

## GJ Source Entry

> [AN7] NON-TRIVIAL GREATEST COMMON DIVISOR (*)
> INSTANCE: Sequences A_i = <(a_i[1],b_i[1]),...,(a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0.
> QUESTION: Does the greatest common divisor of the polynomials Σ_{j=1}^k a_i[j]·z^{b_i[j]}, 1 ≤ i ≤ m, have degree greater than zero?
> Reference: [Plaisted, 1977a]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1 [Plaisted, 1976] or if m = 2 [Plaisted, 1977b]. The analogous problem in which the instance also includes a positive integer K, and we are asked if the least common multiple of the given polynomials has degree less than K, is NP-hard under the same restrictions. Both problems can be solved in pseudo-polynomial time using standard algorithms.

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
- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241–253. IEEE Computer Society.