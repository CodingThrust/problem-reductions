---
name: Problem
about: Propose a new problem type
title: "[Model] InequivalenceOfLoopProgramsWithoutNesting"
labels: model
assignees: ''
---

## Motivation

INEQUIVALENCE OF LOOP PROGRAMS WITHOUT NESTING (P306) from Garey & Johnson, A11 PO14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO14

**Mathematical definition:**

INSTANCE: Finite set X of variables, subset Y ⊆ X of input variables, specified output variable x0, two loop programs P1 and P2 without nested loops, i.e., sequences of instructions of the form "x ← y," "x ← x+1," "x ← 0," "loop x," and "end," where x,y ∈ X and each loop instruction is followed by a corresponding end instruction before any further loop instructions occur.
QUESTION: Is there an initial assignment f: Y→Z+ of integers to the input variables such that the two programs halt with different values for the output variable x0 (see references for details on the execution of such programs)?

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

INSTANCE: Finite set X of variables, subset Y ⊆ X of input variables, specified output variable x0, two loop programs P1 and P2 without nested loops, i.e., sequences of instructions of the form "x ← y," "x ← x+1," "x ← 0," "loop x," and "end," where x,y ∈ X and each loop instruction is followed by a corresponding end instruction before any further loop instructions occur.
QUESTION: Is there an initial assignment f: Y→Z+ of integers to the input variables such that the two programs halt with different values for the output variable x0 (see references for details on the execution of such programs)?
Reference: [Constable, Hunt, and Sahni, 1974], [Tsichritzis, 1970]. Transformation from 3SAT. The second reference proves membership in NP.
Comment: Problem becomes undecidable if nested loops are allowed (even for nesting of only depth 2) [Meyer and Ritchie, 1967]. Solvable in polynomial time if loop statements are not allowed [Tsichritzis, 1970]. See [Hunt, 1977] for a generalization of the main result.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
