---
name: Problem
about: Propose a new problem type
title: "[Model] CodeGenerationWithUnfixedVariableLocations"
labels: model
assignees: ''
---

## Motivation

CODE GENERATION WITH UNFIXED VARIABLE LOCATIONS (P300) from Garey & Johnson, A11 PO8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO8

**Mathematical definition:**

INSTANCE: Sequence I = (I1,I2,...,In) of instructions, finite set V of variables, assignment g: I→I ∪ V, positive integers B and M.
QUESTION: Can the instructions in I be stored as one- and two-byte instructions and the variables stored among them so that the total memory required is at most M, i.e., is there a one-to-one function f: I ∪ V→{1,2,...,M} such that f(Ii) < f(Ij) whenever i < j and such that, for 1 ≤ i ≤ n, either |f(Ii)−f(g(Ii))| < B or f(Ii) + 1 is not in the range of f?

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

INSTANCE: Sequence I = (I1,I2,...,In) of instructions, finite set V of variables, assignment g: I→I ∪ V, positive integers B and M.
QUESTION: Can the instructions in I be stored as one- and two-byte instructions and the variables stored among them so that the total memory required is at most M, i.e., is there a one-to-one function f: I ∪ V→{1,2,...,M} such that f(Ii) < f(Ij) whenever i < j and such that, for 1 ≤ i ≤ n, either |f(Ii)−f(g(Ii))| < B or f(Ii) + 1 is not in the range of f?
Reference: [Robertson, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete even for certain fixed values of B, e.g., B = 31. Solvable in polynomial time if V is empty.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
