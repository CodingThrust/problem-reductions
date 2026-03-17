---
name: Problem
about: Propose a new problem type
title: "[Model] CodeGenerationWithAddressExpressions"
labels: model
assignees: ''
---

## Motivation

CODE GENERATION WITH ADDRESS EXPRESSIONS (P299) from Garey & Johnson, A11 PO7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO7

**Mathematical definition:**

INSTANCE: Sequence I = (I1,I2,...,In) of instructions, for each Ii ∈ I an expression g(Ii) of the form "Ij," "Ij + k," "Ij − k," or "k" where Ij ∈ I and k ∈ Z+, and positive integers B, C, and M.
QUESTION: Can the instructions in I be stored as one- and two-byte instructions so that the total memory required is at most M, i.e., is there a one-to-one function f: I→{1,2,...,M} such that f(Ii) < f(Ij) whenever i < j and such that, if h(Ii) is defined to be f(Ij), f(Ij) ± k, or k depending on whether g(Ii) is Ij, Ij ± k, or k, then for each i, 1 ≤ i ≤ n, either −C < f(Ii)−h(Ii) < B or f(Ii) + 1 is not in the range of f?

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

INSTANCE: Sequence I = (I1,I2,...,In) of instructions, for each Ii ∈ I an expression g(Ii) of the form "Ij," "Ij + k," "Ij − k," or "k" where Ij ∈ I and k ∈ Z+, and positive integers B, C, and M.
QUESTION: Can the instructions in I be stored as one- and two-byte instructions so that the total memory required is at most M, i.e., is there a one-to-one function f: I→{1,2,...,M} such that f(Ii) < f(Ij) whenever i < j and such that, if h(Ii) is defined to be f(Ij), f(Ij) ± k, or k depending on whether g(Ii) is Ij, Ij ± k, or k, then for each i, 1 ≤ i ≤ n, either −C < f(Ii)−h(Ii) < B or f(Ii) + 1 is not in the range of f?
Reference: [Szymanski, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete for certain fixed values of B and C, e.g., 128 and 127 (much smaller values also are possible). Solvable in polynomial time if no "pathological" expressions occur (see reference for details).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
