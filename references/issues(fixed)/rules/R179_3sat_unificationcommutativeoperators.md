---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to UNIFICATION WITH COMMUTATIVE OPERATORS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** UNIFICATION WITH COMMUTATIVE OPERATORS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.252

## GJ Source Entry

> [AN16] UNIFICATION WITH COMMUTATIVE OPERATORS
> INSTANCE: Set V of variables, set C of constants, ordered pairs (e_i,f_i), 1 ≤ i ≤ n, of "expressions," where an expression is either a variable from V, a constant from C, or (e + f) where e and f are expressions.
> QUESTION: Is there an assignment to each v E V of a variable-free expression I(v) such that, if I(e) denotes the expression obtained by replacing each occurrence of each variable v in e by I(v), then I(e_i) ≡ I(f_i) for 1 ≤ i ≤ n, where e ≡ f if e = f or if e = (a+b), f = (c+d), and either a ≡ c and b ≡ d or a ≡ d and b ≡ c?
> Reference: [Sethi, 1977b]. Transformation from 3SAT.
> Comment: Remains NP-complete even if no e_j or f_j contains more than 7 occurrences of constants and variables. The variant in which the operator is non-commutative (and hence e ≡ f only if e = f) is solvable in polynomial time [Paterson and Wegman, 1976].

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

- **[Sethi, 1977b]**: [`Sethi1977b`] R. Sethi (1977). "".
- **[Paterson and Wegman, 1976]**: [`Paterson and Wegman1976`] M. S. Paterson and M. N. Wegman (1976). "Linear unification". *Journal of Computer and System Sciences* 16, pp. 158–167.