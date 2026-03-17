---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to STRONG INEQUIVALENCE OF IANOV SCHEMES"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** STRONG INEQUIVALENCE OF IANOV SCHEMES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO16

## GJ Source Entry

> [PO16]  STRONG INEQUIVALENCE OF IANOV SCHEMES
> INSTANCE:  Finite sets F and P of function and predicate symbols, single variable x, and two Ianov schemes over F,P, and x, each a sequence I1,I2,...,Im of instructions of the form "x←f(x)," "if p(x) then goto Ij else goto Ik," and "halt," where f ∈ F and p ∈ P.
> QUESTION:  Are the two given Ianov schemes not strongly equivalent, i.e., is there a domain set D, an interpretation of each f ∈ F as a function f: D→D, an interpretation of each p ∈ P as a function p: D→{T,F}, and an initial value x0 ∈ D for x, such that either both schemes halt with different final values for x or one halts and the other doesn't?
> Reference:  [Constable, Hunt, and Sahni, 1974], [Rutledge, 1964]. Transformation from 3SAT.  Membership in NP follows from the second reference.
> Comment:  Remains NP-complete even if neither program contains any loops and P2 is the trivial program that leaves the value of x unchanged.  The strong inequivalence problem for Ianov schemes with two variables is undecidable, even if |F|=|P|=1 [Luckham, Park, and Paterson, 1970].  See references, [Hunt, 1978], and [Hunt and Szymanski, 1976b] for analogous results for other properties, such as "weak equivalence," "divergence," "halting," etc.  Strong equivalence can be tested in polynomial time for Ianov schemes that are "strongly free," i.e., in which at least one function application occurs between every two successive predicate tests [Constable, Hunt, and Sahni, 1974].  Strong equivalence is open for "free" Ianov schemes.

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

- **[Constable, Hunt, and Sahni, 1974]**: [`Constable1974`] R. L. Constable and H. B. Hunt, III and S. Sahni (1974). "On the computational complexity of scheme equivalence". Cornell University.
- **[Rutledge, 1964]**: [`Rutledge1964`] J. Rutledge (1964). "On {Ianov}'s program schemata". *Journal of the Association for Computing Machinery* 11, pp. 1–9.
- **[Luckham, Park, and Paterson, 1970]**: [`Luckham1970`] David C. Luckham and D. M. Park and M. S. Paterson (1970). "On formalised computer programs". *Journal of Computer and System Sciences* 4, pp. 220–249.
- **[Hunt, 1978]**: [`Hunt1978a`] Harry B. Hunt III (1978). "Uniform lower bounds on scheme equivalence".
- **[Hunt and Szymanski, 1976b]**: [`Hunt1976d`] Harry B. Hunt III and Thomas G. Szymanski (1976). "Complexity metatheorems for context-free grammar problems". *Journal of Computer and System Sciences* 13, pp. 318–334.