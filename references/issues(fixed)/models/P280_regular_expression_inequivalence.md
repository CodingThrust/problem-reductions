---
name: Problem
about: Propose a new problem type
title: "[Model] RegularExpressionInequivalence"
labels: model
assignees: ''
---

## Motivation

REGULAR EXPRESSION INEQUIVALENCE (P280) from Garey & Johnson, A10 AL9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL9

**Mathematical definition:**

INSTANCE: Regular expressions E_1 and E_2 over the operators {∪,·,*} and the alphabet Σ (see Section 7.4 for definition).
QUESTION: Do E_1 and E_2 represent different languages?

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

INSTANCE: Regular expressions E_1 and E_2 over the operators {∪,·,*} and the alphabet Σ (see Section 7.4 for definition).
QUESTION: Do E_1 and E_2 represent different languages?
Reference: [Stockmeyer and Meyer, 1973], [Stockmeyer, 1974a]. Generic transformation. The second reference proves membership in PSPACE.
Comment: PSPACE-complete, even if |Σ| = 2 and E_2 = Σ* (REGULAR EXPRESSION NON-UNIVERSALITY, see Section 7.4). In fact, PSPACE-complete if E_2 is any fixed expression representing an "unbounded" language [Hunt, Rosenkrantz, and Szymanski, 1976a]. NP-complete for fixed E_2 representing any infinite "bounded" language, but solvable in polynomial time for fixed E_2 representing any finite language. The general problem remains PSPACE-complete if E_1 and E_2 both have "star height" k for a fixed k ≥ 1 [Hunt, Rosenkrantz, and Szymanski, 1976a], but is NP-complete for k = 0 ("star free") [Stockmeyer and Meyer, 1973], [Hunt, 1973a]. Also NP-complete if one or both of E_1 and E_2 represent bounded languages (a property that can be checked in polynomial time) [Hunt, Rosenkrantz, and Szymanski, 1976a] or if |Σ| = 1 [Stockmeyer and Meyer, 1973]. For related results and intractable generalizations, see cited references, [Hunt, 1973b], and [Hunt and Rosenkrantz, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
