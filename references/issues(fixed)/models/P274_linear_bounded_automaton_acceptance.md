---
name: Problem
about: Propose a new problem type
title: "[Model] LinearBoundedAutomatonAcceptance"
labels: model
assignees: ''
---

## Motivation

LINEAR BOUNDED AUTOMATON ACCEPTANCE (P274) from Garey & Johnson, A10 AL3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL3

**Mathematical definition:**

INSTANCE: A "linear bounded automaton" A with input alphabet Σ (see [Hopcroft and Ullman, 1969] for definition), and a string x ∈ Σ*.
QUESTION: Does A accept x?

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

INSTANCE: A "linear bounded automaton" A with input alphabet Σ (see [Hopcroft and Ullman, 1969] for definition), and a string x ∈ Σ*.
QUESTION: Does A accept x?

Reference: [Karp, 1972]. Generic transformation.
Comment: PSPACE-complete, even if A is deterministic (the LINEAR SPACE ACCEPTANCE problem of Section 7.4). Moreover, there exist fixed deterministic linear bounded automata for which the problem is PSPACE-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
