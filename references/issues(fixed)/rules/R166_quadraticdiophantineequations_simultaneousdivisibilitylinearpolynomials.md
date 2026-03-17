---
name: Rule
about: Propose a new reduction rule
title: "[Rule] QUADRATIC DIOPHANTINE EQUATIONS to SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS"
labels: rule
assignees: ''
---

**Source:** QUADRATIC DIOPHANTINE EQUATIONS
**Target:** SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.1, p.249

## GJ Source Entry

> [AN3] SIMULTANEOUS DIVISIBILITY OF LINEAR POLYNOMIALS (*)
> INSTANCE: Vectors a_i = (a_i[0],...,a_i[m]) and b_i = (b_i[0],...,b_i[m]), 1 ≤ i ≤ n, with positive integer entries.
> QUESTION: Do there exist positive integers x_1,x_2,...,x_m such that, for 1 ≤ i ≤ n, a_i[0] + Σ_{j=1}^m (a_i[j]·x_j) divides b_i[0] + Σ_{j=1}^m (b_i[j]·x_j)?
> Reference: [Lipshitz, 1977], [Lipshitz, 1978]. Transformation from QUADRATIC DIOPHANTINE EQUATIONS.
> Comment: Not known to be in NP, but belongs to NP for any fixed n. NP-complete for any fixed n ≥ 5. General problem is undecidable if the vector entries and the x_j are allowed to range over the ring of "integers" in a real quadratic extension of the rationals. See reference for related decidability and undecidability results.

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

- **[Lipshitz, 1977]**: [`Lipshitz1977`] Leonard Lipshitz (1977). "A remark on the {Diophantine} problem for addition and divisibility".
- **[Lipshitz, 1978]**: [`Lipshitz1978`] Leonard Lipshitz (1978). "The {Diophantine} problem for addition and divisibility". *Transactions of the American Mathematical Society* 235, pp. 271–283.