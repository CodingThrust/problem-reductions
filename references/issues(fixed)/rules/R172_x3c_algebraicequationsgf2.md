---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to ALGEBRAIC EQUATIONS OVER GF[2]"
labels: rule
assignees: ''
---

**Source:** X3C
**Target:** ALGEBRAIC EQUATIONS OVER GF[2]
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## GJ Source Entry

> [AN9] ALGEBRAIC EQUATIONS OVER GF[2]
> INSTANCE: Polynomials P_i(x_1,x_2,...,x_n), 1 ≤ i ≤ m, over GF(2), i.e., each polynomial is a sum of terms, where each term is either the integer 1 or a product of distinct x_i.
> QUESTION: Do there exist u_1,u_2,...,u_n E {0,1} such that, for 1 ≤ i ≤ m, P_i(u_1,u_2,...,u_n) = 0, where arithmetic operations are as defined in GF(2), with 1+1 = 0 and 1·1 = 1?
> Reference: [Fraenkel and Yesha, 1977]. Transformation from X3C.
> Comment: Remains NP-complete even if none of the polynomials has a term involving more than two variables [Valiant, 1977c]. Easily solved in polynomial time if no term involves more than one variable or if there is just one polynomial. Variant in which the u_j are allowed to range over the algebraic closure of GF(2) is NP-hard, even if no term involves more than two variables [Fraenkel and Yesha, 1977].

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

- **[Fraenkel and Yesha, 1977]**: [`Fraenkel1977`] A. S. Fraenkel and Y. Yesha (1977). "Complexity of problems in games, graphs, and algebraic equations".
- **[Valiant, 1977c]**: [`Valiant1977c`] Leslie G. Valiant (1977). "private communication".