---
name: Problem
about: Propose a new problem type
title: "[Model] StructuralInequivalenceForLinearGrammars"
labels: model
assignees: ''
---

## Motivation

STRUCTURAL INEQUIVALENCE FOR LINEAR GRAMMARS (P284) from Garey & Johnson, A10 AL13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL13

**Mathematical definition:**

INSTANCE: Two linear context-free grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2).
QUESTION: Are G_1 and G_2 "structurally inequivalent," i.e., do the parenthesized grammars obtained from G_1 and G_2 by replacing each production A → w by A → (w) (where "(" and ")" are new terminal symbols) generate different languages?

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

INSTANCE: Two linear context-free grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2).
QUESTION: Are G_1 and G_2 "structurally inequivalent," i.e., do the parenthesized grammars obtained from G_1 and G_2 by replacing each production A → w by A → (w) (where "(" and ")" are new terminal symbols) generate different languages?
Reference: [Hunt, Rosenkrantz, and Szymanski, 1976a]. Transformation from REGULAR EXPRESSION NON-UNIVERSALITY.
Comment: PSPACE-complete, even if G_1 and G_2 are regular and |Σ| = 2. NP-complete if |Σ| = 1. For arbitrary context-free grammars, problem is decidable but not known to be in PSPACE.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
