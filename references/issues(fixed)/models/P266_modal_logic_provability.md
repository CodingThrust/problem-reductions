---
name: Problem
about: Propose a new problem type
title: "[Model] ModalLogicProvability(*)"
labels: model
assignees: ''
---

## Motivation

MODAL LOGIC PROVABILITY (*) (P266) from Garey & Johnson, A9 LO14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO14

**Mathematical definition:**

INSTANCE: Well-formed modal formula A, modal system S∈{K,T,S4} (see reference or [Hughes and Cresswell, 1968] for details of K, T, and S4).
QUESTION: Is A provable in system S?

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

INSTANCE: Well-formed modal formula A, modal system S∈{K,T,S4} (see reference or [Hughes and Cresswell, 1968] for details of K, T, and S4).
QUESTION: Is A provable in system S?
Reference: [Ladner, 1977]. Transformation from QBF.
Comment: PSPACE-complete for fixed S∈{K,T,S4} or for any fixed modal system S in which everything provable in K, but nothing not provable in S4, can be proved.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
