---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to EXPONENTIAL EXPRESSION DIVISIBILITY"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** EXPONENTIAL EXPRESSION DIVISIBILITY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249-250

## GJ Source Entry

> [AN5] EXPONENTIAL EXPRESSION DIVISIBILITY (*)
> INSTANCE: Sequences a_1,a_2,...,a_n and b_1,b_2,...,b_m of positive integers, and an integer q.
> QUESTION: Does Π_{i=1}^n (q^{a_i} - 1) divide Π_{j=1}^m (q^{b_j} - 1)?
> Reference: [Plaisted, 1976]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP, but solvable in pseudo-polynomial time using standard greatest common divisor algorithms. Remains NP-hard for any fixed value of q with |q| > 1, even if the a_i and b_j are restricted to being products of distinct primes.

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

- **[Plaisted, 1976]**: [`Plaisted1976`] D. Plaisted (1976). "Some polynomial and integer divisibility problems are {NP}-hard". In: *Proceedings of the 17th Annual Symposium on Foundations of Computer Science*, pp. 264–267. IEEE Computer Society.