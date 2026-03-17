---
name: Problem
about: Propose a new problem type
title: "[Model] RegularGrammarInequivalence"
labels: model
assignees: ''
---

## Motivation

REGULAR GRAMMAR INEQUIVALENCE (P285) from Garey & Johnson, A10 AL14. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL14

**Mathematical definition:**

INSTANCE: Regular grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2), where a regular grammar is a context-free grammar in which each production has the form A → aB or A → a with A, B ∈ N and a ∈ Σ.
QUESTION: Do G_1 and G_2 generate different languages?

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

INSTANCE: Regular grammars G_1 = (N_1,Σ,Π_1,S_1) and G_2 = (N_2,Σ,Π_2,S_2), where a regular grammar is a context-free grammar in which each production has the form A → aB or A → a with A, B ∈ N and a ∈ Σ.
QUESTION: Do G_1 and G_2 generate different languages?
Reference: [Chomsky and Miller, 1958]. Transformation from FINITE STATE AUTOMATON INEQUIVALENCE.
Comment: PSPACE-complete, even if |Σ| = 2 and G_2 is a fixed grammar generating Σ* (REGULAR GRAMMAR NON-UNIVERSALITY). The general problem is NP-complete if |Σ| = 1 or if both grammars generate finite languages (a property that can be checked in polynomial time, e.g., see [Hopcroft and Ullman, 1969]). If G_1 is allowed to be an arbitrary linear grammar and G_2 is a fixed grammar generating Σ* (LINEAR GRAMMAR NON-UNIVERSALITY), the problem is undecidable [Hunt, Rosenkrantz, and Szymanski, 1976a].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
