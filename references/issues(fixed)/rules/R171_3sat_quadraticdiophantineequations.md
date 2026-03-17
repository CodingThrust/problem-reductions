---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to QUADRATIC DIOPHANTINE EQUATIONS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** QUADRATIC DIOPHANTINE EQUATIONS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.2, p.250

## GJ Source Entry

> [AN8] QUADRATIC DIOPHANTINE EQUATIONS
> INSTANCE: Positive integers a, b, and c.
> QUESTION: Are there positive integers x and y such that ax^2 + by = c?
> Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
> Comment: Diophantine equations of the forms ax^k = c and Σ_{i=1}^k a_i·x_i = c are solvable in polynomial time for arbitrary values of k. The general Diophantine problem, "Given a polynomial with integer coefficients in k variables, does it have an integer solution?" is undecidable, even for k = 13 [Matijasevic and Robinson, 1975]. However, the given problem can be generalized considerably (to simultaneous equations in many variables) while remaining in NP, so long as only one variable enters into the equations in a non-linear way (see [Gurari and Ibarra, 1978]).

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
- **[Matijasevic and Robinson, 1975]**: [`Matijasevic1975`] Yuri V. Matijasevic and Julia Robinson (1975). "Reduction of an arbitrary {Diophantine} equation to one in 13 unknowns". *Acta Arithmetica* 27, pp. 521–553.
- **[Gurari and Ibarra, 1978]**: [`Gurari1978`] E. M. Gurari and O. H. Ibarra (1978). "An {NP}-complete number theoretic problem". In: *Proceedings of the 10th Annual ACM Symposium on Theory of Computing*, pp. 205–215. Association for Computing Machinery.