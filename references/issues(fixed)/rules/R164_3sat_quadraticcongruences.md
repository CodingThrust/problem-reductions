---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to QUADRATIC CONGRUENCES"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** QUADRATIC CONGRUENCES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249

## GJ Source Entry

> [AN1] QUADRATIC CONGRUENCES
> INSTANCE: Positive integers a, b, and c.
> QUESTION: Is there a positive integer x < c such that x^2 ≡ a (mod b)?
> Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
> Comment: Remains NP-complete even if the instance includes a prime factorization of b and solutions to the congruence modulo all prime powers occurring in the factorization. Solvable in polynomial time if c = ∞ (i.e., there is no upper bound on x) and the prime factorization of b is given. Assuming the Extended Riemann Hypothesis, the problem is solvable in polynomial time when b is prime. The general problem is trivially solvable in pseudo-polynomial time.

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

- **[Manders and Adleman, 1978]**: [`Manders1978`] Kenneth Manders and Leonard Adleman (1978). "{NP}-complete decision problems for binary quadratics". *Journal of Computer and System Sciences* 16, pp. 168–184.