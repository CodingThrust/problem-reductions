---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to SECOND ORDER INSTANTIATION"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** SECOND ORDER INSTANTIATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.264

## GJ Source Entry

> [LO19] SECOND ORDER INSTANTIATION
> INSTANCE: Two "second order logic expressions" E_1 and E_2, the second of which contains no variables (in a second order expression, functions can be variables; see references for details).
> QUESTION: Is there a substitution for the variables of E_1 that yields an expression identical to E_2?
> Reference: [Baxter, 1976]. Transformation from 3SAT. Proof of membership in NP is nontrivial.
> Comment: The more general SECOND ORDER UNIFICATION problem, where both E_1 and E_2 can contain variables and we ask if there is a substitution for the variables in E_1 and E_2 that results in identical expressions, is not known to be decidable. THIRD ORDER UNIFICATION is undecidable [Huet, 1973], whereas FIRST ORDER UNIFICATION can be solved in polynomial time [Baxter, 1975], [Paterson and Wegman, 1978].

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

- **[Baxter, 1976]**: [`Baxter1976`] L. D. Baxter (1976). "The Complexity of Unification". University of Waterloo.
- **[Huet, 1973]**: [`Huet1973`] G{\'e}rard P. Huet (1973). "The undecidability of unification in third order logic". *Information and Control* 22, pp. 257–267.
- **[Baxter, 1975]**: [`Baxter1975`] L. D. Baxter (1975). "The Complexity of Unification". Dept. of Computer Science, University of Waterloo.
- **[Paterson and Wegman, 1978]**: [`Paterson1978`] M. S. Paterson and M. N. Wegman (1978). "Linear unification". *Journal of Computer and System Sciences* 16, pp. 158–167.