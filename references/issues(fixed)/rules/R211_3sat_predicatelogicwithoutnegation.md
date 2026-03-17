---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PREDICATE LOGIC WITHOUT NEGATION"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** PREDICATE LOGIC WITHOUT NEGATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A9.2, p.263

## GJ Source Entry

> [LO15] PREDICATE LOGIC WITHOUT NEGATION
> INSTANCE: Sets U = {u_1,u_2,...,u_n} of variables, F = {f_1^{m_1},f_2^{m_2},...,f_k^{m_k}} of function symbols, and R = {R_1^{r_1},R_2^{r_2},...,R_j^{r_j}} of relation symbols (m_i ≥ 0 and r_i ≥ 0 being the dimensions of the corresponding functions and relations), and a well-formed predicate logic sentence A without negations over U, F, and R. (Such a sentence can be defined inductively as follows: A term is a variable u E U or of the form "f_i^{m_i}(t_1,t_2,...,t_{m_i})" where each t_j is a term. A formula is of the form "t_1=t_2" where t_1 and t_2 are terms, "R_i^{r_i}(t_1,t_2,...,t_{r_i})" where each t_j is a term, or "(A ∧ B)," "(A V B)," "∀u_i(A)," or "∃u_i(A)" where A and B are formulas and u_i E U. A sentence is a formula in which all variables are quantified before they occur.)
> QUESTION: Is A true under all interpretations of F and R?
> Reference: [Kozen, 1977c]. Transformation from 3SAT. Nontrivial part is proving membership in NP.
> Comment: Remains NP-complete even if there are no universal quantifiers, no relation symbols, and only two functions, both with dimension 0 (and hence constants).

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

- **[Kozen, 1977c]**: [`Kozen1977c`] Dexter Kozen (1977). "First order predicate logic without negation is {NP}-complete". Dept. of Computer Science, Cornell University.