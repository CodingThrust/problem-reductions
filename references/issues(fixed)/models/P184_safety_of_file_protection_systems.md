---
name: Problem
about: Propose a new problem type
title: "[Model] SafetyOfFileProtectionSystems(*)"
labels: model
assignees: ''
---

## Motivation

SAFETY OF FILE PROTECTION SYSTEMS (*) (P184) from Garey & Johnson, A4 SR36. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR36

**Mathematical definition:**

INSTANCE: Set R of "rights," set O of objects, set S ⊆ O of subjects, set P(s,o) ⊆ R of rights for each ordered pair s ∈ S and o ∈ O, a finite set C of commands, each having the form "if r1 ∈ P(X1,Y1) and r2 ∈ P(X2,Y2) and · · · and rm ∈ P(Xm,Ym), then θ1,θ2,...,θn" for m,n ≥ 0 and each θi of the form "enter ri into P(Xj,Yk)" or "delete ri from P(Kj,Yk)," and a specified right r' ∈ R.
QUESTION: Is there a sequence of commands from C and a way of identifying each ri, Xi, and Yk with a particular element of R, S, and O, respectively, such that at some point in the execution of the sequence, the right r' is entered into a set P(s,o) that previously did not contain r' (see reference for details on the execution of such a sequence)?

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

INSTANCE: Set R of "rights," set O of objects, set S ⊆ O of subjects, set P(s,o) ⊆ R of rights for each ordered pair s ∈ S and o ∈ O, a finite set C of commands, each having the form "if r1 ∈ P(X1,Y1) and r2 ∈ P(X2,Y2) and · · · and rm ∈ P(Xm,Ym), then θ1,θ2,...,θn" for m,n ≥ 0 and each θi of the form "enter ri into P(Xj,Yk)" or "delete ri from P(Kj,Yk)," and a specified right r' ∈ R.
QUESTION: Is there a sequence of commands from C and a way of identifying each ri, Xi, and Yk with a particular element of R, S, and O, respectively, such that at some point in the execution of the sequence, the right r' is entered into a set P(s,o) that previously did not contain r' (see reference for details on the execution of such a sequence)?
Reference: [Harrison, Ruzzo, and Ullman, 1976]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
Comment: PSPACE-complete. Undecidable if operations that create or delete "subjects" and "objects" are allowed, even for certain "fixed" systems in which only the initial values of the P(s,o) are allowed to vary. If no command can contain more than one operation, then the problem is NP-complete in general and solvable in polynomial time for fixed systems.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
