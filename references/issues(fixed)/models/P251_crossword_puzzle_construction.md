---
name: Problem
about: Propose a new problem type
title: "[Model] CrosswordPuzzleConstruction"
labels: model
assignees: ''
---

## Motivation

CROSSWORD PUZZLE CONSTRUCTION (P251) from Garey & Johnson, A8 GP14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A8 GP14

**Mathematical definition:**

INSTANCE: A finite set W ⊆ Σ* of words and an n×n matrix A of 0's and 1's.
QUESTION: Can an n×n crossword puzzle be built up from the words in W and blank squares corresponding to the 0's of A, i.e., if E is the set of pairs (i,j) such that Aij = 0, is there an assignment f: E → Σ such that the letters assigned to any maximal horizontal or vertical contiguous sequence of members of E form, in order, a word of W?

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

INSTANCE: A finite set W ⊆ Σ* of words and an n×n matrix A of 0's and 1's.
QUESTION: Can an n×n crossword puzzle be built up from the words in W and blank squares corresponding to the 0's of A, i.e., if E is the set of pairs (i,j) such that Aij = 0, is there an assignment f: E → Σ such that the letters assigned to any maximal horizontal or vertical contiguous sequence of members of E form, in order, a word of W?

Reference: [Lewis and Papadimitriou, 1978]. Transformation from X3C.
Comment: Remains NP-complete even if all entries in A are 0.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
