---
name: Problem
about: Propose a new problem type
title: "[Model] SerializabilityOfDatabaseHistories"
labels: model
assignees: ''
---

## Motivation

SERIALIZABILITY OF DATABASE HISTORIES (P181) from Garey & Johnson, A4 SR33. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR33

**Mathematical definition:**

INSTANCE: Set V of database variables, collection T of "transactions" (Ri,Wi), 1 ≤ i ≤ n, where Ri and Wi are both subsets of V (called the "read set" and the "write set," respectively), and a "history" H for T, where a history is simply a permutation of all the Ri and the Wi in which each Ri occurs before the corresponding Wi.
QUESTION: Is there a serial history H' for T (i.e., a history in which each Ri occurs immediately before the corresponding Wi) that is equivalent to H in the sense that (1) both histories have the same set of "live" transactions (where a transaction (Ri,Wi) is live in a history if there is some v ∈ V such that either Wi is the last write set to contain v or Wi is the last write set to contain v before v appears in the read set of some other live transaction), and (2) for any two live transactions (Ri,Wi) and (Rj,Wj) and any v ∈ Wi ∩ Rj, Wi is the last write set to contain v before Rj in H if and only if Wi is the last write set to contain v before Rj in H'?

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

INSTANCE: Set V of database variables, collection T of "transactions" (Ri,Wi), 1 ≤ i ≤ n, where Ri and Wi are both subsets of V (called the "read set" and the "write set," respectively), and a "history" H for T, where a history is simply a permutation of all the Ri and the Wi in which each Ri occurs before the corresponding Wi.
QUESTION: Is there a serial history H' for T (i.e., a history in which each Ri occurs immediately before the corresponding Wi) that is equivalent to H in the sense that (1) both histories have the same set of "live" transactions (where a transaction (Ri,Wi) is live in a history if there is some v ∈ V such that either Wi is the last write set to contain v or Wi is the last write set to contain v before v appears in the read set of some other live transaction), and (2) for any two live transactions (Ri,Wi) and (Rj,Wj) and any v ∈ Wi ∩ Rj, Wi is the last write set to contain v before Rj in H if and only if Wi is the last write set to contain v before Rj in H'?
Reference: [Papadimitriou, Bernstein, and Rothnie, 1977], [Papadimitriou, 1978c]. Transformation from MONOTONE 3SAT.
Comment: For related polynomial time solvable subcases and variants, see [Papadimitriou, 1978c].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
