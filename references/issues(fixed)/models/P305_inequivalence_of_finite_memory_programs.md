---
name: Problem
about: Propose a new problem type
title: "[Model] InequivalenceOfFiniteMemoryPrograms(*)"
labels: model
assignees: ''
---

## Motivation

INEQUIVALENCE OF FINITE MEMORY PROGRAMS (*) (P305) from Garey & Johnson, A11 PO13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO13

**Mathematical definition:**

INSTANCE: Finite set X of variables, finite alphabet Σ, two programs P1 and P2, each a sequence I1,I2,...,Im of instructions (not necessarily of the same length m) of the form "read xi," "write vj," "xi ← vj," "if vj=vk goto Il," "accept," or "halt," where each xi ∈ X, each vj ∈ X ∪ Σ ∪ {$}, and Im is either "halt" or "accept."
QUESTION: Is there a string w ∈ Σ* such that the two programs yield different outputs for input w (see reference for details on the execution of such programs)?

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

INSTANCE: Finite set X of variables, finite alphabet Σ, two programs P1 and P2, each a sequence I1,I2,...,Im of instructions (not necessarily of the same length m) of the form "read xi," "write vj," "xi ← vj," "if vj=vk goto Il," "accept," or "halt," where each xi ∈ X, each vj ∈ X ∪ Σ ∪ {$}, and Im is either "halt" or "accept."
QUESTION: Is there a string w ∈ Σ* such that the two programs yield different outputs for input w (see reference for details on the execution of such programs)?
Reference: [Jones and Muchnik, 1977]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
Comment: PSPACE-complete, even if P2 is a fixed program with no write instructions and hence no output. See reference for a number of other special cases and variants that are PSPACE-complete or harder.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
