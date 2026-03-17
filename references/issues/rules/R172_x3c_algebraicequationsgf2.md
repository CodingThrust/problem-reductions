---
name: Rule
about: Propose a new reduction rule
title: "[Rule] X3C to ALGEBRAIC EQUATIONS OVER GF[2]"
labels: rule
assignees: ''
status: SKIP_SPECIALIZATION
skip_reason: "Source X3C (Exact Cover by 3-Sets) is a specialization of Set Covering — wait for general version"
milestone: 'Garey & Johnson'
---

**Source:** X3C (Exact Cover by 3-Sets)
**Target:** ALGEBRAIC EQUATIONS OVER GF[2]
**Motivation:** Skipped — source problem X3C is a known specialization of Set Covering (each set has exactly 3 elements, exact cover required). This rule should wait until the general Set Covering → Algebraic Equations reduction path is established.
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.251

## Specialization Note

X3C (Exact Cover by 3-Sets) is a restriction of Set Covering where:
- Each set has exactly 3 elements
- An exact cover is required (every element covered exactly once)
- Listed as GJ problem [SP2], reference P129

This rule file is a local-only stub and should **NOT** be submitted as a GitHub issue.

## GJ Source Entry

> [AN9] ALGEBRAIC EQUATIONS OVER GF[2]
> INSTANCE: Polynomials P_i(x_1,x_2,...,x_n), 1 ≤ i ≤ m, over GF(2), i.e., each polynomial is a sum of terms, where each term is either the integer 1 or a product of distinct x_i.
> QUESTION: Do there exist u_1,u_2,...,u_n E {0,1} such that, for 1 ≤ i ≤ m, P_i(u_1,u_2,...,u_n) = 0, where arithmetic operations are as defined in GF(2), with 1+1 = 0 and 1·1 = 1?
> Reference: [Fraenkel and Yesha, 1977]. Transformation from X3C.

## References

- **[Fraenkel and Yesha, 1977]**: A. S. Fraenkel and Y. Yesha (1977). "Complexity of problems in games, graphs, and algebraic equations".
- **[Valiant, 1977c]**: Leslie G. Valiant (1977). "private communication".
