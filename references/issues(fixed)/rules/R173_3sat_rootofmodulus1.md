---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to ROOT OF MODULUS 1"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** ROOT OF MODULUS 1
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## GJ Source Entry

> [AN10] ROOT OF MODULUS 1 (*)
> INSTANCE: Ordered pairs (a[i], b[i]), 1 ≤ i ≤ n, of integers, with each b[i] ≥ 0.
> QUESTION: Does the polynomial Σ_{i=1}^n a[i]·z^{b[i]} have a root on the complex unit circle, i.e., is there a complex number q with |q| = 1 such that Σ_{i=1}^n a[i]·q^{b[i]} = 0?
> Reference: [Plaisted, 1977b]. Transformation from 3SAT.
> Comment: Not known to be in NP or co-NP.

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

- **[Plaisted, 1977b]**: [`Plaisted1977b`] D. Plaisted (1977). "New {NP}-hard and {NP}-complete polynomial and integer divisibility problems". In: *Proceedings of the 18th Annual Symposium on Foundations of Computer Science*, pp. 241–253. IEEE Computer Society.