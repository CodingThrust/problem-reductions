---
name: Problem
about: Propose a new problem type
title: "[Model] PredicateLogicWithoutNegation"
labels: model
assignees: ''
---

## Motivation

PREDICATE LOGIC WITHOUT NEGATION (P267) from Garey & Johnson, A9 LO15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO15

**Mathematical definition:**

INSTANCE: Sets U={u_1,u_2,...,u_n} of variables, F={f_1^{m_1},f_2^{m_2},...,f_k^{m_k}} of function symbols, and R={R_1^{r_1},R_2^{r_2},...,R_j^{r_j}} of relation symbols (m_i≥0 and r_i≥0 being the dimensions of the corresponding functions and relations), and a well-formed predicate logic sentence A without negations over U, F, and R. (Such a sentence can be defined inductively as follows: A term is a variable u∈U or of the form "f_i^{m_i}(t_1,t_2,...,t_{m_i})" where each t_j is a term. A formula is of the form "t_1=t_2" where t_1 and t_2 are terms, "R_i^{r_i}(t_1,t_2,...,t_{r_i})" where each t_j is a term, or "(A∧B)," "(A∨B)," "∀u_i(A)," or "∃u_i(A)" where A and B are formulas and u_i∈U. A sentence is a formula in which all variables are quantified before they occur.)
QUESTION: Is A true under all interpretations of F and R?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: Sets U={u_1,u_2,...,u_n} of variables, F={f_1^{m_1},f_2^{m_2},...,f_k^{m_k}} of function symbols, and R={R_1^{r_1},R_2^{r_2},...,R_j^{r_j}} of relation symbols (m_i≥0 and r_i≥0 being the dimensions of the corresponding functions and relations), and a well-formed predicate logic sentence A without negations over U, F, and R. (Such a sentence can be defined inductively as follows: A term is a variable u∈U or of the form "f_i^{m_i}(t_1,t_2,...,t_{m_i})" where each t_j is a term. A formula is of the form "t_1=t_2" where t_1 and t_2 are terms, "R_i^{r_i}(t_1,t_2,...,t_{r_i})" where each t_j is a term, or "(A∧B)," "(A∨B)," "∀u_i(A)," or "∃u_i(A)" where A and B are formulas and u_i∈U. A sentence is a formula in which all variables are quantified before they occur.)
QUESTION: Is A true under all interpretations of F and R?
Reference: [Kozen, 1977c]. Transformation from 3SAT. Nontrivial part is proving membership in NP.
Comment: Remains NP-complete even if there are no universal quantifiers, no relation symbols, and only two functions, both with dimension 0 (and hence constants).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
