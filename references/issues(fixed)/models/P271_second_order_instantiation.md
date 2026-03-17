---
name: Problem
about: Propose a new problem type
title: "[Model] SecondOrderInstantiation"
labels: model
assignees: ''
---

## Motivation

SECOND ORDER INSTANTIATION (P271) from Garey & Johnson, A9 LO19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO19

**Mathematical definition:**

INSTANCE: Two "second order logic expressions" E_1 and E_2, the second of which contains no variables (in a second order expression, functions can be variables; see references for details).
QUESTION: Is there a substitution for the variables of E_1 that yields an expression identical to E_2?

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

INSTANCE: Two "second order logic expressions" E_1 and E_2, the second of which contains no variables (in a second order expression, functions can be variables; see references for details).
QUESTION: Is there a substitution for the variables of E_1 that yields an expression identical to E_2?
Reference: [Baxter, 1976]. Transformation from 3SAT. Proof of membership in NP is nontrivial.
Comment: The more general SECOND ORDER UNIFICATION problem, where both E_1 and E_2 can contain variables and we ask if there is a substitution for the variables in E_1 and E_2 that results in identical expressions, is not known to be decidable. THIRD ORDER UNIFICATION is undecidable [Huet, 1973], whereas FIRST ORDER UNIFICATION can be solved in polynomial time [Baxter, 1975], [Paterson and Wegman, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
