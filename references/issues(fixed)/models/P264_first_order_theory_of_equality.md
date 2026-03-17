---
name: Problem
about: Propose a new problem type
title: "[Model] FirstOrderTheoryOfEquality(*)"
labels: model
assignees: ''
---

## Motivation

FIRST ORDER THEORY OF EQUALITY (*) (P264) from Garey & Johnson, A9 LO12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO12

**Mathematical definition:**

INSTANCE: Finite set U={u_1,u_2,...,u_n} of variables, sentence S over U in the first order theory of equality. (Such sentences can be defined inductively as follows: An "expression" is of the form "u=v" where u,v∈U, or of the form "¬E," "(E∨F)," "(E∧F)," or "(E→F)" where E and F are expressions. A sentence is of the form (Q_1u_1)(Q_2u_2)···(Q_nu_n)E where E is an expression and each Q_i is either ∀ or ∃.)
QUESTION: Is S true in all models of the theory?

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

INSTANCE: Finite set U={u_1,u_2,...,u_n} of variables, sentence S over U in the first order theory of equality. (Such sentences can be defined inductively as follows: An "expression" is of the form "u=v" where u,v∈U, or of the form "¬E," "(E∨F)," "(E∧F)," or "(E→F)" where E and F are expressions. A sentence is of the form (Q_1u_1)(Q_2u_2)···(Q_nu_n)E where E is an expression and each Q_i is either ∀ or ∃.)
QUESTION: Is S true in all models of the theory?
Reference: [Stockmeyer and Meyer, 1973]. Generic transformation.
Comment: PSPACE-complete. The analogous problem for any fixed first order theory that has a model in which some predicate symbol is interpreted as a relation that holds sometimes but not always is PSPACE-hard [Hunt, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
