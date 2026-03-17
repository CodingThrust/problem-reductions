---
name: Problem
about: Propose a new problem type
title: "[Model] TwoWayFiniteStateAutomatonNonEmptiness"
labels: model
assignees: ''
---

## Motivation

TWO-WAY FINITE STATE AUTOMATON NON-EMPTINESS (P273) from Garey & Johnson, A10 AL2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL2

**Mathematical definition:**

INSTANCE: A two-way nondeterministic finite state automaton A = (Q,Σ,δ,q_0,F) (where Q, Σ, q_0, and F are the same as for a one-way nondeterministic finite state automaton, but the transition function δ maps Q×Σ into subsets of Q×{-1,0,1}, e.g., see [Hopcroft and Ullman, 1969]).
QUESTION: Is there an x ∈ Σ* such that A accepts x?

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

INSTANCE: A two-way nondeterministic finite state automaton A = (Q,Σ,δ,q_0,F) (where Q, Σ, q_0, and F are the same as for a one-way nondeterministic finite state automaton, but the transition function δ maps Q×Σ into subsets of Q×{-1,0,1}, e.g., see [Hopcroft and Ullman, 1969]).
QUESTION: Is there an x ∈ Σ* such that A accepts x?

Reference: [Hunt, 1973b]. Transformation from LINEAR BOUNDED AUTOMATON ACCEPTANCE.
Comment: PSPACE-complete, even if |Σ|=2 and A is deterministic. If |Σ|=1 the general problem is NP-complete [Galil, 1976]. If A is a one-way nondeterministic finite state automaton, the general problem can be solved in polynomial time (e.g., see [Hopcroft and Ullman, 1969]). Analogous results for the question of whether A recognizes an infinite language can be found in the above references.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
