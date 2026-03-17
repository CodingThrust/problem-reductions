---
name: Problem
about: Propose a new problem type
title: "[Model] InequivalenceOfProgramsWithArrays"
labels: model
assignees: ''
---

## Motivation

INEQUIVALENCE OF PROGRAMS WITH ARRAYS (P303) from Garey & Johnson, A11 PO11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO11

**Mathematical definition:**

INSTANCE: Finite sets X, Θ, and R of variables, operators, and array variables, two programs P1 and P2 made up of "operate" (x0 ← θx1x2 ··· xr), "update" (α[xi] ← xj), and "select" (xi ← α[xj]) commands, where each xi ∈ X, θ ∈ Θ, r is the "arity" of θ, and α ∈ R, a finite value set V, and an interpretation of each operator θ ∈ Θ as a specific function from Vr to V.
QUESTION: Is there an initial assignment of a value from V to each variable in X such that the two programs yield different final values for some variable in X (see reference for details on the execution of such programs)?

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

INSTANCE: Finite sets X, Θ, and R of variables, operators, and array variables, two programs P1 and P2 made up of "operate" (x0 ← θx1x2 ··· xr), "update" (α[xi] ← xj), and "select" (xi ← α[xj]) commands, where each xi ∈ X, θ ∈ Θ, r is the "arity" of θ, and α ∈ R, a finite value set V, and an interpretation of each operator θ ∈ Θ as a specific function from Vr to V.
QUESTION: Is there an initial assignment of a value from V to each variable in X such that the two programs yield different final values for some variable in X (see reference for details on the execution of such programs)?
Reference: [Downey and Sethi, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if there are no operate commands and only one array variable. Solvable in polynomial time if there are no update commands, or no select commands, or no array variables.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
