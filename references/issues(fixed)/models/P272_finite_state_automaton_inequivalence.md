---
name: Problem
about: Propose a new problem type
title: "[Model] FiniteStateAutomatonInequivalence"
labels: model
assignees: ''
---

## Motivation

FINITE STATE AUTOMATON INEQUIVALENCE (P272) from Garey & Johnson, A10 AL1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL1

**Mathematical definition:**

INSTANCE: Two nondeterministic finite state automata A_1 and A_2 having the same input alphabet Σ (where such an automaton A = (Q,Σ,δ,q_0,F) consists of a finite set Q of states, input alphabet Σ, transition function δ mapping Q×Σ into subsets of Q, initial state q_0, and a set F⊆K of "accept" states, e.g., see [Hopcroft and Ullman, 1969]).
QUESTION: Do A_1 and A_2 recognize different languages?

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

INSTANCE: Two nondeterministic finite state automata A_1 and A_2 having the same input alphabet Σ (where such an automaton A = (Q,Σ,δ,q_0,F) consists of a finite set Q of states, input alphabet Σ, transition function δ mapping Q×Σ into subsets of Q, initial state q_0, and a set F⊆K of "accept" states, e.g., see [Hopcroft and Ullman, 1969]).
QUESTION: Do A_1 and A_2 recognize different languages?

Reference: [Kleene, 1956]. Transformation from REGULAR EXPRESSION NON-UNIVERSALITY.
Comment: PSPACE-complete, even if |Σ|=2 and A_2 is the trivial automaton recognizing Σ*. The general problem is NP-complete if |Σ|=1, or if A_1 and A_2 both recognize finite languages (a property that can be checked in polynomial time, e.g., see [Hopcroft and Ullman, 1969]). Problem is solvable in polynomial time if A_1 and A_2 are deterministic finite state automata, e.g., see [Hopcroft and Ullman, 1969].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
