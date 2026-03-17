---
name: Problem
about: Propose a new problem type
title: "[Model] QuantifiedBooleanFormulas(qbf)(*)"
labels: model
assignees: ''
---

## Motivation

QUANTIFIED BOOLEAN FORMULAS (QBF) (*) (P263) from Garey & Johnson, A9 LO11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO11

**Mathematical definition:**

INSTANCE: Set U={u_1,u_2,...,u_n} of variables, well-formed quantified Boolean formula F=(Q_1u_1)(Q_2u_2)···(Q_nu_n)E, where E is a Boolean expression and each Q_i is either ∀ or ∃.
QUESTION: Is F true?

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

INSTANCE: Set U={u_1,u_2,...,u_n} of variables, well-formed quantified Boolean formula F=(Q_1u_1)(Q_2u_2)···(Q_nu_n)E, where E is a Boolean expression and each Q_i is either ∀ or ∃.
QUESTION: Is F true?
Reference: [Stockmeyer and Meyer, 1973]. Generic transformation.
Comment: PSPACE-complete, even if E is in conjunctive normal form with three literals per clause (QUANTIFIED 3SAT), but solvable in polynomial time when there are at most two literals per clause [Schaefer, 1978b]. If F is restricted to at most k alternations of quantifiers (i.e., there are at most k indices i such that Q_i≠Q_{i+1}), then the restricted problem is complete for some class in the polynomial hierarchy, depending on k and the allowed values for Q_1 (see Section 7.2).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
