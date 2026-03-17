---
name: Problem
about: Propose a new problem type
title: "[Model] SafetyOfDatabaseTransactionSystems(*)"
labels: model
assignees: ''
---

## Motivation

SAFETY OF DATABASE TRANSACTION SYSTEMS (*) (P182) from Garey & Johnson, A4 SR34. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR34

**Mathematical definition:**

INSTANCE: Set V of database variables, and a collection T of "transactions" (Ri,Wi), 1 ≤ i ≤ n, where Ri and Wi are both subsets of V.
QUESTION: Is every history H for T equivalent to some serial history?

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

INSTANCE: Set V of database variables, and a collection T of "transactions" (Ri,Wi), 1 ≤ i ≤ n, where Ri and Wi are both subsets of V.
QUESTION: Is every history H for T equivalent to some serial history?
Reference: [Papadimitriou, Bernstein, and Rothnie, 1977]. Transformation from HITTING SET.
Comment: Not known either to be in NP or to be in co-NP. Testing whether every history H for T is "D-equivalent" to some serial history can be done in polynomial time, where two histories are D-equivalent if one can be obtained from the other by a sequence of interchanges of adjacent sets in such a way that at each step the new history is equivalent to the one.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
